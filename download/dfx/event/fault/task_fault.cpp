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

#include "task_fault.h"
#include "hisysevent.h"

namespace OHOS::Request::Download {
TaskFault &TaskFault::GetInstance()
{
    static TaskFault instance;
    return instance;
}

void TaskFault::ReportServiceStartFault(int error) const
{
    OHOS::HiviewDFX::HiSysEvent::Write(OHOS::HiviewDFX::HiSysEvent::Domain::REQUEST,
        REQUEST_SERVICE_START_FAULT,
        OHOS::HiviewDFX::HiSysEvent::EventType::FAULT,
        ERROR_INFO, error);
}

void TaskFault::ReportTaskFault(int error) const
{
    OHOS::HiviewDFX::HiSysEvent::Write(OHOS::HiviewDFX::HiSysEvent::Domain::REQUEST,
        REQUEST_TASK_FAULT,
        OHOS::HiviewDFX::HiSysEvent::EventType::FAULT,
        TASKS_TYPE, DOWNLOAD,
        TOTAL_FILE_NUM, 1,
        FAIL_FILE_NUM, 1,
        SUCCESS_FILE_NUM, 0,
        ERROR_INFO, error);
}
} // namespace OHOS::Request::Download