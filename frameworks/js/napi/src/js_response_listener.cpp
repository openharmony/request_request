/*
 * Copyright (C) 2024 Huawei Device Co., Ltd.
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

#include "js_response_listener.h"

#include "log.h"
#include "request_manager.h"
#include "uv_queue.h"

namespace OHOS::Request {

napi_status JSResponseListener::AddListener(napi_value cb)
{
    napi_status ret = this->AddListenerInner(cb);
    if (ret != napi_ok) {
        return ret;
    }
    if (this->validCbNum == 1) {
        RequestManager::GetInstance()->AddListener(this->taskId_, this->type_, shared_from_this());
    }
    return napi_ok;
}

napi_status JSResponseListener::RemoveListener(napi_value cb)
{
    napi_status ret = this->RemoveListenerInner(cb);
    if (ret != napi_ok) {
        return ret;
    }
    if (this->validCbNum == 0) {
        RequestManager::GetInstance()->RemoveListener(this->taskId_, this->type_, shared_from_this());
    }
    return napi_ok;
}

void JSResponseListener::OnResponseReceive(const std::shared_ptr<Response> &response)
{
    REQUEST_HILOGI("OnResponseReceive, tid is %{public}s", response->taskId.c_str());
    uv_loop_s *loop = nullptr;
    napi_get_uv_event_loop(this->env_, &loop);
    if (loop == nullptr) {
        REQUEST_HILOGE("napi_get_uv_event_loop failed");
        return;
    }
    uv_work_t *work = new (std::nothrow) uv_work_t;
    if (work == nullptr) {
        REQUEST_HILOGE("uv_work_t new failed");
        return;
    }
    {
        std::lock_guard<std::mutex> lock(this->responseMutex_);
        this->response_ = response;
    }
    work->data = reinterpret_cast<void *>(this);
    uv_queue_work(
        loop, work, [](uv_work_t *work) {},
        [](uv_work_t *work, int status) {
            JSResponseListener *listener = static_cast<JSResponseListener *>(work->data);
            std::lock_guard<std::mutex> lock(listener->responseMutex_);
            napi_handle_scope scope = nullptr;
            napi_open_handle_scope(listener->env_, &scope);
            napi_value value = NapiUtils::Convert2JSValue(listener->env_, listener->response_);
            listener->OnMessageReceive(&value, 1);
            napi_close_handle_scope(listener->env_, scope);
            delete work;
        });
}

} // namespace OHOS::Request