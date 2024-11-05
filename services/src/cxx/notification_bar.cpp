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
#include "locale_config.h"
#include "log.h"
#include "notification.h"
#include "notification_content.h"
#include "notification_local_live_view_button.h"
#include "notification_local_live_view_content.h"
#include "resource_manager.h"
#include "service/notification_bar.rs.h"
#include "task/config.rs.h"

namespace OHOS::Request {
using namespace Global;

static constexpr int32_t REQUEST_SERVICE_ID = 3815;

static constexpr int32_t REQUEST_STYLE_SIMPLE = 8;

static constexpr uint32_t BINARY_SCALE = 1024;
static constexpr uint32_t PERCENT = 100;
static constexpr uint32_t FRONT_ZERO = 10;
static constexpr size_t PLACEHOLDER_LENGTH = 2;

constexpr const char *DOWNLOAD_FILE = "ohos_id_text_save_button_description_download_file";
constexpr const char *DOWNLOAD_SUCCESS = "request_agent_download_success";
constexpr const char *DOWNLOAD_FAIL = "request_agent_download_fail";
constexpr const char *UPLOAD_FILE = "request_agent_upload_file";
constexpr const char *UPLOAD_SUCCESS = "request_agent_upload_success";
constexpr const char *UPLOAD_FAIL = "request_agent_upload_fail";

static const std::string CLOSE_ICON_PATH = "/etc/request/xmark.svg";

std::string GetSystemResourceString(const char *name)
{
    auto resourceMgr = Resource::GetSystemResourceManagerNoSandBox();
    if (resourceMgr == nullptr) {
        REQUEST_HILOGE("GetSystemResourceManagerNoSandBox failed");
        return "";
    }
    std::unique_ptr<Resource::ResConfig> config(Resource::CreateResConfig());
    if (config == nullptr) {
        REQUEST_HILOGE("Create ResConfig failed");
        return "";
    }
    UErrorCode status = U_ZERO_ERROR;
    icu::Locale locale = icu::Locale::forLanguageTag(I18n::LocaleConfig::GetSystemLanguage(), status);
    config->SetLocaleInfo(locale);
    resourceMgr->UpdateResConfig(*config);

    std::string outValue;
    auto ret = resourceMgr->GetStringByName(name, outValue);
    if (ret != Resource::RState::SUCCESS) {
        REQUEST_HILOGE("GetStringById failed: %{public}d", ret);
    }
    return outValue;
}

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
    if (msg.action == static_cast<uint8_t>(Action::Download)) {
        title = GetSystemResourceString(DOWNLOAD_FILE);
        title.push_back(' ');
        if (msg.sizes[0] == -1) {
            title += ProgressSized(msg.processed[0]);
            localLiveViewContent->SetTitle(title);
            return;
        } else {
            localLiveViewContent->addFlag(
                Notification::NotificationLocalLiveViewContent::LiveViewContentInner::PROGRESS);
            progress.SetIsPercentage(true);
            progress.SetCurrentValue(msg.processed[0] / BINARY_SCALE);
            progress.SetMaxValue(msg.sizes[0] / BINARY_SCALE);
            title += ProgressPercentage(msg.processed[0], msg.sizes[0]);
        }
    } else {
        localLiveViewContent->addFlag(Notification::NotificationLocalLiveViewContent::LiveViewContentInner::PROGRESS);
        title = GetSystemResourceString(UPLOAD_FILE);
        if (msg.sizes.size() > 1) {
            progress.SetCurrentValue(msg.index);
            progress.SetMaxValue(msg.sizes.size());
            size_t pos = title.find("%d");
            if (pos != std::string::npos) {
                title.replace(pos, PLACEHOLDER_LENGTH, ProgressNum(msg.index, msg.sizes.size()));
            } else {
                title.push_back(' ');
                title += ProgressNum(msg.index, msg.sizes.size());
            }
        } else {
            progress.SetCurrentValue(msg.processed[0] / BINARY_SCALE);
            progress.SetMaxValue(msg.sizes[0] / BINARY_SCALE);
            size_t pos = title.find("%d");
            if (pos != std::string::npos) {
                title.replace(pos, PLACEHOLDER_LENGTH, ProgressPercentage(msg.processed[0], msg.sizes[0]));
            } else {
                title.push_back(' ');
                title += ProgressPercentage(msg.processed[0], msg.sizes[0]);
            }
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
    localLiveViewContent->SetType(REQUEST_STYLE_SIMPLE);

    localLiveViewContent->addFlag(Notification::NotificationLocalLiveViewContent::LiveViewContentInner::BUTTON);

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
            title = GetSystemResourceString(DOWNLOAD_SUCCESS);
        } else {
            title = GetSystemResourceString(DOWNLOAD_FAIL);
        }
    } else {
        if (isSucceed) {
            title = GetSystemResourceString(UPLOAD_SUCCESS);
        } else {
            title = GetSystemResourceString(UPLOAD_FAIL);
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

std::string ProgressNum(std::size_t uploaded, std::size_t total)
{
    std::string content;
    content += std::to_string(uploaded);
    content += "/";
    content += std::to_string(total);
    return content;
}

std::string ProgressPercentage(std::size_t processed, std::size_t size)
{
    std::string content;
    if (size == 0) {
        content += "100";
    } else {
        content += std::to_string(processed * PERCENT / size);
    }
    content += "%";
    return content;
}

std::string ProgressSized(std::size_t processed)
{
    std::string content;
    if (processed < BINARY_SCALE) {
        content += std::to_string(processed);
        content += "B";
        return content;
    }
    int remainder = (processed % BINARY_SCALE) * PERCENT / BINARY_SCALE;
    processed /= BINARY_SCALE;
    if (processed < BINARY_SCALE) {
        WithRemainder(content, processed, remainder);
        content += "KB";
        return content;
    }
    remainder = (processed % BINARY_SCALE) * PERCENT / BINARY_SCALE;
    processed /= BINARY_SCALE;
    if (processed < BINARY_SCALE) {
        WithRemainder(content, processed, remainder);
        content += "MB";
    } else {
        remainder = (processed % BINARY_SCALE) * PERCENT / BINARY_SCALE;
        processed = processed / BINARY_SCALE;
        WithRemainder(content, processed, remainder);
        content += "GB";
    }
    return content;
}

void WithRemainder(std::string &content, size_t processed, size_t remainder)
{
    content += std::to_string(processed);
    content += ".";
    if (remainder < FRONT_ZERO) {
        content += "0";
    }
    content += std::to_string(remainder);
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