## Context

request 服务（download_server，SA ID 3706）当前是**单例 SA 进程 + 单 DB 文件 + uid 字段过滤**模型。证据：
- `ability.rs:237` — `build_system_ability(DOWNLOAD_SERVICE_ID, false)` 单例注册
- `manage/database.rs:78-98` — `RequestDb::get_instance()` 进程级单例，路径硬编码 `/data/service/el1/public/database/request/request.db`
- `manage/database.rs:35` — `request_task` 单表，uid 列过滤
- 6 处进程级单例：RequestDb、TaskManager、ClientManager、RunCountManager、SystemConfigManager、ylong_runtime

**samgr 上游已支持 per-user 多 SA 进程**（/tmp/samgr-latest 调研确认）：
- `MultiSystemAbilityManager(userId)` — 每用户独立 SA 管理器（multi_system_ability_manager.cpp:25）
- `StartDynamicSystemProcess(name, said, event)` — 按用户拉起独立进程，event 带 userId（:83-116）
- `OnUserStateChanged(userId, state)` — 监听用户状态变化（system_ability_manager.cpp:741，`#ifdef SUPPORT_MULTI_INSTANCE`）
- SA profile 字段 `"multi-instance": true`（parse_util.cpp:70 `SA_TAG_MULTI_INSTANCE`，受 `SUPPORT_MULTI_INSTANCE` 宏控制）
- `SubscribeSystemProcess` — 跨进程订阅 API（system_ability_manager.h:85，Rust manage.rs:239）

**6 条需求决策**（用户 2026-08-03 确认）：
1. 多 SA 进程 + 宏隔离（PC 产品），用户↔SA进程一一对应，已认可额外功耗
2. 每用户 SA 实例独立 DB
3. 回调/通知栏正常；OTA 容忍历史任务丢失（不迁移）
4. SA 实例生命周期继承现有规格
5. RSS 订阅监听所有实例任务总数（跨实例聚合）
6. 不跨实例排优先级，每实例任务数规格不变

扫描细节见 references/{architecture-map, interface-catalog, risks-and-constraints, pb-inventory}.md。

## 架构概览

> See references/architecture-map.md（依赖图 + 拓扑序）、references/interface-catalog.md（pub API + FFI/unsafe/async 边界）、references/risks-and-constraints.md（8 项风险）、references/pb-inventory.md（PB 可用性）。

**当前架构**：单例 SA 进程（SA ID 3706）+ 单 DB 文件（`.../public/database/request/request.db`）+ uid 字段过滤。6 处进程级全局单例（RequestDb/TaskManager/ClientManager/RunCountManager/SystemConfigManager/ylong_runtime）。

**目标架构**：samgr multi-instance 驱动的 per-user 多 SA 进程。每用户一个 download_server 进程，进程内单例天然 per-user 隔离。samgr `OnUserStateChanged` → `StartDynamicSystemProcess` 自动拉起/销毁进程；request 侧从 **access token** 读 userId（samgr 按用户拉起进程时 init 调 `SetUserIdToAccessToken` 打标，进程启动后 `GET_USERID` ioctl 读回）命名 DB 文件。

```
samgr (SUPPORT_MULTI_INSTANCE)
  └─ MultiSystemAbilityManager(userId) → StartDynamicSystemProcess
       └─ init SetUserIdToAccessToken(userId)（/dev/access_token_id 打标）
            └─ download_server@userId 进程
                 └─ RequestAbility.on_start_with_reason
                      └─ manage::account::get_user_id_from_token()（GET_USERID ioctl 读回）
                           └─ RequestDb::get_instance(path=.../request_<userId>.db)
                                └─ RdbStore::open(OpenConfig::new(path)) → C++ RequestDataBase
```

**跨实例聚合（需求5，客户端侧）**：

```
RSS 客户端（request framework native）
  └─ FwkRunningTaskCountManager::GetCount()          # 多实例分支 → 跨实例总数
       └─ MultiInstanceRunCountManager::GetTotalCount()  # 对 pidCountMap_ 求和（聚合点）
            └─ pidCountMap_ ← SetCountByPid(callerPid, runCount)   # OnCallBack 落表
                 └─ 各 download_server@userId 进程 SubRunCount push-back（sync IPC，复用现有回调）
  └─ RestoreSubRunCount()                              # 第一位订阅者触发重建
       └─ GetExtensionRunningSaList("download_server") → 逐个 SubRunCount 重新订阅
```

