# AGENTS.md

## 基本信息

| 属性 | 值 |
| ----- | ----- |
| 代码仓名称 | request |
| 所属子系统 | request（组件 `@ohos/request`） |
| SysCap | SystemCapability.MiscServices.Download / SystemCapability.MiscServices.Upload / SystemCapability.Request.FileTransferAgent |
| 主要语言 | Rust（服务端核心）、C++（IPC 桥接与旧客户端）、ArkTS/Cangjie（绑定层） |
| 服务形态 | System Ability 3706（`download_server`，按需启动） |

## 1. 代码地图

本 AGENTS.md 适用于仓库根目录。特性级规则在嵌套指令文件中，任务命中对应类别时必须在规划前阅读。

嵌套指令文件：

- `docs/agent/feature-service.md` — 服务端 download_server（任务生命周期、QoS 调度、持久化、通知栏）
- `docs/agent/feature-client.md` — 客户端框架与多语言绑定（NAPI/ANI/Cangjie、IPC+UDS 协议）
- `docs/agent/feature-preload.md` — 缓存下载与预加载（cache_core / cache_download）
- `docs/agent/feature-common.md` — 公共库（request_core 类型中枢、netstack_rs、database 等）
- `docs/agent/feature-test.md` — 测试体系与执行方法（start.sh / testfwk / fuzz）

本仓库实现 OpenHarmony **Request 子系统**，为三方与系统应用提供文件下载、上传能力（`request.agent` / 旧版 `request.download`、`request.upload`），并提供预加载缓存能力（`request.cacheDownload`）。最重要的架构边界是**客户端-服务端分离**：任务调度、HTTP 收发、持久化、通知栏等全部业务在服务端 SA（Rust）内；客户端库只做 IPC 代理与事件回传。

**四层结构**（自上而下）：

- **API 绑定层** `frameworks/js|ets|cj/`：NAPI（JS/ArkTS）、ANI（ArkTS）、Cangjie FFI 三套绑定，把 `@ohos.request` 暴露给各语言
- **客户端框架层** `frameworks/native/`：`request`（C++ 客户端，当前生产主路径）、`request_next`（Rust 客户端，迁移中）、`request_action`（inner_kits 封装）
- **服务端** `services/`：`download_server` SA，Rust 实现——事件驱动 TaskManager、QoS 调度、rdb 持久化、通知栏进度
- **公共库** `common/`：跨层共享 crate——`request_core`（类型中枢）、`utils`、`netstack_rs`、`ffrt_rs`、`database`（rdb FFI）、C++ 的 `sys_event`/`utf8_utils`

关键路径：

| 路径 | 职责 | 变更风险 |
|---|---|---|
| `services/src/service/` | IPC Stub、command 分发、UDS 通道、通知栏、运行计数 | 接口层，影响全部客户端 |
| `services/src/manage/` | TaskManager 事件循环、QoS 调度器、RequestDb、网络/账户/前后台状态 | 服务端核心状态机 |
| `services/src/task/` | RequestTask、下载/上传执行、文件安全校验、限速 | 高频修改，核心传输路径 |
| `services/src/cxx/` | Rust↔C++ 桥接实现（DB/通知/网络/账户等系统服务） | 平台能力边界 |
| `frameworks/native/request/` | C++ 客户端（RequestManager/Proxy/UDS 接收） | 被 NAPI/旧ANI/CJ/request_action 依赖 |
| `frameworks/native/request_next/` | Rust 客户端（仅被新 ANI 使用，cxx 桥大部分注释） | 迁移中，勿假设功能完整 |
| `frameworks/native/cache_download/`、`cache_core/` | 预加载缓存（与主下载不共享任务模型） | 独立特性域 |
| `common/request_core/` | 跨层类型定义（TaskConfig/TaskInfo/State/错误码/IPC 码） | 与 services 内平行定义必须同步 |
| `common/utils/` | cfg 宏、日志、哈希、LRU、task_id、文件校验、MockHttpServer | 被全部 Rust crate 依赖 |
| `etc/` | SA profile（3706.json）、init rc/cfg、系统参数 | 服务启动、SELinux 上下文、权限 |
| `test/` | 全部测试（cpp/js/fuzz/rust） | 见 feature-test.md |

