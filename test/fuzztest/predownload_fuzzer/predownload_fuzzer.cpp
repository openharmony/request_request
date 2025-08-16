/*
 * Copyright (c) 2025 Huawei Device Co., Ltd.
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

#include "predownload_fuzzer.h"

#include <securec.h>

#include <cstddef>
#include <cstdint>
#include <vector>

#include "accesstoken_kit.h"
#include "nativetoken_kit.h"
#include "preload_callback.h"
#include "request_preload.h"
#include "token_setproc.h"

using namespace OHOS::Request;
using namespace OHOS::Security::AccessToken;

namespace OHOS {
constexpr int64_t PRELOAD_UTF8_SIZE_LIMIT = 8192;

uint16_t ConvertToUint16(const uint8_t *ptr, size_t size)
{
    if (ptr == nullptr || size < sizeof(uint16_t)) {
        return 0;
    }
    uint16_t value;
    if (memcpy_s(&value, sizeof(value), ptr, sizeof(uint16_t)) != 0) {
        return 0;
    }
    return value;
}

void ConvertToUTF8(std::string &url)
{
    for (size_t i = 0; i < url.size(); i++) {
        if (url[i] > 0x7F) {
            url[i] = '?';
        }
    }
}

void GrantNativePermission()
{
    const char **perms = new const char *[1];
    perms[0] = "ohos.permission.GET_NETWORK_INFO";
    TokenInfoParams infoInstance = {
        .dcapsNum = 0,
        .permsNum = 1,
        .aclsNum = 0,
        .dcaps = nullptr,
        .perms = perms,
        .acls = nullptr,
        .processName = "preload_info",
        .aplStr = "system_core",
    };
    uint64_t tokenId = GetAccessTokenId(&infoInstance);
    SetSelfTokenID(tokenId);
    AccessTokenKit::ReloadNativeTokenInfo();
    delete[] perms;
}

void GetDownloadInfoFuzzTest(const uint8_t *data, size_t size)
{
    if (size <= 0 || data == nullptr) {
        return;
    }
    if (size > PRELOAD_UTF8_SIZE_LIMIT) {
        return;
    }

    std::string url(reinterpret_cast<const char *>(data), size);
    ConvertToUTF8(url);
    GrantNativePermission();
    Preload::GetInstance()->GetDownloadInfo(url);
}

void SetDownloadInfoListSizeFuzzTest(const uint8_t *data, size_t size)
{
    uint16_t len = ConvertToUint16(data, size);
    GrantNativePermission();
    Preload::GetInstance()->SetDownloadInfoListSize(len);
}

} // namespace OHOS

/* Fuzzer entry point */
extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size)
{
    /* Run your code on data */
    OHOS::GetDownloadInfoFuzzTest(data, size);
    OHOS::SetDownloadInfoListSizeFuzzTest(data, size);
    return 0;
}