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

struct RequestTaskMsg;
struct TaskManagerWrapper;

void RequestProgressNotification(RequestTaskMsg msg);
void RequestCompletedNotification(uint8_t action, uint32_t taskId, int32_t uid, rust::string fileName, bool isSucceed);

void TitleWithProgressNum(std::string &title, std::size_t uploaded, std::size_t total);
void TitleWithProgressSized(std::string &title, std::size_t processed);
void TitleWithProgressPercentage(std::string &title, std::size_t processed, std::size_t size);
void WithRemainder(std::string &title, size_t processed, size_t remainder);

void BasicRequestSettings(Notification::NotificationRequest &request, int32_t uid);

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