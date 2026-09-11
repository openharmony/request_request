# feature-client.md — 客户端框架与多语言绑定领域知识

> 本文档为 Agent 知识路由目标，由 AGENTS.md 的 task-based / path-based / vocabulary-based routing 触发阅读。
> Agent 能自己探索出来的少写；Agent 猜不准、猜错代价高、团队必须统一执行的内容要写。

## 1. 代码地图

本文档适用于 `frameworks/native/`（客户端核心库）、`frameworks/js/`、`frameworks/ets/`、`frameworks/cj/`（语言绑定）及 `interfaces/inner_kits/`。

客户端库的职责边界：**只做 IPC 代理与事件回传**。任务调度、重试、持久化全部在服务端（见 feature-service.md）；客户端把 API 调用转成 IPC 请求，并把服务端 UDS 推送转成语言层回调。

### 模块全景

| 模块 | 语言 | 产物 | 说明 |
|---|---|---|---|
| `frameworks/native/request/` | C++ | `librequest_native.z.so` | **当前生产主路径**客户端核心：`RequestManager`（单例编排）、`RequestServiceProxy`（IPC 代理）、`ResponseMessageReceiver`（UDS 接收） |
| `frameworks/native/request_next/` | Rust | `librequest_client.rlib`（静态） | Rust 新客户端（`RequestClient` 单例、纯 Rust IPC）；**cxx 桥大部分被注释，功能不完整**，仅被新 ANI 使用 |
| `frameworks/native/request_action/` | C++ | `librequest_action.z.so` | inner_kits 入口（`RequestAction` 单例）：路径标准化 + 权限校验 + 转发 `RequestManager` |
| `frameworks/js/napi/request/` | C++ | `librequest.z.so` | NAPI 绑定，`nm_modname = "request"`，安装 `module/` |
| `frameworks/js/napi/cache_download/` | C++ | `libcachedownload.z.so` | `request.cacheDownload` 绑定（见 feature-preload.md） |
| `frameworks/js/napi/preload_napi/` | C++ | `libpreload_napi.z.so` | C++ DownloadInfo → napi_value 转换辅助 |
| `frameworks/js/ani/` | C++ | `librequestmodule_ani.so` + `requestmodule.abc` | 旧 ANI 绑定（API9/10），依赖 request_native |
| `frameworks/ets/ani/request/` | Rust | `librequest_ani.so` + `request.abc` | 新 ANI 绑定（Rust），依赖 request_next |
| `frameworks/ets/ani/cache_download/` | Rust | `libcache_download_ani.so` | 缓存下载新 ANI 绑定 |
| `frameworks/cj/ffi/` | C++ | `libcj_request_ffi.z.so` | Cangjie FFI 绑定（`Ffi*` 符号），依赖 request_native |

依赖关系（当前生效）：

```
NAPI(librequest.so) ─┐
旧ANI(librequestmodule_ani.so) ├→ request_native(C++) → IPC → 服务端
CJ(libcj_request_ffi.so) ─┤
request_action ─┘
新ANI(librequest_ani.so) → request_next(Rust, rlib) → ipc_rust/samgr_rust → 服务端
```

### 查找位置

- JS API 行为变更 → `frameworks/js/napi/request/src/`（入口 `request_module.cpp`，任务封装 `js_task.cpp`，事件分发 `request_event.cpp`）
- 回调链路问题 → `js_response_listener.cpp`（UDS 消息→JS 回调的线程切换）
- IPC 调用参数 → `native/request/src/request_service_proxy.cpp` + `include/download_server_ipc_interface_code.h`
- UDS 协议解析 → `native/request/include/response_message_receiver.h`（C++ 侧）
- 版本兼容（API9/10）→ `request_event.cpp` 的 `supportEventsV9_`/`supportEventsV10_` 两套映射表
- inner_kits 调用方适配 → `interfaces/inner_kits/request_action/include/request_action.h`
- 符号导出问题 → 各模块 `.map` 文件（见 §3.4）

## 2. 知识路由

### 按任务路由

