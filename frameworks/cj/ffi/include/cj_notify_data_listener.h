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

#ifndef OHOS_REQUEST_CJ_NOTIFY_DATA_LISTENER_H
#define OHOS_REQUEST_CJ_NOTIFY_DATA_LISTENER_H

#include "cj_listener_list.h"
#include "i_notify_data_listener.h"
#include "request_common.h"

namespace OHOS::CJSystemapi::Request {
using OHOS::Request::INotifyDataListener;
using OHOS::Request::NotifyData;
using OHOS::Request::SubscribeType;
using OHOS::Request::Reason;

class CJNotifyDataListener : public INotifyDataListener,
                             public ListenerList,
                             public std::enable_shared_from_this<CJNotifyDataListener> {
public:
    CJNotifyDataListener(const std::string &taskId, const SubscribeType &type) : ListenerList(taskId, type)
    {
    }
    void AddListener(std::function<void(CProgress)> cb, CFunc cbId);
    void RemoveListener(CFunc cbId = nullptr);
    void OnNotifyDataReceive(const std::shared_ptr<NotifyData> &notifyData) override;
    void OnFaultsReceive(const std::shared_ptr<int32_t> &tid, const std::shared_ptr<SubscribeType> &type,
        const std::shared_ptr<Reason> &reason) override;
    void OnWaitReceive(std::int32_t taskId, OHOS::Request::WaitingReason reason) override;

private:
    bool IsHeaderReceive(const std::shared_ptr<NotifyData> &notifyData);
    void ProcessHeaderReceive(const std::shared_ptr<NotifyData> &notifyData);
    void NotifyDataProcess(const std::shared_ptr<NotifyData> &notifyData);
};

} // namespace OHOS::CJSystemapi::Request

#endif // OHOS_REQUEST_JS_NOTIFY_DATA_LISTENER_H
