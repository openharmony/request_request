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

#ifndef OHOS_REQUEST_ACTION_WRAPPER_H
#define OHOS_REQUEST_ACTION_WRAPPER_H

#include <cstdint>
#include <iostream>
#include <memory>
#include <vector>

#include "request_action.h"
#include "accesstoken_kit.h"
#include "cxx.h"
#include "nativetoken_kit.h"
#include "token_setproc.h"
namespace OHOS::Request {

const static std::vector<std::string> Permissions = { "ohos.permission.INTERNET",
    "ohos.permission.UPLOAD_SESSION_MANAGER", "ohos.permission.DOWNLOAD_SESSION_MANAGER" };

inline void DisableTaskNotification(rust::str taskId)
{
    std::vector<std::string> tids = { std::string(taskId) };
    auto w = std::unordered_map<std::string, ExceptionErrorCode>();
    RequestAction::GetInstance()->DisableTaskNotification(tids, w);
    for (auto &elem : w) {
        std::cout << "task" << elem.first << static_cast<int32_t>(elem.second) << std::endl;
    }
}

inline NativeTokenInfoParams Permission(std::unique_ptr<const char *[]> &perms)
{
    return NativeTokenInfoParams{
        .dcapsNum = 0,
        .permsNum = Permissions.size(),
        .aclsNum = 0,
        .perms = perms.get(),
        .processName = "disable_task_notification",
        .aplStr = "system_core",
    };
}

inline void SetMode(rust::str taskId, int32_t mode)
{
    std::string tid = std::string(taskId);
    RequestAction::GetInstance()->SetMode(tid, static_cast<Mode>(mode));
}

inline void SetAccessTokenPermission()
{
    auto perms = std::make_unique<const char *[]>(Permissions.size());
    for (size_t i = 0; i < Permissions.size(); i++) {
        perms[i] = Permissions[i].c_str();
    }
    auto infoInstance = Permission(perms);
    auto tokenId = GetAccessTokenId(&infoInstance);
    SetSelfTokenID(tokenId);
    OHOS::Security::AccessToken::AccessTokenKit::ReloadNativeTokenInfo();
}

} // namespace OHOS::Request
#endif