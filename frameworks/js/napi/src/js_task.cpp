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
#include "app_state_callback.h"

#include <chrono>
#include <cstring>
#include <mutex>
#include <securec.h>
#include <sys/stat.h>
#include <filesystem>

#include "async_call.h"
#include "js_initialize.h"
#include "legacy/request_manager.h"
#include "log.h"
#include "napi_base_context.h"
#include "napi_utils.h"
#include "request_event.h"
#include "request_manager.h"
#include "upload/upload_task_napiV5.h"
#include "storage_acl.h"

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
    ClearListener();
}
napi_value JsTask::JsUpload(napi_env env, napi_callback_info info)
{
    REQUEST_HILOGD("JsUpload in");
    std::shared_ptr<Upload::UploadTaskNapiV5> proxy = std::make_shared<Upload::UploadTaskNapiV5>(env);
    if (proxy->ParseCallback(env, info)) {
        return proxy->JsUpload(env, info);
    }
    proxy->SetEnv(nullptr);
    return JsMain(env, info, Version::API8);
}

napi_value JsTask::JsDownload(napi_env env, napi_callback_info info)
{
    REQUEST_HILOGD("JsDownload in");
    if (Legacy::RequestManager::IsLegacy(env, info)) {
        return Legacy::RequestManager::Download(env, info);
    }
    return JsMain(env, info, Version::API8);
}

napi_value JsTask::JsRequestFile(napi_env env, napi_callback_info info)
{
    REQUEST_HILOGD("JsRequestFile in");
    return JsMain(env, info, Version::API9);
}

napi_value JsTask::JsCreate(napi_env env, napi_callback_info info)
{
    REQUEST_HILOGD("JsCreate in");
    return JsMain(env, info, Version::API10);
}

napi_value JsTask::JsMain(napi_env env, napi_callback_info info, Version version)
{
    auto context = std::make_shared<ContextInfo>();
    context->withErrCode_ = version != Version::API8;
    context->version_ = version;
    auto input = [context](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        if (context->version_ == Version::API10) {
            napi_create_reference(context->env_, argv[1], 1, &(context->jsConfig));
        }
        napi_value ctor = GetCtor(context->env_, context->version_);
        napi_value jsTask = nullptr;
        napi_status status = napi_new_instance(context->env_, ctor, argc, argv, &jsTask);
        if (jsTask == nullptr || status != napi_ok) {
            REQUEST_HILOGE("Get jsTask failed");
            return napi_generic_failure;
        }
        napi_unwrap(context->env_, jsTask, reinterpret_cast<void **>(&context->task));
        napi_create_reference(context->env_, jsTask, 1, &(context->taskRef));
        return napi_ok;
    };
    auto exec = [context]() {
        Config config = context->task->config_;
        context->innerCode_ = CreateExec(context);
        if (context->innerCode_ == E_SERVICE_ERROR && config.version == Version::API9
            && config.action == Action::UPLOAD) {
            context->withErrCode_ = false;
        }
    };
    auto output = [context](napi_value *result) -> napi_status {
        if (result == nullptr || context->innerCode_ != E_OK) {
            return napi_generic_failure;
        }
        napi_status status = napi_get_reference_value(context->env_, context->taskRef, result);
        context->task->SetTid(context->tid);
        JsTask::AddTaskMap(std::to_string(context->tid), context->task);
        JsTask::AddTaskContextMap(std::to_string(context->tid), context);
        napi_value config = nullptr;
        napi_get_reference_value(context->env_, context->jsConfig, &config);
        JsInitialize::CreatProperties(context->env_, *result, config, context->task);
        REQUEST_HILOGD("JsMain output");
        return status;
    };
    context->SetInput(input).SetOutput(output).SetExec(exec);
    AsyncCall asyncCall(env, info, context);
    asyncCall.SetQosLevel(napi_qos_utility);
    return asyncCall.Call(context, "create");
}

