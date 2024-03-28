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

#include "data_ability_predicates.h"
#include "errors.h"
#include "log.h"
#include "rdb_errno.h"
#include "rdb_helper.h"
#include "rdb_open_callback.h"
#include "rdb_predicates.h"
#include "rdb_store.h"
#include "request_manager.h"
#include "request_running_task_count.h"
#include "request_sync_load_callback.h"
#include "response_message_receiver.h"
#include "result_set.h"
#include "runcount_notify_stub.h"
#include "system_ability_definition.h"

namespace OHOS::Request {
constexpr const int32_t RETRY_INTERVAL = 500 * 1000;
constexpr const int32_t RETRY_MAX_TIMES = 5;

const std::unique_ptr<RequestManagerImpl> &RequestManagerImpl::GetInstance()
{
    static std::unique_ptr<RequestManagerImpl> instance(new RequestManagerImpl());
    return instance;
}

int32_t RequestManagerImpl::Create(const Config &config, int32_t &tid)
{
    REQUEST_HILOGD("RequestManagerImpl Create start.");

    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        REQUEST_HILOGE("GetRequestServiceProxy fail.");
        return E_SERVICE_ERROR;
    }
    int32_t ret = proxy->Create(config, tid);
    if (ret == E_UNLOADING_SA) {
        REQUEST_HILOGE("Service ability is quitting");
        ret = Retry(tid, config, ret);
        if (ret != E_OK) {
            return ret;
        }
    }
    REQUEST_HILOGD("RequestManagerImpl Create end.");
    if (ret == E_OK) {
        this->Subscribe(std::to_string(tid));
    }

    return ret;
}

int32_t RequestManagerImpl::Retry(int32_t &taskId, const Config &config, int32_t errorCode)
{
    REQUEST_HILOGD("Retry in");
    int32_t interval = 1;
    while (errorCode == E_UNLOADING_SA && interval <= RETRY_MAX_TIMES) {
        if (config.action == Action::DOWNLOAD) {
            for (auto file : config.files) {
                std::remove(file.uri.c_str());
            }
        }

        if (errorCode == E_UNLOADING_SA) {
            // Waitting for system ability quit
            usleep(RETRY_INTERVAL);
        }
        SetRequestServiceProxy(nullptr);
        LoadRequestServer();
        auto proxy = GetRequestServiceProxy();
        if (proxy == nullptr) {
            REQUEST_HILOGE("proxy is nullptr!");
            continue;
        }
        errorCode = proxy->Create(config, taskId);
        ++interval;
    }
    if (errorCode != E_OK && config.action == Action::DOWNLOAD) {
        for (auto file : config.files) {
            std::remove(file.uri.c_str());
        }
    }
    return errorCode;
}

void RequestManagerImpl::SetRequestServiceProxy(sptr<RequestServiceInterface> proxy)
{
    std::lock_guard<std::mutex> lock(serviceProxyMutex_);
    requestServiceProxy_ = proxy;
}

int32_t RequestManagerImpl::GetTask(const std::string &tid, const std::string &token, Config &config)
{
    REQUEST_HILOGD("GetTask in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return E_SERVICE_ERROR;
    }

    return proxy->GetTask(tid, token, config);
}

int32_t RequestManagerImpl::Start(const std::string &tid)
{
    REQUEST_HILOGD("Start in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        if (!RequestManager::GetInstance()->LoadRequestServer()) {
            return E_SERVICE_ERROR;
        }
        proxy = GetRequestServiceProxy();
    }

    return proxy->Start(tid);
}

int32_t RequestManagerImpl::Stop(const std::string &tid)
{
    REQUEST_HILOGD("Stop in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return E_SERVICE_ERROR;
    }

    return proxy->Stop(tid);
}

int32_t RequestManagerImpl::Query(const std::string &tid, TaskInfo &info)
{
    REQUEST_HILOGD("Query in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return E_SERVICE_ERROR;
    }

    return proxy->Query(tid, info);
}

