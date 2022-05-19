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

#ifndef DOWNLOAD_SERVICE_MANAGER_H
#define DOWNLOAD_SERVICE_MANAGER_H

#include <map>
#include <memory>
#include <mutex>
#include <queue>

#include "constant.h"
#include "download_config.h"
#include "download_info.h"
#include "download_service_task.h"
#include "download_thread.h"

namespace OHOS::Request::Download {
class DownloadServiceManager final {
public:
    explicit DownloadServiceManager();
    ~DownloadServiceManager();

    static std::shared_ptr<DownloadServiceManager> Get();

    bool Create(uint32_t threadNum);
    void Destroy();

    uint32_t AddTask(const DownloadConfig &config);
    void InstallCallback(uint32_t taskId, DownloadTaskCallback eventCb);
    bool ProcessTask();

    bool Pause(uint32_t taskId);
    bool Resume(uint32_t taskId);
    bool Remove(uint32_t taskId);
    bool Query(uint32_t taskId, DownloadInfo &info);
    bool QueryMimeType(uint32_t taskId, std::string &mimeType);

    void SetStartId(uint32_t startId);
    uint32_t GetStartId() const;

    void SetInterval(uint32_t interval);
    uint32_t GetInterval() const;
    
    void ResumeTaskByNetwork();
private:
    enum class QueueType {
        NONE_QUEUE,
        PENDING_QUEUE,
        PAUSED_QUEUE,
    };

    uint32_t GetCurrentTaskId();
    QueueType DecideQueueType(DownloadStatus status);
    void MoveTaskToQueue(uint32_t taskId, std::shared_ptr<DownloadServiceTask> task);
    void PushQueue(std::queue<uint32_t> &queue, uint32_t taskId);
    void RemoveFromQueue(std::queue<uint32_t> &queue, uint32_t taskId);
    int32_t MonitorNetwork();

private:
    bool initialized_;
    std::recursive_mutex mutex_;
    std::map<uint32_t, std::shared_ptr<DownloadServiceTask>> taskMap_;
    std::queue<uint32_t> pendingQueue_;
    std::queue<uint32_t> pausedQueue_;
    std::vector<std::shared_ptr<DownloadThread>> threadList_;

    /* configuration for download service manager */
    uint32_t interval_;
    uint32_t threadNum_;
    uint32_t timeoutRetry_;

    uint32_t taskId_;
    static std::recursive_mutex instanceLock_;
    static std::shared_ptr<DownloadServiceManager> instance_;
};
} // namespace OHOS::Request::Download
#endif // DOWNLOAD_SERVICE_MANAGER_H
