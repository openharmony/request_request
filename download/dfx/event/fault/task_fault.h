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

#ifndef TASK_FAULT_H
#define TASK_FAULT_H

namespace OHOS::Request::Download {
class TaskFault {
public:
    static TaskFault &GetInstance();
    void ReportServiceStartFault(int error) const;
    void ReportTaskFault(int error) const;
private:
    TaskFault() = default;
    ~TaskFault() = default;
    TaskFault(const TaskFault &) = delete;
    TaskFault(TaskFault &&) = delete;
    TaskFault &operator=(const TaskFault &) = delete;
    TaskFault &operator=(TaskFault &&) = delete;

    static constexpr const char *REQUEST_SERVICE_START_FAULT = "REQUEST_SERVICE_START_FAULT";
    static constexpr const char *REQUEST_TASK_FAULT = "REQUEST_TASK_FAULT";
    static constexpr const char *TASKS_TYPE = "TASKS_TYPE";
    static constexpr const char *DOWNLOAD = "DOWNLOAD";
    static constexpr const char *TOTAL_FILE_NUM = "TOTAL_FILE_NUM";
    static constexpr const char *FAIL_FILE_NUM = "FAIL_FILE_NUM";
    static constexpr const char *SUCCESS_FILE_NUM = "SUCCESS_FILE_NUM";
    static constexpr const char *ERROR_INFO = "ERROR_INFO";
};
}
#endif // TASK_FAULT_H