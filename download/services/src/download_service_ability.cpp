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
#include "download_service_ability.h"

#include <new>
#include <ctime>
#include <cinttypes>
#include <string>
#include <utility>
#include <vector>
#include <functional>
#include "string_ex.h"
#include "access_token.h"
#include "errors.h"
#include "event_runner.h"
#include "inner_event.h"
#include "iremote_object.h"
#include "message_parcel.h"
#include "ipc_skeleton.h"
#include "accesstoken_kit.h"
#include "system_ability.h"
#include "system_ability_definition.h"
#include "dump_service_impl.h"
#include "task_fault.h"
#include "task_statistics.h"
#include "download_common.h"
#include "download_service_manager.h"
#include "log.h"

namespace OHOS::Request::Download {
using namespace OHOS::HiviewDFX;
using namespace Security::AccessToken;

static const std::string DOWNLOAD_PERMISSION_NAME_INTERNET = "ohos.permission.INTERNET";
static const std::string DOWNLOAD_PERMISSION_NAME_SESSION = "ohos.permission.DOWNLOAD_SESSION_MANAGER";


REGISTER_SYSTEM_ABILITY_BY_ID(DownloadServiceAbility, DOWNLOAD_SERVICE_ID, true);
const std::int64_t INIT_INTERVAL = 5000L;
// const std::int64_t INTERVAL_ZERO = 0L;
std::mutex DownloadServiceAbility::instanceLock_;
sptr<DownloadServiceAbility> DownloadServiceAbility::instance_;
std::shared_ptr<AppExecFwk::EventHandler> DownloadServiceAbility::serviceHandler_;

DownloadServiceAbility::DownloadServiceAbility(int32_t systemAbilityId, bool runOnCreate)
    : SystemAbility(systemAbilityId, runOnCreate), state_(ServiceRunningState::STATE_NOT_START)
{
}

DownloadServiceAbility::~DownloadServiceAbility()
{
    DOWNLOAD_HILOGE("~DownloadServiceAbility state_  is %{public}d.", static_cast<int>(state_));
}

sptr<DownloadServiceAbility> DownloadServiceAbility::GetInstance()
{
    if (instance_ == nullptr) {
        std::lock_guard<std::mutex> autoLock(instanceLock_);
        if (instance_ == nullptr) {
            instance_ = new DownloadServiceAbility(DOWNLOAD_SERVICE_ID, true);
        }
    }
    return instance_;
}

int32_t DownloadServiceAbility::Init()
{
    bool ret = Publish(DownloadServiceAbility::GetInstance());
    if (!ret) {
        DOWNLOAD_HILOGE("DownloadServiceAbility Publish failed.");
        return E_DOWNLOAD_PUBLISH_FAIL;
    }
    auto instance = DownloadServiceManager::GetInstance();
    if (instance == nullptr) {
        DOWNLOAD_HILOGE("DownloadServiceManager is null");
        return ERR_INVALID_VALUE;
    }
    state_ = ServiceRunningState::STATE_RUNNING;
    uint32_t threadNum = 4;
    DOWNLOAD_HILOGI("Start Download Service Manager with %{public}d threas", threadNum);
    instance->Create(threadNum);
    DOWNLOAD_HILOGE("state_  is %{public}d.", static_cast<int>(state_));
    DOWNLOAD_HILOGI("Init DownloadServiceAbility success.");
    return ERR_OK;
}

void DownloadServiceAbility::OnStart()
{
    DOWNLOAD_HILOGI("DownloadServiceAbility::Enter OnStart.");
    if (instance_ == nullptr) {
        instance_ = this;
    }
    if (state_ == ServiceRunningState::STATE_RUNNING) {
        DOWNLOAD_HILOGI("DownloadServiceAbility is already running.");
        return;
    }
    InitServiceHandler();
    TaskStatistics::GetInstance().StartTimerThread();

    int32_t ret = Init();
    if (ret != ERR_OK) {
        TaskFault::GetInstance().ReportServiceStartFault(ret);
        auto callback = [ = ]() { Init(); };
        serviceHandler_->PostTask(callback, INIT_INTERVAL);
        DOWNLOAD_HILOGE("DownloadServiceAbility Init failed. Try again 5s later");
        return;
    }
    state_ = ServiceRunningState::STATE_RUNNING;
    return;
}

void DownloadServiceAbility::InitServiceHandler()
{
    DOWNLOAD_HILOGI("InitServiceHandler started.");
    if (serviceHandler_ != nullptr) {
        DOWNLOAD_HILOGI("InitServiceHandler already init.");
        return;
    }
    std::shared_ptr<AppExecFwk::EventRunner> runner = AppExecFwk::EventRunner::Create("DownloadServiceAbility");
    serviceHandler_ = std::make_shared<AppExecFwk::EventHandler>(runner);
    DOWNLOAD_HILOGI("InitServiceHandler succeeded.");
}

void DownloadServiceAbility::ManualStart()
{
    if (state_ != ServiceRunningState::STATE_RUNNING) {
        DOWNLOAD_HILOGI("DownloadServiceAbility restart.");
        OnStart();
    }
}

void DownloadServiceAbility::OnStop()
{
    DOWNLOAD_HILOGI("OnStop started.");
    if (state_ != ServiceRunningState::STATE_RUNNING) {
        return;
    }
    auto instance = DownloadServiceManager::GetInstance();
    if (instance == nullptr) {
        DOWNLOAD_HILOGE("DownloadServiceManager is null");
        return;
    }
    instance->Destroy();
    serviceHandler_ = nullptr;
    instance_ = nullptr;
    state_ = ServiceRunningState::STATE_NOT_START;
    DOWNLOAD_HILOGI("OnStop end.");
}

int32_t DownloadServiceAbility::Request(const DownloadConfig &config, ExceptionError &err)
{
    ManualStart();
    int32_t taskId = -1;
    auto instance = DownloadServiceManager::GetInstance();
    if (instance == nullptr) {
        DOWNLOAD_HILOGE("DownloadServiceManager is null");
        err.code = EXCEPTION_SERVICE_ERROR;
        err.errInfo = "DownloadServiceManager is null";
        return taskId;
    }
    taskId = static_cast<int32_t>(instance->AddTask(config));
    if (taskId < 0) {
        DOWNLOAD_HILOGE("taskId [%{public}d] is invalid, config url: %{public}s",
                        taskId, config.GetUrl().c_str());
        err.code = EXCEPTION_SERVICE_ERROR;
        err.errInfo = "taskId "+ std::to_string(taskId) + " is invalid, config url: " + config.GetUrl();
        return taskId;
    }
    instance->InstallCallback(taskId, NotifyHandler);
    DOWNLOAD_HILOGI("DownloadServiceAbility Allocate Task[%{public}d] started.", taskId);
    return taskId;
}

bool DownloadServiceAbility::Pause(uint32_t taskId)
{
    ManualStart();
    auto instance = DownloadServiceManager::GetInstance();
    if (instance == nullptr) {
        DOWNLOAD_HILOGE("DownloadServiceManager is null");
        return false;
    }
    DOWNLOAD_HILOGI("DownloadServiceAbility Pause started.");
    return instance->Pause(taskId, IPCSkeleton::GetCallingUid());
}

bool DownloadServiceAbility::Query(uint32_t taskId, DownloadInfo &info)
{
    ManualStart();
    auto instance = DownloadServiceManager::GetInstance();
    if (instance == nullptr) {
        DOWNLOAD_HILOGE("DownloadServiceManager is null");
        return false;
    }
    DOWNLOAD_HILOGI("DownloadServiceAbility Query started.");
    return instance->Query(taskId, IPCSkeleton::GetCallingUid(), info);
}

bool DownloadServiceAbility::QueryMimeType(uint32_t taskId, std::string &mimeType)
{
    ManualStart();
    auto instance = DownloadServiceManager::GetInstance();
    if (instance == nullptr) {
        DOWNLOAD_HILOGE("DownloadServiceManager is null");
        return false;
    }
    DOWNLOAD_HILOGI("DownloadServiceAbility QueryMimeType started.");
    return instance->QueryMimeType(taskId, IPCSkeleton::GetCallingUid(), mimeType);
}

bool DownloadServiceAbility::Remove(uint32_t taskId)
{
    ManualStart();
    auto instance = DownloadServiceManager::GetInstance();
    if (instance == nullptr) {
        DOWNLOAD_HILOGE("DownloadServiceManager is null");
        return false;
    }
    DOWNLOAD_HILOGI("DownloadServiceAbility Remove started.");
    return instance->Remove(taskId, IPCSkeleton::GetCallingUid());
}

bool DownloadServiceAbility::Resume(uint32_t taskId)
{
    ManualStart();
    auto instance = DownloadServiceManager::GetInstance();
    if (instance == nullptr) {
        DOWNLOAD_HILOGE("DownloadServiceManager is null");
        return false;
    }
    DOWNLOAD_HILOGI("DownloadServiceAbility Resume started.");
    return instance->Resume(taskId, IPCSkeleton::GetCallingUid());
}

bool DownloadServiceAbility::On(uint32_t taskId, const std::string &type, const sptr<DownloadNotifyInterface> &listener)
{
    std::string combineType = type + "-" + std::to_string(taskId);
    DOWNLOAD_HILOGI("DownloadServiceAbility::On started. type=%{public}s", combineType.c_str());
    auto iter = registeredListeners_.find(combineType);
    if (iter == registeredListeners_.end()) {
        std::lock_guard<std::mutex> lck(listenerMapMutex_);
        std::pair<std::string, sptr<DownloadNotifyInterface>> newObj(combineType, listener);
        const auto temp = registeredListeners_.insert(newObj);
        if (!temp.second) {
            DOWNLOAD_HILOGE("DownloadServiceAbility::On insert type=%{public}s object fail.", combineType.c_str());
            return false;
        }
        if (DoUnregisteredNotify(taskId, type)) {
            DOWNLOAD_HILOGD("notify unregistered on event");
        }
    } else {
        std::lock_guard<std::mutex> lck(listenerMapMutex_);
        DOWNLOAD_HILOGI("DownloadServiceAbility::On Replace listener.");
        registeredListeners_[combineType] = listener;
    }
    DOWNLOAD_HILOGI("DownloadServiceAbility::On end.");
    return true;
}

bool DownloadServiceAbility::Off(uint32_t taskId, const std::string &type)
{
    std::string combineType = type + "-" + std::to_string(taskId);
    DOWNLOAD_HILOGI("DownloadServiceAbility::Off started.");
    auto iter = registeredListeners_.find(combineType);
    if (iter != registeredListeners_.end()) {
        DOWNLOAD_HILOGE("DownloadServiceAbility::Off delete type=%{public}s object message.", combineType.c_str());
        std::lock_guard<std::mutex> lck(listenerMapMutex_);
        registeredListeners_.erase(iter);
        return true;
    }
    return false;
}

bool DownloadServiceAbility::CheckPermission()
{
    AccessTokenID tokenId = IPCSkeleton::GetCallingTokenID();
    TypeATokenTypeEnum tokenType = AccessTokenKit::GetTokenTypeFlag(tokenId);
    if (tokenType == TOKEN_INVALID) {
        DOWNLOAD_HILOGE("invalid token id %{public}d", tokenId);
        return false;
    }
    int result = AccessTokenKit::VerifyAccessToken(tokenId, DOWNLOAD_PERMISSION_NAME_INTERNET);
    if (result != PERMISSION_GRANTED) {
        DOWNLOAD_HILOGE("Current tokenId permission is %{public}d", result);
    }
    return result == PERMISSION_GRANTED;
}

void DownloadServiceAbility::NotifyHandler(const std::string &type, uint32_t taskId, int64_t argv1, int64_t argv2,
    bool isNotify)
{
    if (!isNotify) {
        DOWNLOAD_HILOGE("isNotify false");
        return;
    }
    std::string combineType = type + "-" + std::to_string(taskId);
    DOWNLOAD_HILOGD("combineType=%{public}s argv1=%{public}" PRId64 "argv2=%{public}" PRId64, combineType.c_str(),
        argv1, argv2);
    auto iter = DownloadServiceAbility::GetInstance()->registeredListeners_.find(combineType);
    if (iter != DownloadServiceAbility::GetInstance()->registeredListeners_.end()) {
        DOWNLOAD_HILOGD("DownloadServiceAbility::NotifyHandler type=%{public}s object message.", combineType.c_str());
        std::vector<int64_t> params;
        params.push_back(argv1);
        params.push_back(argv2);
        iter->second->CallBack(params);
    } else {
        DownloadServiceAbility::GetInstance()->AddUnregisteredNotify(taskId, type);
    }
}

void DownloadServiceAbility::OnDump()
{
    std::lock_guard<std::mutex> guard(lock_);
    struct tm *timeNow = nullptr;
    time_t second = time(0);
    if (second > 0) {
        timeNow = localtime(&second);
        if (timeNow != nullptr) {
            DOWNLOAD_HILOGI(
                "DownloadServiceAbility dump time:%{public}d-%{public}d-%{public}d %{public}d:%{public}d:%{public}d",
                timeNow->tm_year + startTime_, timeNow->tm_mon + extraMonth_, timeNow->tm_mday, timeNow->tm_hour,
                timeNow->tm_min, timeNow->tm_sec);
        }
    } else {
        DOWNLOAD_HILOGI("DownloadServiceAbility dump, time(0) is nullptr");
    }
}

int DownloadServiceAbility::Dump(int fd, const std::vector<std::u16string> &args)
{
    int uid = static_cast<int>(IPCSkeleton::GetCallingUid());
    const int maxUid = 10000;
    if (uid > maxUid) {
        return 0;
    }

    std::vector<std::string> argsStr;
    for (auto item : args) {
        argsStr.emplace_back(Str16ToStr8(item));
    }

    return DumpServiceImpl::GetInstance().Dump(fd, argsStr);
}

void DownloadServiceAbility::AddUnregisteredNotify(uint32_t taskId, const std::string &type)
{
    std::string combineType = type + "-" + std::to_string(taskId);
    DOWNLOAD_HILOGD("add combineType %{public}s", combineType.c_str());
    if (type == EVENT_COMPLETE || type == EVENT_FAIL) {
        std::lock_guard<std::mutex> lck(unregisteredNotifyMutex_);
        auto iter = unregisteredNotify_.find(combineType);
        if (iter == unregisteredNotify_.end()) {
            unregisteredNotify_.insert(std::make_pair(combineType, taskId));
        }
    }
}

bool DownloadServiceAbility::DoUnregisteredNotify(uint32_t taskId, const std::string &type)
{
    std::string combineType = type + "-" + std::to_string(taskId);
    DOWNLOAD_HILOGD("notify combineType: %{public}s", combineType.c_str());
    DownloadInfo info;
    if (!Query(taskId, info)) {
        DOWNLOAD_HILOGD("not find task download info");
        return false;
    }
    auto status = info.GetStatus();
    uint32_t code = 0;
    if (info.GetFailedReason() != ERROR_UNKNOWN) {
        code = static_cast<uint32_t>(info.GetFailedReason());
    }
    std::lock_guard<std::mutex> lck(unregisteredNotifyMutex_);
    auto iter = unregisteredNotify_.find(combineType);
    if (iter != unregisteredNotify_.end()) {
        if (status == SESSION_SUCCESS || status == SESSION_FAILED) {
            DOWNLOAD_HILOGD("notify taskId: %{public}d event: %{public}s", taskId, type.c_str());
            NotifyHandler(type, taskId, code, 0, true);
            unregisteredNotify_.erase(iter);
            return true;
        }
    }
    return false;
}
} // namespace OHOS::Request::Download
