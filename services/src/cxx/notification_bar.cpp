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

#include "notification_bar.h"

#include <cstddef>
#include <cstdint>
#include <string>

#include "account.h"
#include "ans_convert_enum.h"
#include "cxx.h"
#include "image_type.h"
#include "int_wrapper.h"
#include "log.h"
#include "notification.h"
#include "notification_constant.h"
#include "notification_content.h"
#include "notification_helper.h"
#include "notification_local_live_view_button.h"
#include "notification_local_live_view_content.h"
#include "notification_local_live_view_subscriber.h"
#include "notification_progress.h"
#include "notification_request.h"
#include "notification_subscriber.h"
#include "pixel_map.h"
#include "resource_manager.h"
#include "service/notification_bar.rs.h"
#include "string_wrapper.h"
#include "task/config.rs.h"
namespace OHOS::Request {
static constexpr int32_t REQUEST_SERVICE_ID = 3815;
static constexpr int32_t REQUEST_STYLE = 13;

static constexpr uint32_t BINARY_SCALE = 1024;
static constexpr uint32_t PERCENT = 100;
static constexpr uint32_t FRONT_ZERO = 10;

void RequestNotifyProgress(RequestTaskMsg msg)
{
    Notification::NotificationRequest request(msg.task_id);
    std::shared_ptr<Notification::NotificationLocalLiveViewContent> localLiveViewContent =
        std::make_shared<Notification::NotificationLocalLiveViewContent>();

    // basic settings
    BasicRequestSettings(request, localLiveViewContent, msg.uid);
    request.SetInProgress(true);
    localLiveViewContent->addFlag(Notification::NotificationLocalLiveViewContent::LiveViewContentInner::BUTTON);
    localLiveViewContent->addFlag(Notification::NotificationLocalLiveViewContent::LiveViewContentInner::PROGRESS);

    // set text
    localLiveViewContent->SetText(std::string(msg.file_name));

    // set button
    Notification::NotificationLocalLiveViewButton button;
    localLiveViewContent->SetButton(button);

    // set title and progress
    std::string title;
    Notification::NotificationProgress progress;
    progress.SetIsPercentage(true);
    if (msg.action == static_cast<uint8_t>(Action::Download)) {
        title = "已下载 ";
        progress.SetCurrentValue(msg.processed[0] / BINARY_SCALE);
        if (msg.sizes[0] == -1) {
            TitleWithProgressSized(title, msg.processed[0]);
        } else {
            progress.SetMaxValue(msg.sizes[0] / BINARY_SCALE);
            TitleWithProgressPercentage(title, msg.processed[0], msg.sizes[0]);
        }
    } else {
        title = "已上传 ";
        if (msg.sizes.size() > 1) {
            progress.SetCurrentValue(msg.index);
            progress.SetMaxValue(msg.sizes.size());
            TitleWithProgressNum(title, msg.index, msg.sizes.size());
        } else {
            progress.SetCurrentValue(msg.processed[0] / BINARY_SCALE);
            progress.SetMaxValue(msg.sizes[0] / BINARY_SCALE);
            TitleWithProgressPercentage(title, msg.processed[0], msg.sizes[0]);
        }
    }
    localLiveViewContent->SetTitle(title);
    localLiveViewContent->SetProgress(progress);

    // set content
    auto content = std::make_shared<Notification::NotificationContent>(localLiveViewContent);
    request.SetContent(content);

    OHOS::ErrCode errCode = Notification::NotificationHelper::PublishNotification(request);
    if (errCode != OHOS::ERR_OK) {
        REQUEST_HILOGE("%{public}d publish notification error %{public}d", msg.task_id, errCode);
    }
}

void RequestNotifyCompleted(uint8_t action, uint32_t taskId, int32_t uid, rust::string fileName)
{
    Notification::NotificationRequest request(taskId);
    std::shared_ptr<Notification::NotificationLocalLiveViewContent> localLiveViewContent =
        std::make_shared<Notification::NotificationLocalLiveViewContent>();

    // basic settings
    BasicRequestSettings(request, localLiveViewContent, uid);

    // set text
    localLiveViewContent->SetText(std::string(fileName));

    // set title and progress
    std::string title;
    if (action == static_cast<uint8_t>(Action::Download)) {
        title = "下载成功";
    } else {
        title = "上传成功";
    }
    localLiveViewContent->SetTitle(title);

    // set content
    auto content = std::make_shared<Notification::NotificationContent>(localLiveViewContent);
    request.SetContent(content);

    OHOS::ErrCode errCode = Notification::NotificationHelper::PublishNotification(request);
    if (errCode != OHOS::ERR_OK) {
        REQUEST_HILOGE("%{public}d publish notification error %{public}d", taskId, errCode);
    }
}

void RequestNotifyFailed(uint8_t action, uint32_t taskId, int32_t uid, rust::string fileName)
{
    Notification::NotificationRequest request(taskId);
    std::shared_ptr<Notification::NotificationLocalLiveViewContent> localLiveViewContent =
        std::make_shared<Notification::NotificationLocalLiveViewContent>();

    // basic settings
    BasicRequestSettings(request, localLiveViewContent, uid);

    // set text
    localLiveViewContent->SetText(std::string(fileName));

    // set title and progress
    std::string title;
    if (action == static_cast<uint8_t>(Action::Download)) {
        title = "下载失败";
    } else {
        title = "上传失败";
    }
    localLiveViewContent->SetTitle(title);

    // set content
    auto content = std::make_shared<Notification::NotificationContent>(localLiveViewContent);
    request.SetContent(content);

    OHOS::ErrCode errCode = Notification::NotificationHelper::PublishNotification(request);
    if (errCode != OHOS::ERR_OK) {
        REQUEST_HILOGE("%{public}d publish notification error %{public}d", taskId, errCode);
    }
}

void BasicRequestSettings(Notification::NotificationRequest &request,
    std::shared_ptr<Notification::NotificationLocalLiveViewContent> &localLiveViewContent, int32_t uid)
{
    // basic settings
    request.SetSlotType(Notification::NotificationConstant::SlotType::LIVE_VIEW);
    request.SetCreatorUid(REQUEST_SERVICE_ID);
    request.SetOwnerUid(uid);
    request.SetIsAgentNotification(true);
    localLiveViewContent->SetType(REQUEST_STYLE);
    localLiveViewContent->SetContentType(
        static_cast<int32_t>(Notification::NotificationContent::Type::LOCAL_LIVE_VIEW));
    localLiveViewContent->addFlag(Notification::NotificationLocalLiveViewContent::LiveViewContentInner::TIME);
}

void TitleWithProgressNum(std::string &title, std::size_t uploaded, std::size_t total)
{
    title += std::to_string(uploaded);
    title += "/";
    title += std::to_string(total);
}

void TitleWithProgressPercentage(std::string &title, std::size_t processed, std::size_t size)
{
    if (size == 0) {
        title += "100";
    } else {
        title += std::to_string(processed * PERCENT / size);
    }
    title += "%";
}

void TitleWithProgressSized(std::string &title, std::size_t processed)
{
    if (processed < BINARY_SCALE) {
        title += std::to_string(processed);
        title += "b";
        return;
    }
    int remainder = (processed % BINARY_SCALE) * PERCENT / BINARY_SCALE;
    processed /= BINARY_SCALE;
    if (processed < BINARY_SCALE) {
        WithRemainder(title, processed, remainder);
        title += "kb";
        return;
    }
    remainder = (processed % BINARY_SCALE) * PERCENT / BINARY_SCALE;
    processed /= BINARY_SCALE;
    if (processed < BINARY_SCALE) {
        WithRemainder(title, processed, remainder);
        title += "mb";
    } else {
        remainder = (processed % BINARY_SCALE) * PERCENT / BINARY_SCALE;
        processed = processed / BINARY_SCALE;
        WithRemainder(title, processed, remainder);
        title += "G";
    }
}

void WithRemainder(std::string &title, size_t processed, size_t remainder)
{
    title += std::to_string(processed);
    title += ".";
    if (remainder < FRONT_ZERO) {
        title += "0";
    }
    title += std::to_string(remainder);
}

NotificationSubscriber::NotificationSubscriber(rust::Box<TaskManagerWrapper> taskManager)
    : _taskManager(std::move(taskManager)){};

void NotificationSubscriber::OnConnected(){};
void NotificationSubscriber::OnDisconnected(){};
void NotificationSubscriber::OnDied(){};
void NotificationSubscriber::OnResponse(
    int32_t notificationId, sptr<Notification::NotificationButtonOption> buttonOption)
{
    if (buttonOption->GetButtonName() == "stop") {
        this->_taskManager->pause_task(static_cast<uint32_t>(notificationId));
    } else if (buttonOption->GetButtonName() == "start") {
        this->_taskManager->resume_task(static_cast<uint32_t>(notificationId));
    }
};

void SubscribeNotification(rust::Box<TaskManagerWrapper> taskManager)
{
    auto subscriber = new NotificationSubscriber(std::move(taskManager));
    Notification::NotificationHelper::SubscribeLocalLiveViewNotification(*subscriber);
}

} // namespace OHOS::Request