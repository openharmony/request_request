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
};
std::map<std::string, UploadTaskNapi::Exec> UploadTaskNapi::offTypeHandlers_ = {
    {"progress", UploadTaskNapi::OffProgress},
    {"headerReceive", UploadTaskNapi::OffHeaderReceive},
    {"fail", UploadTaskNapi::OffFail},
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
    AsyncCall asyncCall(env, info, context, 1);
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
    AsyncCall asyncCall(env, info, std::dynamic_pointer_cast<AsyncCall::Context>(context), 0);
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

    std::shared_ptr<IProgressCallback> callback = std::make_shared<ProgressCallback>(env, argv[0]);
    proxy->napiUploadTask_->On(TYPE_PROGRESS_CALLBACK, (void *)(callback.get()));
    proxy->offProgress_ = nullptr;
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

    std::shared_ptr<IHeaderReceiveCallback> callback = std::make_shared<HeaderReceiveCallback>(env, argv[0]);
    proxy->napiUploadTask_->On(TYPE_HEADER_RECEIVE_CALLBACK, (void *)(callback.get()));
    proxy->offHeaderReceive_ = nullptr;
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

    std::shared_ptr<IFailCallback> callback = std::make_shared<FailCallback>(env, argv[0]);
    proxy->napiUploadTask_->On(TYPE_FAIL_CALLBACK, (void *)(callback.get()));
    proxy->offFail_ = nullptr;
    proxy->onFail_ = std::move(callback);
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
        callback = std::make_shared<ProgressCallback>(env, argv[0]);
    }

    UploadTaskNapi *proxy = nullptr;
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);

    if (proxy->onProgress_ == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Progress. proxy->onProgress_ == nullptr.");
        return napi_generic_failure;
    } else {
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
        callback = std::make_shared<HeaderReceiveCallback>(env, argv[0]);
    }

    UploadTaskNapi *proxy = nullptr;
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);

    if (proxy->onHeaderReceive_ == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "HeaderReceive. proxy->onHeaderReceive_ == nullptr.");
        return napi_generic_failure;
    } else {
        proxy->napiUploadTask_->Off(TYPE_HEADER_RECEIVE_CALLBACK, (void *)(callback.get()));
        proxy->onHeaderReceive_ = nullptr;
        proxy->offHeaderReceive_ = std::move(callback);
    }
    return napi_ok;
}

napi_status UploadTaskNapi::OffFail(napi_env env, size_t argc, napi_value *argv, napi_value self, napi_value *result)
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
        callback = std::make_shared<FailCallback>(env, argv[0]);
    }

    UploadTaskNapi *proxy = nullptr;
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);

    if (proxy->onFail_ == nullptr) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Fail. proxy->onFail_ == nullptr.");
        return napi_generic_failure;
    } else {
        proxy->napiUploadTask_->Off(TYPE_FAIL_CALLBACK, (void *)(callback.get()));
        proxy->onFail_ = nullptr;
        proxy->offFail_ = std::move(callback);
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
    UploadTaskNapi * proxy)
{
    bool hasSuccess, hasFail, hasComplete;
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
    napi_value argv[JSUtil::MAX_ARGC] = {nullptr};
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, argv, &self, nullptr));
    auto *proxy = new UploadTaskNapi();
    proxy->env_ = env;
    proxy->napiUploadConfig_ = JSUtil::Convert2UploadConfig(env, argv[0]);
    AddCallbackToConfig(proxy->napiUploadConfig_, env, argv[0], proxy);
    proxy->napiUploadTask_ = std::make_unique<Upload::UploadTask>(proxy->napiUploadConfig_);
    napi_status getStatus = GetAndSetContext(env, &argv[0], proxy);
    proxy->napiUploadTask_->ExecuteTask();
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "Initialize. GetAndSetContext[%{public}d]", getStatus);
    auto finalize = [](napi_env env, void * data, void * hint) {
        UploadTaskNapi *proxy = reinterpret_cast<UploadTaskNapi *>(data);
        delete proxy;
    };
    if (napi_wrap(env, self, proxy, finalize, nullptr, nullptr) != napi_ok) {
        finalize(env, proxy, nullptr);
        return nullptr;
    }
    return self;
}

napi_status UploadTaskNapi::GetAndSetContext(napi_env env, napi_value *argv, UploadTaskNapi *proxy)
{
    bool stageMode = false;
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;

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
    proxy->napiUploadTask_->SetContext(context);
    return napi_ok;
}

void UploadTaskNapi::OnSystemSuccess(napi_env env, napi_ref ref, Upload::UploadResponse &response)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OnSystemSuccess enter");
    napi_value callback = nullptr;
    napi_value global = nullptr;
    napi_value result = nullptr;

    napi_value jsResponse = JSUtil::Convert2JSUploadResponse(env, response);
    napi_value args[1] = { jsResponse };

    napi_get_reference_value(env, ref, &callback);
    napi_get_global(env, &global);

    napi_call_function(env, global, callback, 1, args, &result);
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OnSystemSuccess end");
}

void UploadTaskNapi::OnSystemFail(napi_env env, napi_ref ref, std::string &data, int32_t &code)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OnSystemFail enter");
    napi_value callback = nullptr;
    napi_value global = nullptr;
    napi_value result = nullptr;

    napi_value jsData = nullptr;
    napi_create_string_utf8(env, data.c_str(), data.size(), &jsData);

    napi_value jsCode = nullptr;
    napi_create_int32(env, code, &jsCode);

    napi_value args[2] = { jsData, jsCode };

    napi_get_reference_value(env, ref, &callback);
    napi_get_global(env, &global);

    napi_call_function(env, global, callback, sizeof(args) / sizeof(args[0]), args, &result);

    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OnSystemFail end");
}

void UploadTaskNapi::OnSystemComplete(napi_env env, napi_ref ref)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OnSystemComplete enter");
    napi_value callback = nullptr;
    napi_value global = nullptr;
    napi_value result = nullptr;

    napi_get_reference_value(env, ref, &callback);
    napi_get_global(env, &global);

    napi_call_function(env, global, callback, 0, nullptr, &result);
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OnSystemComplete end");
}
} // namespace OHOS::Request::UploadNapi