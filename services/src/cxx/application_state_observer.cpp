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

#include <string>

#include "app_mgr_client.h"
#include "app_mgr_interface.h"
#include "app_process_data.h"
#include "iservice_registry.h"
#include "log.h"
#include "sys_mgr_client.h"
#include "system_ability.h"
#include "system_ability_definition.h"

namespace OHOS::Request {
ApplicationStateObserver::ApplicationStateObserver()
{
}

ApplicationStateObserver::~ApplicationStateObserver()
{
}

ApplicationStateObserver &ApplicationStateObserver::GetInstance()
{
    static ApplicationStateObserver observer;
    return observer;
}

bool ApplicationStateObserver::RegisterAppStateChanged(RegCallBack &&callback)
{
    REQUEST_HILOGD("RegisterAppState In");
    sptr<AppProcessState> appProcessState = new (std::nothrow) AppProcessState(*this);
    if (appProcessState == nullptr) {
        REQUEST_HILOGE("create AppProcessState fail, not enough memory");
        return false;
    }
    auto systemAbilityManager = SystemAbilityManagerClient::GetInstance().GetSystemAbilityManager();
    if (systemAbilityManager == nullptr) {
        REQUEST_HILOGE("get SystemAbilityManager failed.");
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
            REQUEST_HILOGD("register success");
            appStateCallback_ = callback;
            return true;
        }
        REQUEST_HILOGE("register fail, ret = %{public}d", ret);
        return false;
    }
    REQUEST_HILOGD("RegisterAppState Out");
    return false;
}

void ApplicationStateObserver::RegisterProcessStateChanged(RegCallBack &&callback)
{
    processCallback_ = callback;
}

void ApplicationStateObserver::AppProcessState::OnForegroundApplicationChanged(
    const AppExecFwk::AppStateData &appStateData)
{
}

void ApplicationStateObserver::AppProcessState::OnAbilityStateChanged(
    const AppExecFwk::AbilityStateData &abilityStateData)
{
    REQUEST_HILOGD("OnAbilityStateChanged uid=%{public}d,  bundleName=%{public}s,state=%{public}d",
        abilityStateData.uid, abilityStateData.bundleName.c_str(), abilityStateData.abilityState);
    RunAppStateCallback(abilityStateData.uid, abilityStateData.abilityState, abilityStateData.pid);
}

void ApplicationStateObserver::AppProcessState::OnExtensionStateChanged(
    const AppExecFwk::AbilityStateData &extensionStateData)
{
}

void ApplicationStateObserver::AppProcessState::OnProcessCreated(const AppExecFwk::ProcessData &processData)
{
}

void ApplicationStateObserver::AppProcessState::OnProcessDied(const AppExecFwk::ProcessData &processData)
{
    REQUEST_HILOGD("OnProcessDied uid=%{public}d,  bundleName=%{public}s, state=%{public}d, pid=%{public}d",
        processData.uid, processData.bundleName.c_str(), static_cast<int32_t>(processData.state), processData.pid);
    RunProcessStateCallback(processData.uid, static_cast<int32_t>(processData.state), processData.pid);
}

void ApplicationStateObserver::AppProcessState::RunAppStateCallback(int32_t uid, int32_t state, int32_t pid)
{
    if (appStateObserver_.appStateCallback_ == nullptr) {
        REQUEST_HILOGE("appStateObserver callback is nullptr");
        return;
    }
    appStateObserver_.appStateCallback_(uid, state, pid);
}

void ApplicationStateObserver::AppProcessState::RunProcessStateCallback(int32_t uid, int32_t state, int32_t pid)
{
    if (appStateObserver_.processCallback_ == nullptr) {
        REQUEST_HILOGE("processStateObserver callback is nullptr");
        return;
    }
    appStateObserver_.processCallback_(uid, state, pid);
}
} // namespace OHOS::Request

using namespace OHOS::Request;
void RegisterAPPStateCallback(APPStateCallback fun)
{
    ApplicationStateObserver::GetInstance().RegisterAppStateChanged(fun);
    REQUEST_HILOGD("running RegisterAPPStateCallback");
}

void RegisterProcessStateCallback(APPStateCallback fun)
{
    ApplicationStateObserver::GetInstance().RegisterProcessStateChanged(fun);
    REQUEST_HILOGD("running RegisterProcessStateCallback");
}