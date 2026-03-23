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

#include "requestserviceproxysearch_fuzzer.h"
#include "../requestserviceproxy_fuzzer/requestserviceproxy_fuzzer_common.h"

using namespace OHOS::Request;

namespace OHOS {

bool GetTaskFuzzTest(FuzzedDataProvider &provider)
{
    Config config = convertToConfig(provider);
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    std::string token = provider.ConsumeRandomLengthString(MAX_LENGTH);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->GetTask(tid, token, config);

    return true;
}

bool SearchFuzzTest(FuzzedDataProvider &provider)
{
    Filter filter = convertToFilter(provider);
    std::vector<std::string> tids = convertToVectorString(provider);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->Search(filter, tids);

    return true;
}

bool SysEventFuzzTestSendSysEventLog(FuzzedDataProvider &provider)
{
    std::string eventName = provider.ConsumeRandomLengthString(MAX_LENGTH);
    uint32_t num1 = provider.ConsumeIntegral<uint32_t>();
    int32_t num2 = provider.ConsumeIntegral<int32_t>();
    int32_t num3 = provider.ConsumeIntegral<int32_t>();
    SysEventLog::SendSysEventLog(eventName, num1, num2, num3);
    auto iter = ::OHOS::Request::SysEventLog::sysEventMap_.find("EXEC_ERROR");
    if (iter == ::OHOS::Request::SysEventLog::sysEventMap_.end()) {
        return true;
    }
    iter = ::OHOS::Request::SysEventLog::sysEventMap_.find("EXEC_FAULT");
    if (iter == ::OHOS::Request::SysEventLog::sysEventMap_.end()) {
        return true;
    }
    return true;
}

bool SysEventFuzzTestSendStatisticEvent(FuzzedDataProvider &provider)
{
    std::string string1 = provider.ConsumeRandomLengthString(MAX_LENGTH);
    std::string string2 = provider.ConsumeRandomLengthString(MAX_LENGTH);
    std::string string3 = provider.ConsumeRandomLengthString(MAX_LENGTH);
    std::vector<uint8_t> num = convertToVectorUint8_t(provider);
    struct SysEventInfo info = {num[0], string1, string2, string3};
    SysEventLog::SendStatisticEvent(info);
    return true;
}

} // namespace OHOS

extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size)
{
    FuzzedDataProvider provider(data, size);
    OHOS::GetTaskFuzzTest(provider);
    OHOS::SearchFuzzTest(provider);
    OHOS::SysEventFuzzTestSendSysEventLog(provider);
    OHOS::SysEventFuzzTestSendStatisticEvent(provider);
    return 0;
}
