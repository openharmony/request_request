/*
* Copyright (C) 2024 Huawei Device Co., Ltd.
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

#include "request_utils.h"

#include <want.h>

#include "ability_manager_client.h"
#include "access_token.h"
#include "accesstoken_kit.h"
#include "app_mgr_client.h"
#include "common_event_data.h"
#include "common_event_manager.h"
#include "common_event_publish_info.h"
#include "cxx.h"
#include "int_wrapper.h"
#include "log.h"
#include "string_wrapper.h"
#include "tokenid_kit.h"
#include "utils/mod.rs.h"

namespace OHOS::Request {
using namespace OHOS::Security::AccessToken;
using namespace OHOS::EventFwk;

int GetTopUid(int &uid)
{
    sptr<IRemoteObject> token;
    auto ret = OHOS::AAFwk::AbilityManagerClient::GetInstance()->GetTopAbility(token);
    if (ret != 0) {
        REQUEST_HILOGE("GetTopUid failed, ret: %{public}d", ret);
        return ret;
    }
    auto info = OHOS::AppExecFwk::RunningProcessInfo();
    AppExecFwk::AppMgrClient().GetRunningProcessInfoByToken(token, info);

    uid = info.uid_;
    return 0;
}

rust::string GetCallingBundle(rust::u64 tokenId)
{
    auto tokenType = AccessTokenKit::GetTokenTypeFlag(static_cast<uint32_t>(tokenId));
    if (tokenType != TOKEN_HAP) {
        REQUEST_HILOGE("invalid token");
        return rust::string("");
    }
    HapTokenInfo info;
    int ret = AccessTokenKit::GetHapTokenInfo(tokenId, info);
    if (ret != 0) {
        REQUEST_HILOGE("failed to get hap info, ret: %{public}d", ret);
        return rust::string("");
    }
    return rust::string(info.bundleName);
}

bool IsSystemAPI(uint64_t tokenId)
{
    return TokenIdKit::IsSystemAppByFullTokenID(tokenId);
}

bool CheckPermission(uint64_t tokenId, rust::str permission)
{
    auto perm = std::string(permission);
    TypeATokenTypeEnum tokenType = AccessTokenKit::GetTokenTypeFlag(static_cast<AccessTokenID>(tokenId));
    if (tokenType == TOKEN_INVALID) {
        REQUEST_HILOGE("invalid token id");
        return false;
    }
    int result = AccessTokenKit::VerifyAccessToken(tokenId, perm);
    if (result != PERMISSION_GRANTED) {
        REQUEST_HILOGE("check permission %{public}s failed ret %{public}d", perm.c_str(), result);
        return false;
    }
    return true;
}

bool PublishStateChangeEvent(rust::str bundleName, uint32_t taskId, int32_t state)
{
    REQUEST_HILOGD("PublishStateChangeEvents in.");
    static constexpr const char *eventAction = "ohos.request.event.COMPLETE";

    Want want;
    want.SetAction(eventAction);
    want.SetBundle(std::string(bundleName));

    std::string data = std::to_string(taskId);
    CommonEventData commonData(want, state, data);
    CommonEventPublishInfo publishInfo;
    publishInfo.SetBundleName(std::string(bundleName));

    bool res = CommonEventManager::PublishCommonEvent(commonData, publishInfo);
    if (!res) {
        REQUEST_HILOGE("PublishStateChangeEvents failed!");
    }
    return res;
}

} // namespace OHOS::Request