int32_t JsTask::CreateExec(const std::shared_ptr<ContextInfo> &context)
{
    if (!RequestManager::GetInstance()->LoadRequestServer()) {
        return E_SERVICE_ERROR;
    }
    if (context ->task->config_.mode == Mode::FOREGROUND) {
        RegisterForegroundResume();
    }
    sptr<RequestNotify> listener = new RequestNotify();
    std::string key = "done" + context->task->GetTid();
    context->task->AddListener(key, listener);
    return RequestManager::GetInstance()->Create(context->task->config_, context->tid, listener);
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
    auto context = std::make_shared<ContextInfo>();
    context->withErrCode_ = true;
    context->version_ = Version::API10;
    auto input = [context](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        if (!ParseGetTask(context->env_, argc, argv, context)) {
            NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, "Parse tid or token fail!", true);
            return napi_invalid_arg;
        }
        napi_create_reference(context->env_, argv[0], 1, &(context->baseContext));
        return napi_ok;
    };
    auto output = [context](napi_value *result) -> napi_status {
        if (context->innerCode_ != E_OK) {
            return napi_generic_failure;
        }
        if (!GetTaskOutput(context)) {
            return napi_generic_failure;
        }
        napi_status res = napi_get_reference_value(context->env_, context->taskRef, result);
        context->task->SetTid(context->tid);
        napi_value conf = nullptr;
        napi_get_reference_value(context->env_, context->jsConfig, &conf);
        JsInitialize::CreatProperties(context->env_, *result, conf, context->task);
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
    std::string tid = std::to_string(context->tid);
        if (taskContextMap_.find(tid) != taskContextMap_.end()) {
            REQUEST_HILOGD("Find in taskContextMap_");
            if (taskContextMap_[tid]->task->config_.version != Version::API10 ||
                taskContextMap_[tid]->task->config_.token != context->token) {
                context->innerCode_ = E_TASK_NOT_FOUND;
                return;
            }
            context->task = taskContextMap_[tid]->task;
            context->taskRef = taskContextMap_[tid]->taskRef;
            context->jsConfig = taskContextMap_[tid]->jsConfig;
            context->innerCode_ = E_OK;
            return;
        } else {
            context->innerCode_ =
                RequestManager::GetInstance()->GetTask(tid, context->token, context->config);
        }
        if (context->config.version != Version::API10) {
            context->innerCode_ = E_TASK_NOT_FOUND;
        }
}

bool JsTask::GetTaskOutput(std::shared_ptr<ContextInfo> context)
{
    std::string tid = std::to_string(context->tid);
    if (taskMap_.find(tid) == taskMap_.end()) {
        napi_value config = NapiUtils::Convert2JSValue(context->env_, context->config);
        napi_create_reference(context->env_, config, 1, &(context->jsConfig));
        napi_value ctor = GetTaskCtor(context->env_);
        napi_value jsTask = nullptr;
        napi_value baseCtx = nullptr;
        napi_get_reference_value(context->env_, context->baseContext, &baseCtx);
        napi_value args[2] = {baseCtx, config};
        napi_status status = napi_new_instance(context->env_, ctor, 2, args, &jsTask);
        if (jsTask == nullptr || status != napi_ok) {
            REQUEST_HILOGE("Get task failed");
            return false;
        }
        napi_unwrap(context->env_, jsTask, reinterpret_cast<void **>(&context->task));
        napi_create_reference(context->env_, jsTask, 1, &(context->taskRef));
        JsTask::AddTaskMap(tid, context->task);
        JsTask::AddTaskContextMap(tid, context);
    }
    return true;
}