## Goals / Non-Goals

**Goals:**
- 每个 OS 用户拥有独立 SA 实例进程（samgr multi-instance 拉起），进程内 TaskManager/RequestDb 天然 per-user 隔离
- 每进程 DB 文件按 userId 命名（`.../public/database/request/request_<userId>.db`，文件名带 uid，目录共享）
- SA 生命周期由 samgr OnUserStateChanged 自动管理，继承现有 on_start/idle/active 规格
- RSS 订阅能监听所有 SA 实例任务总数变化（跨实例聚合）
- PC 产品宏隔离，非 PC 产品行为不变

**Non-Goals:**
- 不自建 InstanceRegistry 或内部路由层（samgr 管多进程，进程即用户）
- 不改 SA 框架代码（用 samgr 上游能力，不修改 samgr/safwk 源码）
- 不做旧 DB 迁移（OTA 容忍历史丢失）
- 不跨实例排任务优先级
- 不改 ylong_runtime（每进程独立 runtime，天然隔离）
- 不重构任务执行层（download/upload HTTP 逻辑不变，仅作用域收窄到进程内）

## Decisions

### D1: 多 SA 进程 + samgr multi-instance（方案核心）

利用 samgr 上游 `SUPPORT_MULTI_INSTANCE` 能力。SA profile 标 `"multi-instance": true`，samgr 在 `OnUserStateChanged` 时自动通过 `StartDynamicSystemProcess` 按用户拉起独立 download_server 进程。request 侧不主动管理多实例。

```
samgr (SUPPORT_MULTI_INSTANCE 开)
  OnUserStateChanged(user 0, CREATED)
    → StartDynamicSystemProcess("download_server", 3706, event{userId=0})
      → init SetUserIdToAccessToken(0)（/dev/access_token_id 打标）
        → 进程 download_server@user0
          → RequestAbility.on_start_with_reason
            → get_user_id_from_token() 读回 userId=0
              → DB 路径 = /data/service/el1/public/database/request/request_0.db
              → TaskManager/ClientManager/RunCountManager（进程内单例，天然 per-user）
```

**理由**：samgr 上游已实现，request 侧改造量最小，进程级隔离更强，匹配需求 1/4。

### D2: per-user DB 文件名带 userId（进程内单例保持）

DB 路径格式（用户确认）：**文件名带 uid**——`/data/service/el1/public/database/request/request_<userId>.db`。目录共享（现有 `.../public/database/request/` 不变，etc/init mkdir 不改），文件名按 userId 区分。userId 从 **access token** 拿（samgr 按用户拉起进程时 init `SetUserIdToAccessToken` 打标，进程内 `get_user_id_from_token()` 用 `GET_USERID` ioctl 读回，SELinux `allowxperm download_server ... ioctl { 0x410d }` 已放行）。RequestDb 保持进程内单例（`get_instance()` 内部从进程上下文拿 userId 构造路径，或 init 时注入）。

**三处硬编码点同步参数化**（必须全改，不能漏 C++ 侧）：
- services/src/manage/database.rs:86（Rust RequestDb）
- services/src/database/mod.rs:33（Rust）
- services/include/c_request_database.h:35（C++ `DB_NAME` 常量）

OTA 不迁移旧 `request.db`（需求3）。

**理由**：每进程一个 DB 文件（文件名带 uid），物理隔离；目录共享减少 mkdir 复杂度；进程内单例无需改 per-user 实例池（方案简化）。

### D3: RSS 跨实例聚合（需求5）

RunCountManager 进程内计数保持（每 SA 实例各一份）。跨实例聚合在 **RSS 客户端侧**（request framework native）完成：

