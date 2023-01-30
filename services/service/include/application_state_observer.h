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

#ifndef MISCSERVICES_REQUEST_APPLICATION_STATE_OBSERVER_H
#define MISCSERVICES_REQUEST_APPLICATION_STATE_OBSERVER_H

#include <functional>
#include <map>
#include <string>

#include "application_state_observer_stub.h"

namespace OHOS::Request::Download {
class ApplicationStateObserver {
public:
    ~ApplicationStateObserver();
    using RegCallBack = std::function<void(const std::string bundleName, int32_t uid, int32_t state)>;
    static ApplicationStateObserver& GetInstance();
    bool RegisterAppStateChanged(RegCallBack &&callback);
private:
    class AppProcessState : public AppExecFwk::ApplicationStateObserverStub {
    public:
        explicit AppProcessState(ApplicationStateObserver &appStateObserver) : appStateObserver_(appStateObserver) {}
        ~AppProcessState() override = default;
        void OnForegroundApplicationChanged(const AppExecFwk::AppStateData &appStateData) override;
        void OnAbilityStateChanged(const AppExecFwk::AbilityStateData &abilityStateData) override;
        void OnExtensionStateChanged(const AppExecFwk::AbilityStateData &abilityStateData) override;
        void OnProcessCreated(const AppExecFwk::ProcessData &processData) override;
        void OnProcessDied(const AppExecFwk::ProcessData &processData) override;
    private:
        void RunCallback(const std::string bundleName, int32_t uid, int32_t state);
        ApplicationStateObserver& appStateObserver_;
    };
    ApplicationStateObserver();
    RegCallBack callback_ = nullptr;
};
}

#endif // MISCSERVICES_REQUEST_APPLICATION_STATE_OBSERVER_H
