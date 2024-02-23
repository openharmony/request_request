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

#include "i_response_listener.h"

namespace OHOS::Request {

class Request {
public:
    static constexpr uint32_t EVENT_NONE = 0;
    static constexpr uint32_t EVENT_RESPONSE = (1 << 0);

public:
    Request(const std::string &taskId) : taskId_(taskId), events_(0U)
    {
    }

    const std::string &getId() const
    {
        return this->taskId_;
    }

    size_t AddListener(const std::shared_ptr<IResponseListener> &listener)
    {
        this->responseListeners_.emplace(listener);
        return this->responseListeners_.size();
    }

    size_t RemoveListener(const std::shared_ptr<IResponseListener> &listener)
    {
        this->responseListeners_.erase(listener);
        return this->responseListeners_.size();
    }

    bool IsEventSubscribed(uint32_t eventType)
    {
        return ((events_ & eventType) == eventType);
    }

    void MarkEventSubscribed(uint32_t eventType, bool subscribed)
    {
        if (subscribed) {
            events_ |= eventType;
        } else {
            events_ &= (~eventType);
        }
    }

    bool HasListener() const
    {
        return !(this->responseListeners_.empty());
    }

    void OnResponseReceive(const std::shared_ptr<Response> &response)
    {
        for (auto responseListener : responseListeners_) {
            responseListener->OnResponseReceive(response);
        }
    }

private:
    const std::string taskId_;
    uint32_t events_;
    std::set<std::shared_ptr<IResponseListener>> responseListeners_;
};

} // namespace OHOS::Request

#endif // OHOS_REQUEST_REQUEST_H