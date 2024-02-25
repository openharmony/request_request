/*
 * Copyright (C) 2023 Huawei Device Co., Ltd.
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

#include "js_initialize.h"
#include "request_event.h"

#include "log.h"
#include "request_manager.h"

namespace OHOS::Request {
constexpr const std::int32_t DECIMALISM = 10;
static constexpr const char *EVENT_COMPLETED = "completed";
static constexpr const char *EVENT_FAILED = "failed";
static constexpr const char *EVENT_PAUSE = "pause";
static constexpr const char *EVENT_RESUME = "resume";
static constexpr const char *EVENT_REMOVE = "remove";
static constexpr const char *EVENT_PROGRESS = "progress";
static constexpr const char *EVENT_HEADERRECEIVE = "headerReceive";
static constexpr const char *EVENT_FAIL = "fail";
static constexpr const char *EVENT_COMPLETE = "complete";
static constexpr const char *EVENT_RESPONSE = "response";

std::unordered_set<std::string> RequestEvent::supportEventsV9_ = {
    EVENT_COMPLETE,
    EVENT_PAUSE,
    EVENT_REMOVE,
    EVENT_PROGRESS,
    EVENT_HEADERRECEIVE,
    EVENT_FAIL,
};

std::unordered_set<std::string> RequestEvent::supportEventsV10_ = {
    EVENT_PROGRESS,
    EVENT_COMPLETED,
    EVENT_FAILED,
    EVENT_PAUSE,
    EVENT_RESUME,
    EVENT_REMOVE,
    EVENT_RESPONSE,
};

std::map<std::string, RequestEvent::Event> RequestEvent::requestEvent_ = {
    { FUNCTION_PAUSE, RequestEvent::PauseExec },
    { FUNCTION_QUERY, RequestEvent::QueryExec },
    { FUNCTION_QUERY_MIME_TYPE, RequestEvent::QueryMimeTypeExec },
    { FUNCTION_REMOVE, RequestEvent::RemoveExec },
    { FUNCTION_RESUME, RequestEvent::ResumeExec },
    { FUNCTION_START, RequestEvent::StartExec },
    { FUNCTION_STOP, RequestEvent::StopExec },
};

std::map<std::string, uint32_t> RequestEvent::resMap_ = {
    { FUNCTION_PAUSE, BOOL_RES },
    { FUNCTION_QUERY, INFO_RES },
    { FUNCTION_QUERY_MIME_TYPE, STR_RES },
    { FUNCTION_REMOVE, BOOL_RES },
    { FUNCTION_RESUME, BOOL_RES },
    { FUNCTION_START, BOOL_RES },
};

std::map<State, DownloadStatus> RequestEvent::stateMap_ = {
    { State::INITIALIZED, SESSION_PENDING },
    { State::WAITING, SESSION_PAUSED },
    { State::RUNNING, SESSION_RUNNING },
    { State::RETRYING, SESSION_RUNNING },
    { State::PAUSED, SESSION_PAUSED },
    { State::COMPLETED, SESSION_SUCCESS },
    { State::STOPPED, SESSION_FAILED },
    { State::FAILED, SESSION_FAILED },
};

std::map<Reason, DownloadErrorCode> RequestEvent::failMap_ = {
    { REASON_OK, ERROR_FILE_ALREADY_EXISTS },
    { IO_ERROR, ERROR_FILE_ERROR },
    { REDIRECT_ERROR, ERROR_TOO_MANY_REDIRECTS },
    { OTHERS_ERROR, ERROR_UNKNOWN },
    { NETWORK_OFFLINE, ERROR_OFFLINE },
    { UNSUPPORTED_NETWORK_TYPE, ERROR_UNSUPPORTED_NETWORK_TYPE },
    { UNSUPPORT_RANGE_REQUEST, ERROR_UNKNOWN },
};

napi_value RequestEvent::Pause(napi_env env, napi_callback_info info)
{
    REQUEST_HILOGD("Pause in");
    return Exec(env, info, FUNCTION_PAUSE);
}

napi_value RequestEvent::Query(napi_env env, napi_callback_info info)
{
    REQUEST_HILOGD("QueryV8 in");
    return Exec(env, info, FUNCTION_QUERY);
}

napi_value RequestEvent::QueryMimeType(napi_env env, napi_callback_info info)
{
    REQUEST_HILOGD("QueryMimeType in");
    return Exec(env, info, FUNCTION_QUERY_MIME_TYPE);
}

napi_value RequestEvent::Remove(napi_env env, napi_callback_info info)
{
    REQUEST_HILOGD("RemoveV8 in");
    return Exec(env, info, FUNCTION_REMOVE);
}

napi_value RequestEvent::Resume(napi_env env, napi_callback_info info)
{
    REQUEST_HILOGD("Resume in");
    return Exec(env, info, FUNCTION_RESUME);
}

napi_value RequestEvent::Start(napi_env env, napi_callback_info info)
{
    REQUEST_HILOGD("Start in");
    return Exec(env, info, FUNCTION_START);
}

napi_value RequestEvent::Stop(napi_env env, napi_callback_info info)
{
    REQUEST_HILOGD("Stop in");
    return Exec(env, info, FUNCTION_STOP);
}

napi_value RequestEvent::On(napi_env env, napi_callback_info info)
{
    REQUEST_HILOGD("RequestEvent::On in");
    JsParam jsParam;
    ExceptionError err = ParseOnOffParameters(env, info, true, jsParam);
    if (err.code != E_OK) {
        bool withErrCode = jsParam.task->config_.version == Version::API10;
        NapiUtils::ThrowError(env, err.code, err.errInfo, withErrCode);
        return nullptr;
    }

    /* on response */
    if (jsParam.type.compare(EVENT_RESPONSE) == 0) {
        napi_status ret = jsParam.task->responseListener_->AddListener(jsParam.callback);
        if (ret != napi_ok) {
            REQUEST_HILOGE("AddListener fail");
        }
        REQUEST_HILOGD("On event %{public}s + %{public}s", jsParam.type.c_str(), jsParam.task->GetTid().c_str());
        return nullptr;
    }

    sptr<RequestNotify> listener = new (std::nothrow) RequestNotify(env, jsParam.callback);
    if (listener == nullptr) {
        REQUEST_HILOGE("Create callback object fail");
        return nullptr;
    }
    REQUEST_HILOGD("On event %{public}s + %{public}s", jsParam.type.c_str(), jsParam.task->GetTid().c_str());
    std::string key = jsParam.type + jsParam.task->GetTid();
    jsParam.task->AddListener(key, listener);
    if (jsParam.task->GetListenerSize(key) == 1) {
        RequestManager::GetInstance()->On(
            jsParam.type, jsParam.task->GetTid(), listener, jsParam.task->config_.version);
    }
    return nullptr;
}

