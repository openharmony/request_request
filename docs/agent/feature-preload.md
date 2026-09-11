# feature-preload.md — 缓存下载与预加载领域知识

> 本文档为 Agent 知识路由目标，由 AGENTS.md 的 task-based / path-based / vocabulary-based routing 触发阅读。
> Agent 能自己探索出来的少写；Agent 猜不准、猜错代价高、团队必须统一执行的内容要写。

## 1. 代码地图

本文档适用于 `frameworks/native/cache_download/`、`frameworks/native/cache_core/`、`frameworks/js/napi/cache_download/`、`frameworks/js/napi/preload_napi/`、`frameworks/ets/ani/cache_download/` 及 `interfaces/inner_kits/cache_download/`。

预加载缓存（preload / cacheDownload）是**独立于主下载服务**的特性：不走 SA 3706、不建任务、不落 request_task 表，在**应用进程内**直接完成 HTTP 下载并写入两级缓存（RAM + 文件）。最重要的架构边界是**三层职责**：`cache_core` 管存储（缓存数据结构与目录治理），`cache_download` 管下载逻辑（HTTP + 任务管理 + cxx 桥），`Preload`/NAPI/ANI 是对外的 API 壳。

### 模块全景

| 模块 | 语言 | 产物 | 职责 |
|---|---|---|---|
| `frameworks/native/cache_core/` | Rust（cxx 桥） | `libcache_core.rlib` + `libcache_core_cxx.a` | 缓存基础设施：`RamCache`、文件缓存与空间管理、inotify 目录监听、目录重建 |
| `frameworks/native/cache_download/` | Rust + C++（cxx 桥） | `libpreload_native.z.so`（C++ 主库）+ `libpreload_native_rust.z.so`/`rlib` | 下载服务层：`CacheDownloadService`、HTTP 任务、下载信息统计、cxx 桥 |
| `frameworks/js/napi/preload_napi/` | C++ | `libpreload_napi.z.so` | `CppDownloadInfo`→napi_value 转换辅助 |
| `frameworks/js/napi/cache_download/` | C++ | `libcachedownload.z.so` | `@ohos.request.cacheDownload` NAPI 绑定 |
| `frameworks/ets/ani/cache_download/` | Rust | `libcache_download_ani.so` | 新 ANI 绑定（含权限校验 `preload_permission_verify`） |

inner_kits 头文件：`interfaces/inner_kits/cache_download/native/include/request_preload.h`（C++ `Preload` 单例 API）、`napi/include/preload_napi.h`。

### 关键区域

- `cache_download/src/services.rs`：`CacheDownloadService` 单例——对外 API 的实现核心（`ffi_preload`/`ffi_fetch`/`ffi_get_download_info`/`cancel`/`remove`/`contains`/缓存大小配置）。
- `cache_download/src/download/`：HTTP 下载实现。**双后端**：`netstack.rs`（用 `common/netstack_rs` 封装的 `netstack:http_client`）与 `ylong/`（用 `ylong_http`），`task.rs` 管理任务。
- `cache_download/src/wrapper.rs`：最完整的 cxx 桥（命名空间 `OHOS::Request`），Rust↔C++ 类型与回调桥接。
- `cache_download/src/info.rs`：`RustDownloadInfo`——DNS/连接/TLS 各阶段耗时统计。
- `cache_download/src/cxx/preload_callback.cpp`、`request_preload.cpp`：C++ 侧回调包装与 `Preload` 类实现。
- `cache_core/src/data/ram.rs`：`RamCache` 内存缓存。
- `cache_core/src/data/file.rs`、`space.rs`：文件缓存与空间管理。
- `cache_core/src/observe.rs` + `cxx/inotify_event_listener.cpp`：inotify 目录监听（监听相册目录删除事件，联动清理缓存）。
- `cache_core/src/data/observer.rs`：`DirRebuilder` 目录重建。

### 查找位置

- 下载行为/重试/超时 → `cache_download/src/download/`
- 缓存命中与淘汰 → `cache_core/src/data/`（ram/file/space）
- 回调线程问题 → `wrapper.rs`（`FfiCallback`）+ 本文件 §3.2
- cxx 类型扩展 → `wrapper.rs` 桥两端 + `cxx/preload_callback.cpp`
- 权限（ANI 侧）→ `ets/ani/cache_download/src/common/permission_check.rs`
- 缓存目录被删后的恢复 → `cache_core/src/data/observer.rs`（DirRebuilder）

## 2. 知识路由

### 按任务路由

- 预加载行为变更 → `cache_download/src/services.rs` + 本文件 §3.1
- HTTP 后端切换/行为 → `cache_download/src/download/`（注意双后端，见 §3.3）
- 缓存容量/淘汰策略 → `cache_core/src/data/space.rs`
- NAPI/ANI API 变更 → `js/napi/cache_download/`、`ets/ani/cache_download/` + 符号表（feature-client.md §3.4）
- 目录监听/联动清理 → `cache_core/src/observe.rs` + cxx `DirectoryMonitor`

