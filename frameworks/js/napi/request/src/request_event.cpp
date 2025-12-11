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

#include "request_event.h"

#include "constant.h"
#include "js_initialize.h"
#include "log.h"
#include "path_utils.h"
#include "request_manager.h"

namespace OHOS::Request {
constexpr const std::int32_t DECIMALISM = 10;
constexpr const int64_t MIN_SPEED_LIMIT = 16 * 1024;
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
static constexpr const char *EVENT_FAULT_OCCUR = "faultOccur";
static constexpr const char *EVENT_WAIT = "wait";

std::map<std::string, SubscribeType> RequestEvent::supportEventsV9_ = {
    { EVENT_COMPLETE, SubscribeType::COMPLETED },
    { EVENT_PAUSE, SubscribeType::PAUSE },
    { EVENT_REMOVE, SubscribeType::REMOVE },
    { EVENT_PROGRESS, SubscribeType::PROGRESS },
    { EVENT_HEADERRECEIVE, SubscribeType::HEADER_RECEIVE },
    { EVENT_FAIL, SubscribeType::FAILED },
};

std::map<std::string, SubscribeType> RequestEvent::supportEventsV10_ = {
    { EVENT_PROGRESS, SubscribeType::PROGRESS },
    { EVENT_COMPLETED, SubscribeType::COMPLETED },
    { EVENT_FAILED, SubscribeType::FAILED },
    { EVENT_PAUSE, SubscribeType::PAUSE },
    { EVENT_RESUME, SubscribeType::RESUME },
    { EVENT_REMOVE, SubscribeType::REMOVE },
    { EVENT_RESPONSE, SubscribeType::RESPONSE },
    { EVENT_FAULT_OCCUR, SubscribeType::FAULT_OCCUR },
    { EVENT_WAIT, SubscribeType::WAIT },
};

std::map<std::string, RequestEvent::Event> RequestEvent::requestEvent_ = {
    { FUNCTION_PAUSE, RequestEvent::PauseExec },
    { FUNCTION_QUERY, RequestEvent::QueryExec },
    { FUNCTION_QUERY_MIME_TYPE, RequestEvent::QueryMimeTypeExec },
    { FUNCTION_REMOVE, RequestEvent::RemoveExec },
    { FUNCTION_RESUME, RequestEvent::ResumeExec },
    { FUNCTION_START, RequestEvent::StartExec },
    { FUNCTION_STOP, RequestEvent::StopExec },
    { FUNCTION_SET_MAX_SPEED, RequestEvent::SetMaxSpeedExec },
};

std::map<std::string, uint32_t> RequestEvent::resMap_ = {
    { FUNCTION_PAUSE, BOOL_RES },
    { FUNCTION_QUERY, INFO_RES },
    { FUNCTION_QUERY_MIME_TYPE, STR_RES },
    { FUNCTION_REMOVE, BOOL_RES },
    { FUNCTION_RESUME, BOOL_RES },
    { FUNCTION_START, BOOL_RES },
    { FUNCTION_STOP, BOOL_RES },
    { FUNCTION_SET_MAX_SPEED, BOOL_RES },
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
    { INSUFFICIENT_SPACE, ERROR_INSUFFICIENT_SPACE },
    { REDIRECT_ERROR, ERROR_TOO_MANY_REDIRECTS },
    { OTHERS_ERROR, ERROR_UNKNOWN },
    { NETWORK_OFFLINE, ERROR_OFFLINE },
    { UNSUPPORTED_NETWORK_TYPE, ERROR_UNSUPPORTED_NETWORK_TYPE },
    { UNSUPPORT_RANGE_REQUEST, ERROR_UNKNOWN },
};

napi_value RequestEvent::Pause(napi_env env, napi_callback_info info)
{
    return Exec(env, info, FUNCTION_PAUSE);
}

napi_value RequestEvent::Query(napi_env env, napi_callback_info info)
{
    return Exec(env, info, FUNCTION_QUERY);
}

napi_value RequestEvent::QueryMimeType(napi_env env, napi_callback_info info)
{
    return Exec(env, info, FUNCTION_QUERY_MIME_TYPE);
}

napi_value RequestEvent::Remove(napi_env env, napi_callback_info info)
{
    return Exec(env, info, FUNCTION_REMOVE);
}

napi_value RequestEvent::Resume(napi_env env, napi_callback_info info)
{
    return Exec(env, info, FUNCTION_RESUME);
}

napi_value RequestEvent::Start(napi_env env, napi_callback_info info)
{
    return Exec(env, info, FUNCTION_START);
}

napi_value RequestEvent::Stop(napi_env env, napi_callback_info info)
{
    return Exec(env, info, FUNCTION_STOP);
}

napi_value RequestEvent::SetMaxSpeed(napi_env env, napi_callback_info info)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGD("Begin task set max speed, seq: %{public}d", seq);
    std::string execType = FUNCTION_SET_MAX_SPEED;
    auto context = std::make_shared<ExecContext>();
    auto input = [context, seq, info](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        napi_status ret = ParseInputParameters(context->env_, argc, self, context);
        if (ret != napi_ok) {
            REQUEST_HILOGE("End task set max speed, seq: %{public}d, failed: %{public}d", seq, ret);
            return ret;
        }
        int64_t minSpeed = context->task->config_.minSpeed.speed;
        ExceptionError err = ParseSetMaxSpeedParameters(context->env_, self, info, minSpeed, context->maxSpeed);
        if (err.code != E_OK) {
            REQUEST_HILOGE("End task set max speed, seq: %{public}d, failed: %{public}d, maxSpeed: %{public}d", seq,
                err.code, static_cast<int32_t>(context->maxSpeed));
            NapiUtils::ThrowError(context->env_, err.code, err.errInfo, true);
            return napi_invalid_arg;
        }
        return napi_ok;
    };
    auto output = [context, execType, seq](napi_value *result) -> napi_status {
        if (context->innerCode_ != E_OK) {
            REQUEST_HILOGE("End task %{public}s in AsyncCall output, seq: %{public}d, failed: %{public}d",
                execType.c_str(), seq, context->innerCode_);
            return napi_generic_failure;
        }

        napi_status status = GetResult(context->env_, context, execType, *result);
        if (status != napi_ok) {
            REQUEST_HILOGE("End task %{public}s in AsyncCall output, seq: %{public}d, failed: %{public}d",
                execType.c_str(), seq, status);
        } else {
            REQUEST_HILOGI("%{public}s ok seq %{public}d", execType.c_str(), seq);
        }
        return status;
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

napi_value RequestEvent::On(napi_env env, napi_callback_info info)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGD("Begin task on, seq: %{public}d", seq);
    JsParam jsParam;
    ExceptionError err = ParseOnOffParameters(env, info, true, jsParam);
    if (err.code != E_OK) {
        bool withErrCode = jsParam.task->config_.version == Version::API10;
        REQUEST_HILOGE("End task on, seq: %{public}d, failed: %{public}d", seq, err.code);
        NapiUtils::ThrowError(env, err.code, err.errInfo, withErrCode);
        return nullptr;
    }

    if (jsParam.subscribeType == SubscribeType::RESPONSE) {
        jsParam.task->listenerMutex_.lock();
        if (jsParam.task->responseListener_ == nullptr) {
            jsParam.task->responseListener_ = std::make_shared<JSResponseListener>(env, jsParam.task->GetTid());
        }
        jsParam.task->listenerMutex_.unlock();
        napi_status ret = jsParam.task->responseListener_->AddListener(jsParam.callback);
        if (ret != napi_ok) {
            REQUEST_HILOGE("End task on, seq: %{public}d, failed: AddListener fail code %{public}d", seq, ret);
            return nullptr;
        }
    } else {
        jsParam.task->listenerMutex_.lock();
        auto listener = jsParam.task->notifyDataListenerMap_.find(jsParam.subscribeType);
        if (listener == jsParam.task->notifyDataListenerMap_.end()) {
            jsParam.task->notifyDataListenerMap_[jsParam.subscribeType] =
                std::make_shared<JSNotifyDataListener>(env, jsParam.task->GetTid(), jsParam.subscribeType);
        }
        jsParam.task->listenerMutex_.unlock();
        napi_status ret = jsParam.task->notifyDataListenerMap_[jsParam.subscribeType]->AddListener(jsParam.callback);
        if (ret != napi_ok) {
            REQUEST_HILOGE("End task on, seq: %{public}d, failed: AddListener fail code %{public}d", seq, ret);
            return nullptr;
        }
    }

    REQUEST_HILOGI("%{public}s on %{public}s", jsParam.task->GetTid().c_str(), jsParam.type.c_str());
    return nullptr;
}

napi_value RequestEvent::Off(napi_env env, napi_callback_info info)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGD("Begin task off, seq %{public}d", seq);
    JsParam jsParam;
    ExceptionError err = ParseOnOffParameters(env, info, false, jsParam);
    if (err.code != E_OK) {
        bool withErrCode = jsParam.task->config_.version == Version::API10;
        REQUEST_HILOGE("End task off, seq: %{public}d, failed: %{public}d", seq, err.code);
        NapiUtils::ThrowError(env, err.code, err.errInfo, withErrCode);
        return nullptr;
    }

