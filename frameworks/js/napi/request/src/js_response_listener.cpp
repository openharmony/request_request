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
#include "napi/native_node_api.h"
#include "request_manager.h"

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
    {
        std::lock_guard<std::mutex> lock(this->responseMutex_);
        this->response_ = response;
    }
    std::shared_ptr<JSResponseListener> listener = shared_from_this();
    REQUEST_HILOGI("OnResponseReceive, tid: %{public}s", response->taskId.c_str());
    int32_t ret = napi_send_event(
        listener->env_,
        [listener]() {
            std::lock_guard<std::mutex> lock(listener->responseMutex_);
            napi_handle_scope scope = nullptr;
            napi_status status = napi_open_handle_scope(listener->env_, &scope);
            if (status != napi_ok || scope == nullptr) {
                REQUEST_HILOGE("OnResponseReceive napi_scope failed");
                return;
            }
            napi_value value = NapiUtils::Convert2JSValue(listener->env_, listener->response_);
            listener->OnMessageReceive(&value, 1);
            napi_close_handle_scope(listener->env_, scope);
        },
        napi_eprio_high,
        "request:task.on");
    if (ret != napi_ok) {
        REQUEST_HILOGE("napi_send_event failed: %{public}d", ret);
    }
}

} // namespace OHOS::Request