- **聚合点** = `MultiInstanceRunCountManager`（frameworks/native/request），维护 `pidCountMap_`（实例 pid → 本地任务数）
- **数据上报**：各 SA 实例任务数变化时，进程内 RunCountManager 通过现有 `notify_run_count` → sync IPC → 客户端 `RunCountNotifyStub::OnCallBack` → `SetCountByPid(callerPid, runCount)`。复用现有 SubRunCount 回调路径，无新增跨进程 IPC
- **总数**：`FwkRunningTaskCountManager::GetCount()` 多实例分支返回 `MultiInstanceRunCountManager::GetTotalCount()`（对 `pidCountMap_` 求和）＝跨实例总数，通知 RSS 订阅者
- **实例发现/重连**：`RestoreSubRunCount()` 用 `GetExtensionRunningSaList(DOWNLOAD_SERVER_EXTENSION)` 枚举所有在跑实例，逐个 `SubRunCount` 重新订阅；每个实例收到后立即 push-back 当前计数，重填 map（死实例自然掉出）。`rebuilding_` atomic 标志防重入（同步 IPC 期间 push-back 会重入 `SetCountByPid`）
- **触发点**：第一位订阅者订阅（observer 集合空→非空）时重建
- **count==0 语义**：`SetCountByPid` 在 count==0 时 erase 该 pid，map 只保留活跃实例的计数

**不依赖**：`subscribe_system_process` 进程启停监听、常驻聚合 SA、主实例（user 0）、文件共享。

**已知限制（F-2）**：实例被强杀（用户注销 USER_REMOVED，samgr 直接杀进程）时 SA 侧没有 `on_stop` 清 0 推送，客户端 `pidCountMap_` 保留死 pid 旧值、总数虚高，直到下一次 `RestoreSubRunCount`（仅新进程第一位订阅者触发）才重建修正。SA 级 `OnRemoveSystemAbility` 监听的是 `DOWNLOAD_SERVICE_ID`（3706 整体卸载）且只 `SetCount(0)` 清单实例 `count_`，不清 per-pid map。补齐方向：ability.rs 增加 `on_stop` 生命周期（SA 停止前向客户端推送 count=0）或客户端在实例卸载时定向 erase——未实现，见修订日志 v6。

### D4: 宏隔离（BUILD.gn 配置）

- samgr 侧：`SUPPORT_MULTI_INSTANCE` C++ 编译宏（samgr 仓 BUILD.gn 控制，request 确认 PC product 的 samgr 编译已开）
- request 侧：**services/BUILD.gn** 配置（非 bundle.json）
  - C++：`defines += ["SUPPORT_MULTI_INSTANCE"]`（PC product 条件，沿用现有 :155-170 的 `defines` 条件模式）
  - Rust：`features += ["multi-instance"]`（PC product 条件，加到 `ohos_rust_shared_library("download_server")` 的 features 数组，现有 `features = [ "oh" ]`）
  - Cargo.toml `[features]` 定义 `multi-instance = []`
- request 代码：`cfg(feature="multi-instance")` 隔离多实例 vs 单实例路径
- 非 PC 产品：宏不开，行为完全不变（单 SA 单 DB）

### D5: 生命周期继承现有规格（需求4）

`on_start_with_reason` / `on_idle` / `on_active` 逻辑复用。samgr 的 `OnUserStateChanged` 自动管进程拉起/销毁。start-on-demand 加 `USER_ADDED` 事件（用户添加拉起实例），USER_REMOVED 保留（销毁实例）。

## Risks / Trade-offs

- **R-001 依赖升级**：samgr 必须升级到含 multi-instance 的版本，可能伴随 Rust API 变化（新版新增 send_strategy/subscribe_system_process 等）。Phase 1 先升级验证。
- **R-002 宏隔离**：SUPPORT_MULTI_INSTANCE + cfg(feature="multi-instance") 双宏，需 build 系统正确传递。非 PC 产品不能误开。
- **R-005 RSS 跨实例聚合**：复杂度最高的部分（独立 Phase 4）。实现为 RSS 客户端侧 per-pid 聚合（MultiInstanceRunCountManager + GetExtensionRunningSaList 重建），强杀场景死 pid 残留未闭环（F-2，见修订日志 v6）。
- **R-008 多进程资源**：每用户一个进程，内存/CPU 占用随用户数增长。PC 已认可功耗。
- **Trade-off**：OTA 后历史任务丢失（不迁移）——PC 已认可，简化实现。
- **方案简化收益**：不需 InstanceRegistry、不需 IPC user_id 路由、不需 per-user 实例池——request 侧改造量大幅低于单进程内部路由方案。

## Crate Boundaries

