# AGENTS.md

## 基本信息

| 属性 | 值 |
| ----- | ----- |
| 代码仓名称 | request |
| 所属子系统 | request |
| 主要语言 | C++、rust |

## 目录结构

```
/base/request/request
├── common/              # 公共工具和模块
│   ├── utils/          # 通用工具 (日志、文件、哈希、LRU缓存等)
│   ├── request_core/   # 请求核心模块
│   ├── netstack_rs/    # 网络栈 Rust 封装
│   ├── ffrt_rs/        # FFRT (Foundation Foundation Runtime) Rust 封装
│   ├── database/       # 数据库模块
│   ├── sys_event/      # 系统事件
│   └── utf8_utils/     # UTF8 工具
├── frameworks/          # 框架实现
│   ├── native/         # 原生 C++/Rust 实现
│   │   ├── request/           # 请求模块
│   │   ├── request_next/      # 新请求模块 (Rust)
│   │   ├── request_action/    # 请求动作
│   │   ├── cache_download/    # 缓存下载
│   │   └── cache_core/        # 缓存核心
│   ├── js/             # JS NAPI 实现
│   │   └── napi/       # NAPI 绑定层
│   ├── ets/            # ArkTS 实现
│   │   └── ani/        # ANI (Ark Native Interface)
│   └── cj/             # Cangjie 语言支持
│       └── ffi/        # FFI 绑定
├── services/            # 服务实现
│   └── src/            # 下载服务主逻辑
├── interfaces/          # 接口定义
│   └── inner_kits/     # 内部 API
├── test/                # 测试
│   ├── unittest/       # 单元测试
│   │   ├── cpp_test/   # C++ 测试
│   │   ├── js_test/    # JS/ArkTS 测试
│   │   └── common/     # 公共测试工具
│   ├── fuzztest/       # 模糊测试
│   ├── rustest/        # Rust 测试
│   └── pretest/        # 预测试
├── etc/                 # 配置文件
│   ├── init/           # 服务启动配置
│   ├── sa_profile/     # SA (System Ability) 配置
│   └── icon/           # 图标配置
├── figures/             # 架构图
├── Cargo.toml          # Rust workspace 配置
├── rustfmt.toml        # Rust 格式化配置
└── bundle.json         # 组件元数据
```

## 代码仓概览

### 简介

本项目是 OpenHarmony Request 子系统，为第三方应用提供下载和上传服务。支持创建、删除、暂停、恢复和查询下载/上传任务。

### 核心功能

- 下载服务: 提供文件下载能力，支持断点续传、多任务并发、进度回调
- 上传服务: 提供文件上传能力，支持表单数据上传、进度监控
- 缓存下载: 提供预加载和缓存管理能力，优化重复下载体验
- 文件传输代理: 提供跨进程文件传输能力，支持后台任务管理

### 主要依赖

- rust_cxx: Rust 与 C++ 互操作桥接
- ylong_runtime: Rust 异步运行时
- os_account: 系统账号管理
- samgr: 系统能力管理器
- init: 系统初始化
- ipc: 进程间通信
- certificate_manager: 证书管理
- netmanager_base: 网络管理基础服务
- eventhandler: 事件处理
- ability_runtime: 能力运行时
- relational_store: 关系型数据库存储
- hilog: 系统日志
- hisysevent: 系统事件

## 技术栈

### 编程语言

- Rust (核心实现，310+ 文件)
- C++ (FFI 和系统交互，113+ 文件)
- TypeScript/ArkTS (JS/ETS API 层，58+ 文件)

### 框架与库

- request_native: 原生请求库 (C++)
- request: JS NAPI 请求模块
- cache_download: 缓存下载库
- preload_native: 预加载原生库
- preload_napi: 预加载 NAPI 模块
- cj_request_ffi: Cangjie FFI 绑定
- ani_package: ArkTS 包
- download_server: 下载服务

### 构建工具

- GN: 构建配置
- Ninja: 构建执行


## 构建与测试

### 构建命令
```bash
# cd 到 ../../../ 目录下运行， 依赖 build.sh 文件
./build.sh --product-name rk3568 --build-target out/rk3568/build_configs/request/request:request --no-indep
```

### 构建测试
```bash
# cd 到 ../../../ 目录下运行， 依赖 build.sh 文件
./build.sh --product-name rk3568 --build-target out/rk3568/build_configs/request/request:request_test --no-indep
```

### 产物位置
../../../out/rk3568/request/request
