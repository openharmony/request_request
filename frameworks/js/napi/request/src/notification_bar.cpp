/*
 * Copyright (C) 2023 Huawei Device Co., Ltd.
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

#include "notification_bar.h"

#include <memory>
#include <string>

#include "async_call.h"
#include "napi/native_node_api.h"
#include "napi_utils.h"
#include "request_manager.h"

namespace OHOS::Request {

const std::string PARAMETER_ERROR_INFO = "wrong parameters";

struct CreateContext : public AsyncCall::Context {
    std::string gid;
    bool gauge = false;
    bool customized = false;
    std::string title;
    std::string text;
};

napi_status createInput(CreateContext *context, size_t argc, napi_value *argv, napi_value self)
{
    if (argc < 1) {
        NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, PARAMETER_ERROR_INFO, true);
        return napi_invalid_arg;
    }
    if (NapiUtils::GetValueType(context->env_, argv[0]) != napi_valuetype::napi_object) {
        NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, PARAMETER_ERROR_INFO, true);
        return napi_invalid_arg;
    }
    if (NapiUtils::HasNamedProperty(context->env_, argv[0], "gauge")) {
        napi_value gauge = NapiUtils::GetNamedProperty(context->env_, argv[0], "gauge");
        if (NapiUtils::GetValueType(context->env_, gauge) == napi_boolean) {
            bool value = false;
            napi_get_value_bool(context->env_, gauge, &value);
            context->gauge = value;
        } else {
            NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, PARAMETER_ERROR_INFO, true);
            return napi_invalid_arg;
        }
    }
    if (!NapiUtils::HasNamedProperty(context->env_, argv[0], "notification")) {
        return napi_ok;
    }
    napi_value customized_notification = NapiUtils::GetNamedProperty(context->env_, argv[0], "notification");
    if (NapiUtils::GetValueType(context->env_, customized_notification) != napi_object) {
        return napi_ok;
    }
    if (NapiUtils::HasNamedProperty(context->env_, customized_notification, "title")) {
        context->customized = true;
        napi_value title = NapiUtils::GetNamedProperty(context->env_, customized_notification, "title");
        if (NapiUtils::GetValueType(context->env_, title) == napi_string) {
            context->title = NapiUtils::Convert2String(context->env_, title);
        } else {
            NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, PARAMETER_ERROR_INFO, true);
            return napi_invalid_arg;
        }
    }
    if (NapiUtils::HasNamedProperty(context->env_, customized_notification, "text")) {
        context->customized = true;
        napi_value text = NapiUtils::GetNamedProperty(context->env_, customized_notification, "text");
        if (NapiUtils::GetValueType(context->env_, text) == napi_string) {
            context->text = NapiUtils::Convert2String(context->env_, text);
        } else {
            NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, PARAMETER_ERROR_INFO, true);
            return napi_invalid_arg;
        }
    }
    return napi_ok;
}

napi_value createGroup(napi_env env, napi_callback_info info)
{
    auto context = std::make_shared<CreateContext>();
    auto input = [context](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        return createInput(context.get(), argc, argv, self);
    };
    auto output = [context](napi_value *result) -> napi_status {
        napi_create_string_utf8(context->env_, context->gid.c_str(), context->gid.length(), result);
        return napi_ok;
    };
    auto exec = [context]() {
        RequestManager::GetInstance()->CreateGroup(
            context->gid, context->gauge, context->customized, context->title, context->text);
    };
    context->SetInput(input).SetOutput(output).SetExec(exec);
    AsyncCall asyncCall(env, info, context);
    return asyncCall.Call(context, "createGroup");
}

struct AttachContext : public AsyncCall::Context {
    std::string gid;
    std::string tid;
};

napi_value attachGroup(napi_env env, napi_callback_info info)
{
    auto context = std::make_shared<AttachContext>();
    auto input = [context](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        if (argc != 2) {
            NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, PARAMETER_ERROR_INFO, true);
            return napi_invalid_arg;
        }
        if (NapiUtils::GetValueType(context->env_, argv[0]) != napi_string
            || NapiUtils::GetValueType(context->env_, argv[1]) != napi_string) {
            NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, PARAMETER_ERROR_INFO, true);
            return napi_invalid_arg;
        }
        context->gid = NapiUtils::Convert2String(context->env_, argv[0]);
        context->tid = NapiUtils::Convert2String(context->env_, argv[1]);
        return napi_ok;
    };
    auto output = [context](napi_value *result) -> napi_status {
        if (context->innerCode_ != E_OK) {
            return napi_generic_failure;
        }
        return napi_ok;
    };
    auto exec = [context]() {
        context->innerCode_ = RequestManager::GetInstance()->AttachGroup(context->gid, context->tid);
    };
    context->SetInput(input).SetOutput(output).SetExec(exec);
    AsyncCall asyncCall(env, info, context);
    return asyncCall.Call(context, "createGroup");
}

struct DeleteContext : public AsyncCall::Context {
    std::string gid;
};

napi_value deleteGroup(napi_env env, napi_callback_info info)
{
    auto context = std::make_shared<DeleteContext>();
    auto input = [context](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        if (argc != 1) {
            NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, PARAMETER_ERROR_INFO, true);
            return napi_invalid_arg;
        }
        if (NapiUtils::GetValueType(context->env_, argv[0]) != napi_string) {
            NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, PARAMETER_ERROR_INFO, true);
            return napi_invalid_arg;
        }
        context->gid = NapiUtils::Convert2String(context->env_, argv[0]);
        return napi_ok;
    };
    auto output = [context](napi_value *result) -> napi_status {
        if (context->innerCode_ != E_OK) {
            return napi_generic_failure;
        }
        return napi_ok;
    };
    auto exec = [context]() { context->innerCode_ = RequestManager::GetInstance()->DeleteGroup(context->gid); };
    context->SetInput(input).SetOutput(output).SetExec(exec);
    AsyncCall asyncCall(env, info, context);
    return asyncCall.Call(context, "createGroup");
}

} // namespace OHOS::Request