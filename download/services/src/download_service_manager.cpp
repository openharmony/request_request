/*
 * Copyright (c) 2022 Huawei Device Co., Ltd.
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

#include "download_service_manager.h"
#include <cstddef>
#include <algorithm>
#include <cstdint>
#include <functional>
#include <map>
#include <memory>
#include <new>
#include <queue>
#include <mutex>
#include <thread>
#include <utility>
#include "network_adapter.h"
#include "net_conn_constants.h"
#include "application_state_observer.h"
#include "net_all_capabilities.h"
#include "unistd.h"
#include "log.h"

static constexpr uint32_t THREAD_POOL_NUM = 4;
static constexpr uint32_t TASK_SLEEP_INTERVAL = 1;
static constexpr uint32_t MAX_RETRY_TIMES = 3;

using namespace OHOS::NetManagerStandard;
namespace OHOS::Request::Download {
std::mutex DownloadServiceManager::instanceLock_;
DownloadServiceManager *DownloadServiceManager::instance_ = nullptr;
namespace {
enum class ApplicationState {
    APP_STATE_BEGIN = 0,
    APP_STATE_CREATE = APP_STATE_BEGIN,
    APP_STATE_READY,
    APP_STATE_FOREGROUND,
    APP_STATE_FOCUS,
    APP_STATE_BACKGROUND,
    APP_STATE_TERMINATED,
    APP_STATE_END,
};
}

DownloadServiceManager::DownloadServiceManager()
    : initialized_(false), interval_(TASK_SLEEP_INTERVAL), threadNum_(THREAD_POOL_NUM), timeoutRetry_(MAX_RETRY_TIMES),
    taskId_(0)
{
}

DownloadServiceManager::~DownloadServiceManager()
{
    Destroy();
}

DownloadServiceManager *DownloadServiceManager::GetInstance()
{
    if (instance_ == nullptr) {
        std::lock_guard<std::mutex> lock(instanceLock_);
        if (instance_ == nullptr) {
            instance_ = new (std::nothrow) DownloadServiceManager;
        }
    }
    return instance_;
}

bool DownloadServiceManager::Create(uint32_t threadNum)
{
    std::lock_guard<std::recursive_mutex> lock(mutex_);
    if (initialized_) {
        return true;
    }

    threadNum_ = threadNum;
    for (uint32_t i = 0; i < threadNum; i++) {
        threadList_.push_back(std::make_shared<DownloadThread>([this]() {
            return ProcessTask();
        }, interval_));
        threadList_[i]->Start();
    }

    std::thread th = std::thread([this]() {
        pthread_setname_np(pthread_self(), "download_network");
        if (!MonitorNetwork()) {
            DOWNLOAD_HILOGE("network management SA does not exist");
        }
        MonitorAppState();
    });
    th.detach();
    initialized_ = true;
    return initialized_;
}

void DownloadServiceManager::Destroy()
{
    std::for_each(threadList_.begin(), threadList_.end(), [](auto t) { t->Stop(); });
    threadList_.clear();
    initialized_ = false;
}

uint32_t DownloadServiceManager::AddTask(const DownloadConfig& config)
{
    if (!initialized_) {
        return -1;
    }
    uint32_t taskId = GetCurrentTaskId();
    if (taskMap_.find(taskId) != taskMap_.end()) {
        DOWNLOAD_HILOGD("Invalid case: duplicate taskId");
        return -1;
    }
    auto task = std::make_shared<DownloadServiceTask>(taskId, config);
    if (task == nullptr) {
        DOWNLOAD_HILOGD("No mem to add task");
        return -1;
    }
    // move new task into pending queue
    task->SetRetryTime(timeoutRetry_);
    taskMap_[taskId] = task;
    MoveTaskToQueue(taskId, task);
    return taskId;
}

void DownloadServiceManager::InstallCallback(uint32_t taskId, DownloadTaskCallback eventCb)
{
    if (!initialized_) {
        return;
    }
    std::map<uint32_t, std::shared_ptr<DownloadServiceTask>>::iterator it = taskMap_.find(taskId);
    if (it != taskMap_.end()) {
        it->second->InstallCallback(eventCb);
    }
}

bool DownloadServiceManager::ProcessTask()
{
    if (!initialized_) {
        return false;
    }
    uint32_t taskId;
    auto pickupTask = [this, &taskId]() -> std::shared_ptr<DownloadServiceTask> {
        // pick up one task from pending queue
        std::lock_guard<std::recursive_mutex> autoLock(mutex_);
        if (pendingQueue_.size() > 0) {
            taskId = pendingQueue_.front();
            pendingQueue_.pop();
            if (taskMap_.find(taskId) != taskMap_.end()) {
                return taskMap_[taskId];
            }
        }
        return nullptr;
    };

    auto execTask = [this, &taskId](std::shared_ptr<DownloadServiceTask> task) -> bool {
        if (task == nullptr) {
            return false;
        }
        bool result = task->Run();
        this->MoveTaskToQueue(taskId, task);
        return result;
    };
    return execTask(pickupTask());
}

bool DownloadServiceManager::Pause(uint32_t taskId, uint32_t uid)
{
    if (!initialized_) {
        return false;
    }
    DOWNLOAD_HILOGD("Pause Task[%{public}d]", taskId);
    auto it = taskMap_.find(taskId);
    if (it == taskMap_.end()) {
        return false;
    }
    if (!IsSameUid(static_cast<int32_t>(uid), it->second->GetTaskApplicationInfoUid())) {
        return false;
    }
    if (it->second->Pause()) {
        MoveTaskToQueue(taskId, it->second);
        return true;
    }
    return false;
}

bool DownloadServiceManager::Resume(uint32_t taskId, uint32_t uid)
{
    if (!initialized_) {
        return false;
    }
    DOWNLOAD_HILOGD("Resume Task[%{public}d]", taskId);
    auto it = taskMap_.find(taskId);
    if (it == taskMap_.end()) {
        return false;
    }
    if (!IsSameUid(static_cast<int32_t>(uid), it->second->GetTaskApplicationInfoUid())) {
        return false;
    }
    if (it->second->Resume()) {
        MoveTaskToQueue(taskId, it->second);
        return true;
    }
    return false;
}

bool DownloadServiceManager::Remove(uint32_t taskId, uint32_t uid)
{
    if (!initialized_) {
        return false;
    }
    DOWNLOAD_HILOGD("Remove Task[%{public}d]", taskId);
    std::lock_guard<std::recursive_mutex> autoLock(mutex_);
    auto it = taskMap_.find(taskId);
    if (it == taskMap_.end()) {
        return false;
    }
    if (!IsSameUid(static_cast<int32_t>(uid), it->second->GetTaskApplicationInfoUid())) {
        return false;
    }
    bool result = it->second->Remove();
    if (result) {
        taskMap_.erase(it);
        RemoveFromQueue(pendingQueue_, taskId);
        RemoveFromQueue(pausedQueue_, taskId);
    }
    return result;
}

bool DownloadServiceManager::Query(uint32_t taskId, DownloadInfo &info)
{
    if (!initialized_) {
        return false;
    }
    auto it = taskMap_.find(taskId);
    if (it == taskMap_.end()) {
        return false;
    }
    return it->second->Query(info);
}

bool DownloadServiceManager::Query(uint32_t taskId, uint32_t uid, DownloadInfo &info)
{
    if (!initialized_) {
        return false;
    }
    std::lock_guard<std::recursive_mutex> autoLock(mutex_);
    auto it = taskMap_.find(taskId);
    if (it == taskMap_.end()) {
        return false;
    }
    if (!IsSameUid(static_cast<int32_t>(uid), it->second->GetTaskApplicationInfoUid())) {
        return false;
    }
    return it->second->Query(info);
}

bool DownloadServiceManager::QueryMimeType(uint32_t taskId, uint32_t uid, std::string &mimeType)
{
    if (!initialized_) {
        return false;
    }
    auto it = taskMap_.find(taskId);
    if (it == taskMap_.end()) {
        return false;
    }
    if (!IsSameUid(static_cast<int32_t>(uid), it->second->GetTaskApplicationInfoUid())) {
        return false;
    }
    return it->second->QueryMimeType(mimeType);
}

uint32_t DownloadServiceManager::GetStartId() const
{
    return taskId_;
}

uint32_t DownloadServiceManager::GetCurrentTaskId()
{
    std::lock_guard<std::recursive_mutex> autoLock(mutex_);
    return taskId_++;
}

DownloadServiceManager::QueueType DownloadServiceManager::DecideQueueType(DownloadStatus status)
{
    switch (status) {
        case SESSION_PAUSED:
            return QueueType::PAUSED_QUEUE;

        case SESSION_UNKNOWN:
            return QueueType::PENDING_QUEUE;
    
        case SESSION_PENDING:
        case SESSION_RUNNING:
        case SESSION_SUCCESS:
        case SESSION_FAILED:
        default:
            return QueueType::NONE_QUEUE;
    }
    return QueueType::NONE_QUEUE;
}

void DownloadServiceManager::MoveTaskToQueue(uint32_t taskId, std::shared_ptr<DownloadServiceTask> task)
{
    DownloadStatus status;
    ErrorCode code;
    PausedReason reason;
    task->GetRunResult(status, code, reason);
    DOWNLOAD_HILOGD("Status [%{public}d], Code [%{public}d], Reason [%{public}d]", status, code, reason);
    switch (DecideQueueType(status)) {
        case QueueType::PENDING_QUEUE: {
            std::lock_guard<std::recursive_mutex> autoLock(mutex_);
            RemoveFromQueue(pausedQueue_, taskId);
            PushQueue(pendingQueue_, taskId);
            break;
        }
        case QueueType::PAUSED_QUEUE: {
            std::lock_guard<std::recursive_mutex> autoLock(mutex_);
            RemoveFromQueue(pendingQueue_, taskId);
            PushQueue(pausedQueue_, taskId);
            break;
        }
        case QueueType::NONE_QUEUE:
        default:
            break;
    }
}

void DownloadServiceManager::PushQueue(std::queue<uint32_t> &queue, uint32_t taskId)
{
    std::lock_guard<std::recursive_mutex> autoLock(mutex_);
    if (taskMap_.find(taskId) == taskMap_.end()) {
        DOWNLOAD_HILOGD("invalid task id [%{public}d]", taskId);
        return;
    }

    if (queue.empty()) {
        queue.push(taskId);
        return;
    }

    auto headElement = queue.front();
    if (headElement == taskId) {
        return;
    }

    bool foundIt = false;
    uint32_t indicatorId = headElement;
    do {
        if (queue.front() == taskId) {
            foundIt = true;
        }
        queue.push(headElement);
        queue.pop();
        headElement = queue.front();
    } while (headElement != indicatorId);

    if (!foundIt) {
        queue.push(taskId);
    }
}

void DownloadServiceManager::RemoveFromQueue(std::queue<uint32_t> &queue, uint32_t taskId)
{
    std::lock_guard<std::recursive_mutex> autoLock(mutex_);
    if (queue.empty()) {
        return;
    }

    auto headElement = queue.front();
    if (headElement == taskId) {
        queue.pop();
        return;
    }

    auto indicatorId = headElement;
    do {
        if (headElement != taskId) {
            queue.push(queue.front());
        }
        queue.pop();
        headElement = queue.front();
    } while (headElement != indicatorId);
}

void DownloadServiceManager::SetInterval(uint32_t interval)
{
    interval_ = interval;
}
uint32_t DownloadServiceManager::GetInterval() const
{
    return interval_;
}

void DownloadServiceManager::ResumeTaskByNetwork()
{
    int taskCount = 0;
    std::lock_guard<std::recursive_mutex> autoLock(mutex_);
    size_t size = pausedQueue_.size();
    while (size-- > 0) {
        uint32_t taskId = pausedQueue_.front();
        if (taskMap_.find(taskId) != taskMap_.end()) {
            pausedQueue_.pop();
            auto task = taskMap_[taskId];
            DownloadStatus status;
            ErrorCode code;
            PausedReason reason;
            task->GetRunResult(status, code, reason);
            if (reason != PAUSED_BY_USER) {
                task->Resume();
                PushQueue(pendingQueue_, taskId);
                taskCount++;
            } else {
                pausedQueue_.push(taskId);
            }
        }
    }
    DOWNLOAD_HILOGD("[%{public}d] task has been resumed by network status changed", taskCount);
}

bool DownloadServiceManager::MonitorNetwork()
{
    return NetworkAdapter::GetInstance().RegOnNetworkChange([this]() {
        this->ResumeTaskByNetwork();
        this->UpdateNetworkType();
    });
}

void DownloadServiceManager::UpdateNetworkType()
{
    DOWNLOAD_HILOGD("UpdateNetworkType start\n");
    std::lock_guard<std::recursive_mutex> autoLock(mutex_);
    DownloadStatus status;
    ErrorCode code;
    PausedReason reason;
    for (const auto &it : taskMap_) {
        it.second->GetRunResult(status, code, reason);
        bool bRet = status == SESSION_RUNNING || status == SESSION_PENDING
                    || status == SESSION_PAUSED;
        if (bRet) {
            if (!it.second->IsSatisfiedConfiguration()) {
                RemoveFromQueue(pendingQueue_, it.first);
                PushQueue(pausedQueue_, it.first);
            }
        }
    }
}
void DownloadServiceManager::MonitorAppState()
{
    bool ret = ApplicationStateObserver::GetInstance().RegisterAppStateChanged(
        [this](const std::string bundleName, int32_t uid, int32_t state) {
        this->UpdateAppState(bundleName, uid, state);
    });
    DOWNLOAD_HILOGD("RegisterAppStateChanged retcode= %{public}d", ret);
}

void DownloadServiceManager::UpdateAppState(const std::string bundleName, int32_t uid, int32_t state)
{
    DOWNLOAD_HILOGI("UpdateAppState uid=%{public}d, bundleName=%{public}s, state=%{public}d",
                    uid, bundleName.c_str(), state);
    std::lock_guard<std::mutex> lck(appStateMutex_);
    for (const auto &iter : taskMap_) {
        if (IsSameApplication(bundleName, uid,
                              iter.second->GetTaskBundleName(), iter.second->GetTaskApplicationInfoUid())) {
            if (IsBackgroundOrTerminated(state)) {
                iter.second->SetNotifyApp(false);
            } else if (IsForeground(state)) {
                iter.second->SetNotifyApp(true);
            }
        }
    }
}

bool DownloadServiceManager::IsSameApplication(const std::string sName, int32_t sUid,
                                               const std::string dName, int32_t dUid)
{
    return  (IsSameBundleName(sName, dName)) && (IsSameUid(sUid, dUid));
}

bool DownloadServiceManager::IsBackgroundOrTerminated(int32_t state)
{
    return state == static_cast<int32_t>(ApplicationState::APP_STATE_BACKGROUND) ||
           state == static_cast<int32_t>(ApplicationState::APP_STATE_TERMINATED);
}

bool DownloadServiceManager::IsForeground(int32_t state)
{
    return state == static_cast<int32_t>(ApplicationState::APP_STATE_FOREGROUND);
}

bool DownloadServiceManager::QueryAllTask(std::vector<DownloadInfo> &taskVector) const
{
    for (const auto &it : taskMap_) {
        DownloadInfo downloadInfo;
        it.second->Query(downloadInfo);
        taskVector.push_back(downloadInfo);
    }
    return true;
}

bool DownloadServiceManager::IsSameBundleName(const std::string &sName, const std::string &dName)
{
    return sName == dName;
}

bool DownloadServiceManager::IsSameUid(int32_t sUid, int32_t dUid)
{
    return sUid = dUid;
}
} // namespace OHOS::Request::Download