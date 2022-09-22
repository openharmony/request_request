/*
 * Copyright (C) 2021-2022 Huawei Device Co., Ltd.
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

#include <thread>
#include <atomic>
#include <sys/types.h>
#include <unistd.h>
#include <ctime>
#include "log.h"
#include "hisysevent.h"
#include "task_statistics.h"

namespace OHOS::Request::Download {
TaskStatistics &TaskStatistics::GetInstance()
{
    static TaskStatistics instance;
    return instance;
}

void TaskStatistics::ReportTasksSize(uint64_t totalSize)
{
    dayTasksSize_ += totalSize;
}

void TaskStatistics::ReportTasksNumber()
{
    dayTasksNumber_ ++;
}

int32_t TaskStatistics::GetNextReportInterval() const
{
    time_t current = time(nullptr);
    if (current == -1) {
        DOWNLOAD_HILOGE("GetNextReportInterval time fail");
        return -1;
    }
    tm localTime = { 0 };
    tm *result = localtime_r(&current, &localTime);
    if (result == nullptr) {
        DOWNLOAD_HILOGE("GetNextReportInterval localTime fail");
        return -1;
    }
    int currentTime = localTime.tm_hour * ONE_HOUR_SEC + localTime.tm_min * ONE_MINUTE_SEC + localTime.tm_sec;
    return  ONE_DAY_SEC - currentTime;
}

void TaskStatistics::ReportStatistics() const
{
    OHOS::HiviewDFX::HiSysEvent::Write(OHOS::HiviewDFX::HiSysEvent::Domain::REQUEST,
        REQUEST_TASK_INFO_STATISTICS,
        OHOS::HiviewDFX::HiSysEvent::EventType::STATISTIC,
        TASKS_SIZE, &dayTasksSize_,
        TASKS_NUMBER, &dayTasksNumber_);
}

void TaskStatistics::StartTimerThread()
{
    if (running_) {
        return;
    }

    running_ = true;
    auto fun = [this]() {
        while (true) {
            int32_t nextReportInterval = GetNextReportInterval();
            if (nextReportInterval < 0) {
                nextReportInterval = ONE_DAY_SEC;
            }
            DOWNLOAD_HILOGD("taskRun next interval: %{public}d", nextReportInterval);
            sleep(nextReportInterval);
            ReportStatistics();
            this->dayTasksNumber_ = 0;
            this->dayTasksSize_ = 0;
        }
    };
    std::thread th = std::thread(fun);
    th.detach();
}
} // namespace OHOS::Request::Download