# feature-service.md — 服务端 download_server 领域知识

> 本文档为 Agent 知识路由目标，由 AGENTS.md 的 task-based / path-based / vocabulary-based routing 触发阅读。
> Agent 能自己探索出来的少写；Agent 猜不准、猜错代价高、团队必须统一执行的内容要写。

## 1. 代码地图

本文档适用于 `services/` 目录（服务端 Rust 实现，SA 3706 `download_server`）。

本服务实现下载/上传任务的全部业务逻辑：任务生命周期管理、QoS 调度、HTTP 传输、断点与恢复、rdb 持久化、通知栏进度、运行计数。最重要的架构边界是**三层职责分离**：`service/`（IPC 接入层）只做协议解析与命令分发；`manage/`（管理层）持有事件循环与调度决策；`task/`（执行层）做 HTTP 收发与文件 IO。跨这三层的调用一律经 `TaskManager` 事件通道，不直接函数调用。

### 关键区域

- `services/src/ability.rs`：SA 定义与启动。`#[link_section = ".init_array"]` 自注册；`init()` 顺序：panic hook → ylong_runtime（4 worker）→ RunCountManager → ClientManager → SystemConfigManager → TaskManager → 订阅 APP_MGR → 发布 stub 到 samgr。
- `services/src/service/stub.rs`：`RequestServiceStub`（`RemoteStub` 实现），按 `interface.rs` 的命令码路由到 `service/command/` 下对应模块；含 `serialize_task_info`/`serialize_task_config` 手动逐字段序列化。
- `services/src/service/interface.rs`：IPC 命令码常量（`CONSTRUCT=0` … `SET_MAX_SPEED=21`、`SET_MODE=100`、`DISABLE_TASK_NOTIFICATION=101`），跨进程协议。
- `services/src/service/command/`：每个 IPC 码一个 handler 模块。
- `services/src/service/client/`：UDS 数据面。`ClientManager` 维护客户端连接与 task→client 订阅映射；`client/mod.rs` 定义消息格式（魔数 `0x43434646`，类型 HttpResponse/NotifyData/Faults/Waiting）。
- `services/src/manage/task_manager.rs`：`TaskManager` 事件循环主控；`TaskManagerTx`/`Rx` 是全服务事件总线。
- `services/src/manage/events/`：事件定义（`ServiceEvent`/`StateEvent`/`TaskEvent`/`ScheduleEvent`/`QueryEvent`）及各事件处理。
- `services/src/manage/scheduler/`：QoS 调度器。`mod.rs`（排序与状态变更）、`qos/`（应用/方向/RSS 三维优先级）、`queue/`（RunningQueue、SAKeeper 30s 空闲卸载）、`state/`（网络/前后台/账户状态联动）。
- `services/src/manage/database.rs`：`RequestDb` 任务持久化单例（OH 路径持 C++ `RequestDataBase*`，非 OH 用 rusqlite）。
- `services/src/task/request_task.rs`：`RequestTask` 核心结构体（conf/client/files/progress/status + 原子量）。
- `services/src/task/client.rs`：`build_client()`——构建 `ylong_http_client::Client`（超时、TLS≥1.2、重定向、代理、证书、域名策略、公钥钉扎）。
- `services/src/task/download.rs` / `upload.rs` / `operator.rs`：下载/上传执行与共用的进度上报/限速（前台进度 1s、通知栏 500ms）。
- `services/src/task/files.rs`：`Files`/`AttachedFiles`——文件打开与安全校验链（`O_NOFOLLOW`+`O_CLOEXEC`+`/proc/self/fd` 域校验）。
- `services/src/service/notification_bar/`：通知栏进度（`NotificationDispatcher` 单例 + `NotificationDb` + cxx 桥到 C++ AnsInnerkits）。
- `services/src/cxx/`：16 个手写 C++ 桥接实现（`c_request_database.cpp` 是 DB 核心）。
- `services/src/database/db_monitor.rs`：`REQUEST_DB: LazyLock<RdbStore>`（Rust 句柄）——DB 监控/清理/WAL checkpoint。

### 查找位置

- 任务生命周期变更 → `manage/events/` + `manage/scheduler/mod.rs`（状态机在 `change_status`）
- 下载/上传传输行为变更 → `task/download.rs`/`upload.rs`/`operator.rs` + `task/client.rs`
- 文件路径处理变更 → `task/files.rs`（先读本文件 §3.6 安全链）
- 持久化变更 → `manage/database.rs` + `cxx/c_request_database.cpp` + `include/c_request_database.h`
- IPC 新增接口 → `service/interface.rs` + `service/stub.rs` + `service/command/`（双端，见 feature-client.md）
- 通知栏行为 → `service/notification_bar/` + `cxx/notification_bar.cpp`
- 空闲卸载/恢复 → `manage/scheduler/queue/keeper.rs`（SAKeeper）+ `ScheduleEvent`

