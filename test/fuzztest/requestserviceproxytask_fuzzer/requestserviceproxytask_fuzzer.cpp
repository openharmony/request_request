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

#include "requestserviceproxytask_fuzzer.h"
#include "../requestserviceproxy_fuzzer/requestserviceproxy_fuzzer_common.h"

using namespace OHOS::Request;

namespace OHOS {

bool CreateTasksFuzzTest(FuzzedDataProvider &provider)
{
    std::vector<Config> configs = convertToVectorConfig(provider);
    std::vector<TaskRet> rets = convertToVectorTaskRet(provider);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->CreateTasks(configs, rets);

    return true;
}

bool StartTasksFuzzTest(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<ExceptionErrorCode> rets = convertToVectorExceptionErrorCode(provider);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->StartTasks(tids, rets);

    return true;
}

bool StopTasksFuzzTest(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<ExceptionErrorCode> rets = convertToVectorExceptionErrorCode(provider);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->StopTasks(tids, rets);

    return true;
}

bool ResumeTasksFuzzTest(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<ExceptionErrorCode> rets = convertToVectorExceptionErrorCode(provider);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->ResumeTasks(tids, rets);

    return true;
}

bool PauseTasksFuzzTest(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<ExceptionErrorCode> rets = convertToVectorExceptionErrorCode(provider);
    size_t versionIndex = provider.ConsumeIntegralInRange<size_t>(0, versions.size() - 1);
    Version version = versions[versionIndex];

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->PauseTasks(tids, version, rets);

    return true;
}

} // namespace OHOS

extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size)
{
    FuzzedDataProvider provider(data, size);
    OHOS::CreateTasksFuzzTest(provider);
    OHOS::StartTasksFuzzTest(provider);
    OHOS::StopTasksFuzzTest(provider);
    OHOS::ResumeTasksFuzzTest(provider);
    OHOS::PauseTasksFuzzTest(provider);
    return 0;
}
