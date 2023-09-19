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
constexpr int32_t MAX_WAIT_TIME = 3000;
BlockQueue<NotifyEventInfo> RequestNotify::notifyQueue_{ MAX_WAIT_TIME };

RequestNotify::RequestNotify(napi_env env, napi_value callback) : NotifyStub()
{
    std::lock_guard<std::mutex> lock(validMutex_);
    env_ = env;
    napi_create_reference(env, callback, 1, &ref_);
    valid_ = true;
}

RequestNotify::~RequestNotify()
{
    REQUEST_HILOGI("~RequestNotify()");
    std::lock_guard<std::mutex> lock(validMutex_);
    if (valid_ && env_ != nullptr && ref_ != nullptr) {
        UvQueue::DeleteRef(env_, ref_);
        ref_ = nullptr;
    }
}

void RequestNotify::CallBack(const Notify &notify)
{
    REQUEST_HILOGI("RequestNotify CallBack in");
    SetNotify(notify);
    info_.timestamp = std::chrono::system_clock::now().time_since_epoch().count();
    notifyQueue_.Push(info_);
    NotifyDataPtr *dataPtr = new NotifyDataPtr();
    dataPtr->callback = this;

    uv_loop_s *loop = nullptr;
    napi_get_uv_event_loop(env_, &loop);
    if (loop == nullptr) {
        return;
    }
    uv_work_t *work = new (std::nothrow) uv_work_t;
    if (work == nullptr) {
        return;
    }
    work->data = reinterpret_cast<void *>(dataPtr);
    uv_queue_work(loop, work, [](uv_work_t *work) {
        if (work == nullptr) {
            return;
        }
        NotifyDataPtr *dataPtr = static_cast<NotifyDataPtr *>(work->data);
        if (dataPtr != nullptr) {
            REQUEST_HILOGI("timestamp is %{public}" PRId64, dataPtr->callback->info_.timestamp);
            notifyQueue_.Wait(dataPtr->callback->info_);
        }
    }, [](uv_work_t *work, int status) {
        if (work == nullptr) {
            return;
        }
        NotifyDataPtr *dataPtr = static_cast<NotifyDataPtr *>(work->data);
        if (dataPtr != nullptr) {
            dataPtr->callback->ExecCallBack();
            delete dataPtr;
        }
        notifyQueue_.Pop();
        delete work;
    });
}

void RequestNotify::Done(const TaskInfo &taskInfo)
{
}

void RequestNotify::ExecCallBack()
{
    REQUEST_HILOGI("ExecCallBack in");
    if (!valid_ || ref_ == nullptr) {
        REQUEST_HILOGE("valid is false");
        return;
    }
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
    if (notify_.type == EventType::DATA_CALLBACK) {
        paramNumber = notify_.data.size();
        if (paramNumber > valueSize) {
            return;
        }
        for (uint32_t i = 0; i < paramNumber; i++) {
            value[i] = NapiUtils::Convert2JSValue(env_, notify_.data[i]);
        }
    } else if (notify_.type == EventType::HEADER_CALLBACK) {
        value[0] = NapiUtils::Convert2JSHeaders(env_, notify_.header);
    } else if (notify_.type == EventType::TASK_STATE_CALLBACK) {
        value[0] = NapiUtils::Convert2JSValue(env_, notify_.taskStates);
    } else if (notify_.type == EventType::PROGRESS_CALLBACK) {
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
    std::lock_guard<std::mutex> lock(validMutex_);
    if (env_ != nullptr && ref_ != nullptr) {
        valid_ = false;
        napi_delete_reference(env_, ref_);
        ref_ = nullptr;
    }
}
} // namespace OHOS::Request::Download