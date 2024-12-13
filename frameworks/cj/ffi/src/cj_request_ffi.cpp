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
#include "cj_request_common.h"
#include "cj_request_impl.h"
#include "cj_request_task.h"
#include "log.h"

namespace OHOS::CJSystemapi::Request {

extern "C" {
void FfiOHOSRequestFreeTask(const char *taskId)
{
    CJRequestImpl::FreeTask(taskId);
}

RetError FfiOHOSRequestTaskProgressOn(char *event, const char *taskId, void *callback)
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

RetReqData FfiOHOSRequestCreateTask(void *context, CConfig config)
{
    return CJRequestImpl::CreateTask((OHOS::AbilityRuntime::Context *)context, &config);
}

RetTask FfiOHOSRequestGetTask(void *context, const char *taskId, RequestNativeOptionCString token)
{
    return CJRequestImpl::GetTask((OHOS::AbilityRuntime::Context *)context, taskId, token);
}

RetError FfiOHOSRequestRemoveTask(const char *taskId)
{
    return CJRequestImpl::RemoveTask(taskId);
}

RetTaskInfo FfiOHOSRequestShowTask(const char *taskId)
{
    return CJRequestImpl::ShowTask(taskId);
}

RetTaskInfo FfiOHOSRequestTouchTask(const char *taskId, char *token)
{
    return CJRequestImpl::TouchTask(taskId, token);
}

RetTaskArr FfiOHOSRequestSearchTask(CFilter filter)
{
    return CJRequestImpl::SearchTask(filter);
}
}
} // namespace OHOS::CJSystemapi::Request