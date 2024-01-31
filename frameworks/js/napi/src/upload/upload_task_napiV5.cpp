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

#include <uv.h>

#include "ability.h"
#include "js_initialize.h"
#include "napi_base_context.h"
#include "upload/js_util.h"
#include "uv_queue.h"

namespace OHOS::Request::Upload {
constexpr int FIRST_ARGV = 0;
constexpr int PARAM_COUNT_ZERO = 0;
constexpr int PARAM_COUNT_ONE = 1;
constexpr int PARAM_COUNT_TWO = 2;
UploadTaskNapiV5::~UploadTaskNapiV5()
{
    if (env_ == nullptr) {
        return;
    }

    RecycleRef *callbackData = new (std::nothrow)
        RecycleRef{ .env = env_, .successRef = success_, .failRef = fail_, .completeRef = complete_ };
    if (callbackData == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Failed to create callbackData");
        return;
    }
    uv_after_work_cb afterCallback = [](uv_work_t *work, int status) {
        RecycleRef *callbackDataPtr = reinterpret_cast<RecycleRef *>(work->data);
        if (callbackDataPtr != nullptr) {
            UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "~UploadTaskNapiV5 callbackDataPtr delete start");
            napi_delete_reference(callbackDataPtr->env, callbackDataPtr->successRef);
            napi_delete_reference(callbackDataPtr->env, callbackDataPtr->failRef);
            napi_delete_reference(callbackDataPtr->env, callbackDataPtr->completeRef);
            delete callbackDataPtr;
        }
        delete work;
    };
    UvQueue::Call(env_, reinterpret_cast<void *>(callbackData), afterCallback);
}

bool UploadTaskNapiV5::ParseCallback(napi_env env, napi_callback_info info)
{
    napi_value self = nullptr;
    size_t argc = JSUtil::MAX_ARGC;
    napi_value argv[JSUtil::MAX_ARGC] = { nullptr };
    NAPI_CALL_BASE(env, napi_get_cb_info(env, info, &argc, argv, &self, nullptr), false);
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
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, argv, &self, nullptr));

    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;
    napi_status getStatus = JsInitialize::GetContext(env, argv[FIRST_ARGV], context);
    if (getStatus != napi_ok) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "GetContext fail.");
        NAPI_ASSERT(env, false, "GetContext fail");
    }

    std::shared_ptr<UploadConfig> uploadConfig = JSUtil::ParseUploadConfig(env, argv[FIRST_ARGV], API3);
    if (uploadConfig == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "ParseUploadConfig fail.");
        NAPI_ASSERT(env, false, "ParseUploadConfig fail");
    }

    AddCallbackToConfig(env, uploadConfig);
    uploadTask_ = std::make_shared<Upload::UploadTask>(uploadConfig);
    uploadTask_->SetContext(context);
    uploadTask_->SetUploadProxy(shared_from_this());
    uploadTask_->ExecuteTask();
    uploadTask_ = nullptr;
    return nullptr;
}

void UploadTaskNapiV5::OnSystemSuccess(napi_env env, napi_ref ref, Upload::UploadResponse &response)
{
    UPLOAD_HILOGI(UPLOAD_MODULE_JS_NAPI, "OnSystemSuccess enter");

    SystemSuccessCallback *successCallback = new (std::nothrow)
        SystemSuccessCallback{ .env = env, .ref = ref, .response = response };
    if (successCallback == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Failed to create successCallback");
        return;
    }
    uv_after_work_cb afterCallback = [](uv_work_t *work, int status) {
        SystemSuccessCallback *successCallback = reinterpret_cast<SystemSuccessCallback *>(work->data);
        napi_handle_scope scope = nullptr;
        napi_open_handle_scope(successCallback->env, &scope);
        napi_value callback = nullptr;
        napi_value global = nullptr;
        napi_value result = nullptr;
        napi_value jsResponse = JSUtil::Convert2JSUploadResponse(successCallback->env, successCallback->response);
        napi_value args[PARAM_COUNT_ONE] = { jsResponse };
        napi_get_reference_value(successCallback->env, successCallback->ref, &callback);
        napi_get_global(successCallback->env, &global);
        napi_call_function(successCallback->env, global, callback, PARAM_COUNT_ONE, args, &result);
        napi_close_handle_scope(successCallback->env, scope);
        delete successCallback;
        delete work;
    };
    UvQueue::Call(env, reinterpret_cast<void *>(successCallback), afterCallback);
}