**受影响的现有 crate：**
- `download_server`(services)：ability.rs（on_start 读 access token userId）、manage/account.rs（`get_user_id_from_token()` 纯 Rust libc::ioctl）、database/mod.rs（DB 路径参数化 + set_current_user_id）
- request framework native（frameworks/native/request）：multi_instance_runcount_manager.cpp（跨实例聚合）、runcount_notify_stub.cpp（OnCallBack 多实例分支）、request_running_task_count.cpp（GetCount 委托 + 首订阅触发重建）
- `rdb`(common/database)：config.rs（OpenConfig::new(path) 已支持路径参数）、wrapper.rs（cxx bridge open_rdb_store 路径透传）、database.rs（RdbStore::open(config) 已支持）

**新增 crate：** 无。跨实例聚合在 request framework native（C++）实现，不在 services crate 内。

**依赖图（无环）：**
```
samgr (SUPPORT_MULTI_INSTANCE)
  └─ MultiSystemAbilityManager(userId) → StartDynamicSystemProcess
       └─ init SetUserIdToAccessToken(userId) 打标
            └─ download_server@userId 进程
                 └─ RequestAbility.on_start_with_reason
                      └─ get_user_id_from_token()（GET_USERID ioctl 读回）
                           └─ RequestDb::get_instance(path=.../request_<userId>.db)
                                └─ RdbStore::open(OpenConfig::new(path)) → C++ RequestDataBase
```

## Async/Send/Sync Considerations

**异步边界：**
- 每进程独立 ylong_runtime（`build_global()` 在 init 里，每进程一份）——天然 per-user 隔离
- `on_remote_request` 同步 IPC 入口，分派到进程内 TaskManager（异步 channel）
- 跨实例聚合：`SubRunCount` 是同步 IPC，远端实例收到后立即 push-back 走 `OnCallBack`（同一调用栈重入 `SetCountByPid`）——`rebuilding_` 守卫防递归

**Send/Sync：**
- 进程内单例无需改 Arc<Mutex<HashMap>>（每进程独立实例，无跨线程竞争问题超出现有）
- `MultiInstanceRunCountManager` 的 `lock_` 保护 `pidCountMap_` 读写；`rebuilding_`（atomic）跨线程互斥重建

**陷阱：**
- 进程内现有 `static mut` 单例（RequestDb/TaskManager）保持不变——每进程独立，无跨进程共享
- `RestoreSubRunCount` 持锁期间不可发起同步 SubRunCount IPC（远端 push-back 会拿同一把锁死锁）——清 map 用独立小锁域，订阅在锁外执行

## Feature Flags

| Feature | 默认 | 描述 | 启用的依赖 |
|---------|------|------|-----------|
| `oh` | 是 | on-device 走 C++ RequestDataBase | system_ability_fwk, samgr, ipc, hilog_* |
| `multi-instance` | 否 | PC 产品多实例——启用 per-user DB 路径 + RSS 跨实例聚合代码路径 | （无新依赖，cfg 控制） |

**宏隔离：**
```rust
#[cfg(feature = "multi-instance")]
// per-user DB 路径 + 跨实例聚合代码

#[cfg(not(feature = "multi-instance"))]
// 单 SA 单 DB 原有路径（public/request.db）
```

**samgr 侧**：`SUPPORT_MULTI_INSTANCE` C++ 宏（PC product build 传递），控制 multi-instance 能力开关。

## Target Platforms

- **PC 产品**：`SUPPORT_MULTI_INSTANCE` + `multi-instance` feature 开启，多 SA 进程 per-user
- **非 PC 产品**（手机/平板等）：宏不开，单 SA 单 DB，行为不变
- **测试目标**：x86_64 Linux host，off-device rusqlite，cfg(test) 覆盖 per-user 内存 DB

## 切割策略

**切割维度**：基础设施 → 实现 → 集成（分层顺序）+ 依赖拓扑（自底向上）。

**理由**：本变更跨 4 层（依赖升级 → DB 路径化 → SA userId 注入 → RSS 跨实例聚合），每层依赖下层就绪。按拓扑序切割保证每 phase 独立可验证：
- Phase 1（基础设施）：依赖升级 + 宏 + profile——所有后续 phase 的前提
- Phase 2（数据层）：DB 路径参数化——Phase 3 的 userId 注入需要路径参数化就绪
- Phase 3（服务层）：SA 启动拿 userId 注入 DB 路径——依赖 Phase 2
- Phase 4（聚合层）：RSS 跨实例聚合——依赖 Phase 1（SUPPORT_MULTI_INSTANCE 宏）+ Phase 2（实例隔离基础），聚合点在 request framework native（C++），与 Phase 3 文件不重叠可并行
- Phase 5（集成层）：宏收尾 + 回归 + 端到端测试——依赖 Phase 1-4

