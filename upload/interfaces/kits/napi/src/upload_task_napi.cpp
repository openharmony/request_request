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

napi_value UploadTaskNapi::JsOn(napi_env env, napi_callback_info info)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter JsOn.");
    napi_value self = nullptr;
    size_t argc = JSUtil::MAX_ARGC;
    napi_value argv[JSUtil::MAX_ARGC] = {nullptr};
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, argv, &self, nullptr));
    NAPI_ASSERT(env, argc > 0, "there is no args");
    napi_valuetype valueType;
    NAPI_CALL(env, napi_typeof(env, argv[0], &valueType));
    NAPI_ASSERT(env, valueType == napi_string, "type is not string");
    std::string type = JSUtil::Convert2String(env, argv[0]);

    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "on type : %{public}s", type.c_str());
    auto handle = onTypeHandlers_.find(type);
    NAPI_ASSERT(env, handle != onTypeHandlers_.end(), "invalid type");
    napi_value result = nullptr;
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "JsOn. Turn to corresponding On.");
    handle->second(env, argc - 1, &argv[1], self, &result);
    return nullptr;
}

napi_value UploadTaskNapi::JsOff(napi_env env, napi_callback_info info)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter JsOff.");
    napi_value self = nullptr;
    size_t argc = JSUtil::MAX_ARGC;
    napi_value argv[JSUtil::MAX_ARGC] = {nullptr};
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, argv, &self, nullptr));
    NAPI_ASSERT(env, argc > 0, "there is no args");
    napi_valuetype valueType;
    NAPI_CALL(env, napi_typeof(env, argv[0], &valueType));
    NAPI_ASSERT(env, valueType == napi_string, "type is not string");
    std::string type = JSUtil::Convert2String(env, argv[0]);

    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "off type : %{public}s", type.c_str());
    auto handle = offTypeHandlers_.find(type);
    NAPI_ASSERT(env, handle != offTypeHandlers_.end(), "invalid type");
    napi_value result = nullptr;
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "JsOff. Turn to corresponding Off.");
    handle->second(env, argc - 1, &argv[1], self, &result);
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

napi_status UploadTaskNapi::OnProgress(napi_env env, size_t argc, napi_value *argv, napi_value self, napi_value *result)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OnProgress.");
    NAPI_ASSERT_BASE(env, argc == 1, "argc not equals 1", napi_invalid_arg);
    NAPI_ASSERT_BASE(env, self != nullptr, "self is nullptr", napi_invalid_arg);

    napi_valuetype valueType = napi_undefined;
    napi_typeof(env, argv[0], &valueType);
    NAPI_ASSERT_BASE(env, valueType == napi_function, "callback is not a function", napi_invalid_arg);

    UploadTaskNapi *proxy = nullptr;
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);

    std::shared_ptr<IProgressCallback> callback = std::make_shared<ProgressCallback>(proxy, env, argv[0]);
    if (proxy->onProgress_ != nullptr) {
        proxy->napiUploadTask_->Off(TYPE_PROGRESS_CALLBACK, (void *)((proxy->onProgress_).get()));
    }
    proxy->offProgress_ = std::move(proxy->onProgress_);
    proxy->napiUploadTask_->On(TYPE_PROGRESS_CALLBACK, (void *)(callback.get()));
    proxy->onProgress_ = std::move(callback);
    return napi_ok;
}

napi_status UploadTaskNapi::OnHeaderReceive(napi_env env,
    size_t argc, napi_value *argv, napi_value self, napi_value *result)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OnHeaderReceive.");
    NAPI_ASSERT_BASE(env, argc == 1, "argc not equals 1", napi_invalid_arg);
    NAPI_ASSERT_BASE(env, self != nullptr, "self is nullptr", napi_invalid_arg);

    napi_valuetype valueType = napi_undefined;
    napi_typeof(env, argv[0], &valueType);
    NAPI_ASSERT_BASE(env, valueType == napi_function, "callback is not a function", napi_invalid_arg);

    UploadTaskNapi *proxy = nullptr;
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);

    std::shared_ptr<IHeaderReceiveCallback> callback = std::make_shared<HeaderReceiveCallback>(proxy, env, argv[0]);
    if (proxy->onHeaderReceive_ != nullptr) {
        proxy->napiUploadTask_->Off(TYPE_HEADER_RECEIVE_CALLBACK, (void *)((proxy->onHeaderReceive_).get()));
    }
    proxy->offHeaderReceive_ = std::move(proxy->onHeaderReceive_);
    proxy->napiUploadTask_->On(TYPE_HEADER_RECEIVE_CALLBACK, (void *)(callback.get()));
    proxy->onHeaderReceive_ = std::move(callback);
    return napi_ok;
}

