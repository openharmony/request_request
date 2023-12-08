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

#include "request_manager.h"

#include "data_ability_predicates.h"
#include "request_sync_load_callback.h"
#include "iservice_registry.h"
#include "log.h"
#include "rdb_errno.h"
#include "rdb_helper.h"
#include "rdb_open_callback.h"
#include "rdb_predicates.h"
#include "rdb_store.h"
#include "result_set.h"
#include "system_ability_definition.h"

namespace OHOS::Request {
std::mutex RequestManager::instanceLock_;
sptr<RequestManager> RequestManager::instance_ = nullptr;
constexpr const int32_t RETRY_INTERVAL = 500 * 1000;
constexpr const int32_t RETRY_MAX_TIMES = 5;

RequestManager::RequestManager() : requestServiceProxy_(nullptr), deathRecipient_(nullptr)
{
}

RequestManager::~RequestManager()
{
}

sptr<RequestManager> RequestManager::GetInstance()
{
    if (instance_ == nullptr) {
        std::lock_guard<std::mutex> autoLock(instanceLock_);
        if (instance_ == nullptr) {
            instance_ = new RequestManager;
        }
    }
    return instance_;
}

int32_t RequestManager::Create(const Config &config, int32_t &tid, sptr<NotifyInterface> listener)
{
    REQUEST_HILOGD("RequestManager Create start.");

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
    REQUEST_HILOGD("RequestManager Create end.");
    return ret;
}

int32_t RequestManager::Retry(int32_t &taskId, const Config &config, int32_t errorCode, sptr<NotifyInterface> listener)
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

void RequestManager::SetRequestServiceProxy(sptr<RequestServiceInterface> proxy)
{
    std::lock_guard<std::mutex> lock(serviceProxyMutex_);
    requestServiceProxy_ = proxy;
}

int32_t RequestManager::GetTask(const std::string &tid, const std::string &token, Config &config)
{
    REQUEST_HILOGD("GetTask in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return E_SERVICE_ERROR;
    }

    return proxy->GetTask(tid, token, config);
}

int32_t RequestManager::Start(const std::string &tid)
{
    REQUEST_HILOGD("Start in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return E_SERVICE_ERROR;
    }

    return proxy->Start(tid);
}

int32_t RequestManager::Stop(const std::string &tid)
{
    REQUEST_HILOGD("Stop in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return E_SERVICE_ERROR;
    }

    return proxy->Stop(tid);
}

int32_t RequestManager::Query(const std::string &tid, TaskInfo &info)
{
    REQUEST_HILOGD("Query in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return E_SERVICE_ERROR;
    }

    return proxy->Query(tid, info);
}

int32_t RequestManager::Touch(const std::string &tid, const std::string &token, TaskInfo &info)
{
    REQUEST_HILOGD("Touch in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return E_SERVICE_ERROR;
    }

    return proxy->Touch(tid, token, info);
}

int32_t RequestManager::Search(const Filter &filter, std::vector<std::string> &tids)
{
    REQUEST_HILOGD("Search in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return E_SERVICE_ERROR;
    }

    return proxy->Search(filter, tids);
}

int32_t RequestManager::Show(const std::string &tid, TaskInfo &info)
{
    REQUEST_HILOGD("Show in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return E_SERVICE_ERROR;
    }

    return proxy->Show(tid, info);
}

int32_t RequestManager::Pause(const std::string &tid, Version version)
{
    REQUEST_HILOGD("Pause in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return E_SERVICE_ERROR;
    }

    return proxy->Pause(tid, version);
}

int32_t RequestManager::QueryMimeType(const std::string &tid, std::string &mimeType)
{
    REQUEST_HILOGD("QueryMimeType in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return E_SERVICE_ERROR;
    }

    return proxy->QueryMimeType(tid, mimeType);
}

int32_t RequestManager::Remove(const std::string &tid, Version version)
{
    REQUEST_HILOGD("Remove in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return E_SERVICE_ERROR;
    }

    return proxy->Remove(tid, version);
}

int32_t RequestManager::Resume(const std::string &tid)
{
    REQUEST_HILOGD("Resume in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return E_SERVICE_ERROR;
    }

    return proxy->Resume(tid);
}

int32_t RequestManager::On(const std::string &type, const std::string &tid,
    const sptr<NotifyInterface> &listener, Version version)
{
    REQUEST_HILOGD("On in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return false;
    }

    return proxy->On(type, tid, listener, version);
}

int32_t RequestManager::Off(const std::string &type, const std::string &tid, Version version)
{
    REQUEST_HILOGD("Off in");
    auto proxy = GetRequestServiceProxy();
    if (proxy == nullptr) {
        return false;
    }

    return proxy->Off(type, tid, version);
}

sptr<RequestServiceInterface> RequestManager::GetRequestServiceProxy()
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
    auto saChangeListener = new (std::nothrow) SystemAbilityStatusChangeListener();
    if (systemAbility == nullptr || saChangeListener == nullptr) {
        REQUEST_HILOGE("Get SystemAbility or saChangeListener failed.");
        return nullptr;
    }
    if (systemAbilityManager->SubscribeSystemAbility(DOWNLOAD_SERVICE_ID, saChangeListener) != E_OK) {
        REQUEST_HILOGE("SubscribeSystemAbility failed.");
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

void RequestManager::RestoreListener(void (*callback)())
{
    callback_ = callback;
}

RequestManager::SystemAbilityStatusChangeListener::SystemAbilityStatusChangeListener()
{
}

void RequestManager::SystemAbilityStatusChangeListener::OnAddSystemAbility(int32_t saId, const std::string& deviceId)
{
    if (saId != DOWNLOAD_SERVICE_ID) {
        REQUEST_HILOGE("SA ID is not DOWNLOAD_SERVICE_ID.");
    }
    REQUEST_HILOGD("SystemAbility Add.");
    if (RequestManager::GetInstance()->callback_ != nullptr) {
        RequestManager::GetInstance()->callback_();
    }
}

void RequestManager::SystemAbilityStatusChangeListener::OnRemoveSystemAbility(int32_t saId, const std::string& deviceId)
{
    if (saId != DOWNLOAD_SERVICE_ID) {
        REQUEST_HILOGE("SA ID is not DOWNLOAD_SERVICE_ID.");
    }
    REQUEST_HILOGD("SystemAbility Remove.");
}

void RequestManager::OnRemoteSaDied(const wptr<IRemoteObject> &remote)
{
    REQUEST_HILOGD(" RequestManager::OnRemoteSaDied");
    ready_ = false;
    SetRequestServiceProxy(nullptr);
}

RequestSaDeathRecipient::RequestSaDeathRecipient()
{
}

void RequestSaDeathRecipient::OnRemoteDied(const wptr<IRemoteObject> &object)
{
    REQUEST_HILOGE("RequestSaDeathRecipient on remote systemAbility died.");
    RequestManager::GetInstance()->OnRemoteSaDied(object);
}

bool RequestManager::LoadRequestServer()
{
    REQUEST_HILOGD("Begin load request server");
    if (ready_) {
        REQUEST_HILOGD("GetSystemAbilityManager ready_ true");
        return true;
    }
    std::lock_guard<std::mutex> lock(downloadMutex_);
    if (ready_) {
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
        REQUEST_HILOGE("service already exists");
        return true;
    }
    sptr<RequestSyncLoadCallback> loadCallback_ = new (std::nothrow) RequestSyncLoadCallback();
    if (loadCallback_ == nullptr) {
        REQUEST_HILOGE("new DownloadAbilityCallback fail");
        return false;
    }

    int32_t result = sm->LoadSystemAbility(DOWNLOAD_SERVICE_ID, loadCallback_);
    if (result != ERR_OK) {
        REQUEST_HILOGE("LoadSystemAbility %{public}d failed, result: %{public}d", DOWNLOAD_SERVICE_ID, result);
        return false;
    }

    {
        std::unique_lock<std::mutex> conditionLock(conditionMutex_);
        auto waitStatus = syncCon_.wait_for(conditionLock, std::chrono::milliseconds(LOAD_SA_TIMEOUT_MS),
            [this]() { return ready_; });
        if (!waitStatus) {
            REQUEST_HILOGE("download server load sa timeout");
            return false;
        }
    }
    return true;
}

void RequestManager::LoadServerSuccess()
{
    std::unique_lock<std::mutex> lock(conditionMutex_);
    ready_ = true;
    syncCon_.notify_one();
    REQUEST_HILOGE("load download server success");
}

void RequestManager::LoadServerFail()
{
    ready_ = false;
    REQUEST_HILOGE("load download server fail");
}
} // namespace OHOS::Request