napi_value RequestEvent::Off(napi_env env, napi_callback_info info)
{
    JsParam jsParam;
    ExceptionError err = ParseOnOffParameters(env, info, false, jsParam);
    if (err.code != E_OK) {
        bool withErrCode = jsParam.task->config_.version == Version::API10;
        NapiUtils::ThrowError(env, err.code, err.errInfo, withErrCode);
        return nullptr;
    }

    /* off response */
    if (jsParam.type.compare(EVENT_RESPONSE) == 0) {
        napi_status ret = jsParam.task->responseListener_->RemoveListener(jsParam.callback);
        if (ret != napi_ok) {
            REQUEST_HILOGE("RemoveListener fail");
        }
        return nullptr;
    }

    if (jsParam.callback == nullptr) {
        jsParam.task->RemoveListener(jsParam.type, jsParam.task->GetTid(), jsParam.task->config_.version);
    } else {
        jsParam.task->RemoveListener(
            jsParam.type, jsParam.task->GetTid(), jsParam.callback, jsParam.task->config_.version);
    }
    return nullptr;
}

bool RequestEvent::IsSupportType(const std::string &type, Version version)
{
    if (version == Version::API10) {
        return supportEventsV10_.find(type) != supportEventsV10_.end();
    } else {
        return supportEventsV9_.find(type) != supportEventsV9_.end();
    }
}

NotifyData RequestEvent::BuildNotifyData(const std::shared_ptr<TaskInfo> &taskInfo)
{
    NotifyData notifyData;
    notifyData.progress = taskInfo->progress;
    notifyData.action = taskInfo->action;
    notifyData.version = taskInfo->version;
    notifyData.mode = taskInfo->mode;
    notifyData.taskStates = taskInfo->taskStates;
    return notifyData;
}