napi_status UploadTaskNapi::OnFail(napi_env env, size_t argc, napi_value *argv, napi_value self, napi_value *result)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OnFail.");
    NAPI_ASSERT_BASE(env, argc == 1, "argc not equals 1", napi_invalid_arg);
    NAPI_ASSERT_BASE(env, self != nullptr, "self is nullptr", napi_invalid_arg);

    napi_valuetype valueType = napi_undefined;
    napi_typeof(env, argv[0], &valueType);
    NAPI_ASSERT_BASE(env, valueType == napi_function, "callback is not a function", napi_invalid_arg);

    UploadTaskNapi *proxy = nullptr;
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);

    std::shared_ptr<IFailCallback> callback = std::make_shared<FailCallback>(proxy, env, argv[0]);
    if (JSUtil::Equals(env, argv[0], callback->GetCallback()) && proxy->onFail_ != nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OnFail callback already register!");
        return napi_generic_failure;
    }

    proxy->napiUploadTask_->On(TYPE_FAIL_CALLBACK, (void *)(callback.get()));
    proxy->onFail_ = std::move(callback);
    return napi_ok;
}

napi_status UploadTaskNapi::OnComplete(napi_env env, size_t argc, napi_value *argv, napi_value self, napi_value *result)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OnComplete.");
    NAPI_ASSERT_BASE(env, argc == 1, "argc not equals 1", napi_invalid_arg);
    NAPI_ASSERT_BASE(env, self != nullptr, "self is nullptr", napi_invalid_arg);

    napi_valuetype valueType = napi_undefined;
    napi_typeof(env, argv[0], &valueType);
    NAPI_ASSERT_BASE(env, valueType == napi_function, "callback is not a function", napi_invalid_arg);

    UploadTaskNapi *proxy = nullptr;
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);

    std::shared_ptr<ICompleteCallback> callback = std::make_shared<CompleteCallback>(proxy, env, argv[0]);
    if (JSUtil::Equals(env, argv[0], callback->GetCallback()) && proxy->onComplete_ != nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OnComplete callback already register!");
        return napi_generic_failure;
    }

    proxy->napiUploadTask_->On(TYPE_COMPLETE_CALLBACK, (void *)(callback.get()));
    proxy->onComplete_ = std::move(callback);
    return napi_ok;
}

napi_status UploadTaskNapi::OffProgress(napi_env env,
    size_t argc, napi_value *argv, napi_value self, napi_value *result)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OffProgress.");
    NAPI_ASSERT_BASE(env, argc == 0 || argc == 1, "argc should be 0 or 1", napi_invalid_arg);
    NAPI_ASSERT_BASE(env, self != nullptr, "self is nullptr", napi_invalid_arg);

    std::shared_ptr<IProgressCallback> callback = nullptr;

    if (argc == 1) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OffProgress. argc == 1.");
        napi_valuetype valueType = napi_undefined;
        napi_typeof(env, argv[0], &valueType);
        NAPI_ASSERT_BASE(env, valueType == napi_function, "callback is not a function", napi_invalid_arg);
    }

    UploadTaskNapi *proxy = nullptr;
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);

    if (proxy->onProgress_ == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Progress. proxy->onProgress_ == nullptr.");
        return napi_generic_failure;
    } else {
        callback = std::make_shared<ProgressCallback>(proxy, env, argv[0]);
        proxy->napiUploadTask_->Off(TYPE_PROGRESS_CALLBACK, (void *)(callback.get()));
        proxy->onProgress_ = nullptr;
        proxy->offProgress_ = std::move(callback);
    }
    return napi_ok;
}

