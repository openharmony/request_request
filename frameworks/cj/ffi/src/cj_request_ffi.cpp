/*
 * Copyright (c) 2024-2026 Huawei Device Co., Ltd.
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
#include "request_common.h"

namespace OHOS::CJSystemapi::Request {

using OHOS::Request::ExceptionErrorCode;

static RetError CreateParamError()
{
    RetError ret = {};
    ret.errCode = static_cast<int32_t>(ExceptionErrorCode::E_PARAMETER_CHECK);
    ret.errMsg = MallocCString("Parameter verification failed");
    return ret;
}

static RetError CreateTaskNotFoundError()
{
    RetError ret = {};
    ret.errCode = static_cast<int32_t>(ExceptionErrorCode::E_TASK_NOT_FOUND);
    ret.errMsg = MallocCString("Task not found");
    return ret;
}

extern "C" {
void FfiOHOSRequestFreeDownloadInfo(CDownloadInfo *info)
{
    if (info == nullptr) {
        return;
    }
    free(info->description);
    info->description = nullptr;
    free(info->fileName);
    info->fileName = nullptr;
    free(info->filePath);
    info->filePath = nullptr;
    free(info->targetURI);
    info->targetURI = nullptr;
    free(info->downloadTitle);
    info->downloadTitle = nullptr;
    free(info->mimeType);
    info->mimeType = nullptr;
}

void FfiOHOSRequestFreeTask(const char *taskId)
{
    if (taskId == nullptr) {
        return;
    }
    CJRequestImpl::FreeTask(taskId);
}

RetError FfiOHOSRequestTaskProgressOn(char *event, const char *taskId, int64_t callback)
{
    if (event == nullptr) {
        return CreateParamError();
    }
    if (taskId == nullptr) {
        return CreateTaskNotFoundError();
    }
    return CJRequestImpl::ProgressOn(event, taskId, callback);
}

RetError FfiOHOSRequestTaskProgressOff(char *event, const char *taskId, int64_t callback)
{
    if (event == nullptr) {
        return CreateParamError();
    }
    if (taskId == nullptr) {
        return CreateTaskNotFoundError();
    }
    return CJRequestImpl::ProgressOff(event, taskId, callback);
}

RetError FfiOHOSRequestTaskFailedOn(const char *taskId, int64_t callback)
{
    if (taskId == nullptr) {
        return CreateTaskNotFoundError();
    }
    return CJRequestImpl::FailedOn(taskId, callback);
}

RetError FfiOHOSRequestTaskFailedOff(const char *taskId, int64_t callback)
{
    if (taskId == nullptr) {
        return CreateTaskNotFoundError();
    }
    return CJRequestImpl::FailedOff(taskId, callback);
}

RetError FfiOHOSRequestTaskStart(const char *taskId)
{
    if (taskId == nullptr) {
        return CreateTaskNotFoundError();
    }
    return CJRequestImpl::TaskStart(taskId);
}

RetError FfiOHOSRequestTaskPause(const char *taskId)
{
    if (taskId == nullptr) {
        return CreateTaskNotFoundError();
    }
    return CJRequestImpl::TaskPause(taskId);
}

RetError FfiOHOSRequestTaskResume(const char *taskId)
{
    if (taskId == nullptr) {
        return CreateTaskNotFoundError();
    }
    return CJRequestImpl::TaskResume(taskId);
}

RetError FfiOHOSRequestTaskStop(const char *taskId)
{
    if (taskId == nullptr) {
        return CreateTaskNotFoundError();
    }
    return CJRequestImpl::TaskStop(taskId);
}

RetReqData FfiOHOSRequestCreateTask(void *context, CConfig config)
{
    RetReqData ret = {};
    if (context == nullptr) {
        ret.err = CreateParamError();
        return ret;
    }
    return CJRequestImpl::CreateTask((OHOS::AbilityRuntime::Context *)context, &config);
}

RetTask FfiOHOSRequestGetTask(void *context, const char *taskId, RequestNativeOptionCString token)
{
    RetTask ret = {};
    if (context == nullptr) {
        ret.err = CreateParamError();
        return ret;
    }
    if (taskId == nullptr) {
        ret.err = CreateTaskNotFoundError();
        return ret;
    }
    return CJRequestImpl::GetTask((OHOS::AbilityRuntime::Context *)context, taskId, token);
}

RetError FfiOHOSRequestRemoveTask(const char *taskId)
{
    if (taskId == nullptr) {
        return CreateTaskNotFoundError();
    }
    return CJRequestImpl::RemoveTask(taskId);
}

RetTaskInfo FfiOHOSRequestShowTask(const char *taskId)
{
    RetTaskInfo ret = {};
    if (taskId == nullptr) {
        ret.err = CreateTaskNotFoundError();
        return ret;
    }
    return CJRequestImpl::ShowTask(taskId);
}

RetTaskInfo FfiOHOSRequestTouchTask(const char *taskId, char *token)
{
    RetTaskInfo ret = {};
    if (taskId == nullptr) {
        ret.err = CreateTaskNotFoundError();
        return ret;
    }
    return CJRequestImpl::TouchTask(taskId, token);
}

RetTaskArr FfiOHOSRequestSearchTask(CFilter filter)
{
    return CJRequestImpl::SearchTask(filter);
}

RetDownloadInfo FfiOHOSRequestShowDownloadTask(const char *taskId)
{
    RetDownloadInfo ret = {};
    if (taskId == nullptr) {
        ret.err = CreateTaskNotFoundError();
        return ret;
    }
    return CJRequestImpl::ShowDownloadTask(taskId);
}
}
} // namespace OHOS::CJSystemapi::Request
