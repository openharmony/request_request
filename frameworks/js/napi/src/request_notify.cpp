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

namespace OHOS::Request {
RequestNotify::RequestNotify(napi_env env, napi_value callback) : NotifyStub()
{
    env_ = env;
    napi_create_reference(env, callback, 1, &ref_);
    data_ = std::make_shared<CallbackData>();
}

RequestNotify::~RequestNotify()
{
    REQUEST_HILOGI("~RequestNotify()");
}

void RequestNotify::CallBack(const Notify &notify)
{
    REQUEST_HILOGI("RequestNotify CallBack in");
    GetDataPtrParam(data_, notify);
    NotifyDataPtr *notifyDataPtr = new (std::nothrow) NotifyDataPtr;
    notifyDataPtr->dataPtr = data_;
    uv_after_work_cb afterCallback = [](uv_work_t *work, int status) {
        NotifyDataPtr *notifyDataPtr = static_cast<NotifyDataPtr *>(work->data);
        if (notifyDataPtr != nullptr) {
            GetCallBackData(notifyDataPtr);
            delete notifyDataPtr;
            delete work;
        }
    };
    UvQueue::Call(data_->env, reinterpret_cast<void *>(notifyDataPtr), afterCallback);
}

void RequestNotify::Done(const TaskInfo &taskInfo)
{
}
void RequestNotify::ConvertCallBackData(const std::shared_ptr<CallbackData> &dataPtr, uint32_t &paramNumber,
    napi_value *value)
{
    std::lock_guard<std::mutex> lock(dataPtr->mutex);
    if (dataPtr->notify.type == DATA_CALLBACK) {
        paramNumber = dataPtr->notify.data.size();
        for (uint32_t i = 0; i < dataPtr->notify.data.size(); i++) {
            value[i] = NapiUtils::Convert2JSValue(dataPtr->env, dataPtr->notify.data[i]);
        }
    } else if (dataPtr->notify.type == HEADER_CALLBACK) {
        value[0] = NapiUtils::Convert2JSHeaders(dataPtr->env, dataPtr->notify.header);
    } else if (dataPtr->notify.type == TASK_STATE_CALLBACK) {
        value[0] = NapiUtils::Convert2JSValue(dataPtr->env, dataPtr->notify.taskStates);
    } else if (dataPtr->notify.type == PROGRESS_CALLBACK) {
        value[0] = NapiUtils::Convert2JSValue(dataPtr->env, dataPtr->notify.progress);
    }
}

void RequestNotify::GetCallBackData(NotifyDataPtr *notifyDataPtr)
{
    napi_handle_scope scope = nullptr;
    napi_open_handle_scope(notifyDataPtr->dataPtr->env, &scope);
    napi_value undefined = nullptr;
    napi_get_undefined(notifyDataPtr->dataPtr->env, &undefined);
    napi_value callbackFunc = nullptr;
    napi_get_reference_value(notifyDataPtr->dataPtr->env, notifyDataPtr->dataPtr->ref, &callbackFunc);
    napi_value callbackResult = nullptr;
    uint32_t paramNumber = 1;
    napi_value callbackValues[NapiUtils::TWO_ARG] = { nullptr };
    ConvertCallBackData(notifyDataPtr->dataPtr, paramNumber, callbackValues);
    napi_call_function(notifyDataPtr->dataPtr->env, nullptr, callbackFunc, paramNumber, callbackValues,
        &callbackResult);
    napi_close_handle_scope(notifyDataPtr->dataPtr->env, scope);
}

void RequestNotify::GetDataPtrParam(const std::shared_ptr<CallbackData> &dataPtr, const Notify &notify)
{
    std::lock_guard<std::mutex> lock(dataPtr->mutex);
    dataPtr->env = env_;
    dataPtr->ref = ref_;
    dataPtr->notify = notify;
}
} // namespace OHOS::Request::Download