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
#include "app_mgr_proxy.h"
#include "common_event_data.h"
#include "common_event_manager.h"
#include "common_event_publish_info.h"
#include "cxx.h"
#include "int_wrapper.h"
#include "iservice_registry.h"
#include "log.h"
#include "string_wrapper.h"
#include "sys_event.h"
#include "system_ability_definition.h"
#include "tokenid_kit.h"
#include "utils/mod.rs.h"

namespace OHOS::Request {
using namespace OHOS::Security::AccessToken;
using namespace OHOS::EventFwk;
using namespace OHOS::AppExecFwk;

int GetForegroundAbilities(rust::vec<int> &uid)
{
    sptr<IRemoteObject> token;
    auto abilities = std::vector<AppExecFwk::AppStateData>();
    auto sysm = SystemAbilityManagerClient::GetInstance().GetSystemAbilityManager();
    if (sysm == nullptr) {
        REQUEST_HILOGE("GetForegroundAbilities failed, sysm is nullptr");
        SysEventLog::SendSysEventLog(FAULT_EVENT, SAMGR_FAULT_00, "Get SAM failed");
        return -1;
    }
    auto remote = sysm->CheckSystemAbility(APP_MGR_SERVICE_ID);
    if (remote == nullptr) {
        REQUEST_HILOGE("GetForegroundAbilities failed, remote is nullptr");
        SysEventLog::SendSysEventLog(FAULT_EVENT, SAMGR_FAULT_02, "Check SA failed");
        return -1;
    }
    auto proxy = AppMgrProxy(remote);
    auto ret = proxy.GetForegroundApplications(abilities);
    if (ret != 0) {
        REQUEST_HILOGE("GetForegroundAbilities, ret: %{public}d", ret);
        return ret;
    }
    for (auto ability : abilities) {
        uid.push_back(ability.uid);
    }
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
        return false;
    }
    return true;
}

bool PublishStateChangeEvent(rust::str bundleName, uint32_t taskId, int32_t state, int32_t uid)
{
    REQUEST_HILOGD("PublishStateChangeEvents in.");
    static constexpr const char *eventAction = "ohos.request.event.COMPLETE";

    Want want;
    want.SetAction(eventAction);
    want.SetBundle(std::string(bundleName));
    std::vector<int32_t> subscriberUids;
    subscriberUids.push_back(uid);

    std::string data = std::to_string(taskId);
    CommonEventData commonData(want, state, data);
    CommonEventPublishInfo publishInfo;
    publishInfo.SetBundleName(std::string(bundleName));
    publishInfo.SetSubscriberUid(subscriberUids);

    bool res = CommonEventManager::PublishCommonEvent(commonData, publishInfo);
    if (!res) {
        REQUEST_HILOGE("PublishStateChangeEvents failed!");
    }
    return res;
}

int32_t UpdatePolicy(bool result)
{
    sptr<ISystemAbilityManager> systemAbilityManager =
        SystemAbilityManagerClient::GetInstance().GetSystemAbilityManager();
    if (systemAbilityManager == nullptr) {
        REQUEST_HILOGE("Getting SystemAbilityManager failed.");
        return -1;
    }
    int32_t systemAbilityId = 3706;

    OnDemandPolicyType policyType = OnDemandPolicyType::START_POLICY;

    std::vector<SystemAbilityOnDemandEvent> abilityOnDemandEvents;

    SystemAbilityOnDemandEvent event2;
    event2.eventId = OnDemandEventId::COMMON_EVENT;
    event2.name = "usual.event.USER_REMOVED";
    abilityOnDemandEvents.push_back(event2);

    SystemAbilityOnDemandEvent event3;
    event3.eventId = OnDemandEventId::COMMON_EVENT;
    event3.name = "usual.event.USER_SWITCHED";
    abilityOnDemandEvents.push_back(event3);

    SystemAbilityOnDemandEvent event4;
    event4.eventId = OnDemandEventId::COMMON_EVENT;
    event4.name = "usual.event.USER_STARTED";
    abilityOnDemandEvents.push_back(event4);

    if (!result) {
        SystemAbilityOnDemandEvent event1;
        event1.eventId = OnDemandEventId::COMMON_EVENT;
        event1.name = "usual.event.CONNECTIVITY_CHANGE";
        event1.value = "3";
        abilityOnDemandEvents.push_back(event1);
    }

    int32_t ret = systemAbilityManager->UpdateOnDemandPolicy(systemAbilityId, policyType, abilityOnDemandEvents);

    return ret;
}

bool IsCalledByHAP(uint32_t tokenId)
{
    return AccessTokenKit::GetTokenTypeFlag(tokenId) == ATokenTypeEnum::TOKEN_HAP;
}

} // namespace OHOS::Request