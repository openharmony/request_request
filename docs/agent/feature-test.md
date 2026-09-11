# feature-test.md — 测试体系与执行方法领域知识

> 本文档为 Agent 知识路由目标，由 AGENTS.md 的 task-based / path-based / vocabulary-based routing 触发阅读。
> Agent 能自己探索出来的少写；Agent 猜不准、猜错代价高、团队必须统一执行的内容要写。

## 1. 代码地图

本文档适用于 `test/` 全目录、`services/tests/`、各 common crate 与 frameworks 模块内嵌的 Rust UT，以及一切"怎么编译、怎么跑、怎么判读结果"的问题。

测试全景：

| 类别 | 位置 | 执行环境 | 说明 |
|---|---|---|---|
| C++ 单测 | `test/unittest/cpp_test/`（7 套件） | 设备（xtest） | `ohos_unittest` |
| JS/HAP 用例 | `test/unittest/js_test/`（5 套件） | 设备（start.sh） | Hypium，FA/stage 两代模板 |
| Fuzz | `test/fuzztest/`（15 target） | 主机/CI | `ohos_fuzztest`，libFuzzer |
| Rust UT/集成 | `services/tests/`、各 crate `tests/ut/` | 设备（xtest）+ 主机（cargo） | `ohos_rust_unittest` |
| 预测试 HAP | `test/pretest/` | 手动 | 独立 HAP 工程手动页面 |
| 测试公共库 | `test/unittest/common/`、`test/rustest/` | — | dir_operation、set_permission、test_common |

C++ 套件：`fwkTest`（框架/parcel/proxy/receiver）、`innerTest`（request_action/path_control/task_builder）、`saTest`（证书适配/字符串包装）、`preload`（预加载 7 类场景）、`preloadNapi`、`netstack`（common_netstack_test）、`inotifyImage`。

JS 套件：`requestTest`/`requestPermissonTest`（FA 模型，config.json）、`requestAgentTaskTest`（stage，约 20 个 .test.ets）、`requestQueryTaskTest`、`requestSystemQueryTest`（stage）。注意 `bundle.json` 的 test 列表（CI 入口）只含 3 个 stage 套件；两个 FA 套件不在默认入口，改动后需确认是否有必要单独编译执行。

Fuzz target：`downloaduploadmanager_fuzzer`、`predownload_fuzzer`、`requestserviceproxy{task,remove,query,subscribe,group,touch,search,utils}_fuzzer`、`requestserviceproxymanager{1..5}_fuzzer`。注意 `requestserviceproxy_fuzzer/` 目录只是**共享头文件目录**（common.h），不是 target。

### 查找位置

- JS 用例怎么写的 → `js_test/requestAgentTaskTest/entry/src/main/ets/test/*.test.ets` + `List.test.ets`（套件聚合）
- JS 测试配置（超时/安装/清理）→ 各 js_test 目录的 `Test.json`（driver `OHJSUnitTest`、`testcase-timeout`、kits AppInstallKit/ShellKit）
- Rust 单测在哪登记 → 各 crate `lib.rs`/`mod.rs` 的 `#[cfg(test)] mod ut_xxx { include!(...) }`
- Rust 集成测试 → `services/tests/entry.rs`（注意 §3.5 的注释现状）
- fuzz 用例结构 → `test/fuzztest/<name>/`（`project.xml` + `*_fuzzer.cpp`）
- 测试权限辅助 → `test/unittest/common/set_permission`
- 设备端 mock server → `common/utils/src/test/server.rs`（主机）/ `iTcpServer.ets`（JS 侧，见 §3.6）

## 2. 知识路由

### 按任务路由

- 新增/修改任何测试 → 本文件 §3（执行纪律）+ §4（编写规范）
- 编译测试 → §3.1（构建命令与坑）
- 执行 JS 用例 → §3.2（start.sh，唯一入口）
- 执行 C++/Rust 单测 → §3.3
- fuzz 整改 → §3.4 + `.claude/skills/fuzz-api-check/SKILL.md`
- 用例失败判读 → §3.7

### 按路径路由

- `test/unittest/js_test/` → §3.2
- `test/fuzztest/` → §3.4
- `services/tests/`、`test/rustest/` → §3.3 + §3.5
- `test/pretest/` → 手动工程，不进 CI

### 按词汇路由