bool JsTask::ParseGetTask(napi_env env, size_t argc, napi_value *argv, std::shared_ptr<ContextInfo> context)
{
    // need at least 2 params.
    if (argc < 2) {
        REQUEST_HILOGE("Wrong number of arguments");
        return false;
    }
    if (NapiUtils::GetValueType(env, argv[1]) != napi_string) {
        REQUEST_HILOGE("The parameter is not of string type");
        return false;
    }
    std::string tid = NapiUtils::Convert2String(env, argv[1]);
    if (tid.empty()) {
        REQUEST_HILOGE("tid is empty");
        return false;
    }
    context->tid = std::stoi(tid);
    // handle 3rd param TOKEN
    if (argc == 3) {
        if (NapiUtils::GetValueType(env, argv[2]) != napi_string) {     // argv[2] is the 3rd param
            REQUEST_HILOGE("The parameter is not of string type");
            return false;
        }
        uint32_t bufferLen = TOKEN_MAX_BYTES + 2;
        std::unique_ptr<char[]> token = std::make_unique<char[]>(bufferLen);
        size_t len = 0;
        napi_status status = napi_get_value_string_utf8(env, argv[2], token.get(), bufferLen, &len);
        if (status != napi_ok) {
            REQUEST_HILOGE("napi get value string utf8 failed");
            memset_s(token.get(), bufferLen, 0, bufferLen);
            return false;
        }
        if (len < TOKEN_MIN_BYTES || len > TOKEN_MAX_BYTES) {
            memset_s(token.get(), bufferLen, 0, bufferLen);
            return false;
        }
        context->token = NapiUtils::SHA256(token.get(), len);
        memset_s(token.get(), bufferLen, 0, bufferLen);
    }
    return true;
}

napi_value JsTask::Remove(napi_env env, napi_callback_info info)
{
    struct RemoveContext : public AsyncCall::Context {
        std::string tid;
        bool res = false;
    };

    auto context = std::make_shared<RemoveContext>();
    context->withErrCode_ = true;
    context->version_ = Version::API10;
    auto input = [context](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        context->tid = ParseTid(context->env_, argc, argv);
        if (context->tid.empty()) {
            NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, "Parse tid fail!", true);
            return napi_invalid_arg;
        }
        return napi_ok;
    };
    auto output = [context](napi_value *result) -> napi_status {
        if (context->innerCode_ != E_OK) {
            context->res = false;
            return napi_generic_failure;
        }
        return NapiUtils::Convert2JSValue(context->env_, context->res, *result);
    };
    auto exec = [context]() {
        context->innerCode_ = RequestManager::GetInstance()->Remove(context->tid, Version::API10);
        // Removed Task can not return notify, so unref in this.
        JsTask::ClearTaskContext(context->tid);
    };
    context->SetInput(std::move(input)).SetOutput(std::move(output)).SetExec(std::move(exec));
    AsyncCall asyncCall(env, info, context);
    return asyncCall.Call(context, "remove");
}

std::string JsTask::ParseTid(napi_env env, size_t argc, napi_value *argv)
{
    if (argc < 1) {
        REQUEST_HILOGE("Wrong number of arguments");
        return "";
    }
    if (NapiUtils::GetValueType(env, argv[0]) != napi_string) {
        REQUEST_HILOGE("The first parameter is not of string type");
        return "";
    }
    return NapiUtils::Convert2String(env, argv[0]);
}

napi_value JsTask::Show(napi_env env, napi_callback_info info)
{
    auto context = std::make_shared<TouchContext>();
    auto input = [context](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        context->tid = ParseTid(context->env_, argc, argv);
        if (context->tid.empty()) {
            NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, "Parse tid fail!", true);
            return napi_invalid_arg;
        }
        return napi_ok;
    };
    return TouchInner(env, info, std::move(input), std::move(context));
}

napi_value JsTask::Touch(napi_env env, napi_callback_info info)
{
    auto context = std::make_shared<TouchContext>();
    auto input = [context](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        bool ret = ParseTouch(context->env_, argc, argv, context);
        if (!ret) {
            NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, "Parse tid or token fail!", true);
            return napi_invalid_arg;
        }
        return napi_ok;
    };
    return TouchInner(env, info, std::move(input), std::move(context));
}

