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
#include <vector>

#include "constant.h"
#include "i_notify_data_listener.h"
#include "i_response_message_handler.h"
#include "iremote_object.h"
#include "iservice_registry.h"
#include "log.h"
#include "refbase.h"
#include "request.h"
#include "request_common.h"
#include "request_service_interface.h"
#include "response_message_receiver.h"
#include "system_ability_status_change_stub.h"
#include "visibility.h"

namespace OHOS::Request {

constexpr int RETRY_TIMES = 5;

class RequestManagerImpl : public IResponseMessageHandler {
public:
    static const std::unique_ptr<RequestManagerImpl> &GetInstance();
    ExceptionErrorCode CreateTasks(const std::vector<Config> &configs, std::vector<TaskRet> &rets);
    ExceptionErrorCode StartTasks(const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets);
    ExceptionErrorCode StopTasks(const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets);
    ExceptionErrorCode ResumeTasks(const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets);
    ExceptionErrorCode RemoveTasks(
        const std::vector<std::string> &tids, const Version version, std::vector<ExceptionErrorCode> &rets);
    ExceptionErrorCode PauseTasks(
        const std::vector<std::string> &tids, const Version version, std::vector<ExceptionErrorCode> &rets);
    ExceptionErrorCode QueryTasks(const std::vector<std::string> &tids, std::vector<TaskInfoRet> &rets);
    ExceptionErrorCode ShowTasks(const std::vector<std::string> &tids, std::vector<TaskInfoRet> &rets);
    ExceptionErrorCode TouchTasks(const std::vector<TaskIdAndToken> &tids, std::vector<TaskInfoRet> &rets);
    ExceptionErrorCode SetMaxSpeeds(const std::vector<SpeedConfig> &speedConfig, std::vector<ExceptionErrorCode> &rets);
    ExceptionErrorCode SetMode(const std::string &tid, const Mode mode);
    ExceptionErrorCode DisableTaskNotification(
        const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets);

    int32_t Create(const Config &config, int32_t seq, std::string &tid);
    int32_t GetTask(const std::string &tid, const std::string &token, Config &config);
    int32_t Start(const std::string &tid);
    int32_t Stop(const std::string &tid);
    int32_t Query(const std::string &tid, TaskInfo &info);
    int32_t Touch(const std::string &tid, const std::string &token, TaskInfo &info);
    int32_t Search(const Filter &filter, std::vector<std::string> &tids);
    int32_t Show(const std::string &tid, TaskInfo &info);
    int32_t Pause(const std::string &tid, const Version version);
    int32_t QueryMimeType(const std::string &tid, std::string &mimeType);
    int32_t Remove(const std::string &tid, const Version version);
    int32_t Resume(const std::string &tid);
    int32_t SetMaxSpeed(const std::string &tid, const int64_t maxSpeed);

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
    void LoadRequestServer();
    bool IsSaReady();
    void ReopenChannel();
    int32_t GetNextSeq();
    bool SubscribeSA();
    bool UnsubscribeSA();
    int32_t CreateGroup(
        std::string &gid, const bool gauge, Notification &notification);
    int32_t AttachGroup(const std::string &gid, const std::vector<std::string> &tids);
    int32_t DeleteGroup(const std::string &gid);

private:
    RequestManagerImpl() = default;
    RequestManagerImpl(const RequestManagerImpl &) = delete;
    RequestManagerImpl(RequestManagerImpl &&) = delete;
    RequestManagerImpl &operator=(const RequestManagerImpl &) = delete;
    sptr<RequestServiceInterface> GetRequestServiceProxy(bool load);
    int32_t EnsureChannelOpen();
    std::shared_ptr<Request> GetTask(const std::string &taskId);
    void OnChannelBroken() override;
    void OnResponseReceive(const std::shared_ptr<Response> &response) override;
    void OnNotifyDataReceive(const std::shared_ptr<NotifyData> &notifyData) override;
    void OnFaultsReceive(const std::shared_ptr<int32_t> &tid, const std::shared_ptr<SubscribeType> &type,
        const std::shared_ptr<Reason> &reason) override;
    void OnWaitReceive(std::int32_t taskId, WaitingReason reason) override;

private:
    std::mutex serviceProxyMutex_;
    std::mutex saChangeListenerMutex_;

    sptr<RequestServiceInterface> requestServiceProxy_;
    sptr<ISystemAbilityStatusChange> saChangeListener_;
    static constexpr int LOAD_SA_TIMEOUT_MS = 15000;
    void (*callback_)() = nullptr;
    std::mutex tasksMutex_;
    std::map<std::string, std::shared_ptr<Request>> tasks_;
    std::recursive_mutex msgReceiverMutex_;
    std::shared_ptr<ResponseMessageReceiver> msgReceiver_;

    class SystemAbilityStatusChangeListener : public OHOS::SystemAbilityStatusChangeStub {
    public:
        SystemAbilityStatusChangeListener();
        ~SystemAbilityStatusChangeListener() = default;
        virtual void OnAddSystemAbility(int32_t saId, const std::string &deviceId) override;
        virtual void OnRemoveSystemAbility(int32_t asId, const std::string &deviceId) override;
    };

    template<typename ProxyMethod, typename... Args> int32_t CallProxyMethod(ProxyMethod method, Args &&...args)
    {
        int32_t ret = E_SERVICE_ERROR;
        for (int i = 0; i < RETRY_TIMES; i++) {
            auto proxy = this->GetRequestServiceProxy(true);
            if (proxy == nullptr) {
                REQUEST_HILOGE("Get service proxy failed");
                continue;
            }
            ret = (proxy->*method)(args...);
            if (ret == E_SERVICE_ERROR) {
                REQUEST_HILOGE("Remote died, retry times: %{public}d", i);
                {
                    std::lock_guard<std::mutex> lock(serviceProxyMutex_);
                    requestServiceProxy_ = nullptr;
                }
                continue;
            }
            break;
        }
        return ret;
    }
};

} // namespace OHOS::Request
#endif // OHOS_REQUEST_DOWNLOAD_MANAGER_IMPL_H
