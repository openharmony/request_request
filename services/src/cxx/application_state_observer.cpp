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

#include "application_state_observer.h"

#include <mutex>
#include <string>

#include "app_mgr_client.h"
#include "app_mgr_interface.h"
#include "app_process_data.h"
#include "iservice_registry.h"
#include "log.h"
#include "sys_event.h"
#include "sys_mgr_client.h"
#include "system_ability.h"
#include "system_ability_definition.h"

namespace OHOS::Request {
AppProcessState::AppProcessState()
{
}

AppProcessState::~AppProcessState()
{
}

sptr<AppProcessState> AppProcessState::GetInstance()
{
    static sptr<AppProcessState> observer = new AppProcessState();
    return observer;
}

bool AppProcessState::RegisterAppStateChanged(RegCallBack &&callback)
{
    REQUEST_HILOGI("RegisterAppState In");
    sptr<AppProcessState> appProcessState = AppProcessState::GetInstance();
    if (appProcessState == nullptr) {
        REQUEST_HILOGE("create AppProcessState fail");
        return false;
    }
    auto systemAbilityManager = SystemAbilityManagerClient::GetInstance().GetSystemAbilityManager();
    if (systemAbilityManager == nullptr) {
        REQUEST_HILOGE("get SystemAbilityManager failed.");
        SysEventLog::SendSysEventLog(FAULT_EVENT, SAMGR_FAULT_00, "Get SAM failed");
        return false;
    }
    auto systemAbility = systemAbilityManager->GetSystemAbility(APP_MGR_SERVICE_ID);
    if (systemAbility == nullptr) {
        REQUEST_HILOGE("get SystemAbility failed.");
        return false;
    }
    sptr<AppExecFwk::IAppMgr> appObject = iface_cast<AppExecFwk::IAppMgr>(systemAbility);
    if (appObject) {
        int ret = appObject->RegisterApplicationStateObserver(appProcessState);
        if (ret == ERR_OK) {
            {
                std::lock_guard<std::mutex> lockApp(appStateMutex);
                REQUEST_HILOGI("register success");
                appStateCallback_ = callback;
            }
            return true;
        }
        REQUEST_HILOGE("register fail, ret = %{public}d", ret);
        return false;
    }
    REQUEST_HILOGI("RegisterAppState Out");
    return false;
}

void AppProcessState::RegisterProcessDied(ProcessCallBack &&callback)
{
    std::lock_guard<std::mutex> lockProcess(processMutex);
    processCallback_ = callback;
}

void AppProcessState::OnAppStateChanged(const AppExecFwk::AppStateData &appStateData)
{
    REQUEST_HILOGI("OnAppStateChanged uid=%{public}d, bundleName=%{public}s,state=%{public}d", appStateData.uid,
        appStateData.bundleName.c_str(), appStateData.state);
    RunAppStateCallback(appStateData.uid, appStateData.state, appStateData.pid);
}

void AppProcessState::RunAppStateCallback(int32_t uid, int32_t state, int32_t pid)
{
    std::lock_guard<std::mutex> lockApp(appStateMutex);
    if (appStateCallback_ == nullptr) {
        REQUEST_HILOGE("appStateObserver callback is nullptr");
        return;
    }
    appStateCallback_(uid, state, pid);
}

void AppProcessState::OnProcessDied(const AppExecFwk::ProcessData &processData)
{
    REQUEST_HILOGD("OnProcessDied uid=%{public}d, bundleName=%{public}s, state=%{public}d, pid=%{public}d",
        processData.uid, processData.bundleName.c_str(), static_cast<int32_t>(processData.state), processData.pid);
    RunProcessDiedCallback(
        processData.uid, static_cast<int32_t>(processData.state), processData.pid, processData.bundleName);
}

void AppProcessState::RunProcessDiedCallback(int32_t uid, int32_t state, int32_t pid, const std::string &bundleName)
{
    std::lock_guard<std::mutex> lockProcess(processMutex);
    if (processCallback_ == nullptr) {
        REQUEST_HILOGE("processStateObserver callback is nullptr");
        return;
    }
    CStringWrapper name = WrapperCString(bundleName);
    processCallback_(uid, state, pid, name);
}

void AppProcessState::OnAbilityStateChanged(const AppExecFwk::AbilityStateData &abilityStateData)
{
}

void AppProcessState::OnExtensionStateChanged(const AppExecFwk::AbilityStateData &extensionStateData)
{
}

void AppProcessState::OnProcessCreated(const AppExecFwk::ProcessData &processData)
{
}

void AppProcessState::OnForegroundApplicationChanged(const AppExecFwk::AppStateData &appStateData)
{
}
} // namespace OHOS::Request

using namespace OHOS::Request;
void RegisterAPPStateCallback(APPStateCallback fun)
{
    AppProcessState::GetInstance()->RegisterAppStateChanged(fun);
    REQUEST_HILOGD("running RegisterAPPStateCallback");
}

void RegisterProcessDiedCallback(ProcessStateCallback fun)
{
    AppProcessState::GetInstance()->RegisterProcessDied(fun);
    REQUEST_HILOGD("running RegisterProcessDiedCallback");
}
