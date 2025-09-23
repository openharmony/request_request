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
#include <optional>
#include <string>
#include <vector>

#include "async_call.h"
#include "js_native_api_types.h"
#include "log.h"
#include "napi/native_node_api.h"
#include "napi_utils.h"
#include "request_manager.h"

#include "want_agent_helper.h"
#include "want_agent.h"

namespace OHOS::Request {

const std::string PARAMETER_ERROR_INFO = "wrong parameters";
const std::size_t MAX_TITLE_LENGTH = 1024;
const std::size_t MAX_TEXT_LENGTH = 3072;

struct CreateContext : public AsyncCall::Context {
    std::string gid;
    bool gauge = false;
    Notification notification;
};

napi_status ValidateAndSetTitle(CreateContext *context, napi_value customized)
{
    if (NapiUtils::HasNamedProperty(context->env_, customized, "title")) {
        napi_value title = NapiUtils::GetNamedProperty(context->env_, customized, "title");
        if (NapiUtils::GetValueType(context->env_, title) == napi_string) {
            context->notification.title = NapiUtils::Convert2String(context->env_, title);
            if (context->notification.title->size() > MAX_TITLE_LENGTH) {
                NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, PARAMETER_ERROR_INFO, true);
                return napi_invalid_arg;
            }
        }
    }
    return napi_ok;
}

napi_status ValidateAndSetText(CreateContext *context, napi_value customized)
{
    if (NapiUtils::HasNamedProperty(context->env_, customized, "text")) {
        napi_value text = NapiUtils::GetNamedProperty(context->env_, customized, "text");
        if (NapiUtils::GetValueType(context->env_, text) == napi_string) {
            context->notification.text = NapiUtils::Convert2String(context->env_, text);
            if (context->notification.text->size() > MAX_TEXT_LENGTH) {
                NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, PARAMETER_ERROR_INFO, true);
                return napi_invalid_arg;
            }
        }
    }
    return napi_ok;
}

napi_status ValidateAndSetDisable(CreateContext *context, napi_value customized)
{
    if (NapiUtils::HasNamedProperty(context->env_, customized, "disable")) {
        napi_value disable = NapiUtils::GetNamedProperty(context->env_, customized, "disable");
        if (NapiUtils::GetValueType(context->env_, disable) == napi_boolean) {
            bool value = false;
            napi_get_value_bool(context->env_, disable, &value);
            context->notification.disable = value;
        }
    }
    return napi_ok;
}

napi_status ValidateAndSetVisibility(CreateContext *context, napi_value customized)
{
    if (NapiUtils::HasNamedProperty(context->env_, customized, "visibility")) {
        napi_value visibility = NapiUtils::GetNamedProperty(context->env_, customized, "visibility");
        if (NapiUtils::GetValueType(context->env_, visibility) == napi_number) {
            context->notification.visibility = NapiUtils::Convert2Uint32(context->env_, visibility);
            if (context->notification.visibility == static_cast<uint32_t>(Visibility::NONE) ||
                (context->notification.visibility & static_cast<uint32_t>(Visibility::ANY)) !=
                context->notification.visibility) {
                NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, PARAMETER_ERROR_INFO, true);
                return napi_invalid_arg;
            }
        } else if (NapiUtils::GetValueType(context->env_, visibility) == napi_undefined) {
            if (!context->gauge) {
                context->notification.visibility = VISIBILITY_COMPLETION;
            }
        } else {
            NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, PARAMETER_ERROR_INFO, true);
            return napi_invalid_arg;
        }
    }
    return napi_ok;
}

napi_status ValidateAndSetWantAgent(CreateContext *context, napi_value customized)
{
    OHOS::AbilityRuntime::WantAgent::WantAgent *wantAgent = nullptr;
    napi_value wantValue = nullptr;
    if (NapiUtils::GetValueType(context->env_, NapiUtils::GetNamedProperty(context->env_, customized, "wantAgent"))
            != napi_undefined) {
        napi_get_named_property(context->env_, customized, "wantAgent", &wantValue);
        napi_status status = napi_unwrap(context->env_, wantValue, (void **)&wantAgent);
        if (status == napi_ok && wantAgent != nullptr) {
            std::shared_ptr<OHOS::AbilityRuntime::WantAgent::WantAgent> sWantAgent =
                std::make_shared<OHOS::AbilityRuntime::WantAgent::WantAgent>(*wantAgent);
            context->notification.wantAgent = OHOS::AbilityRuntime::WantAgent::WantAgentHelper::ToString(sWantAgent);
        } else {
            NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, PARAMETER_ERROR_INFO, true);
            return napi_invalid_arg;
        }
    }
    return napi_ok;
}

