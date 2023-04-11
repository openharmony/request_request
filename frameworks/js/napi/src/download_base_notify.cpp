/*
 * Copyright (C) 2021-2022 Huawei Device Co., Ltd.
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

#include "download_base_notify.h"
#include "log.h"
#include "uv_queue.h"
#include "napi_utils.h"

namespace OHOS::Request::Download {
DownloadBaseNotify::DownloadBaseNotify(napi_env env, uint32_t paramNumber, napi_ref ref) : DownloadNotifyStub()
{
    notifyData_ = std::make_shared<NotifyData>();
    notifyData_->env = env;
    notifyData_->paramNumber = paramNumber;
    notifyData_->ref = ref;
}

DownloadBaseNotify::~DownloadBaseNotify()
{
}

void DownloadBaseNotify::CallBack(const std::vector<int64_t> &params)
{
    DOWNLOAD_HILOGD("Pause callback in");
    NotifyDataPtr *notifyDataPtr = GetNotifyDataPtr();
    {
        std::lock_guard<std::mutex> lock(notifyData_->mutex);
        notifyData_->params = params;
    }
    notifyDataPtr->notifyData = notifyData_;
    uv_after_work_cb afterCallback = [](uv_work_t *work, int status) {
        NotifyDataPtr *notifyDataPtr = static_cast<NotifyDataPtr *>(work->data);
        if (notifyDataPtr != nullptr && notifyDataPtr->notifyData != nullptr) {
            napi_handle_scope scope = nullptr;
            napi_open_handle_scope(notifyDataPtr->notifyData->env, &scope);
            napi_value undefined = 0;
            napi_get_undefined(notifyDataPtr->notifyData->env, &undefined);
            napi_value callbackFunc = nullptr;
            napi_get_reference_value(notifyDataPtr->notifyData->env, notifyDataPtr->notifyData->ref, &callbackFunc);
            napi_value callbackResult = nullptr;
            napi_value callbackValues[Download::TWO_PARAMETER] = { 0 };
            {
                std::lock_guard<std::mutex> lock(notifyDataPtr->notifyData->mutex);
                for (uint32_t i = 0; i < notifyDataPtr->notifyData->paramNumber; i++) {
                    napi_create_int64(notifyDataPtr->notifyData->env, notifyDataPtr->notifyData->params[i],
                        &callbackValues[i]);
                }
            }
            napi_call_function(notifyDataPtr->notifyData->env, nullptr, callbackFunc,
                notifyDataPtr->notifyData->paramNumber, callbackValues, &callbackResult);
            napi_close_handle_scope(notifyDataPtr->notifyData->env, scope);
            delete notifyDataPtr;
            delete work;
        }
    };
    UvQueue::Call(notifyData_->env, reinterpret_cast<void *>(notifyDataPtr), afterCallback);
}

NotifyDataPtr *DownloadBaseNotify::GetNotifyDataPtr()
{
    return new (std::nothrow) NotifyDataPtr;
}
} // namespace OHOS::Request::Download