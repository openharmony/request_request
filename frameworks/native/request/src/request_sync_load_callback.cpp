/*
 * Copyright (c) 2023 Huawei Device Co., Ltd.
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

#include "request_sync_load_callback.h"

#include "iservice_registry.h"
#include "isystem_ability_load_callback.h"
#include "log.h"
#include "request_manager.h"
#include "request_manager_impl.h"
#include "system_ability_definition.h"

namespace OHOS::Request {
void RequestSyncLoadCallback::OnLoadSystemAbilitySuccess(
    int32_t systemAbilityId, const sptr<IRemoteObject> &remoteObject)
{
    if (systemAbilityId != DOWNLOAD_SERVICE_ID) {
        REQUEST_HILOGE("start systemAbilityId is not download server");
        return;
    }
    RequestManagerImpl::GetInstance()->LoadServerSuccess();
}

void RequestSyncLoadCallback::OnLoadSystemAbilityFail(int32_t systemAbilityId)
{
    if (systemAbilityId != DOWNLOAD_SERVICE_ID) {
        REQUEST_HILOGE("start systemAbilityId is not download server");
        return;
    }
    RequestManagerImpl::GetInstance()->LoadServerFail();
}
} // namespace OHOS::Request