### 按路径路由

- `frameworks/native/cache_core/` → 本文件 §3.4
- `frameworks/native/cache_download/src/download/` → 本文件 §3.3
- `frameworks/native/cache_download/src/wrapper.rs` → 本文件 §3.2
- `interfaces/inner_kits/cache_download/` → feature-client.md §3.7

### 按词汇路由

| 术语 | 风险提示 | 阅读 |
|---|---|---|
| CacheDownloadService | 预加载服务单例，对外 API 实现 | `cache_download/src/services.rs` |
| Preload | C++ 对外单例类（inner_kits） | `request_preload.h` + `cxx/request_preload.cpp` |
| RamCache | 内存缓存；容量配置经 set_ram_cache_size | `cache_core/src/data/ram.rs` |
| DirectoryMonitor | inotify 监听（C++→Rust 桥） | `cache_core/src/cxx/` |
| DirRebuilder | 缓存目录被外部删除后的重建 | `cache_core/src/data/observer.rs` |
| RustDownloadInfo | 阶段耗时统计（DNS/连接/TLS） | `cache_download/src/info.rs` |
| FfiPredownloadOptions | cxx 桥配置结构（headers 为扁平数组） | `wrapper.rs` + §3.2 |
| FfiCallback | C++ 回调的 Rust 包装，手动 unsafe impl Send | `wrapper.rs` + §3.2 |
| netstack / ylong 双后端 | 下载有两条 HTTP 路径，改行为要确认走哪条 | §3.3 |

在计划中声明：任务类别、已读文档、发现的约束、是否应使用特定 Skill/工作流。

## 3. 约束与边界

### 架构/领域不变量

- 预加载不走服务端 SA：任务、缓存、回调全部在**调用方进程内**；不要在这里引入 IPC/SA 依赖。
- 与主下载服务（feature-service.md）**不共享**任务模型、DB、调度器。
- cache_core 不做 HTTP；cache_download 不直接管理缓存存储，经 cache_core API。
- 对外能力经 cxx 桥（`wrapper.rs`）暴露给 C++（`Preload` 类），NAPI/ANI 再包 C++/Rust。
- 符号导出受 `libcache_download.map` 白名单限制（feature-client.md §3.4）。

### 3.1 服务单例与 API 语义

`CacheDownloadService` 提供：`load`（预加载，带回调与选项）、`fetch`（取缓存数据）、`get_download_info`（统计）、`cancel`/`remove`/`contains`、`set_file_cache_size`/`set_ram_cache_size`。注意语义差异：`remove` 删缓存记录，`cancel` 取消进行中的任务；`contains` 查缓存命中。改动 API 语义前核对 JS/C++/ANI 三侧调用方。

### 3.2 cxx 桥与回调（改 wrapper.rs 前必读）

- 桥两端类型必须同时声明；cxx 只支持特定类型集（String/&str/Vec<T>/Box<T>/UniquePtr/SharedPtr 等）。
- `FfiPredownloadOptions.headers` 是 `Vec<&str>` **扁平数组**，C++ 侧按两两一组（k,v）配对消费——不是 `Vec<(String,String)>`。
- `FfiCallback` 持有 C++ `UniquePtr<PreloadCallbackWrapper>`，cxx 生成类型默认不 Send/Sync；代码中**手动 `unsafe impl Send`**（前提是 C++ 侧设计为线程安全）。若修改 C++ 回调实现引入非线程安全状态，这里会变 UB。
- `RustData`/`TaskHandle`/`RustDownloadInfo` 跨界必须经 `SharedData`/`ShareTaskHandle`/`UniqueData`/`UniqueInfo` 工厂函数转换，不能直接返回 `Arc` 等类型。

### 3.3 HTTP 双后端

`cache_download/src/download/` 下两条 HTTP 实现并存：`netstack.rs`（`common/netstack_rs`→`netstack:http_client`，C++ HttpSession 经 FFI）与 `ylong/`（Rust `ylong_http`）。改下载行为先确认目标路径用哪个后端（BUILD.gn 与运行时开关），避免只改一条。这与服务端主下载的 `ylong_http_client`（feature-service.md §3.7）又是不同的栈——三处 HTTP 栈勿混淆。

### 3.4 缓存存储与目录治理

- 两级缓存：RAM（`RamCache`，默认容量可配）+ 文件缓存（`space.rs` 控制空间）。
- inotify：`DirectoryMonitor`（C++）监听目标目录变更（如相册文件删除）→ Rust 侧联动清理关联缓存；`DirRebuilder` 在缓存目录被整体删除后重建。
- 缓存目录布局变更会影响存量缓存命中与清理逻辑，需评估升级兼容。

