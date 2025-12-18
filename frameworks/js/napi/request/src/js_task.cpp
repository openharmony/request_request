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
#include <new>

#include "app_state_callback.h"
#include "async_call.h"
#include "constant.h"
#include "js_initialize.h"
#include "js_native_api_types.h"
#include "legacy/request_manager.h"
#include "log.h"
#include "napi_base_context.h"
#include "napi_utils.h"
#include "path_utils.h"
#include "request_common.h"
#include "request_event.h"
#include "request_manager.h"
#include "storage_acl.h"
#include "sys_event.h"
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
std::map<std::string, std::shared_ptr<JsTask::ContextInfo>> JsTask::taskContextMap_;
bool JsTask::register_ = false;

napi_property_descriptor clzDes[] = {
    DECLARE_NAPI_FUNCTION(FUNCTION_ON, RequestEvent::On),
    DECLARE_NAPI_FUNCTION(FUNCTION_OFF, RequestEvent::Off),
    DECLARE_NAPI_FUNCTION(FUNCTION_START, RequestEvent::Start),
    DECLARE_NAPI_FUNCTION(FUNCTION_PAUSE, RequestEvent::Pause),
    DECLARE_NAPI_FUNCTION(FUNCTION_RESUME, RequestEvent::Resume),
    DECLARE_NAPI_FUNCTION(FUNCTION_STOP, RequestEvent::Stop),
    DECLARE_NAPI_FUNCTION(FUNCTION_SET_MAX_SPEED, RequestEvent::SetMaxSpeed),
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
    REQUEST_HILOGI("JsUpload seq %{public}d", seq);
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
    REQUEST_HILOGI("JsDownload seq %{public}d", seq);
    if (Legacy::RequestManager::IsLegacy(env, info)) {
        return Legacy::RequestManager::Download(env, info);
    }
    return JsMain(env, info, Version::API8, seq);
}

napi_value JsTask::JsRequestFile(napi_env env, napi_callback_info info)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("Js seq %{public}d", seq);
    return JsMain(env, info, Version::API9, seq);
}

napi_value JsTask::JsCreate(napi_env env, napi_callback_info info)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("create seq %{public}d", seq);
    return JsMain(env, info, Version::API10, seq);
}

napi_status JsTask::CreateInput(std::shared_ptr<ContextInfo> context, const int32_t seq, size_t argc, napi_value *argv)
{
    napi_value ctor = GetCtor(context->env_, context->version_);
    napi_value jsTask = nullptr;
    napi_status status = napi_new_instance(context->env_, ctor, argc, argv, &jsTask);
    if (jsTask == nullptr || status != napi_ok) {
        REQUEST_HILOGE("End task create input, seq: %{public}d, failed:%{public}d", seq, status);
        return napi_generic_failure;
    }
    status = napi_unwrap(context->env_, jsTask, reinterpret_cast<void **>(&context->task));
    if (status != napi_ok) {
        return status;
    }
    status = napi_create_reference(context->env_, jsTask, NapiUtils::ONE_REF, &(context->taskRef));
    if (status != napi_ok) {
        return status;
    }
    if (context->version_ == Version::API10) {
        status = napi_set_named_property(context->env_, jsTask, "config", argv[1]);
        if (status != napi_ok) {
            return status;
        }
    }
    return napi_ok;
}

napi_value JsTask::JsMain(napi_env env, napi_callback_info info, Version version, int32_t seq)
{
    auto context = std::make_shared<ContextInfo>();
    context->withErrCode_ = version != Version::API8;
    context->version_ = version;
    auto input = [context, seq](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        return JsTask::CreateInput(context, seq, argc, argv);
    };
    auto exec = [context, seq]() {
        Config config = context->task->config_;
        context->innerCode_ = CreateExec(context, seq);
        if (context->innerCode_ == E_SERVICE_ERROR && config.version == Version::API9
            && config.action == Action::UPLOAD) {
            context->withErrCode_ = false;
        }
        if (config.version == Version::API8 && context->innerCode_ == E_PERMISSION) {
            context->withErrCode_ = true;
        }
    };
    auto output = [context, seq](napi_value *result) -> napi_status {
        if (result == nullptr || context->innerCode_ != E_OK) {
            REQUEST_HILOGE(
                "End task create in AsyncCall output, seq: %{public}d, failed:%{public}d", seq, context->innerCode_);
            return napi_generic_failure;
        }
        napi_status status = napi_get_reference_value(context->env_, context->taskRef, result);
        context->task->SetTid(context->tid);
        JsTask::AddTaskWhenCreate(context);
        if (context->version_ == Version::API10) {
            NapiUtils::SetStringPropertyUtf8(context->env_, *result, "tid", context->tid);
        }
        REQUEST_HILOGI("End create seq %{public}d, tid %{public}s", seq, context->tid.c_str());
        return status;
    };
    context->SetInput(input).SetOutput(output).SetExec(exec);
    AsyncCall asyncCall(env, info, context);
    asyncCall.SetQosLevel(napi_qos_utility);
    return asyncCall.Call(context, "create");
}

