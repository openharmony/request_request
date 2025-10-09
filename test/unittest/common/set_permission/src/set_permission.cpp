/*
 * Copyright (C) 2025 Huawei Device Co., Ltd.
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

#include "set_permission.h"

#include "accesstoken_kit.h"
#include "log.h"
#include "nativetoken_kit.h"
#include "token_setproc.h"

namespace OHOS::Request::SetPermission {

void SetAccessTokenPermission(const std::vector<std::string> &permissions, const std::string &processName)
{
    if (permissions.empty()) {
        REQUEST_HILOGI("Permissions list is empty.");
        return;
    }
    if (processName.empty()) {
        REQUEST_HILOGI("Process name is empty.");
        return;
    }
    auto perms = std::make_unique<const char *[]>(permissions.size());
    for (size_t i = 0; i < permissions.size(); ++i) {
        perms[i] = permissions[i].c_str();
    }
    NativeTokenInfoParams infoInstance = {
        .dcapsNum = 0,
        .permsNum = static_cast<uint32_t>(permissions.size()),
        .aclsNum = 0,
        .dcaps = nullptr,
        .perms = perms.get(),
        .acls = nullptr,
        .processName = processName.c_str(),
        .aplStr = "system_core",
    };

    auto tokenId = GetAccessTokenId(&infoInstance);
    if (tokenId == 0) {
        REQUEST_HILOGI("GetAccessTokenId failed.");
        return;
    }
    int ret = SetSelfTokenID(tokenId);
    if (ret != 0) {
        REQUEST_HILOGI("SetSelfTokenID failed, code is %{public}d.", ret);
        return;
    }
    ret = OHOS::Security::AccessToken::AccessTokenKit::ReloadNativeTokenInfo();
    if (ret < 0) {
        REQUEST_HILOGI("ReloadNativeTokenInfo failed, code is %{public}d.", ret);
        return;
    }
    REQUEST_HILOGI("Set access token permission successfully for process: %{public}s", processName.c_str());
}

void SetAccesslNoPermission(const std::string &processName)
{
    const char **perms = new const char *[0];
    NativeTokenInfoParams infoInstance = {
        .dcapsNum = 0,
        .permsNum = 0,
        .aclsNum = 0,
        .dcaps = nullptr,
        .perms = perms,
        .acls = nullptr,
        .processName = processName.c_str(),
        .aplStr = "system_core",
    };
    uint64_t tokenId = GetAccessTokenId(&infoInstance);
    SetSelfTokenID(tokenId);
    OHOS::Security::AccessToken::AccessTokenKit::ReloadNativeTokenInfo();
    delete[] perms;
}

} // namespace OHOS::Request::SetPermission