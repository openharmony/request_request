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

#include "request_manager_impl.h"

#include <atomic>
#include <cstdint>
#include <memory>
#include <vector>

#include "data_ability_predicates.h"
#include "download_server_ipc_interface_code.h"
#include "errors.h"
#include "log.h"
#include "rdb_errno.h"
#include "rdb_helper.h"
#include "rdb_open_callback.h"
#include "rdb_predicates.h"
#include "rdb_store.h"
#include "request_common.h"
#include "request_manager.h"
#include "request_running_task_count.h"
#include "request_service_interface.h"
#include "response_message_receiver.h"
#include "result_set.h"
#include "runcount_notify_stub.h"
#include "sys_event.h"
#include "system_ability_definition.h"

namespace OHOS::Request {

const std::unique_ptr<RequestManagerImpl> &RequestManagerImpl::GetInstance()
{
    static std::unique_ptr<RequestManagerImpl> instance(new RequestManagerImpl());
    return instance;
}

ExceptionErrorCode RequestManagerImpl::SetMode(const std::string &tid, const Mode mode)
{
    return static_cast<ExceptionErrorCode>(CallProxyMethod(&RequestServiceInterface::SetMode, tid, mode));
}

ExceptionErrorCode RequestManagerImpl::DisableTaskNotification(
    const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets)
{
    return static_cast<ExceptionErrorCode>(
        CallProxyMethod(&RequestServiceInterface::DisableTaskNotification, tids, rets));
}

ExceptionErrorCode RequestManagerImpl::CreateTasks(const std::vector<Config> &configs, std::vector<TaskRet> &rets)
{
    if (configs.size() == 0) {
        return ExceptionErrorCode::E_OK;
    }
    this->EnsureChannelOpen();
    int ret = CallProxyMethod(&RequestServiceInterface::CreateTasks, configs, rets);
    if (ret == E_OK) {
        bool channelOpened = false;
        for (auto taskRet : rets) {
            if (taskRet.code != E_CHANNEL_NOT_OPEN) {
                continue;
            }
            if (!channelOpened) {
                this->ReopenChannel();
                channelOpened = true;
            }
            taskRet.code =
                static_cast<ExceptionErrorCode>(CallProxyMethod(&RequestServiceInterface::Subscribe, taskRet.tid));
        }
    }
    return static_cast<ExceptionErrorCode>(ret);
}

ExceptionErrorCode RequestManagerImpl::StartTasks(
    const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets)
{
    return static_cast<ExceptionErrorCode>(CallProxyMethod(&RequestServiceInterface::StartTasks, tids, rets));
}

ExceptionErrorCode RequestManagerImpl::StopTasks(
    const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets)
{
    return static_cast<ExceptionErrorCode>(CallProxyMethod(&RequestServiceInterface::StopTasks, tids, rets));
}

ExceptionErrorCode RequestManagerImpl::ResumeTasks(
    const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets)
{
    return static_cast<ExceptionErrorCode>(CallProxyMethod(&RequestServiceInterface::ResumeTasks, tids, rets));
}

ExceptionErrorCode RequestManagerImpl::RemoveTasks(
    const std::vector<std::string> &tids, const Version version, std::vector<ExceptionErrorCode> &rets)
{
    return static_cast<ExceptionErrorCode>(CallProxyMethod(&RequestServiceInterface::RemoveTasks, tids, version, rets));
}

ExceptionErrorCode RequestManagerImpl::PauseTasks(
    const std::vector<std::string> &tids, const Version version, std::vector<ExceptionErrorCode> &rets)
{
    return static_cast<ExceptionErrorCode>(CallProxyMethod(&RequestServiceInterface::PauseTasks, tids, version, rets));
}

ExceptionErrorCode RequestManagerImpl::QueryTasks(const std::vector<std::string> &tids, std::vector<TaskInfoRet> &rets)
{
    return static_cast<ExceptionErrorCode>(CallProxyMethod(&RequestServiceInterface::QueryTasks, tids, rets));
}

ExceptionErrorCode RequestManagerImpl::ShowTasks(const std::vector<std::string> &tids, std::vector<TaskInfoRet> &rets)
{
    return static_cast<ExceptionErrorCode>(CallProxyMethod(&RequestServiceInterface::ShowTasks, tids, rets));
}

ExceptionErrorCode RequestManagerImpl::TouchTasks(
    const std::vector<TaskIdAndToken> &tidTokens, std::vector<TaskInfoRet> &rets)
{
    return static_cast<ExceptionErrorCode>(CallProxyMethod(&RequestServiceInterface::TouchTasks, tidTokens, rets));
}

ExceptionErrorCode RequestManagerImpl::SetMaxSpeeds(
    const std::vector<SpeedConfig> &speedConfig, std::vector<ExceptionErrorCode> &rets)
{
    return static_cast<ExceptionErrorCode>(CallProxyMethod(&RequestServiceInterface::SetMaxSpeeds, speedConfig, rets));
}

int32_t RequestManagerImpl::Create(const Config &config, int32_t seq, std::string &tid)
{
    this->EnsureChannelOpen();

    int ret = CallProxyMethod(&RequestServiceInterface::Create, config, tid);
    if (ret == E_CHANNEL_NOT_OPEN) {
        this->ReopenChannel();
        ret = CallProxyMethod(&RequestServiceInterface::Subscribe, tid);
    }
    if (ret == E_OK && config.version != Version::API10) {
        ret = CallProxyMethod(&RequestServiceInterface::Start, tid);
    }
    if (ret != E_OK) {
        REQUEST_HILOGE("Request create, seq: %{public}d, failed: %{public}d", seq, ret);
    }
    for (auto &file : config.files) {
        if (file.isUserFile) {
            if (file.fd > 0) {
                fdsan_close_with_tag(file.fd, REQUEST_FDSAN_TAG);
            }
        }
    }
    return ret;
}

int32_t RequestManagerImpl::GetTask(const std::string &tid, const std::string &token, Config &config)
{
    REQUEST_HILOGD("GetTask in");
    this->EnsureChannelOpen();
    int32_t ret = CallProxyMethod(&RequestServiceInterface::GetTask, tid, token, config);
    if (ret == E_CHANNEL_NOT_OPEN) {
        this->ReopenChannel();
        ret = CallProxyMethod(&RequestServiceInterface::Subscribe, tid);
    }
    if (ret != E_OK) {
        REQUEST_HILOGE("Request getTask, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
    }

    return ret;
}

int32_t RequestManagerImpl::Start(const std::string &tid)
{
    return CallProxyMethod(&RequestServiceInterface::Start, tid);
}

int32_t RequestManagerImpl::Stop(const std::string &tid)
{
    return CallProxyMethod(&RequestServiceInterface::Stop, tid);
}

int32_t RequestManagerImpl::Query(const std::string &tid, TaskInfo &info)
{
    return CallProxyMethod(&RequestServiceInterface::Query, tid, info);
}

int32_t RequestManagerImpl::Touch(const std::string &tid, const std::string &token, TaskInfo &info)
{
    return CallProxyMethod(&RequestServiceInterface::Touch, tid, token, info);
}

int32_t RequestManagerImpl::Search(const Filter &filter, std::vector<std::string> &tids)
{
    return CallProxyMethod(&RequestServiceInterface::Search, filter, tids);
}

int32_t RequestManagerImpl::Show(const std::string &tid, TaskInfo &info)
{
    return CallProxyMethod(&RequestServiceInterface::Show, tid, info);
}

int32_t RequestManagerImpl::Pause(const std::string &tid, const Version version)
{
    return CallProxyMethod(&RequestServiceInterface::Pause, tid, version);
}

int32_t RequestManagerImpl::QueryMimeType(const std::string &tid, std::string &mimeType)
{
    return CallProxyMethod(&RequestServiceInterface::QueryMimeType, tid, mimeType);
}

int32_t RequestManagerImpl::Remove(const std::string &tid, const Version version)
{
    auto proxy = this->GetRequestServiceProxy(false);
    if (proxy == nullptr) {
        REQUEST_HILOGE("Get service proxy failed");
        return E_SERVICE_ERROR;
    }
    return proxy->Remove(tid, version);
}

int32_t RequestManagerImpl::Resume(const std::string &tid)
{
    return CallProxyMethod(&RequestServiceInterface::Resume, tid);
}

int32_t RequestManagerImpl::CreateGroup(std::string &gid, const bool gauge, Notification &notification)
{
    return CallProxyMethod(&RequestServiceInterface::CreateGroup, gid, gauge, notification);
}
int32_t RequestManagerImpl::AttachGroup(const std::string &gid, const std::vector<std::string> &tids)
{
    return CallProxyMethod(&RequestServiceInterface::AttachGroup, gid, tids);
}
int32_t RequestManagerImpl::DeleteGroup(const std::string &gid)
{
    return CallProxyMethod(&RequestServiceInterface::DeleteGroup, gid);
}

int32_t RequestManagerImpl::SetMaxSpeed(const std::string &tid, const int64_t maxSpeed)
{
    return CallProxyMethod(&RequestServiceInterface::SetMaxSpeed, tid, maxSpeed);
}

int32_t RequestManagerImpl::AddListener(
    const std::string &taskId, const SubscribeType &type, const std::shared_ptr<IResponseListener> &listener)
{
    REQUEST_HILOGD("AddListener in, tid:%{public}s, type: %{public}d", taskId.c_str(), type);
    std::shared_ptr<Request> task = this->GetTask(taskId);
    if (task.get()) {
        task->AddListener(type, listener);
        return E_OK;
    } else {
        return E_OTHER;
    }
}

int32_t RequestManagerImpl::RemoveListener(
    const std::string &taskId, const SubscribeType &type, const std::shared_ptr<IResponseListener> &listener)
{
    REQUEST_HILOGD("RemoveListener in, tid:%{public}s, type: %{public}d", taskId.c_str(), type);
    std::shared_ptr<Request> task = this->GetTask(taskId);
    if (task.get()) {
        task->RemoveListener(type, listener);
        return E_OK;
    } else {
        return E_OTHER;
    }
}

int32_t RequestManagerImpl::AddListener(
    const std::string &taskId, const SubscribeType &type, const std::shared_ptr<INotifyDataListener> &listener)
{
    REQUEST_HILOGD("AddListener in, tid:%{public}s, type: %{public}d", taskId.c_str(), type);
    std::shared_ptr<Request> task = this->GetTask(taskId);
    if (task.get()) {
        task->AddListener(type, listener);
        return E_OK;
    } else {
        REQUEST_HILOGE("GetTask Failed");
        return E_OTHER;
    }
}

int32_t RequestManagerImpl::RemoveListener(
    const std::string &taskId, const SubscribeType &type, const std::shared_ptr<INotifyDataListener> &listener)
{
    REQUEST_HILOGD("RemoveListener in, tid:%{public}s, type: %{public}d", taskId.c_str(), type);
    std::shared_ptr<Request> task = this->GetTask(taskId);
    if (task.get()) {
        task->RemoveListener(type, listener);
        return E_OK;
    } else {
        return E_OTHER;
    }
}

void RequestManagerImpl::RemoveAllListeners(const std::string &taskId)
{
    REQUEST_HILOGD("RemoveAllListeners in, tid:%{public}s", taskId.c_str());
    std::lock_guard<std::mutex> lock(tasksMutex_);
    tasks_.erase(taskId);
}

int32_t RequestManagerImpl::Subscribe(const std::string &taskId)
{
    this->EnsureChannelOpen();
    int ret = CallProxyMethod(&RequestServiceInterface::Subscribe, taskId);
    if (ret == E_CHANNEL_NOT_OPEN) {
        this->ReopenChannel();
        ret = CallProxyMethod(&RequestServiceInterface::Subscribe, taskId);
    }
    return ret;
}

int32_t RequestManagerImpl::Unsubscribe(const std::string &taskId)
{
    return CallProxyMethod(&RequestServiceInterface::Unsubscribe, taskId);
}

int32_t RequestManagerImpl::SubRunCount(const sptr<NotifyInterface> &listener)
{
    REQUEST_HILOGD("Impl SubRunCount in");
    auto proxy = GetRequestServiceProxy(false);
    if (proxy == nullptr) {
        REQUEST_HILOGE("Impl SubRunCount in, get request service proxy failed.");
        FwkRunningTaskCountManager::GetInstance()->SetSaStatus(false);
        // Proxy does not affect sub runcount at framework.
        return E_OK;
    }
    return proxy->SubRunCount(listener);
}

int32_t RequestManagerImpl::UnsubRunCount()
{
    REQUEST_HILOGD("Impl UnsubRunCount in");
    auto proxy = GetRequestServiceProxy(false);
    if (proxy == nullptr) {
        REQUEST_HILOGE("GetRequestServiceProxy fail.");
        return E_SERVICE_ERROR;
    }
    return proxy->UnsubRunCount();
}

int32_t RequestManagerImpl::EnsureChannelOpen()
{
    std::lock_guard<std::recursive_mutex> lock(msgReceiverMutex_);
    if (msgReceiver_) {
        return E_OK;
    }

    int32_t sockFd = -1;
    int ret = CallProxyMethod(&RequestServiceInterface::OpenChannel, sockFd);
    if (ret != E_OK) {
        REQUEST_HILOGE("EnsureChannelOpen failed: %{public}d, %{public}d", ret, sockFd);
        return ret;
    }
    if (sockFd == -1) {
        REQUEST_HILOGE("EnsureChannelOpen but fd -1: %{public}d", sockFd);
        return ret;
    }
    fdsan_exchange_owner_tag(sockFd, 0, REQUEST_FDSAN_TAG);
    REQUEST_HILOGD("EnsureChannelOpen ok: %{public}d", sockFd);
    msgReceiver_ = std::make_shared<ResponseMessageReceiver>(this, sockFd);
    msgReceiver_->BeginReceive();
    return E_OK;
}

std::shared_ptr<Request> RequestManagerImpl::GetTask(const std::string &taskId)
{
    std::lock_guard<std::mutex> lock(tasksMutex_);
    auto it = tasks_.find(taskId);
    if (it != tasks_.end()) {
        return it->second;
    }

    auto retPair = this->tasks_.emplace(taskId, std::make_shared<Request>(taskId));
    if (retPair.second) {
        return retPair.first->second;
    } else {
        this->tasks_.erase(taskId);
        REQUEST_HILOGE("Response Task create fail");
        return std::shared_ptr<Request>(nullptr);
    }
}

void RequestManagerImpl::OnChannelBroken()
{
    std::lock_guard<std::recursive_mutex> lock(msgReceiverMutex_);
    this->msgReceiver_.reset();
}

void RequestManagerImpl::OnResponseReceive(const std::shared_ptr<Response> &response)
{
    std::shared_ptr<Request> task = this->GetTask(response->taskId);
    if (task.get() == nullptr) {
        REQUEST_HILOGE("OnResponseReceive task not found");
        return;
    }
    task->OnResponseReceive(response);
}

void RequestManagerImpl::OnNotifyDataReceive(const std::shared_ptr<NotifyData> &notifyData)
{
    std::shared_ptr<Request> task = this->GetTask(std::to_string(notifyData->taskId));
    if (task.get() == nullptr) {
        REQUEST_HILOGE("OnNotifyDataReceive task not found");
        return;
    }
    task->OnNotifyDataReceive(notifyData);
}

void RequestManagerImpl::OnFaultsReceive(const std::shared_ptr<int32_t> &tid,
    const std::shared_ptr<SubscribeType> &type, const std::shared_ptr<Reason> &reason)
{
    std::shared_ptr<Request> task = this->GetTask(std::to_string(*tid));
    if (task.get() == nullptr) {
        REQUEST_HILOGE("OnFaultsReceive task not found");
        return;
    }
    task->OnFaultsReceive(tid, type, reason);
}

void RequestManagerImpl::OnWaitReceive(std::int32_t taskId, WaitingReason reason)
{
    std::shared_ptr<Request> task = this->GetTask(std::to_string(taskId));
    if (task.get() == nullptr) {
        REQUEST_HILOGE("OnWaitReceive task not found");
        return;
    }
    task->OnWaitReceive(taskId, reason);
}

sptr<RequestServiceInterface> RequestManagerImpl::GetRequestServiceProxy(bool needLoadSA)
{
    std::lock_guard<std::mutex> lock(serviceProxyMutex_);
    // When SubRuncount/UnSubRuncount/RestoreSubRunCount/Remove need to get proxy but not need to load
    if (!needLoadSA) {
        sptr<ISystemAbilityManager> systemAbilityManager =
            SystemAbilityManagerClient::GetInstance().GetSystemAbilityManager();
        if (systemAbilityManager == nullptr) {
            REQUEST_HILOGE("Getting SystemAbilityManager failed.");
            SysEventLog::SendSysEventLog(FAULT_EVENT, SAMGR_FAULT_00, "Get SAM failed");
            return nullptr;
        }
        // Update the proxy to avoid holding an expired object
        auto systemAbility = systemAbilityManager->GetSystemAbility(DOWNLOAD_SERVICE_ID, "");
        if (systemAbility != nullptr) {
            requestServiceProxy_ = iface_cast<RequestServiceInterface>(systemAbility);
        } else {
            REQUEST_HILOGI("Get SystemAbility failed.");
        }
        return requestServiceProxy_;
    }
    if (requestServiceProxy_ != nullptr) {
        return requestServiceProxy_;
    }

    REQUEST_HILOGI("Load System Ability");
    sptr<ISystemAbilityManager> systemAbilityManager =
        SystemAbilityManagerClient::GetInstance().GetSystemAbilityManager();
    if (systemAbilityManager == nullptr) {
        REQUEST_HILOGE("Getting SystemAbilityManager failed.");
        SysEventLog::SendSysEventLog(FAULT_EVENT, SAMGR_FAULT_00, "Get SAM failed");
        return nullptr;
    }
    auto systemAbility = systemAbilityManager->LoadSystemAbility(DOWNLOAD_SERVICE_ID, LOAD_SA_TIMEOUT_MS);
    if (systemAbility == nullptr) {
        REQUEST_HILOGE("Load SystemAbility failed.");
        SysEventLog::SendSysEventLog(FAULT_EVENT, SAMGR_FAULT_01, "Load SA failed");
        return nullptr;
    }
    requestServiceProxy_ = iface_cast<RequestServiceInterface>(systemAbility);

    return requestServiceProxy_;
}

bool RequestManagerImpl::SubscribeSA()
{
    std::lock_guard<std::mutex> lock(saChangeListenerMutex_);
    if (saChangeListener_ != nullptr) {
        return true;
    }
    sptr<ISystemAbilityManager> systemAbilityManager =
        SystemAbilityManagerClient::GetInstance().GetSystemAbilityManager();
    if (systemAbilityManager == nullptr) {
        REQUEST_HILOGE("Getting SystemAbilityManager failed.");
        SysEventLog::SendSysEventLog(FAULT_EVENT, SAMGR_FAULT_00, "Get SAM failed");
        return false;
    }
    saChangeListener_ = new (std::nothrow) SystemAbilityStatusChangeListener();
    if (saChangeListener_ == nullptr) {
        REQUEST_HILOGE("Get saChangeListener_ failed.");
        return false;
    }
    if (systemAbilityManager->SubscribeSystemAbility(DOWNLOAD_SERVICE_ID, saChangeListener_) != E_OK) {
        REQUEST_HILOGE("SubscribeSystemAbility failed.");
        SysEventLog::SendSysEventLog(FAULT_EVENT, SAMGR_FAULT_03, "Subscribe SA failed");
        return false;
    }
    REQUEST_HILOGI("SubscribeSA Success");
    return true;
}

bool RequestManagerImpl::UnsubscribeSA()
{
    std::lock_guard<std::mutex> lock(saChangeListenerMutex_);
    if (saChangeListener_ == nullptr) {
        return true;
    }
    sptr<ISystemAbilityManager> systemAbilityManager =
        SystemAbilityManagerClient::GetInstance().GetSystemAbilityManager();
    if (systemAbilityManager == nullptr) {
        REQUEST_HILOGE("Getting SystemAbilityManager failed.");
        SysEventLog::SendSysEventLog(FAULT_EVENT, SAMGR_FAULT_00, "Get SAM failed");
        return false;
    }
    if (systemAbilityManager->UnSubscribeSystemAbility(DOWNLOAD_SERVICE_ID, saChangeListener_) != E_OK) {
        SysEventLog::SendSysEventLog(FAULT_EVENT, SAMGR_FAULT_04, "UnSubscribe SA failed");
        REQUEST_HILOGE("UnsubscribeSystemAbility failed.");
        return false;
    }
    saChangeListener_ = nullptr;

    // Not sure about the SA state when it was supposed to set requestServiceProxy_ to nullptr, but there
    // was already retry logic in place when the message was sent, and also,
    // SA accesses a low-memory framework and doesn't hang frequently.

    REQUEST_HILOGI("UnsubscribeSA Success");
    return true;
}

void RequestManagerImpl::RestoreListener(void (*callback)())
{
    callback_ = callback;
}

void RequestManagerImpl::RestoreSubRunCount()
{
    REQUEST_HILOGD("Restore sub run count in");
    auto proxy = GetRequestServiceProxy(false);
    if (proxy == nullptr) {
        REQUEST_HILOGE("Restore sub run count, but get request service proxy fail.");
        return;
    }
    auto listener = RunCountNotifyStub::GetInstance();
    int ret = CallProxyMethod(&RequestServiceInterface::SubRunCount, listener);
    if (ret != E_OK) {
        REQUEST_HILOGE("Restore sub run count failed, ret: %{public}d.", ret);
    }
}

RequestManagerImpl::SystemAbilityStatusChangeListener::SystemAbilityStatusChangeListener()
{
}

// Sometimes too slow to return.
void RequestManagerImpl::SystemAbilityStatusChangeListener::OnAddSystemAbility(
    int32_t saId, const std::string &deviceId)
{
    if (saId != DOWNLOAD_SERVICE_ID) {
        REQUEST_HILOGE("SA ID is not DOWNLOAD_SERVICE_ID.");
    }
    REQUEST_HILOGD("SystemAbility Add.");
    if (RequestManagerImpl::GetInstance()->callback_ != nullptr) {
        RequestManagerImpl::GetInstance()->callback_();
    }
    if (FwkRunningTaskCountManager::GetInstance()->HasObserver()) {
        RequestManagerImpl::GetInstance()->RestoreSubRunCount();
    }
}

void RequestManagerImpl::SystemAbilityStatusChangeListener::OnRemoveSystemAbility(
    int32_t saId, const std::string &deviceId)
{
    REQUEST_HILOGI("SystemAbility Unloaded");
    if (saId != DOWNLOAD_SERVICE_ID) {
        REQUEST_HILOGE("SA ID is not DOWNLOAD_SERVICE_ID.");
    }
    {
        std::lock_guard<std::mutex> locks(RequestManagerImpl::GetInstance()->serviceProxyMutex_);
        RequestManagerImpl::GetInstance()->requestServiceProxy_ = nullptr;
    }
    FwkRunningTaskCountManager::GetInstance()->SetCount(0);
    FwkRunningTaskCountManager::GetInstance()->SetSaStatus(false);
    FwkRunningTaskCountManager::GetInstance()->NotifyAllObservers();
    std::lock_guard<std::recursive_mutex> lock(RequestManagerImpl::GetInstance()->msgReceiverMutex_);
    if (!RequestManagerImpl::GetInstance()->msgReceiver_) {
        return;
    }
    RequestManagerImpl::GetInstance()->msgReceiver_->Shutdown();
}

void RequestManagerImpl::LoadRequestServer()
{
    this->GetRequestServiceProxy(true);
}

bool RequestManagerImpl::IsSaReady()
{
    sptr<ISystemAbilityManager> systemAbilityManager =
        SystemAbilityManagerClient::GetInstance().GetSystemAbilityManager();
    if (systemAbilityManager == nullptr) {
        REQUEST_HILOGE("Getting SystemAbilityManager failed.");
        SysEventLog::SendSysEventLog(FAULT_EVENT, SAMGR_FAULT_00, "Get SAM failed");
        return false;
    }
    return systemAbilityManager->CheckSystemAbility(DOWNLOAD_SERVICE_ID) != nullptr;
}

void RequestManagerImpl::ReopenChannel()
{
    std::lock_guard<std::recursive_mutex> lock(msgReceiverMutex_);
    if (!msgReceiver_) {
        return;
    }
    msgReceiver_->Shutdown();
    this->EnsureChannelOpen();
}

int32_t RequestManagerImpl::GetNextSeq()
{
    static std::atomic<int32_t> seq{ 0 };
    return seq.fetch_add(1);
}

} // namespace OHOS::Request
