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

#include <list>
#include <mutex>
#include <string>
#include <functional>
#include "js_common.h"
#include "cj_request_ffi.h"

namespace OHOS::CJSystemapi::Request {
using OHOS::Request::SubscribeType;
using OHOS::Request::ExceptionErrorCode;
using OHOS::Request::NotifyData;

using CFunc = void *;
using ProgressOnCallBackType  = std::function<void(CProgress)>;

class ListenerList {
public:
    ListenerList(const std::string &taskId, const SubscribeType &type)
        : taskId_(taskId), type_(type)
    {
    }
    bool HasListener();
    struct CallBackInfo {
        ProgressOnCallBackType cb_;
        CFunc cbId_ = nullptr;

        CallBackInfo(ProgressOnCallBackType cb_, CFunc cbId_)
            : cb_(cb_), cbId_(cbId_) {}
    };

protected:
    bool IsListenerAdded(void *cb);
    void OnMessageReceive(const std::shared_ptr<NotifyData> &notifyData);
    void AddListenerInner(ProgressOnCallBackType &cb, CFunc cbId);
    void RemoveListenerInner(CFunc cbId);

protected:
    const std::string taskId_;
    const SubscribeType type_;

    std::recursive_mutex allCbMutex_;
    std::list<std::pair<bool, std::shared_ptr<CallBackInfo>>> allCb_;
    std::atomic<uint32_t> validCbNum{ 0 };
};



} // namespace OHOS::Request

#endif // OHOS_REQUEST_LISTENER_LIST_H