int32_t RequestManagerImpl::Touch(const std::string &tid, const std::string &token, TaskInfo &info)
{
    REQUEST_HILOGD("Touch in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return E_SERVICE_ERROR;
    }

    return proxy->Touch(tid, token, info);
}

int32_t RequestManagerImpl::Search(const Filter &filter, std::vector<std::string> &tids)
{
    REQUEST_HILOGD("Search in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return E_SERVICE_ERROR;
    }

    return proxy->Search(filter, tids);
}

int32_t RequestManagerImpl::Show(const std::string &tid, TaskInfo &info)
{
    REQUEST_HILOGD("Show in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return E_SERVICE_ERROR;
    }

    return proxy->Show(tid, info);
}

int32_t RequestManagerImpl::Pause(const std::string &tid, Version version)
{
    REQUEST_HILOGD("Pause in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return E_SERVICE_ERROR;
    }

    return proxy->Pause(tid, version);
}

int32_t RequestManagerImpl::QueryMimeType(const std::string &tid, std::string &mimeType)
{
    REQUEST_HILOGD("QueryMimeType in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return E_SERVICE_ERROR;
    }

    return proxy->QueryMimeType(tid, mimeType);
}

int32_t RequestManagerImpl::Remove(const std::string &tid, Version version)
{
    REQUEST_HILOGD("Remove in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return E_SERVICE_ERROR;
    }

    return proxy->Remove(tid, version);
}

int32_t RequestManagerImpl::Resume(const std::string &tid)
{
    REQUEST_HILOGD("Resume in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return E_SERVICE_ERROR;
    }

    return proxy->Resume(tid);
}