| 术语 | 风险提示 | 阅读 |
|---|---|---|
| start.sh | JS 用例唯一合法执行入口，手动 `aa test` 会误报超时 | §3.2 |
| testcase-timeout | 只有 start.sh 链路读取该配置 | §3.2 |
| module_out_path | 测试产物归档路径，各套件不统一（见 §3.8） | §3.8 |
| ohos_js_stage_unittest / ohos_js_unittest | stage 与 FA 两代 HAP 模板，配置文件不同 | §4.2 |
| Test.json | Hypium driver 配置（超时/安装/清理 kits） | §4.2 |
| #define private public | fuzzer 暴露私有成员的手法，改头文件会破坏 fuzzer | §3.4 |
| FuzzedDataProvider | fuzz 输入消费约定 | §4.3 |
| rust_request_sdv_test | 入口模块当前全被注释，勿当回归依据 | §3.5 |
| 常亮屏 | 设备默认 30s 灭屏，跑用例前必须设置 | §3.2 |

在计划中声明：任务类别、已读文档、发现的约束、是否应使用特定 Skill/工作流。

## 3. 执行纪律（团队统一执行）

### 3.1 编译

```bash
# cd 到源码根目录（../../../）运行
./build.sh --product-name rk3568 --build-target out/rk3568/build_configs/request/request:request_test --no-indep
```

- 必须带 `--no-indep`：单测 target 会触发独立构建，缺 `ylong_runtime.rlib` 导致全量失败；直接复用 `out/rk3568` 已有产物。
- 不使用 `--fast-rebuild`（触发 out/standard 独立构建，ICU 数据缺失失败）。
- 修改过 BUILD.gn 后不走快速构建路径。

### 3.2 JS/HAP 用例执行（start.sh，唯一入口）

```bash
# 位于源码根的 test/testfwk/developer_test/start.sh
cd <源码根>/test/testfwk/developer_test
./start.sh
```

- **禁止**手动 `aa test`：`Test.json` 的 `testcase-timeout` 只有 start.sh 链路读取，手动执行会大量误报超时。
- 执行前设设备常亮屏（默认 30s 灭屏直接导致用例失败）：
  `power-shell wakeup && power-shell setmode 602`，并把灭屏 timeout 调到足够长。
- 全量基线：**960/960 通过**（2026-08-11 起，preload 证书自动化修复后）。回归对照该基线。
- HdcMonitor 走 TCP 直连 hdc server（`192.168.128.1:8710` 可达时可用）；WSL2 内 hdc 因 libusb 不可用，设备 USB 连 Windows 侧时用 `/mnt/d/env/hdc.exe` + `wslpath` 推拉文件。

### 3.3 C++ / Rust 单测执行

- 编译出可执行后推设备用 `xtest` 执行（产物在 `out/rk3568/tests/unittest/request/request/request/`）。
- Rust UT：services 的 `rust_request_ut_test`（`module_out_path = request/request/request_rust`）、common 各 crate（`request/request/common`）。单测代码在 `services/tests/ut/`，与 `src/` 目录结构镜射。
- 主机端 `cargo test` 仅适用于无 OH FFI 依赖的 crate（request_core、database 的非 OH 路径）。

### 3.4 Fuzz

- 15 个 target 见 §1；`project.xml` 统一 `max_len=1000`、`max_total_time=300`、`rss_limit_mb=4096`。
- fuzzer 以 `-Dprivate=public` + `#define private public/protected public` 编译，且**直接编译 frameworks/common 源文件**——改 `frameworks/native/request` 或 `common/sys_event`/`utf8_utils` 的头文件/私有成员会连带破坏 fuzzer 构建，改完必须编译 fuzz target 验证。
- 本地跑 fuzz 用 thin LTO 时建议 `-k0` 隔离（历史经验）。
- driver 规范（`LLVMFuzzerTestOneInput` 内被测 API ≤5 个左右）见 `.claude/skills/fuzz-api-check/SKILL.md`。

### 3.5 已知失效/陷阱项

- `rust_request_sdv_test`（`services/tests/entry.rs`）：construct/search/start/resume 模块**全部被注释**（`// no selinux right now`），该 target 当前不执行有效测试，**不要**依赖它做回归结论。
- `test/`、`test/unittest/`、`test/unittest/common/`、`test/unittest/cpp_test/` 无顶层 BUILD.gn，构建入口分散在叶子目录——找 target 用叶子目录的 BUILD.gn。
- JS 套件的 target 名大小写敏感（历史 bug：`requestSystemQueryTest` 曾因小写 r 导致 json/hap 不匹配、testfwk 发现不了 hap）。

### 3.6 mock server

- Rust 主机单测：`request_utils::test::server::test_server(handler)`（见 feature-common.md §3.5）。
- JS 用例：`requestAgentTaskTest/entry/src/main/ets/common/iTcpServer.ets` 提供设备内 TCP 服务端，两者独立实现。
- 现状两套独立、按需选用；新增设备端到端用例优先复用 `iTcpServer.ets` 模式（设备内自闭环，不依赖主机网络）。

