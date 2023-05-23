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
    REQUEST_HILOGI("RegisterAppState In");
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
            callback_ = callback;
            return true;
        }
        REQUEST_HILOGE("register fail, ret = %{public}d", ret);
        return false;
    }
    REQUEST_HILOGI("RegisterAppState Out");
    return false;
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
    RunCallback(abilityStateData.uid, abilityStateData.abilityState);
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
    REQUEST_HILOGD("OnProcessDied uid=%{public}d,  bundleName=%{public}s, state=%{public}d", processData.uid,
        processData.bundleName.c_str(), static_cast<int32_t>(processData.state));
    RunCallback(processData.uid, static_cast<int32_t>(processData.state));
}

void ApplicationStateObserver::AppProcessState::RunCallback(int32_t uid, int32_t state)
{
    REQUEST_HILOGI("running callback function in");
    if (appStateObserver_.callback_ != nullptr) {
        REQUEST_HILOGI("appStateObserver_.callback_ != nullptr");
        appStateObserver_.callback_(uid, state);
    }
    REQUEST_HILOGI("running callback function end");
}
} // namespace OHOS::Request

using namespace OHOS::Request;
void RegisterAPPStateCallback(APPStateCallback fun)
{
    ApplicationStateObserver::GetInstance().RegisterAppStateChanged(fun);
    REQUEST_HILOGD("running RegisterAPPStateCallback");
}