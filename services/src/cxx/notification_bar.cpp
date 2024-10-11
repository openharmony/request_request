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

#include "cxx.h"
#include "image_source.h"
#include "log.h"
#include "notification.h"
#include "notification_action_button.h"
#include "notification_content.h"
#include "notification_helper.h"
#include "notification_local_live_view_button.h"
#include "notification_local_live_view_content.h"
#include "service/notification_bar.rs.h"
#include "string_wrapper.h"
#include "task/config.rs.h"
namespace OHOS::Request {
static constexpr int32_t REQUEST_SERVICE_ID = 3815;

static constexpr int32_t REQUEST_STYLE_SIMPLE = 8;
static constexpr int32_t REQUEST_STYLE_WITH_PAUSE_BUTTON = 13;

static constexpr uint32_t BINARY_SCALE = 1024;
static constexpr uint32_t PERCENT = 100;
static constexpr uint32_t FRONT_ZERO = 10;

static const std::string CLOSE_ICON_PATH = "/etc/request/xmark.svg";

std::shared_ptr<Media::PixelMap> CreatePixelMap()
{
    static std::shared_ptr<Media::PixelMap> pixelMap = nullptr;
    static std::once_flag flag;

    std::call_once(flag, []() {
        uint32_t errorCode = 0;
        Media::SourceOptions opts;
        auto source = Media::ImageSource::CreateImageSource(CLOSE_ICON_PATH, opts, errorCode);
        if (source == nullptr) {
            REQUEST_HILOGE("create image source failed");
            return;
        }
        Media::DecodeOptions decodeOpts;
        std::unique_ptr<Media::PixelMap> pixel = source->CreatePixelMap(decodeOpts, errorCode);
        if (pixel == nullptr) {
            REQUEST_HILOGE("create pixel map failed");
            return;
        }
        pixelMap = std::move(pixel);
    });
    return pixelMap;
}

void SetProgress(
    std::shared_ptr<Notification::NotificationLocalLiveViewContent> &localLiveViewContent, RequestTaskMsg msg)
{
    std::string title;
    Notification::NotificationProgress progress;
    progress.SetIsPercentage(true);
    if (msg.action == static_cast<uint8_t>(Action::Download)) {
        title = "下载文件 ";
        progress.SetCurrentValue(msg.processed[0] / BINARY_SCALE);
        if (msg.sizes[0] == -1) {
            TitleWithProgressSized(title, msg.processed[0]);
        } else {
            progress.SetMaxValue(msg.sizes[0] / BINARY_SCALE);
            TitleWithProgressPercentage(title, msg.processed[0], msg.sizes[0]);
        }
    } else {
        title = "上传文件 ";
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
}

void RequestProgressNotification(RequestTaskMsg msg)
{
    Notification::NotificationRequest request(msg.task_id);
    std::shared_ptr<Notification::NotificationLocalLiveViewContent> localLiveViewContent =
        std::make_shared<Notification::NotificationLocalLiveViewContent>();

    // basic settings
    request.SetSlotType(Notification::NotificationConstant::SlotType::LIVE_VIEW);
    localLiveViewContent->SetContentType(
        static_cast<int32_t>(Notification::NotificationContent::Type::LOCAL_LIVE_VIEW));

    BasicRequestSettings(request, msg.uid);

    request.SetInProgress(true);
    if (msg.support_range && msg.action == static_cast<uint8_t>(Action::Download)) {
        localLiveViewContent->SetType(REQUEST_STYLE_WITH_PAUSE_BUTTON);
    } else {
        localLiveViewContent->SetType(REQUEST_STYLE_SIMPLE);
    }

    localLiveViewContent->addFlag(Notification::NotificationLocalLiveViewContent::LiveViewContentInner::BUTTON);
    localLiveViewContent->addFlag(Notification::NotificationLocalLiveViewContent::LiveViewContentInner::PROGRESS);

    // set text
    localLiveViewContent->SetText(std::string(msg.file_name));

    // set button
    auto button = localLiveViewContent->GetButton();
    auto icon = CreatePixelMap();
    if (icon != nullptr) {
        button.addSingleButtonName("cancel");
        button.addSingleButtonIcon(icon);
        localLiveViewContent->SetButton(button);
    }

    // set title and progress
    SetProgress(localLiveViewContent, msg);

    // set content
    auto content = std::make_shared<Notification::NotificationContent>(localLiveViewContent);
    request.SetContent(content);

    OHOS::ErrCode errCode = Notification::NotificationHelper::PublishNotification(request);
    if (errCode != OHOS::ERR_OK) {
        REQUEST_HILOGE("%{public}d publish notification error %{public}d", msg.task_id, errCode);
    }
}

void RequestCompletedNotification(uint8_t action, uint32_t taskId, int32_t uid, rust::string fileName, bool isSucceed)
{
    Notification::NotificationRequest request(taskId);
    std::shared_ptr<Notification::NotificationNormalContent> normalContent =
        std::make_shared<Notification::NotificationNormalContent>();

    // basic settings
    BasicRequestSettings(request, uid);

    // set text
    normalContent->SetText(std::string(fileName));

    // set title
    std::string title;
    if (action == static_cast<uint8_t>(Action::Download)) {
        if (isSucceed) {
            title = "下载成功";
        } else {
            title = "下载失败";
        }
    } else {
        if (isSucceed) {
            title = "上传成功";
        } else {
            title = "上传失败";
        }
    }
    normalContent->SetTitle(title);

    // set content
    auto content = std::make_shared<Notification::NotificationContent>(normalContent);
    request.SetContent(content);

    OHOS::ErrCode errCode = Notification::NotificationHelper::PublishNotification(request);
    if (errCode != OHOS::ERR_OK) {
        REQUEST_HILOGE("%{public}d publish notification error %{public}d", taskId, errCode);
    }
}

void BasicRequestSettings(Notification::NotificationRequest &request, int32_t uid)
{
    // basic settings
    request.SetCreatorUid(REQUEST_SERVICE_ID);
    request.SetOwnerUid(uid);
    request.SetIsAgentNotification(true);
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
    } else if (buttonOption->GetButtonName() == "cancel") {
        this->_taskManager->stop_task(static_cast<uint32_t>(notificationId));
        Notification::NotificationHelper::CancelNotification(notificationId);
    }
};

void SubscribeNotification(rust::Box<TaskManagerWrapper> taskManager)
{
    auto subscriber = new NotificationSubscriber(std::move(taskManager));
    Notification::NotificationHelper::SubscribeLocalLiveViewNotification(*subscriber);
}

} // namespace OHOS::Request