## 2. 知识路由

### 按任务路由

- 状态机/调度变更 → 读 `manage/scheduler/mod.rs`（`change_status`/`reschedule`）+ 本文件 §3.3
- 持久化/表结构变更 → 读 `manage/database.rs` + `cxx/c_request_database.cpp` + 本文件 §3.4（双句柄）
- UDS 推送变更 → 读 `service/client/mod.rs`（`handle_send_notify_data`）+ feature-client.md §UDS 协议
- 通知栏变更 → 读 `service/notification_bar/publish.rs` + `notify_flow.rs`
- SA 生命周期变更 → 读 `ability.rs` + `scheduler/queue/keeper.rs` + 本文件 §3.2
- HTTP 行为/重试/超时 → 读 `task/client.rs` + `task/download.rs` + `task/http_error_registry.rs`
- 错误码变更 → 读 `task/reason.rs`（Reason）+ `error.rs`（ErrorCode）+ feature-common.md（错误码表）

### 按路径路由

- `services/src/service/` → 本文件 §3.1（IPC 层）+ §3.5（active_counter）
- `services/src/manage/` → 本文件 §3.3（调度）
- `services/src/task/` → 本文件 §3.6（文件安全）+ §3.7（限速/进度）
- `services/src/cxx/` → 本文件 §3.8（cxx 桥约定）

### 按词汇路由

| 术语 | 风险提示 | 阅读 |
|---|---|---|
| TaskManager | 事件循环主控；跨层调用走 `TaskManagerTx` 事件，勿直接函数调用 | `manage/task_manager.rs` |
| TaskManagerEvent | 全部事件的枚举入口（Service/State/Task/Schedule/Query） | `manage/events/mod.rs` |
| Scheduler / Qos | QoS 三维优先级（应用/方向/RSS）；状态变更产出 SQL | `manage/scheduler/` |
| RunningQueue | 下载队列 + 上传队列；执行分发按 `Action` | `manage/scheduler/queue/` |
| SAKeeper | 30s 空闲倒计时后卸载 SA；`active_counter` 非零则不卸载 | `queue/keeper.rs` |
| RequestTask | 任务运行时对象；含大量原子量，跨线程访问看字段注释 | `task/request_task.rs` |
| RequestDb | 任务持久化单例；OH 持 C++ 指针，非 OH 用 rusqlite | `manage/database.rs` |
| RequestDataBase | C++ DB 实现类，经 `GetDatabaseInstance` cxx 桥获取 | `cxx/c_request_database.cpp` |
| REQUEST_DB | **另一个** Rust rdb 句柄（LazyLock），监控/清理/WAL 用 | `database/db_monitor.rs` |
| ClientManager | UDS 客户端管理；task→client 订阅映射 | `service/client/manager.rs` |
| ActiveCounter | IPC 活跃计数；increment/decrement 必须严格配对 | `service/active_counter.rs` |
| build_client | HTTP 客户端工厂；证书/代理/钉扎都在这配 | `task/client.rs` |
| Mode | FrontEnd/BackGround；模式切换联动 QoS 与通知栏 | `task/config.rs` |
| user_file_tasks | 必须常驻内存的任务缓存（fd 不可序列化） | `manage/database.rs` |

在计划中声明：任务类别、已读文档、发现的约束、是否应使用特定 Skill/工作流。

## 3. 约束与边界

### 架构/领域不变量

- 三层间调用走事件通道（`TaskManagerTx`），`service/` 不得直接调 `task/`，`task/` 通过 `TaskEvent` 上报。
- 异步原语只用 ylong_runtime：`ylong_runtime::sync::mpsc`/`oneshot`、`ylong_runtime::spawn`、`ylong_runtime::block_on`。禁止 tokio。
- Rust 不直接调用系统服务；通知/网络/账户/权限/bundle 一律 cxx 桥到 `src/cxx/`。
- IPC 码、`serialize_task_info`/`serialize_task_config` 字段顺序、UDS 消息格式是协议，双端同步。
- 任务状态/原因码（State/Reason/ErrorCode）数值只增不改。

### 3.1 IPC 层

`RequestServiceStub::on_remote_request`（`stub.rs`）：`cancel_idle` → `active_counter.increment` → 校验 token `"OHOS.Download.RequestServiceInterface"` → 按 `interface.rs` 命令码路由 → decrement。任何新增分支的早退路径都必须 decrement。

