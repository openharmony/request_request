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

#include "upload_task_napi.h"
#include <uv.h>
#include "async_call.h"
#include "js_util.h"
#include "upload_task.h"
#include "napi_base_context.h"
#include "napi_data_ability_operation.h"
#include "upload_task_napiV5.h"

using namespace OHOS::AppExecFwk;
using namespace OHOS::Request::Upload;
namespace OHOS::Request::UploadNapi {
std::map<std::string, UploadTaskNapi::Exec> UploadTaskNapi::onTypeHandlers_ = {
    {"progress", UploadTaskNapi::OnProgress},
    {"headerReceive", UploadTaskNapi::OnHeaderReceive},
    {"fail", UploadTaskNapi::OnFail},
    {"complete", UploadTaskNapi::OnComplete},
};
std::map<std::string, UploadTaskNapi::Exec> UploadTaskNapi::offTypeHandlers_ = {
    {"progress", UploadTaskNapi::OffProgress},
    {"headerReceive", UploadTaskNapi::OffHeaderReceive},
    {"fail", UploadTaskNapi::OffFail},
    {"complete", UploadTaskNapi::OffComplete},
};

napi_value UploadTaskNapi::JsUpload(napi_env env, napi_callback_info info)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter JsUpload.");

    std::shared_ptr<UploadTaskNapiV5> proxy = std::make_shared<UploadTaskNapiV5>(env);
    if (proxy->ParseCallback(env, info)) {
        return proxy->JsUpload(env, info);
    }
    proxy->SetEnv(nullptr);
    struct ContextInfo {
        napi_ref ref = nullptr;
    };
    auto ctxInfo = std::make_shared<ContextInfo>();
    auto input = [ctxInfo](napi_env env, size_t argc, napi_value *argv, napi_value self) -> napi_status {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Upload parser to native params %{public}d!", static_cast<int>(argc));
        NAPI_ASSERT_BASE(env, (argc > 0) && (argc <= 2), " need 1 or 2 parameters!", napi_invalid_arg);
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

napi_status UploadTaskNapi::ParseParam(napi_env env, napi_callback_info info, bool IsRequiredParam,
    JsParam &jsParam)
{
    size_t argc = JSUtil::MAX_ARGC;
    napi_value argv[JSUtil::MAX_ARGC] = {nullptr};
    napi_status status = napi_get_cb_info(env, info, &argc, argv, &jsParam.self, nullptr);
    if (status != napi_ok) {
        return napi_invalid_arg;
    }
    if (jsParam.self == nullptr) {
        return napi_invalid_arg;
    }

    if (!JSUtil::CheckParamNumber(argc, IsRequiredParam)) {
        return napi_invalid_arg;
    }
    if (!JSUtil::CheckParamType(env, argv[0], napi_string)) {
        return napi_invalid_arg;
    }
    jsParam.type = JSUtil::Convert2String(env, argv[0]);
    if (onTypeHandlers_.find(jsParam.type) == onTypeHandlers_.end()) {
        return napi_invalid_arg;
    }
    if (argc == TWO_ARG) {
        if (!JSUtil::CheckParamType(env, argv[1], napi_function)) {
            return napi_invalid_arg;
        }
        jsParam.callback = argv[1];
    }
    return napi_ok;
}

napi_value UploadTaskNapi::JsOn(napi_env env, napi_callback_info info)
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

napi_value UploadTaskNapi::JsOff(napi_env env, napi_callback_info info)
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

napi_value UploadTaskNapi::JsRemove(napi_env env, napi_callback_info info)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter JsRemove.");
    auto context = std::make_shared<RemoveContextInfo>();
    auto input = [context](napi_env env, size_t argc, napi_value *argv, napi_value self) -> napi_status {
        NAPI_ASSERT_BASE(env, argc == 0, " should 0 parameter!", napi_invalid_arg);
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

napi_status UploadTaskNapi::OnProgress(napi_env env, napi_value callback, napi_value self)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OnProgress.");
    UploadTaskNapi *proxy = nullptr;
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);

    std::shared_ptr<IProgressCallback> progressCallback = std::make_shared<ProgressCallback>(env, callback);
    if (JSUtil::Equals(env, callback, progressCallback->GetCallback()) && proxy->onProgress_ != nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OnProgress callback already register!");
        return napi_generic_failure;
    }

    proxy->napiUploadTask_->On(TYPE_PROGRESS_CALLBACK, (void *)(progressCallback.get()));
    proxy->onProgress_ = std::move(progressCallback);
    return napi_ok;
}

napi_status UploadTaskNapi::OnHeaderReceive(napi_env env, napi_value callback, napi_value self)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OnHeaderReceive.");
    UploadTaskNapi *proxy = nullptr;
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);

    std::shared_ptr<IHeaderReceiveCallback> headerReceiveCallback =
                                            std::make_shared<HeaderReceiveCallback>(env, callback);
    if (JSUtil::Equals(env, callback, headerReceiveCallback->GetCallback()) && proxy->onHeaderReceive_ != nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OnHeaderReceive callback already register!");
        return napi_generic_failure;
    }

    proxy->napiUploadTask_->On(TYPE_HEADER_RECEIVE_CALLBACK, (void *)(headerReceiveCallback.get()));
    proxy->onHeaderReceive_ = std::move(headerReceiveCallback);
    return napi_ok;
}

