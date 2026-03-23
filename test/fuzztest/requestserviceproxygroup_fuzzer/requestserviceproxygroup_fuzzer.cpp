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

#include "requestserviceproxygroup_fuzzer.h"
#include "../requestserviceproxy_fuzzer/requestserviceproxy_fuzzer_common.h"

using namespace OHOS::Request;

namespace OHOS {

bool DeleteGroupFuzzTest(FuzzedDataProvider &provider)
{
    std::string gid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->DeleteGroup(gid);

    return true;
}

bool QueryTasksFuzzTest(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<TaskInfoRet> rets = convertToVectorTaskInfoRet(provider);
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->QueryTasks(tids, rets);

    return true;
}

bool ShowTasksFuzzTest(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<TaskInfoRet> rets = convertToVectorTaskInfoRet(provider);
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->ShowTasks(tids, rets);

    return true;
}

bool TouchTasksFuzzTest(FuzzedDataProvider &provider)
{
    std::vector<TaskIdAndToken> tids = convertToVectorTaskIdAndToken(provider);
    std::vector<TaskInfoRet> rets = convertToVectorTaskInfoRet(provider);
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->TouchTasks(tids, rets);

    return true;
}

bool QueryFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    size_t index = provider.ConsumeIntegralInRange<size_t>(0, exceptionErrorCodes.size() - 1);
    ExceptionErrorCode code = exceptionErrorCodes[index];
    TaskInfoRet infoRet{ .code = code };

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->Query(tid, infoRet.info);

    return true;
}

} // namespace OHOS

extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size)
{
    FuzzedDataProvider provider(data, size);
    OHOS::DeleteGroupFuzzTest(provider);
    OHOS::QueryTasksFuzzTest(provider);
    OHOS::ShowTasksFuzzTest(provider);
    OHOS::TouchTasksFuzzTest(provider);
    OHOS::QueryFuzzTest(provider);
    return 0;
}
