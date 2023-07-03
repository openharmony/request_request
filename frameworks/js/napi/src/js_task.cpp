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

#include <mutex>
#include <ctime>
#include <regex>
#include "async_call.h"
#include "request_event.h"
#include "request_manager.h"
#include "legacy/request_manager.h"
#include "log.h"
#include "napi_base_context.h"
#include "upload/upload_task_napiV5.h"
#include "napi_utils.h"
#include "js_initialize.h"
#include "js_task.h"
namespace OHOS::Request {
std::mutex JsTask::createMutex_;
thread_local napi_ref JsTask::createCtor = nullptr;
std::mutex JsTask::requestMutex_;
thread_local napi_ref JsTask::requestCtor = nullptr;
std::mutex JsTask::requestFileMutex_;
thread_local napi_ref JsTask::requestFileCtor = nullptr;
std::mutex JsTask::taskMutex_;
std::map<std::string, JsTask*> JsTask::taskMap_;

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
    ClearListener();
    RequestEvent::RemoveCache(tid_);
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
        if (context->innerCode_ == E_SERVICE_ERROR && config.version == Version::API9 &&
            config.action == Action::UPLOAD) {
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
        napi_value config = nullptr;
        napi_get_reference_value(context->env_, context->jsConfig, &config);
        JsInitialize::CreatProperties(context->env_, *result, config, context->task);
        napi_delete_reference(context->env_, context->taskRef);
        return status;
    };
    context->SetInput(std::move(input)).SetOutput(std::move(output)).SetExec(std::move(exec));
    AsyncCall asyncCall(env, info, context);
    return asyncCall.Call(context, "create");
}

int32_t JsTask::CreateExec(const std::shared_ptr<ContextInfo> &context)
{
    if (!RequestManager::GetInstance()->LoadRequestServer()) {
        return E_SERVICE_ERROR;
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

napi_value JsTask::DefineClass(napi_env env, const napi_property_descriptor* desc, size_t count,
    napi_callback cb, napi_ref *ctor)
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
    };
    context->SetInput(input).SetOutput(output).SetExec(exec);
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
    struct ShowContext : public AsyncCall::Context {
        std::string tid;
        TaskInfo taskInfo;
        std::string token;
    };

    auto context = std::make_shared<ShowContext>();
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
    context->SetInput(input).SetOutput(output).SetExec(exec);
    AsyncCall asyncCall(env, info, context);
    return asyncCall.Call(context, "show");
}

napi_value JsTask::Touch(napi_env env, napi_callback_info info)
{
    struct TouchContext : public AsyncCall::Context {
        std::string tid;
        std::string token;
        TaskInfo taskInfo;
    };

    auto context = std::make_shared<TouchContext>();
    context->withErrCode_ = true;
    context->version_ = Version::API10;
    auto input = [context](size_t argc, napi_value *argv, napi_value self) -> napi_status {
        bool ret = ParseTouch(context->env_, argc, argv, context->tid, context->token);
        if (!ret) {
            NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, "Parse tid or token fail!", true);
            return napi_invalid_arg;
        }
        return napi_ok;
    };
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
    context->SetInput(input).SetOutput(output).SetExec(exec);
    AsyncCall asyncCall(env, info, context);
    return asyncCall.Call(context, "show");
}

bool JsTask::ParseTouch(napi_env env, size_t argc, napi_value *argv, std::string &tid, std::string &token)
{
    if (argc < 2) {
        REQUEST_HILOGE("Wrong number of arguments");
        return false;
    }
    if (NapiUtils::GetValueType(env, argv[0]) != napi_string || NapiUtils::GetValueType(env, argv[1]) != napi_string) {
        REQUEST_HILOGE("The parameter is not of string type");
        return false;
    }
    tid = NapiUtils::Convert2String(env, argv[0]);
    token = NapiUtils::Convert2String(env, argv[1]);
    if (tid.empty() || token.empty()) {
        return false;
    }
    if (token.size() < TOKEN_MIN_BYTES || token.size() > TOKEN_MAX_BYTES) {
        return false;
    }
    return true;
}

