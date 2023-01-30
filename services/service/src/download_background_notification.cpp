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

#include "notification.h"
#include "notification_helper.h"
#include "notification_constant.h"
#include "notification_content.h"
#include "int_wrapper.h"
#include "string_wrapper.h"
#include "want_params.h"
#include "log.h"
#include "download_background_notification.h"

using namespace OHOS::Notification;
namespace OHOS::Request::Download {
void DownloadBackgroundNotification::PublishDownloadNotification(uint32_t taskId, pid_t uid,
                                                                 const std::string &filePath, uint32_t percent)
{
    auto downloadTemplate = std::make_shared<NotificationTemplate>();
    if (downloadTemplate == nullptr) {
        DOWNLOAD_HILOGE("taskId: %{public}d, downloadTemplate is null", taskId);
        return;
    }
    downloadTemplate->SetTemplateName("downloadTemplate");
    AAFwk::WantParams wantParams;
    wantParams.SetParam("progressValue",  AAFwk::Integer::Box(percent));
    wantParams.SetParam("fileName",  AAFwk::String::Box(filePath));
    wantParams.SetParam("title",  AAFwk::String::Box("Download"));
    downloadTemplate->SetTemplateData(std::make_shared<AAFwk::WantParams>(wantParams));
    auto normalContent = std::make_shared<NotificationNormalContent>();
    if (normalContent == nullptr) {
        DOWNLOAD_HILOGE("taskId: %{public}d, normalContent is null", taskId);
        return;
    }
    auto content = std::make_shared<NotificationContent>(normalContent);
    if (content == nullptr) {
        DOWNLOAD_HILOGE("taskId: %{public}d, content is null", taskId);
        return;
    }
    NotificationRequest req(taskId);
    req.SetCreatorUid(uid);
    req.SetContent(content);
    req.SetTemplate(downloadTemplate);
    req.SetSlotType(NotificationConstant::OTHER);
    ErrCode errCode = NotificationHelper::PublishNotification(req);
    if (errCode != ERR_OK) {
        DOWNLOAD_HILOGE("notification errCode: %{public}d", errCode);
    }
}
} // Download