**跨 crate 说明**：Phase 2 同时改 services 和 common/database 两个 crate（FFI 接缝，不可分割——DB 路径参数化需 Rust RequestDb + C++ DB_NAME + rdb wrapper cxx bridge 同步改）。其余 phase 均在 services crate 内。

## Phase Details

| Phase | 名称 | change | 状态 | 依赖 | 验证方式 | Wave |
|-------|------|--------|------|------|---------|------|
| 1 | multi-instance-sa-foundation | request-multi-instance-p1 | ✓ 设计完成 | 无 | cargo check（升级后编译通过） | W1 |
| 2 | per-user-database | request-multi-instance-p2 | ✓ 设计完成 | Phase 1 | cargo test（off-device per-user 内存 DB） | W2 |
| 3 | sa-startup-userid | request-multi-instance-p3 | ✓ 设计完成 | Phase 2 | cargo check + 单测 | W3 |
| 4 | cross-instance-rss | request-multi-instance-p4 | ✓ 已实现 | Phase 1, Phase 2 | cargo check + 单测 + DT | W3 |
| 5 | config-macro-integration | request-multi-instance-p5 | ✓ 设计完成 | Phase 1, 2, 3, 4 | 多实例端到端集成测试 | W4 |

### Phase 1: multi-instance-sa-foundation
- **目标**: 升级 samgr/safwk 依赖到含 multi-instance 能力的版本；SA profile 标 `"multi-instance": true`；配置 SUPPORT_MULTI_INSTANCE 宏与 Cargo `multi-instance` feature。验证升级后编译通过、Rust API 兼容。
- **预估文件**: services/Cargo.toml、services/BUILD.gn、etc/sa_profile/3706.json、Cargo.toml（workspace 根）— 约 4 文件
- **Success Criteria**:
  1. samgr 升级后 `cargo check -p download_server` 通过
  2. 3706.json 含 `"multi-instance": true` 字段
  3. services/BUILD.gn PC product 条件下 defines 含 SUPPORT_MULTI_INSTANCE、features 含 multi-instance
- **依赖**: 无
- **关键风险**: R-001 依赖升级可能伴随 Rust API 变化；R-002 宏传递需 build 系统正确配置

### Phase 2: per-user-database
- **目标**: DB 路径参数化为 `.../request_<userId>.db`（文件名带 uid，目录共享）。三处硬编码点（manage/database.rs:86、database/mod.rs:33、c_request_database.h:35）同步改。OpenConfig/wrapper cxx bridge 确认路径透传。
- **预估文件**: services/src/manage/database.rs、services/src/database/mod.rs、services/include/c_request_database.h、common/database/src/wrapper.rs、common/database/src/config.rs、services/src/cxx/c_request_database.cpp — 约 6 文件
- **跨 crate 修改**：services + common/database 两 crate（FFI 接缝，DB 路径参数化需 Rust + C++ + cxx bridge 同步，不可分割）
- **Success Criteria**:
  1. RequestDb::get_instance 接受/构造按 userId 命名的路径
  2. off-device 测试：不同 userId 打开不同内存 DB，任务互不串扰
  3. cxx bridge 路径透传验证
- **依赖**: Phase 1 完成（samgr API 就绪）
- **关键风险**: R-004 三处硬编码点必须同步；R-007 cxx bridge 路径透传

### Phase 3: sa-startup-userid
- **目标**: ability.rs `on_start_with_reason` 从 **access token** 读 userId（`manage::account::get_user_id_from_token()`，GET_USERID ioctl）注入 DB 路径。SA on-demand 加 USER_ADDED 事件（用户添加拉起实例）。生命周期继承现有 on_start/idle/active 规格。
- **预估文件**: services/src/ability.rs、services/src/manage/account.rs、services/src/database/mod.rs、etc/sa_profile/3706.json（on-demand 事件）、etc/init/downloadservice.cfg — 约 5 文件
- **Success Criteria**:
  1. on_start_with_reason 正确从 access token 读 userId 并传入 RequestDb
  2. 3706.json start-on-demand 含 USER_ADDED
  3. 单测覆盖 userId 读取与 DB 路径注入
