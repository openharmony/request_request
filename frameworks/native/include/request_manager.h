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

#ifndef DOWNLOAD_MANAGER_H
#define DOWNLOAD_MANAGER_H

#include <atomic>
#include <condition_variable>
#include <map>
#include <mutex>

#include "constant.h"
#include "data_ability_helper.h"
#include "iremote_object.h"
#include "iservice_registry.h"
#include "js_common.h"
#include "notify_stub.h"
#include "refbase.h"
#include "request_service_interface.h"
#include "system_ability_status_change_stub.h"
#include "visibility.h"

namespace OHOS::Request {
class RequestSaDeathRecipient : public IRemoteObject::DeathRecipient {
public:
    explicit RequestSaDeathRecipient();
    ~RequestSaDeathRecipient() = default;
    void OnRemoteDied(const wptr<IRemoteObject> &object) override;
};

class RequestManager : public RefBase {
public:
    RequestManager();
    ~RequestManager();
    REQUEST_API static sptr<RequestManager> GetInstance();
    REQUEST_API int32_t Create(const Config &config, int32_t &tid, sptr<NotifyInterface> listener);
    REQUEST_API int32_t GetTask(const std::string &tid, const std::string &token, Config &config);
    REQUEST_API int32_t Start(const std::string &tid);
    REQUEST_API int32_t Stop(const std::string &tid);
    REQUEST_API int32_t Query(const std::string &tid, TaskInfo &info);
    REQUEST_API int32_t Touch(const std::string &tid, const std::string &token, TaskInfo &info);
    REQUEST_API int32_t Search(const Filter &filter, std::vector<std::string> &tids);
    REQUEST_API int32_t Show(const std::string &tid, TaskInfo &info);
    REQUEST_API int32_t Pause(const std::string &tid, Version version);
    REQUEST_API int32_t QueryMimeType(const std::string &tid, std::string &mimeType);
    REQUEST_API int32_t Remove(const std::string &tid, Version version);
    REQUEST_API int32_t Resume(const std::string &tid);

    REQUEST_API int32_t On(
        const std::string &type, const std::string &tid, const sptr<NotifyInterface> &listener, Version version);
    REQUEST_API int32_t Off(const std::string &type, const std::string &tid, Version version);

    REQUEST_API void RestoreListener(void (*callback)());
    void OnRemoteSaDied(const wptr<IRemoteObject> &object);
    REQUEST_API bool LoadRequestServer();

    REQUEST_API bool IsSaReady();
    void LoadServerSuccess();
    void LoadServerFail();

private:
    sptr<RequestServiceInterface> GetRequestServiceProxy();
    int32_t Retry(int32_t &taskId, const Config &config, int32_t errorCode, sptr<NotifyInterface> listener);
    void SetRequestServiceProxy(sptr<RequestServiceInterface> proxy);
    bool SubscribeSA(sptr<ISystemAbilityManager> systemAbilityManager);

private:
    static std::mutex instanceLock_;
    static sptr<RequestManager> instance_;
    std::mutex downloadMutex_;
    std::mutex conditionMutex_;
    std::mutex serviceProxyMutex_;

    sptr<RequestServiceInterface> requestServiceProxy_;
    sptr<RequestSaDeathRecipient> deathRecipient_;
    sptr<ISystemAbilityStatusChange> saChangeListener_;
    std::condition_variable syncCon_;
    std::atomic<bool> ready_ = false;
    static constexpr int LOAD_SA_TIMEOUT_MS = 15000;
    void (*callback_)() = nullptr;

private:
    class SystemAbilityStatusChangeListener : public OHOS::SystemAbilityStatusChangeStub {
    public:
        SystemAbilityStatusChangeListener();
        ~SystemAbilityStatusChangeListener() = default;
        virtual void OnAddSystemAbility(int32_t saId, const std::string &deviceId) override;
        virtual void OnRemoveSystemAbility(int32_t asId, const std::string &deviceId) override;
    };
};
} // namespace OHOS::Request
#endif // DOWNLOAD_MANAGER_H