Where to look：

- 下载/上传行为变更 → `services/src/task/` + `docs/agent/feature-service.md`
- 任务调度、状态机、恢复逻辑 → `services/src/manage/scheduler/` + `docs/agent/feature-service.md`
- IPC 接口变更 → `services/src/service/interface.rs` + `frameworks` 两侧 proxy + `docs/agent/feature-client.md`
- JS/ArkTS API 行为变更 → `frameworks/js/napi/request/` + `docs/agent/feature-client.md`
- 预加载/缓存行为变更 → `frameworks/native/cache_download/` + `docs/agent/feature-preload.md`
- 共享类型、错误码变更 → `common/request_core/` + `docs/agent/feature-common.md`
- 测试编写与执行 → `test/` + `docs/agent/feature-test.md`
- 服务启动、权限、SELinux → `etc/` + `services/src/ability.rs`
- 编译配置、产物清单 → `bundle.json` + 本文件 §4

## 2. 知识路由

以下文档不是可选背景阅读。当任务命中对应类别时，必须在规划前阅读匹配文档。

### 按任务路由

- 服务端任务行为、调度、持久化、通知栏变更 → `docs/agent/feature-service.md`
- 客户端库、NAPI/ANI/CJ 绑定、IPC/UDS 协议变更 → `docs/agent/feature-client.md`
- 预加载、缓存下载行为变更 → `docs/agent/feature-preload.md`
- 公共 crate（request_core/utils/database/netstack_rs）变更 → `docs/agent/feature-common.md`
- 编写、编译或执行任何测试 → `docs/agent/feature-test.md`

### 按路径路由

- `services/` → `docs/agent/feature-service.md`
- `frameworks/native/request/`、`frameworks/native/request_next/`、`frameworks/js/`、`frameworks/ets/`、`frameworks/cj/` → `docs/agent/feature-client.md`
- `frameworks/native/cache_download/`、`frameworks/native/cache_core/`、`interfaces/inner_kits/cache_download/` → `docs/agent/feature-preload.md`
- `common/` → `docs/agent/feature-common.md`
- `test/`、`services/tests/` → `docs/agent/feature-test.md`

### 按词汇路由

当任务、issue、日志、API 名称或变更文件中出现以下术语时，在规划前阅读链接文档：

| 术语 | 风险提示 | 阅读 |
|---|---|---|
| ylong_runtime | 服务端唯一异步运行时，禁止混入 tokio 原语 | `feature-service.md` |
| SA 3706 / download_server | 服务端 SA，on-demand 按需启动与空闲卸载 | `feature-service.md` |
| OPEN_CHANNEL / UDS | IPC 之外的数据面通道；魔数 `0x43434646` 的手写二进制协议，双端必须同步 | `feature-service.md` + `feature-client.md` |
| TaskConfig / TaskInfo | 多处平行定义（request_core / services / C++ Parcel），字段与枚举值是跨进程协议 | `feature-common.md` |
| RequestDb / request_task 表 | 任务持久化；同一 db 文件存在 C++ 与 Rust 两个句柄 | `feature-service.md` |
| cxx bridge | Rust↔C++ 唯一通道，统一命名空间 `OHOS::Request` | `feature-service.md` |
| cfg_oh! / feature "oh" | 平台门控宏；OH 与非 OH 是两套代码路径，改一侧要检查另一侧 | `feature-common.md` |
| version_script（*.map） | so 符号导出白名单；新增对外函数漏加即 dlopen 失败 | `feature-client.md` |
| QoS / Scheduler / RunningQueue | 任务优先级调度体系 | `feature-service.md` |
| State / Reason / ErrorCode | 任务状态与原因码，数值是 IPC 协议，只增不改 | `feature-service.md` + `feature-common.md` |
| request_native / request_next | 新旧两套客户端并存，改 IPC 接口需同步两侧 | `feature-client.md` |
| preload / cacheDownload | 预加载缓存特性，独立于主下载任务模型 | `feature-preload.md` |
| start.sh | JS/HAP 测试唯一合法执行入口 | `feature-test.md` |
| MockHttpServer | 主机端 Rust 单测内置 HTTP 服务器 | `feature-test.md` |