napi_value JsTask::TouchInner(napi_env env, napi_callback_info info, AsyncCall::Context::InputAction input,
    std::shared_ptr<TouchContext> context)
{
    context->withErrCode_ = true;
    context->version_ = Version::API10;
    auto output = [context](napi_value *result) -> napi_status {
        if (context->innerCode_ != E_OK) {
            return napi_generic_failure;
        }
        *result = NapiUtils::Convert2JSValue(context->env_, context->taskInfo);
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

bool JsTask::ParseTouch(napi_env env, size_t argc, napi_value *argv, std::shared_ptr<TouchContext> context)
{
    // 2 means least param num.
    if (argc < 2) {
        REQUEST_HILOGE("Wrong number of arguments");
        return false;
    }
    if (NapiUtils::GetValueType(env, argv[0]) != napi_string || NapiUtils::GetValueType(env, argv[1]) != napi_string) {
        REQUEST_HILOGE("The parameter is not of string type");
        return false;
    }
    context->tid = NapiUtils::Convert2String(env, argv[0]);
    if (context->tid.empty()) {
        REQUEST_HILOGE("tid is empty");
        return false;
    }
    uint32_t bufferLen = TOKEN_MAX_BYTES + 2;
    char *token = new char[bufferLen];
    size_t len = 0;
    napi_status status = napi_get_value_string_utf8(env, argv[1], token, bufferLen, &len);
    if (status != napi_ok) {
        REQUEST_HILOGE("napi get value string utf8 failed");
        memset_s(token, bufferLen, 0, bufferLen);
        delete[] token;
        return false;
    }
    if (len < TOKEN_MIN_BYTES || len > TOKEN_MAX_BYTES) {
        memset_s(token, bufferLen, 0, bufferLen);
        delete[] token;
        return false;
    }
    context->token = NapiUtils::SHA256(token, len);
    memset_s(token, bufferLen, 0, bufferLen);
    delete[] token;
    return true;
}

bool JsTask::ParseSearch(napi_env env, size_t argc, napi_value *argv, Filter &filter)
{
    using namespace std::chrono;
    filter.bundle = "*";
    filter.before = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    filter.after = filter.before - MILLISECONDS_IN_ONE_DAY;
    if (argc < 1) {
        return true;
    }
    napi_valuetype valueType = NapiUtils::GetValueType(env, argv[0]);
    if (valueType == napi_null || valueType == napi_undefined) {
        return true;
    }
    if (valueType != napi_object) {
        REQUEST_HILOGE("The parameter is not of object type");
        return false;
    }
    filter.bundle = ParseBundle(env, argv[0]);
    filter.before = ParseBefore(env, argv[0]);
    filter.after = ParseAfter(env, argv[0], filter.before);
    if (filter.before < filter.after) {
        REQUEST_HILOGE("before is small than after");
        return false;
    }
    filter.state = ParseState(env, argv[0]);
    filter.action = ParseAction(env, argv[0]);
    filter.mode = ParseMode(env, argv[0]);
    return true;
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
    struct SearchContext : public AsyncCall::Context {
        Filter filter;
        std::vector<std::string> tids;
    };

    auto context = std::make_shared<SearchContext>();
    context->withErrCode_ = true;
    context->version_ = Version::API10;
    auto input = [context](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        bool ret = ParseSearch(context->env_, argc, argv, context->filter);
        if (!ret) {
            NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, "Parse filter fail!", true);
            return napi_invalid_arg;
        }
        return napi_ok;
    };
    auto output = [context](napi_value *result) -> napi_status {
        if (context->innerCode_ != E_OK) {
            return napi_generic_failure;
        }
        *result = NapiUtils::Convert2JSValue(context->env_, context->tids);
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
    struct QueryContext : public AsyncCall::Context {
        std::string tid;
        TaskInfo taskInfo;
    };

    auto context = std::make_shared<QueryContext>();
    context->withErrCode_ = true;
    context->version_ = Version::API10;
    auto input = [context](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        context->tid = ParseTid(context->env_, argc, argv);
        if (context->tid.empty()) {
            NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, "Parse tid fail!", true);
            return napi_invalid_arg;
        }
        return napi_ok;
    };
    auto output = [context](napi_value *result) -> napi_status {
        if (context->innerCode_ != E_OK) {
            return napi_generic_failure;
        }
        context->taskInfo.withSystem = true;
        *result = NapiUtils::Convert2JSValue(context->env_, context->taskInfo);
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

void JsTask::SetTid(int32_t tid)
{
    tid_ = std::to_string(tid);
}

size_t JsTask::GetListenerSize(const std::string &key)
{
    std::lock_guard<std::mutex> autoLock(listenerMutex_);
    auto it = listenerMap_.find(key);
    if (it == listenerMap_.end()) {
        return 0;
    }
    REQUEST_HILOGD("listenerMap_ size %{public}zu", it->second.size());
    return it->second.size();
}

void JsTask::AddTaskMap(const std::string &key, JsTask *task)
{
    std::lock_guard<std::mutex> lockGuard(JsTask::taskMutex_);
    JsTask::taskMap_[key] = task;
}

void JsTask::AddTaskContextMap(const std::string &key, std::shared_ptr<ContextInfo> context)
{
    std::lock_guard<std::mutex> lockGuard(JsTask::taskContextMutex_);
    JsTask::taskContextMap_[key] = context;
}

void JsTask::AddListener(const std::string &key, const sptr<RequestNotify> &listener)
{
    REQUEST_HILOGD("AddListener key %{public}s", key.c_str());
    std::lock_guard<std::mutex> autoLock(listenerMutex_);
    listenerMap_[key].push_back(listener);
}

void JsTask::RemoveListener(const std::string &type, const std::string &tid, napi_value callback, Version version)
{
    std::string key = type + tid;
    std::lock_guard<std::mutex> autoLock(listenerMutex_);
    auto it = listenerMap_.find(key);
    if (it == listenerMap_.end()) {
        return;
    }
    for (auto item = it->second.begin(); item != it->second.end(); item++) {
        if (Equals((*item)->env_, callback, (*item)->ref_)) {
            listenerMap_[key].erase(item);
            break;
        }
    }
    if (listenerMap_[key].empty()) {
        RequestManager::GetInstance()->Off(type, tid, version);
        listenerMap_.erase(key);
    }
}

void JsTask::RemoveListener(const std::string &type, const std::string &tid, Version version)
{
    {
        std::lock_guard<std::mutex> autoLock(listenerMutex_);
        auto it = listenerMap_.find(type + tid);
        if (it == listenerMap_.end()) {
            return;
        }
    }
    int32_t ret = RequestManager::GetInstance()->Off(type, tid, version);
    {
        std::lock_guard<std::mutex> autoLock(listenerMutex_);
        auto it = listenerMap_.find(type + tid);
        if (it == listenerMap_.end()) {
            return;
        }
        if (ret == E_OK) {
            listenerMap_.erase(it);
        }
    }
}

void JsTask::ClearListener()
{
    std::lock_guard<std::mutex> autoLock(listenerMutex_);
    for (const auto &listener : listenerMap_) {
        for (const auto &iter : listener.second) {
            if (iter != nullptr) {
                iter->DeleteCallbackRef();
            }
        }
    }
    listenerMap_.clear();
}

void JsTask::ReloadListener()
{
    REQUEST_HILOGD("ReloadListener in");
    std::lock_guard<std::mutex> lockGuard(JsTask::taskMutex_);
    for (const auto &it : taskMap_) {
        std::string tid = it.first;
        for (auto itListener : it.second->listenerMap_) {
            std::string key = itListener.first;
            if (key.find(tid) == std::string::npos) {
                continue;
            }
            std::string type = key.substr(0, key.find(tid));
            for (const auto &listener : itListener.second) {
                RequestManager::GetInstance()->On(type, tid, listener, it.second->config_.version);
            }
        }
    }
}

void JsTask::ClearTaskMap(const std::string &key)
{
    std::lock_guard<std::mutex> lockGuard(JsTask::taskMutex_);
    auto it = taskMap_.find(key);
    if (it == taskMap_.end()) {
        return;
    }
    taskMap_.erase(it);
}

bool JsTask::SetDirsPermission(std::vector<std::string> &dirs)
{
    std::string newPath = "/data/storage/el2/base/.ohos/.request/.certs";
    if (!fs::exists(newPath)) {
        if (fs::create_directories(newPath)) {
            REQUEST_HILOGD("Creat folder path succss.");
        }
    }
    for (const auto &folderPath : dirs) {
        fs::path folder = folderPath;
        if (!(fs::exists(folder) && fs::is_directory(folder))) {
            return false;
        }
        for (const auto& entry : fs::directory_iterator(folder)) {
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
    if (!CheckPathBaseDir(filepath, baseDir)) {
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

bool JsTask::CheckPathBaseDir(const std::string &filepath, std::string &baseDir)
{
    if (!JsInitialize::GetBaseDir(baseDir)) {
        return false;
    }

    if (filepath.find(baseDir) != std::string::npos) {
        return true;
    }
    // check baseDir replaced with el2
    if (baseDir.find(AREA1) != std::string::npos) {
        baseDir = baseDir.replace(baseDir.find(AREA1), AREA1.length(), AREA2);
        if (filepath.find(baseDir) == std::string::npos) {
            REQUEST_HILOGE("File dir not include base dir: %{public}s", baseDir.c_str());
            return false;
        }
        return true;
    }
    // check baseDir replaced with el1
    if (baseDir.find(AREA2) != std::string::npos) {
        baseDir = baseDir.replace(baseDir.find(AREA2), AREA2.length(), AREA1);
        if (filepath.find(baseDir) == std::string::npos) {
            REQUEST_HILOGE("File dir not include base dir: %{public}s", baseDir.c_str());
            return false;
        }
        return true;
    }
    return false;
}

void JsTask::AddPathMap(const std::string &filepath, const std::string &baseDir)
{
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
    if (!CheckPathBaseDir(filepath, baseDir)) {
        return;
    }

    if (chmod(filepath.c_str(), S_IRUSR | S_IWUSR | S_IRGRP) != 0) {
        REQUEST_HILOGE("File remove OTH access Failed.");
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
        for (const auto& entry : fs::directory_iterator(folder)) {
            fs::path path = entry.path();
            std::string filePath = folder.string() + "/" + path.filename().string();
            RemovePathMap(filePath);
        }
    }
}

void JsTask::ClearTaskContext(const std::string &key)
{
    std::lock_guard<std::mutex> lockGuard(JsTask::taskContextMutex_);
    auto it = taskContextMap_.find(key);
    if (it == taskContextMap_.end()) {
        REQUEST_HILOGD("Clear task context, not in ContextMap");
        return;
    }
    auto context = it->second;
    auto bodyFileNames = context->task->config_.bodyFileNames;
    std::thread([bodyFileNames]() {
        for (auto &filePath : bodyFileNames) {
            // Delete file.
            std::remove(filePath.c_str());
        }
    }).detach();
    // Reset Acl permission
    for (auto &file : context->task->config_.files) {
        RemovePathMap(file.uri);
    }
    RemoveDirsPermission(context->task->config_.certsPath);
    taskContextMap_.erase(it);
    UnrefTaskContextMap(context);
}

void JsTask::UnrefTaskContextMap(std::shared_ptr<ContextInfo> context)
{
    ContextCallbackData *data = new ContextCallbackData();
    if (data == nullptr) {
        return;
    }
    data->context = context;
    UvQueue::Call(data->context->env_, static_cast<void *>(data), UvUnrefTaskContext);
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
    register_ = true;
    auto context = AbilityRuntime::ApplicationContext::GetInstance();
    if (context == nullptr) {
        REQUEST_HILOGE("Get ApplicationContext failed");
        return;
    }
    context -> RegisterAbilityLifecycleCallback(std::make_shared<AppStateCallback>());
    REQUEST_HILOGD("Register foreground resume callback success");
}
} // namespace OHOS::Request