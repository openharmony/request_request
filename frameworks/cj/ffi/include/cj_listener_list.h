/*
 * Copyright (c) 2024 Huawei Device Co., Ltd.
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

#ifndef OHOS_REQUEST_CJ_LISTENER_LIST_H
#define OHOS_REQUEST_CJ_LISTENER_LIST_H

#include <functional>
#include <list>
#include <mutex>
#include <string>

#include "cj_request_ffi.h"
#include "request_common.h"

namespace OHOS::CJSystemapi::Request {
using OHOS::Request::NotifyData;
using OHOS::Request::Response;
using OHOS::Request::SubscribeType;

using CFunc = void *;

class ListenerList {
public:
    ListenerList(const std::string &taskId, const SubscribeType &type) : taskId_(taskId), type_(type)
    {
    }
    bool HasListener();
    struct CallBackInfo {
        std::function<void(CProgress)> progressCB_;
        std::function<void(CResponse)> responseCB_;
        CFunc cbId_ = nullptr;

        CallBackInfo(std::function<void(CProgress)> cb, CFunc cbId) : progressCB_(cb), cbId_(cbId) {}

        CallBackInfo(std::function<void(CResponse)> cb, CFunc cbId) : responseCB_(cb), cbId_(cbId) {}
    };

protected:
    bool IsListenerAdded(void *cb);
    void OnMessageReceive(const std::shared_ptr<NotifyData> &notifyData);
    void OnMessageReceive(const std::shared_ptr<Response> &response);
    void AddListenerInner(std::function<void(CProgress)> &cb, CFunc cbId);
    void AddListenerInner(std::function<void(CResponse)> &cb, CFunc cbId);
    void RemoveListenerInner(CFunc cb);

protected:
    const std::string taskId_;
    const SubscribeType type_;

    std::recursive_mutex allCbMutex_;
    std::list<std::pair<bool, std::shared_ptr<CallBackInfo>>> allCb_;
    std::atomic<uint32_t> validCbNum{0};
};
} // namespace OHOS::CJSystemapi::Request
#endif // OHOS_REQUEST_CJ_LISTENER_LIST_H
