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

#include "js_initialize.h"

#include "log.h"
#include "napi_utils.h"
#include "request_manager.h"
#include "sys_event.h"

#include "want_agent_helper.h"
#include "want_agent.h"

namespace OHOS::Request {

napi_value JsInitialize::Initialize(napi_env env, napi_callback_info info, Version version, bool firstInit)
{
    REQUEST_HILOGD("constructor request task!");
    // todo: check if needed
    bool withErrCode = version != Version::API8;
    napi_value self = nullptr;
    size_t argc = NapiUtils::MAX_ARGC;
    napi_value argv[NapiUtils::MAX_ARGC] = { nullptr };
    REQUEST_NAPI_CALL(env, napi_get_cb_info(env, info, &argc, argv, &self, nullptr), "napi_get_cb_info failed");
    int32_t number = version == Version::API8 ? NapiUtils::ONE_ARG : NapiUtils::TWO_ARG;
    if (static_cast<int32_t>(argc) < number) {
        NapiUtils::ThrowError(
            env, E_PARAMETER_CHECK, "Missing mandatory parameters, invalid parameter count", withErrCode);
        return nullptr;
    }

    Config config;
    config.version = version;
    config.withErrCode = withErrCode;
    // todo: check if needed
    config.firstInit = firstInit;
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;
    ExceptionError err = InitParam(env, argv, context, config);
    if (err.code != E_OK) {
        REQUEST_HILOGE("err.code : %{public}d, err.errInfo :  %{public}s", err.code, err.errInfo.c_str());
        NapiUtils::ThrowError(env, err.code, err.errInfo, withErrCode);
        return nullptr;
    }

    auto *task = new (std::nothrow) JsTask();
    if (task == nullptr) {
        REQUEST_HILOGE("Create task object failed");
        return nullptr;
    }
    task->config_ = config;
    task->isGetPermission = true;
    RequestManager::GetInstance()->RestoreListener(JsTask::ReloadListener);
    // `finalize` executes on the JS thread
    auto finalize = [](napi_env env, void *data, void *hint) {
        JsTask *task = reinterpret_cast<JsTask *>(data);
        RequestManager::GetInstance()->RemoveAllListeners(task->GetTid());
        REQUEST_HILOGI("finalize task %{public}s", task->GetTid().c_str());
        delete task;
    };
    if (napi_wrap(env, self, task, finalize, nullptr, nullptr) != napi_ok) {
        finalize(env, task, nullptr);
        return nullptr;
    }
    return self;
}

ExceptionError JsInitialize::InitParam(
    napi_env env, napi_value *argv, std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config)
{
    REQUEST_HILOGD("InitParam in");
    ExceptionError err = { .code = E_OK };
    int parametersPosition = config.version == Version::API8 ? CONFIG_PARAM_AT_FIRST : CONFIG_PARAM_AT_SECOND;

    napi_status getStatus = GetContext(env, argv[0], context);
    if (getStatus != napi_ok) {
        REQUEST_HILOGE("Get context fail");
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Parameter verification failed, Get context fail";
        return err;
    }
    auto applicationInfo = context->GetApplicationInfo();
    if (applicationInfo == nullptr) {
        err.code = E_OTHER;
        err.errInfo = "ApplicationInfo is null";
        SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_03, err.errInfo);
        return err;
    }
    config.bundleType = static_cast<u_int32_t>(applicationInfo->bundleType);
    REQUEST_HILOGD("config.bundleType is %{public}d", config.bundleType);
    if (!ParseConfig(env, argv[parametersPosition], config, err.errInfo)) {
        err.code = E_PARAMETER_CHECK;
        return err;
    }
    config.bundleName = context->GetBundleName();
    REQUEST_HILOGD("config.bundleName is %{public}s", config.bundleName.c_str());
    CheckFilePath(context, config, err);
    return err;
}

napi_status JsInitialize::GetContext(
    napi_env env, napi_value value, std::shared_ptr<OHOS::AbilityRuntime::Context> &context)
{
    if (!IsStageMode(env, value)) {
        auto ability = OHOS::AbilityRuntime::GetCurrentAbility(env);
        if (ability == nullptr) {
            REQUEST_HILOGE("Get current ability fail");
            SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_04, "Get current ability fail");
            return napi_generic_failure;
        }
        context = ability->GetAbilityContext();
    } else {
        context = OHOS::AbilityRuntime::GetStageModeContext(env, value);
    }
    if (context == nullptr) {
        REQUEST_HILOGE("Get Context failed, context is nullptr.");
        SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_06, "Get Context failed");
        return napi_generic_failure;
    }
    return napi_ok;
}

bool JsInitialize::GetAppBaseDir(std::string &baseDir)
{
    auto context = AbilityRuntime::Context::GetApplicationContext();
    if (context == nullptr) {
        REQUEST_HILOGE("AppContext is null.");
        SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_02, "AppContext is null");
        return false;
    }
    baseDir = context->GetBaseDir();
    if (baseDir.empty()) {
        REQUEST_HILOGE("Base dir not found.");
        SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_07, "Base dir not found");
        return false;
    }
    return true;
}

bool JsInitialize::IsStageMode(napi_env env, napi_value value)
{
    bool stageMode = true;
    napi_status status = OHOS::AbilityRuntime::IsStageContext(env, value, stageMode);
    if (status != napi_ok || !stageMode) {
        return false;
    }
    return true;
}

} // namespace OHOS::Request