### 禁止事项

- NEVER 在 cache_download/cache_core 引入对 SA 3706 的依赖。
- NEVER 绕过 `wrapper.rs` 工厂函数直接跨 cxx 传 Rust 原生类型。
- NEVER 修改 `FfiPredownloadOptions` 字段布局而不同步 C++ 侧（headers 扁平数组契约）。
- NEVER 在 C++ 回调实现中引入非线程安全状态（Rust 侧已 unsafe impl Send）。
- NEVER 删除 inotify 联动清理或 DirRebuilder（缓存目录治理依赖它们）。

### 已知陷阱

**P1: 哨兵值与 integer_overflow sanitize** — `FfiPredownloadOptions.max_retry` 等用 `usize::MAX` 作哨兵，sanitize 开启 integer_overflow，相关算术溢出会 trap；处理边界值时显式判哨兵。

**P2: 双后端行为漂移** — netstack 与 ylong 后端在重试/超时/进度语义上有差异；bug 修复要确认两条路径是否都需要。

**P3: RAM 缓存与文件缓存的一致性** — `fetch` 命中 RAM 或文件两级，写文件后 RAM 条目的失效时序在 `cache_core/src/data/` 内部；跨层改写入路径注意两级一致性。

**P4: inotify 事件误删** — 目录监听触发清理时的事件过滤逻辑在 C++ `inotify_event_listener.cpp`，放宽过滤会误删未关联缓存。

### 询问后再做

- 修改 `request_preload.h`（inner_kits 公共头文件）。
- 变更缓存目录布局或持久化缓存索引格式。
- 切换默认 HTTP 后端。
- 修改缓存默认容量配置。

## 4. 验证

### 最小检查

- 构建预加载目标（随 `request` target 一并产出 `libpreload_native.z.so` 等）
- C++ UT：`preload_test`（abnormal/cancel/clear_cache/fail/get_info/progress/success）、`preloadNapi`（`preload_napi_test`）、`inotifyImage`（`inotify_image_test`）
- Rust UT：`cache_core`/`cache_download` 的 GN unittest target（`frameworks/native/cache_core:unittest` 等，见 bundle.json test 列表）

### 任务级检查

- 下载行为变更 → `preload_test` 全套 + 确认双后端路径
- 缓存治理变更 → `inotify_image_test` + `clear_cache` 用例
- cxx 桥变更 → 全量构建（桥代码重新生成）+ `preload_test`
- NAPI/ANI API 变更 → JS 用例（preload 相关）+ 设备冒烟

### 完成定义

- 请求的行为已实现。
- 相关构建/测试/lint/兼容性检查已执行，或已说明无法执行的原因。
- 最终回复包含：变更摘要、变更文件列表、验证结果、剩余风险。
- 不包含无关的格式化、重构或附带变更。

## 5. 关键文件索引

| 文件 | 职责 |
|---|---|
| `frameworks/native/cache_download/src/services.rs` | CacheDownloadService 单例（对外 API 实现） |
| `frameworks/native/cache_download/src/download/` | HTTP 下载（netstack.rs / ylong/ 双后端 + task.rs） |
| `frameworks/native/cache_download/src/wrapper.rs` | cxx 桥（类型、回调、工厂函数） |
| `frameworks/native/cache_download/src/info.rs` | RustDownloadInfo 阶段耗时统计 |
| `frameworks/native/cache_download/src/cxx/request_preload.cpp` | C++ `Preload` 类实现 |
| `frameworks/native/cache_download/src/cxx/preload_callback.cpp` | C++ 回调包装 |
| `frameworks/native/cache_core/src/data/ram.rs` | RamCache |
| `frameworks/native/cache_core/src/data/file.rs`、`space.rs` | 文件缓存与空间管理 |
| `frameworks/native/cache_core/src/data/observer.rs` | DirRebuilder 目录重建 |
| `frameworks/native/cache_core/src/observe.rs` | inotify 监听接入 |
| `frameworks/native/cache_core/src/cxx/inotify_event_listener.cpp` | C++ inotify 实现（DirectoryMonitor） |
| `interfaces/inner_kits/cache_download/native/include/request_preload.h` | C++ 公共 API（Preload 类、选项、错误枚举） |
| `frameworks/ets/ani/cache_download/src/common/permission_check.rs` | ANI 侧权限校验 |

### 注意事项 / 外部依赖

- `common/netstack_rs`、`common/ffrt_rs`、`common/utils` 由本仓提供（见 feature-common.md）。
- inotify 能力经 C++ 实现（cxx 桥 `DirectoryMonitor`），Rust 侧不直接调 inotify。
- 与主下载服务的差异表：无 SA、无 DB、无调度器、进程内回调——不要从 feature-service.md 的模型类推。
