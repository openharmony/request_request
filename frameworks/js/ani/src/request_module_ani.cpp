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
#include "class.h"
#include "log.h"
#include "memory.h"
#include "ani_utils.h"
#include "ani_task.h"
#include "ani_js_initialize.h"
#include "request_common.h"

using namespace OHOS;
using namespace OHOS::Request;
using namespace OHOS::AniUtil;

template<>
bool UnionAccessor::TryConvertArray<ani_ref>(std::vector<ani_ref> &value)
{
    ani_double length;
    if (ANI_OK != env_->Object_GetPropertyByName_Double(obj_, "length", &length)) {
        return false;
    }

    for (int i = 0; i < int(length); i++) {
        ani_ref ref;
        if (ANI_OK != env_->Object_CallMethodByName_Ref(obj_, "$_get", "I:Lstd/core/Object;", &ref, (ani_int)i)) {
            return false;
        }
        value.push_back(ref);
    }
    return true;
}

static void ThrowBusinessError(ani_env *env, int errCode, std::string&& errMsg)
{
    REQUEST_HILOGI("into ThrowBusinessError.");
    static const char *errorClsName = "L@ohos/base/BusinessError;";
    ani_class cls {};
    if (env->FindClass(errorClsName, &cls) != ANI_OK) {
        REQUEST_HILOGE("find class BusinessError %{public}s failed", errorClsName);
        return;
    }
    ani_method ctor;
    if (env->Class_FindMethod(cls, "<ctor>", ":V", &ctor) != ANI_OK) {
        REQUEST_HILOGE("find method BusinessError.constructor failed");
        return;
    }
    ani_object errorObject;
    if (env->Object_New(cls, ctor, &errorObject) != ANI_OK) {
        REQUEST_HILOGE("create BusinessError object failed");
        return;
    }
    ani_double aniErrCode = static_cast<ani_double>(errCode);
    ani_string errMsgStr;
    if (env->String_NewUTF8(errMsg.c_str(), errMsg.size(), &errMsgStr) != ANI_OK) {
        REQUEST_HILOGE("convert errMsg to ani_string failed");
        return;
    }
    if (env->Object_SetFieldByName_Double(errorObject, "code", aniErrCode) != ANI_OK) {
        REQUEST_HILOGE("set error code failed");
        return;
    }
    if (env->Object_SetPropertyByName_Ref(errorObject, "message", errMsgStr) != ANI_OK) {
        REQUEST_HILOGE("set error message failed");
        return;
    }
    env->ThrowError(static_cast<ani_error>(errorObject));
    return;
}

static ExceptionError InitConfig(ani_env *env, ani_object object, Config &config)
{
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;
    ExceptionError error = { .code = E_OK };
    context = JsInitialize::GetContext(env, object);
    if (context == nullptr) {
        REQUEST_HILOGE("context == null");
        error.code = E_PARAMETER_CHECK;
        error.errInfo = "Parameter verification failed, Get context fail";
        return error;
    }
    auto applicationInfo = context->GetApplicationInfo();
    if (applicationInfo == nullptr) {
        REQUEST_HILOGE("ApplicationInfo == null");
        error.code = E_OTHER;
        error.errInfo = "ApplicationInfo is null";
        return error;
    }
    config.bundleType = static_cast<u_int32_t>(applicationInfo->bundleType);
    config.bundleName = context->GetBundleName();
    config.version = Version::API10;
    bool ret = JsInitialize::CheckFilePath(context, config, error);
    if (!ret) {
        REQUEST_HILOGE("error info is: %{public}s", error.errInfo.c_str());
    }
    return error;
}

static bool IsArray(ani_env *env, ani_object aniData)
{
    ani_double length;
    if (ANI_OK != env->Object_GetPropertyByName_Double(aniData, "length", &length)) {
        return false;
    }
    return true;
}

ani_boolean OHOS::AniUtil::IsInstanceOf(ani_env *env, const std::string &cls_name, ani_object obj)
{
    ani_class cls;
    if (ANI_OK != env->FindClass(cls_name.c_str(), &cls)) {
        return ANI_FALSE;
    }

    ani_boolean ret;
    env->Object_InstanceOf(obj, cls, &ret);
    return ret;
}

