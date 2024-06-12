/*
 * Copyright (c) 2022 Huawei Device Co., Ltd.
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

#include "background_notification.h"

#include "int_wrapper.h"
#include "log.h"
#include "notification.h"
#include "notification_constant.h"
#include "notification_content.h"
#include "notification_helper.h"
#include "string_wrapper.h"
#include "want_params.h"

using namespace OHOS::Notification;
static constexpr uint8_t DOWNLOAD_ACTION = 0;
void RequestBackgroundNotify(
    RequestTaskMsg msg, CStringWrapper wrappedPath, CStringWrapper wrappedFileName, uint32_t percent)
{
    REQUEST_HILOGD("Background Notification, percent is %{public}d", percent);
    auto requestTemplate = std::make_shared<NotificationTemplate>();
    std::string filepath(wrappedPath.cStr, wrappedPath.len);
    std::string fileName(wrappedFileName.cStr, wrappedFileName.len);
    requestTemplate->SetTemplateName("downloadTemplate");
    OHOS::AAFwk::WantParams wantParams;
    wantParams.SetParam("progressValue", OHOS::AAFwk::Integer::Box(percent));
    wantParams.SetParam("fileName", OHOS::AAFwk::String::Box(fileName));
    std::shared_ptr<NotificationNormalContent> normalContent = std::make_shared<NotificationNormalContent>();
    if (msg.action == DOWNLOAD_ACTION) {
        wantParams.SetParam("title", OHOS::AAFwk::String::Box("下载"));
        normalContent->SetTitle("下载");
    } else {
        wantParams.SetParam("title", OHOS::AAFwk::String::Box("上传"));
        normalContent->SetTitle("上传");
    }
    requestTemplate->SetTemplateData(std::make_shared<OHOS::AAFwk::WantParams>(wantParams));
    std::string contentText(wrappedFileName.cStr, wrappedFileName.len);
    normalContent->SetText(contentText);
    auto content = std::make_shared<NotificationContent>(normalContent);
    NotificationRequest req(msg.taskId);
    req.SetCreatorUid(msg.uid);
    req.SetContent(content);
    req.SetTemplate(requestTemplate);
    req.SetSlotType(NotificationConstant::OTHER);
    OHOS::ErrCode errCode = NotificationHelper::PublishNotification(req);
    if (errCode != OHOS::ERR_OK) {
        REQUEST_HILOGE("notification errCode: %{public}d", errCode);
    }
}
