/*
 * Copyright (C) 2025 Huawei Device Co., Ltd.
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
#include <ani.h>
#include <fcntl.h>
#include <securec.h>
#include <sys/stat.h>
#include <filesystem>
#include <iostream>
#include <regex>
#include <string>
#include <system_error>

#include "constant.h"
#include "log.h"
#include "ani_utils.h"
#include "ani_task.h"

using namespace OHOS;
using namespace OHOS::Request;

static ani_object Create([[maybe_unused]] ani_env *env, ani_object context, ani_object config)
{
    REQUEST_HILOGI("Create start");
    ani_object nullobj{};
    if (context == nullptr) {
        REQUEST_HILOGI("context == null");
        return nullobj;
    }
    if (config == nullptr) {
        REQUEST_HILOGI("config == null");
        return nullobj;
    }

    Config aniConfig{};
    ani_ref url;
    if (ANI_OK != env->Object_GetPropertyByName_Ref(config, "url", &url)) {
        REQUEST_HILOGI("Failed to get property named type");
        return nullobj;
    }
    auto urlStr = AniStringUtils::ToStd(env, static_cast<ani_string>(url));
    REQUEST_HILOGI("vibrateInfo.type: %{public}s", urlStr.c_str());

    ani_ref aniAction;
    if (ANI_OK != env->Object_GetPropertyByName_Ref(config, "action", &aniAction)) {
        REQUEST_HILOGI("Failed to get property named type");
        return nullobj;
    }
    EnumAccessor actionAccessor(env, static_cast<ani_enum_item>(aniAction));
    expected<Action, ani_status> actionExpected = actionAccessor.To<Action>();
    if (!actionExpected) {
        return nullobj;
    }
    Action action = actionExpected.value();
    REQUEST_HILOGI("vibrateInfo.type: %{public}d", action);
    aniConfig.action = action;
    aniConfig.url = urlStr;
    AniTask *task = AniTask::Create(env, aniConfig);
    if (task == nullptr) {
        REQUEST_HILOGI("AniTask::Create task == nullptr!");
        return nullobj;
    }

    auto taskImpl = AniObjectUtils::Create(env, "L@ohos/request/request;", "Lagent;", "LTaskImpl;");
    AniObjectUtils::Wrap<AniTask>(env, taskImpl, task);
    return taskImpl;
}

static void startSync([[maybe_unused]] ani_env *env, ani_object object)
{
    REQUEST_HILOGI("Enter Start");
    auto task = AniObjectUtils::Unwrap<AniTask>(env, object);
    if (task == nullptr) {
        REQUEST_HILOGE("task is nullptr");
        return;
    }
    task->Start();
}

static void onSync([[maybe_unused]] ani_env *env, [[maybe_unused]] ani_object object,
    ani_string response, ani_object callback)
{
    REQUEST_HILOGI("Enter On");

    ani_ref callbackRef = nullptr;
    env->GlobalReference_Create(reinterpret_cast<ani_ref>(callback), &callbackRef);
    auto responseEvent = AniStringUtils::ToStd(env, static_cast<ani_string>(response));
    auto task = AniObjectUtils::Unwrap<AniTask>(env, object);
    if (task == nullptr) {
        REQUEST_HILOGE("task is nullptr");
        return;
    }
    task->On(env, responseEvent, callbackRef);
}

ANI_EXPORT ani_status ANI_Constructor(ani_vm *vm, uint32_t *result)
{
    REQUEST_HILOGI("Enter ANI_Constructor Start");
    ani_env *env;
    if (ANI_OK != vm->GetEnv(ANI_VERSION_1, &env)) {
        REQUEST_HILOGI("Unsupported ANI_VERSION_1");
        return ANI_ERROR;
    }

    static const char *namespaceName = "L@ohos/request/request;";
    ani_namespace request;
    if (ANI_OK != env->FindNamespace(namespaceName, &request)) {
        REQUEST_HILOGI("Not found '%{public}s'", namespaceName);
        return ANI_ERROR;
    }

    static const char *agentNamespaceName = "Lagent;";
    ani_namespace agent;
    if (ANI_OK != env->Namespace_FindNamespace(request, agentNamespaceName, &agent)) {
        REQUEST_HILOGI("Not found '%{public}s'", agentNamespaceName);
        return ANI_ERROR;
    }
    std::array nsMethods = {
        ani_native_function {"createSync", nullptr, reinterpret_cast<void *>(Create)},
    };

    if (ANI_OK != env->Namespace_BindNativeFunctions(agent, nsMethods.data(), nsMethods.size())) {
        REQUEST_HILOGI("Cannot bind native methods to '%{public}s'", namespaceName);
        return ANI_ERROR;
    };

    static const char *requestclsName = "LTaskImpl;";
    ani_class requestClass;
    if (ANI_OK != env->Namespace_FindClass(agent, requestclsName, &requestClass)) {
        REQUEST_HILOGI("Not found class %{public}s", requestclsName);
        return ANI_NOT_FOUND;
    }

    std::array methods = {
        ani_native_function {"startSync", nullptr, reinterpret_cast<void *>(startSync)},
        ani_native_function {"onSync", nullptr, reinterpret_cast<void *>(onSync)},
    };

    if (ANI_OK != env->Class_BindNativeMethods(requestClass, methods.data(), methods.size())) {
        REQUEST_HILOGI("Cannot bind native methods to %{public}s", requestclsName);
        return ANI_ERROR;
    }

    *result = ANI_VERSION_1;
    return ANI_OK;
}