在计划中声明：

- 任务类别
- 已读文档
- 发现的约束
- 是否应使用特定 Skill/工作流（如 fuzz 整改参见 `.claude/skills/fuzz-api-check`）

## 3. 约束边界

### 架构不变量

- 业务逻辑只在服务端：客户端库不做调度、重试、持久化决策，只做 IPC 代理与事件回传。
- 服务端异步原语只用 ylong_runtime（channel/spawn/block_on），禁止引入 tokio/async-std。
- Rust 访问系统服务（通知、网络管理、账户、权限、bundle 等）必须经 cxx 桥到 `services/src/cxx/` 的 C++ 实现，不得直接 FFI 系统头文件。
- `services/src/service/interface.rs` 的 IPC 接口码与 `stub.rs`/客户端 proxy 的字段序列化顺序是跨进程协议，任何变更必须双端同步并评估兼容性。
- UDS 消息格式（魔数、消息类型、字段顺序）是自有二进制协议，C++ 客户端与 Rust 客户端两侧解析代码必须同步修改。
- `TaskConfig`/`TaskInfo`/`State`/`Reason`/`SubscribeType` 的枚举数值是 IPC 协议常量，只追加不修改不复用。
- 所有跨语言 so 的对外符号必须列入对应 version_script `.map` 文件。
- 平台相关代码用 `cfg_oh!`/`cfg_not_oh!`（或 `#[cfg(feature = "oh")]`）门控，OH 与非 OH 两条路径都要保证可编译。

### Do not

- 不要绕过 `services/src/task/files.rs` 的文件安全校验链（`O_NOFOLLOW` + `/proc/self/fd` 域校验）直接操作路径；路径穿越是本模块最高优先级安全问题（历史实锤过 `body_file_paths` 穿越）。
- 不要修改 IPC 接口码数值、序列化字段顺序、UDS 消息格式，除非任务明确要求并同时更新双端。
- 不要在非 OH 测试通过后直接认定 OH 设备行为正确（平台门控下是两套实现，如 DB 一个走 rdb 一个走 rusqlite）。
- 不要在 IPC handler 中新增早退路径而漏掉 `active_counter` 的 decrement（下溢会使 SA 永不空闲卸载）。
- 不要为通过测试删除日志、HiSysEvent 事件、错误码或诊断信息。
- 不要在异步回调中持有裸 `this`/长生命周期借用（UAF 风险）。
- 不要引入新的生产依赖（Cargo.toml / bundle.json deps），除非获得明确批准。
- 不要变更公共 API（d.ts 语义）、权限行为、错误码数值（除非任务明确要求）。
- 不要假设 `request_next` 功能完整：其 cxx 桥大部分被注释，当前生产主路径是 C++ 的 `request_native`。
- 不要在 32 位 ARM（RK3568）上硬编码内核常量（如 `O_NOFOLLOW` 的值在 arm32 与 x86_64 不同），必须按 target 区分。
- 不要直接修改生成文件（cxx 生成代码、abc 字节码），修改 source of truth 后重新生成。

### Agent 典型失败模式（对照知识类型自查）

以下为本仓高频踩坑模式，执行任务前对照自查；命中即按对应知识类型回读文档：

