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
#include "uv_queue.h"

namespace OHOS::Request {
bool UvQueue::Call(napi_env env, void *data, uv_after_work_cb afterCallback)
{
    uv_loop_s *loop = nullptr;
    napi_get_uv_event_loop(env, &loop);
    if (loop == nullptr) {
        return false;
    }
    uv_work_t *work = new (std::nothrow) uv_work_t;
    if (work == nullptr) {
        return false;
    }
    work->data = data;
    uv_queue_work(
        loop, work, [](uv_work_t *work) {}, afterCallback);
    return true;
}

void UvQueue::DeleteRef(napi_env env, napi_ref ref)
{
    UvCallbackData *callbackData = new (std::nothrow) UvCallbackData();
    if (callbackData == nullptr) {
        return;
    }
    callbackData->env = env;
    callbackData->ref = ref;
    if (!UvQueue::Call(env, reinterpret_cast<void *>(callbackData), UvDelete)) {
        delete callbackData;
    }
}

void UvQueue::UvDelete(uv_work_t *work, int status)
{
    UvCallbackData *callbackData = reinterpret_cast<UvCallbackData *>(work->data);
    if (callbackData != nullptr) {
        napi_delete_reference(callbackData->env, callbackData->ref);
        delete callbackData;
    }
    delete work;
}
} // namespace OHOS::Request