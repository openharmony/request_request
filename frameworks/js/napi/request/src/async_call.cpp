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

#include "async_call.h"

#include "log.h"

namespace OHOS::Request {
constexpr uint8_t MAX_ARGC = 10;
AsyncCall::AsyncCall(napi_env env, napi_callback_info info, const std::shared_ptr<Context> &context)
{
    if (context == nullptr) {
        return;
    }
    context->env_ = env;
    size_t argc = MAX_ARGC;
    napi_value argv[MAX_ARGC] = { nullptr };
    napi_value self = nullptr;
    napi_get_cb_info(env, info, &argc, argv, &self, nullptr);
    napi_valuetype valueType = napi_undefined;
    if (argc > 0) {
        napi_typeof(env, argv[argc - 1], &valueType);
        if (valueType == napi_function) {
            napi_create_reference(env, argv[argc - 1], 1, &context->callbackRef_);
            argc = argc - 1;
        }
    }

    if (context->input_ == nullptr) {
        REQUEST_HILOGD("ignored input handler");
        return;
    }
    napi_status status = context->input_(argc, argv, self);
    context->input_ = nullptr;
    if (status != napi_ok) {
        context->innerCode_ = E_PARAMETER_CHECK;
        context->exec_ = nullptr;
        context->output_ = nullptr;
        REQUEST_HILOGE("input_ status fail");
        return;
    }
    napi_create_reference(env, self, 1, &context->self_);
}

AsyncCall::~AsyncCall()
{
}

napi_value AsyncCall::Call(const std::shared_ptr<Context> &context, const std::string &resourceName)
{
    if (context == nullptr) {
        REQUEST_HILOGE("Context is null");
        return nullptr;
    }
    if (context->innerCode_ != E_OK) {
        REQUEST_HILOGE("Business execution failed");
        return nullptr;
    }
    napi_value ret = nullptr;
    if (context->callbackRef_ == nullptr) {
        napi_create_promise(context->env_, &context->defer_, &ret);
    } else {
        napi_get_undefined(context->env_, &ret);
    }
    napi_value resource = nullptr;
    std::string name = "REQUEST_" + resourceName;
    napi_create_string_utf8(context->env_, name.c_str(), NAPI_AUTO_LENGTH, &resource);
    WorkData *workData = new WorkData{ .ctx = context };
    workData->ctx = context;
    napi_create_async_work(
        context->env_, nullptr, resource, AsyncCall::OnExecute, AsyncCall::OnComplete, workData, &context->work_);
    napi_queue_async_work_with_qos(context->env_, context->work_, napiQosLevel_);
    REQUEST_HILOGD("async call exec");
    return ret;
}

void AsyncCall::OnExecute(napi_env env, void *data)
{
    WorkData *workData = reinterpret_cast<WorkData *>(data);
    if (workData->ctx != nullptr && workData->ctx->exec_ != nullptr) {
        workData->ctx->exec_();
        workData->ctx->exec_ = nullptr;
    }
}

void AsyncCall::OnComplete(napi_env env, napi_status status, void *data)
{
    REQUEST_HILOGD("AsyncCall OnComplete in");
    WorkData *workData = reinterpret_cast<WorkData *>(data);
    auto context = workData->ctx;
    if (context == nullptr || context->output_ == nullptr) {
        REQUEST_HILOGD("missing output handler");
        delete workData;
        return;
    }
    napi_value result[ARG_BUTT] = { nullptr };
    if (context->version_ == Version::API10) {
        napi_get_null(env, &result[ARG_ERROR]);
    } else {
        napi_get_undefined(env, &result[ARG_ERROR]);
    }
    napi_get_undefined(env, &result[ARG_DATA]);
    napi_status outputStatus = workData->ctx->output_(&result[ARG_DATA]);
    context->output_ = nullptr;
    if (status != napi_ok || outputStatus != napi_ok) {
        result[ARG_ERROR] = context->CreateErr();
    }
    if (context->defer_ != nullptr) {
        // promise
        if (status == napi_ok && outputStatus == napi_ok) {
            napi_resolve_deferred(env, context->defer_, result[ARG_DATA]);
        } else {
            napi_reject_deferred(env, context->defer_, result[ARG_ERROR]);
        }
    } else {
        // callback
        napi_value callback = nullptr;
        napi_get_reference_value(env, context->callbackRef_, &callback);
        napi_value returnValue;
        napi_call_function(env, nullptr, callback, ARG_BUTT, result, &returnValue);
        napi_delete_reference(env, context->callbackRef_);
        context->callbackRef_ = nullptr;
    }
    delete workData;
}
} // namespace OHOS::Request