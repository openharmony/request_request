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

#ifndef OHOS_REQUEST_REQUEST_H
#define OHOS_REQUEST_REQUEST_H

#include <set>

#include "i_notify_data_listener.h"
#include "i_response_listener.h"

namespace OHOS::Request {

class Request {
public:
    Request(const std::string &taskId) : taskId_(taskId), events_(0U)
    {
    }

    const std::string &getId() const
    {
        return this->taskId_;
    }

    void AddListener(const SubscribeType &type, const std::shared_ptr<IResponseListener> &listener)
    {
        if (type == SubscribeType::RESPONSE) {
            responseListener_ = listener;
        }
    }

    void RemoveListener(const SubscribeType &type, const std::shared_ptr<IResponseListener> &listener)
    {
        if (type == SubscribeType::RESPONSE) {
            responseListener_.reset();
        }
    }

    void AddListener(const SubscribeType &type, const std::shared_ptr<INotifyDataListener> &listener)
    {
        if (type != SubscribeType::RESPONSE && type < SubscribeType::BUTT) {
            notifyDataListenerMap_[type] = listener;
        }
    }

    void RemoveListener(const SubscribeType &type, const std::shared_ptr<INotifyDataListener> &listener)
    {
        if (type != SubscribeType::RESPONSE && type < SubscribeType::BUTT) {
            notifyDataListenerMap_.erase(type);
        }
    }

    bool IsEventSubscribed(SubscribeType eventType)
    {
        uint32_t type = 1 << static_cast<uint32_t>(eventType);
        return ((events_ & type) == type);
    }

    void MarkEventSubscribed(SubscribeType eventType, bool subscribed)
    {
        uint32_t type = 1 << static_cast<uint32_t>(eventType);
        if (subscribed) {
            events_ |= type;
        } else {
            events_ &= (~type);
        }
    }

    bool HasListener() const
    {
        if (responseListener_ != nullptr) {
            return true;
        }
        return !notifyDataListenerMap_.empty();
    }

    void OnResponseReceive(const std::shared_ptr<Response> &response)
    {
        if (responseListener_ != nullptr) {
            responseListener_->OnResponseReceive(response);
        }
    }

    void OnNotifyDataReceive(const std::shared_ptr<NotifyData> &notifyData)
    {
        auto listener = notifyDataListenerMap_.find(notifyData->type);
        if (listener != notifyDataListenerMap_.end()) {
            listener->second->OnNotifyDataReceive(notifyData);
        }
    }

private:
    const std::string taskId_;
    uint32_t events_;
    std::shared_ptr<IResponseListener> responseListener_;
    std::map<SubscribeType, std::shared_ptr<INotifyDataListener>> notifyDataListenerMap_;
};

} // namespace OHOS::Request

#endif // OHOS_REQUEST_REQUEST_H