    if (jsParam.subscribeType == SubscribeType::RESPONSE) {
        jsParam.task->listenerMutex_.lock();
        if (jsParam.task->responseListener_ == nullptr) {
            jsParam.task->responseListener_ = std::make_shared<JSResponseListener>(env, jsParam.task->GetTid());
        }
        jsParam.task->listenerMutex_.unlock();
        napi_status ret = jsParam.task->responseListener_->RemoveListener(jsParam.callback);
        if (ret != napi_ok) {
            REQUEST_HILOGE("End task off, seq: %{public}d, failed: RemoveListener fail code %{public}d", seq, ret);
            return nullptr;
        }
    } else {
        jsParam.task->listenerMutex_.lock();
        auto listener = jsParam.task->notifyDataListenerMap_.find(jsParam.subscribeType);
        if (listener == jsParam.task->notifyDataListenerMap_.end()) {
            jsParam.task->notifyDataListenerMap_[jsParam.subscribeType] =
                std::make_shared<JSNotifyDataListener>(env, jsParam.task->GetTid(), jsParam.subscribeType);
        }
        jsParam.task->listenerMutex_.unlock();
        napi_status ret = jsParam.task->notifyDataListenerMap_[jsParam.subscribeType]->RemoveListener(jsParam.callback);
        if (ret != napi_ok) {
            REQUEST_HILOGE("End task off, seq: %{public}d, failed: RemoveListener fail code %{public}d", seq, ret);
            return nullptr;
        }
    }