static bool GetDownloadData(ani_env *env, Config &aniConfig, ani_object aniData)
{
    UnionAccessor unionAccessor(env, aniData);
    if (unionAccessor.IsInstanceOf("Lstd/core/String;")) {
        aniConfig.data = AniStringUtils::ToStd(env, static_cast<ani_string>(aniData));
    }
    return true;
}

static bool ProcessDatas(ani_env *env, Config &aniConfig, ani_object aniData)
{
    UnionAccessor unionAccessor(env, aniData);
    if (aniConfig.action == Action::DOWNLOAD) {
        return GetDownloadData(env, aniConfig, aniData);
    }
    if (aniConfig.action != Action::UPLOAD) {
        return false;
    }

    std::vector<ani_ref> arrayDoubleValues = {};
    if (!unionAccessor.TryConvertArray<ani_ref>(arrayDoubleValues) || arrayDoubleValues.empty()) {
        return false;
    }

    for (uint16_t i = 0; i < arrayDoubleValues.size(); i++) {
        ani_object data = static_cast<ani_object>(arrayDoubleValues[i]);
        ani_ref nameRef;
        if (ANI_OK != env->Object_GetPropertyByName_Ref(data, "name", &nameRef)) {
            REQUEST_HILOGE("Object_GetFieldByName_Ref name from data Faild");
            return false;
        }
        auto name = AniStringUtils::ToStd(env, static_cast<ani_string>(nameRef));

        ani_ref valueRef;
        if (ANI_OK != env->Object_GetPropertyByName_Ref(data, "value", &valueRef)) {
            REQUEST_HILOGE("Object_GetFieldByName_Ref value from data Faild");
            return false;
        }
        if (IsInstanceOf(env, "Lstd/core/String;", static_cast<ani_object>(valueRef))) {
            FormItem form;
            form.name = name;
            form.value = AniStringUtils::ToStd(env, static_cast<ani_string>(valueRef));
            aniConfig.forms.push_back(form);
            continue;
        }
        if (IsInstanceOf(env, "L@ohos/request/request/agent/FileSpec;", static_cast<ani_object>(valueRef))) {
            FileSpec file;
            if (!JsInitialize::Convert2FileSpec(env, static_cast<ani_object>(valueRef), name, file)) {
                REQUEST_HILOGE("Convert2FileSpec failed");
                return false;
            }
            aniConfig.files.push_back(file);
            continue;
        }
        if (!IsArray(env, static_cast<ani_object>(valueRef))) {
            return false;
        }
        if (!JsInitialize::Convert2FileSpecs(env, static_cast<ani_object>(valueRef), name, aniConfig.files)) {
            return false;
        }
    }
    return true;
}

static bool SetConfigInfo(ani_env *env, Config &aniConfig, ani_object config)
{
    ani_ref url;
    if (ANI_OK != env->Object_GetPropertyByName_Ref(config, "url", &url)) {
        REQUEST_HILOGI("Failed to get property named type");
        return false;
    }
    auto urlStr = AniStringUtils::ToStd(env, static_cast<ani_string>(url));
    REQUEST_HILOGI("urlStr: %{public}s", urlStr.c_str());
    aniConfig.url = urlStr;
    ani_ref aniAction;
    if (ANI_OK != env->Object_GetPropertyByName_Ref(config, "action", &aniAction)) {
        REQUEST_HILOGI("Failed to get property named type");
        return false;
    }
    EnumAccessor actionAccessor(env, static_cast<ani_enum_item>(aniAction));
    expected<Action, ani_status> actionExpected = actionAccessor.To<Action>();
    if (!actionExpected) {
        return false;
    }
    Action action = actionExpected.value();
    aniConfig.action = action;
    REQUEST_HILOGI("vibrateInfo.type: %{public}d", action);

    aniConfig.overwrite = true;
    
    ani_ref aniMethod;
    if (env->Object_GetPropertyByName_Ref(config, "method", &aniMethod) == ANI_OK && aniMethod != nullptr) {
        auto method = AniStringUtils::ToStd(env, static_cast<ani_string>(aniMethod));
        aniConfig.method = method;
    }

    ani_ref aniSaveas;
    if (env->Object_GetPropertyByName_Ref(config, "saveas", &aniSaveas) == ANI_OK && aniSaveas != nullptr) {
        auto saveas = AniStringUtils::ToStd(env, static_cast<ani_string>(aniSaveas));
        aniConfig.saveas = saveas;
    }

    ani_ref aniData;
    if (env->Object_GetPropertyByName_Ref(config, "data", &aniData) == ANI_OK && aniData != nullptr) {
        bool ret = ProcessDatas(env, aniConfig, static_cast<ani_object>(aniData));
        if (!ret) {
            REQUEST_HILOGE("ProcessDatas data error.");
            return ret;
        }
    }
    return true;
}

