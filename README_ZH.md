

# Request服务
##  简介
### 内容介绍

Request服务向三方应用提供文件下载/上传能力，以支撑应用开发者方便、高效地使用下载/上传业务的功能。

下载支持后台管理能力，通过download（downloadFile）接口新建任务，请求成功后会通过异步方式返回task对象。用户可以通过task对象管理任务，支持新建、移除、暂停、恢复以及查询任务，以及订阅任务进度、成功、失败等能力。

上传任务在客户端运行，通过upload（uploadFile）接口新建任务，请求成功后会通过异步方式返回task对象。上传支持新建、移除任务，以及订阅任务进度、成功、失败等能力。

### 架构图介绍

**图 1** 下载子系统架构图

![](figures/download_archit.png "子系统架构图")

系统应用：依赖的系统应用如浏览器等。

三方应用：依赖的第三方应用如淘宝等。

下载 API：提供下载功能的接口。

IDL：提供NAPI之间的相互通信。

下载服务：负责下载的具体过程，客户端发起一个下载请求，下载内部创建下载任务，处理下载请求并完成下载任务。

**图 2** 上传子系统架构图
![](figures/upload_archit.png "子系统架构图")

扩展/三方 应用：上传功能的客户端，是上传的发起方。

JS接口：对外提供上传功能JS接口。

本地接口：上传本地文件的接口。

上传任务：客户端每发起一个上传请求，上传内部都会创建一个上传任务，用于处理上传请求，转发服务器的上传应答。

libcurl: 封装libcurl接口完成上传。

## 目录

```
/base/request/request
├── figures                                # 架构图
├── download/ability                       # 下载服务数据库管理模块
├── download/etc 	                       # 下载服务包含的进程配置文件
	 └──interfaces/kits/js/napi            # 本组件对外提供的下载服务接口代码
		 └──download_single                # 下载服务的napi接口
├── download/sa_profile                    # 下载服务包含的系统服务的配置文件
├── download/services                      # 下载系统服务实现
├── download/utils                         # 下载服务包含日志打印和公共事件定义的常量
├── upload/frameworks                      # 上传服务功能实现
├── upload/interfaces/kits                 # 本组件对外提供的上传服务接口代码
│   ├── js                                 # 本组件js接口定义
│   └── napi                               # 上传服务的napi接口
└── upload/unitest                         # 上传模块的单元测试
```

##  约束

-   如需使用request服务，需要申请如下权限：
    -   ohos.permission.INTERNET
    -   ohos.permission.WRITE_MEDIA
    -   ohos.permission.READ_MEDIA
-   request数据单元为文件形式，其余数据形式需要调用者自行封装为文件路径。
-   request服务不提供完整的HTTP/HTTPS SDK接口，只是HTTP/HTTPS SDK 的用户，如需此接口推荐使用[NetStack](https://gitee.com/openharmony/communication_netstack)。
-   下载服务器需要支持HTTP协议的head方法，能够通过Content-length获取下载数据大小，否则下载任务失败。
-   下载时用户指定文件已存在，会在创建任务时校验并抛出异常，创建任务失败。
-   允许用户指定多文件上传成功策略：多文件在同一个任务中上传，以任务维度为判断标准，必须所有文件上传成功判定为成功。


##  说明
###  接口说明
-   Request接口文档
    -   [js-apis-request.md](https://gitee.com/openharmony/docs/blob/master/zh-cn/application-dev/reference/apis/js-apis-request.md) 提供从API version 6开始接口定义和示例代码。
    -   [js-apis-system-request.md](https://gitee.com/openharmony/docs/blob/master/zh-cn/application-dev/reference/apis/js-apis-system-request.md) 提供从API version 3开始接口定义和示例代码，此文档接口从API version 9开始已全部废弃，不推荐使用。



##  相关仓

[request_request](https://gitee.com/openharmony/request_request)