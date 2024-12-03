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

#include "request.h"

namespace OHOS::Request {

Request::Request(const std::string &taskId) : taskId_(taskId)
{
}

const std::string &Request::getId() const
{
    return this->taskId_;
}

void Request::AddListener(const SubscribeType &type, const std::shared_ptr<IResponseListener> &listener)
{
    if (type == SubscribeType::RESPONSE) {
        std::lock_guard<std::mutex> lock(listenerMutex_);
        responseListener_ = listener;
    }
}

void Request::RemoveListener(const SubscribeType &type, const std::shared_ptr<IResponseListener> &listener)
{
    if (type == SubscribeType::RESPONSE) {
        std::lock_guard<std::mutex> lock(listenerMutex_);
        responseListener_.reset();
    }
}

// for api9, remove do not notify after complete/fail
bool Request::NeedNotify(const std::shared_ptr<NotifyData> &notifyData)
{
    if (notifyData->type == SubscribeType::REMOVE && notifyData->version != Version::API10
        && this->needRemove_ == false) {
        return false;
    }
    if ((notifyData->type == SubscribeType::COMPLETED || notifyData->type == SubscribeType::FAILED)
        && notifyData->version != Version::API10) {
        this->needRemove_ = false;
    }
    return true;
}

void Request::AddListener(const SubscribeType &type, const std::shared_ptr<INotifyDataListener> &listener)
{
    std::lock_guard<std::mutex> lock(listenerMutex_);
    if (type != SubscribeType::RESPONSE && type < SubscribeType::BUTT) {
        notifyDataListenerMap_[type] = listener;
    }
    if (unusedNotifyData_.find(type) != unusedNotifyData_.end()) {
        if (NeedNotify(unusedNotifyData_[type])) {
            listener->OnNotifyDataReceive(unusedNotifyData_[type]);
        }
        unusedNotifyData_.erase(type);
    }
}

void Request::RemoveListener(const SubscribeType &type, const std::shared_ptr<INotifyDataListener> &listener)
{
    if (type != SubscribeType::RESPONSE && type < SubscribeType::BUTT) {
        std::lock_guard<std::mutex> lock(listenerMutex_);
        notifyDataListenerMap_.erase(type);
    }
}

bool Request::HasListener()
{
    std::lock_guard<std::mutex> lock(listenerMutex_);
    if (responseListener_ != nullptr) {
        return true;
    }
    return !notifyDataListenerMap_.empty();
}

void Request::OnResponseReceive(const std::shared_ptr<Response> &response)
{
    std::lock_guard<std::mutex> lock(listenerMutex_);
    if (responseListener_ != nullptr) {
        responseListener_->OnResponseReceive(response);
    }
}

void Request::OnNotifyDataReceive(const std::shared_ptr<NotifyData> &notifyData)
{
    std::lock_guard<std::mutex> lock(listenerMutex_);
    auto listener = notifyDataListenerMap_.find(notifyData->type);
    if (listener != notifyDataListenerMap_.end()) {
        if (NeedNotify(notifyData)) {
            listener->second->OnNotifyDataReceive(notifyData);
        }
    } else if (notifyData->version != Version::API10) {
        unusedNotifyData_[notifyData->type] = notifyData;
    }
}

} // namespace OHOS::Request