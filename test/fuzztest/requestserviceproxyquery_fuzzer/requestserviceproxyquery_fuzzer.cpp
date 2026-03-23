/*
 * Copyright (c) 2026 Huawei Device Co., Ltd.
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

#include "requestserviceproxyquery_fuzzer.h"
#include "../requestserviceproxy_fuzzer/requestserviceproxy_fuzzer_common.h"

using namespace OHOS::Request;

namespace OHOS {

bool QueryMimeTypeFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    std::string mimeType = provider.ConsumeRandomLengthString(MAX_LENGTH);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->QueryMimeType(tid, mimeType);

    return true;
}

bool RemoveFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    size_t versionIndex = provider.ConsumeIntegralInRange<size_t>(0, versions.size() - 1);
    Version version = versions[versionIndex];

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->Remove(tid, version);

    return true;
}

bool ResumeFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->Resume(tid);

    return true;
}

bool SetMaxSpeedFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    int64_t maxSpeed = provider.ConsumeIntegral<int64_t>();

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->SetMaxSpeed(tid, maxSpeed);

    return true;
}

bool OpenChannelFuzzTest(FuzzedDataProvider &provider)
{
    int32_t sockFd = provider.ConsumeIntegral<int32_t>();

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->OpenChannel(sockFd);

    return true;
}

} // namespace OHOS

extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size)
{
    FuzzedDataProvider provider(data, size);
    OHOS::QueryMimeTypeFuzzTest(provider);
    OHOS::RemoveFuzzTest(provider);
    OHOS::ResumeFuzzTest(provider);
    OHOS::SetMaxSpeedFuzzTest(provider);
    OHOS::OpenChannelFuzzTest(provider);
    return 0;
}