napi_status UploadTaskNapi::OffHeaderReceive(napi_env env,
    size_t argc, napi_value *argv, napi_value self, napi_value *result)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OffHeaderReceive.");
    NAPI_ASSERT_BASE(env, argc == 0 || argc == 1, "argc should be 0 or 1", napi_invalid_arg);
    NAPI_ASSERT_BASE(env, self != nullptr, "self is nullptr", napi_invalid_arg);

    std::shared_ptr<IHeaderReceiveCallback> callback = nullptr;

    if (argc == 1) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OffHeaderReceive. argc == 1.");
        napi_valuetype valueType = napi_undefined;
        napi_typeof(env, argv[0], &valueType);
        NAPI_ASSERT_BASE(env, valueType == napi_function, "callback is not a function", napi_invalid_arg);
    }

    UploadTaskNapi *proxy = nullptr;
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);

    if (proxy->onHeaderReceive_ == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "HeaderReceive. proxy->onHeaderReceive_ == nullptr.");
        return napi_generic_failure;
    } else {
        callback = std::make_shared<HeaderReceiveCallback>(proxy, env, argv[0]);
        proxy->napiUploadTask_->Off(TYPE_HEADER_RECEIVE_CALLBACK, (void *)(callback.get()));
        proxy->onHeaderReceive_ = nullptr;
        proxy->offHeaderReceive_ = std::move(callback);
    }
    return napi_ok;
}

napi_status UploadTaskNapi::OffFail(napi_env env,
    size_t argc, napi_value *argv, napi_value self, napi_value *result)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OffFail.");
    NAPI_ASSERT_BASE(env, argc == 0 || argc == 1, "argc should be 0 or 1", napi_invalid_arg);
    NAPI_ASSERT_BASE(env, self != nullptr, "self is nullptr", napi_invalid_arg);

    std::shared_ptr<IFailCallback> callback = nullptr;

    if (argc == 1) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OffFail. argc == 1.");
        napi_valuetype valueType = napi_undefined;
        napi_typeof(env, argv[0], &valueType);
        NAPI_ASSERT_BASE(env, valueType == napi_function, "callback is not a function", napi_invalid_arg);
    }

    UploadTaskNapi *proxy = nullptr;
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);

    if (proxy->onFail_ == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Fail. proxy->onFail_ == nullptr.");
        return napi_generic_failure;
    } else {
        callback = std::make_shared<FailCallback>(proxy, env, argv[0]);
        proxy->napiUploadTask_->Off(TYPE_FAIL_CALLBACK, (void *)(callback.get()));
        proxy->onFail_ = nullptr;
    }
    return napi_ok;
}

napi_status UploadTaskNapi::OffComplete(napi_env env,
    size_t argc, napi_value *argv, napi_value self, napi_value *result)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Enter OffComplete.");
    NAPI_ASSERT_BASE(env, argc == 0 || argc == 1, "argc should be 0 or 1", napi_invalid_arg);
    NAPI_ASSERT_BASE(env, self != nullptr, "self is nullptr", napi_invalid_arg);

    std::shared_ptr<ICompleteCallback> callback = nullptr;

    if (argc == 1) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OffComplete. argc == 1.");
        napi_valuetype valueType = napi_undefined;
        napi_typeof(env, argv[0], &valueType);
        NAPI_ASSERT_BASE(env, valueType == napi_function, "callback is not a function", napi_invalid_arg);
    }

    UploadTaskNapi *proxy = nullptr;
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);

    if (proxy->onComplete_ == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "CompleteCallback. proxy->onFail_ == nullptr.");
        return napi_generic_failure;
    } else {
        callback = std::make_shared<CompleteCallback>(proxy, env, argv[0]);
        proxy->napiUploadTask_->Off(TYPE_FAIL_CALLBACK, (void *)(callback.get()));
        proxy->onComplete_ = nullptr;
    }
    return napi_ok;
}

UploadTaskNapi &UploadTaskNapi::operator=(std::unique_ptr<Upload::UploadTask> &&uploadTask)
{
    if (napiUploadTask_ == uploadTask) {
        return *this;
    }
    napiUploadTask_ = std::move(uploadTask);
    return *this;
}

bool UploadTaskNapi::operator==(const std::unique_ptr<Upload::UploadTask> &uploadTask)
{
    return napiUploadTask_ == uploadTask;
}

