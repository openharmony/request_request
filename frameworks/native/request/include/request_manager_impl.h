/*
 * Copyright (C) 2024 Huawei Device Co., Ltd.
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

#ifndef OHOS_REQUEST_DOWNLOAD_MANAGER_IMPL_H
#define OHOS_REQUEST_DOWNLOAD_MANAGER_IMPL_H

#include <atomic>
#include <condition_variable>
#include <map>
#include <mutex>

#include "constant.h"
#include "i_notify_data_listener.h"
#include "i_response_message_handler.h"
#include "iremote_object.h"
#include "iservice_registry.h"
#include "js_common.h"
#include "refbase.h"
#include "request.h"
#include "request_service_interface.h"
#include "response_message_receiver.h"
#include "system_ability_status_change_stub.h"
#include "visibility.h"

namespace OHOS::Request {
class RequestSaDeathRecipient : public IRemoteObject::DeathRecipient {
public:
    explicit RequestSaDeathRecipient();
    ~RequestSaDeathRecipient() = default;
    void OnRemoteDied(const wptr<IRemoteObject> &object) override;
};

class RequestManagerImpl : public IResponseMessageHandler {
public:
    static const std::unique_ptr<RequestManagerImpl> &GetInstance();
    int32_t Create(const Config &config, int32_t seq, std::string &tid);
    int32_t GetTask(const std::string &tid, const std::string &token, Config &config);
    int32_t Start(const std::string &tid);
    int32_t Stop(const std::string &tid);
    int32_t Query(const std::string &tid, TaskInfo &info);
    int32_t Touch(const std::string &tid, const std::string &token, TaskInfo &info);
    int32_t Search(const Filter &filter, std::vector<std::string> &tids);
    int32_t Show(const std::string &tid, TaskInfo &info);
    int32_t Pause(const std::string &tid, Version version);
    int32_t QueryMimeType(const std::string &tid, std::string &mimeType);
    int32_t Remove(const std::string &tid, Version version);
    int32_t Resume(const std::string &tid);

    int32_t Subscribe(const std::string &taskId);
    int32_t Unsubscribe(const std::string &taskId);

    int32_t AddListener(
        const std::string &taskId, const SubscribeType &type, const std::shared_ptr<IResponseListener> &listener);
    int32_t RemoveListener(
        const std::string &taskId, const SubscribeType &type, const std::shared_ptr<IResponseListener> &listener);
    int32_t AddListener(
        const std::string &taskId, const SubscribeType &type, const std::shared_ptr<INotifyDataListener> &listener);
    int32_t RemoveListener(
        const std::string &taskId, const SubscribeType &type, const std::shared_ptr<INotifyDataListener> &listener);
    void RemoveAllListeners(const std::string &taskId);

    int32_t SubRunCount(const sptr<NotifyInterface> &listener);
    int32_t UnsubRunCount();

    void RestoreListener(void (*callback)());
    void RestoreSubRunCount();
    bool LoadRequestServer();
    bool IsSaReady();
    void OnRemoteSaDied(const wptr<IRemoteObject> &object);
    void LoadServerSuccess();
    void LoadServerFail();
    void ReopenChannel();
    int32_t GetNextSeq();
    bool SubscribeSA();
    bool UnsubscribeSA();

private:
    RequestManagerImpl() = default;
    RequestManagerImpl(const RequestManagerImpl &) = delete;
    RequestManagerImpl(RequestManagerImpl &&) = delete;
    RequestManagerImpl &operator=(const RequestManagerImpl &) = delete;
    sptr<RequestServiceInterface> GetRequestServiceProxy();
    int32_t Retry(std::string &taskId, const Config &config, int32_t errorCode);
    void SetRequestServiceProxy(sptr<RequestServiceInterface> proxy);
    int32_t EnsureChannelOpen();
    std::shared_ptr<Request> GetTask(const std::string &taskId);
    void OnChannelBroken() override;
    void OnResponseReceive(const std::shared_ptr<Response> &response) override;
    void OnNotifyDataReceive(const std::shared_ptr<NotifyData> &notifyData) override;

private:
    static std::mutex instanceLock_;
    static sptr<RequestManagerImpl> instance_;
    std::mutex downloadMutex_;
    std::mutex conditionMutex_;
    std::mutex serviceProxyMutex_;
    std::mutex saChangeListenerMutex_;

    sptr<RequestServiceInterface> requestServiceProxy_;
    sptr<RequestSaDeathRecipient> deathRecipient_;
    sptr<ISystemAbilityStatusChange> saChangeListener_;
    std::condition_variable syncCon_;
    std::atomic<bool> ready_ = false;
    static constexpr int LOAD_SA_TIMEOUT_MS = 15000;
    void (*callback_)() = nullptr;
    std::mutex tasksMutex_;
    std::map<std::string, std::shared_ptr<Request>> tasks_;
    std::recursive_mutex msgReceiverMutex_;
    std::shared_ptr<ResponseMessageReceiver> msgReceiver_;

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
#endif // OHOS_REQUEST_DOWNLOAD_MANAGER_IMPL_H
