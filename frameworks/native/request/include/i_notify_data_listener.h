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

#ifndef OHOS_REQUEST_I_NOTIFY_DATA_LISTENER_H
#define OHOS_REQUEST_I_NOTIFY_DATA_LISTENER_H

#include "request_common.h"

namespace OHOS::Request {

class INotifyDataListener {
public:
    virtual ~INotifyDataListener() = default;
    virtual void OnNotifyDataReceive(const std::shared_ptr<NotifyData> &notifyData) = 0;
    virtual void OnFaultsReceive(const std::shared_ptr<int32_t> &tid, const std::shared_ptr<SubscribeType> &type,
        const std::shared_ptr<Reason> &reason) = 0;
    virtual void OnWaitReceive(std::int32_t taskId, WaitingReason reason) = 0;
};

} // namespace OHOS::Request

#endif // OHOS_REQUEST_I_NOTIFY_DATA_LISTENER_H