void AddCallbackToConfig(std::shared_ptr<Upload::UploadConfig> &config, napi_env env, napi_value jsConfig,
    UploadTaskNapi *proxy)
{
    bool hasSuccess = false;
    bool hasFail = false;
    bool hasComplete = false;

    JSUtil::ParseFunction(env, jsConfig, "success", hasSuccess, proxy->success_);
    JSUtil::ParseFunction(env, jsConfig, "fail", hasFail, proxy->fail_);
    JSUtil::ParseFunction(env, jsConfig, "complete", hasComplete, proxy->complete_);

    if (hasSuccess || hasFail || hasComplete) {
        config->protocolVersion = "L5";
    }
    config->fsuccess = std::bind(&UploadTaskNapi::OnSystemSuccess, proxy->env_, proxy->success_,
        std::placeholders::_1);
    config->ffail = std::bind(&UploadTaskNapi::OnSystemFail, proxy->env_, proxy->fail_,
        std::placeholders::_1, std::placeholders::_2);
    config->fcomplete = std::bind(&UploadTaskNapi::OnSystemComplete, proxy->env_, proxy->complete_);
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
    int parametersPosition = 0;
    napi_value argv[JSUtil::MAX_ARGC] = {nullptr};
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, argv, &self, nullptr));
    auto *proxy = new UploadTaskNapi();
    proxy->env_ = env;
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;
    napi_status getStatus = GetContext(env, &argv[0], parametersPosition, context);
    if (getStatus != napi_ok) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Initialize. GetContext fail.");
        return nullptr;
    }
    proxy->napiUploadConfig_ = JSUtil::Convert2UploadConfig(env, argv[parametersPosition]);
    AddCallbackToConfig(proxy->napiUploadConfig_, env, argv[parametersPosition], proxy);
    proxy->napiUploadTask_ = std::make_unique<Upload::UploadTask>(proxy->napiUploadConfig_);
    proxy->napiUploadTask_->SetContext(context);
    proxy->napiUploadTask_->ExecuteTask();
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Initialize. GetAndSetContext[%{public}d]", getStatus);
    auto finalize = [](napi_env env, void * data, void * hint) {
        UploadTaskNapi *proxy = reinterpret_cast<UploadTaskNapi *>(data);
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "UploadTaskNapi. delete.");
        delete proxy;
    };
    UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "UploadTaskNapi. napi_wrap OK.");
    if (napi_wrap(env, self, proxy, finalize, nullptr, nullptr) != napi_ok) {
        finalize(env, proxy, nullptr);
        return nullptr;
    }
    return self;
}

napi_status UploadTaskNapi::GetContext(napi_env env, napi_value *argv, int& parametersPosition,
    std::shared_ptr<OHOS::AbilityRuntime::Context>& context)
{
    bool stageMode = false;
    napi_status status = OHOS::AbilityRuntime::IsStageContext(env, argv[0], stageMode);
    if (status != napi_ok) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "GetAndSetContext. L7");
        auto ability = OHOS::AbilityRuntime::GetCurrentAbility(env);
        if (ability == nullptr) {
            UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "GetAndSetContext. L7. GetCurrentAbility ability == nullptr.");
            return napi_generic_failure;
        }
        context = ability->GetAbilityContext();
    } else {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "GetAndSetContext. L8");
        parametersPosition = 1;
        if (stageMode) {
            context = OHOS::AbilityRuntime::GetStageModeContext(env, argv[0]);
            if (context == nullptr) {
                UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI,
                    "GetAndSetContext. L8. GetStageModeContext contextRtm == nullptr.");
                return napi_generic_failure;
            }
        } else {
            auto ability = OHOS::AbilityRuntime::GetCurrentAbility(env);
            if (ability == nullptr) {
                UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "GetAndSetContext. L8. GetCurrentAbility ability == nullptr.");
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

void UploadTaskNapi::OnSystemSuccess(napi_env env, napi_ref ref, Upload::UploadResponse &response)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OnSystemSuccess enter");
    uv_loop_s *loop_ = nullptr;
    napi_get_uv_event_loop(env, &loop_);
    if (loop_ == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Failed to get uv event loop");
        return;
    }
    uv_work_t *work = new (std::nothrow) uv_work_t;
    if (work == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Failed to create uv work");
        return;
    }
    SystemSuccessCallback *successCallback = new (std::nothrow)SystemSuccessCallback;
    if (successCallback == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Failed to create successCallback");
        return;
    }
    successCallback->env = env;
    successCallback->ref = ref;
    successCallback->response = response;
    work->data = (void *)successCallback;
    int ret = uv_queue_work(loop_, work,
        [](uv_work_t *work) {},
        [](uv_work_t *work, int status) {
            SystemSuccessCallback *successCallback = reinterpret_cast<SystemSuccessCallback *>(work->data);
            napi_value callback = nullptr;
            napi_value global = nullptr;
            napi_value result = nullptr;
            napi_value jsResponse = JSUtil::Convert2JSUploadResponse(successCallback->env, successCallback->response);
            napi_value args[1] = { jsResponse };
            napi_get_reference_value(successCallback->env, successCallback->ref, &callback);
            napi_get_global(successCallback->env, &global);
            napi_call_function(successCallback->env, global, callback, 1, args, &result);
            delete successCallback;
            successCallback = nullptr;
            delete work;
            work = nullptr;
        });
    if (ret != 0) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "OnSystemSuccess. uv_queue_work Failed");
        delete successCallback;
        delete work;
    }
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OnSystemSuccess end");
}