napi_status UploadTaskNapi::OnFail(napi_env env, napi_value callback, napi_value self)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OnFail.");
    UploadTaskNapi *proxy = nullptr;
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

napi_status UploadTaskNapi::OnComplete(napi_env env, napi_value callback, napi_value self)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OnComplete.");
    UploadTaskNapi *proxy = nullptr;
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

napi_status UploadTaskNapi::OffProgress(napi_env env, napi_value callback, napi_value self)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OffProgress.");
    UploadTaskNapi *proxy = nullptr;
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);

    if (proxy->onProgress_ == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Progress. proxy->onProgress_ == nullptr.");
        return napi_generic_failure;
    } else {
        std::shared_ptr<IProgressCallback> progressCallback = std::make_shared<ProgressCallback>(env, callback);
        proxy->napiUploadTask_->Off(TYPE_PROGRESS_CALLBACK, (void *)(progressCallback.get()));
        proxy->onProgress_ = nullptr;
    }
    return napi_ok;
}

napi_status UploadTaskNapi::OffHeaderReceive(napi_env env, napi_value callback, napi_value self)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OffHeaderReceive.");
    UploadTaskNapi *proxy = nullptr;
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


napi_status UploadTaskNapi::OffFail(napi_env env, napi_value callback, napi_value self)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OffFail.");
    UploadTaskNapi *proxy = nullptr;
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


napi_status UploadTaskNapi::OffComplete(napi_env env, napi_value callback, napi_value self)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OffComplete.");
    UploadTaskNapi *proxy = nullptr;
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

UploadTaskNapi &UploadTaskNapi::operator=(std::shared_ptr<Upload::UploadTask> &&uploadTask)
{
    if (napiUploadTask_ == uploadTask) {
        return *this;
    }
    napiUploadTask_ = std::move(uploadTask);
    return *this;
}

bool UploadTaskNapi::operator==(const std::shared_ptr<Upload::UploadTask> &uploadTask)
{
    return napiUploadTask_ == uploadTask;
}

napi_value UploadTaskNapi::GetCtor(napi_env env)
{
    napi_value cons = nullptr;
    napi_property_descriptor clzDes[] = {
        DECLARE_NAPI_METHOD("on", JsOn),
        DECLARE_NAPI_METHOD("off", JsOff),
        DECLARE_NAPI_METHOD("remove", JsRemove),
    };
    NAPI_CALL(env, napi_define_class(env, "UploadTaskNapi", NAPI_AUTO_LENGTH, Initialize, nullptr,
                                     sizeof(clzDes) / sizeof(napi_property_descriptor), clzDes, &cons));
    return cons;
}

