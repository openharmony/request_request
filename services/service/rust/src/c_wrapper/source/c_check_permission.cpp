/*
* Copyright (C) 2023 Huawei Device Co., Ltd.
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
#include "c_check_permission.h"

#include "access_token.h"
#include "accesstoken_kit.h"
#include "log.h"

using namespace OHOS::Security::AccessToken;

static constexpr const char *DOWNLOAD_PERMISSION_NAME_INTERNET = "ohos.permission.INTERNET";
bool CheckPermission(uint64_t tokenId)
{
    REQUEST_HILOGD("C++ CheckPermission");
    TypeATokenTypeEnum tokenType = AccessTokenKit::GetTokenTypeFlag(static_cast<AccessTokenID>(tokenId));
    if (tokenType == TOKEN_INVALID) {
        REQUEST_HILOGE("invalid token id");
        return false;
    }
    REQUEST_HILOGD("GetTokenTypeFlag");
    int result = AccessTokenKit::VerifyAccessToken(tokenId, DOWNLOAD_PERMISSION_NAME_INTERNET);
    if (result != PERMISSION_GRANTED) {
        REQUEST_HILOGE("Current tokenId permission is %{public}d", result);
    }
    REQUEST_HILOGD("VerifyAccessToken");
    return result == PERMISSION_GRANTED;
}