void UploadTaskNapi::OnSystemFail(napi_env env, napi_ref ref, std::string &data, int32_t &code)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OnSystemFail enter");
    uv_loop_s *loop_ = nullptr;
    napi_get_uv_event_loop(env, &loop_);
    if (loop_ == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Failed to get uv event loop");
        return;
    }
    uv_work_t *work = new (std::nothrow) uv_work_t;
    if (work == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Failed to create uv work");
        return;
    }
    SystemFailCallback *failCallback = new (std::nothrow) SystemFailCallback;
    failCallback->data = data;
    failCallback->code = code;
    failCallback->env = env;
    failCallback->ref = ref;
    work->data = (void *)failCallback;
    int ret = uv_queue_work(loop_, work, [](uv_work_t *work) {},
        [](uv_work_t *work, int status) {
            SystemFailCallback *failCallback = reinterpret_cast<SystemFailCallback *>(work->data);
            napi_value callback = nullptr;
            napi_value global = nullptr;
            napi_value result = nullptr;
            napi_value jsData = nullptr;
            napi_create_string_utf8(failCallback->env, failCallback->data.c_str(), failCallback->data.size(), &jsData);
            napi_value jsCode = nullptr;
            napi_create_int32(failCallback->env, failCallback->code, &jsCode);
            napi_value args[2] = { jsData, jsCode };
            napi_get_reference_value(failCallback->env, failCallback->ref, &callback);
            napi_get_global(failCallback->env, &global);
            napi_call_function(failCallback->env, global, callback, sizeof(args) / sizeof(args[0]), args, &result);
            delete failCallback;
            failCallback = nullptr;
            delete work;
            work = nullptr;
        });
    if (ret != 0) {
        delete failCallback;
        delete work;
    }
}

void UploadTaskNapi::OnSystemComplete(napi_env env, napi_ref ref)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OnSystemComplete enter");
    uv_loop_s *loop_ = nullptr;
    napi_get_uv_event_loop(env, &loop_);
    if (loop_ == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Failed to get uv event loop");
        return;
    }
    uv_work_t *work = new (std::nothrow) uv_work_t;
    if (work == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Failed to create uv work");
        return;
    }
    SystemCompleteCallback *completeCallback = new (std::nothrow)SystemCompleteCallback;
    completeCallback->env = env;
    completeCallback->ref = ref;
    work->data = (void *)completeCallback;
    int ret = uv_queue_work(loop_, work,
        [](uv_work_t *work) {},
        [](uv_work_t *work, int status) {
            SystemCompleteCallback *completeCallback = reinterpret_cast<SystemCompleteCallback *>(work->data);
            napi_value callback = nullptr;
            napi_value global = nullptr;
            napi_value result = nullptr;

            napi_get_reference_value(completeCallback->env, completeCallback->ref, &callback);
            napi_get_global(completeCallback->env, &global);
            napi_call_function(completeCallback->env, global, callback, 0, nullptr, &result);
            delete completeCallback;
            completeCallback = nullptr;
            delete work;
            work = nullptr;
        });
    if (ret != 0) {
        delete completeCallback;
        delete work;
    }
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OnSystemComplete end");
}

bool UploadTaskNapi::JudgeFail(const IFailCallback *target)
{
    if ((this->onFail_ != nullptr && this->onFail_.get() == target)) {
        return true;
    }
    return false;
}
bool UploadTaskNapi::JudgeComplete(const ICompleteCallback *target)
{
    if ((this->onComplete_ != nullptr && this->onComplete_.get() == target)) {
        return true;
    }
    return false;
}
bool UploadTaskNapi::JudgeProgress(const IProgressCallback *target)
{
    if ((this->onProgress_ != nullptr && this->onProgress_.get() == target) ||
       (this->offProgress_ != nullptr && this->offProgress_.get() == target)) {
        return true;
    }
    return false;
}
bool UploadTaskNapi::JudgeHeaderReceive(const IHeaderReceiveCallback *target)
{
    if ((this->onHeaderReceive_ != nullptr && this->onHeaderReceive_.get() == target) ||
       (this->offHeaderReceive_ != nullptr && this->offHeaderReceive_.get() == target)) {
        return true;
    }
    return false;
}
} // namespace OHOS::Request::UploadNapi