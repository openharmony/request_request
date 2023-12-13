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

bool DownloadServerCheckPermission(uint64_t tokenId, CStringWrapper permission)
{
    TypeATokenTypeEnum tokenType = AccessTokenKit::GetTokenTypeFlag(static_cast<AccessTokenID>(tokenId));
    if (tokenType == TOKEN_INVALID) {
        REQUEST_HILOGE("invalid token id");
        return false;
    }
    int result = AccessTokenKit::VerifyAccessToken(tokenId, std::string(permission.cStr, permission.len));
    if (result != PERMISSION_GRANTED) {
        REQUEST_HILOGE("check permission failed");
        return false;
    }
    REQUEST_HILOGD("check permission success");
    return true;
}

bool RequestIsSystemAPI(uint64_t tokenId)
{
    return TokenIdKit::IsSystemAppByFullTokenID(tokenId);
}