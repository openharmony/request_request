# feature-common.md — 公共库领域知识

> 本文档为 Agent 知识路由目标，由 AGENTS.md 的 task-based / path-based / vocabulary-based routing 触发阅读。
> Agent 能自己探索出来的少写；Agent 猜不准、猜错代价高、团队必须统一执行的内容要写。

## 1. 代码地图

本文档适用于 `common/` 目录——被服务端与客户端共同依赖的公共模块。最重要的认知：`common/request_core` 是**类型中枢**，但服务端 `services/src/task/` 内有一套**平行定义**（cxx 版），二者加 C++ `ParcelHelper` 三处必须人工保持一致；`common/utils` 的宏决定了全仓的平台门控方式。

### 模块全景

| 模块 | crate/库名 | 形态 | 职责 |
|---|---|---|---|
| `common/utils` | `request_utils` | Rust | cfg 宏（cfg_ohos!）、hilog 封装、哈希（sha256/url）、LRU、task_id 生成、文件路径校验（file_control）、网络观测、应用上下文、存储、MockHttpServer |
| `common/request_core` | `request_core` | Rust | 跨层类型中枢：TaskConfig/TaskInfo/State/Reason/SubscribeType/错误码/IPC 码/SearchFilter |
| `common/database` | **`rdb`**（注意名字） | Rust（cxx） | relational_store 的 Rust FFI 封装：RdbStore/OpenConfig/Params/FromSql/ToSql |
| `common/netstack_rs` | `netstack_rs` | Rust（cxx） | `netstack:http_client` 封装（给 preload 用）：Request/RequestTask/Response/回调/重试 |
| `common/ffrt_rs` | `ffrt_rs` | Rust（cxx） | FFRT 绑定：ffrt_spawn / ffrt_sleep |
| `common/sys_event` | `request_sysevent` | C++ 静态库 | HiSysEvent 封装：DfxErrorCode 枚举 + SendSysEventLog（STATISTIC/FAULT） |
| `common/utf8_utils` | `request_utf8_utils` | C++ 静态库 | RFC 3629 UTF-8 字节流校验（RunUtf8Validation） |
| `common/include/` | — | C++ 头 | constant.h（PausedReason/ExceptionErrorCode/网络常量/TLS 串）、log.h（REQUEST_HILOG*）、visibility.h（REQUEST_API） |

### 关键区域

- `common/utils/src/macros.rs`：`cfg_ohos!`/`cfg_not_ohos!`/`cfg_test!`/`cfg_not_test!`——**全仓平台门控入口**。
- `common/utils/src/test/server.rs`：`test_server()`——主机端 MockHttpServer（127.0.0.1:0 随机端口，单连接，返回 200）。
- `common/request_core/src/config.rs`：`TaskConfig`/`TaskConfigBuilder`/`Version`/`Action`/`Mode`/`NetworkConfig`/`FormItem`/`CommonTaskConfig`/`GroupConfig`/`MinSpeed`。
- `common/request_core/src/info.rs`：`State`/`SubscribeType`/`Faults`/`Reason`/`WaitingReason`/`Progress`/`NotifyData`/`TaskInfo`（手动实现 MsgParcel `Deserialize`）。
- `common/request_core/src/error_code.rs`：错误码表（ERR_OK/PERMISSION=201/SYSTEM_API=202/PARAMETER_CHECK=401/FILE_OPERATION_ERR=13400001/TASK_* 2190000x）。
- `common/request_core/src/interface.rs`：IPC 命令码（与 `services/src/service/interface.rs` 数值对齐）。
- `common/database/src/wrapper.rs`：rdb 的 cxx 桥（`OHOS::NativeRdb` 类型桥接）。
- `common/netstack_rs/src/wrapper.rs`：HttpClient cxx 桥 + `CallbackWrapper`（on_success/on_failure/on_progress/on_cancel + 自动重试）。

### 查找位置

- 加共享类型/字段 → `request_core/src/config.rs` 或 `info.rs`，**同步检查 services 平行定义**（§3.1）
- 错误码 → `request_core/src/error_code.rs`（Rust）+ `common/include/constant.h`（C++ ExceptionErrorCode）
- 平台门控写法 → `utils/src/macros.rs`
- DB 封装问题 → `database/src/database.rs`（RdbStore）+ `wrapper.cpp`（C++ 侧 GetRdbStore）
- HTTP 封装问题 → `netstack_rs/src/request.rs`/`task.rs`/`wrapper.rs`
- 主机端测试要 HTTP 服务 → `utils/src/test/server.rs`

