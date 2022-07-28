/*
 * Copyright (c) 2021 Huawei Device Co., Ltd.
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

#ifndef COMMON_EVENT_H
#define COMMON_EVENT_H

#include "common_event_manager.h"
#include "common_event_subscriber.h"

namespace OHOS::Request::Download {
constexpr const char *DOWNLOAD_EVENT = "com.ohos.downloaddataability";

constexpr int DOWNLOAD_EVENT_CODE = 10000;

// action
constexpr int DOWNLOAD_INSERT = 0;
constexpr int DOWNLOAD_UPDATE = 1;
constexpr int DOWNLOAD_DELETE = 2;

class CommonEvent : public OHOS::EventFwk::CommonEventSubscriber {
public:
    CommonEvent(const OHOS::EventFwk::CommonEventSubscribeInfo &subscriberInfo) : CommonEventSubscriber(subscriberInfo)
    {
    }
    ~CommonEvent() = default;
    static std::shared_ptr<CommonEvent> subscriber;
    void OnReceiveEvent(const OHOS::EventFwk::CommonEventData &data);
    static bool PublishEvent(const OHOS::AAFwk::Want &want, int eventCode, const std::string &eventData);
    static void UnregisterSubscriber(std::shared_ptr<OHOS::EventFwk::CommonEventSubscriber> subscriber);
    static void RegisterSubscriber();
    static void SendChange(int actionCode);
};
} // namespace OHOS::Request::Download
#endif // COMMON_EVENT_H