| 失败表现 | 根因 | 补救知识 |
|---|---|---|
| 修改了 `TaskConfig`/`TaskInfo` 字段后运行时协议错乱 | 只改了三处平行定义之一（`common/request_core`、`services/src/task`、C++ ParcelHelper） | `feature-common.md` §3.1 |
| 改 IPC/UDS 后另一语言客户端行为异常 | 只改了双端之一（服务端 stub / C++ request_native / Rust request_next） | `feature-client.md` §3.2-3.3 |
| 非 OH 单测通过但设备上失败 | 平台门控下是两套实现（rusqlite vs rdb、mock vs samgr） | `feature-service.md` P6 |
| 路径/文件常量在设备上行为异常 | RK3568 用户态是 32 位 ARM，x86_64 本地验证不等于设备正确 | `feature-service.md` §3.6 |
| 绕过安全校验写文件被审计打回 | 未走 `files.rs` 校验链（`O_NOFOLLOW` + `/proc/self/fd`） | `feature-service.md` §3.6 |
| 改 `request_next` 后发现功能不生效 | 其 cxx 桥大部分被注释，生产主路径是 `request_native` | `feature-client.md` §3.6 |
| JS 用例大量超时失败 | 手动 `aa test` 不读 `testcase-timeout` | `feature-test.md` §3.2 |
| 构建莫名全量失败 | 用了 fast-rebuild 或漏 `--no-indep` | 本文件 §4 构建 |
| 新增对外函数后 dlopen 找不到符号 | 漏更新 version_script `.map` | `feature-client.md` §3.4 |

### Ask before

- 添加新的第三方依赖或外部组件依赖。
- 变更 IPC 接口码、UDS 消息格式或序列化字段顺序。
- 变更公共 API 语义（`@ohos.request` d.ts 对应行为）。
- 变更权限模型（`etc/downloadservice.cfg` 权限列表、SELinux 策略相关内容）。
- 变更 `request_task` 表结构或数据库版本（涉及存量数据兼容）。
- 变更 version_script 导出符号集合（影响二进制兼容）。
- 删除兼容性适配或迁移逻辑。
- 执行可能影响连接设备的操作（清数据、卸载重装、低内存压测）。

## 4. 验证

### 构建

```bash
# cd 到源码根目录（../../../，即包含 build.sh 的目录）运行
./build.sh --product-name rk3568 --build-target out/rk3568/build_configs/request/request:request --no-indep

# 测试编译
./build.sh --product-name rk3568 --build-target out/rk3568/build_configs/request/request:request_test --no-indep
```

构建注意事项：

- 必须带 `--no-indep`：单测 target 触发独立构建时会缺 `ylong_runtime.rlib` 导致全量失败；`out/rk3568` 已有产物可直接复用。
- 避免 `--fast-rebuild`：会触发 out/standard 独立构建，因 ICU 数据缺失而失败，使用普通 rk3568 全量或增量编译。
- 修改 `BUILD.gn` 等构建文件后，不要走快速构建路径。

### 产物位置

二进制产物：`out/rk3568/request/request/`

| 产物 | 说明 |
|---|---|
| `libdownload_server.dylib.so` | 服务端 SA 动态库（Rust） |
| `librequest.z.so` | NAPI 请求模块（`@ohos.request`） |
| `librequest_native.z.so` | C++ 客户端框架库 |
| `librequest_action.z.so` | inner_kits 文件传输代理入口 |
| `libcj_request_ffi.z.so` | Cangjie FFI 绑定 |
| `libpreload_native.z.so` / `libpreload_native_rust.z.so` | 预加载 C++/Rust 库 |
| `libcachedownload.z.so` | `@ohos.request.cacheDownload` NAPI |
| `librequest_ani.so` / `libcache_download_ani.so` | 新 ANI 绑定（Rust） |

测试产物：`out/rk3568/tests/unittest/request/request/request/`（`preload_test`、`fwkTest`、`innerTest`、`saTest`、`common_netstack_test`、`preload_napi_test`、`inotify_image_test`）。

### 最小验证