## 2. 知识路由

### 按任务路由

- 新增/修改共享类型 → `request_core/` + 本文件 §3.1（平行定义同步）
- 错误码变更 → `error_code.rs` + `constant.h` + 本文件 §3.2
- DB 封装变更 → `database/` + feature-service.md §3.4
- 平台门控/条件编译 → `utils/src/macros.rs` + 本文件 §3.3
- DFX 上报 → `sys_event/include/sys_event.h`（DfxCode 枚举）
- 新增主机端单测 → 本文件 §3.5 + feature-test.md

### 按路径路由

- `common/request_core/` → 本文件 §3.1 + §3.2
- `common/utils/` → 本文件 §3.3 + §3.5
- `common/database/`、`common/netstack_rs/`、`common/ffrt_rs/` → 本文件 §3.4
- `common/sys_event/`、`common/utf8_utils/`、`common/include/` → 本文件 §3.6

### 按词汇路由

| 术语 | 风险提示 | 阅读 |
|---|---|---|
| TaskConfig/TaskInfo | 三处平行定义（request_core / services / C++ ParcelHelper），改一处查三处 | §3.1 |
| cfg_ohos! | 平台门控宏；OH 与非 OH 两套路径都要可编译 | §3.3 |
| unimplemented!() | request_core 多个 enum `From<u32>` 对未知值 panic，非优雅降级 | §3.7 P2 |
| rdb（crate 名） | `common/database` 的 crate 名是 `rdb`，GN target 是 `database_rs` | §3.7 P1 |
| DfxCode | HiSysEvent 事件码枚举，服务端 `sys_event!` 宏使用 | `sys_event/include/sys_event.h` |
| MockHttpServer / test_server | 主机端单测 HTTP 服务器，仅非 OH 测试可用 | §3.5 |
| REQUEST_HILOG* | C++ 日志宏（domain 0xD001C50，tag Requestkit） | `common/include/log.h` |
| FromSql/ToSql | rdb 参数绑定 trait（元组宏至 16 元素） | `database/src/params.rs` |

在计划中声明：任务类别、已读文档、发现的约束、是否应使用特定 Skill/工作流。

## 3. 约束与边界

### 架构/领域不变量

- `request_core` 保持**零平台依赖**（Cargo.toml 无 ohos feature，仅依赖 ipc crate）；平台相关能力放 `utils`。
- 枚举数值（State/Reason/SubscribeType/错误码/IPC 码）是跨进程协议常量，只增不改。
- Rust crate 通过 cxx 桥访问的 C++ 类型，其 `unsafe impl Send/Sync` 声明依赖 C++ 侧线程安全前提，修改前核对。

### 3.1 类型中枢与平行定义（本目录最大坑）

`TaskConfig`/`TaskInfo`/`State`/`Reason`/`SubscribeType` 等在仓内存在**多份平行定义**：

| 定义处 | 形态 | 消费方 |
|---|---|---|
| `common/request_core/src/config.rs`、`info.rs` | 纯 Rust（无 cxx） | Rust 客户端（request_next）、rustest |
| `services/src/task/config.rs`、`info.rs`、`reason.rs` | cxx::bridge 版 | 服务端（可跨语言） |
| `frameworks/native/request/`（ParcelHelper）+ `common/include/constant.h` | C++ | C++ 客户端/NAPI |

序列化链：服务端 `stub.rs` 手动逐字段写 MsgParcel → 客户端手动读。**修改任何字段/枚举值必须三处同步**，编译器不会帮你发现。`TaskInfo` 的 `Deserialize`（request_core/src/info.rs）还与 rustest 仓的独立反序列化实现并存（两套 SubscribeType 变体数不同）。

### 3.2 错误码体系