// Only used in create.
void JsTask::AddTaskWhenCreate(std::shared_ptr<ContextInfo> context)
{
    std::lock_guard<std::mutex> lockGuard(JsTask::taskMutex_);
    auto [it, inserted] = JsTask::taskContextMap_.try_emplace(context->tid, context);
    // Unreachable.
    if (!inserted) {
        REQUEST_HILOGE("CAddContext Exist %{public}s", context->tid.c_str());
    }
    if (!JsTask::taskContextMap_.empty()) {
        JsTask::SubscribeSA();
    }
}

int32_t JsTask::CreateExec(const std::shared_ptr<ContextInfo> &context, int32_t seq)
{
    REQUEST_HILOGD("JsTask CreateExec: Action %{public}d, Mode %{public}d, seq: %{public}d",
        context->task->config_.action, context->task->config_.mode, seq);

    if (context->task->config_.mode == Mode::FOREGROUND) {
        RegisterForegroundResume();
    }
    // Authorize the path
    int32_t err = JsTask::AuthorizePath(context->task->config_);
    if (err != E_OK) {
        return err;
    }

    int32_t ret = RequestManager::GetInstance()->Create(context->task->config_, seq, context->tid);
    if (ret != E_OK) {
        REQUEST_HILOGE("End create task in JsTask CreateExec, seq: %{public}d, failed: %{public}d", seq, ret);
        return ret;
    }
    JsTask::AddRemoveListener(context);
    return ret;
}

int32_t JsTask::AuthorizePath(const Config &config)
{
    if (config.action == Action::DOWNLOAD) {
        FileSpec fileSpec = config.files[0];
        if (fileSpec.isUserFile) {
            return E_OK;
        }
        if (!PathUtils::AddPathsToMap(fileSpec.uri, config.action)) {
            REQUEST_HILOGE("Add Path acl failed, %{public}s", PathUtils::ShieldPath(fileSpec.uri).c_str());
            return E_FILE_IO;
        }
    } else {
        for (auto &fileSpec : config.files) {
            if (fileSpec.isUserFile) {
                continue;
            }
            if (!PathUtils::AddPathsToMap(fileSpec.uri, config.action)) {
                return E_FILE_IO;
            }
        }

        for (auto &path : config.bodyFileNames) {
            // bodyFileNames need rw.
            if (!PathUtils::AddPathsToMap(path, Action::DOWNLOAD)) {
                return E_FILE_IO;
            }
        }
    }
    return E_OK;
}

