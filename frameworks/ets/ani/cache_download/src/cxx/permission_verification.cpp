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

#include "permission_verification.h"

#include "access_token.h"
#include "accesstoken_kit.h"
#include "ipc_skeleton.h"

namespace OHOS::Request {
using namespace OHOS::Security::AccessToken;

bool CheckInternetPermission()
{
    uint64_t tokenId = IPCSkeleton::GetCallingFullTokenID();
    TypeATokenTypeEnum tokenType = AccessTokenKit::GetTokenTypeFlag(static_cast<AccessTokenID>(tokenId));
    if (tokenType == TOKEN_INVALID) {
        return false;
    }
    int result = AccessTokenKit::VerifyAccessToken(tokenId, "ohos.permission.INTERNET");
    return result == PERMISSION_GRANTED;
}

bool CheckGetNetworkInfoPermission()
{
    uint64_t tokenId = IPCSkeleton::GetCallingFullTokenID();
    TypeATokenTypeEnum tokenType = AccessTokenKit::GetTokenTypeFlag(static_cast<AccessTokenID>(tokenId));
    if (tokenType == TOKEN_INVALID) {
        return false;
    }
    int result = AccessTokenKit::VerifyAccessToken(tokenId, "ohos.permission.GET_NETWORK_INFO");
    return result == PERMISSION_GRANTED;
}

} // namespace OHOS::Request