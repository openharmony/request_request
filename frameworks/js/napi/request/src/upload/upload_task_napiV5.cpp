/*
 * Copyright (c) 2023 Huawei Device Co., Ltd.
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

#include "upload/upload_task_napiV5.h"

#include "ability.h"
#include "js_initialize.h"
#include "napi_base_context.h"
#include "upload/js_util.h"

namespace OHOS::Request::Upload {
constexpr int FIRST_ARGV = 0;
constexpr int PARAM_COUNT_ZERO = 0;
constexpr int PARAM_COUNT_ONE = 1;
constexpr int PARAM_COUNT_TWO = 2;

bool UploadTaskNapiV5::CreateNapiScope(napi_env env, napi_handle_scope &scope)
{
    napi_status status = napi_open_handle_scope(env, &scope);
    if (status != napi_ok || scope == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Create napi scope failed, status: %d", status);
        return false;
    }
    return true;
}

void UploadTaskNapiV5::CloseNapiScope(napi_env env, napi_handle_scope &scope)
{
    if (scope != nullptr) {
        napi_close_handle_scope(env, scope);
        scope = nullptr;
    }
}

UploadTaskNapiV5::~UploadTaskNapiV5()
{
    if (env_ == nullptr) {
        return;
    }
    auto callbackData = std::make_shared<RecycleRef>(
        RecycleRef{ .env = env_, .successRef = success_, .failRef = fail_, .completeRef = complete_ });
    auto afterCallback = [callbackData]() {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "~UploadTaskNapiV5 callbackDataPtr delete start");
        napi_delete_reference(callbackData->env, callbackData->successRef);
        napi_delete_reference(callbackData->env, callbackData->failRef);
        napi_delete_reference(callbackData->env, callbackData->completeRef);
    };
    int32_t ret = napi_send_event(env_, afterCallback, napi_eprio_high, "request:upload");
    if (ret != napi_ok) {
        REQUEST_HILOGE("napi_send_event failed: %{public}d", ret);
    }
}

bool UploadTaskNapiV5::ParseCallback(napi_env env, napi_callback_info info)
{
    napi_value self = nullptr;
    size_t argc = JSUtil::MAX_ARGC;
    napi_value argv[JSUtil::MAX_ARGC] = { nullptr };
    REQUEST_NAPI_CALL_RETURN(env, napi_get_cb_info(env, info, &argc, argv, &self, nullptr), "napi_get_cb_info", false);
    bool successCb = JSUtil::ParseFunction(env, argv[FIRST_ARGV], "success", success_);
    bool failCb = JSUtil::ParseFunction(env, argv[FIRST_ARGV], "fail", fail_);
    bool completeCb = JSUtil::ParseFunction(env, argv[FIRST_ARGV], "complete", complete_);
    return successCb || failCb || completeCb;
}

void UploadTaskNapiV5::AddCallbackToConfig(napi_env env, std::shared_ptr<UploadConfig> &config)
{
    config->fsuccess = std::bind(&UploadTaskNapiV5::OnSystemSuccess, env_, success_, std::placeholders::_1);
    config->ffail =
        std::bind(&UploadTaskNapiV5::OnSystemFail, env_, fail_, std::placeholders::_1, std::placeholders::_2);
    config->fcomplete = std::bind(&UploadTaskNapiV5::OnSystemComplete, shared_from_this());
}

napi_value UploadTaskNapiV5::JsUpload(napi_env env, napi_callback_info info)
{
    UPLOAD_HILOGI(UPLOAD_MODULE_JS_NAPI, "Enter JsUploadV5.");
    napi_value self = nullptr;
    size_t argc = JSUtil::MAX_ARGC;
    napi_value argv[JSUtil::MAX_ARGC] = { nullptr };
    REQUEST_NAPI_CALL(env, napi_get_cb_info(env, info, &argc, argv, &self, nullptr), "napi_get_cb_info failed");

    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;
    napi_status getStatus = JsInitialize::GetContext(env, argv[FIRST_ARGV], context);
    if (getStatus != napi_ok) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "GetContext fail.");
        REQUEST_NAPI_ASSERT(env, "GetContext fail", false);
    }

    std::shared_ptr<UploadConfig> uploadConfig = JSUtil::ParseUploadConfig(env, argv[FIRST_ARGV], API3);
    if (uploadConfig == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "ParseUploadConfig fail.");
        REQUEST_NAPI_ASSERT(env, "ParseUploadConfig fail", false);
    }

    AddCallbackToConfig(env, uploadConfig);
    uploadTask_ = std::make_shared<Upload::UploadTask>(uploadConfig);
    uploadTask_->SetContext(context);
    uploadTask_->SetUploadProxy(shared_from_this());
    uploadTask_->ExecuteTask();
    uploadTask_ = nullptr;
    return nullptr;
}

bool UploadTaskNapiV5::CallNoParamCallback(napi_env env, napi_ref ref)
{
    return CallCallbackWithParam(env, ref, PARAM_COUNT_ZERO, nullptr);
}

bool UploadTaskNapiV5::CallSingleParamCallback(napi_env env, napi_ref ref, napi_value param)
{
    if (param == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Callback param is null");
        return false;
    }
    napi_value args[PARAM_COUNT_ONE] = { param };
    return CallCallbackWithParam(env, ref, PARAM_COUNT_ONE, args);
}

bool UploadTaskNapiV5::CallDoubleParamCallback(napi_env env, napi_ref ref, napi_value param1, napi_value param2)
{
    if (param1 == nullptr || param2 == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Callback param is null");
        return false;
    }
    napi_value args[PARAM_COUNT_TWO] = { param1, param2 };
    return CallCallbackWithParam(env, ref, PARAM_COUNT_TWO, args);
}

bool UploadTaskNapiV5::CallCallbackWithParam(napi_env env, napi_ref ref, size_t paramCount, napi_value *params)
{
    napi_value callback = nullptr;
    napi_status ret = napi_get_reference_value(env, ref, &callback);
    if (ret != napi_ok) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Get ref value failed, status: %d", ret);
        return false;
    }
    napi_value global = nullptr;
    ret = napi_get_global(env, &global);
    if (ret != napi_ok) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Get global failed, status: %d", ret);
        return false;
    }
    napi_value result = nullptr;
    ret = napi_call_function(env, global, callback, paramCount, params, &result);
    if (ret != napi_ok) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Call callback failed, status: %d", ret);
        return false;
    }
    return true;
}

bool UploadTaskNapiV5::CreateFailJsParams(
    napi_env env, const std::string &data, int32_t code, napi_value &jsData, napi_value &jsCode)
{
    napi_status ret = napi_create_string_utf8(env, data.c_str(), data.size(), &jsData);
    if (ret != napi_ok) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Create js data failed, status: %d", ret);
        return false;
    }
    ret = napi_create_int32(env, code, &jsCode);
    if (ret != napi_ok) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Create js code failed, status: %d", ret);
        return false;
    }
    return true;
}

void UploadTaskNapiV5::OnSystemSuccess(napi_env env, napi_ref ref, Upload::UploadResponse &response)
{
    UPLOAD_HILOGI(UPLOAD_MODULE_JS_NAPI, "OnSystemSuccess enter");
    if (env == nullptr || ref == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Env or ref is null");
        return;
    }
    auto callback =
        std::make_shared<SystemSuccessCallback>(SystemSuccessCallback{ .env = env, .ref = ref, .response = response });
    auto afterCallback = [callback]() {
        napi_handle_scope scope = nullptr;
        if (!CreateNapiScope(callback->env, scope)) {
            return;
        }
        napi_value jsResponse = JSUtil::Convert2JSUploadResponse(callback->env, callback->response);
        CallSingleParamCallback(callback->env, callback->ref, jsResponse);
        CloseNapiScope(callback->env, scope);
    };
    int32_t ret = napi_send_event(env, afterCallback, napi_eprio_high, "request:upload");
    if (ret != napi_ok) {
        REQUEST_HILOGE("napi_send_event failed: %{public}d", ret);
    }
}

void UploadTaskNapiV5::OnSystemFail(napi_env env, napi_ref ref, std::string &data, int32_t &code)
{
    UPLOAD_HILOGI(UPLOAD_MODULE_JS_NAPI, "OnSystemFail enter");
    if (env == nullptr || ref == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Env or ref is null");
        return;
    }
    auto callback =
        std::make_shared<SystemFailCallback>(SystemFailCallback{ .data = data, .code = code, .env = env, .ref = ref });
    auto afterCallback = [callback]() {
        napi_handle_scope scope = nullptr;
        if (!CreateNapiScope(callback->env, scope)) {
            return;
        }
        napi_value jsData = nullptr;
        napi_value jsCode = nullptr;
        if (CreateFailJsParams(callback->env, callback->data, callback->code, jsData, jsCode)) {
            CallDoubleParamCallback(callback->env, callback->ref, jsData, jsCode);
        }
        CloseNapiScope(callback->env, scope);
    };
    int32_t ret = napi_send_event(env, afterCallback, napi_eprio_high, "request:upload");
    if (ret != napi_ok) {
        REQUEST_HILOGE("napi_send_event failed: %{public}d", ret);
    }
}

void UploadTaskNapiV5::OnSystemComplete(std::shared_ptr<Upload::UploadTaskNapiV5> proxy)
{
    UPLOAD_HILOGI(UPLOAD_MODULE_JS_NAPI, "OnSystemComplete enter");
    if (proxy == nullptr || proxy->env_ == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Proxy or env is null");
        return;
    }
    auto callback = std::make_shared<SystemCompleteCallback>(SystemCompleteCallback{ .proxy = proxy });
    auto afterCallback = [callback]() {
        napi_handle_scope scope = nullptr;
        if (!CreateNapiScope(callback->proxy->env_, scope)) {
            return;
        }
        if (callback->proxy->complete_ != nullptr) {
            CallNoParamCallback(callback->proxy->env_, callback->proxy->complete_);
        }
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "OnSystemComplete proxy use count: %ld", callback->proxy.use_count());
        CloseNapiScope(callback->proxy->env_, scope);
    };
    int32_t ret = napi_send_event(proxy->env_, afterCallback, napi_eprio_high, "request:upload");
    if (ret != napi_ok) {
        REQUEST_HILOGE("napi_send_event failed: %{public}d", ret);
    }
}
} // namespace OHOS::Request::Upload