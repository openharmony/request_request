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

#ifndef REQUEST_NOTIFICATION_BAR_H
#define REQUEST_NOTIFICATION_BAR_H

#include <cstdint>

#include "cxx.h"
#include "notification_button_option.h"
#include "notification_helper.h"
#include "notification_local_live_view_subscriber.h"
namespace OHOS::Request {

struct TaskManagerWrapper;
struct NotifyContent;
struct ProgressCircle;

rust::string GetSystemResourceString(const rust::str);
rust::string GetSystemLanguage();
int PublishNotification(const NotifyContent &content);

class NotificationSubscriber : public Notification::NotificationLocalLiveViewSubscriber {
public:
    NotificationSubscriber(rust::Box<TaskManagerWrapper> taskManager);
    void OnConnected() override;
    void OnDisconnected() override;
    void OnResponse(int32_t notificationId, sptr<Notification::NotificationButtonOption> buttonOption) override;
    void OnDied() override;

private:
    rust::Box<TaskManagerWrapper> _taskManager;
};

void SubscribeNotification(rust::Box<TaskManagerWrapper> taskManager);

inline int32_t CancelNotification(uint32_t notificationId)
{
    return Notification::NotificationHelper::CancelNotification(notificationId);
}

} // namespace OHOS::Request

#endif