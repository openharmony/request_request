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
#include "service/notification_bar/mod.rs.h"
#include "task/config.rs.h"

#include "want_agent_helper.h"

namespace OHOS::Request {
using namespace Global;

static constexpr int32_t REQUEST_SERVICE_ID = 3815;

static constexpr int32_t REQUEST_STYLE_SIMPLE = 8;

// static constexpr uint32_t BINARY_SCALE = 1024;
// static constexpr uint32_t PERCENT = 100;
// static constexpr uint32_t FRONT_ZERO = 10;
// static constexpr size_t PLACEHOLDER_LENGTH = 2;

static const std::string CLOSE_ICON_PATH = "/etc/request/xmark.svg";

rust::string GetSystemResourceString(const rust::str name)
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
    auto ret = resourceMgr->GetStringByName(name.data(), outValue);
    if (ret != Resource::RState::SUCCESS) {
        REQUEST_HILOGE("GetStringById failed: %{public}d", ret);
    }
    return rust::string(outValue);
}

rust::string GetSystemLanguage()
{
    return rust::string(I18n::LocaleConfig::GetSystemLanguage().c_str());
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

void BasicRequestSettings(Notification::NotificationRequest &request, int32_t uid)
{
    request.SetCreatorUid(REQUEST_SERVICE_ID);
    request.SetOwnerUid(uid);
    request.SetIsAgentNotification(true);
}

std::shared_ptr<OHOS::Notification::NotificationContent> NormalContent(const NotifyContent &content)
{
    auto normalContent = std::make_shared<Notification::NotificationNormalContent>();
    normalContent->SetTitle(std::string(content.title));
    normalContent->SetText(std::string(content.text));
    return std::make_shared<Notification::NotificationContent>(normalContent);
}

std::shared_ptr<OHOS::Notification::NotificationContent> LiveViewContent(const NotifyContent &content)
{
    auto liveViewContent = std::make_shared<Notification::NotificationLocalLiveViewContent>();

    liveViewContent->SetContentType(static_cast<int32_t>(Notification::NotificationContent::Type::LOCAL_LIVE_VIEW));
    liveViewContent->SetType(REQUEST_STYLE_SIMPLE);

    liveViewContent->SetText(std::string(content.text));
    liveViewContent->SetTitle(std::string(content.title));

    if (content.x_mark || content.progress_circle.open) {
        liveViewContent->addFlag(Notification::NotificationLocalLiveViewContent::LiveViewContentInner::BUTTON);
    }

    if (content.x_mark) {
        auto button = liveViewContent->GetButton();
        auto icon = CreatePixelMap();
        if (icon != nullptr) {
            button.addSingleButtonName("cancel");
            button.addSingleButtonIcon(icon);
            liveViewContent->SetButton(button);
        }
    }

    if (content.progress_circle.open) {
        liveViewContent->addFlag(Notification::NotificationLocalLiveViewContent::LiveViewContentInner::PROGRESS);
        Notification::NotificationProgress progress;
        progress.SetIsPercentage(true);
        progress.SetCurrentValue(content.progress_circle.current);
        progress.SetMaxValue(content.progress_circle.total);
        liveViewContent->SetProgress(progress);
    }

    return std::make_shared<Notification::NotificationContent>(liveViewContent);
}

int PublishNotification(const NotifyContent &content)
{
    Notification::NotificationRequest request(content.request_id);
    BasicRequestSettings(request, content.uid);
    request.SetInProgress(content.progress_circle.open);
    if (content.live_view) {
        request.SetSlotType(Notification::NotificationConstant::SlotType::LIVE_VIEW);
        request.SetContent(LiveViewContent(content));
    } else {
        request.SetContent(NormalContent(content));
    }
    if (!content.want_agent.empty()) {
        request.SetWantAgent(
            OHOS::AbilityRuntime::WantAgent::WantAgentHelper::FromString(std::string(content.want_agent)));
    }
    return Notification::NotificationHelper::PublishNotification(request);
}

NotificationSubscriber::NotificationSubscriber(rust::Box<TaskManagerWrapper> taskManager)
    : _taskManager(std::move(taskManager)){};

void NotificationSubscriber::OnConnected(){};
void NotificationSubscriber::OnDisconnected(){};
void NotificationSubscriber::OnDied(){};
void NotificationSubscriber::OnResponse(
    int32_t notificationId, sptr<Notification::NotificationButtonOption> buttonOption)
{
    if (buttonOption == nullptr) {
        REQUEST_HILOGE("buttonOption empty");
        return;
    }
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
    static auto subscriber = std::make_unique<NotificationSubscriber>(std::move(taskManager));
    Notification::NotificationHelper::SubscribeLocalLiveViewNotification(*subscriber);
}

} // namespace OHOS::Request