- 新增/修改 JS API → `js/napi/request/src/` + 本文件 §3.1（线程模型）
- IPC 接口变更 → `native/request/`（C++ proxy）+ `services/src/service/`（服务端）**双端同步** + feature-service.md
- UDS 消息格式变更 → `response_message_receiver.h`（C++）+ `request_next/src/listen/uds.rs`（Rust）+ `services/src/service/client/mod.rs`（服务端）**三处同步**
- 符号导出/链接问题 → 本文件 §3.4 + 对应 `.map`
- 新 ANI（Rust）变更 → `ets/ani/` + `native/request_next/`，先确认桥是否被注释
- Cangjie 变更 → `cj/ffi/`，注意 `Ffi*` 前缀约定

### 按路径路由

- `frameworks/native/request/` → 本文件 §3.2（IPC）+ §3.3（UDS）
- `frameworks/native/request_next/` → 本文件 §3.6（迁移状态）
- `frameworks/js/napi/` → 本文件 §3.1（NAPI 线程）+ §3.4（符号）
- `frameworks/ets/ani/`、`frameworks/cj/` → 本文件 §3.5（boot abc/FFI 约定）
- `interfaces/inner_kits/` → 本文件 §3.7

### 按词汇路由

| 术语 | 风险提示 | 阅读 |
|---|---|---|
| RequestManager | C++ 客户端编排单例，NAPI/CJ/旧ANI/request_action 共用 | `native/request/include/request_manager.h` |
| RequestServiceProxy | IPC 代理；接口码与 parcel 顺序必须与服务端一致 | `request_service_proxy.cpp` |
| ResponseMessageReceiver | UDS 接收器，epoll 监听 fd，手写二进制解析 | `response_message_receiver.h` |
| 魔数 0x43434646 | UDS 协议魔数（"CCFF"），三处定义必须一致 | §3.3 |
| nm_modname / relative_install_dir | JS import 路径与模块注册名/安装目录强绑定 | §3.4 |
| version_script / .map | 符号导出白名单，漏加即 dlopen 失败 | §3.4 |
| request_native | C++ 旧客户端，生产主路径 | 本文件 §1 |
| request_next | Rust 新客户端，cxx 桥未启用，仅新 ANI 用 | §3.6 |
| boot abc | ArkTS 字节码，与 so 产物名精确配对 | §3.5 |
| supportEventsV9_/V10_ | API9 与 API10 两套事件映射，勿混用 | `request_event.cpp` |
| ParcelHelper | C++ 侧 parcel 编解码辅助，fuzzer 也依赖 | `native/request/` |

在计划中声明：任务类别、已读文档、发现的约束、是否应使用特定 Skill/工作流。

## 3. 约束与边界

### 架构/领域不变量

- 客户端不做业务决策：无调度、无重试、无持久化；`RequestManager` 只管理任务对象生命周期与订阅。
- IPC 接口码与 parcel 字段顺序与 `services/src/service/interface.rs`、`stub.rs` 严格对齐。
- 所有跨 DSO 对外符号必须列入 version_script `.map`。
- 客户端 sanitize 全开（cfi/cfi_cross_dso/ubsan/integer_overflow/boundary_sanitize）：跨语言调用签名必须精确匹配，整数溢出会 trap。

### 3.1 NAPI 回调线程模型

服务端 UDS 消息到达时运行在 epoll/EventRunner 线程，**不能直接调 JS**。链路（`js_response_listener.cpp`）：

1. `OnResponseReceive` 收到消息；
2. `napi_send_event(env, lambda, napi_eprio_high)` 切回 JS 线程；
3. lambda 内 `napi_open_handle_scope` → 转换 → `OnMessageReceive` → `napi_close_handle_scope`。

新增回调路径必须走该模式；直接 `napi_call_function` 会崩溃，漏开 handle scope 会泄漏。

### 3.2 IPC 控制面

- C++：`RequestServiceProxy` 按 `download_server_ipc_interface_code.h` 的码发 IPC；接口 token 与服务端 `stub.rs` 一致。
- Rust（request_next）：`src/proxy/` 独立实现同一套码。**改接口码两处都要改**。
- 序列化是手动逐字段（C++ `ParcelHelper` / 服务端 `serialize_task_*`），加字段必须双端同步且评估旧客户端兼容。

### 3.3 UDS 数据面协议

自有二进制协议，**三处实现**必须同步：服务端 `services/src/service/client/mod.rs`、C++ `response_message_receiver.h`、Rust `request_next/src/listen/uds.rs`。

- 魔数 `0x43434646`；消息类型：HttpResponse=0、NotifyData=1、Faults=2、Waiting=3。
- C++ 侧手动字节解析（`Int64FromParcel` 等）对端序和对齐敏感。
- 加字段/改类型大小 = 协议破坏，必须三端同改并考虑版本兼容。

