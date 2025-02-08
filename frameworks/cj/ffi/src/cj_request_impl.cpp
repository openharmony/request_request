/*
 * Copyright (c) 2024 Huawei Device Co., Ltd.
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

#include "cj_request_impl.h"

#include <string>
#include "constant.h"
#include "cj_request_task.h"
#include "cj_request_common.h"
#include "cj_request_log.h"
#include "cj_request_event.h"
#include "cj_initialize.h"

namespace OHOS::CJSystemapi::Request {
using OHOS::AbilityRuntime::Context;
using OHOS::Request::ExceptionErrorCode;
using OHOS::Request::Version;
using OHOS::Request::E_OK_INFO;
using OHOS::Request::E_PERMISSION_INFO;
using OHOS::Request::E_PARAMETER_CHECK_INFO;
using OHOS::Request::E_UNSUPPORTED_INFO;
using OHOS::Request::E_FILE_IO_INFO;
using OHOS::Request::E_FILE_PATH_INFO;
using OHOS::Request::E_SERVICE_ERROR_INFO;
using OHOS::Request::E_TASK_QUEUE_INFO;
using OHOS::Request::E_TASK_MODE_INFO;
using OHOS::Request::E_TASK_NOT_FOUND_INFO;
using OHOS::Request::E_TASK_STATE_INFO;
using OHOS::Request::E_OTHER_INFO;
using OHOS::Request::FUNCTION_PAUSE;
using OHOS::Request::FUNCTION_RESUME;
using OHOS::Request::FUNCTION_START;
using OHOS::Request::FUNCTION_STOP;

static constexpr const char *NOT_SYSTEM_APP = "permission verification failed, application which is not a system "
                                              "application uses system API";
static const std::map<ExceptionErrorCode, std::string> ErrorCodeToMsg{
    { ExceptionErrorCode::E_OK, E_OK_INFO },
    { ExceptionErrorCode::E_PERMISSION, E_PERMISSION_INFO },
    { ExceptionErrorCode::E_PARAMETER_CHECK, E_PARAMETER_CHECK_INFO },
    { ExceptionErrorCode::E_UNSUPPORTED, E_UNSUPPORTED_INFO },
    { ExceptionErrorCode::E_FILE_IO, E_FILE_IO_INFO },
    { ExceptionErrorCode::E_FILE_PATH, E_FILE_PATH_INFO },
    { ExceptionErrorCode::E_SERVICE_ERROR, E_SERVICE_ERROR_INFO },
    { ExceptionErrorCode::E_TASK_QUEUE, E_TASK_QUEUE_INFO },
    { ExceptionErrorCode::E_TASK_MODE, E_TASK_MODE_INFO },
    { ExceptionErrorCode::E_TASK_NOT_FOUND, E_TASK_NOT_FOUND_INFO },
    { ExceptionErrorCode::E_TASK_STATE, E_TASK_STATE_INFO },
    { ExceptionErrorCode::E_OTHER, E_OTHER_INFO },
    { ExceptionErrorCode::E_NOT_SYSTEM_APP, NOT_SYSTEM_APP }
};

RetError CJRequestImpl::Convert2RetErr(ExceptionErrorCode code)
{
    auto iter = ErrorCodeToMsg.find(code);
    std::string strMsg = (iter != ErrorCodeToMsg.end() ? iter->second : "");
    return {
        .errCode = code,
        .errMsg = MallocCString(strMsg)
    };
}

RetError CJRequestImpl::Convert2RetErr(ExceptionError &err)
{
    auto iter = ErrorCodeToMsg.find(err.code);
    std::string strMsg;
    if (err.errInfo.empty()) {
        strMsg = (iter != ErrorCodeToMsg.end() ? iter->second : "");
    } else {
        strMsg = (iter != ErrorCodeToMsg.end() ? iter->second + "   " : "") + err.errInfo;
    }
    return {
        .errCode = err.code,
        .errMsg = MallocCString(strMsg)
    };
}

std::map<std::string, std::string> CJRequestImpl::ConvertCArr2Map(const CHashStrArr *cheaders)
{
    std::map<std::string, std::string> result;
    for (int i = 0; i < cheaders->size; ++i) {
        const CHashStrPair *cheader = &cheaders->headers[i];
        result[cheader->key] = cheader->value;
    }

    return result;
}

void CJRequestImpl::Convert2Config(CConfig *config, Config &out)
{
    out.action = static_cast<OHOS::Request::Action>(config->action);
    out.url = config->url;
    out.version = Version::API10;  // CJ only support API10
    out.mode = static_cast<OHOS::Request::Mode>(config->mode);
    out.network = static_cast<OHOS::Request::Network>(config->network);
    out.index = config->index;
    out.begins = config->begins;
    out.ends = config->ends;
    out.priority = config->priority;
    out.overwrite = config->overwrite;
    out.metered = config->metered;
    out.roaming = config->roaming;
    out.retry = config->retry;
    out.redirect = config->redirect;
    out.gauge = config->gauge;
    out.precise = config->precise;
    out.title = config->title;
    out.saveas = config->saveas;
    out.method = config->method;
    out.token = config->token;
    out.description = config->description;
    out.headers =  ConvertCArr2Map(&config->headers);
    out.extras = ConvertCArr2Map(&config->extras);
}

RetReqData CJRequestImpl::CreateTask(OHOS::AbilityRuntime::Context* context, CConfig *ffiConfig)
{
    REQUEST_HILOGD("[CJRequestImpl] CreateTask start");
    Config config{};
    Convert2Config(ffiConfig, config);
    ExceptionError result = CJInitialize::ParseConfig(context, ffiConfig, config);
    if (result.code != 0) {
        return {
            .err = Convert2RetErr(result)
        };
    }

    RetReqData ret{};
    CJTask *task = new (std::nothrow) CJTask();
    if (task == nullptr) {
        REQUEST_HILOGE("[CJRequestImpl] Fail to create task.");
        ret.err.errCode = ExceptionErrorCode::E_OTHER;
        return ret;
    }
    result = task->Create(context, config);
    if (result.code != 0) {
        REQUEST_HILOGE("[CJRequestImpl] task create failed, ret:%{public}d.", result.code);
        delete task;
        return {
            .err = Convert2RetErr(result)
        };
    }

    ret.taskId = MallocCString(task->taskId_);
    REQUEST_HILOGD("[CJRequestImpl] CreateTask end");
    return ret;
}

RetError CJRequestImpl::RemoveTask(std::string taskId)
{
    RetError ret{};
    ExceptionError result = CJTask::Remove(taskId);
    if (result.code != ExceptionErrorCode::E_OK) {
        return Convert2RetErr(result);
    }

    return ret;
}

void CJRequestImpl::FreeTask(std::string taskId)
{
    REQUEST_HILOGD("[CJRequestImpl] FreeTask start");
    delete CJTask::ClearTaskMap(taskId);
}

RetError CJRequestImpl::ProgressOn(char *event, std::string taskId, void (*callback)(CProgress progress))
{
    REQUEST_HILOGD("[CJRequestImpl] ProgressOn start");
    RetError ret{};
    CJTask *task = CJTask::FindTaskById(taskId);
    if (task == nullptr) {
        REQUEST_HILOGE("[CJRequestImpl] Fail to find task, id:%{public}s.", taskId.c_str());
        return Convert2RetErr(ExceptionErrorCode::E_TASK_NOT_FOUND);
    }

    ExceptionError result = task->On(event, taskId, callback);
    if (result.code != 0) {
        REQUEST_HILOGE("[CJRequestImpl] task on failed, ret:%{public}d.", result.code);
        return Convert2RetErr(result);
    }

    return ret;
}

RetError CJRequestImpl::ProgressOff(char *event, std::string taskId, void *callback)
{
    REQUEST_HILOGD("[CJRequestImpl] ProgressOff start");
    RetError ret{};
    CJTask *task = CJTask::FindTaskById(taskId);
    if (task == nullptr) {
        REQUEST_HILOGE("[CJRequestImpl] Fail to find task, id:%{public}s.", taskId.c_str());
        return ret;
    }

    ExceptionError result = task->Off(event, callback);
    if (result.code != 0) {
        REQUEST_HILOGE("[CJRequestImpl] task off failed, ret:%{public}d.", result.code);
        return Convert2RetErr(result);
    }

    return ret;
}

RetError CJRequestImpl::TaskExec(std::string execType, std::string taskId)
{
    REQUEST_HILOGD("[CJRequestImpl] TaskExec start");
    RetError ret{};
    CJTask *task = CJTask::FindTaskById(taskId);
    if (task == nullptr) {
        REQUEST_HILOGE("[CJRequestImpl] Fail to find task, id:%{public}s.", taskId.c_str());
        return Convert2RetErr(ExceptionErrorCode::E_TASK_NOT_FOUND);
    }
    
    ExceptionErrorCode code = CJRequestEvent::Exec(execType, task);
    if (code != ExceptionErrorCode::E_OK) {
        return Convert2RetErr(code);
    }

    return ret;
}

RetError CJRequestImpl::TaskStart(std::string taskId)
{
    return CJRequestImpl::TaskExec(FUNCTION_START, taskId);
}

RetError CJRequestImpl::TaskPause(std::string taskId)
{
    return CJRequestImpl::TaskExec(FUNCTION_PAUSE, taskId);
}

RetError CJRequestImpl::TaskResume(std::string taskId)
{
    return CJRequestImpl::TaskExec(FUNCTION_RESUME, taskId);
}

RetError CJRequestImpl::TaskStop(std::string taskId)
{
    return CJRequestImpl::TaskExec(FUNCTION_STOP, taskId);
}
} // namespace OHOS::CJSystemapi::Request