napi_status createNotificationParse(CreateContext *context, napi_value customized)
{
    if (context == nullptr || context->env_ == nullptr) {
        return napi_invalid_arg;
    }
    
    if (NapiUtils::GetValueType(context->env_, customized) != napi_object) {
        return napi_ok;
    }
    
    napi_status status = ValidateAndSetTitle(context, customized);
    if (status != napi_ok) {
        return status;
    }
    
    status = ValidateAndSetText(context, customized);
    if (status != napi_ok) {
        return status;
    }
    
    status = ValidateAndSetWantAgent(context, customized);
    if (status != napi_ok) {
        return status;
    }
    
    status = ValidateAndSetDisable(context, customized);
    if (status != napi_ok) {
        return status;
    }
    
    status = ValidateAndSetVisibility(context, customized);
    if (status != napi_ok) {
        return status;
    }
    
    return napi_ok;
}

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
    context->gauge = false;
    if (NapiUtils::HasNamedProperty(context->env_, argv[0], "gauge")) {
        napi_value gauge = NapiUtils::GetNamedProperty(context->env_, argv[0], "gauge");
        if (NapiUtils::GetValueType(context->env_, gauge) == napi_boolean) {
            bool value = false;
            napi_get_value_bool(context->env_, gauge, &value);
            context->gauge = value;
            if (context->gauge) {
                context->notification.visibility = VISIBILITY_COMPLETION | VISIBILITY_PROGRESS;
            } else {
                context->notification.visibility = VISIBILITY_COMPLETION;
            }
        } else if (NapiUtils::GetValueType(context->env_, gauge) == napi_undefined) {
            context->notification.visibility = VISIBILITY_COMPLETION;
        } else {
            NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, PARAMETER_ERROR_INFO, true);
            return napi_invalid_arg;
        }
    }
    if (!NapiUtils::HasNamedProperty(context->env_, argv[0], "notification")) {
        return napi_ok;
    }
    napi_value customized_notification = NapiUtils::GetNamedProperty(context->env_, argv[0], "notification");
    return createNotificationParse(context, customized_notification);
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
        RequestManager::GetInstance()->CreateGroup(context->gid, context->gauge, context->notification);
    };
    context->SetInput(input).SetOutput(output).SetExec(exec);
    AsyncCall asyncCall(env, info, context);
    return asyncCall.Call(context, "createGroup");
}

struct AttachContext : public AsyncCall::Context {
    std::string gid;
    std::vector<std::string> tids;
};

napi_value attachGroup(napi_env env, napi_callback_info info)
{
    auto context = std::make_shared<AttachContext>();
    context->withErrCode_ = true;
    auto input = [context](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        if (argc != 2) {
            NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, PARAMETER_ERROR_INFO, true);
            return napi_invalid_arg;
        }
        if (NapiUtils::GetValueType(context->env_, argv[0]) != napi_string
            || NapiUtils::GetValueType(context->env_, argv[1]) != napi_object) {
            NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, PARAMETER_ERROR_INFO, true);
            return napi_invalid_arg;
        }
        context->gid = NapiUtils::Convert2String(context->env_, argv[0]);
        if (context->gid == "") {
            NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, PARAMETER_ERROR_INFO, true);
            return napi_invalid_arg;
        }
        uint32_t length = 0;
        napi_get_array_length(context->env_, argv[1], &length);
        for (uint32_t index = 0; index < length; ++index) {
            napi_value name = nullptr;
            if (napi_get_element(context->env_, argv[1], index, &name) != napi_ok) {
                continue;
            }
            if (NapiUtils::GetValueType(context->env_, name) != napi_string) {
                continue;
            }
            context->tids.emplace_back(NapiUtils::Convert2String(context->env_, name));
        }
        return napi_ok;
    };
    auto output = [context](napi_value *result) -> napi_status {
        if (context->innerCode_ != E_OK) {
            return napi_generic_failure;
        }
        return napi_ok;
    };
    auto exec = [context]() {
        context->innerCode_ = RequestManager::GetInstance()->AttachGroup(context->gid, context->tids);
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
    context->withErrCode_ = true;
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
        if (context->gid == "") {
            NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, PARAMETER_ERROR_INFO, true);
            return napi_invalid_arg;
        }
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