void JsTask::AddRemoveListener(const std::shared_ptr<ContextInfo> &context)
{
    std::string tid = context->tid;
    context->task->listenerMutex_.lock();
    context->task->notifyDataListenerMap_[SubscribeType::REMOVE] =
        std::make_shared<JSNotifyDataListener>(context->env_, tid, SubscribeType::REMOVE);
    context->task->listenerMutex_.unlock();
    RequestManager::GetInstance()->AddListener(
        tid, SubscribeType::REMOVE, context->task->notifyDataListenerMap_[SubscribeType::REMOVE]);
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
    REQUEST_HILOGI("GetTask seq %{public}d", seq);
    auto context = std::make_shared<ContextInfo>();
    context->withErrCode_ = true;
    context->version_ = Version::API10;
    auto input = [context, seq](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        ExceptionError err = ParseGetTask(context->env_, argc, argv, context);
        if (err.code != E_OK) {
            REQUEST_HILOGE("End get task in AsyncCall input, seq: %{public}d, failed: parse task failed", seq);
            NapiUtils::ThrowError(context->env_, err.code, err.errInfo, true);
            return napi_invalid_arg;
        }
        napi_status status = napi_create_reference(context->env_, argv[0], NapiUtils::ONE_REF, &(context->baseContext));
        return status;
    };
    auto output = [context, seq](napi_value *result) -> napi_status {
        if (context->innerCode_ != E_OK) {
            REQUEST_HILOGE(
                "End get task in AsyncCall output, seq: %{public}d, failed: %{public}d", seq, context->innerCode_);
            return napi_generic_failure;
        }
        if (GetTaskOutput(context, result, seq) != napi_ok) {
            REQUEST_HILOGE("End get task in AsyncCall output, seq: %{public}d, failed: get task output failed", seq);
            return napi_generic_failure;
        }
        REQUEST_HILOGI("End GetTask %{public}s", context->tid.c_str());
        return napi_ok;
    };
    auto exec = [context]() { GetTaskExecution(context); };
    context->SetInput(input).SetOutput(output).SetExec(exec);
    AsyncCall asyncCall(env, info, context);
    return asyncCall.Call(context, "getTask");
}

void JsTask::GetTaskExecution(std::shared_ptr<ContextInfo> context)
{
    // Set context->config.
    {
        std::lock_guard<std::mutex> lockGuard(JsTask::taskMutex_);
        std::string tid = context->tid;
        auto it = taskContextMap_.find(tid);
        if (it != taskContextMap_.end() && it->second->task != nullptr) {
            context->config = it->second->config;
            return;
        }
    }
    std::string tid = context->tid;
    REQUEST_HILOGD("Process get task, tid: %{public}s", context->tid.c_str());
    Config &config = context->config;
    context->innerCode_ = RequestManager::GetInstance()->GetTask(tid, context->token, config);
    if (config.action == Action::DOWNLOAD && config.files.size() != 0) {
        config.saveas = config.files[0].uri;
    }
    if (context->innerCode_ == E_OK) {
        // Authorize the path
        int32_t err = JsTask::AuthorizePath(config);
        if (err != E_OK) {
            context->innerCode_ = err;
            return;
        }
    }

    if (context->config.version != Version::API10 || context->config.token != context->token) {
        context->innerCode_ = E_TASK_NOT_FOUND;
        return;
    }
}

napi_status JsTask::CtorJsTask(std::shared_ptr<ContextInfo> context, napi_value *result)
{
    napi_value jsConfig = NapiUtils::Convert2JSValueConfig(context->env_, context->config);
    napi_value ctor = GetTaskCtor(context->env_);
    napi_value baseCtx = nullptr;
    napi_status status = napi_generic_failure;
    status = napi_get_reference_value(context->env_, context->baseContext, &baseCtx);

    napi_value args[2] = { baseCtx, jsConfig };
    status = napi_new_instance(context->env_, ctor, NapiUtils::TWO_ARG, args, result);
    if (status != napi_ok || result == nullptr) {
        REQUEST_HILOGE("Get task failed, reason: %{public}d", status);
        return napi_generic_failure;
    }
    status = napi_unwrap(context->env_, *result, reinterpret_cast<void **>(&context->task));
    if (status != napi_ok) {
        return status;
    }
    context->task->SetTid(context->tid);
    if (context->version_ == Version::API10) {
        status = napi_set_named_property(context->env_, *result, "config", jsConfig);
        if (status != napi_ok) {
            return status;
        }
        NapiUtils::SetStringPropertyUtf8(context->env_, *result, "tid", context->tid);
    }
    status = napi_create_reference(context->env_, *result, NapiUtils::ONE_REF, &(context->taskRef));
    return status;
}

