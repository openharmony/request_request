/*
 * Copyright (c) 2024 Huawei Device Co., Ltd.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "cj_request_ffi.h"
#include <cinttypes>
#include "cj_request_log.h"
#include "cj_request_task.h"
#include "cj_request_common.h"
#include "cj_request_impl.h"

namespace OHOS::CJSystemapi::Request {

extern "C" {
    void FfiOHOSRequestFreeTask(const char *taskId)
    {
        CJRequestImpl::FreeTask(taskId);
    }

    RetError FfiOHOSRequestTaskProgressOn(char *event, const char *taskId, void (*callback)(CProgress progress))
    {
        return CJRequestImpl::ProgressOn(event, taskId, callback);
    }

    RetError FfiOHOSRequestTaskProgressOff(char *event, const char *taskId, void *callback)
    {
        return CJRequestImpl::ProgressOff(event, taskId, callback);
    }

    RetError FfiOHOSRequestTaskStart(const char *taskId)
    {
        return CJRequestImpl::TaskStart(taskId);
    }

    RetError FfiOHOSRequestTaskPause(const char *taskId)
    {
        return CJRequestImpl::TaskPause(taskId);
    }

    RetError FfiOHOSRequestTaskResume(const char *taskId)
    {
        return CJRequestImpl::TaskResume(taskId);
    }

    RetError FfiOHOSRequestTaskStop(const char *taskId)
    {
        return CJRequestImpl::TaskStop(taskId);
    }

    RetReqData FfiOHOSRequestCreateTask(void* context, CConfig config)
    {
        return CJRequestImpl::CreateTask((OHOS::AbilityRuntime::Context *)context, &config);
    }

    RetError FfiOHOSRequestRemoveTask(const char *taskId)
    {
        return CJRequestImpl::RemoveTask(taskId);
    }
}
}