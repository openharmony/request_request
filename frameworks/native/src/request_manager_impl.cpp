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
#include "request_sync_load_callback.h"
#include "response_message_receiver.h"
#include "result_set.h"
#include "system_ability_definition.h"

namespace OHOS::Request {
constexpr const int32_t RETRY_INTERVAL = 500 * 1000;
constexpr const int32_t RETRY_MAX_TIMES = 5;

const std::unique_ptr<RequestManagerImpl> &RequestManagerImpl::GetInstance()
{
    static std::unique_ptr<RequestManagerImpl> instance(new RequestManagerImpl());
    return instance;
}

int32_t RequestManagerImpl::Create(const Config &config, int32_t &tid, sptr<NotifyInterface> listener)
{
    REQUEST_HILOGD("RequestManagerImpl Create start.");

    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        REQUEST_HILOGE("GetRequestServiceProxy fail.");
        return E_SERVICE_ERROR;
    }
    int32_t ret = proxy->Create(config, tid, listener);
    if (ret == E_UNLOADING_SA) {
        REQUEST_HILOGE("Service ability is quitting");
        return Retry(tid, config, ret, listener);
    }
    REQUEST_HILOGD("RequestManagerImpl Create end.");
    return ret;
}

int32_t RequestManagerImpl::Retry(
    int32_t &taskId, const Config &config, int32_t errorCode, sptr<NotifyInterface> listener)
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
        errorCode = proxy->Create(config, taskId, listener);
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
    if (proxy == nullptr ) {
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

int32_t RequestManagerImpl::On(
    const std::string &type, const std::string &tid, const sptr<NotifyInterface> &listener, Version version)
{
    REQUEST_HILOGD("On in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return false;
    }

    return proxy->On(type, tid, listener, version);
}

int32_t RequestManagerImpl::Off(const std::string &type, const std::string &tid, Version version)
{
    REQUEST_HILOGD("Off in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return false;
    }

    return proxy->Off(type, tid, version);
}

int32_t RequestManagerImpl::Subscribe(const std::string &taskId, const std::shared_ptr<IResponseListener> &listener)
{
    REQUEST_HILOGD("Subscribe in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return false;
    }
    std::shared_ptr<Request> task = this->GetTask(taskId);
    task->AddListener(listener);
    if (task->IsEventSubscribed(Request::EVENT_RESPONSE)) {
        return E_OK;
    }

    this->EnsureChannelOpen();
    proxy->Subscribe(taskId, Request::EVENT_RESPONSE);
    task->MarkEventSubscribed(Request::EVENT_RESPONSE, true);
    return E_OK;
}

int32_t RequestManagerImpl::Unsubscribe(const std::string &taskId, const std::shared_ptr<IResponseListener> &listener)
{
    REQUEST_HILOGD("Unsubscribe in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return false;
    }
    std::shared_ptr<Request> task = this->GetTask(taskId);
    size_t size = task->RemoveListener(listener);
    if (size != 0U) {
        return E_OK;
    }

    if (!task->IsEventSubscribed(Request::EVENT_RESPONSE)) {
        return E_OK;
    }

    proxy->Unsubscribe(taskId, Request::EVENT_RESPONSE);
    task->MarkEventSubscribed(Request::EVENT_RESPONSE, false);
    return E_OK;
}

int32_t RequestManagerImpl::EnsureChannelOpen()
{
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
    auto it = tasks_.find(taskId);
    if (it != tasks_.end()) {
        return it->second;
    }

    auto retPair = this->tasks_.emplace(taskId, std::make_shared<Request>(Request(taskId)));

    if (retPair.second) {
        return retPair.first->second;
    }
    REQUEST_HILOGE("Response Task create fail");
    return std::shared_ptr<Request>();
}

void RequestManagerImpl::OnChannelBroken()
{
    this->msgReceiver_.reset();
}

void RequestManagerImpl::OnResponseReceive(const std::shared_ptr<Response> &response)
{
    auto it = tasks_.find(response->taskId);
    if (it == tasks_.end()) {
        REQUEST_HILOGE("task not found");
        return;
    }

    it->second->OnResponseReceive(response);
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
    if (!msgReceiver_) {
        return;
    }
    msgReceiver_->Shutdown();
    this->EnsureChannelOpen();
}

} // namespace OHOS::Request
