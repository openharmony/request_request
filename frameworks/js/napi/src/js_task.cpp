/*
 * Copyright (c) 2023 Huawei Device Co., Ltd.
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

#include "js_task.h"

#include <securec.h>
#include <sys/stat.h>

#include <chrono>
#include <cstring>
#include <filesystem>
#include <mutex>

#include "app_state_callback.h"
#include "async_call.h"
#include "js_initialize.h"
#include "legacy/request_manager.h"
#include "log.h"
#include "napi_base_context.h"
#include "napi_utils.h"
#include "request_event.h"
#include "request_manager.h"
#include "storage_acl.h"
#include "upload/upload_task_napiV5.h"

using namespace OHOS::StorageDaemon;
namespace fs = std::filesystem;
namespace OHOS::Request {
constexpr int64_t MILLISECONDS_IN_ONE_DAY = 24 * 60 * 60 * 1000;
std::mutex JsTask::createMutex_;
thread_local napi_ref JsTask::createCtor = nullptr;
std::mutex JsTask::requestMutex_;
thread_local napi_ref JsTask::requestCtor = nullptr;
std::mutex JsTask::requestFileMutex_;
thread_local napi_ref JsTask::requestFileCtor = nullptr;
std::mutex JsTask::getTaskCreateMutex_;
thread_local napi_ref JsTask::getTaskCreateCtor = nullptr;
std::mutex JsTask::taskMutex_;
std::map<std::string, JsTask *> JsTask::taskMap_;
bool JsTask::register_ = false;
std::mutex JsTask::pathMutex_;
std::map<std::string, int32_t> JsTask::pathMap_;
std::map<std::string, int32_t> JsTask::fileMap_;
std::mutex JsTask::taskContextMutex_;
std::map<std::string, std::shared_ptr<JsTask::ContextInfo>> JsTask::taskContextMap_;

napi_property_descriptor clzDes[] = {
    DECLARE_NAPI_FUNCTION(FUNCTION_ON, RequestEvent::On),
    DECLARE_NAPI_FUNCTION(FUNCTION_OFF, RequestEvent::Off),
    DECLARE_NAPI_FUNCTION(FUNCTION_START, RequestEvent::Start),
    DECLARE_NAPI_FUNCTION(FUNCTION_PAUSE, RequestEvent::Pause),
    DECLARE_NAPI_FUNCTION(FUNCTION_RESUME, RequestEvent::Resume),
    DECLARE_NAPI_FUNCTION(FUNCTION_STOP, RequestEvent::Stop),
};

napi_property_descriptor clzDesV9[] = {
    DECLARE_NAPI_FUNCTION(FUNCTION_ON, RequestEvent::On),
    DECLARE_NAPI_FUNCTION(FUNCTION_OFF, RequestEvent::Off),
    DECLARE_NAPI_FUNCTION(FUNCTION_SUSPEND, RequestEvent::Pause),
    DECLARE_NAPI_FUNCTION(FUNCTION_GET_TASK_INFO, RequestEvent::Query),
    DECLARE_NAPI_FUNCTION(FUNCTION_GET_TASK_MIME_TYPE, RequestEvent::QueryMimeType),
    DECLARE_NAPI_FUNCTION(FUNCTION_DELETE, RequestEvent::Remove),
    DECLARE_NAPI_FUNCTION(FUNCTION_RESTORE, RequestEvent::Resume),
    DECLARE_NAPI_FUNCTION(FUNCTION_PAUSE, RequestEvent::Pause),
    DECLARE_NAPI_FUNCTION(FUNCTION_QUERY, RequestEvent::Query),
    DECLARE_NAPI_FUNCTION(FUNCTION_QUERY_MIME_TYPE, RequestEvent::QueryMimeType),
    DECLARE_NAPI_FUNCTION(FUNCTION_REMOVE, RequestEvent::Remove),
    DECLARE_NAPI_FUNCTION(FUNCTION_RESUME, RequestEvent::Resume),
};

JsTask::~JsTask()
{
    REQUEST_HILOGD("~JsTask()");
}
napi_value JsTask::JsUpload(napi_env env, napi_callback_info info)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("Begin JsUpload, seq: %{public}d", seq);
    std::shared_ptr<Upload::UploadTaskNapiV5> proxy = std::make_shared<Upload::UploadTaskNapiV5>(env);
    if (proxy->ParseCallback(env, info)) {
        return proxy->JsUpload(env, info);
    }
    proxy->SetEnv(nullptr);

    return JsMain(env, info, Version::API8, seq);
}

napi_value JsTask::JsDownload(napi_env env, napi_callback_info info)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("Begin JsDownload, seq: %{public}d", seq);
    if (Legacy::RequestManager::IsLegacy(env, info)) {
        return Legacy::RequestManager::Download(env, info);
    }
    return JsMain(env, info, Version::API8, seq);
}

napi_value JsTask::JsRequestFile(napi_env env, napi_callback_info info)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("Begin JsRequestFile, seq: %{public}d", seq);
    return JsMain(env, info, Version::API9, seq);
}

napi_value JsTask::JsCreate(napi_env env, napi_callback_info info)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("Begin task create, seq: %{public}d", seq);
    return JsMain(env, info, Version::API10, seq);
}

napi_value JsTask::JsMain(napi_env env, napi_callback_info info, Version version, int32_t seq)
{
    auto context = std::make_shared<ContextInfo>();
    context->withErrCode_ = version != Version::API8;
    context->version_ = version;
    auto input = [context, seq](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        if (context->version_ == Version::API10) {
            napi_create_reference(context->env_, argv[1], 1, &(context->jsConfig));
        }
        napi_value ctor = GetCtor(context->env_, context->version_);
        napi_value jsTask = nullptr;
        napi_status status = napi_new_instance(context->env_, ctor, argc, argv, &jsTask);
        if (jsTask == nullptr || status != napi_ok) {
            REQUEST_HILOGE("End task create in AsyncCall input, seq: %{public}d, failed with reason:%{public}d not "
                           "napi_ok",
                seq, status);
            return napi_generic_failure;
        }
        napi_unwrap(context->env_, jsTask, reinterpret_cast<void **>(&context->task));
        napi_create_reference(context->env_, jsTask, 1, &(context->taskRef));
        return napi_ok;
    };
    auto exec = [context, seq]() {
        Config config = context->task->config_;
        context->innerCode_ = CreateExec(context, seq);
        if (context->innerCode_ == E_SERVICE_ERROR && config.version == Version::API9
            && config.action == Action::UPLOAD) {
            context->withErrCode_ = false;
        }
    };
    auto output = [context, seq](napi_value *result) -> napi_status {
        if (result == nullptr || context->innerCode_ != E_OK) {
            REQUEST_HILOGE("End task create in AsyncCall output, seq: %{public}d, failed with reason:%{public}d", seq,
                context->innerCode_);
            return napi_generic_failure;
        }
        napi_status status = napi_get_reference_value(context->env_, context->taskRef, result);
        context->task->SetTid(context->tid);
        JsTask::AddTaskMap(context->tid, context->task);
        JsTask::AddTaskContextMap(context->tid, context);
        napi_value config = nullptr;
        napi_get_reference_value(context->env_, context->jsConfig, &config);
        JsInitialize::CreatProperties(context->env_, *result, config, context->task);
        REQUEST_HILOGI("End create task successfully, seq: %{public}d, tid: %{public}s", seq, context->tid.c_str());
        return status;
    };
    context->SetInput(input).SetOutput(output).SetExec(exec);
    AsyncCall asyncCall(env, info, context);
    asyncCall.SetQosLevel(napi_qos_utility);
    return asyncCall.Call(context, "create");
}

int32_t JsTask::CreateExec(const std::shared_ptr<ContextInfo> &context, int32_t seq)
{
    REQUEST_HILOGI("Process JsTask CreateExec: Action %{public}d, Mode %{public}d, seq: %{public}d",
        context->task->config_.action, context->task->config_.mode, seq);
    if (!RequestManager::GetInstance()->LoadRequestServer()) {
        REQUEST_HILOGE("End create task in JsTask CreateExec, seq: %{public}d, failed with reason: request service "
                       "not ready",
            seq);
        return E_SERVICE_ERROR;
    }
    if (context->task->config_.mode == Mode::FOREGROUND) {
        RegisterForegroundResume();
    }
    int32_t ret = RequestManager::GetInstance()->Create(context->task->config_, seq, context->tid);
    if (ret != E_OK) {
        REQUEST_HILOGE(
            "End create task in JsTask CreateExec, seq: %{public}d, failed with reason: %{public}d", seq, ret);
        return ret;
    }
    std::string tid = context->tid;
    context->task->listenerMutex_.lock();
    context->task->notifyDataListenerMap_[SubscribeType::REMOVE] =
        std::make_shared<JSNotifyDataListener>(context->env_, tid, SubscribeType::REMOVE);
    context->task->listenerMutex_.unlock();
    RequestManager::GetInstance()->AddListener(
        tid, SubscribeType::REMOVE, context->task->notifyDataListenerMap_[SubscribeType::REMOVE]);
    return ret;
}

napi_value JsTask::GetCtor(napi_env env, Version version)
{
    switch (version) {
        case Version::API8:
            return GetCtorV8(env);
        case Version::API9:
            return GetCtorV9(env);
        case Version::API10:
            return GetCtorV10(env);
        default:
            break;
    }
    return nullptr;
}

napi_value JsTask::GetCtorV10(napi_env env)
{
    REQUEST_HILOGD("GetCtorV10 in");
    std::lock_guard<std::mutex> lock(createMutex_);
    napi_value cons;
    if (createCtor != nullptr) {
        NAPI_CALL(env, napi_get_reference_value(env, createCtor, &cons));
        return cons;
    }
    size_t count = sizeof(clzDes) / sizeof(napi_property_descriptor);
    return DefineClass(env, clzDes, count, Create, &createCtor);
}

napi_value JsTask::GetCtorV9(napi_env env)
{
    REQUEST_HILOGD("GetCtorV9 in");
    std::lock_guard<std::mutex> lock(requestFileMutex_);
    napi_value cons;
    if (requestFileCtor != nullptr) {
        NAPI_CALL(env, napi_get_reference_value(env, requestFileCtor, &cons));
        return cons;
    }
    size_t count = sizeof(clzDesV9) / sizeof(napi_property_descriptor);
    return DefineClass(env, clzDesV9, count, RequestFile, &requestFileCtor);
}

napi_value JsTask::GetCtorV8(napi_env env)
{
    REQUEST_HILOGD("GetCtorV8 in");
    std::lock_guard<std::mutex> lock(requestMutex_);
    napi_value cons;
    if (requestCtor != nullptr) {
        NAPI_CALL(env, napi_get_reference_value(env, requestCtor, &cons));
        return cons;
    }
    size_t count = sizeof(clzDesV9) / sizeof(napi_property_descriptor);
    return DefineClass(env, clzDesV9, count, RequestFileV8, &requestCtor);
}

napi_value JsTask::DefineClass(
    napi_env env, const napi_property_descriptor *desc, size_t count, napi_callback cb, napi_ref *ctor)
{
    napi_value cons = nullptr;
    napi_status status = napi_define_class(env, "Request", NAPI_AUTO_LENGTH, cb, nullptr, count, desc, &cons);
    if (status != napi_ok) {
        REQUEST_HILOGE("napi_define_class failed");
        return nullptr;
    }
    status = napi_create_reference(env, cons, 1, ctor);
    if (status != napi_ok) {
        REQUEST_HILOGE("napi_create_reference failed");
        return nullptr;
    }
    return cons;
}

napi_value JsTask::Create(napi_env env, napi_callback_info info)
{
    REQUEST_HILOGD("Create API10");
    return JsInitialize::Initialize(env, info, Version::API10);
}

napi_value JsTask::RequestFile(napi_env env, napi_callback_info info)
{
    REQUEST_HILOGD("RequestFile API9");
    return JsInitialize::Initialize(env, info, Version::API9);
}

napi_value JsTask::RequestFileV8(napi_env env, napi_callback_info info)
{
    REQUEST_HILOGD("Request API8");
    return JsInitialize::Initialize(env, info, Version::API8);
}

napi_value JsTask::GetTaskCtor(napi_env env)
{
    REQUEST_HILOGD("GetTaskCtor in");
    std::lock_guard<std::mutex> lock(getTaskCreateMutex_);
    napi_value cons;
    if (getTaskCreateCtor != nullptr) {
        NAPI_CALL(env, napi_get_reference_value(env, getTaskCreateCtor, &cons));
        return cons;
    }
    size_t count = sizeof(clzDes) / sizeof(napi_property_descriptor);
    return DefineClass(env, clzDes, count, GetTaskCreate, &getTaskCreateCtor);
}

napi_value JsTask::GetTaskCreate(napi_env env, napi_callback_info info)
{
    REQUEST_HILOGD("GetTask Create");
    return JsInitialize::Initialize(env, info, Version::API10, false);
}

napi_value JsTask::GetTask(napi_env env, napi_callback_info info)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("Begin get task, seq: %{public}d", seq);
    auto context = std::make_shared<ContextInfo>();
    context->withErrCode_ = true;
    context->version_ = Version::API10;
    auto input = [context, seq](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        ExceptionError err = ParseGetTask(context->env_, argc, argv, context);
        if (err.code != E_OK) {
            REQUEST_HILOGE(
                "End get task in AsyncCall input, seq: %{public}d, failed with reason: parse tid or token fail", seq);
            NapiUtils::ThrowError(context->env_, err.code, err.errInfo, true);
            return napi_invalid_arg;
        }
        napi_create_reference(context->env_, argv[0], 1, &(context->baseContext));
        return napi_ok;
    };
    auto output = [context, seq](napi_value *result) -> napi_status {
        if (context->innerCode_ != E_OK) {
            REQUEST_HILOGE("End get task in AsyncCall output, seq: %{public}d, failed with reason: %{public}d", seq,
                context->innerCode_);
            return napi_generic_failure;
        }
        if (!GetTaskOutput(context)) {
            REQUEST_HILOGE(
                "End get task in AsyncCall output, seq: %{public}d, failed with reason: get task output failed", seq);
            return napi_generic_failure;
        }
        napi_status res = napi_get_reference_value(context->env_, context->taskRef, result);
        context->task->SetTid(context->tid);
        napi_value conf = nullptr;
        napi_get_reference_value(context->env_, context->jsConfig, &conf);
        JsInitialize::CreatProperties(context->env_, *result, conf, context->task);
        REQUEST_HILOGI("End get task successfully, seq: %{public}d", seq);
        return res;
    };
    auto exec = [context]() {
        if (!RequestManager::GetInstance()->LoadRequestServer()) {
            context->innerCode_ = E_SERVICE_ERROR;
            return;
        }
        GetTaskExecution(context);
    };
    context->SetInput(input).SetOutput(output).SetExec(exec);
    AsyncCall asyncCall(env, info, context);
    return asyncCall.Call(context, "getTask");
}

void JsTask::GetTaskExecution(std::shared_ptr<ContextInfo> context)
{
    std::string tid = context->tid;
    REQUEST_HILOGI("Process get task, tid: %{public}s", context->tid.c_str());
    if (taskContextMap_.find(tid) != taskContextMap_.end()) {
        REQUEST_HILOGD("Find in taskContextMap_");
        if (taskContextMap_[tid]->task->config_.version != Version::API10
            || taskContextMap_[tid]->task->config_.token != context->token) {
            context->innerCode_ = E_TASK_NOT_FOUND;
            return;
        }
        context->task = taskContextMap_[tid]->task;
        context->taskRef = taskContextMap_[tid]->taskRef;
        context->jsConfig = taskContextMap_[tid]->jsConfig;
        context->innerCode_ = E_OK;
        return;
    } else {
        context->innerCode_ = RequestManager::GetInstance()->GetTask(tid, context->token, context->config);
    }
    if (context->config.version != Version::API10) {
        context->innerCode_ = E_TASK_NOT_FOUND;
    }
}

bool JsTask::GetTaskOutput(std::shared_ptr<ContextInfo> context)
{
    std::string tid = context->tid;
    if (taskMap_.find(tid) == taskMap_.end()) {
        napi_value config = NapiUtils::Convert2JSValue(context->env_, context->config);
        napi_create_reference(context->env_, config, 1, &(context->jsConfig));
        napi_value ctor = GetTaskCtor(context->env_);
        napi_value jsTask = nullptr;
        napi_value baseCtx = nullptr;
        napi_get_reference_value(context->env_, context->baseContext, &baseCtx);
        napi_value args[2] = { baseCtx, config };
        napi_status status = napi_new_instance(context->env_, ctor, 2, args, &jsTask);
        if (jsTask == nullptr || status != napi_ok) {
            REQUEST_HILOGE("Get task failed, reason: %{public}d", status);
            return false;
        }
        napi_unwrap(context->env_, jsTask, reinterpret_cast<void **>(&context->task));
        napi_create_reference(context->env_, jsTask, 1, &(context->taskRef));
        JsTask::AddTaskMap(tid, context->task);
        JsTask::AddTaskContextMap(tid, context);
    }
    return true;
}

ExceptionError JsTask::ParseGetTask(napi_env env, size_t argc, napi_value *argv, std::shared_ptr<ContextInfo> context)
{
    ExceptionError err = { .code = E_OK };
    // need at least 2 params.
    if (argc < 2) {
        REQUEST_HILOGE("Wrong number of arguments");
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Missing mandatory parameters, need at least two params, context and id";
        return err;
    }
    if (NapiUtils::GetValueType(env, argv[1]) != napi_string) {
        REQUEST_HILOGE("The parameter: tid is not of string type");
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Incorrect parameter type, tid is not of string type";
        return err;
    }
    std::string tid = NapiUtils::Convert2String(env, argv[1]);
    if (tid.empty()) {
        REQUEST_HILOGE("tid is empty");
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Parameter verification failed, tid is empty";
        return err;
    }
    // tid length <= 32
    if (tid.size() > 32) {
        REQUEST_HILOGE("tid invalid, %{public}s", tid.c_str());
        err.code = E_TASK_NOT_FOUND;
        err.errInfo = "task not found error";
        return err;
    }
    context->tid = tid;
    // handle 3rd param TOKEN
    if (argc == 3) {
        if (NapiUtils::GetValueType(env, argv[2]) != napi_string) { // argv[2] is the 3rd param
            REQUEST_HILOGE("The parameter: token is not of string type");
            err.code = E_PARAMETER_CHECK;
            err.errInfo = "Incorrect parameter type, token is not of string type";
            return err;
        }
        uint32_t bufferLen = TOKEN_MAX_BYTES + 2;
        std::unique_ptr<char[]> token = std::make_unique<char[]>(bufferLen);
        size_t len = 0;
        napi_status status = napi_get_value_string_utf8(env, argv[2], token.get(), bufferLen, &len);
        if (status != napi_ok) {
            REQUEST_HILOGE("napi get value string utf8 failed");
            memset_s(token.get(), bufferLen, 0, bufferLen);
            err.code = E_PARAMETER_CHECK;
            err.errInfo = "Parameter verification failed, get parameter token failed";
            return err;
        }
        if (len < TOKEN_MIN_BYTES || len > TOKEN_MAX_BYTES) {
            memset_s(token.get(), bufferLen, 0, bufferLen);
            err.code = E_PARAMETER_CHECK;
            err.errInfo = "Parameter verification failed, the length of token should between 8 and 2048 bytes";
            return err;
        }
        context->token = NapiUtils::SHA256(token.get(), len);
        memset_s(token.get(), bufferLen, 0, bufferLen);
    }
    return err;
}

napi_value JsTask::Remove(napi_env env, napi_callback_info info)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("Begin task remove, seq: %{public}d", seq);
    struct RemoveContext : public AsyncCall::Context {
        std::string tid;
        bool res = false;
    };

    auto context = std::make_shared<RemoveContext>();
    context->withErrCode_ = true;
    context->version_ = Version::API10;
    auto input = [context, seq](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        ExceptionError err = ParseTid(context->env_, argc, argv, context->tid);
        if (err.code != E_OK) {
            REQUEST_HILOGE("End task remove in AsyncCall input, seq: %{public}d, failed with reason: tid invalid", seq);
            NapiUtils::ThrowError(context->env_, err.code, err.errInfo, true);
            return napi_invalid_arg;
        }
        return napi_ok;
    };
    auto output = [context, seq](napi_value *result) -> napi_status {
        if (context->innerCode_ != E_OK) {
            context->res = false;
            REQUEST_HILOGE("End task remove in AsyncCall output, seq: %{public}d, failed with reason: %{public}d", seq,
                context->innerCode_);
            return napi_generic_failure;
        }
        REQUEST_HILOGI("End task remove successfully, seq: %{public}d", seq);
        return NapiUtils::Convert2JSValue(context->env_, context->res, *result);
    };
    auto exec = [context]() {
        context->innerCode_ = RequestManager::GetInstance()->Remove(context->tid, Version::API10);
    };
    context->SetInput(std::move(input)).SetOutput(std::move(output)).SetExec(std::move(exec));
    AsyncCall asyncCall(env, info, context);
    return asyncCall.Call(context, "remove");
}

ExceptionError JsTask::ParseTid(napi_env env, size_t argc, napi_value *argv, std::string &tid)
{
    ExceptionError err = { .code = E_OK };
    if (argc < 1) {
        REQUEST_HILOGE("Wrong number of arguments");
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Missing mandatory parameters, missing tid";
        return err;
    }
    if (NapiUtils::GetValueType(env, argv[0]) != napi_string) {
        REQUEST_HILOGE("The first parameter is not of string type");
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Incorrect parameter type, tid is not of string type";
        return err;
    }
    tid = NapiUtils::Convert2String(env, argv[0]);
    if (tid.empty()) {
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Parameter verification failed, tid is empty";
        return err;
    }
    return err;
}

napi_value JsTask::Show(napi_env env, napi_callback_info info)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("Begin task show, seq: %{public}d", seq);
    auto context = std::make_shared<TouchContext>();
    auto input = [context, seq](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        ExceptionError err = ParseTid(context->env_, argc, argv, context->tid);
        if (err.code != E_OK) {
            REQUEST_HILOGE("End task show in AsyncCall input, seq: %{public}d, failed with reason: tid invalid", seq);
            NapiUtils::ThrowError(context->env_, err.code, err.errInfo, true);
            return napi_invalid_arg;
        }
        // tid length <= 32
        if (context->tid.size() > 32) {
            REQUEST_HILOGE("End task show in AsyncCall input, seq: %{public}d, failed with reason: tid invalid", seq);
            NapiUtils::ThrowError(context->env_, E_TASK_NOT_FOUND, "task not found error", true);
            return napi_invalid_arg;
        }
        return napi_ok;
    };
    return TouchInner(env, info, std::move(input), std::move(context), seq);
}

napi_value JsTask::Touch(napi_env env, napi_callback_info info)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("Begin task touch, seq: %{public}d", seq);
    auto context = std::make_shared<TouchContext>();
    auto input = [context, seq](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        ExceptionError err = ParseTouch(context->env_, argc, argv, context);
        if (err.code != E_OK) {
            REQUEST_HILOGE("End task touch in AsyncCall input, seq: %{public}d, failed with reason: arg invalid", seq);
            NapiUtils::ThrowError(context->env_, err.code, err.errInfo, true);
            return napi_invalid_arg;
        }
        return napi_ok;
    };
    return TouchInner(env, info, std::move(input), std::move(context), seq);
}

napi_value JsTask::TouchInner(napi_env env, napi_callback_info info, AsyncCall::Context::InputAction input,
    std::shared_ptr<TouchContext> context, int32_t seq)
{
    context->withErrCode_ = true;
    context->version_ = Version::API10;
    auto output = [context, seq](napi_value *result) -> napi_status {
        if (context->innerCode_ != E_OK) {
            REQUEST_HILOGE("End task show in AsyncCall output, seq: %{public}d, failed with reason: %{public}d", seq,
                context->innerCode_);
            return napi_generic_failure;
        }
        *result = NapiUtils::Convert2JSValue(context->env_, context->taskInfo);
        REQUEST_HILOGI("End task show successfully, seq: %{public}d", seq);
        return napi_ok;
    };
    auto exec = [context]() {
        if (!RequestManager::GetInstance()->LoadRequestServer()) {
            context->innerCode_ = E_SERVICE_ERROR;
            return;
        }
        context->innerCode_ = RequestManager::GetInstance()->Touch(context->tid, context->token, context->taskInfo);
    };
    context->SetInput(std::move(input)).SetOutput(std::move(output)).SetExec(std::move(exec));
    AsyncCall asyncCall(env, info, context);
    return asyncCall.Call(context, "touch");
}

ExceptionError JsTask::ParseTouch(napi_env env, size_t argc, napi_value *argv, std::shared_ptr<TouchContext> context)
{
    ExceptionError err = { .code = E_OK };
    // 2 means least param num.
    if (argc < 2) {
        REQUEST_HILOGE("Wrong number of arguments");
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Missing mandatory parameters, need at least two params, id and token";
        return err;
    }
    if (NapiUtils::GetValueType(env, argv[0]) != napi_string || NapiUtils::GetValueType(env, argv[1]) != napi_string) {
        REQUEST_HILOGE("The parameter: tid is not of string type");
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Incorrect parameter type, tid is not of string type";
        return err;
    }
    context->tid = NapiUtils::Convert2String(env, argv[0]);
    if (context->tid.empty()) {
        REQUEST_HILOGE("tid is empty");
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Parameter verification failed, tid is empty";
        return err;
    }
    // tid length <= 32
    if (context->tid.size() > 32) {
        REQUEST_HILOGE("tid invalid, %{public}s", context->tid.c_str());
        err.code = E_TASK_NOT_FOUND;
        err.errInfo = "task not found error";
        return err;
    }
    uint32_t bufferLen = TOKEN_MAX_BYTES + 2;
    char *token = new char[bufferLen];
    size_t len = 0;
    napi_status status = napi_get_value_string_utf8(env, argv[1], token, bufferLen, &len);
    if (status != napi_ok) {
        REQUEST_HILOGE("napi get value string utf8 failed");
        memset_s(token, bufferLen, 0, bufferLen);
        delete[] token;
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Parameter verification failed, get token failed";
        return err;
    }
    if (len < TOKEN_MIN_BYTES || len > TOKEN_MAX_BYTES) {
        memset_s(token, bufferLen, 0, bufferLen);
        delete[] token;
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Parameter verification failed, the length of token should between 8 and 2048 bytes";
        return err;
    }
    context->token = NapiUtils::SHA256(token, len);
    memset_s(token, bufferLen, 0, bufferLen);
    delete[] token;
    return err;
}

ExceptionError JsTask::ParseSearch(napi_env env, size_t argc, napi_value *argv, Filter &filter)
{
    ExceptionError err = { .code = E_OK };
    using namespace std::chrono;
    filter.bundle = "*";
    filter.before = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    filter.after = filter.before - MILLISECONDS_IN_ONE_DAY;
    if (argc < 1) {
        return err;
    }
    napi_valuetype valueType = NapiUtils::GetValueType(env, argv[0]);
    if (valueType == napi_null || valueType == napi_undefined) {
        return err;
    }
    if (valueType != napi_object) {
        REQUEST_HILOGE("The parameter: filter is not of object type");
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Incorrect parameter type, filter is not of object type";
        return err;
    }
    filter.bundle = ParseBundle(env, argv[0]);
    filter.before = ParseBefore(env, argv[0]);
    filter.after = ParseAfter(env, argv[0], filter.before);
    if (filter.before < filter.after) {
        REQUEST_HILOGE("before is small than after");
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Parameter verification failed, filter before is small than after";
        return err;
    }
    filter.state = ParseState(env, argv[0]);
    filter.action = ParseAction(env, argv[0]);
    filter.mode = ParseMode(env, argv[0]);
    return err;
}

std::string JsTask::ParseBundle(napi_env env, napi_value value)
{
    if (!NapiUtils::HasNamedProperty(env, value, "bundle")) {
        return "*";
    }
    napi_value value1 = NapiUtils::GetNamedProperty(env, value, "bundle");
    if (NapiUtils::GetValueType(env, value1) != napi_string) {
        return "*";
    }
    return NapiUtils::Convert2String(env, value1);
}

State JsTask::ParseState(napi_env env, napi_value value)
{
    if (!NapiUtils::HasNamedProperty(env, value, "state")) {
        return State::ANY;
    }
    napi_value value1 = NapiUtils::GetNamedProperty(env, value, "state");
    if (NapiUtils::GetValueType(env, value1) != napi_number) {
        return State::ANY;
    }
    return static_cast<State>(NapiUtils::Convert2Uint32(env, value1));
}

Action JsTask::ParseAction(napi_env env, napi_value value)
{
    if (!NapiUtils::HasNamedProperty(env, value, "action")) {
        return Action::ANY;
    }
    napi_value value1 = NapiUtils::GetNamedProperty(env, value, "action");
    if (NapiUtils::GetValueType(env, value1) != napi_number) {
        return Action::ANY;
    }
    return static_cast<Action>(NapiUtils::Convert2Uint32(env, value1));
}

Mode JsTask::ParseMode(napi_env env, napi_value value)
{
    if (!NapiUtils::HasNamedProperty(env, value, "mode")) {
        return Mode::ANY;
    }
    napi_value value1 = NapiUtils::GetNamedProperty(env, value, "mode");
    if (NapiUtils::GetValueType(env, value1) != napi_number) {
        return Mode::ANY;
    }
    return static_cast<Mode>(NapiUtils::Convert2Uint32(env, value1));
}

int64_t JsTask::ParseBefore(napi_env env, napi_value value)
{
    using namespace std::chrono;
    int64_t now = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    if (!NapiUtils::HasNamedProperty(env, value, "before")) {
        return now;
    }
    napi_value value1 = NapiUtils::GetNamedProperty(env, value, "before");
    if (NapiUtils::GetValueType(env, value1) != napi_number) {
        return now;
    }
    int64_t ret = 0;
    NAPI_CALL_BASE(env, napi_get_value_int64(env, value1, &ret), now);
    return ret;
}

int64_t JsTask::ParseAfter(napi_env env, napi_value value, int64_t before)
{
    int64_t defaultValue = before - MILLISECONDS_IN_ONE_DAY;
    if (!NapiUtils::HasNamedProperty(env, value, "after")) {
        return defaultValue;
    }
    napi_value value1 = NapiUtils::GetNamedProperty(env, value, "after");
    if (NapiUtils::GetValueType(env, value1) != napi_number) {
        return defaultValue;
    }
    int64_t ret = 0;
    NAPI_CALL_BASE(env, napi_get_value_int64(env, value1, &ret), defaultValue);
    return ret;
}

napi_value JsTask::Search(napi_env env, napi_callback_info info)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("Begin task search, seq: %{public}d", seq);
    struct SearchContext : public AsyncCall::Context {
        Filter filter;
        std::vector<std::string> tids;
    };

    auto context = std::make_shared<SearchContext>();
    context->withErrCode_ = true;
    context->version_ = Version::API10;
    auto input = [context, seq](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        ExceptionError err = ParseSearch(context->env_, argc, argv, context->filter);
        if (err.code != E_OK) {
            REQUEST_HILOGE("End task search in AsyncCall input, seq: %{public}d, failed with reason: arg invalid", seq);
            NapiUtils::ThrowError(context->env_, err.code, err.errInfo, true);
            return napi_invalid_arg;
        }
        return napi_ok;
    };
    auto output = [context, seq](napi_value *result) -> napi_status {
        if (context->innerCode_ != E_OK) {
            REQUEST_HILOGE("End task search in AsyncCall output, seq: %{public}d, failed with reason: %{public}d", seq,
                context->innerCode_);
            return napi_generic_failure;
        }
        *result = NapiUtils::Convert2JSValue(context->env_, context->tids);
        REQUEST_HILOGI("End task search successfully, seq: %{public}d", seq);
        return napi_ok;
    };
    auto exec = [context]() {
        if (!RequestManager::GetInstance()->LoadRequestServer()) {
            context->innerCode_ = E_SERVICE_ERROR;
            return;
        }
        context->innerCode_ = RequestManager::GetInstance()->Search(context->filter, context->tids);
    };
    context->SetInput(std::move(input)).SetOutput(std::move(output)).SetExec(std::move(exec));
    AsyncCall asyncCall(env, info, context);
    return asyncCall.Call(context, "search");
}

napi_value JsTask::Query(napi_env env, napi_callback_info info)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("Begin task query, seq: %{public}d", seq);
    struct QueryContext : public AsyncCall::Context {
        std::string tid;
        TaskInfo taskInfo;
    };

    auto context = std::make_shared<QueryContext>();
    context->withErrCode_ = true;
    context->version_ = Version::API10;
    auto input = [context, seq](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        ExceptionError err = ParseTid(context->env_, argc, argv, context->tid);
        if (err.code != E_OK) {
            REQUEST_HILOGE("End task query in AsyncCall input, seq: %{public}d, failed with reason: tid invalid", seq);
            NapiUtils::ThrowError(context->env_, err.code, err.errInfo, true);
            return napi_invalid_arg;
        }
        return napi_ok;
    };
    auto output = [context, seq](napi_value *result) -> napi_status {
        if (context->innerCode_ != E_OK) {
            REQUEST_HILOGE("End task query in AsyncCall output, seq: %{public}d, failed with reason: %{public}d", seq,
                context->innerCode_);
            return napi_generic_failure;
        }
        context->taskInfo.withSystem = true;
        *result = NapiUtils::Convert2JSValue(context->env_, context->taskInfo);
        REQUEST_HILOGI("End task query successfully, seq: %{public}d", seq);
        return napi_ok;
    };
    auto exec = [context]() {
        if (!RequestManager::GetInstance()->LoadRequestServer()) {
            context->innerCode_ = E_SERVICE_ERROR;
            return;
        }
        context->innerCode_ = RequestManager::GetInstance()->Query(context->tid, context->taskInfo);
    };
    context->SetInput(std::move(input)).SetOutput(std::move(output)).SetExec(std::move(exec));
    AsyncCall asyncCall(env, info, context);
    return asyncCall.Call(context, "query");
}

std::string JsTask::GetTid()
{
    return tid_;
}

void JsTask::SetTid(std::string &tid)
{
    tid_ = tid;
}

void JsTask::AddTaskMap(const std::string &key, JsTask *task)
{
    std::lock_guard<std::mutex> lockGuard(JsTask::taskMutex_);
    JsTask::taskMap_[key] = task;

    if (!JsTask::taskMap_.empty()) {
        JsTask::SubscribeSA();
    }
}

void JsTask::AddTaskContextMap(const std::string &key, std::shared_ptr<ContextInfo> context)
{
    std::lock_guard<std::mutex> lockGuard(JsTask::taskContextMutex_);
    JsTask::taskContextMap_[key] = context;
}

void JsTask::SubscribeSA()
{
    REQUEST_HILOGD("SubscribeSA in");
    if (!RequestManager::GetInstance()->SubscribeSA()) {
        REQUEST_HILOGE("SubscribeSA Failed");
    }
}

void JsTask::UnsubscribeSA()
{
    REQUEST_HILOGD("UnsubscribeSA in");
    if (!RequestManager::GetInstance()->UnsubscribeSA()) {
        REQUEST_HILOGE("UnsubscribeSA Failed");
    }
}

void JsTask::ReloadListener()
{
    REQUEST_HILOGD("ReloadListener in");
    // collect all tids first to reduce lock holding time
    std::vector<std::string> tids;
    {
        std::lock_guard<std::mutex> lockGuard(JsTask::taskMutex_);
        for (const auto &it : taskMap_) {
            tids.push_back(it.first);
        }
    }
    for (const auto &it : tids) {
        RequestManager::GetInstance()->Subscribe(it);
    }
}

void JsTask::ClearTaskMap(const std::string &key)
{
    std::lock_guard<std::mutex> lockGuard(JsTask::taskMutex_);
    auto it = taskMap_.find(key);
    if (it != taskMap_.end()) {
        taskMap_.erase(it);
    }
    if (taskMap_.empty()) {
        JsTask::UnsubscribeSA();
    }
}

bool JsTask::SetDirsPermission(std::vector<std::string> &dirs)
{
    if (dirs.empty()) {
        return true;
    }
    std::string newPath = "/data/storage/el2/base/.ohos/.request/.certs";
    std::vector<std::string> dirElems;
    JsInitialize::StringSplit(newPath, '/', dirElems);
    if (!JsInitialize::CreateDirs(dirElems)) {
        REQUEST_HILOGE("CreateDirs Err: %{public}s", newPath.c_str());
        return false;
    }

    for (const auto &folderPath : dirs) {
        fs::path folder = folderPath;
        if (!(fs::exists(folder) && fs::is_directory(folder))) {
            return false;
        }
        for (const auto &entry : fs::directory_iterator(folder)) {
            fs::path path = entry.path();
            std::string existfilePath = folder.string() + "/" + path.filename().string();
            std::string newfilePath = newPath + "/" + path.filename().string();
            if (!fs::exists(newfilePath)) {
                fs::copy(existfilePath, newfilePath);
            }
            if (chmod(newfilePath.c_str(), S_IRWXU | S_IRWXG | S_IROTH | S_IXOTH) != 0) {
                REQUEST_HILOGD("File add OTH access Failed.");
            }
            REQUEST_HILOGD("current filePath is %{public}s", newfilePath.c_str());
            if (!JsTask::SetPathPermission(newfilePath)) {
                REQUEST_HILOGE("Set path permission fail.");
                return false;
            }
        }
    }
    if (!dirs.empty()) {
        dirs.clear();
        dirs.push_back(newPath);
    }
    return true;
}

bool JsTask::SetPathPermission(const std::string &filepath)
{
    std::string baseDir;
    if (!JsInitialize::CheckBelongAppBaseDir(filepath, baseDir)) {
        return false;
    }

    AddPathMap(filepath, baseDir);
    for (auto it : pathMap_) {
        if (it.second <= 0) {
            continue;
        }
        if (AclSetAccess(it.first, SA_PERMISSION_X) != ACL_SUCC) {
            REQUEST_HILOGD("AclSetAccess Parent Dir Failed: %{public}s", it.first.c_str());
        }
    }

    std::string childDir = filepath.substr(0, filepath.rfind("/"));
    if (AclSetAccess(childDir, SA_PERMISSION_RWX) != ACL_SUCC) {
        REQUEST_HILOGE("AclSetAccess Child Dir Failed: %{public}s", childDir.c_str());
        return false;
    }
    return true;
}

void JsTask::AddPathMap(const std::string &filepath, const std::string &baseDir)
{
    {
        std::lock_guard<std::mutex> lockGuard(JsTask::pathMutex_);
        auto it = fileMap_.find(filepath);
        if (it == fileMap_.end()) {
            fileMap_[filepath] = 1;
        } else {
            fileMap_[filepath] += 1;
        }
    }

    std::string childDir(filepath);
    std::string parentDir;
    while (childDir.length() > baseDir.length()) {
        parentDir = childDir.substr(0, childDir.rfind("/"));
        std::lock_guard<std::mutex> lockGuard(JsTask::pathMutex_);
        auto it = pathMap_.find(parentDir);
        if (it == pathMap_.end()) {
            pathMap_[parentDir] = 1;
        } else {
            pathMap_[parentDir] += 1;
        }
        childDir = parentDir;
    }
}

void JsTask::ResetDirAccess(const std::string &filepath)
{
    int ret = AclSetAccess(filepath, SA_PERMISSION_CLEAN);
    if (ret != ACL_SUCC) {
        REQUEST_HILOGD("AclSetAccess Reset Dir Failed: %{public}s", filepath.c_str());
    }
}

void JsTask::RemovePathMap(const std::string &filepath)
{
    std::string baseDir;
    if (!JsInitialize::CheckBelongAppBaseDir(filepath, baseDir)) {
        return;
    }

    {
        std::lock_guard<std::mutex> lockGuard(JsTask::pathMutex_);
        auto it = fileMap_.find(filepath);
        if (it != fileMap_.end()) {
            if (fileMap_[filepath] <= 1) {
                fileMap_.erase(filepath);
                if (chmod(filepath.c_str(), S_IRUSR | S_IWUSR | S_IRGRP) != 0) {
                    REQUEST_HILOGE("File remove OTH access Failed: %{public}s", filepath.c_str());
                }
            } else {
                fileMap_[filepath] -= 1;
            }
        } else {
            return;
        }
    }

    std::string childDir(filepath);
    std::string parentDir;
    while (childDir.length() > baseDir.length()) {
        parentDir = childDir.substr(0, childDir.rfind("/"));
        std::lock_guard<std::mutex> lockGuard(JsTask::pathMutex_);
        auto it = pathMap_.find(parentDir);
        if (it != pathMap_.end()) {
            if (pathMap_[parentDir] <= 1) {
                pathMap_.erase(parentDir);
                ResetDirAccess(parentDir);
            } else {
                pathMap_[parentDir] -= 1;
            }
        }
        childDir = parentDir;
    }
}

void JsTask::RemoveDirsPermission(const std::vector<std::string> &dirs)
{
    for (const auto &folderPath : dirs) {
        fs::path folder = folderPath;
        for (const auto &entry : fs::directory_iterator(folder)) {
            fs::path path = entry.path();
            std::string filePath = folder.string() + "/" + path.filename().string();
            RemovePathMap(filePath);
        }
    }
}

void JsTask::ClearTaskTemp(const std::string &tid, bool isRmFiles, bool isRmAcls, bool isRmCertsAcls, bool isRmContext)
{
    std::lock_guard<std::mutex> lockGuard(JsTask::taskContextMutex_);
    auto it = taskContextMap_.find(tid);
    if (it == taskContextMap_.end()) {
        REQUEST_HILOGD("Clear task tmp files, not in ContextMap");
        return;
    }
    auto context = it->second;

    if (isRmFiles) {
        auto bodyFileNames = context->task->config_.bodyFileNames;
        for (auto &filePath : bodyFileNames) {
            std::error_code err;
            if (!std::filesystem::exists(filePath, err)) {
                continue;
            }
            err.clear();
            RemovePathMap(filePath);
            NapiUtils::RemoveFile(filePath);
        }
    }
    if (isRmAcls) {
        // Reset Acl permission
        for (auto &file : context->task->config_.files) {
            RemovePathMap(file.uri);
        }
        context->task->isGetPermission = false;
    }
    if (isRmCertsAcls) {
        RemoveDirsPermission(context->task->config_.certsPath);
    }
    if (isRmContext) {
        taskContextMap_.erase(it);
        UnrefTaskContextMap(context);
    }
}

void JsTask::UnrefTaskContextMap(std::shared_ptr<ContextInfo> context)
{
    ContextCallbackData *data = new ContextCallbackData();
    if (data == nullptr) {
        return;
    }
    data->context = context;
    if (!UvQueue::Call(data->context->env_, static_cast<void *>(data), UvUnrefTaskContext)) {
        delete data;
    }
    return;
}

void JsTask::UvUnrefTaskContext(uv_work_t *work, int status)
{
    ContextCallbackData *data = static_cast<ContextCallbackData *>(work->data);
    if (data == nullptr) {
        // Ensure that the `work` is not nullptr.
        delete work;
        return;
    }
    napi_handle_scope scope = nullptr;
    napi_open_handle_scope(data->context->env_, &scope);
    if (scope == nullptr) {
        delete data;
        delete work;
        return;
    }
    u_int32_t taskRefCount = 0;
    napi_reference_unref(data->context->env_, data->context->taskRef, &taskRefCount);
    REQUEST_HILOGD("Unref task ref, count is %{public}d", taskRefCount);
    if (taskRefCount == 0) {
        napi_delete_reference(data->context->env_, data->context->taskRef);
    }
    if (data->context->version_ == Version::API10) {
        u_int32_t configRefCount = 0;
        napi_reference_unref(data->context->env_, data->context->jsConfig, &configRefCount);
        REQUEST_HILOGI("Unref task config ref, count is %{public}d", configRefCount);
        if (configRefCount == 0) {
            napi_delete_reference(data->context->env_, data->context->jsConfig);
        }
    }
    napi_close_handle_scope(data->context->env_, scope);
    delete data;
    delete work;
    return;
}

bool JsTask::Equals(napi_env env, napi_value value, napi_ref copy)
{
    if (copy == nullptr) {
        return (value == nullptr);
    }

    napi_value copyValue = nullptr;
    napi_get_reference_value(env, copy, &copyValue);

    bool isEquals = false;
    napi_strict_equals(env, value, copyValue, &isEquals);
    return isEquals;
}

void JsTask::RegisterForegroundResume()
{
    if (register_) {
        return;
    }
    REQUEST_HILOGI("Process register foreground resume callback");
    register_ = true;
    auto context = AbilityRuntime::ApplicationContext::GetInstance();
    if (context == nullptr) {
        REQUEST_HILOGE("End register foreground resume callback, failed with reason: Get ApplicationContext failed");
        return;
    }
    context->RegisterAbilityLifecycleCallback(std::make_shared<AppStateCallback>());
    REQUEST_HILOGI("End register foreground resume callback successfully");
}
} // namespace OHOS::Request