napi_status JsTask::CheckTaskInMap(
    std::shared_ptr<ContextInfo> context, std::shared_ptr<ContextInfo> mapContext, napi_value *result)
{
    context->taskRef = nullptr;
    if (context->env_ != mapContext->env_) {
        REQUEST_HILOGE("getTask in different envs %{public}s", context->tid.c_str());
        context->innerCode_ = E_PARAMETER_CHECK;
        return napi_generic_failure;
    }
    context->task = mapContext->task;
    napi_status status = napi_get_reference_value(context->env_, mapContext->taskRef, result);
    if (status != napi_ok) {
        return status;
    }
    context->contextIf = false;
    return napi_ok;
}

napi_status JsTask::GetTaskOutput(std::shared_ptr<ContextInfo> context, napi_value *result, int32_t seq)
{
    std::lock_guard<std::mutex> lockGuard(JsTask::taskMutex_);
    std::string tid = context->tid;
    auto it = taskContextMap_.find(tid);
    if (it != taskContextMap_.end() && it->second->task != nullptr) {
        return JsTask::CheckTaskInMap(context, it->second, result);
    }
    if (context->contextIf) {
        context->innerCode_ = E_PARAMETER_CHECK;
        REQUEST_HILOGE("End get task in AsyncCall output failed by error context");
        return napi_generic_failure;
    }
    napi_status status = JsTask::CtorJsTask(context, result);
    if (status != napi_ok) {
        JsTask::DeleteContextTaskRef(context);
        return status;
    }
    auto [itContext, insertedContext] = JsTask::taskContextMap_.try_emplace(context->tid, context);
    if (!insertedContext) {
        REQUEST_HILOGE("GAddContext Exist %{public}s", context->tid.c_str());
    }
    if (!JsTask::taskContextMap_.empty()) {
        JsTask::SubscribeSA();
    }
    JsTask::AddRemoveListener(context);
    return napi_ok;
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
    std::shared_ptr<OHOS::AbilityRuntime::Context> runtimeContext = nullptr;
    napi_status getStatus = JsInitialize::GetContext(env, argv[0], runtimeContext);
    if (getStatus != napi_ok) {
        REQUEST_HILOGE("GetTask context fail");
        context->contextIf = true;
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
        context->token = std::string(token.get(), len);
        memset_s(token.get(), bufferLen, 0, bufferLen);
    }
    return err;
}

