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

#include <cstdint>
#include <string>
#include "cj_initialize.h"
#include "cj_request_common.h"
#include "cj_request_event.h"
#include "cj_request_ffi.h"
#include "cj_request_task.h"
#include "constant.h"
#include "log.h"

namespace OHOS::CJSystemapi::Request {
using OHOS::Request::E_FILE_IO_INFO;
using OHOS::Request::E_FILE_PATH_INFO;
using OHOS::Request::E_OK_INFO;
using OHOS::Request::E_OTHER_INFO;
using OHOS::Request::E_PARAMETER_CHECK_INFO;
using OHOS::Request::E_PERMISSION_INFO;
using OHOS::Request::E_SERVICE_ERROR_INFO;
using OHOS::Request::E_TASK_MODE_INFO;
using OHOS::Request::E_TASK_NOT_FOUND_INFO;
using OHOS::Request::E_TASK_QUEUE_INFO;
using OHOS::Request::E_TASK_STATE_INFO;
using OHOS::Request::E_UNSUPPORTED_INFO;
using OHOS::Request::ExceptionErrorCode;
using OHOS::Request::Filter;
using OHOS::Request::FUNCTION_PAUSE;
using OHOS::Request::FUNCTION_RESUME;
using OHOS::Request::FUNCTION_START;
using OHOS::Request::FUNCTION_STOP;
using OHOS::Request::Reason;
using OHOS::Request::TaskInfo;
using OHOS::Request::Version;

static constexpr const char *NOT_SYSTEM_APP = "permission verification failed, application which is not a system "
                                              "application uses system API";
static const std::map<ExceptionErrorCode, std::string> ErrorCodeToMsg{
    {ExceptionErrorCode::E_OK, E_OK_INFO},
    {ExceptionErrorCode::E_PERMISSION, E_PERMISSION_INFO},
    {ExceptionErrorCode::E_PARAMETER_CHECK, E_PARAMETER_CHECK_INFO},
    {ExceptionErrorCode::E_UNSUPPORTED, E_UNSUPPORTED_INFO},
    {ExceptionErrorCode::E_FILE_IO, E_FILE_IO_INFO},
    {ExceptionErrorCode::E_FILE_PATH, E_FILE_PATH_INFO},
    {ExceptionErrorCode::E_SERVICE_ERROR, E_SERVICE_ERROR_INFO},
    {ExceptionErrorCode::E_TASK_QUEUE, E_TASK_QUEUE_INFO},
    {ExceptionErrorCode::E_TASK_MODE, E_TASK_MODE_INFO},
    {ExceptionErrorCode::E_TASK_NOT_FOUND, E_TASK_NOT_FOUND_INFO},
    {ExceptionErrorCode::E_TASK_STATE, E_TASK_STATE_INFO},
    {ExceptionErrorCode::E_OTHER, E_OTHER_INFO},
    {ExceptionErrorCode::E_NOT_SYSTEM_APP, NOT_SYSTEM_APP}};

RetError CJRequestImpl::Convert2RetErr(ExceptionErrorCode code)
{
    RetError ret = {0};
    auto iter = ErrorCodeToMsg.find(code);
    std::string strMsg = (iter != ErrorCodeToMsg.end() ? iter->second : "");
    ret.errCode = code;
    ret.errMsg = MallocCString(strMsg);
    return ret;
}

RetError CJRequestImpl::Convert2RetErr(ExceptionError &err)
{
    RetError ret = {0};
    auto iter = ErrorCodeToMsg.find(err.code);
    std::string strMsg;
    if (err.errInfo.empty()) {
        strMsg = (iter != ErrorCodeToMsg.end() ? iter->second : "");
    } else {
        strMsg = (iter != ErrorCodeToMsg.end() ? iter->second + "   " : "") + err.errInfo;
    }
    ret.errCode = err.code;
    ret.errMsg = MallocCString(strMsg);
    return ret;
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
    out.version = Version::API10; // CJ only support API10
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
    out.headers = ConvertCArr2Map(&config->headers);
    out.extras = ConvertCArr2Map(&config->extras);
}

CConfigDataTypeUion CJRequestImpl::Convert2RequestData(Action action, std::string &data,
                                                       const std::vector<FileSpec> &files,
                                                       const std::vector<FormItem> &forms)
{
    CConfigDataTypeUion res{};
    if (action == Action::DOWNLOAD) {
        res.str = MallocCString(data);
    } else {
        res.formItems = Convert2CFormItemArr(files, forms);
    }
    return res;
}

CTaskInfo CJRequestImpl::Convert2CTaskInfo(TaskInfo &task)
{
    CTaskInfo out = {NULL};

    if (task.withSystem) {
        out.uid = MallocCString(task.uid);
        out.bundle = MallocCString(task.bundle);
        task.url = "";
        task.data = "";
        if (task.action == Action::UPLOAD) {
            task.files.clear();
            task.forms.clear();
        }
    }

    out.url = MallocCString(task.url);
    out.saveas = MallocCString(GetSaveas(task.files, task.action));
    if (task.action == Action::DOWNLOAD) {
        out.data.str = MallocCString(task.data);
    } else {
        out.data.formItems = Convert2CFormItemArr(task.files, task.forms);
    }
    out.data = Convert2RequestData(task.action, task.data, task.files, task.forms);

    out.tid = MallocCString(task.tid);
    out.title = MallocCString(task.title);
    out.description = MallocCString(task.description);
    out.action = static_cast<uint32_t>(task.action);
    out.mode = static_cast<uint32_t>(task.mode);
    out.mimeType = MallocCString(task.mimeType);
    out.progress = Convert2CProgress(task.progress);
    out.gauge = task.gauge;
    out.priority = task.priority;
    out.ctime = task.ctime;
    out.mtime = task.mtime;
    out.retry = task.retry;
    out.tries = task.tries;

    if (task.code != Reason::REASON_OK) {
        out.faults = Convert2Broken(task.code);
    }

    out.reason = MallocCString(Convert2ReasonMsg(task.code));
    out.extras = Convert2CHashStrArr(task.extras);

    return out;
}

RetReqData CJRequestImpl::CreateTask(OHOS::AbilityRuntime::Context *context, CConfig *ffiConfig)
{
    REQUEST_HILOGD("[CJRequestImpl] CreateTask start");
    Config config{};
    RetReqData ret{};
    Convert2Config(ffiConfig, config);
    ExceptionError result = CJInitialize::ParseConfig(context, ffiConfig, config);
    if (result.code != 0) {
        ret.err = Convert2RetErr(result);
        return ret;
    }

    CJRequestTask *task = new (std::nothrow) CJRequestTask();
    if (task == nullptr) {
        REQUEST_HILOGE("[CJRequestImpl] Fail to create task.");
        ret.err.errCode = ExceptionErrorCode::E_OTHER;
        return ret;
    }
    result = task->Create(context, config);
    if (result.code != 0) {
        REQUEST_HILOGE("[CJRequestImpl] task create failed, ret:%{public}d.", result.code);
        delete task;
        ret.err = Convert2RetErr(result);
        return ret;
    }

    ret.taskId = MallocCString(task->taskId_);

    REQUEST_HILOGD("[CJRequestImpl] CreateTask end");
    return ret;
}

ExceptionError CJRequestImpl::ParseToken(RequestNativeOptionCString &cToken, std::string &out)
{
    ExceptionError err = {.code = ExceptionErrorCode::E_OK};
    if (!cToken.hasValue) {
        out = "null";
        return err;
    }

    size_t len = strlen(cToken.value);
    if (len < TOKEN_MIN_BYTES || len > TOKEN_MAX_BYTES) {
        err.code = ExceptionErrorCode::E_PARAMETER_CHECK;
        err.errInfo = "Parameter verification failed, the length of token should between 8 and 2048 bytes";
        return err;
    }
    if (CheckApiVersionAfter19()) {
        out = std::string(cToken.value, len);
    } else {
        out = SHA256(cToken.value, len);
    }
    return err;
}

void CJRequestImpl::Convert2CConfig(Config &in, CConfig &out)
{
    out.action = static_cast<uint32_t>(in.action);
    out.url = MallocCString(in.url);
    out.title = MallocCString(in.title);
    out.description = MallocCString(in.description);
    out.mode = static_cast<uint32_t>(in.mode);
    out.overwrite = in.overwrite;
    out.method = MallocCString(in.method);
    out.headers = Convert2CHashStrArr(in.headers);
    out.data = Convert2RequestData(in.action, in.data, in.files, in.forms);
    out.saveas = MallocCString(in.saveas);
    out.network = static_cast<uint32_t>(in.network);
    out.metered = in.metered;
    out.roaming = in.roaming;
    out.retry = in.retry;
    out.redirect = in.redirect;
    out.index = in.index;
    out.begins = in.begins;
    out.ends = in.ends;
    out.gauge = in.gauge;
    out.precise = in.precise;
    out.token = MallocCString(in.token);
    out.priority = in.priority;
    out.extras = Convert2CHashStrArr(in.extras);
}

RetTask CJRequestImpl::GetTask(OHOS::AbilityRuntime::Context *context, std::string taskId,
                               RequestNativeOptionCString &cToken)
{
    RetTask ret{};
    std::string token = "null";
    ExceptionError err = ParseToken(cToken, token);
    if (err.code != 0) {
        ret.err = Convert2RetErr(err);
        return ret;
    }
    Config out{};
    err = CJRequestTask::GetTask(context, taskId, token, out);
    if (err.code != 0) {
        ret.err = Convert2RetErr(err);
        return ret;
    }

    ret.tid.taskId = MallocCString(taskId);
    Convert2CConfig(out, ret.tid.config);
    return ret;
}

RetError CJRequestImpl::RemoveTask(std::string taskId)
{
    RetError ret{};
    ExceptionError result = CJRequestTask::Remove(taskId);
    if (result.code != ExceptionErrorCode::E_OK) {
        return Convert2RetErr(result);
    }

    return ret;
}

RetTaskInfo CJRequestImpl::ShowTask(std::string taskId)
{
    RetTaskInfo ret{};
    TaskInfo task{};
    ExceptionError result = CJRequestTask::Touch(taskId, task);
    if (result.code != ExceptionErrorCode::E_OK) {
        ret.err = Convert2RetErr(result);
        return ret;
    }

    ret.task = Convert2CTaskInfo(task);
    return ret;
}

RetTaskInfo CJRequestImpl::TouchTask(std::string taskId, const char *cToken)
{
    RetTaskInfo ret{};
    TaskInfo task{};
    std::string token = "null";
    RequestNativeOptionCString tmp = {.hasValue = (cToken != NULL), .value = cToken};
    ExceptionError err = ParseToken(tmp, token);
    if (err.code != 0) {
        ret.err = Convert2RetErr(err);
        return ret;
    }

    err = CJRequestTask::Touch(taskId, task, token);
    if (err.code != ExceptionErrorCode::E_OK) {
        ret.err = Convert2RetErr(err);
        return ret;
    }

    ret.task = Convert2CTaskInfo(task);
    return ret;
}

RequestCArrString CJRequestImpl::Convert2CStringArray(std::vector<std::string> &tids)
{
    RequestCArrString res{};
    if (tids.empty()) {
        return res;
    }

    size_t size = tids.size();
    if (size == 0 || size > std::numeric_limits<size_t>::max() / sizeof(char *)) {
        return res;
    }
    res.head = static_cast<char **>(malloc(sizeof(char *) * size));
    if (!res.head) {
        return res;
    }

    size_t i = 0;
    for (; i < size; ++i) {
        res.head[i] = MallocCString(tids[i]);
    }
    res.size = static_cast<int64_t>(i);

    return res;
}

std::string CJRequestImpl::ParseBundle(RequestNativeOptionCString &bundle)
{
    return bundle.hasValue ? bundle.value : "*";
}

int64_t CJRequestImpl::ParseBefore(RequestNativeOptionInt64 &before)
{
    using namespace std::chrono;
    int64_t now = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();

    return before.hasValue ? before.value : now;
}

constexpr int64_t MILLISECONDS_IN_ONE_DAY = 24 * 60 * 60 * 1000;
int64_t CJRequestImpl::ParseAfter(RequestNativeOptionInt64 &after, int64_t before)
{
    return after.hasValue ? after.value : (before - MILLISECONDS_IN_ONE_DAY);
}

State CJRequestImpl::ParseState(RequestNativeOptionUInt32 &state)
{
    return state.hasValue ? static_cast<State>(state.value) : State::ANY;
}

Action CJRequestImpl::ParseAction(RequestNativeOptionUInt32 &action)
{
    return action.hasValue ? static_cast<Action>(action.value) : Action::ANY;
}

Mode CJRequestImpl::ParseMode(RequestNativeOptionUInt32 &mode)
{
    return mode.hasValue ? static_cast<Mode>(mode.value) : Mode::ANY;
}

ExceptionError CJRequestImpl::Convert2Filter(CFilter &filter, Filter &out)
{
    ExceptionError err = {.code = ExceptionErrorCode::E_OK};
    out.bundle = ParseBundle(filter.bundle);
    out.before = ParseBefore(filter.before);
    out.after = ParseAfter(filter.after, out.before);
    if (out.before < out.after) {
        REQUEST_HILOGE("before is small than after");
        err.code = ExceptionErrorCode::E_PARAMETER_CHECK;
        err.errInfo = "Parameter verification failed, filter before is small than after";
        return err;
    }

    out.state = ParseState(filter.state);
    out.action = ParseAction(filter.action);
    out.mode = ParseMode(filter.mode);
    return err;
}

RetTaskArr CJRequestImpl::SearchTask(CFilter &filter)
{
    RetTaskArr ret{};
    Filter para{};
    ExceptionError result = Convert2Filter(filter, para);
    if (result.code != ExceptionErrorCode::E_OK) {
        ret.err = Convert2RetErr(result);
        return ret;
    }

    std::vector<std::string> tids;
    result = CJRequestTask::Search(para, tids);
    if (result.code != ExceptionErrorCode::E_OK) {
        ret.err = Convert2RetErr(result);
        return ret;
    }

    ret.tasks = Convert2CStringArray(tids);
    return ret;
}

void CJRequestImpl::FreeTask(std::string taskId)
{
    REQUEST_HILOGD("[CJRequestImpl] FreeTask start");
    delete CJRequestTask::ClearTaskMap(taskId);
}

RetError CJRequestImpl::ProgressOn(char *event, std::string taskId, void *callback)
{
    REQUEST_HILOGD("[CJRequestImpl] ProgressOn start");
    RetError ret{};
    CJRequestTask *task = CJRequestTask::FindTaskById(taskId);
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
    CJRequestTask *task = CJRequestTask::FindTaskById(taskId);
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
    CJRequestTask *task = CJRequestTask::FindTaskById(taskId);
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