| 段 | 值域 | 定义 |
|---|---|---|
| 权限/参数 | 201 / 202 / 401 | `error_code.rs` |
| 文件操作 | 13400001+ | `error_code.rs` |
| 任务状态 | 21900004+（入队/模式/不存在/状态/组不存在） | `error_code.rs` |
| C++ 侧 | ExceptionErrorCode | `common/include/constant.h` |
| 任务失败原因 | Reason（值有空洞） | `info.rs` + services `reason.rs` |

新增错误码只追加；错误码是对外 API 契约（d.ts 文档化）。

### 3.3 平台门控

- Rust：`#[cfg(feature = "ohos")]`（utils）与 `#[cfg(feature = "oh")]`（services/request_next）两种 feature 名并存；门控宏用 `cfg_ohos!`/`cfg_oh!` 写。
- OH 模式：hilog/observe/context/wrapper/storage；非 OH 模式：标准 `log` crate re-export。
- 改公共代码必须**两条路径都编译**：主机 `cargo test`（非 OH）+ 设备构建（OH）。

### 3.4 FFI 封装层（database/netstack_rs/ffrt_rs）

- 三者都是"Rust API 壳 + cxx 桥 + C++ 实现"结构：`wrapper.rs`（桥）→ `cxx/wrapper.cpp`（实现）。
- `database`：`RdbStore`（open/execute/execute_sql/query）+ `OpenConfig`（builder + on_create/on_upgrade/on_downgrade/on_open/on_corrupt 回调）；表结构不在此层定义（业务表在 services）。
- `netstack_rs`：底层是 `netstack:http_client`（**不是** curl、不是 ylong）；`CallbackWrapper` 含自动重试逻辑。
- `ffrt_rs`：仅 `ffrt_spawn`/`ffrt_sleep` 两个函数；`ClosureWrapper` 执行后 `take` 消费闭包，保证只调用一次。
- 这些模块大量使用 `unsafe impl Send/Sync`（RdbStore/HttpClientTask/RequestTask/NetUnregistration 等），前提写在 `// SAFETY` 注释；给这些结构新增字段前先验证 C++ 侧线程安全性。

### 3.5 主机端测试设施

- `request_utils::test::server::test_server(handler)`：启动 127.0.0.1:0 的 TcpListener，单连接，把请求行交给 handler 后回固定 200；返回 URL 供用例构造下载/上传任务。仅 `cfg(not(feature = "ohos"))` 可用。
- Rust 单测通过 `#[cfg(test)] mod ut_xxx { include!("../tests/ut/...") }` 引入外部文件——`include!` 相对**源文件**解析路径，移动文件即断，新增测试文件必须在 lib.rs/mod.rs 登记。

### 3.6 C++ 公共件

- `sys_event`：`DfxErrorCode` 覆盖 IPC/任务/SA/ACL/Samgr/ABMS/RDB/网络/媒体/通知等故障码；`SendSysEventLog` 发 STATISTIC（EXEC_ERROR）与 FAULT（EXEC_FAULT）。服务端 Rust 的 `sys_event!` 宏最终走到这里。**禁止删除或重编号 DfxCode**（DFX 看板依赖）。
- `utf8_utils`：单一函数 `RunUtf8Validation`，fuzz 与 NAPI 层用于入参校验。
- `common/include/log.h`：`CONFIG_REQUEST_LOG` 定义时日志宏为空（发布裁剪），调试时注意该开关。

### 禁止事项

- NEVER 修改既有枚举数值/错误码数值（只追加）。
- NEVER 只改三处平行定义之一（request_core / services / C++）。
- NEVER 给 `request_core` 引入平台依赖（ohos feature、系统 crate）。
- NEVER 移除 `// SAFETY` 注释或在其前提不成立时保留 `unsafe impl Send/Sync`。
- NEVER 在 fuzz/任意输入路径上直接依赖 request_core 的 `From<u32>`（会 panic，见 P2）。

### 已知陷阱

**P1: crate 名 ≠ GN target 名** — `common/database` crate 名 `rdb`、target `database_rs`；`test/rustest` crate 名 `test_common`、target `rust_request_test_common`。依赖路径写 target 名，`use` 写 crate 名。

**P2: `From<u32>` 用 `unimplemented!()`** — `request_core` 的 `SubscribeType`/`WaitingReason`/`Reason`/`Faults`/`Version`/`Action`/`Mode` 等的 `From<u32>` 对未知值 panic。fuzz 或不可信输入先判范围再转换。