### 3.4 符号导出与模块注册

version_script 白名单（默认 `local: *`，不在表内即隐藏）：

| .map | 位置 | 导出 |
|---|---|---|
| `libdownload_single.map` | `native/request/` | `*RequestManager*`、`*NotifyStub*`、`*DownloadTask*`、`*ParcelHelper*`、`*IRunningTaskObserver*`、`*SubscribeRunningTaskCount*` 等 |
| `libcj_request_ffi.map` | `cj/ffi/` | `Ffi*`（Cangjie FFI 约定） |
| `libcache_download.map` | `js/napi/preload_napi/` | `*BuildDownloadInfo*`、`*BuildInfo*`、`*SetOptions*` 等 |

- 新增对外函数：改符号名会破坏其他子系统二进制兼容；新增必须同步 `.map`。
- `nm_modname` 与 `relative_install_dir` 决定 JS import 路径：`request`→`@ohos.request`（安装 `module/`）；`request.cacheDownload`→`@ohos.request.cacheDownload`（安装 `module/request/`）。改任一项都会导致运行时 import 失败。

### 3.5 ANI 与 Cangjie 约定

- 新旧两套 ANI 并存：`js/ani`（C++，abc `requestmodule.abc`，loadLibrary `requestmodule_ani`）与 `ets/ani`（Rust，abc `request.abc`，loadLibrary `request_ani`）。改 so target 名必须同步 `.ets` 里的 `loadLibrary` 字符串与 BUILD.gn 的 abc 产物名。
- boot abc（`is_boot_abc = "True"`）安装到 `/system/framework/`，与 so 产物名精确配对。
- Cangjie FFI 导出函数名必须 `Ffi` 前缀（map 文件 glob `Ffi*`），CJ listener 系列对应 NAPI listener 语义。

### 3.6 request_next 迁移状态

- `native/request_next/BUILD.gn` 的 cxx 桥段（`request_next_cxx_gen`/`request_next_cxx`）**整段被注释**；`wrapper.rs` 中 `on_response`/`OpenChannel`/`AclSetAccess`/`GetAppBaseDir` 等桥接均注释，仅保留 `FileUri` 桥。
- 当前纯 Rust IPC 路径（`src/proxy/` + `listen/uds.rs`）可用，C++ 桥未启用。
- 修改时勿"顺手恢复"注释代码；涉及则先确认依赖链（`subscribe.h`/`wrapper.h` 与 `ets/ani/request` 的引用关系）。

### 3.7 inner_kits（request_action）

`interfaces/inner_kits/request_action/include/request_action.h` 是给其他子系统的稳定入口。`RequestAction` 做三件事：路径标准化（沙箱相对路径→绝对路径）、权限校验（DOWNLOAD_SESSION_MANAGER/UPLOAD_SESSION_MANAGER）、转发 `RequestManager`。改动公共头文件签名需要评审所有调用子系统。

其他 inner_kits：`running_count/include/running_task_count.h`（运行计数订阅）、`cache_download/native|napi/include`（见 feature-preload.md）。

### 禁止事项

- NEVER 在非 JS 线程直接调用 napi 函数（先 `napi_send_event`）。
- NEVER 新增对外符号而不更新 `.map` 文件。
- NEVER 修改 UDS 魔数、消息类型值或字段顺序而不三端同步。
- NEVER 修改 `nm_modname`/`relative_install_dir`（JS import 会断）。
- NEVER 假设 `request_next` 已完整可用（cxx 桥被注释）。
- NEVER 修改 `.ets` 的 `loadLibrary` 名而不核对 so target 名。
- NEVER 为通过测试删除回调、事件或错误码路径。

### 已知陷阱

**P1: NAPI 线程与作用域** — 见 §3.1；两类崩溃（跨线程调用、漏 handle scope）都只在运行时暴露。

**P2: API9/API10 双版本事件表** — `request_event.cpp` 维护 `supportEventsV9_`/`supportEventsV10_` 两套映射；新事件要确认加在哪套，JS 侧按 API 版本取表。

**P3: fuzz 依赖私有成员** — fuzzer 用 `#define private public` + 直接编译 frameworks 源文件；改 `native/request` 头文件的私有成员会破坏 fuzzer 编译（见 feature-test.md）。

