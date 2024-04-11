/*
 * Copyright (c) 2023 Huawei Device Co., Ltd.
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

#include "downloaduploadmanager_fuzzer.h"

#include <cstddef>
#include <cstdint>

#include "accesstoken_kit.h"
#include "js_common.h"
#include "message_parcel.h"
#include "nativetoken_kit.h"
#include "request_manager.h"
#include "request_service_interface.h"
#include "token_setproc.h"

using namespace OHOS::Request;

namespace OHOS {
constexpr size_t THRESHOLD = 10;

using namespace OHOS::Security::AccessToken;
uint32_t ConvertToUint32(const uint8_t *ptr, size_t size)
{
    if (ptr == nullptr || (size < sizeof(uint32_t))) {
        return 0;
    }
    return *(reinterpret_cast<const uint32_t *>(ptr));
}

void GrantNativePermission()
{
    const char **perms = new const char *[1];
    perms[0] = "ohos.permission.INTERNET";
    TokenInfoParams infoInstance = {
        .dcapsNum = 0,
        .permsNum = 1,
        .aclsNum = 0,
        .dcaps = nullptr,
        .perms = perms,
        .acls = nullptr,
        .processName = "request_service",
        .aplStr = "system_core",
    };
    uint64_t tokenId = GetAccessTokenId(&infoInstance);
    SetSelfTokenID(tokenId);
    AccessTokenKit::ReloadNativeTokenInfo();
    delete[] perms;
}

void CreateRequestFuzzTest(const uint8_t *data, size_t size)
{
    Config config;
    auto tid = static_cast<int32_t>(size);

    GrantNativePermission();
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    RequestManager::GetInstance()->Create(config, seq, tid);
}

void StartRequestFuzzTest(const uint8_t *data, size_t size)
{
    std::string tid(reinterpret_cast<const char *>(data), size);
    GrantNativePermission();
    RequestManager::GetInstance()->Start(tid);
}

void StopRequestFuzzTest(const uint8_t *data, size_t size)
{
    std::string tid(reinterpret_cast<const char *>(data), size);
    GrantNativePermission();
    RequestManager::GetInstance()->Stop(tid);
}

void ShowRequestFuzzTest(const uint8_t *data, size_t size)
{
    TaskInfo info;
    std::string tid(reinterpret_cast<const char *>(data), size);
    GrantNativePermission();
    RequestManager::GetInstance()->Show(tid, info);
}

void TouchRequestFuzzTest(const uint8_t *data, size_t size)
{
    TaskInfo info;
    std::string tid(reinterpret_cast<const char *>(data), size);
    std::string token(data, data + size);
    GrantNativePermission();
    RequestManager::GetInstance()->Touch(tid, token, info);
}

void SearchRequestFuzzTest(const uint8_t *data, size_t size)
{
    Filter filter;
    std::vector<std::string> tids;
    std::string str(reinterpret_cast<const char *>(data), size);
    tids.push_back(str);
    GrantNativePermission();
    RequestManager::GetInstance()->Search(filter, tids);
}

void PauseRequestFuzzTest(const uint8_t *data, size_t size)
{
    Version version = static_cast<Version>(ConvertToUint32(data, size));
    std::string tid(reinterpret_cast<const char *>(data), size);
    GrantNativePermission();
    RequestManager::GetInstance()->Pause(tid, version);
}

void QueryMimeTypeRequestFuzzTest(const uint8_t *data, size_t size)
{
    std::string tid(reinterpret_cast<const char *>(data), size);
    std::string mimeType(data, data + size);
    GrantNativePermission();
    RequestManager::GetInstance()->QueryMimeType(tid, mimeType);
}

void RemoveRequestFuzzTest(const uint8_t *data, size_t size)
{
    Version version = static_cast<Version>(ConvertToUint32(data, size));
    std::string tid(reinterpret_cast<const char *>(data), size);
    GrantNativePermission();
    RequestManager::GetInstance()->Remove(tid, version);
}

void ResumeRequestFuzzTest(const uint8_t *data, size_t size)
{
    std::string tid(reinterpret_cast<const char *>(data), size);
    GrantNativePermission();
    RequestManager::GetInstance()->Resume(tid);
}

void GetTaskRequestFuzzTest(const uint8_t *data, size_t size)
{
    Config config;
    std::string tid(reinterpret_cast<const char *>(data), size);
    std::string token(reinterpret_cast<const char *>(data), size);
    GrantNativePermission();
    RequestManager::GetInstance()->GetTask(tid, token, config);
}

void SubscribeRequestFuzzTest(const uint8_t *data, size_t size)
{
    std::string tid(reinterpret_cast<const char *>(data), size);
    GrantNativePermission();
    RequestManager::GetInstance()->Subscribe(tid);
}

void UnsubscribeRequestFuzzTest(const uint8_t *data, size_t size)
{
    std::string tid(reinterpret_cast<const char *>(data), size);
    GrantNativePermission();
    RequestManager::GetInstance()->Unsubscribe(tid);
}

void IsSaReadyRequestFuzzTest(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    RequestManager::GetInstance()->IsSaReady();
}

void ReopenChannelRequestFuzzTest(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    RequestManager::GetInstance()->ReopenChannel();
}

void TestFunc(void)
{
    return;
}

void RestoreListenerRequestFuzzTest(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    RequestManager::GetInstance()->RestoreListener(TestFunc);
}

void QueryRequestFuzzTest(const uint8_t *data, size_t size)
{
    std::string tid(reinterpret_cast<const char *>(data), size);
    TaskInfo taskinfo;
    GrantNativePermission();
    RequestManager::GetInstance()->Query(tid, taskinfo);
}

} // namespace OHOS

/* Fuzzer entry point */
extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size)
{
    if (size < OHOS::THRESHOLD) {
        return 0;
    }

    /* Run your code on data */
    OHOS::CreateRequestFuzzTest(data, size);
    OHOS::StartRequestFuzzTest(data, size);
    OHOS::StopRequestFuzzTest(data, size);
    OHOS::ShowRequestFuzzTest(data, size);
    OHOS::TouchRequestFuzzTest(data, size);
    OHOS::SearchRequestFuzzTest(data, size);
    OHOS::PauseRequestFuzzTest(data, size);
    OHOS::QueryMimeTypeRequestFuzzTest(data, size);
    OHOS::RemoveRequestFuzzTest(data, size);
    OHOS::ResumeRequestFuzzTest(data, size);
    OHOS::GetTaskRequestFuzzTest(data, size);
    OHOS::SubscribeRequestFuzzTest(data, size);
    OHOS::UnsubscribeRequestFuzzTest(data, size);
    OHOS::RestoreListenerRequestFuzzTest(data, size);
    OHOS::IsSaReadyRequestFuzzTest(data, size);
    OHOS::ReopenChannelRequestFuzzTest(data, size);
    OHOS::QueryRequestFuzzTest(data, size);
    return 0;
}