`service/command/` 每个命令模块负责从 MsgParcel 反序列化参数，转成 `ServiceEvent`/`QueryEvent` 发给 TaskManager，`block_on` 等 oneshot 回执。

### 3.2 SA 生命周期

- 注册：`.init_array` 自注册 `RequestAbility::new().build_system_ability(DOWNLOAD_SERVICE_ID, false)`。
- 空闲卸载：`on_idle` 检查 `active_counter.is_active()`，空闲则发 `ScheduleEvent::Shutdown`；SAKeeper 30s 倒计时。
- 启动恢复：延迟 10s `restore_all_tasks` 从 DB 恢复任务。
- SA ID 注意：注册用 `samgr::definition::DOWNLOAD_SERVICE_ID`；卸载用硬编码 `3706`（`task_manager.rs`）；`cxx/notification_bar.cpp` 里 `REQUEST_SERVICE_ID = 3815` 是**通知创建者标识**，与 SA ID 无关，勿混淆。

### 3.3 任务状态机与调度

State 枚举（`task/info.rs`）：

```
Initialized(0x00) → Waiting(0x10) → Running(0x20) → Completed(0x40)
                   ↕                ↕
                   Paused(0x30)   Retrying(0x21) → Failed(0x41)
                   ↓
                   Stopped(0x31) / Removed(0x50)
```

- 状态变更由 `Scheduler::change_status` 生成 SQL 执行并回验（`scheduler/mod.rs`）。
- 调度：`Scheduler::reschedule` 按 QoS（应用排序/下载上传方向/RSS 等级）产出变更集，`RunningQueue` 执行。
- `Mode::FrontEnd`/`BackGround` 切换会更新 QoS 并联动通知栏（前台 `unregister_task`，后台 `enable_task_progress_notification`）。
- 运行中任务状态事件（Completed/Failed/Running/Offline）经 `TaskEvent` → TaskManager → `Notifier` → `ClientManager` → 订阅客户端。

### 3.4 持久化（双句柄，易踩坑）

同一数据库文件 `/data/service/el1/public/database/request/request.db`（加密，S1）存在**两个独立句柄**：

| 句柄 | 位置 | 用途 |
|---|---|---|
| `RequestDb`（C++ `RequestDataBase*`） | `manage/database.rs` | 任务 CRUD、状态/进度更新 |
| `REQUEST_DB`（Rust `RdbStore`，LazyLock） | `database/db_monitor.rs` | DB 监控、按状态清理、WAL checkpoint |

- 表结构：主表 `request_task`（40+ 列，`form_items`/`file_specs`/`each_file_status`/`body_file_names`/`certs_paths` 为 BLOB，BLOB 占存量约 63%），`request_version` 管理库版本；通知栏另有 `task_config`、`group_notification*` 表（`notification_bar/database.rs`）。
- 维护时机：SA 卸载前 `database_maintenance`（>10MB 上报、Removed 立即清、Completed 超 1 天清、其他超 7 天清、WAL checkpoint）；每 30 分钟 `clear_timeout_tasks`。
- `execute_sql` 与 `execute` 是不同方法：只有前者能接受 PRAGMA（见 `database/mod.rs` 注释）。
- 修改表结构必须处理存量数据迁移并更新 `request_version`。

### 3.5 UDS 数据面

- `OPEN_CHANNEL` IPC 创建 `UnixDatagram::pair()`，一端 fd 回传客户端。
- 消息：魔数 `0x43434646` + msg_id + msg_type + length（固定偏移 10）+ body；headers 上限 8KB（`HEADERS_MAX_SIZE`）。
- 进度去重：同 task 只保留最后一条 progress；发送后等 500ms ACK。
- `handle_send_notify_data`（`client/mod.rs`）序列化最复杂，含 API9/API10 版本差异，改动必须同步客户端解析（feature-client.md §UDS）。

### 3.6 文件安全校验链（最高优先级）

`task/files.rs` 的安全模型：

- 打开文件用 `O_NOFOLLOW | O_CLOEXEC`，但 `O_NOFOLLOW` 只防最终组件的符号链接，**中间目录的链接仍会被跟随**。
- 因此打开后必须经 `/proc/self/fd` 校验真实路径在应用 base 目录内（`verify_within_base`）。
- 新增任何文件打开路径都必须走该校验链；历史上 `body_file_paths` 出过实锤穿越漏洞。
- **arm32 常量坑**：`O_NOFOLLOW` 数值在 arm32（RK3568 用户态 32 位）与 x86_64 不同（arm32 上该位实际是 `O_LARGEFILE`），手写内核标志必须按 target_arch 分支；x86_64 本地测试通过不代表设备正确。