static ani_object Create([[maybe_unused]] ani_env *env, ani_object object, ani_object config)
{
    REQUEST_HILOGI("Create Start");
    ani_object nullobj{};
    if (object == nullptr) {
        REQUEST_HILOGE("context == null");
        return nullobj;
    }
    if (config == nullptr) {
        REQUEST_HILOGE("config == null");
        return nullobj;
    }

    Config aniConfig{};
    aniConfig.saveas = "default.txt";
    if (!SetConfigInfo(env, aniConfig, config)) {
        REQUEST_HILOGE("Failed to SetConfigInfo.");
        return nullobj;
    }

    ExceptionError err = InitConfig(env, object, aniConfig);
    if (err.code != E_OK) {
        REQUEST_HILOGE("err.code : %{public}d, err.errInfo :  %{public}s", err.code, err.errInfo.c_str());
        ThrowBusinessError(env, err.code, std::move(err.errInfo));
        return nullobj;
    }

    AniTask *task = AniTask::Create(env, aniConfig);
    if (task == nullptr) {
        REQUEST_HILOGE("AniTask::Create task == nullptr!");
        return nullobj;
    }

    auto taskImpl = AniObjectUtils::Create(env, "@ohos.request.request", "agent", "TaskImpl");

    NativePtrWrapper wrapper(env, taskImpl);
    wrapper.Wrap<AniTask>(task);
    return taskImpl;
}

static void StartSync([[maybe_unused]] ani_env *env, ani_object object)
{
    REQUEST_HILOGI("Enter Start");
    if (env == nullptr) {
        return;
    }
    NativePtrWrapper wrapper(env, object);
    auto task = wrapper.Unwrap<AniTask>();
    if (task == nullptr) {
        REQUEST_HILOGE("task is nullptr");
        return;
    }
    task->Start(env);
}

static void OnSync([[maybe_unused]] ani_env *env, [[maybe_unused]] ani_object object,
    ani_string response, ani_object callback)
{
    REQUEST_HILOGI("Enter On");

    ani_ref callbackRef = nullptr;
    env->GlobalReference_Create(reinterpret_cast<ani_ref>(callback), &callbackRef);
    auto responseEvent = AniStringUtils::ToStd(env, static_cast<ani_string>(response));
    NativePtrWrapper wrapper(env, object);
    auto task = wrapper.Unwrap<AniTask>();
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

    static const char *agentNamespaceName = "@ohos.request.request.agent";
    ani_namespace agent;
    if (ANI_OK != env->FindNamespace(agentNamespaceName, &agent)) {
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

    static const char *requestclsName = "@ohos.request.request.agent.TaskImpl";
    ani_class requestClass;
    if (ANI_OK != env->FindClass(requestclsName, &requestClass)) {
        REQUEST_HILOGI("Not found class %{public}s", requestclsName);
        return ANI_NOT_FOUND;
    }

    std::array methods = {
        ani_native_function {"startSync", nullptr, reinterpret_cast<void *>(StartSync)},
        ani_native_function {"onSync", nullptr, reinterpret_cast<void *>(OnSync)},
    };

    if (ANI_OK != env->Class_BindNativeMethods(requestClass, methods.data(), methods.size())) {
        REQUEST_HILOGI("Cannot bind native methods to %{public}s", requestclsName);
        return ANI_ERROR;
    }

    auto cleanerCls = TypeFinder(env).FindClass("ohos.request.request.agent.Cleaner");
    NativePtrCleaner(env).Bind(cleanerCls.value());

    *result = ANI_VERSION_1;
    return ANI_OK;
}