- **依赖**: Phase 2 完成（DB 路径参数化就绪）
- **关键风险**: GET_USERID ioctl 的 SELinux xperm 白名单（download_server.te 需 `allowxperm ... ioctl { 0x410d }`）；read token 失败的降级（返回 0 回退单实例逻辑）

### Phase 4: cross-instance-rss
- **目标**: 跨实例任务数聚合（需求5）。聚合点在 **RSS 客户端侧**：`MultiInstanceRunCountManager`（frameworks/native/request）维护 `pidCountMap_`，SA 实例通过现有 SubRunCount push-back（`RunCountNotifyStub::OnCallBack` → `SetCountByPid`）上报本地任务数；`FwkRunningTaskCountManager::GetCount()` 多实例分支返回 `GetTotalCount()` 求和。第一位订阅者触发 `RestoreSubRunCount()`：`GetExtensionRunningSaList("download_server")` 枚举在跑实例 → 逐个 SubRunCount 重新订阅重填 map（`rebuilding_` 防重入）。count==0 时 erase pid。
- **预估文件**: frameworks/native/request/src/{multi_instance_runcount_manager.cpp, runcount_notify_stub.cpp, request_running_task_count.cpp}、frameworks/native/request/include/multi_instance_runcount_manager.h — 约 4 文件
- **Success Criteria**:
  1. RSS 订阅者收到所有活跃实例任务总数
  2. 新实例启动后纳入聚合，实例销毁后退出聚合
  3. DT（MultiInstanceTest）覆盖 SetCountByPid/GetTotalCount/防重入/OnCallBack 聚合/GetCount 委托
- **依赖**: Phase 1（SUPPORT_MULTI_INSTANCE 宏）、Phase 2（DB/实例隔离基础）
- **关键风险**: R-005 强杀场景死 pid 残留（F-2，未实现）；`GetExtensionRunningSaList` extension 字段依赖升级后 samgr（R-001，无法本仓验证）

### Phase 5: config-macro-integration
- **目标**: services/BUILD.gn PC product 宏收尾（defines + features）；Cargo.toml feature 隔离；非 PC 产品回归验证（宏不开行为不变）；多实例端到端集成测试。
- **预估文件**: services/BUILD.gn、Cargo.toml、services/src/lib.rs（cfg 隔离）、test/ 集成测试 — 约 5 文件
- **Success Criteria**:
  1. 非 PC 产品编译运行行为与改造前一致（回归通过）
  2. PC 产品多用户端到端：两用户各自任务隔离、DB 独立、RSS 总数正确
  3. 集成测试覆盖多实例场景
  4. 多用户并发内存 benchmark（R-008 验证：每进程内存占用随用户数线性增长可接受）
- **依赖**: Phase 1-4 全部完成
- **关键风险**: R-002 宏隔离非 PC 产品不能误开；R-008 多进程资源/SELinux 策略

## Open Questions

（实现后状态更新）

- **RSS 跨实例聚合（需求5）**：✅ 客户端侧 per-pid 聚合已实现——`MultiInstanceRunCountManager`（frameworks/native/request）维护 `pidCountMap_`，SA 实例经现有 SubRunCount push-back（`OnCallBack` → `SetCountByPid`）上报本地任务数，`GetTotalCount()` 求和，`FwkRunningTaskCountManager::GetCount()` 多实例分支返回聚合值。第一位订阅者触发 `RestoreSubRunCount()`：`GetExtensionRunningSaList("download_server")` 枚举在跑实例 + 逐个重新订阅。不依赖常驻 SA / 主实例 / subscribe_system_process / 文件共享。**已知缺口 F-2**：实例被强杀（USER_REMOVED）时无 on_stop 清 0，死 pid 残留至下次重建，未闭环（见修订日志 v6）。
- **hidumper 等基于 SA ID 的运维区分**：🟡 已知限制——所有实例 SA ID 都是 3706，`GetSystemAbility(3706)` 只返回调用方所在用户的实例，`hidumper -s 3706` 无法区分/枚举所有用户实例。samgr multi-instance 进程拉起就绪但 SA ID 级运维路由未跟上。待 samgr 后续支持（如 `hidumper --user <uid> -s 3706`）。
- **进程命名规则**：🟡 samgr `StartDynamicSystemProcess` 的进程命名/SELinux 策略待 PC product 设备验证。

## 修订日志

