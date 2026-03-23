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

#include "requestserviceproxytouch_fuzzer.h"
#include "../requestserviceproxy_fuzzer/requestserviceproxy_fuzzer_common.h"

using namespace OHOS::Request;

namespace OHOS {

bool TouchFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    std::string token = provider.ConsumeRandomLengthString(MAX_LENGTH);
    size_t index = provider.ConsumeIntegralInRange<size_t>(0, exceptionErrorCodes.size() - 1);
    ExceptionErrorCode code = exceptionErrorCodes[index];
    TaskInfoRet infoRet{ .code = code };

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->Touch(tid, token, infoRet.info);

    return true;
}

bool SetModeFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    size_t index = provider.ConsumeIntegralInRange<size_t>(0, modes.size() - 1);
    Mode mode = modes[index];

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->SetMode(tid, mode);

    return true;
}

bool ShowFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    size_t index = provider.ConsumeIntegralInRange<size_t>(0, exceptionErrorCodes.size() - 1);
    ExceptionErrorCode code = exceptionErrorCodes[index];
    TaskInfoRet infoRet{ .code = code };

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->Show(tid, infoRet.info);

    return true;
}

bool CreateGroupFuzzTest(FuzzedDataProvider &provider)
{
    std::string gid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    bool gauge = provider.ConsumeBool();
    Notification notification;

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->CreateGroup(gid, gauge, notification);

    return true;
}

bool CreateFuzzTest(FuzzedDataProvider &provider)
{
    Config config = convertToConfig(provider);
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->Create(config, tid);

    return true;
}

} // namespace OHOS

extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size)
{
    FuzzedDataProvider provider(data, size);
    OHOS::TouchFuzzTest(provider);
    OHOS::SetModeFuzzTest(provider);
    OHOS::ShowFuzzTest(provider);
    OHOS::CreateGroupFuzzTest(provider);
    OHOS::CreateFuzzTest(provider);
    return 0;
}