void UploadTaskNapiV5::OnSystemFail(napi_env env, napi_ref ref, std::string &data, int32_t &code)
{
    UPLOAD_HILOGI(UPLOAD_MODULE_JS_NAPI, "OnSystemFail enter");
    SystemFailCallback *failCallback = new (std::nothrow)
        SystemFailCallback{ .data = data, .code = code, .env = env, .ref = ref };
    if (failCallback == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Failed to create failCallback");
        return;
    }
    uv_after_work_cb afterCallback = [](uv_work_t *work, int status) {
        SystemFailCallback *failCallback = reinterpret_cast<SystemFailCallback *>(work->data);
        napi_handle_scope scope = nullptr;
        napi_open_handle_scope(failCallback->env, &scope);
        napi_value callback = nullptr;
        napi_value global = nullptr;
        napi_value result = nullptr;
        napi_value jsData = nullptr;
        napi_create_string_utf8(failCallback->env, failCallback->data.c_str(), failCallback->data.size(), &jsData);
        napi_value jsCode = nullptr;
        napi_create_int32(failCallback->env, failCallback->code, &jsCode);
        napi_value args[PARAM_COUNT_TWO] = { jsData, jsCode };
        napi_get_reference_value(failCallback->env, failCallback->ref, &callback);
        napi_get_global(failCallback->env, &global);
        napi_call_function(failCallback->env, global, callback, PARAM_COUNT_TWO, args, &result);
        napi_close_handle_scope(failCallback->env, scope);
        delete failCallback;
        delete work;
    };
    UvQueue::Call(env, reinterpret_cast<void *>(failCallback), afterCallback);
}

void UploadTaskNapiV5::OnSystemComplete(std::shared_ptr<Upload::UploadTaskNapiV5> proxy)
{
    UPLOAD_HILOGI(UPLOAD_MODULE_JS_NAPI, "OnSystemComplete enter");
    SystemCompleteCallback *completeCallback = new (std::nothrow) SystemCompleteCallback{ .proxy = proxy };
    if (completeCallback == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Failed to create completeCallback");
        return;
    }
    uv_after_work_cb afterCallback = [](uv_work_t *work, int status) {
        SystemCompleteCallback *completeCallback = reinterpret_cast<SystemCompleteCallback *>(work->data);
        napi_handle_scope scope = nullptr;
        napi_open_handle_scope(completeCallback->proxy->env_, &scope);
        napi_value callback = nullptr;
        napi_value global = nullptr;
        napi_value result = nullptr;

        napi_status ret =
            napi_get_reference_value(completeCallback->proxy->env_, completeCallback->proxy->complete_, &callback);
        if (ret == napi_ok) {
            napi_get_global(completeCallback->proxy->env_, &global);
            napi_call_function(completeCallback->proxy->env_, global, callback, PARAM_COUNT_ZERO, nullptr, &result);
        }
        UPLOAD_HILOGD(
            UPLOAD_MODULE_JS_NAPI, "OnSystemComplete NapiV5Proxy: %{public}ld", completeCallback->proxy.use_count());
        napi_close_handle_scope(completeCallback->proxy->env_, scope);
        delete completeCallback;
        delete work;
    };
    UvQueue::Call(proxy->env_, reinterpret_cast<void *>(completeCallback), afterCallback);
}
} // namespace OHOS::Request::Upload