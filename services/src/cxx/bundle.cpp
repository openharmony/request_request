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

#include "bundle.h"

#include "sys_event.h"

using namespace OHOS::AppExecFwk;

namespace OHOS::Request {

std::mutex appInfoMutex_;

AppInfo GetNameAndIndex(int32_t uid)
{
    std::lock_guard<std::mutex> lockGuard(appInfoMutex_);
    AppInfo appInfo;
    appInfo.ret = false;
    int32_t appIndex = 0;
    std::string bundleName;
    sptr<ISystemAbilityManager> systemAbilityManager =
        SystemAbilityManagerClient::GetInstance().GetSystemAbilityManager();
    if (!systemAbilityManager) {
        REQUEST_HILOGE("GetNameAndIndex, fail to get system ability mgr.");
        SysEventLog::SendSysEventLog(FAULT_EVENT, SAMGR_FAULT_00, "Get SAM failed");
        return appInfo;
    }
    sptr<IRemoteObject> remoteObject = systemAbilityManager->GetSystemAbility(BUNDLE_MGR_SERVICE_SYS_ABILITY_ID);
    if (!remoteObject) {
        REQUEST_HILOGE("GetNameAndIndex, fail to get bundle manager proxy.");
        return appInfo;
    }
    sptr<IBundleMgr> bundleMgr = iface_cast<IBundleMgr>(remoteObject);
    ErrCode ret = bundleMgr->GetNameAndIndexForUid(uid, bundleName, appIndex);
    if (ret != ERR_OK) {
        REQUEST_HILOGE("GetNameAndIndex, err ret: %{public}d", ret);
        return appInfo;
    }

    appInfo.ret = true;
    appInfo.index = appIndex;
    appInfo.name = rust::String(bundleName);
    return appInfo;
}
} // namespace OHOS::Request
