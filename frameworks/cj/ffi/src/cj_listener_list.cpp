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

#include "cj_listener_list.h"
#include "cj_request_common.h"

namespace OHOS::CJSystemapi::Request {

void ListenerList::AddListenerInner(std::function<void(CProgress)> &cb, CFunc cbId)
{
    std::lock_guard<std::recursive_mutex> lock(allCbMutex_);
    if (this->IsListenerAdded(cbId)) {
        return;
    }

    this->allCb_.push_back(std::make_pair(true, std::make_shared<CallBackInfo>(cb, cbId)));
    ++this->validCbNum;
}

void ListenerList::AddListenerInner(std::function<void(CResponse)> &cb, CFunc cbId)
{
    std::lock_guard<std::recursive_mutex> lock(allCbMutex_);
    if (this->IsListenerAdded(cbId)) {
        return;
    }

    this->allCb_.push_back(std::make_pair(true, std::make_shared<CallBackInfo>(cb, cbId)));
    ++this->validCbNum;
}

void ListenerList::RemoveListenerInner(CFunc cb)
{
    std::lock_guard<std::recursive_mutex> lock(allCbMutex_);
    if (this->validCbNum == 0) {
        return;
    }

    if (cb == nullptr) {
        for (auto it = this->allCb_.begin(); it != this->allCb_.end(); it++) {
            it->first = false;
        }
        this->validCbNum = 0;
        return;
    }

    for (auto it = this->allCb_.begin(); it != this->allCb_.end(); it++) {
        if (it->second->cbId_ == cb) {
            if (it->first == true) {
                it->first = false;
                --this->validCbNum;
            }
            break;
        }
    }
}

void ListenerList::OnMessageReceive(const std::shared_ptr<NotifyData> &notifyData)
{
    std::lock_guard<std::recursive_mutex> lock(allCbMutex_);
    for (auto it = this->allCb_.begin(); it != this->allCb_.end();) {
        if (it->first == false) {
            it = this->allCb_.erase(it);
            continue;
        }
        it->second->progressCB_(Convert2CProgress(notifyData->progress));
        it++;
    }
}

void ListenerList::OnMessageReceive(const std::shared_ptr<Response> &response)
{
    std::lock_guard<std::recursive_mutex> lock(allCbMutex_);
    for (auto it = this->allCb_.begin(); it != this->allCb_.end();) {
        if (it->first == false) {
            it = this->allCb_.erase(it);
            continue;
        }
        it->second->responseCB_(Convert2CResponse(response));
        it++;
    }
}

bool ListenerList::IsListenerAdded(void *cb)
{
    if (cb == nullptr) {
        return true;
    }

    for (auto it = this->allCb_.begin(); it != this->allCb_.end(); it++) {
        if (it->second->cbId_ == cb) {
            return it->first;
        }
    }

    return false;
}

bool ListenerList::HasListener()
{
    return this->validCbNum != 0;
}

} // namespace OHOS::CJSystemapi::Request