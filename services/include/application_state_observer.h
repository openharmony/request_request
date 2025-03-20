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

#ifndef REQUEST_APPLICATION_STATE_OBSERVER_H
#define REQUEST_APPLICATION_STATE_OBSERVER_H

#include <cstdint>
#include <functional>
#include <map>
#include <mutex>
#include <string>

#include "application_state_observer_stub.h"
#include "c_string_wrapper.h"

namespace OHOS::Request {
class AppProcessState : public AppExecFwk::ApplicationStateObserverStub {
public:
    AppProcessState();
    ~AppProcessState();
    void OnForegroundApplicationChanged(const AppExecFwk::AppStateData &appStateData) override;
    void OnAppStateChanged(const AppExecFwk::AppStateData &appStateData) override;
    void OnAbilityStateChanged(const AppExecFwk::AbilityStateData &abilityStateData) override;
    void OnExtensionStateChanged(const AppExecFwk::AbilityStateData &abilityStateData) override;
    void OnProcessCreated(const AppExecFwk::ProcessData &processData) override;
    void OnProcessDied(const AppExecFwk::ProcessData &processData) override;

public:
    using RegCallBack = std::function<void(int32_t uid, int32_t state, int32_t pid)>;
    using ProcessCallBack = std::function<void(int32_t uid, int32_t state, int32_t pid, CStringWrapper bundleName)>;
    bool RegisterAppStateChanged(RegCallBack &&callback);
    void RegisterProcessDied(ProcessCallBack &&callback);
    static sptr<AppProcessState> GetInstance();

private:
    RegCallBack appStateCallback_ = nullptr;
    ProcessCallBack processCallback_ = nullptr;
    std::mutex appStateMutex;
    std::mutex processMutex;
    void RunAppStateCallback(int32_t uid, int32_t state, int32_t pid);
    void RunProcessDiedCallback(int32_t uid, int32_t state, int32_t pid, const std::string &bundleName);
};

} // namespace OHOS::Request

#ifdef __cplusplus
extern "C" {
#endif

typedef void (*APPStateCallback)(int32_t, int32_t, int32_t);
typedef void (*ProcessStateCallback)(int32_t, int32_t, int32_t, CStringWrapper);
void RegisterAPPStateCallback(APPStateCallback fun);
void RegisterProcessDiedCallback(ProcessStateCallback fun);

#ifdef __cplusplus
}
#endif

#endif // REQUEST_APPLICATION_STATE_OBSERVER_H