**P3: rustest 反序列化与 request_core 并存** — `test/rustest/src/lib.rs` 有独立的一套裸字节解析（`Take` trait），与 `request_core` 的 `Deserialize` 平行；两套 SubscribeType 变体数不同（缺 FaultOccur/Wait）。协议变更两处都要看。

**P4: fuzzer 直接编译 common 源文件** — fuzz target 的 sources 直接包含 `sys_event.cpp`/`utf8_utils.cpp`；改这些文件的依赖或头文件会连带破坏 fuzzer 编译。

**P5: 非 OH 的 DB 行为差异** — `rusqlite`（非 OH）与 rdb（OH）在错误处理/PRAGMA 上行为不同，主机测试通过不代表设备一致（同 feature-service.md P6）。

### 询问后再做

- 新增共享类型字段（涉及 IPC 序列化三处同步与兼容性）。
- 新增错误码或 DfxCode。
- 给 `request_core`/`utils` 新增依赖。
- 修改 cxx 桥公开类型集合。

## 4. 验证

### 最小检查

- 构建（common 随 `request` target 编译）
- Rust UT：`common/database`、`common/ffrt_rs`、`common/netstack_rs`、`common/utils` 各有 GN unittest target（bundle.json test 列表）
- 主机端：`request_core`/`database` 可 `cargo test`（非 OH 路径）；`utils`/`netstack_rs`/`ffrt_rs` 依赖 OH FFI，只能设备/模拟构建

### 任务级检查

- request_core 类型变更 → 全量构建 + services/frameworks 使用点审查 + IPC 双端序列化核对
- 错误码变更 → 全量构建 + JS 用例断言核对 + d.ts 文档同步确认
- database/netstack 封装变更 → 对应 crate UT + 服务端/preload 受影响路径回归
- utils 变更 → `rust_utils_ut_test` + 全量构建（被所有 crate 依赖）

### 完成定义

- 请求的行为已实现。
- 相关构建/测试/lint/兼容性检查已执行，或已说明无法执行的原因。
- 最终回复包含：变更摘要、变更文件列表、验证结果、剩余风险。
- 不包含无关的格式化、重构或附带变更。

## 5. 关键文件索引

| 文件 | 职责 |
|---|---|
| `common/utils/src/macros.rs` | cfg_ohos!/cfg_not_ohos!/cfg_test! 平台门控宏 |
| `common/utils/src/test/server.rs` | MockHttpServer（test_server） |
| `common/utils/src/task_id.rs` | 任务 ID 生成 |
| `common/utils/src/file_control.rs` | 文件路径校验 |
| `common/request_core/src/config.rs` | TaskConfig 等配置类型 |
| `common/request_core/src/info.rs` | State/Reason/TaskInfo（含手动 MsgParcel Deserialize） |
| `common/request_core/src/error_code.rs` | 错误码表 |
| `common/request_core/src/interface.rs` | IPC 命令码（客户端侧对齐） |
| `common/database/src/database.rs` | RdbStore 封装 |
| `common/database/src/config.rs` | OpenConfig 与建库回调 |
| `common/database/src/wrapper.rs` | rdb cxx 桥（OHOS::NativeRdb） |
| `common/netstack_rs/src/request.rs`、`task.rs` | HTTP 请求 builder 与任务 |
| `common/netstack_rs/src/wrapper.rs` | HttpClient cxx 桥 + 重试回调 |
| `common/ffrt_rs/src/wrapper.rs` | FFRT 绑定（spawn/sleep） |
| `common/sys_event/include/sys_event.h` | DfxCode 枚举 + HiSysEvent 发送 |
| `common/utf8_utils/src/utf8_utils.cpp` | UTF-8 校验 |
| `common/include/constant.h` | C++ 侧枚举与常量（与 Rust 对应） |
| `common/include/log.h` | REQUEST_HILOG* 日志宏 |

### 注意事项 / 外部依赖

- `ipc` crate（OpenHarmony Rust IPC）是 request_core 的唯一外部依赖。
- `ani_rs` 来自 communication_netmanager_base（utils 的网络观测用）。
- `rusqlite` 仅在非 OH 测试路径使用，不进设备。
