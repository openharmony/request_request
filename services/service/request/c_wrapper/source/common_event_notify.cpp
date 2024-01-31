/*
 * Copyright (C) 2023 Huawei Device Co., Ltd.
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

#include "common_event_notify.h"

#include <want.h>

#include "common_event_data.h"
#include "common_event_manager.h"
#include "common_event_publish_info.h"
#include "log.h"

using namespace OHOS::EventFwk;

void PublishStateChangeEvents(const char *bundleName, uint32_t len, uint32_t taskId, int32_t state)
{
    REQUEST_HILOGD("PublishStateChangeEvents in.");
    static constexpr const char *eventAction = "ohos.request.event.COMPLETE";

    std::string bundle(bundleName, len);
    Want want;
    want.SetAction(eventAction);
    want.SetBundle(bundle);

    std::string data = std::to_string(taskId);
    CommonEventData commonData(want, state, data);
    CommonEventPublishInfo publishInfo;
    publishInfo.SetBundleName(bundle);

    bool res = CommonEventManager::PublishCommonEvent(commonData, publishInfo);
    if (!res) {
        REQUEST_HILOGE("PublishStateChangeEvents failed!");
    }
}