ExceptionError RequestEvent::ParseOnOffParameters(
    napi_env env, napi_callback_info info, bool IsRequiredParam, JsParam &jsParam)
{
    ExceptionError err = { .code = E_OK };
    size_t argc = NapiUtils::MAX_ARGC;
    napi_value argv[NapiUtils::MAX_ARGC] = { nullptr };
    napi_status status = napi_get_cb_info(env, info, &argc, argv, &jsParam.self, nullptr);
    if (status != napi_ok) {
        return { .code = E_PARAMETER_CHECK, .errInfo = "Failed to obtain parameters" };
    }
    napi_unwrap(env, jsParam.self, reinterpret_cast<void **>(&jsParam.task));
    if (jsParam.task == nullptr) {
        return { .code = E_PARAMETER_CHECK, .errInfo = "Failed to obtain the current object" };
    }

    if ((IsRequiredParam && argc < NapiUtils::TWO_ARG) || (!IsRequiredParam && argc < NapiUtils::ONE_ARG)) {
        return { .code = E_PARAMETER_CHECK, .errInfo = "Wrong number of arguments" };
    }
    napi_valuetype valuetype;
    napi_typeof(env, argv[NapiUtils::FIRST_ARGV], &valuetype);
    if (valuetype != napi_string) {
        return { .code = E_PARAMETER_CHECK, .errInfo = "The first parameter is not of string type" };
    }
    jsParam.type = NapiUtils::Convert2String(env, argv[NapiUtils::FIRST_ARGV]);
    if (!IsSupportType(jsParam.type, jsParam.task->config_.version)) {
        return { .code = E_PARAMETER_CHECK, .errInfo = "First parameter error" };
    }
    ConvertType(jsParam.type);
    if (argc == NapiUtils::ONE_ARG) {
        return err;
    }
    valuetype = napi_undefined;
    napi_typeof(env, argv[NapiUtils::SECOND_ARGV], &valuetype);
    if (valuetype != napi_function) {
        return { .code = E_PARAMETER_CHECK, .errInfo = "The second parameter is not of function type" };
    }
    jsParam.callback = argv[NapiUtils::SECOND_ARGV];
    return err;
}

void RequestEvent::ConvertType(std::string &type)
{
    if (type == EVENT_COMPLETED) {
        type = EVENT_COMPLETE;
    }
    if (type == EVENT_FAILED) {
        type = EVENT_FAIL;
    }
}

napi_value RequestEvent::Exec(napi_env env, napi_callback_info info, const std::string &execType)
{
    auto context = std::make_shared<ExecContext>();
    auto input = [context](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        return ParseInputParameters(context->env_, argc, self, context);
    };
    auto output = [context, execType](napi_value *result) -> napi_status {
        if (context->innerCode_ != E_OK) {
            return napi_generic_failure;
        }
        return GetResult(context->env_, context, execType, *result);
    };
    auto exec = [context, execType]() {
        auto handle = requestEvent_.find(execType);
        if (handle != requestEvent_.end()) {
            context->innerCode_ = handle->second(context);
        }
    };

    context->SetInput(input).SetOutput(output).SetExec(exec);
    AsyncCall asyncCall(env, info, context);
    return asyncCall.Call(context, execType);
}

napi_status RequestEvent::ParseInputParameters(
    napi_env env, size_t argc, napi_value self, const std::shared_ptr<ExecContext> &context)
{
    NAPI_ASSERT_BASE(env, self != nullptr, "self is nullptr", napi_invalid_arg);
    NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&context->task)), napi_invalid_arg);
    NAPI_ASSERT_BASE(env, context->task != nullptr, "there is no native task", napi_invalid_arg);
    context->version_ = context->task->config_.version;
    context->withErrCode_ = context->version_ != Version::API8;
    return napi_ok;
}

napi_status RequestEvent::GetResult(
    napi_env env, const std::shared_ptr<ExecContext> &context, const std::string &execType, napi_value &result)
{
    if (resMap_[execType] == BOOL_RES) {
        return NapiUtils::Convert2JSValue(env, context->boolRes, result);
    }
    if (resMap_[execType] == STR_RES) {
        return NapiUtils::Convert2JSValue(env, context->strRes, result);
    }
    if (resMap_[execType] == INFO_RES) {
        return NapiUtils::Convert2JSValue(env, context->infoRes, result);
    }
    return napi_generic_failure;
}

int32_t RequestEvent::StartExec(const std::shared_ptr<ExecContext> &context)
{
    REQUEST_HILOGD("RequestEvent::StartExec in");
    JsTask* task = context->task;
    Config config = task->config_;

    // Rechecks file path.
    if (config.files.size() == 0) {
        return E_FILE_IO;
    }
    FileSpec file = config.files[0];
    if (JsInitialize::FindDir(file.uri) && config.action == Action::DOWNLOAD) {
        REQUEST_HILOGD("Found the downloaded file: %{public}s.", file.uri.c_str());
        if (chmod(file.uri.c_str(), S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH | S_IWOTH) != 0) {
            REQUEST_HILOGD("File add OTH access Failed.");
        }
        if (!JsTask::SetPathPermission(file.uri)) {
            REQUEST_HILOGE("Set path permission fail.");
            return E_FILE_IO;
        }
    }
    std::string tid = context->task->GetTid();
    JsTask::ReloadListenerByTaskId(tid);

    int32_t ret = RequestManager::GetInstance()->Start(tid);
    if (ret == E_OK) {
        context->boolRes = true;
    }
    return ret;
}

