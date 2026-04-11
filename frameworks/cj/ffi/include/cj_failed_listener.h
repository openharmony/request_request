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

#ifndef OHOS_REQUEST_CJ_FAILED_LISTENER_H
#define OHOS_REQUEST_CJ_FAILED_LISTENER_H

#include <functional>
#include <list>
#include <mutex>
#include <string>

#include "i_notify_data_listener.h"
#include "request_common.h"

namespace OHOS::CJSystemapi::Request {
using OHOS::Request::INotifyDataListener;
using OHOS::Request::NotifyData;
using OHOS::Request::Reason;
using OHOS::Request::SubscribeType;

using CFunc = void *;

struct CJFailedCallBackInfo {
    std::function<void(int32_t)> cb_;
    CFunc cbId_ = nullptr;
    CJFailedCallBackInfo(std::function<void(int32_t)> cb, CFunc cbId) : cb_(cb), cbId_(cbId) {}
};

class CJFailedListener : public INotifyDataListener,
                          public std::enable_shared_from_this<CJFailedListener> {
public:
    CJFailedListener(const std::string &taskId) : taskId_(taskId)
    {
    }
    void AddListener(std::function<void(int32_t)> cb, CFunc cbId);
    void RemoveListener(CFunc cbId = nullptr);
    void OnNotifyDataReceive(const std::shared_ptr<NotifyData> &notifyData) override;
    void OnFaultsReceive(const std::shared_ptr<int32_t> &tid, const std::shared_ptr<SubscribeType> &type,
        const std::shared_ptr<Reason> &reason) override;
    void OnWaitReceive(std::int32_t taskId, OHOS::Request::WaitingReason reason) override;

private:
    int32_t ConvertToErrCode(const std::shared_ptr<NotifyData> &notifyData);
    bool IsListenerAdded(void *cb);

    const std::string taskId_;
    std::recursive_mutex allCbMutex_;
    std::list<std::pair<bool, std::shared_ptr<CJFailedCallBackInfo>>> allCb_;
    std::atomic<uint32_t> validCbNum{0};
};

} // namespace OHOS::CJSystemapi::Request
#endif // OHOS_REQUEST_CJ_FAILED_LISTENER_H