### 3.7 传输执行

- HTTP 客户端：`ylong_http_client`（features: async、c_openssl_3_0、http1_1），TLS 为 OpenSSL 3.0。注意：**不是** curl，也**不是** `common/netstack_rs` 封装的 `netstack:http_client`（那是 preload 特性用的）。
- 下载实现 `DownloadOperator`，上传实现 `UploadOperator`（`TaskReader` 流式读文件）。
- 进度节奏：前台 1s（`FRONT_NOTIFY_INTERVAL`，`operator.rs`）、通知栏 500ms（`NOTIFY_PROGRESS_INTERVAL`，`publish.rs`）。
- 限速：`task/speed_limiter.rs`，由 `SET_MAX_SPEED` IPC 设置。

### 3.8 cxx 桥约定

- 桥定义分散在 `config.rs`（Action/Mode）、`info.rs`（State）、`reason.rs`（Reason）、`notification_bar/mod.rs`、`utils/mod.rs`、`manage/database.rs`、`task_manager.rs`，统一命名空间 `OHOS::Request`。
- `BUILD.gn` 的 `rust_cxx("download_server_cxx_gen")` 从 10 个 Rust 文件生成桥代码；手写实现放 `src/cxx/`，头文件放 `include/`。
- 修改任何 cxx::bridge 后必须重新全量编译（生成代码会变）。

### 禁止事项

- NEVER 混用 tokio 与 ylong_runtime 原语（channel/spawn/block_on 运行时不兼容）。
- NEVER 新增 IPC 早退路径时漏掉 `active_counter.decrement()`（下溢后 SA 永不卸载）。
- NEVER 绕过 `files.rs` 校验链直接 `File::open`/`std::fs` 操作客户端可控路径。
- NEVER 修改 State/Reason/ErrorCode/IPC 码的既有数值。
- NEVER 在 OH 环境外验证通过就认定设备行为正确（DB/IPC/通知在非 OH 下是 mock）。
- NEVER 长期持有跨请求的 DB 事务；`RequestDb` 操作粒度是单语句。
- NEVER 删除 HiSysEvent 上报（`sys_event!` 宏，DfxCode 定义在 `common/sys_event`）。

### 已知陷阱

**P1: 全局 `static mut` + `MaybeUninit`** — `ability.rs`（SYSTEM_CONFIG_MANAGER）、`manage/database.rs`（DB）、`lib.rs`（DB_LOCK）用 `static mut MaybeUninit` 存全局状态，poison 后甚至重赋值。修改初始化顺序或并发访问路径极易 UB；新增全局状态优先用 `LazyLock`/`OnceLock`。

**P2: `block_on` 死锁风险** — `TaskManagerTx::show/query/touch` 在 IPC 线程 `block_on(rx)` 等事件循环回执。若事件循环被同线程任务阻塞即死锁。新增同步查询接口前确认回执不依赖当前线程。

**P3: 双 DB 句柄并发** — C++ `RequestDataBase` 与 Rust `REQUEST_DB` 是同文件的两个连接。跨句柄的写+读可能读到旧数据；清理逻辑在 Rust 句柄上，任务写在 C++ 句柄上。

**P4: `user_file_tasks` 必须常驻内存** — fd 无法序列化，`RequestDb` 缓存这些 `Arc<RequestTask>`。SA 卸载后 fd 全部丢失，恢复时重开文件。改任务内存生命周期前先确认不破坏该约束（否则文件写入位置错乱）。

**P5: 清理循环可被打断** — `database_maintenance` 分批删（每批 1000，最多 10 批），中途有任务开始运行会提前返回且不 checkpoint WAL。

**P6: 非 OH 路径与 OH 路径行为漂移** — DB（rusqlite vs rdb）、IPC（mock vs samgr）、通知（无 vs AnsInnerkits）在两条 feature 路径上是两套实现；`manage/database.rs` 非 OH 分支甚至存在已知的编译问题代码。以 OH 路径为准做行为判断。

**P7: 通知栏事件回传方向** — 通知栏按钮（暂停/恢复/停止）经 `TaskManagerWrapper`（cxx 桥 Rust→C++ 暴露方法）回调 TaskManager，不是走 IPC。

**P8: Reason 枚举值有空洞** — `Reason` 数值不连续（0,1,4,5..8,10..12,14..19,21,23..,27..31），`From<u8>` 未识别值回落 `OthersError`；新增值不得填洞。

### 询问后再做