### 3.7 结果判读

- 失败先分类：用例断言失败 / 环境失败（灭屏、SELinux、网络、证书）/ 超时误报（没走 start.sh）。
- 全量跑之前先跑最小相关子集；回归以 960 基线对照，新增用例后更新基线记录。
- 涉及证书的 preload 用例失败优先查证书推送自动化是否执行（历史根因：preload 证书缺失）。

### 3.8 module_out_path 对照

| 套件 | module_out_path |
|---|---|
| cpp_test 全部 | `request/request/request` |
| js_test 各套件 | `request/request/<targetName>`（各自不同） |
| services Rust | `request/request/request_rust` |
| common crate UT | `request/request/common` |

xtest 按 module_out_path 定位产物，target 名/json/hap 名必须精确匹配（大小写敏感）。

## 4. 编写规范

### 4.1 通用

- 用例必须含显式断言；禁止不可能失败的断言。
- Rust UT 新文件必须登记到对应 `#[cfg(test)] mod` 的 `include!`（文件移动会破坏相对路径）。
- JS 用例在 `List.test.ets` 聚合登记；新建套件需完整 stage 结构（AppScope/app.json、entry/src/main/module.json、TestAbility、test/）。
- "验证回调不触发"类负向用例：用状态轮询（如 getDownloadInfo）替代固定 sleep 等待。

### 4.2 JS 模板选择

- stage 模型（新用例一律用这个）：`ohos_js_stage_unittest` + `ets2abc = true` + module.json；配置在 `Test.json`（driver `OHJSUnitTest`、bundle `com.acts.request`、module `testModule`、testcase-timeout、kits 安装/清理）。
- FA 模型（存量）：`ohos_js_unittest` + `hap_profile = "./config.json"`；仅维护存量 `requestTest`/`requestPermissonTest`，不新增。

### 4.3 Fuzz 编写

- 入口 `LLVMFuzzerTestOneInput`，用 `FuzzedDataProvider` 消费输入。
- 单个 driver 被测 API 数量控制在 5 个左右（过多会稀释 fuzz 深度）。
- 新增 fuzzer 目录含 `BUILD.gn`（`ohos_fuzztest`）+ `project.xml` + `*_fuzzer.cpp`，并在 `test/fuzztest/BUILD.gn` 登记。
- 复用 `requestserviceproxy_fuzzer/requestserviceproxy_fuzzer_common.h` 的公共工具。

## 5. 验证

### 最小检查（测试类变更）

- 编译 `request_test` target 全绿
- 被改测试所在套件单独执行通过
- 至少一个相邻相关套件执行通过

### 完成定义

- 请求的行为已实现。
- 相关构建/测试/lint/兼容性检查已执行，或已说明无法执行的原因。
- 最终回复包含：变更摘要、变更文件列表、验证结果（含基线对照）、剩余风险。
- 不包含无关的格式化、重构或附带变更。

## 6. 关键文件索引

| 文件 | 职责 |
|---|---|
| `test/unittest/js_test/requestAgentTaskTest/` | 主 JS 用例套件（stage） |
| `test/unittest/js_test/requestAgentTaskTest/entry/src/main/ets/test/List.test.ets` | 套件聚合入口 |
| `test/unittest/js_test/requestAgentTaskTest/Test.json` | driver/超时/kit 配置 |
| `test/unittest/js_test/requestAgentTaskTest/entry/src/main/ets/common/iTcpServer.ets` | JS 侧 mock TCP 服务 |
| `test/unittest/cpp_test/*/BUILD.gn` | 各 C++ 套件 target |
| `test/fuzztest/BUILD.gn` | fuzz target 登记 |
| `test/fuzztest/requestserviceproxy_fuzzer/requestserviceproxy_fuzzer_common.h` | fuzz 公共工具 |
| `services/tests/ut/` | 服务端 Rust UT（与 src/ 镜射） |
| `services/tests/entry.rs` | SDV 集成测试入口（当前模块被注释） |
| `test/rustest/src/lib.rs` | RequestAgent IPC 驱动封装（test_common） |
| `test/unittest/common/set_permission/` | 测试权限设置辅助 |
| `<源码根>/test/testfwk/developer_test/start.sh` | JS 用例执行入口（源码根即仓库上三级） |

### 注意事项 / 外部依赖

- start.sh 与 testfwk 框架在源码根 `test/testfwk/`，不在本仓——本仓只提供用例与 Test.json。
- HAP 签名文件（`signature/openharmony_sx.p7b`）随套件携带，替换需同步 BUILD.gn 的 certificate_profile。
- 设备网络：mock server 与分片下载用例需要设备与主机互通（RK3568 网口 192.168.128.x 网段）。
