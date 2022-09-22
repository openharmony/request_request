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

#include "common_event.h"
#include "log.h"

namespace OHOS::Request::Download {
std::shared_ptr<CommonEvent> CommonEvent::subscriber = nullptr;

void CommonEvent::OnReceiveEvent(const OHOS::EventFwk::CommonEventData &data)
{
    std::string action = data.GetWant().GetAction();
    DOWNLOAD_HILOGI("CommonEvent::OnReceiveEvent action = %{public}s", action.c_str());
    int msgCode = data.GetCode();
    std::string msgData = data.GetData();
    DOWNLOAD_HILOGI("CommonEvent::OnReceiveEvent msgData = %{public}s", msgData.c_str());
    DOWNLOAD_HILOGI("CommonEvent::OnReceiveEvent msgCode = %{public}d", msgCode);
}

bool CommonEvent::PublishEvent(const OHOS::AAFwk::Want &want, int eventCode, const std::string &eventData)
{
    OHOS::EventFwk::CommonEventData data;
    data.SetWant(want);
    data.SetCode(eventCode);
    data.SetData(eventData);
    OHOS::EventFwk::CommonEventPublishInfo publishInfo;
    publishInfo.SetOrdered(true);
    bool publishResult = OHOS::EventFwk::CommonEventManager::PublishCommonEvent(data, publishInfo, nullptr);
    DOWNLOAD_HILOGI("PublishEvent end publishResult = %{public}d", publishResult);
    return publishResult;
}

void CommonEvent::UnregisterSubscriber(std::shared_ptr<OHOS::EventFwk::CommonEventSubscriber> subscriber)
{
    if (subscriber != nullptr) {
        bool subscribeResult = OHOS::EventFwk::CommonEventManager::UnSubscribeCommonEvent(subscriber);
        subscriber = nullptr;
        DOWNLOAD_HILOGI("UnregisterSubscriber end###subscribeResult = %{public}d", subscribeResult);
    }
}

void CommonEvent::RegisterSubscriber()
{
    OHOS::EventFwk::MatchingSkills matchingSkills;
    matchingSkills.AddEvent(DOWNLOAD_EVENT);
    OHOS::EventFwk::CommonEventSubscribeInfo subscriberInfo(matchingSkills);
    subscriber = std::make_shared<CommonEvent>(subscriberInfo);
    bool subscribeResult = OHOS::EventFwk::CommonEventManager::SubscribeCommonEvent(subscriber);
    DOWNLOAD_HILOGI("RegisterSubscriber end###subscribeResult = %{public}d", subscribeResult);
}

void CommonEvent::SendChange(int actionCode)
{
    OHOS::AAFwk::Want want;
    int32_t eventCode = DOWNLOAD_EVENT_CODE;
    want.SetParam("ActionCode", actionCode);
    want.SetAction(DOWNLOAD_EVENT);
    std::string eventData("DataChange");
    PublishEvent(want, eventCode, eventData);
}
} // namespace OHOS::Request::Download