- 变更 `request_task` 表结构或数据库版本。
- 变更 IPC 命令码、序列化字段顺序、UDS 消息格式。
- 变更 SA 空闲卸载策略（30s/10s 恢复延迟等常量）。
- 变更数据库清理策略（1 天/7 天/10MB 阈值）。
- 新增 cxx 桥目标系统服务依赖。

## 4. 验证

### 最小检查

- 构建服务端：`./build.sh --product-name rk3568 --build-target out/rk3568/build_configs/request/request:request --no-indep`
- Rust UT：构建 `request_test` target，Rust 用例位于 `services/tests/ut/`（target `rust_request_ut_test`），详见 feature-test.md
- 设备端功能：Rust UT 需设备/模拟环境（依赖 ylong_runtime 与系统服务 mock）

### 任务级检查

- 状态机/调度变更 → `ut_task_manager`/`ut_sql` + `requestAgentTaskTest`（JS）生命周期用例
- 持久化变更 → `ut_database` + 设备端建库/恢复验证（卸载 SA 后重启恢复）
- UDS 变更 → `ut_client` + JS 用例（进度/事件回调），双端代码同查
- 通知栏变更 → `ut_notify_flow` + 人工设备验证（通知栏实时进度与操作按钮）
- HTTP 行为变更 → `common_netstack_test`（如涉及 netstack_rs）+ JS 下载/上传用例
- DFX 变更 → 构建 + `hisysevent` 输出核对，禁止删除既有事件

### 完成定义

- 请求的行为已实现。
- 相关构建/测试/lint/兼容性检查已执行，或已说明无法执行的原因。
- 最终回复包含：变更摘要、变更文件列表、验证结果、剩余风险。
- 不包含无关的格式化、重构或附带变更。
- 新增/修改的服务端接口有对应 UT；对外新增接口有 fuzz 覆盖。

## 5. 关键文件索引

| 文件 | 职责 |
|---|---|
| `services/src/ability.rs` | SA 定义、`.init_array` 注册、init 顺序 |
| `services/src/service/stub.rs` | IPC Stub 路由 + task_info/task_config 序列化 |
| `services/src/service/interface.rs` | IPC 命令码（0-22, 100-101） |
| `services/src/service/command/` | 每个 IPC 码的 handler 实现 |
| `services/src/service/client/mod.rs` | UDS 消息定义与序列化（魔数 0x43434646） |
| `services/src/service/client/manager.rs` | ClientManager、订阅映射 |
| `services/src/service/run_count/` | 运行任务计数订阅推送 |
| `services/src/service/notification_bar/` | 通知栏调度（Dispatcher/Db/Flow/进度类型） |
| `services/src/service/active_counter.rs` | 活跃计数（配对增减，防 SA 误卸载） |
| `services/src/manage/task_manager.rs` | 事件循环主控 |
| `services/src/manage/events/` | 全部事件定义与处理 |
| `services/src/manage/scheduler/` | QoS 调度（qos/queue/state） |
| `services/src/manage/database.rs` | RequestDb 单例（C++ 路径） |
| `services/src/manage/network*.rs`、`app_state.rs`、`account.rs` | 网络/前后台/账户状态源 |
| `services/src/manage/config/` | SystemConfigManager、证书、系统代理 |
| `services/src/task/request_task.rs` | RequestTask 运行时对象 |
| `services/src/task/client.rs` | ylong_http_client 构建（TLS/代理/钉扎） |
| `services/src/task/files.rs` | 文件打开与安全校验链 |
| `services/src/task/download.rs`/`upload.rs`/`operator.rs` | 传输执行与共用进度/限速 |
| `services/src/task/config.rs`/`info.rs`/`reason.rs` | 任务配置/状态/原因（cxx 桥枚举） |
| `services/src/database/db_monitor.rs` | REQUEST_DB（Rust rdb 句柄）、清理、WAL |
| `services/src/cxx/c_request_database.cpp` | C++ DB 实现（含损坏重建、10 次重试） |
| `services/src/cxx/notification_bar.cpp` | C++ 通知发布（AnsInnerkits） |
| `services/include/c_request_database.h` | `request_task` 建表 SQL 与表结构 |

### 注意事项 / 外部依赖

- `ylong_http_client`、`ylong_runtime` 来自 commonlibrary_rust 仓（OpenHarmony 自研），版本由集成仓锁定。
- `ipc`/`samgr`/`system_ability_fwk` 是 OpenHarmony Rust IPC 框架 crate；`DOWNLOAD_SERVICE_ID` 取自 `samgr::definition`。
- 通知（distributed_notification_service）、网络管理（netmanager_base）、账户（os_account）、权限（access_token）等系统服务调用全部在 `src/cxx/` C++ 侧，Rust 侧不可见其头文件。