napi_value UploadTaskNapi::Initialize(napi_env env, napi_callback_info info)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "constructor upload task!");
    napi_value self = nullptr;
    size_t argc = JSUtil::MAX_ARGC;
    napi_value argv[JSUtil::MAX_ARGC] = {nullptr};
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, argv, &self, nullptr));
    auto *proxy = new (std::nothrow) UploadTaskNapi();
    if (proxy == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Failed to create UploadTaskNapi");
        NAPI_ASSERT(env, false, "Failed to create UploadTaskNapi");
        return nullptr;
    }
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;
    int paramPosition = 0;
    napi_status getStatus = GetContext(env, &argv[0], paramPosition, context);
    if (getStatus != napi_ok) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Initialize. GetContext fail.");
        delete proxy;
        NAPI_ASSERT(env, false, "Initialize. GetContext fail");
        return nullptr;
    }

    proxy->napiUploadConfig_ = JSUtil::ParseUploadConfig(env, argv[paramPosition], "");
    if (proxy->napiUploadConfig_ == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Initialize. ParseUploadConfig fail.");
        delete proxy;
        NAPI_ASSERT(env, false, "Initialize. ParseUploadConfig fail");
        return nullptr;
    }

    proxy->napiUploadTask_ = std::make_shared<Upload::UploadTask>(proxy->napiUploadConfig_);
    proxy->napiUploadTask_->SetContext(context);
    proxy->napiUploadTask_->ExecuteTask();
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Initialize. GetAndSetContext[%{public}d]", getStatus);
    auto finalize = [](napi_env env, void * data, void * hint) {
        UploadTaskNapi *proxy = reinterpret_cast<UploadTaskNapi *>(data);
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "UploadTaskNapi. delete.");
        proxy->napiUploadTask_->Remove();
        delete proxy;
    };
    UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "UploadTaskNapi. napi_wrap OK.");
    if (napi_wrap(env, self, proxy, finalize, nullptr, nullptr) != napi_ok) {
        finalize(env, proxy, nullptr);
        NAPI_ASSERT(env, false, "napi_wrap fail");
        return nullptr;
    }
    return self;
}

napi_status UploadTaskNapi::GetContext(napi_env env, napi_value *argv, int& paramPosition,
    std::shared_ptr<OHOS::AbilityRuntime::Context>& context)
{
    bool stageMode = false;
    napi_status status = OHOS::AbilityRuntime::IsStageContext(env, argv[0], stageMode);
    if (status != napi_ok) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "GetAndSetContext. API8");
        auto ability = OHOS::AbilityRuntime::GetCurrentAbility(env);
        if (ability == nullptr) {
            UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "GetAndSetContext. API8. GetCurrentAbility ability == nullptr.");
            return napi_generic_failure;
        }
        context = ability->GetAbilityContext();
    } else {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "GetAndSetContext. API9");
        paramPosition = 1;
        if (stageMode) {
            context = OHOS::AbilityRuntime::GetStageModeContext(env, argv[0]);
            if (context == nullptr) {
                UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI,
                    "GetAndSetContext. API9. GetStageModeContext contextRtm == nullptr.");
                return napi_generic_failure;
            }
        } else {
            auto ability = OHOS::AbilityRuntime::GetCurrentAbility(env);
            if (ability == nullptr) {
                UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "GetAndSetContext. API9. GetCurrentAbility ability == nullptr.");
                return napi_generic_failure;
            }
            context = ability->GetAbilityContext();
        }
    }
    if (context == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "GetAndSetContext failed. context is nullptr.");
        return napi_generic_failure;
    }
    return napi_ok;
}
} // namespace OHOS::Request::UploadNapi

