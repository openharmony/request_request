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

// @tc.name: ut_get_download_info_fuzzer
// @tc.desc: Fuzz test for Preload GetDownloadInfo method
// @tc.precon: NA
// @tc.step: 1. Check input data validity
// 2. Convert input data to URL string
// 3. Convert URL to valid UTF-8 format
// 4. Grant native permission
// 5. Call GetInstance()->GetDownloadInfo with URL
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
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

// @tc.name: ut_set_download_info_list_size_fuzzer
// @tc.desc: Fuzz test for Preload SetDownloadInfoListSize method
// @tc.precon: NA
// @tc.step: 1. Convert input data to uint16_t length
// 2. Grant native permission
// 3. Call GetInstance()->SetDownloadInfoListSize with length
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void SetDownloadInfoListSizeFuzzTest(const uint8_t *data, size_t size)
{
    uint16_t len = ConvertToUint16(data, size);
    GrantNativePermission();
    Preload::GetInstance()->SetDownloadInfoListSize(len);
}

} // namespace OHOS

// @tc.name: ut_llvm_fuzzer_test_one_input
// @tc.desc: Fuzzer entry point function
// @tc.precon: NA
// @tc.step: 1. Call GetDownloadInfoFuzzTest with input data
// 2. Call SetDownloadInfoListSizeFuzzTest with input data
// @tc.expect: Entry point should execute all fuzz tests without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
/* Fuzzer entry point */
extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size)
{
    /* Run your code on data */
    OHOS::GetDownloadInfoFuzzTest(data, size);
    OHOS::SetDownloadInfoListSizeFuzzTest(data, size);
    return 0;
}