**P4: CFI 跨 DSO 校验** — sanitize 开启 cfi_cross_dso，vtable/签名不匹配直接 trap；cxx 桥类型与 C++ 声明必须逐字段一致。

**P5: 静态库与动态库双产物** — `js/napi/request` 同时产出 `librequest.so`（动态，base_group 安装）与 `librequest_static.a`（inner_kits）；改动公共头文件影响两者。

### 询问后再做

- 新增/修改 IPC 接口码或序列化字段。
- 修改 inner_kits 头文件（request_action/running_task_count）。
- 修改 `.map` 导出符号集合、`nm_modname`、安装路径。
- 恢复 request_next 被注释的 cxx 桥。
- 新增语言绑定或依赖组件。

## 4. 验证

### 最小检查

- 构建客户端：`./build.sh --product-name rk3568 --build-target out/rk3568/build_configs/request/request:request --no-indep`（覆盖全部绑定产物）
- C++ 客户端 UT：`fwkTest`（parcel/manager/proxy/receiver）、`innerTest`（request_action/path_control）
- JS 用例：`requestAgentTaskTest`（API 行为）、`requestQueryTaskTest`、`requestSystemQueryTest`，执行方式见 feature-test.md

### 任务级检查

- NAPI 绑定变更 → `requestAgentTaskTest` 对应用例 + 全量 JS 回归
- IPC/UDS 协议变更 → `fwkTest` + 服务端 UT + JS 全量（进度/事件回调类用例）
- 符号导出变更 → 构建全量（链接期暴露）+ 设备 dlopen 冒烟
- ANI/CJ 变更 → 构建通过 + 设备加载冒烟（对应语言的 runtime 可用性人工验证）
- inner_kits 变更 → `innerTest` + 调用子系统兼容性确认

### 完成定义

- 请求的行为已实现。
- 相关构建/测试/lint/兼容性检查已执行，或已说明无法执行的原因。
- 最终回复包含：变更摘要、变更文件列表、验证结果、剩余风险。
- 不包含无关的格式化、重构或附带变更。
- 新增对外接口有 UT 与 fuzz 覆盖。

## 5. 关键文件索引

| 文件 | 职责 |
|---|---|
| `frameworks/native/request/include/request_manager.h` | C++ 客户端编排单例 |
| `frameworks/native/request/include/request_service_proxy.h` | IPC 代理 |
| `frameworks/native/request/include/download_server_ipc_interface_code.h` | 客户端侧 IPC 码定义（与服务端 interface.rs 对齐） |
| `frameworks/native/request/include/response_message_receiver.h` | UDS 接收与二进制解析（魔数 0x43434646） |
| `frameworks/native/request/libdownload_single.map` | request_native 符号白名单 |
| `frameworks/native/request_action/` | inner_kits Action（路径标准化/权限校验/转发） |
| `frameworks/native/request_next/src/proxy/`、`listen/uds.rs` | Rust 客户端 IPC/UDS（独立实现） |
| `frameworks/js/napi/request/src/request_module.cpp` | NAPI 模块注册（nm_modname="request"） |
| `frameworks/js/napi/request/src/js_task.cpp` | JS Task 对象封装 |
| `frameworks/js/napi/request/src/request_event.cpp` | 事件订阅分发（V9/V10 双表） |
| `frameworks/js/napi/request/src/js_response_listener.cpp` | UDS→JS 回调线程切换 |
| `frameworks/ets/ani/request/ets/@ohos.request.ets` | 新 ANI 胶水（loadLibrary "request_ani"） |
| `frameworks/cj/ffi/libcj_request_ffi.map` | Cangjie FFI 符号（Ffi*） |
| `interfaces/inner_kits/request_action/include/request_action.h` | 文件传输代理公共入口 |
| `interfaces/inner_kits/running_count/include/running_task_count.h` | 运行计数订阅公共入口 |

### 注意事项 / 外部依赖

- 对外 JS/ArkTS API 的 d.ts 定义不在本仓库（在 interface/sdk 仓）；本仓库 `.ets` 胶水只是加载原生库的引导。
- `ani_rs`、`cj_bind_ffi`/`cj_bind_native` 来自 napi/ets_frontend 组件，是语言绑定运行时。
- 服务端对应知识见 `docs/agent/feature-service.md`（IPC 码、UDS 格式的 source of truth 在服务端）。
