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
#include "tokenid_kit.h"

using namespace OHOS::Security::AccessToken;

static constexpr const char *DOWNLOAD_PERMISSION_NAME_INTERNET = "ohos.permission.INTERNET";
static constexpr const char *DOWNLOAD_PERMISSION_SESSION_MANAGER = "ohos.permission.DOWNLOAD_SESSION_MANAGER";
static constexpr const char *UPLOAD_PERMISSION_SESSION_MANAGER = "ohos.permission.UPLOAD_SESSION_MANAGER";

bool IsTokenType(const uint64_t tokenId)
{
    REQUEST_HILOGD("C++ CheckPermission");
    TypeATokenTypeEnum tokenType = AccessTokenKit::GetTokenTypeFlag(static_cast<AccessTokenID>(tokenId));
    if (tokenType == TOKEN_INVALID) {
        REQUEST_HILOGE("invalid token id");
        return false;
    }
    return true;
}

bool IsAccessToken(const uint64_t tokenId, const std::string permission)
{
    int result = AccessTokenKit::VerifyAccessToken(tokenId, DOWNLOAD_PERMISSION_NAME_INTERNET);
    if (result != PERMISSION_GRANTED) {
        REQUEST_HILOGE("Current tokenId permission is %{public}d", result);
    }
    REQUEST_HILOGD("VerifyAccessToken");
    return result == PERMISSION_GRANTED;
}

bool CheckPermission(uint64_t tokenId)
{
    REQUEST_HILOGD("C++ CheckPermission");
    return IsTokenType(tokenId) && IsAccessToken(tokenId, DOWNLOAD_PERMISSION_NAME_INTERNET);
}

QueryPermission CheckSessionManagerPermission(uint64_t tokenId)
{
    REQUEST_HILOGD("C++ CheckSessionManagerPermission");
    if (!IsTokenType(tokenId)) {
        return QueryPermission::NoPermisson;
    }
    if (IsAccessToken(tokenId, DOWNLOAD_PERMISSION_SESSION_MANAGER) &&
        IsAccessToken(tokenId, UPLOAD_PERMISSION_SESSION_MANAGER)) {
        return QueryPermission::QueryAll;
    }
    if (IsAccessToken(tokenId, DOWNLOAD_PERMISSION_SESSION_MANAGER)) {
        return QueryPermission::QueryDownLoad;
    }
    if (IsAccessToken(tokenId, UPLOAD_PERMISSION_SESSION_MANAGER)) {
        return QueryPermission::QueryUpload;
    }
    return QueryPermission::NoPermisson;
}

bool IsSystemAPI(uint64_t tokenId)
{
    return TokenIdKit::IsSystemAppByFullTokenID(tokenId);
}