    REQUEST_HILOGD("End task off %{public}s ok, seq %{public}d tid %{public}s", jsParam.type.c_str(), seq,
        jsParam.task->GetTid().c_str());
    return nullptr;
}

SubscribeType RequestEvent::StringToSubscribeType(const std::string &type, Version version)
{
    if (version == Version::API10) {
        if (supportEventsV10_.find(type) != supportEventsV10_.end()) {
            return supportEventsV10_[type];
        }
    } else {
        if (supportEventsV9_.find(type) != supportEventsV9_.end()) {
            return supportEventsV9_[type];
        }
    }
    return SubscribeType::BUTT;
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

ExceptionError RequestEvent::ParseSetMaxSpeedParameters(
    napi_env env, napi_value self, napi_callback_info info, int64_t minSpeed, int64_t &maxSpeed)
{
    ExceptionError err = { .code = E_OK };
    size_t argc = NapiUtils::MAX_ARGC;
    napi_value argv[NapiUtils::MAX_ARGC] = { nullptr };
    if (argc < NapiUtils::ONE_ARG) {
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Missing mandatory parameters, Wrong number of arguments";
        return err;
    }
    napi_status status = napi_get_cb_info(env, info, &argc, argv, &self, nullptr);
    if (status != napi_ok) {
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Parameter verification failed, Failed to obtain parameters";
        return err;
    }

    napi_valuetype valuetype;
    napi_typeof(env, argv[NapiUtils::FIRST_ARGV], &valuetype);
    if (valuetype != napi_number) {
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Incorrect parameter type, speed is not of number type";
        return err;
    }

    maxSpeed = NapiUtils::Convert2Int64(env, argv[NapiUtils::FIRST_ARGV]);
    if (maxSpeed < MIN_SPEED_LIMIT) {
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Incorrect parameter value, minimum speed value is 16 KB/s";
    }

    if (maxSpeed < minSpeed) {
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Incorrect parameter value, max speed value is less than min speed";
    }

    return err;
}

ExceptionError RequestEvent::ParseOnOffParameters(
    napi_env env, napi_callback_info info, bool IsRequiredParam, JsParam &jsParam)
{
    ExceptionError err = { .code = E_OK };
    size_t argc = NapiUtils::MAX_ARGC;
    napi_value argv[NapiUtils::MAX_ARGC] = { nullptr };
    napi_status status = napi_get_cb_info(env, info, &argc, argv, &jsParam.self, nullptr);
    if (status != napi_ok) {
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Parameter verification failed, Failed to obtain parameters";
        return err;
    }
    napi_unwrap(env, jsParam.self, reinterpret_cast<void **>(&jsParam.task));
    if (jsParam.task == nullptr) {
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Parameter verification failed, Failed to obtain the current object";
        return err;
    }

    if ((IsRequiredParam && argc < NapiUtils::TWO_ARG) || (!IsRequiredParam && argc < NapiUtils::ONE_ARG)) {
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Missing mandatory parameters, Wrong number of arguments";
        return err;
    }
    napi_valuetype valuetype;
    napi_typeof(env, argv[NapiUtils::FIRST_ARGV], &valuetype);
    if (valuetype != napi_string) {
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Incorrect parameter type, event is not of string type";
        return err;
    }
    jsParam.type = NapiUtils::Convert2String(env, argv[NapiUtils::FIRST_ARGV]);
    jsParam.subscribeType = StringToSubscribeType(jsParam.type, jsParam.task->config_.version);
    if (jsParam.subscribeType == SubscribeType::BUTT) {
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Parameter verification failed, event parse error";
        return err;
    }
    if (argc == NapiUtils::ONE_ARG) {
        return err;
    }
    valuetype = napi_undefined;
    napi_typeof(env, argv[NapiUtils::SECOND_ARGV], &valuetype);
    if (valuetype != napi_function) {
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Incorrect parameter type, callback is not of function type";
        return err;
    }
    jsParam.callback = argv[NapiUtils::SECOND_ARGV];
    return err;
}

napi_value RequestEvent::Exec(napi_env env, napi_callback_info info, const std::string &execType)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("Begin task %{public}s seq %{public}d", execType.c_str(), seq);
    auto context = std::make_shared<ExecContext>();
    auto input = [context](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        return ParseInputParameters(context->env_, argc, self, context);
    };
    auto output = [context, execType, seq](napi_value *result) -> napi_status {
        if (context->innerCode_ != E_OK) {
            REQUEST_HILOGE("End task %{public}s in AsyncCall output, seq: %{public}d, failed: %{public}d",
                execType.c_str(), seq, context->innerCode_);
            return napi_generic_failure;
        }

        napi_status status = GetResult(context->env_, context, execType, *result);
        if (status != napi_ok) {
            REQUEST_HILOGE("End task %{public}s in AsyncCall output, seq: %{public}d, failed: %{public}d",
                execType.c_str(), seq, status);
        } else {
            REQUEST_HILOGI("End task %{public}s ok seq %{public}d", execType.c_str(), seq);
        }
        return status;
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
    auto res = resMap_.find(execType);
    if (res == resMap_.end()) {
        return napi_generic_failure;
    }
    if (res->second == BOOL_RES) {
        return NapiUtils::Convert2JSValue(env, context->boolRes, result);
    }
    if (res->second == STR_RES) {
        return NapiUtils::Convert2JSValue(env, context->strRes, result);
    }
    if (res->second == INFO_RES) {
        return NapiUtils::Convert2JSValue(env, context->infoRes, result);
    }
    return napi_generic_failure;
}

int32_t RequestEvent::StartExec(const std::shared_ptr<ExecContext> &context)
{
    REQUEST_HILOGD("RequestEvent::StartExec in");
    JsTask *task = context->task;
    Config config = task->config_;

    // Rechecks file path.
    if (config.files.size() == 0) {
        return E_FILE_IO;
    }
    FileSpec file = config.files[0];
    if (JsInitialize::FindDir(file.uri) && config.action == Action::DOWNLOAD && !task->isGetPermission) {
        REQUEST_HILOGD("Found the downloaded file");
        if (!PathUtils::AddPathsToMap(file.uri, config.action)) {
            REQUEST_HILOGE("Set path permission fail.");
            return E_FILE_IO;
        }
    }
    std::string tid = context->task->GetTid();
    {
        std::lock_guard<std::mutex> lockGuard(JsTask::taskMutex_);
        auto it = JsTask::taskContextMap_.find(tid);
        if (it == JsTask::taskContextMap_.end() || it->second->task == nullptr) {
            REQUEST_HILOGE("Start taskContextMap_ not find %{public}s.", tid.c_str());
            // In JS d.ts, only can throw 201/13400003/21900007（E_TASK_STATE）
            return E_TASK_STATE;
        }
    }

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

int32_t RequestEvent::SetMaxSpeedExec(const std::shared_ptr<ExecContext> &context)
{
    int32_t ret = RequestManager::GetInstance()->SetMaxSpeed(context->task->GetTid(), context->maxSpeed);
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