int32_t RequestEvent::StopExec(const std::shared_ptr<ExecContext> &context)
{
    int32_t ret = RequestManager::GetInstance()->Stop(context->task->GetTid());
    if (ret == E_OK) {
        context->boolRes = true;
    }
    return ret;
}

int32_t RequestEvent::PauseExec(const std::shared_ptr<ExecContext> &context)
{
    int32_t ret = RequestManager::GetInstance()->Pause(context->task->GetTid(), context->version_);
    if (ret == E_OK) {
        context->boolRes = true;
    }
    if (context->version_ != Version::API10 && ret != E_PERMISSION) {
        return E_OK;
    }
    return ret;
}

int32_t RequestEvent::QueryExec(const std::shared_ptr<ExecContext> &context)
{
    TaskInfo infoRes;
    int32_t ret = E_OK;
    if (!RequestManager::GetInstance()->LoadRequestServer()) {
        ret = E_SERVICE_ERROR;
        return ret;
    }
    ret = RequestManager::GetInstance()->Show(context->task->GetTid(), infoRes);
    if (context->version_ != Version::API10 && ret != E_PERMISSION) {
        ret = E_OK;
    }
    GetDownloadInfo(infoRes, context->infoRes);
    return ret;
}

int32_t RequestEvent::QueryMimeTypeExec(const std::shared_ptr<ExecContext> &context)
{
    int32_t ret = E_OK;
    if (!RequestManager::GetInstance()->LoadRequestServer()) {
        ret = E_SERVICE_ERROR;
        return ret;
    }
    ret = RequestManager::GetInstance()->QueryMimeType(context->task->GetTid(), context->strRes);
    if (context->version_ != Version::API10 && ret != E_PERMISSION) {
        ret = E_OK;
    }
    return ret;
}

void RequestEvent::GetDownloadInfo(const TaskInfo &infoRes, DownloadInfo &info)
{
    info.downloadId = strtoul(infoRes.tid.c_str(), NULL, DECIMALISM);
    if (infoRes.progress.state == State::FAILED) {
        auto it = failMap_.find(infoRes.code);
        if (it != failMap_.end()) {
            info.failedReason = it->second;
        } else {
            info.failedReason = ERROR_UNKNOWN;
        }
    }
    if (infoRes.progress.state == State::WAITING
        && (infoRes.code == NETWORK_OFFLINE || infoRes.code == UNSUPPORTED_NETWORK_TYPE)) {
        info.pausedReason = PAUSED_WAITING_FOR_NETWORK;
    }
    if (infoRes.progress.state == State::PAUSED) {
        if (infoRes.code == USER_OPERATION) {
            info.pausedReason = PAUSED_BY_USER;
        }
    }
    if (!infoRes.files.empty()) {
        info.fileName = infoRes.files[0].filename;
        info.filePath = infoRes.files[0].uri;
    }
    auto it = stateMap_.find(infoRes.progress.state);
    if (it != stateMap_.end()) {
        info.status = it->second;
    }
    info.url = infoRes.url;
    info.downloadTitle = infoRes.title;
    if (!infoRes.progress.sizes.empty()) {
        info.downloadTotalBytes = infoRes.progress.sizes[0];
    }
    info.description = infoRes.description;
    info.downloadedBytes = infoRes.progress.processed;
}

int32_t RequestEvent::RemoveExec(const std::shared_ptr<ExecContext> &context)
{
    int32_t ret = RequestManager::GetInstance()->Remove(context->task->GetTid(), context->version_);
    if (context->version_ != Version::API10 && ret != E_PERMISSION) {
        ret = E_OK;
    }
    if (ret == E_OK) {
        context->boolRes = true;
    }
    return ret;
}

int32_t RequestEvent::ResumeExec(const std::shared_ptr<ExecContext> &context)
{
    int32_t ret = E_OK;
    if (!RequestManager::GetInstance()->LoadRequestServer()) {
        ret = E_SERVICE_ERROR;
        return ret;
    }
    ret = RequestManager::GetInstance()->Resume(context->task->GetTid());
    if (context->version_ != Version::API10 && ret != E_PERMISSION) {
        ret = E_OK;
    }
    if (ret == E_OK) {
        context->boolRes = true;
    }
    return ret;
}
} // namespace OHOS::Request
