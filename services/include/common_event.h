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

#ifndef REQUEST_COMMON_EVENT_H
#define REQUEST_COMMON_EVENT_H

#include <memory>

#include "common_event_data.h"
#include "common_event_subscribe_info.h"
#include "common_event_subscriber.h"
#include "cxx.h"

namespace OHOS::Request {
struct EventHandler;

class EventSubscriber : public EventFwk::CommonEventSubscriber {
public:
    EventSubscriber(EventFwk::CommonEventSubscribeInfo &subscribeInfo, rust::Box<EventHandler> handler);
    ~EventSubscriber();
    void OnReceiveEvent(const EventFwk::CommonEventData &data) override;

private:
    EventHandler *_handler;
};

int SubscribeCommonEvent(rust::Vec<rust::Str> events, rust::Box<EventHandler> handler);

class WantWrapper {
public:
    WantWrapper(EventFwk::Want want);
    rust::String ToString() const;
    int GetIntParam(rust::str key) const;

private:
    EventFwk::Want want_;
};

} // namespace OHOS::Request
#endif // REQUEST_COMMON_EVENT_H