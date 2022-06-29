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

#ifndef TASK_STATISTICS_H
#define TASK_STATISTICS_H

#include <mutex>

namespace OHOS::Request::Download {
class TaskStatistics {
public:
    static TaskStatistics &GetInstance();
    void ReportTasksSize(uint64_t totalSize);
    void ReportTasksNumber(uint32_t number);
    uint64_t GetDayTasksSize() const;
    uint32_t GetDayTasksNumber() const;
    void StartTimerThread();

private:
    TaskStatistics();
    virtual ~TaskStatistics() = default;
    TaskStatistics(TaskStatistics const &) = delete;
    void operator=(TaskStatistics const &) = delete;

    int32_t GetNextReportInterval() const;
    void ReportStatistics() const;
private:
    std::mutex mutex_;
    uint64_t DayTasksSize_;
    uint32_t DayTasksNumber_;
    bool running_ = false;
};
}
#endif // TASK_STATISTICS_H
