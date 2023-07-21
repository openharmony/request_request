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

#include "request_notify.h"

#include <uv.h>

#include "log.h"
#include "napi_utils.h"
#include "uv_queue.h"
#include "js_task.h"

namespace OHOS::Request {
RequestNotify::RequestNotify(napi_env env, napi_value callback) : NotifyStub()
{
    std::lock_guard<std::mutex> lock(envMutex_);
    env_ = env;
    napi_create_reference(env, callback, 1, &ref_);
    valid_ = true;
}

RequestNotify::~RequestNotify()
{
    std::lock_guard<std::mutex> lock(envMutex_);
    if (valid_ && env_ != nullptr && ref_ != nullptr) {
        UvQueue::DeleteRef(env_, ref_);
    }
    REQUEST_HILOGI("~RequestNotify()");
}

void RequestNotify::CallBack(const std::string &type, const std::string &tid, const Notify &notify)
{
    REQUEST_HILOGI("RequestNotify CallBack in");
    auto item = JsTask::taskMap_.find(tid);
    if (item == JsTask::taskMap_.end()) {
        REQUEST_HILOGE("Task ID not found");
        return;
    }
    auto task = item->second;
    std::string key = type + tid;
    auto it = task->listenerMap_.find(key);
    if (it == task->listenerMap_.end()) {
        REQUEST_HILOGE("Unregistered %{public}s callback", type.c_str());
        return;
    }
    SetNotify(notify);
    NotifyDataPtr *dataPtr = new NotifyDataPtr;
    dataPtr->callbacks = it->second;
    uv_after_work_cb afterCallback = [](uv_work_t *work, int status) {
        if (work == nullptr) {
            return;
        }
        NotifyDataPtr *dataPtr = static_cast<NotifyDataPtr *>(work->data);
        if (dataPtr != nullptr) {
            for (const auto &callback : dataPtr->callbacks) {
                callback->ExecCallBack();
            }
            delete dataPtr;
        }
        delete work;
    };
    UvQueue::Call(env_, reinterpret_cast<void *>(dataPtr), afterCallback);
}

void RequestNotify::Done(const TaskInfo &taskInfo)
{
}

void RequestNotify::ExecCallBack()
{
    REQUEST_HILOGI("ExecCallBack in");
    std::lock_guard<std::mutex> lock(envMutex_);
    napi_handle_scope scope = nullptr;
    napi_open_handle_scope(env_, &scope);
    napi_value callbackFunc = nullptr;
    napi_get_reference_value(env_, ref_, &callbackFunc);
    napi_value callbackResult = nullptr;
    uint32_t paramNumber = 1;
    napi_value callbackValues[NapiUtils::TWO_ARG] = { nullptr };
    ConvertCallBackData(paramNumber, callbackValues, NapiUtils::TWO_ARG);
    napi_call_function(env_, nullptr, callbackFunc, paramNumber, callbackValues, &callbackResult);
    napi_close_handle_scope(env_, scope);
}

void RequestNotify::ConvertCallBackData(uint32_t &paramNumber, napi_value *value, uint32_t valueSize)
{
    std::lock_guard<std::mutex> lock(notifyMutex_);
    if (notify_.type == DATA_CALLBACK) {
        paramNumber = notify_.data.size();
        if (paramNumber > valueSize) {
            return;
        }
        for (uint32_t i = 0; i < paramNumber; i++) {
            value[i] = NapiUtils::Convert2JSValue(env_, notify_.data[i]);
        }
    } else if (notify_.type == HEADER_CALLBACK) {
        value[0] = NapiUtils::Convert2JSHeaders(env_, notify_.header);
    } else if (notify_.type == TASK_STATE_CALLBACK) {
        value[0] = NapiUtils::Convert2JSValue(env_, notify_.taskStates);
    } else if (notify_.type == PROGRESS_CALLBACK) {
        value[0] = NapiUtils::Convert2JSValue(env_, notify_.progress);
    }
}

void RequestNotify::SetNotify(const Notify &notify)
{
    std::lock_guard<std::mutex> lock(notifyMutex_);
    notify_ = notify;
}

void RequestNotify::DeleteCallbackRef()
{
    std::lock_guard<std::mutex> lock(envMutex_);
    if (env_ != nullptr && ref_ != nullptr) {
        valid_ = false;
        napi_delete_reference(env_, ref_);
    }
}
} // namespace OHOS::Request::Download