| 日期 | 版本 | 变更 | 触发 |
|------|------|------|------|
| 2026-08-03 | v1 | 初版方案——单 SA 进程 + 内部 InstanceRegistry 路由（D1）+ per-user DB 分库 + DB 迁移 | Step 1-5 首轮生成 |
| 2026-08-03 | v2 | **方案转向**：从"单 SA + 内部 InstanceRegistry"改为"多 SA 进程 + samgr multi-instance"（D1 重写）。起因：用户提供 6 条需求澄清——PC 多用户需独立 SA 实例进程 + 宏隔离。调研 samgr 上游确认已支持 per-user 多 SA 进程。方案简化：不需 InstanceRegistry/IPC user_id 路由/per-user 实例池（进程即用户，单例保持进程内）。DB 改为文件名带 uid。OTA 不迁移。新增 RSS 跨实例聚合。Phase 从 4 拆为 5。 | 用户 6 条需求澄清 + samgr 调研 |
| 2026-08-03 | v3 | 宏隔离位置修正为 services/BUILD.gn；DB 路径格式改为 `request_<uid>.db`；补架构概览/切割策略/修订日志节（Step 7 审计） | 用户指正 + Step 7 master-plan 审计 |
| 2026-08-04 | v4 | **依赖方向纠正**：`db_path`/userId 存储从 `manage/database.rs` 移到 `database/mod.rs`（manage→database 正确方向）。GN 不允许跨仓 import gni，改用本地 `request_multi_instance` 变量。samgr/safwk 本地源码树 git pull 升级。 | 用户指正依赖反向 + 编译验证 |
| 2026-08-04 | v5 | **RSS 聚合方案确定（旧，已废弃）**：服务端文件聚合——每实例写 run_count/\<userId\> 共享文件 + 求和 + 推 total 给本实例订阅者。去"主实例"概念。aggregator.rs 原子写 + 求和，on_stop 清 0。该方案随后被客户端 per-pid 聚合取代（见 v6）。 | 用户指正无主实例 + SA 非常驻 + 只改 request 仓 |
| 2026-08-04 | v6 | **RSS 聚合方案重写为客户端 per-pid 聚合**：废弃 v5 文件聚合。聚合点移到 RSS 客户端侧 `MultiInstanceRunCountManager`（frameworks/native/request C++），复用现有 SubRunCount push-back 回调路径上报（无新增跨进程 IPC）；`RestoreSubRunCount` 用 `GetExtensionRunningSaList` 枚举 + 重新订阅重建，第一位订阅者触发；`SetCountByPid` count==0 时 erase pid；`rebuilding_` atomic 防重入。**遗留缺口 F-2**：实例强杀（用户注销）无 on_stop 清 0，死 pid 残留至下次重建——ability.rs 无 on_stop、RunCountManager 无关机清零推送，SA 级 OnRemoveSystemAbility 仅清单实例 count_。补齐方向（未实现）：ability.rs 增加 on_stop 推送 count=0，或客户端实例卸载时定向 erase。spec 的 subscribe_system_process SHALL 已同步更新为客户端 per-pid 聚合。 | 代码评审 F-2/F-3（设计文档与实现不符） |
| 2026-09-08 | v7 | **评审闭环 + 测试补强**：① F-1 unsafe 补 // SAFETY 契约注释；② F-5 兜底路径 user_id_initialized 运行时检查/日志，后按用户要求移除改纯注释；③ F-7 RestoreSubRunCount 返回 int32_t 错误码，多实例订阅路径不再吞错（与单实例 SubRunCount 契约一致）；④ F-8 新增 ut_multi_instance_db_physical_isolation 真实打开两个 per-user DB 验证任务互不串扰（原来只断言路径字符串）；⑤ F-9 libc 依赖收窄到 request_multi_instance（BUILD.gn 条件 + Cargo optional）；⑥ F-11 移除依赖 /dev/access_token_id 环境假设的脆弱 UT；⑦ 高频日志降级（count change / GetTotalCount 由 INFO 降 DEBUG）；⑧ 需求注释大幅精简；⑨ RestoreSubRunCount 拆出 RebuildFromSaList 解耦 SAM 枚举、UT 空列表覆盖重建逻辑；⑩ 新增 DT 覆盖 OnCallBack 多实例聚合、Fwk GetCount 委托。 | 代码评审 F-1/F-5/F-7/F-8/F-9/F-11 + 用户反馈 |

