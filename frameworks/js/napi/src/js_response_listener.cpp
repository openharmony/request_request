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

#include "request_manager.h"

namespace OHOS::Request {

napi_status JSResponseListener::AddListener(napi_value cb)
{
    if (this->IsListenerAdded(cb)) {
        return napi_ok;
    }

    napi_ref ref;
    napi_status status = napi_create_reference(env_, cb, 1, &ref);
    if (status != napi_ok) {
        return status;
    }

    this->allCb_.push_back(ref);
    if (this->allCb_.size() == 1) {
        RequestManager::GetInstance()->Subscribe(this->taskId_, shared_from_this());
    }

    return napi_ok;
}

napi_status JSResponseListener::RemoveListener(napi_value cb)
{
    if (this->allCb_.empty()) {
        return napi_ok;
    }

    if (cb == nullptr) {
        RequestManager::GetInstance()->Unsubscribe(this->taskId_, shared_from_this());
        while (!this->allCb_.empty()) {
            napi_ref ref = this->allCb_.front();
            napi_delete_reference(this->env_, ref);
            this->allCb_.pop_front();
        }
        return napi_ok;
    }

    for (auto it = this->allCb_.begin(); it != this->allCb_.end(); it++) {
        napi_value copyValue = nullptr;
        napi_get_reference_value(this->env_, *it, &copyValue);

        bool isEquals = false;
        napi_strict_equals(this->env_, cb, copyValue, &isEquals);
        if (isEquals) {
            napi_delete_reference(this->env_, *it);
            this->allCb_.erase(it);
            break;
        }
    }

    if (this->allCb_.empty()) {
        RequestManager::GetInstance()->Unsubscribe(this->taskId_, shared_from_this());
    }

    return napi_ok;
}

void JSResponseListener::OnResponseReceive(const std::shared_ptr<Response> &response)
{
    napi_value value = NapiUtils::Convert2JSValue(this->env_, response);
    for (auto it = this->allCb_.begin(); it != this->allCb_.end(); it++) {
        napi_handle_scope scope = nullptr;
        napi_open_handle_scope(this->env_, &scope);
        napi_value callbackFunc = nullptr;
        napi_get_reference_value(this->env_, *it, &callbackFunc);

        napi_value callbackResult = nullptr;
        uint32_t paramNumber = 1;
        napi_call_function(this->env_, nullptr, callbackFunc, paramNumber, &value, &callbackResult);
        napi_close_handle_scope(this->env_, scope);
    }
}

bool JSResponseListener::IsListenerAdded(napi_value cb)
{
    if (cb == nullptr) {
        return true;
    }
    for (auto it = this->allCb_.begin(); it != this->allCb_.end(); it++) {
        napi_value copyValue = nullptr;
        napi_get_reference_value(this->env_, *it, &copyValue);

        bool isEquals = false;
        napi_strict_equals(this->env_, cb, copyValue, &isEquals);
        if (isEquals) {
            return true;
        }
    }

    return false;
}

bool JSResponseListener::HasListener()
{
    return !this->allCb_.empty();
}
} // namespace OHOS::Request