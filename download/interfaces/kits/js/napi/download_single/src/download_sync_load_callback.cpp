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

#include "download_manager.h"
#include "iservice_registry.h"
#include "isystem_ability_load_callback.h"
#include "system_ability_definition.h"
#include "log.h"
#include "download_sync_load_callback.h"

namespace OHOS::Request::Download {
void DownloadSyncLoadCallback::OnLoadSystemAbilitySuccess(int32_t systemAbilityId,
                                                          const sptr<IRemoteObject>& remoteObject)
{
    if (systemAbilityId != DOWNLOAD_SERVICE_ID) {
        DOWNLOAD_HILOGE("start systemAbilityId is not download server");
        return;
    }
    DownloadManager::GetInstance()->LoadServerSuccess();
}

void DownloadSyncLoadCallback::OnLoadSystemAbilityFail(int32_t systemAbilityId)
{
    if (systemAbilityId != DOWNLOAD_SERVICE_ID) {
        DOWNLOAD_HILOGE("start systemAbilityId is not download server");
        return;
    }
    DownloadManager::GetInstance()->LoadServerFail();
}
}