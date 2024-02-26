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
    std::lock_guard<std::recursive_mutex> lock(allCbMutex_);
    if (this->IsListenerAdded(cb)) {
        return napi_ok;
    }

    napi_ref ref;
    napi_status status = napi_create_reference(env_, cb, 1, &ref);
    if (status != napi_ok) {
        return status;
    }

    this->allCb_.push_back(std::make_pair(true, ref));
    ++this->validCbNum;
    if (this->validCbNum == 1) {
        RequestManager::GetInstance()->Subscribe(this->taskId_, shared_from_this());
    }

    return napi_ok;
}

napi_status JSResponseListener::RemoveListener(napi_value cb)
{
    std::lock_guard<std::recursive_mutex> lock(allCbMutex_);
    if (this->validCbNum == 0) {
        return napi_ok;
    }

    if (cb == nullptr) {
        RequestManager::GetInstance()->Unsubscribe(this->taskId_, shared_from_this());
        for (auto it = this->allCb_.begin(); it != this->allCb_.end(); it++) {
            it->first = false;
        }
        this->validCbNum = 0;
        return napi_ok;
    }

    for (auto it = this->allCb_.begin(); it != this->allCb_.end(); it++) {
        napi_value copyValue = nullptr;
        napi_get_reference_value(this->env_, it->second, &copyValue);

        bool isEquals = false;
        napi_strict_equals(this->env_, cb, copyValue, &isEquals);
        if (isEquals) {
            if (it->first == true) {
                it->first = false;
                --this->validCbNum;
            }
            break;
        }
    }

    if (this->validCbNum == 0) {
        RequestManager::GetInstance()->Unsubscribe(this->taskId_, shared_from_this());
    }

    return napi_ok;
}

void JSResponseListener::OnResponseReceive(const std::shared_ptr<Response> &response)
{
    std::lock_guard<std::recursive_mutex> lock(allCbMutex_);
    napi_value value = NapiUtils::Convert2JSValue(this->env_, response);
    for (auto it = this->allCb_.begin(); it != this->allCb_.end();) {
        if (it->first == false) {
            napi_delete_reference(this->env_, it->second);
            it = this->allCb_.erase(it);
            continue;
        }
        napi_handle_scope scope = nullptr;
        napi_open_handle_scope(this->env_, &scope);
        napi_value callbackFunc = nullptr;
        napi_get_reference_value(this->env_, it->second, &callbackFunc);

        napi_value callbackResult = nullptr;
        uint32_t paramNumber = 1;
        napi_call_function(this->env_, nullptr, callbackFunc, paramNumber, &value, &callbackResult);
        napi_close_handle_scope(this->env_, scope);
        it++;
    }
}

bool JSResponseListener::IsListenerAdded(napi_value cb)
{
    if (cb == nullptr) {
        return true;
    }
    for (auto it = this->allCb_.begin(); it != this->allCb_.end(); it++) {
        napi_value copyValue = nullptr;
        napi_get_reference_value(this->env_, it->second, &copyValue);

        bool isEquals = false;
        napi_strict_equals(this->env_, cb, copyValue, &isEquals);
        if (isEquals) {
            return it->first;
        }
    }

    return false;
}

bool JSResponseListener::HasListener()
{
    return this->validCbNum != 0;
}

} // namespace OHOS::Request