/*
 * Copyright (c) 2022 Huawei Device Co., Ltd.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "upload_task_napiV9.h"
#include <uv.h>
#include "async_call.h"
#include "js_util.h"
#include "constant.h"
#include "upload_task.h"
#include "upload_config.h"
#include "napi_base_context.h"
#include "file_adapterV9.h"
#include "obtain_fileV9.h"
#include "napi_data_ability_operation.h"

using namespace OHOS::AppExecFwk;
using namespace OHOS::Request::Upload;
namespace OHOS::Request::UploadNapi {
std::map<std::string, UploadTaskNapiV9::Exec> UploadTaskNapiV9::onTypeHandlers_ = {
    {"progress", UploadTaskNapiV9::OnProgress},
    {"headerReceive", UploadTaskNapiV9::OnHeaderReceive},
    {"fail", UploadTaskNapiV9::OnFail},
    {"complete", UploadTaskNapiV9::OnComplete},
};
std::map<std::string, UploadTaskNapiV9::Exec> UploadTaskNapiV9::offTypeHandlers_ = {
    {"progress", UploadTaskNapiV9::OffProgress},
    {"headerReceive", UploadTaskNapiV9::OffHeaderReceive},
    {"fail", UploadTaskNapiV9::OffFail},
    {"complete", UploadTaskNapiV9::OffComplete},
};

napi_value UploadTaskNapiV9::JsUploadFile(napi_env env, napi_callback_info info)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter JsUpload.");
    struct ContextInfo {
        napi_ref ref = nullptr;
    };
    auto ctxInfo = std::make_shared<ContextInfo>();
    auto input = [ctxInfo](napi_env env, size_t argc, napi_value *argv, napi_value self) -> napi_status {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Upload parser to native params %{public}d!", static_cast<int>(argc));
        if (argc != 2) {
            JSUtil::ThrowError(env, Download::EXCEPTION_PARAMETER_CHECK, "need 2 parameters!");
            return napi_invalid_arg;
        }
        
        napi_value uploadProxy = nullptr;
        napi_status status = napi_new_instance(env, GetCtor(env), argc, argv, &uploadProxy);
        if ((uploadProxy == nullptr) || (status != napi_ok)) {
            return napi_generic_failure;
        }
        napi_create_reference(env, uploadProxy, 1, &(ctxInfo->ref));
        return napi_ok;
    };
    auto output = [ctxInfo](napi_env env, napi_value *result) -> napi_status {
        napi_status status = napi_get_reference_value(env, ctxInfo->ref, result);
        napi_delete_reference(env, ctxInfo->ref);
        return status;
    };
    auto context = std::make_shared<AsyncCall::Context>(input, output);
    AsyncCall asyncCall(env, info, context);
    return asyncCall.Call(env);
}

napi_value UploadTaskNapiV9::GetCtor(napi_env env)
{
    napi_value cons = nullptr;
    napi_property_descriptor clzDes[] = {
        DECLARE_NAPI_METHOD("on", JsOn),
        DECLARE_NAPI_METHOD("off", JsOff),
        DECLARE_NAPI_METHOD("delete", JsDelete),
    };
    napi_status status =  napi_define_class(env, "UploadTaskNapiV9", NAPI_AUTO_LENGTH, Initialize, nullptr,
                                     sizeof(clzDes) / sizeof(napi_property_descriptor), clzDes, &cons);
    if (status != napi_ok || cons == nullptr) {
        return nullptr;
    }
    return cons;
}

napi_value UploadTaskNapiV9::Initialize(napi_env env, napi_callback_info info)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "constructor upload task!");
    napi_value self = nullptr;
    auto *proxy = new (std::nothrow) UploadTaskNapiV9();
    if (proxy == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Failed to create UploadTaskNapiV9");
        return nullptr;
    }

    napi_status status = InitParam(env, info, self, proxy);
    if (status != napi_ok) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Failed to InitParam");
        delete proxy;
        return nullptr;
    }

    proxy->napiUploadTask_ = std::make_unique<Upload::UploadTask>(proxy->napiUploadConfig_);
    if (proxy->napiUploadTask_ == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Failed to create UploadTask");
        delete proxy;
        return nullptr;
    }

    proxy->napiUploadTask_->SetContext(proxy->context_);
    bool isStage = true;
    proxy->napiUploadTask_->SetFileParam(proxy->fileDatas_, proxy->totalSize_, isStage);
    proxy->napiUploadTask_->ExecuteTask();
    
    auto finalize = [](napi_env env, void * data, void * hint) {
        UploadTaskNapiV9 *proxy = reinterpret_cast<UploadTaskNapiV9 *>(data);
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "UploadTaskNapiV9. delete.");
        delete proxy;
    };

    if (napi_wrap(env, self, proxy, finalize, nullptr, nullptr) != napi_ok) {
        finalize(env, proxy, nullptr);
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "UploadTaskNapiV9. napi_wrap fail.");
        return nullptr;
    }
    return self;
}

napi_status UploadTaskNapiV9::InitParam(napi_env env, napi_callback_info info, napi_value &self,
    UploadTaskNapiV9 *proxy)
{
    size_t argc = JSUtil::MAX_ARGC;
    napi_value argv[JSUtil::MAX_ARGC] = {nullptr};
    NAPI_CALL_BASE(env, napi_get_cb_info(env, info, &argc, argv, &self, nullptr), napi_invalid_arg);

    napi_status getStatus = GetContext(env, &argv[0], proxy->context_);
    if (getStatus != napi_ok) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Initialize. GetContext fail.");
        JSUtil::ThrowError(env, Download::EXCEPTION_OTHER, "GetContext fail");
        return napi_invalid_arg;
    }

    proxy->napiUploadConfig_ = JSUtil::ParseUploadConfig(env, argv[1]);
    if (proxy->napiUploadConfig_ == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Initialize. ParseConfig fail.");
        JSUtil::ThrowError(env, Download::EXCEPTION_PARAMETER_CHECK, "config error!");
        return napi_invalid_arg;
    }

    uint32_t ret = InitFileArray(proxy->napiUploadConfig_, proxy->context_, proxy->totalSize_, proxy->fileDatas_);
    if (ret != UPLOAD_OK) {
        std::string msg;
        JSUtil::GetMessage(proxy->fileDatas_, msg);
        JSUtil::ThrowError(env, static_cast<Download::ExceptionErrorCode>(ret), msg);
        return napi_invalid_arg;
    }
    return napi_ok;
}

napi_status UploadTaskNapiV9::GetContext(napi_env env, napi_value *argv,
    std::shared_ptr<OHOS::AbilityRuntime::Context>& context)
{
    bool stageMode = false;
    napi_status status = OHOS::AbilityRuntime::IsStageContext(env, argv[0], stageMode);
    if ((status != napi_ok) || (!stageMode)) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "GetContext fail");
        return napi_generic_failure;
    }

    context = OHOS::AbilityRuntime::GetStageModeContext(env, argv[0]);
    if (context == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI,
            "GetAndSetContext. L8. GetStageModeContext contextRtm == nullptr.");
        return napi_generic_failure;
    }
    return napi_ok;
}

uint32_t UploadTaskNapiV9::InitFileArray(const std::shared_ptr<Upload::UploadConfig> &config,
    std::shared_ptr<OHOS::AbilityRuntime::Context> &context, int64_t &totalSize, std::vector<FileData> &fileDatas)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "InitFileArray. In.");
    unsigned int fileSize = 0;
    FileData data;
    FILE *file;
    totalSize = 0;
    uint32_t initResult = UPLOAD_OK;
    ObtainFileV9 obtainFile;
    uint32_t index = 1;
    for (auto f : config->files) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "filename is %{public}s", f.filename.c_str());
        data.result = Download::EXCEPTION_OTHER;
        uint32_t ret = obtainFile.GetFile(&file, f.uri, fileSize, context);
        if (ret != UPLOAD_OK) {
            initResult = data.result;
            data.result = ret;
        }

        data.fp = file;
        std::size_t position = f.uri.find_last_of("/");
        if (position != std::string::npos) {
            data.filename = std::string(f.uri, position + 1);
            data.filename.erase(data.filename.find_last_not_of(" ") + 1);
        }
        data.name = f.name;
        data.type = f.type;
        data.fileIndex = index++;
        data.adp = nullptr;
        data.upsize = 0;
        data.totalsize = fileSize;
        data.list = nullptr;
        data.headSendFlag = 0;
        data.httpCode = 0;
        
        fileDatas.push_back(data);
        totalSize += static_cast<int64_t>(fileSize);
    }

    return initResult;
}

napi_status UploadTaskNapiV9::ParseParam(napi_env env, napi_callback_info info, bool IsRequiredParam,
    JsParam &jsParam)
{
    size_t argc = JSUtil::MAX_ARGC;
    napi_value argv[JSUtil::MAX_ARGC] = {nullptr};
    napi_status status = napi_get_cb_info(env, info, &argc, argv, &jsParam.self, nullptr);
    if (status != napi_ok) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "napi_get_cb_info is fail");
        return napi_invalid_arg;
    }
    if (jsParam.self == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "jsParam.self is nullptr");
        return napi_invalid_arg;
    }

    if (!JSUtil::CheckParamNumber(argc, IsRequiredParam)) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "CheckParamNumber is fail");
        return napi_invalid_arg;
    }
    if (!JSUtil::CheckParamType(env, argv[0], napi_string)) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "argv[0] CheckParamType is fail");
        return napi_invalid_arg;
    }     
    jsParam.type = JSUtil::Convert2String(env, argv[0]);
    if (onTypeHandlers_.find(jsParam.type) == onTypeHandlers_.end()) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "type find fail");
        return napi_invalid_arg;
    }
    if (argc == TWO_ARG) {
        if (!JSUtil::CheckParamType(env, argv[1], napi_function)) {
            UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "argv[1] CheckParamType is fail");
            return napi_invalid_arg;
        }
        jsParam.callback = argv[1];
    }
    return napi_ok;
}

napi_value UploadTaskNapiV9::JsOn(napi_env env, napi_callback_info info)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter JsOn.");
    bool IsRequiredParam = true;
    JsParam jsParam;
    napi_status status = ParseParam(env, info, IsRequiredParam, jsParam);
    NAPI_ASSERT(env, status == napi_ok, "ParseParam fail");
    auto handle = onTypeHandlers_.find(jsParam.type);
    handle->second(env, jsParam.callback, jsParam.self);
    return nullptr;
}

napi_value UploadTaskNapiV9::JsOff(napi_env env, napi_callback_info info)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter JsOff.");
    bool IsRequiredParam = false;
    JsParam jsParam;
    napi_status status = ParseParam(env, info, IsRequiredParam, jsParam);
    NAPI_ASSERT(env, status == napi_ok, "ParseParam fail");
    auto handle = offTypeHandlers_.find(jsParam.type);
    handle->second(env, jsParam.callback, jsParam.self);
    return nullptr;
}

napi_value UploadTaskNapiV9::JsDelete(napi_env env, napi_callback_info info)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter JsRemove.");
    auto context = std::make_shared<RemoveContextInfo>();
    auto input = [context](napi_env env, size_t argc, napi_value *argv, napi_value self) -> napi_status {
        if (argc != 0) {
            JSUtil::ThrowError(env, Download::EXCEPTION_PARAMETER_CHECK, "should 0 parameter!");
            return napi_invalid_arg;
        }
        return napi_ok;
    };
    auto output = [context](napi_env env, napi_value *result) -> napi_status {
        napi_status status = napi_get_boolean(env, context->removeStatus, result);
        return status;
    };
    auto exec = [context](AsyncCall::Context *ctx) {
        context->removeStatus = context->proxy->napiUploadTask_->Remove();
        if (context->removeStatus == true) {
            context->status = napi_ok;
        }
    };
    context->SetAction(std::move(input), std::move(output));
    AsyncCall asyncCall(env, info, std::dynamic_pointer_cast<AsyncCall::Context>(context));
    return asyncCall.Call(env, exec);
}

napi_status UploadTaskNapiV9::OnProgress(napi_env env, napi_value callback, napi_value self)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OnProgress.");
    UploadTaskNapiV9 *proxy = nullptr;
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);

    std::shared_ptr<IProgressCallback> progressCallback = std::make_shared<ProgressCallback>(env, callback);
    if (JSUtil::Equals(env, callback, progressCallback->GetCallback()) && proxy->onFail_ != nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OnProgress callback already register!");
        return napi_generic_failure;
    }

    proxy->napiUploadTask_->On(TYPE_PROGRESS_CALLBACK, (void *)(progressCallback.get()));
    proxy->onProgress_ = std::move(progressCallback);
    return napi_ok;
}

napi_status UploadTaskNapiV9::OnHeaderReceive(napi_env env, napi_value callback, napi_value self)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OnHeaderReceive.");
    UploadTaskNapiV9 *proxy = nullptr;
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);

    std::shared_ptr<IHeaderReceiveCallback> headerReceiveCallback =
        std::make_shared<HeaderReceiveCallback>(env, callback);
    if (JSUtil::Equals(env, callback, headerReceiveCallback->GetCallback()) && proxy->onFail_ != nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OnHeaderReceive callback already register!");
        return napi_generic_failure;
    }

    proxy->napiUploadTask_->On(TYPE_HEADER_RECEIVE_CALLBACK, (void *)(headerReceiveCallback.get()));
    proxy->onHeaderReceive_ = std::move(headerReceiveCallback);
    return napi_ok;
}

napi_status UploadTaskNapiV9::OnFail(napi_env env, napi_value callback, napi_value self)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OnFail.");
    UploadTaskNapiV9 *proxy = nullptr;
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);

    std::shared_ptr<INotifyCallback> failCallback = std::make_shared<NotifyCallback>(env, callback);
    if (JSUtil::Equals(env, callback, failCallback->GetCallback()) && proxy->onFail_ != nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OnFail callback already register!");
        return napi_generic_failure;
    }

    proxy->napiUploadTask_->On(TYPE_FAIL_CALLBACK, (void *)(failCallback.get()));
    proxy->onFail_ = std::move(failCallback);
    return napi_ok;
}

napi_status UploadTaskNapiV9::OnComplete(napi_env env, napi_value callback, napi_value self)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OnComplete.");
    UploadTaskNapiV9 *proxy = nullptr;
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);

    std::shared_ptr<INotifyCallback> completeCallback = std::make_shared<NotifyCallback>(env, callback);
    if (JSUtil::Equals(env, callback, completeCallback->GetCallback()) && proxy->onComplete_ != nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OnComplete callback already register!");
        return napi_generic_failure;
    }

    proxy->napiUploadTask_->On(TYPE_COMPLETE_CALLBACK, (void *)(completeCallback.get()));
    proxy->onComplete_ = std::move(completeCallback);
    return napi_ok;
}

napi_status UploadTaskNapiV9::OffProgress(napi_env env, napi_value callback, napi_value self)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OffProgress.");
    UploadTaskNapiV9 *proxy = nullptr;
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);

    if (proxy->onProgress_ == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Progress. proxy->onProgress_ == nullptr.");
        return napi_generic_failure;
    } else {
        std::shared_ptr<IProgressCallback>  progressCallback =
            std::make_shared<ProgressCallback>(env, callback);
        proxy->napiUploadTask_->Off(TYPE_PROGRESS_CALLBACK, (void *)(progressCallback.get()));
        proxy->onProgress_ = nullptr;
    }
    return napi_ok;
}

napi_status UploadTaskNapiV9::OffHeaderReceive(napi_env env, napi_value callback, napi_value self)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OffHeaderReceive.");
    UploadTaskNapiV9 *proxy = nullptr;
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);

    if (proxy->onHeaderReceive_ == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "HeaderReceive. proxy->onHeaderReceive_ == nullptr.");
        return napi_generic_failure;
    } else {
        std::shared_ptr<IHeaderReceiveCallback> headerReceiveCallback =
            std::make_shared<HeaderReceiveCallback>(env, callback);
        proxy->napiUploadTask_->Off(TYPE_HEADER_RECEIVE_CALLBACK, (void *)(headerReceiveCallback.get()));
        proxy->onHeaderReceive_ = nullptr;
    }
    return napi_ok;
}


napi_status UploadTaskNapiV9::OffFail(napi_env env, napi_value callback, napi_value self)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OffFail.");
    UploadTaskNapiV9 *proxy = nullptr;
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);

    if (proxy->onFail_ == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Fail. proxy->onFail_ == nullptr.");
        return napi_generic_failure;
    } else {
        std::shared_ptr<INotifyCallback> failCallback = std::make_shared<NotifyCallback>(env, callback);
        proxy->napiUploadTask_->Off(TYPE_FAIL_CALLBACK, failCallback.get());
        proxy->onFail_ = nullptr;
    }
    return napi_ok;
}


napi_status UploadTaskNapiV9::OffComplete(napi_env env, napi_value callback, napi_value self)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OffComplete.");
    UploadTaskNapiV9 *proxy = nullptr;
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);
    if (proxy->onComplete_ == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "CompleteCallback. proxy->OffComplete_ == nullptr.");
        return napi_generic_failure;
    } else {
        std::shared_ptr<INotifyCallback> completeCallback = std::make_shared<NotifyCallback>(env, callback);
        proxy->napiUploadTask_->Off(TYPE_COMPLETE_CALLBACK, completeCallback.get());
        proxy->onComplete_ = nullptr;
    }
    return napi_ok;
}

UploadTaskNapiV9 &UploadTaskNapiV9::operator=(std::unique_ptr<Upload::UploadTask> &&uploadTask)
{
    if (napiUploadTask_ == uploadTask) {
        return *this;
    }
    napiUploadTask_ = std::move(uploadTask);
    return *this;
}

bool UploadTaskNapiV9::operator==(const std::unique_ptr<Upload::UploadTask> &uploadTask)
{
    return napiUploadTask_ == uploadTask;
}
} // namespace OHOS::Request::UploadNapi