int32_t RequestManagerImpl::AddListener(
    const std::string &taskId, const SubscribeType &type, const std::shared_ptr<IResponseListener> &listener)
{
    REQUEST_HILOGD("AddListener in, tid:%{public}s, type: %{public}d", taskId.c_str(), type);
    std::shared_ptr<Request> task = this->GetTask(taskId);
    if (task) {
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
    if (task) {
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
    if (task) {
        task->AddListener(type, listener);
        return E_OK;
    } else {
        return E_OTHER;
    }
}

int32_t RequestManagerImpl::RemoveListener(
    const std::string &taskId, const SubscribeType &type, const std::shared_ptr<INotifyDataListener> &listener)
{
    REQUEST_HILOGD("RemoveListener in, tid:%{public}s, type: %{public}d", taskId.c_str(), type);
    std::shared_ptr<Request> task = this->GetTask(taskId);
    if (task) {
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
    REQUEST_HILOGD("Subscribe in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        REQUEST_HILOGE("GetRequestServiceProxy fail.");
        return E_SERVICE_ERROR;
    }
    this->EnsureChannelOpen();

    return proxy->Subscribe(taskId);
}

int32_t RequestManagerImpl::Unsubscribe(const std::string &taskId)
{
    REQUEST_HILOGD("Unsubscribe in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        REQUEST_HILOGE("GetRequestServiceProxy fail.");
        return E_SERVICE_ERROR;
    }
    return proxy->Unsubscribe(taskId);
}

int32_t RequestManagerImpl::SubRunCount(const sptr<NotifyInterface> &listener)
{
    REQUEST_HILOGD("Impl SubRunCount in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        REQUEST_HILOGE("Impl SubRunCount in, get request service proxy failed.");
        return E_SERVICE_ERROR;
    }

    return proxy->SubRunCount(listener);
}

int32_t RequestManagerImpl::UnsubRunCount()
{
    REQUEST_HILOGD("Impl UnsubRunCount in");
    auto proxy = GetRequestServiceProxy();
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

    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return false;
    }

    int32_t sockFd = -1;
    int32_t ret = proxy->OpenChannel(sockFd);
    if (ret != E_OK) {
        return ret;
    }
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
        return std::shared_ptr<Request>();
    }
}

void RequestManagerImpl::OnChannelBroken()
{
    std::lock_guard<std::recursive_mutex> lock(msgReceiverMutex_);
    this->msgReceiver_.reset();
}

void RequestManagerImpl::OnResponseReceive(const std::shared_ptr<Response> &response)
{
    std::shared_ptr<Request> task;
    {
        std::lock_guard<std::mutex> lock(tasksMutex_);
        auto it = tasks_.find(response->taskId);
        if (it == tasks_.end()) {
            REQUEST_HILOGD("OnResponseReceive task not found");
            return;
        }
        task = it->second;
    }
    task->OnResponseReceive(response);
}

void RequestManagerImpl::OnNotifyDataReceive(const std::shared_ptr<NotifyData> &notifyData)
{
    std::shared_ptr<Request> task;
    {
        std::lock_guard<std::mutex> lock(tasksMutex_);
        auto it = tasks_.find(std::to_string(notifyData->taskId));
        if (it == tasks_.end()) {
            REQUEST_HILOGD("OnNotifyDataReceive task not found");
            return;
        }
        task = it->second;
    }
    task->OnNotifyDataReceive(notifyData);
}

sptr<RequestServiceInterface> RequestManagerImpl::GetRequestServiceProxy()
{
    std::lock_guard<std::mutex> lock(serviceProxyMutex_);
    if (requestServiceProxy_ != nullptr) {
        return requestServiceProxy_;
    }
    sptr<ISystemAbilityManager> systemAbilityManager =
        SystemAbilityManagerClient::GetInstance().GetSystemAbilityManager();
    if (systemAbilityManager == nullptr) {
        REQUEST_HILOGE("Getting SystemAbilityManager failed.");
        return nullptr;
    }
    auto systemAbility = systemAbilityManager->GetSystemAbility(DOWNLOAD_SERVICE_ID, "");
    if (systemAbility == nullptr) {
        REQUEST_HILOGE("Get SystemAbility failed.");
        return nullptr;
    }
    if (!SubscribeSA(systemAbilityManager)) {
        REQUEST_HILOGE("Subscribe SystemAbility failed.");
        return nullptr;
    }
    deathRecipient_ = new RequestSaDeathRecipient();
    systemAbility->AddDeathRecipient(deathRecipient_);
    requestServiceProxy_ = iface_cast<RequestServiceInterface>(systemAbility);
    if (requestServiceProxy_ == nullptr) {
        REQUEST_HILOGE("Get requestServiceProxy_ fail.");
        return nullptr;
    }
    return requestServiceProxy_;
}

// Subscribe SA status changes only once
bool RequestManagerImpl::SubscribeSA(sptr<ISystemAbilityManager> systemAbilityManager)
{
    if (saChangeListener_ != nullptr) {
        return true;
    }
    saChangeListener_ = new (std::nothrow) SystemAbilityStatusChangeListener();
    if (saChangeListener_ == nullptr) {
        REQUEST_HILOGE("Get saChangeListener_ failed.");
        return false;
    }
    if (systemAbilityManager->SubscribeSystemAbility(DOWNLOAD_SERVICE_ID, saChangeListener_) != E_OK) {
        REQUEST_HILOGE("SubscribeSystemAbility failed.");
        return false;
    }
    return true;
}

void RequestManagerImpl::RestoreListener(void (*callback)())
{
    callback_ = callback;
}

void RequestManagerImpl::RestoreSubRunCount()
{
    REQUEST_HILOGD("Restore sub run count in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        REQUEST_HILOGE("Restore sub run count, but get request service proxy fail.");
        return;
    }

    auto listener = RunCountNotifyStub::GetInstance();
    int32_t ret = proxy->SubRunCount(listener);
    if (ret != E_OK) {
        REQUEST_HILOGE("Restore sub run count failed, ret: %{public}d.", ret);
    }
}

RequestManagerImpl::SystemAbilityStatusChangeListener::SystemAbilityStatusChangeListener()
{
}

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
    RequestManagerImpl::GetInstance()->RestoreSubRunCount();
}

