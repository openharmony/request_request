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

#include "common_event.h"

#include <memory>

#include "common_event_manager.h"
#include "common_event_subscribe_info.h"
#include "common_event_subscriber.h"
#include "cxx.h"
#include "utils/common_event.rs.h"

namespace OHOS::Request {
EventSubscriber::EventSubscriber(EventFwk::CommonEventSubscribeInfo &subscribeInfo, rust::Box<EventHandler> handler)
    : CommonEventSubscriber(subscribeInfo)
{
    _handler = handler.into_raw();
}

EventSubscriber::~EventSubscriber()
{
    rust::Box<EventHandler>::from_raw(_handler);
}

WantWrapper::WantWrapper(EventFwk::Want want) : want_(want)
{
}

void EventSubscriber::OnReceiveEvent(const EventFwk::CommonEventData &data)
{
    _handler->on_receive_event(
        data.GetCode(), rust::string(data.GetData()), std::make_unique<WantWrapper>(data.GetWant()));
}

int SubscribeCommonEvent(rust::Vec<rust::Str> events, rust::Box<EventHandler> handler)
{
    EventFwk::MatchingSkills matchingSkills;
    for (auto event : events) {
        matchingSkills.AddEvent(std::string(event));
    }

    EventFwk::CommonEventSubscribeInfo subscribeInfo = EventFwk::CommonEventSubscribeInfo(matchingSkills);
    auto subscriber = std::make_shared<EventSubscriber>(subscribeInfo, std::move(handler));

    return EventFwk::CommonEventManager::NewSubscribeCommonEvent(subscriber);
}

rust::string WantWrapper::ToString() const
{
    return rust::string(want_.ToString());
}

int WantWrapper::GetIntParam(rust::str key) const
{
    return want_.GetIntParam(std::string(key), -1);
}
} // namespace OHOS::Request