napi_value JsTask::Remove(napi_env env, napi_callback_info info)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("Begin remove seq %{public}d", seq);
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
            REQUEST_HILOGE("End task remove in AsyncCall input, seq: %{public}d, failed: tid invalid", seq);
            NapiUtils::ThrowError(context->env_, err.code, err.errInfo, true);
            return napi_invalid_arg;
        }
        return napi_ok;
    };
    auto output = [context, seq](napi_value *result) -> napi_status {
        if (context->innerCode_ != E_OK) {
            context->res = false;
            REQUEST_HILOGE(
                "End task remove in AsyncCall output, seq: %{public}d, failed: %{public}d", seq, context->innerCode_);
            return napi_generic_failure;
        }
        REQUEST_HILOGI("End remove seq %{public}d", seq);
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
    REQUEST_HILOGI("Begin show seq %{public}d", seq);
    auto context = std::make_shared<TouchContext>();
    auto input = [context, seq](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        ExceptionError err = ParseTid(context->env_, argc, argv, context->tid);
        if (err.code != E_OK) {
            REQUEST_HILOGE("End task show in AsyncCall input, seq: %{public}d, failed: tid invalid", seq);
            NapiUtils::ThrowError(context->env_, err.code, err.errInfo, true);
            return napi_invalid_arg;
        }
        // tid length <= 32
        if (context->tid.size() > 32) {
            REQUEST_HILOGE("End task show in AsyncCall input, seq: %{public}d, failed: tid invalid", seq);
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
    REQUEST_HILOGI("Begin touch seq %{public}d", seq);
    auto context = std::make_shared<TouchContext>();
    auto input = [context, seq](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        ExceptionError err = ParseTouch(context->env_, argc, argv, context);
        if (err.code != E_OK) {
            REQUEST_HILOGE("End task touch in AsyncCall input, seq: %{public}d, failed: arg invalid", seq);
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
            REQUEST_HILOGE(
                "End task show in AsyncCall output, seq: %{public}d, failed: %{public}d", seq, context->innerCode_);
            return napi_generic_failure;
        }
        *result = NapiUtils::Convert2JSValue(context->env_, context->taskInfo);
        REQUEST_HILOGI("End show seq %{public}d", seq);
        return napi_ok;
    };
    auto exec = [context]() {
        context->innerCode_ = RequestManager::GetInstance()->Touch(context->tid, context->token, context->taskInfo);
    };
    context->SetInput(std::move(input)).SetOutput(std::move(output)).SetExec(std::move(exec));
    AsyncCall asyncCall(env, info, context);
    return asyncCall.Call(context, "touch");
}

bool JsTask::ParseTouchCheck(const napi_env env, const size_t argc, const napi_value *argv,
    const std::shared_ptr<TouchContext> context, ExceptionError &err)
{
    // 2 means least param num.
    if (argc < 2) {
        REQUEST_HILOGE("Wrong number of arguments");
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Missing mandatory parameters, need at least two params, id and token";
        return false;
    }
    if (NapiUtils::GetValueType(env, argv[0]) != napi_string || NapiUtils::GetValueType(env, argv[1]) != napi_string) {
        REQUEST_HILOGE("The parameter: tid is not of string type");
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Incorrect parameter type, tid is not of string type";
        return false;
    }
    context->tid = NapiUtils::Convert2String(env, argv[0]);
    if (context->tid.empty()) {
        REQUEST_HILOGE("tid is empty");
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Parameter verification failed, tid is empty";
        return false;
    }
    // tid length <= 32
    if (context->tid.size() > 32) {
        REQUEST_HILOGE("tid invalid, %{public}s", context->tid.c_str());
        err.code = E_TASK_NOT_FOUND;
        err.errInfo = "task not found error";
        return false;
    }
    return true;
}

ExceptionError JsTask::ParseTouch(napi_env env, size_t argc, napi_value *argv, std::shared_ptr<TouchContext> context)
{
    ExceptionError err = { .code = E_OK };
    if (!JsTask::ParseTouchCheck(env, argc, argv, context, err)) {
        return err;
    }
    uint32_t bufferLen = TOKEN_MAX_BYTES + 2;
    char *token = new (std::nothrow) char[bufferLen];
    if (token == nullptr) {
        err.code = E_OTHER;
        err.errInfo = "cannot new token";
        return err;
    }
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
    context->token = std::string(token, len);
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
    REQUEST_HILOGI("Begin search seq %{public}d", seq);
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
            REQUEST_HILOGE("End task search in AsyncCall input, seq: %{public}d, failed: arg invalid", seq);
            NapiUtils::ThrowError(context->env_, err.code, err.errInfo, true);
            return napi_invalid_arg;
        }
        return napi_ok;
    };
    auto output = [context, seq](napi_value *result) -> napi_status {
        if (context->innerCode_ != E_OK) {
            REQUEST_HILOGE(
                "End task search in AsyncCall output, seq: %{public}d, failed: %{public}d", seq, context->innerCode_);
            return napi_generic_failure;
        }
        *result = NapiUtils::Convert2JSValue(context->env_, context->tids);
        REQUEST_HILOGI("End search seq %{public}d", seq);
        return napi_ok;
    };
    auto exec = [context]() {
        context->innerCode_ = RequestManager::GetInstance()->Search(context->filter, context->tids);
    };
    context->SetInput(std::move(input)).SetOutput(std::move(output)).SetExec(std::move(exec));
    AsyncCall asyncCall(env, info, context);
    return asyncCall.Call(context, "search");
}