void RequestManagerImpl::SystemAbilityStatusChangeListener::OnRemoveSystemAbility(
    int32_t saId, const std::string &deviceId)
{
    if (saId != DOWNLOAD_SERVICE_ID) {
        REQUEST_HILOGE("SA ID is not DOWNLOAD_SERVICE_ID.");
    }
    REQUEST_HILOGD("SystemAbility Remove.");
}

void RequestManagerImpl::OnRemoteSaDied(const wptr<IRemoteObject> &remote)
{
    REQUEST_HILOGD(" RequestManagerImpl::OnRemoteSaDied");
    ready_.store(false);
    SetRequestServiceProxy(nullptr);
    FwkRunningTaskCountManager::GetInstance()->SetCount(0);
    FwkRunningTaskCountManager::GetInstance()->NotifyAllObservers();
    std::lock_guard<std::recursive_mutex> lock(msgReceiverMutex_);
    if (!msgReceiver_) {
        return;
    }
    msgReceiver_->Shutdown();
}

RequestSaDeathRecipient::RequestSaDeathRecipient()
{
}

void RequestSaDeathRecipient::OnRemoteDied(const wptr<IRemoteObject> &object)
{
    REQUEST_HILOGI("RequestSaDeathRecipient on remote systemAbility died.");
    RequestManagerImpl::GetInstance()->OnRemoteSaDied(object);
}

bool RequestManagerImpl::LoadRequestServer()
{
    REQUEST_HILOGD("Begin load request server");
    if (ready_.load()) {
        REQUEST_HILOGD("GetSystemAbilityManager ready_ true");
        return true;
    }
    std::lock_guard<std::mutex> lock(downloadMutex_);
    if (ready_.load()) {
        REQUEST_HILOGD("GetSystemAbilityManager ready_ is true");
        return true;
    }

    auto sm = SystemAbilityManagerClient::GetInstance().GetSystemAbilityManager();
    if (sm == nullptr) {
        REQUEST_HILOGE("GetSystemAbilityManager return null");
        return false;
    }
    auto systemAbility = sm->CheckSystemAbility(DOWNLOAD_SERVICE_ID);
    if (systemAbility != nullptr) {
        REQUEST_HILOGI("service already exists");
        return true;
    }
    sptr<RequestSyncLoadCallback> loadCallback_ = new (std::nothrow) RequestSyncLoadCallback();
    if (loadCallback_ == nullptr) {
        REQUEST_HILOGE("new DownloadAbilityCallback fail");
        return false;
    }

    int32_t result = sm->LoadSystemAbility(DOWNLOAD_SERVICE_ID, loadCallback_);
    if (result != E_OK) {
        REQUEST_HILOGE("LoadSystemAbility %{public}d failed, result: %{public}d", DOWNLOAD_SERVICE_ID, result);
        return false;
    }

    {
        std::unique_lock<std::mutex> conditionLock(conditionMutex_);
        auto waitStatus = syncCon_.wait_for(
            conditionLock, std::chrono::milliseconds(LOAD_SA_TIMEOUT_MS), [this]() { return ready_.load(); });
        if (!waitStatus) {
            REQUEST_HILOGE("download server load sa timeout");
            return false;
        }
    }
    return true;
}

bool RequestManagerImpl::IsSaReady()
{
    return ready_.load();
}

void RequestManagerImpl::LoadServerSuccess()
{
    std::unique_lock<std::mutex> lock(conditionMutex_);
    ready_.store(true);
    syncCon_.notify_one();
    REQUEST_HILOGI("load download server success");
}

void RequestManagerImpl::LoadServerFail()
{
    ready_.store(false);
    REQUEST_HILOGE("load download server fail");
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

} // namespace OHOS::Request
