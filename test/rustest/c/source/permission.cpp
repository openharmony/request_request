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

#include "permission.h"

#include <iostream>
#include <vector>

#include "accesstoken_kit.h"
#include "nativetoken_kit.h"
#include "token_setproc.h"

void SetAccessTokenPermission()
{
    auto permissions = std::vector<std::string>();
    permissions.push_back("ohos.permission.INTERNET");
    permissions.push_back("ohos.permission.GET_NETWORK_INFO");
    permissions.push_back("ohos.permission.READ_MEDIA");
    permissions.push_back("ohos.permission.WRITE_MEDIA");
    permissions.push_back("ohos.permission.RUNNING_STATE_OBSERVER");
    permissions.push_back("ohos.permission.GET_NETWORK_INFO");
    permissions.push_back("ohos.permission.CONNECTIVITY_INTERNAL");
    permissions.push_back("ohos.permission.SEND_TASK_COMPLETE_EVENT");
    permissions.push_back("ohos.permission.ACCESS_CERT_MANAGER");
    permissions.push_back("ohos.permission.INTERACT_ACROSS_LOCAL_ACCOUNTS");
    permissions.push_back("ohos.permission.MANAGE_LOCAL_ACCOUNTS");

    auto processName = std::string("rust_request_test");
    auto perms = std::make_unique<const char *[]>(permissions.size());
    for (size_t i = 0; i < permissions.size(); i++) {
        perms[i] = permissions[i].c_str();
    }

    NativeTokenInfoParams infoInstance = {
        .dcapsNum = 0,
        .permsNum = permissions.size(),
        .aclsNum = 0,
        .dcaps = nullptr,
        .perms = perms.get(),
        .acls = nullptr,
        .processName = processName.c_str(),
        .aplStr = "system_core",
    };
    auto tokenId = GetAccessTokenId(&infoInstance);
    if (tokenId == 0) {
        std::cout << "GetAccessTokenId failed" << std::endl;
        return;
    }
    int ret = SetSelfTokenID(tokenId);
    if (ret != 0) {
        return;
    }
    ret = OHOS::Security::AccessToken::AccessTokenKit::ReloadNativeTokenInfo();
    if (ret < 0) {
        return;
    }
}
