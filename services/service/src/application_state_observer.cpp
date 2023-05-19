/*
* Copyright (c) 2022 Huawei Device Co., Ltd.
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
#include "app_mgr_interface.h"
#include "app_process_data.h"
#include "app_mgr_client.h"
#include "iservice_registry.h"
#include "system_ability.h"
#include "system_ability_definition.h"
#include "sys_mgr_client.h"
#include "log.h"

namespace OHOS::Request::Download {
ApplicationStateObserver::ApplicationStateObserver()
{}

ApplicationStateObserver::~ApplicationStateObserver()
{}

ApplicationStateObserver &ApplicationStateObserver::GetInstance()
{
    static ApplicationStateObserver observer;
    return observer;
}

bool ApplicationStateObserver::RegisterAppStateChanged(RegCallBack&& callback)
{
    DOWNLOAD_HILOGD("RegisterAppState");
    sptr<AppProcessState> appProcessState_ = new (std::nothrow) AppProcessState(*this);
    if (appProcessState_ == nullptr) {
        DOWNLOAD_HILOGE("create AppProcessState fail, not enough memory");
        return false;
    }
    auto systemAbilityManager = SystemAbilityManagerClient::GetInstance().GetSystemAbilityManager();
    if (systemAbilityManager == nullptr) {
        DOWNLOAD_HILOGE("get SystemAbilityManager failed.");
        return false;
    }
    sptr<AppExecFwk::IAppMgr> appObject =
        iface_cast<AppExecFwk::IAppMgr>(systemAbilityManager->GetSystemAbility(APP_MGR_SERVICE_ID));
    if (appObject) {
        int ret = appObject->RegisterApplicationStateObserver(appProcessState_);
        if (ret == ERR_OK) {
            DOWNLOAD_HILOGD("register success");
            callback_ = callback;
            return true;
        }
        DOWNLOAD_HILOGE("register fail, ret = %{public}d", ret);
        return false;
    }
    DOWNLOAD_HILOGE("get SystemAbilityManager fail");
    return false;
}

void ApplicationStateObserver::AppProcessState::OnForegroundApplicationChanged(
    const AppExecFwk::AppStateData &appStateData)
{
}

void ApplicationStateObserver::AppProcessState::OnAbilityStateChanged(
    const AppExecFwk::AbilityStateData &abilityStateData)
{
    DOWNLOAD_HILOGD("OnAbilityStateChanged uid=%{public}d,  bundleName=%{public}s,state=%{public}d",
        abilityStateData.uid, abilityStateData.bundleName.c_str(), abilityStateData.abilityState);
    RunCallback(abilityStateData.bundleName, abilityStateData.uid, abilityStateData.abilityState);
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
    DOWNLOAD_HILOGD("OnProcessDied uid=%{public}d, bundleName=%{public}s", processData.uid,
        processData.bundleName.c_str());
    RunCallback(processData.bundleName, processData.uid, static_cast<int32_t>(processData.state));
}

void ApplicationStateObserver::AppProcessState::RunCallback(const std::string bundleName, int32_t uid, int32_t state)
{
    if (appStateObserver_.callback_ != nullptr) {
        appStateObserver_.callback_(bundleName, uid, state);
    }
}
}
