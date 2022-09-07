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

#ifndef DOWNLOAD_TASK_H
#define DOWNLOAD_TASK_H

#include <map>
#include <mutex>
#include <string>

#include "download_notify_interface.h"

static constexpr const char *EVENT_COMPLETE = "complete";
static constexpr const char *EVENT_PAUSE = "pause";
static constexpr const char *EVENT_REMOVE = "remove";
static constexpr const char *EVENT_PROGRESS = "progress";
static constexpr const char *EVENT_FAIL = "fail";

namespace OHOS::Request::Download {

enum ParamNumber {
    NO_PARAMETER,
    ONE_PARAMETER,
    TWO_PARAMETER,
};
class DownloadTask {
public:
    explicit DownloadTask(uint32_t taskId);
    ~DownloadTask();

    uint32_t GetId() const;

    bool AddListener(const std::string &type, sptr<DownloadNotifyInterface> listener);
    void RemoveListener(const std::string &type, sptr<DownloadNotifyInterface> listener);
    void RemoveListener(const std::string &type);

    bool IsSupportType(const std::string &type);

private:
    int taskId_;
    std::mutex mutex_;
    std::map<std::string, sptr<DownloadNotifyInterface>> listenerMap_;
    std::map<std::string, bool> supportEvents_;
};
} // namespace OHOS::Request::Download
#endif // DOWNLOAD_TASK_H
