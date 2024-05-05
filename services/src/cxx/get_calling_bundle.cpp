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

#include "get_calling_bundle.h"

#include "access_token.h"
#include "accesstoken_kit.h"
#include "log.h"
#include "tokenid_kit.h"

using namespace OHOS::Security::AccessToken;

CStringWrapper GetCallingBundle(uint64_t tokenId)
{
    auto tokenType = AccessTokenKit::GetTokenTypeFlag(static_cast<uint32_t>(tokenId));
    if (tokenType != TOKEN_HAP) {
        REQUEST_HILOGE("invalid token");
        return WrapperCString("");
    }
    HapTokenInfo info;
    int ret = AccessTokenKit::GetHapTokenInfo(tokenId, info);
    if (ret != 0) {
        REQUEST_HILOGE("failed to get hap info, ret: %{public}d", ret);
        return WrapperCString("");
    }
    return WrapperCString(info.bundleName);
}