- Rust 格式化：`cargo fmt --check`（对照仓库 `rustfmt.toml`；本仓 Rust 源码以此为准）
- 构建当前模块（上方命令）
- 任务命中特性域时，按对应 feature-*.md §验证 执行

### 任务级验证

- 服务端行为变更 → 构建 `request` target + 跑 Rust UT（`rust_request_ut_test`）+ 相应 JS 用例
- IPC/序列化变更 → 构建 + 双端（services + frameworks）代码审查 + fuzz target 回归
- 客户端绑定变更 → 构建 + 对应语言用例（NAPI→`requestAgentTaskTest`，CJ/ANI→构建通过 + 人工冒烟）
- 预加载变更 → 构建 + `preload_test`/`preloadNapi` 用例
- DB 变更 → 构建 + `ut_database`/`ut_sql` + 设备端建库验证
- 仅测试变更 → 运行变更的测试及至少一个相邻相关测试

### 测试执行（JS/HAP 用例）

- 一律用 start.sh 执行（`test/testfwk/developer_test/start.sh`），手动 `aa test` 会因不读 `testcase-timeout` 配置而误报超时。
- 执行前设置设备常亮屏（设备默认 30s 灭屏会导致用例失败）：`power-shell wakeup && power-shell setmode 602` 并设置超长 timeout。
- 全量基线：960/960 通过（2026-08-11 起），回归时对照。
- 详见 `docs/agent/feature-test.md`。

### 回退方案

主验证路径失败时按以下顺序回退，不得直接放弃验证：

- **构建失败（全量/莫名失败）** → 检查是否用了 `--fast-rebuild`（触发 out/standard 独立构建，ICU 缺失）或漏 `--no-indep`（缺 `ylong_runtime.rlib`）；回退到普通 rk3568 增量编译；`out/rk3568` 已有产物可直接复用。
- **设备不可用（无 USB/网络）** → 主机端 `cargo test` 验证无 OH FFI 依赖的 crate（`request_core`、`common/database` 非 OH 路径）；其余变更做双端代码审查并在最终回复中明确"设备验证未执行"及原因。
- **Rust 单测全量跑不动** → 按 crate 拆分单测 target 单独编译执行；本地 fuzz 相关用 thin LTO 时加 `-k0` 隔离。
- **JS 用例失败** → 先分类再处理：断言失败（真问题）/ 环境失败（灭屏、SELinux、证书、网络）/ 超时误报（未走 start.sh）。环境类先修环境再重跑，不要改用例迁就环境。preload 用例失败优先查证书推送自动化。
- **无法构建 fuzz target** → 先确认是否改了 fuzzer 直接编译的源文件（`frameworks/native/request`、`common/sys_event`、`common/utf8_utils`）的私有成员，再检查 `#define private public` 相关的头文件依赖。
- **验证后结论仍不确定** → 在最终回复中给出剩余风险清单，标注"需要设备/人工确认"的事项，不得默认通过。

### Done 定义

任务完成仅在以下条件全部满足时：

- 请求的行为已实现。
- 相关构建/测试/lint/兼容性检查已执行，或已说明无法执行的原因。
- 最终回复包含：变更摘要、变更文件列表、验证结果、剩余风险。
- 不包含无关的格式化、重构或附带变更。
- 测试覆盖：修改和新代码有 UT 覆盖，新增外部接口有 FUZZ 测试覆盖。
- Fuzz driver 的 `LLVMFuzzerTestOneInput` 内被测 API 数量控制在 5 个左右（详见 `.claude/skills/fuzz-api-check/SKILL.md`）。

### 测试约束

- 测试用例必须包含显式断言，禁止无断言测试。
- 禁止不可能失败的断言（如 `EXPECT_TRUE(true)`）。
- Rust UT 通过 `#[cfg(test)] mod ut_xxx { include!("../tests/ut/...") }` 引入，新增测试文件必须同步在对应 `mod.rs`/`lib.rs` 登记。
- JS 用例遵循 Hypium 框架，测试套件在 `List.test.ets` 聚合登记。
