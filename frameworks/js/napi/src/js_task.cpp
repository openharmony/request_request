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
            context->jsConfig = argv[1];
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
        if (!RequestManager::GetInstance()->LoadRequestServer()) {
            context->innerCode_ = E_SERVICE_ERROR;
            return;
        }
        int32_t ret = RequestManager::GetInstance()->Create(context->task->config_, context->tid);
        if (ret != E_OK || context->tid < 0) {
            context->innerCode_ = ret;
        }
        RequestManager::GetInstance()->On("done", context->task->GetTid(), sptr<RequestNotify>(new RequestNotify()));
    };
    auto output = [context](napi_value *result) -> napi_status {
        if (result == nullptr) {
            return napi_generic_failure;
        }
        napi_status status = napi_get_reference_value(context->env_, context->taskRef, result);
        context->task->SetTid(context->tid);
        JsTask::AddTaskMap(std::to_string(context->tid), context->task);
        JsInitialize::CreatProperties(context->env_, *result, context->jsConfig, context->task);
        return status;
    };
    context->SetInput(input).SetOutput(output).SetExec(exec);
    AsyncCall asyncCall(env, info, context);
    return asyncCall.Call(context, "create");
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
        if (argc < 1) {
            NapiUtils::ThrowError(context->env_, E_PARAMETER_CHECK, "should 1 parameter!", true);
            return napi_invalid_arg;
        }
        context->tid = NapiUtils::Convert2String(context->env_, argv[0]);
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

napi_value JsTask::Show(napi_env env, napi_callback_info info)
{
    return nullptr;
}

napi_value JsTask::Touch(napi_env env, napi_callback_info info)
{
    return nullptr;
}

napi_value JsTask::Search(napi_env env, napi_callback_info info)
{
    return nullptr;
}

napi_value JsTask::Query(napi_env env, napi_callback_info info)
{
    return nullptr;
}

napi_value JsTask::Clear(napi_env env, napi_callback_info info)
{
    return nullptr;
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
    uint32_t ret = RequestManager::GetInstance()->Off(type, tid);
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