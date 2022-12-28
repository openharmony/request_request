/*
 * Copyright (C) 2022 Huawei Device Co., Ltd.
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

#include "download_manager.h"

#include "system_ability_definition.h"
#include "iservice_registry.h"

#include "data_ability_predicates.h"
#include "rdb_errno.h"
#include "rdb_helper.h"
#include "rdb_open_callback.h"
#include "rdb_predicates.h"
#include "rdb_store.h"
#include "result_set.h"
#include "log.h"
#include "download_sync_load_callback.h"

namespace OHOS::Request::Download {
std::mutex DownloadManager::instanceLock_;
sptr<DownloadManager> DownloadManager::instance_ = nullptr;

DownloadManager::DownloadManager() : downloadServiceProxy_(nullptr), deathRecipient_(nullptr)
{
}

DownloadManager::~DownloadManager()
{
}

sptr<DownloadManager> DownloadManager::GetInstance()
{
    if (instance_ == nullptr) {
        std::lock_guard<std::mutex> autoLock(instanceLock_);
        if (instance_ == nullptr) {
            instance_ = new DownloadManager;
        }
    }
    return instance_;
}

DownloadTask* DownloadManager::EnqueueTask(const DownloadConfig &config, ExceptionError &err)
{
    DOWNLOAD_HILOGD("DownloadManager EnqueueTask start.");

    auto proxy = GetDownloadServiceProxy();
    if (proxy == nullptr) {
        return nullptr;
    }
    
    int32_t taskId = proxy->Request(config, err);
    if (taskId < 0) {
        DOWNLOAD_HILOGE("taskId invalid");
        return nullptr;
    }
    DOWNLOAD_HILOGD("DownloadManager EnqueueTask succeeded.");
    return new DownloadTask(taskId);
}

bool DownloadManager::Pause(uint32_t taskId)
{
    auto proxy = GetDownloadServiceProxy();
    if (proxy == nullptr) {
        return false;
    }

    return proxy->Pause(taskId);
}

bool DownloadManager::Query(uint32_t taskId, DownloadInfo &info)
{
    auto proxy = GetDownloadServiceProxy();
    if (proxy == nullptr) {
        return false;
    }
    
    return proxy->Query(taskId, info);
}

bool DownloadManager::QueryMimeType(uint32_t taskId, std::string &mimeType)
{
    auto proxy = GetDownloadServiceProxy();
    if (proxy == nullptr) {
        return false;
    }
    
    return proxy->QueryMimeType(taskId, mimeType);
}

bool DownloadManager::Remove(uint32_t taskId)
{
    auto proxy = GetDownloadServiceProxy();
    if (proxy == nullptr) {
        return false;
    }

    return proxy->Remove(taskId);
}

bool DownloadManager::Resume(uint32_t taskId)
{
    auto proxy = GetDownloadServiceProxy();
    if (proxy == nullptr) {
        return false;
    }

    return proxy->Resume(taskId);
}

bool DownloadManager::On(uint32_t taskId, const std::string &type, const sptr<DownloadNotifyInterface> &listener)
{
    auto proxy = GetDownloadServiceProxy();
    if (proxy == nullptr) {
        return false;
    }

    return proxy->On(taskId, type, listener);
}

bool DownloadManager::Off(uint32_t taskId, const std::string &type)
{
    auto proxy = GetDownloadServiceProxy();
    if (proxy == nullptr) {
        return false;
    }

    return proxy->Off(taskId, type);
}

bool DownloadManager::CheckPermission()
{
    auto proxy = GetDownloadServiceProxy();
    if (proxy == nullptr) {
        return false;
    }

    return proxy->CheckPermission();
}

sptr<DownloadServiceInterface> DownloadManager::GetDownloadServiceProxy()
{
    if (downloadServiceProxy_ != nullptr) {
        return downloadServiceProxy_;
    }
    sptr<ISystemAbilityManager> systemAbilityManager =
        SystemAbilityManagerClient::GetInstance().GetSystemAbilityManager();
    if (systemAbilityManager == nullptr) {
        DOWNLOAD_HILOGE("Getting SystemAbilityManager failed.");
        return nullptr;
    }
    auto systemAbility = systemAbilityManager->GetSystemAbility(DOWNLOAD_SERVICE_ID, "");
    if (systemAbility == nullptr) {
        DOWNLOAD_HILOGE("Get SystemAbility failed.");
        return nullptr;
    }
    deathRecipient_ = new DownloadSaDeathRecipient();
    systemAbility->AddDeathRecipient(deathRecipient_);
    downloadServiceProxy_ = iface_cast<DownloadServiceInterface>(systemAbility);
    if (downloadServiceProxy_ == nullptr) {
        DOWNLOAD_HILOGE("Get downloadServiceProxy_ fail.");
        return nullptr;
    }
    return downloadServiceProxy_;
}

void DownloadManager::OnRemoteSaDied(const wptr<IRemoteObject> &remote)
{
    downloadServiceProxy_ = nullptr;
    ready_ = false;
    LoadDownloadServer();
    GetDownloadServiceProxy();
}

DownloadSaDeathRecipient::DownloadSaDeathRecipient()
{
}

void DownloadSaDeathRecipient::OnRemoteDied(const wptr<IRemoteObject> &object)
{
    DOWNLOAD_HILOGE("DownloadSaDeathRecipient on remote systemAbility died.");
    DownloadManager::GetInstance()->OnRemoteSaDied(object);
}

bool DownloadManager::LoadDownloadServer()
{
    if (ready_) {
        return true;
    }
    std::lock_guard<std::mutex> lock(downloadMutex_);
    if (ready_) {
        return true;
    }

    auto sm = SystemAbilityManagerClient::GetInstance().GetSystemAbilityManager();
    if (sm == nullptr) {
        DOWNLOAD_HILOGE("GetSystemAbilityManager return null");
        return false;
    }
    auto systemAbility = sm->GetSystemAbility(DOWNLOAD_SERVICE_ID);
    if (systemAbility != nullptr) {
        DOWNLOAD_HILOGE("service already exists");
        return true;
    }
    sptr<DownloadSyncLoadCallback> loadCallback_ = new (std::nothrow) DownloadSyncLoadCallback();
    if (loadCallback_ == nullptr) {
        DOWNLOAD_HILOGE("new DownloadAbilityCallback fail");
        return false;
    }

    int32_t result =  sm->LoadSystemAbility(DOWNLOAD_SERVICE_ID, loadCallback_);
    if (result != ERR_OK) {
        DOWNLOAD_HILOGE("LoadSystemAbility %{public}d failed, result: %{public}d", DOWNLOAD_SERVICE_ID, result);
        return false;
    }

    {
        std::unique_lock<std::mutex> conditionLock(conditionMutex_);
        auto waitStatus = downloadSyncCon_.wait_for(conditionLock, std::chrono::milliseconds(LOAD_SA_TIMEOUT_MS),
                                                    [this]() { return ready_; });
        if (!waitStatus) {
            DOWNLOAD_HILOGE("download server load sa timeout");
            return false;
        }
    }
    return true;
}

void DownloadManager::LoadServerSuccess()
{
    std::unique_lock<std::mutex> lock(conditionMutex_);
    ready_ = true;
    downloadSyncCon_.notify_one();
    DOWNLOAD_HILOGE("load download server success");
}

void DownloadManager::LoadServerFail()
{
    ready_ = false;
    DOWNLOAD_HILOGE("load download server fail");
}
} // namespace OHOS::Request::Download