bool JsTask::ParseSearch(napi_env env, size_t argc, napi_value *argv, Filter &filter)
{
    if (argc < 1) {
        REQUEST_HILOGE("Wrong number of arguments");
        return false;
    }
    if (NapiUtils::GetValueType(env, argv[0]) != napi_object) {
        REQUEST_HILOGE("The parameter is not of object type");
        return false;
    }
    filter.bundle = ParseBundle(env, argv[0]);
    if (!ParseBefore(env, argv[0], filter.before)) {
        return false;
    }
    if (!ParseAfter(env, argv[0], filter.before, filter.after)) {
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

bool JsTask::ParseBefore(napi_env env, napi_value value, uint64_t &before)
{
    time_t timeStamp = time(nullptr);
    std::string beforeStr = NapiUtils::Convert2String(env, value, "before");
    if (beforeStr.empty()) {
        before = static_cast<uint64_t>(timeStamp);
        return true;
    }
    std::string regexStr =
        "(\\d{4})-(0\\d{1}|1[0-2])-(0\\d{1}|[12]\\d{1}|3[01])\\s(0\\d{1}|1\\d{1}|2[0-3]):[0-5]\\d{1}:([0-5]\\d{1})";
    if (!regex_match(beforeStr, std::regex(regexStr))) {
        REQUEST_HILOGE("Parse data error");
        return false;
    }
	struct tm timeInfo;
	strptime(beforeStr.c_str(), "%Y-%m-%d %H:%M:%S", &timeInfo);
	timeStamp = mktime(&timeInfo);
    before = static_cast<uint64_t>(timeStamp);
    return true;
}

bool JsTask::ParseAfter(napi_env env, napi_value value, uint64_t before, uint64_t &after)
{
    std::string afterStr = NapiUtils::Convert2String(env, value, "after");
    if (afterStr.empty()) {
        after = before - 24*60*60;
        return true;
    }
    std::string regexStr =
        "(\\d{4})-(0\\d{1}|1[0-2])-(0\\d{1}|[12]\\d{1}|3[01])\\s(0\\d{1}|1\\d{1}|2[0-3]):[0-5]\\d{1}:([0-5]\\d{1})";
    if (!regex_match(afterStr, std::regex(regexStr))) {
        REQUEST_HILOGE("Parse data error");
        return false;
    }
	struct tm timeInfo;
	strptime(afterStr.c_str(), "%Y-%m-%d %H:%M:%S", &timeInfo);
	time_t time = mktime(&timeInfo);
    after = static_cast<uint64_t>(time);
    return true;
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
    context->SetInput(input).SetOutput(output).SetExec(exec);
    AsyncCall asyncCall(env, info, context);
    return asyncCall.Call(context, "show");
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
    context->SetInput(input).SetOutput(output).SetExec(exec);
    AsyncCall asyncCall(env, info, context);
    return asyncCall.Call(context, "show");
}

//napi_value JsTask::Clear(napi_env env, napi_callback_info info)
//{
//    return nullptr;
//}

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

void JsTask::AddTaskMap(const std::string &key, JsTask* task)
{
    std::lock_guard<std::mutex> lockGuard(JsTask::taskMutex_);
    JsTask::taskMap_[key] = task;
}

void JsTask::AddListener(const std::string &key, const sptr<RequestNotify> &listener)
{
    REQUEST_HILOGD("AddListener key %{public}s", key.c_str());
    std::lock_guard<std::mutex> autoLock(listenerMutex_);
    listenerMap_[key].push_back(listener);
}

void JsTask::RemoveListener(const std::string &type, const std::string &tid, napi_value callback)
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
        RequestManager::GetInstance()->Off(type, tid);
        listenerMap_.erase(key);
    }
}

void JsTask::RemoveListener(const std::string &type, const std::string &tid)
{
    std::lock_guard<std::mutex> autoLock(listenerMutex_);
    auto it = listenerMap_.find(type + tid);
    if (it == listenerMap_.end()) {
        return;
    }
    int32_t ret = RequestManager::GetInstance()->Off(type, tid);
    if (ret == E_OK) {
        listenerMap_.erase(it);
    }
}

void JsTask::ClearListener()
{
    std::lock_guard<std::mutex> autoLock(listenerMutex_);
    listenerMap_.clear();
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
}