napi_value JsTask::Query(napi_env env, napi_callback_info info)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("Begin query seq %{public}d", seq);
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
            REQUEST_HILOGE("End task query in AsyncCall input, seq: %{public}d, failed: tid invalid", seq);
            NapiUtils::ThrowError(context->env_, err.code, err.errInfo, true);
            return napi_invalid_arg;
        }
        return napi_ok;
    };
    auto output = [context, seq](napi_value *result) -> napi_status {
        if (context->innerCode_ != E_OK) {
            REQUEST_HILOGE(
                "End task query in AsyncCall output, seq: %{public}d, failed: %{public}d", seq, context->innerCode_);
            return napi_generic_failure;
        }
        context->taskInfo.withSystem = true;
        *result = NapiUtils::Convert2JSValue(context->env_, context->taskInfo);
        REQUEST_HILOGI("End query seq %{public}d", seq);
        return napi_ok;
    };
    auto exec = [context]() {
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
        for (const auto &it : taskContextMap_) {
            tids.push_back(it.first);
        }
    }
    for (const auto &it : tids) {
        REQUEST_HILOGD("ReloadListener tid: %{public}s", it.c_str());
        RequestManager::GetInstance()->Subscribe(it);
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
        REQUEST_HILOGE("CreateDirs Error");
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
            // Certs only need read permission.
            if (!PathUtils::AddPathsToMap(newfilePath, Action::UPLOAD)) {
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

void JsTask::RemoveDirsPermission(const std::vector<std::string> &dirs)
{
    for (const auto &folderPath : dirs) {
        fs::path folder = folderPath;
        for (const auto &entry : fs::directory_iterator(folder)) {
            fs::path path = entry.path();
            std::string filePath = folder.string() + "/" + path.filename().string();
            PathUtils::SubPathsToMap(filePath);
        }
    }
}

void JsTask::ClearTaskTemp(const std::string &tid, bool isRmFiles, bool isRmAcls, bool isRmCertsAcls)
{
    std::lock_guard<std::mutex> lockGuard(JsTask::taskMutex_);
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
            PathUtils::SubPathsToMap(filePath);
            NapiUtils::RemoveFile(filePath);
        }
    }
    if (isRmAcls) {
        // Reset Acl permission
        for (auto &file : context->task->config_.files) {
            PathUtils::SubPathsToMap(file.uri);
        }
        context->task->isGetPermission = false;
    }
    if (isRmCertsAcls) {
        RemoveDirsPermission(context->task->config_.certsPath);
    }
}

void JsTask::RemoveTaskContext(const std::string &tid)
{
    std::lock_guard<std::mutex> lockGuard(JsTask::taskMutex_);
    auto it = taskContextMap_.find(tid);
    if (it == taskContextMap_.end()) {
        REQUEST_HILOGD("Clear task tmp files, not in ContextMap");
        return;
    }
    auto context = it->second;

    auto map = context->task->notifyDataListenerMap_;
    for (auto i = map.begin(); i != map.end(); i++) {
        i->second->DeleteAllListenerRef();
    }
    map.clear();
    taskContextMap_.erase(it);
    if (taskContextMap_.empty()) {
        JsTask::UnsubscribeSA();
    }
    DeleteContextTaskRef(context);
}

void JsTask::DeleteContextTaskRef(std::shared_ptr<ContextInfo> context)
{
    ContextCallbackData *data = new ContextCallbackData();
    if (data == nullptr) {
        return;
    }
    data->context = context;
    auto callback = [data]() {
        if (data == nullptr) {
            return;
        }
        if (data->context == nullptr || data->context->env_ == nullptr || data->context->taskRef == nullptr) {
            delete data;
            return;
        }
        napi_handle_scope scope = nullptr;
        napi_status status = napi_open_handle_scope(data->context->env_, &scope);
        if (status != napi_ok || scope == nullptr) {
            REQUEST_HILOGE("UnrefTask napi_scope failed");
            delete data;
            return;
        }
        status = napi_delete_reference(data->context->env_, data->context->taskRef);
        if (status != napi_ok) {
            delete data;
            return;
        }
        data->context->taskRef = nullptr;
        status = napi_close_handle_scope(data->context->env_, scope);
        if (status != napi_ok) {
            REQUEST_HILOGE("UnrefTask napi_close_handle_scope failed");
        }
        delete data;
        return;
    };

    int32_t ret = napi_send_event(data->context->env_, callback, napi_eprio_high,
        "request:download|downloadfile|upload|uploadfile|agent.create");
    if (ret != napi_ok) {
        REQUEST_HILOGE("napi_send_event failed: %{public}d", ret);
        delete data;
    }
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
        REQUEST_HILOGE("End register foreground resume callback, failed: Get ApplicationContext failed");
        SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_00, "Register failed get AppContext");
        return;
    }
    context->RegisterAbilityLifecycleCallback(std::make_shared<AppStateCallback>());
    REQUEST_HILOGI("End register foreground resume callback successfully");
}
} // namespace OHOS::Request