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

#include "requestserviceproxymanager2_fuzzer.h"
#include "../requestserviceproxy_fuzzer/requestserviceproxy_fuzzer_common.h"

using namespace OHOS::Request;

namespace OHOS {

void RequestManagerImplFuzzTestResumeTasks(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<ExceptionErrorCode> err = convertToVectorExceptionErrorCode(provider);
    RequestManagerImpl::GetInstance()->ResumeTasks(tids, err);
}

void RequestManagerImplFuzzTestRemoveTasks(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    Version version = versions[provider.ConsumeIntegralInRange<size_t>(0, versions.size() - 1)];
    std::vector<ExceptionErrorCode> err = convertToVectorExceptionErrorCode(provider);
    RequestManagerImpl::GetInstance()->RemoveTasks(tids, version, err);
}

void RequestManagerImplFuzzTestPauseTasks(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    Version version = versions[provider.ConsumeIntegralInRange<size_t>(0, versions.size() - 1)];
    std::vector<ExceptionErrorCode> err = convertToVectorExceptionErrorCode(provider);
    RequestManagerImpl::GetInstance()->PauseTasks(tids, version, err);
}

void RequestManagerImplFuzzTestQueryTasks(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<TaskInfoRet> taskinfo = convertToVectorTaskInfoRet(provider);
    RequestManagerImpl::GetInstance()->QueryTasks(tids, taskinfo);
}

void RequestManagerImplFuzzTestShowTasks(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<TaskInfoRet> taskinfo = convertToVectorTaskInfoRet(provider);
    RequestManagerImpl::GetInstance()->ShowTasks(tids, taskinfo);
}

} // namespace OHOS

extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size)
{
    FuzzedDataProvider provider(data, size);
    OHOS::RequestManagerImplFuzzTestResumeTasks(provider);
    OHOS::RequestManagerImplFuzzTestRemoveTasks(provider);
    OHOS::RequestManagerImplFuzzTestPauseTasks(provider);
    OHOS::RequestManagerImplFuzzTestQueryTasks(provider);
    OHOS::RequestManagerImplFuzzTestShowTasks(provider);
    return 0;
}
