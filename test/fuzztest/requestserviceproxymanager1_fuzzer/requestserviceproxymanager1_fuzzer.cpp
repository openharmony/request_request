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

#include "requestserviceproxymanager1_fuzzer.h"
#include "../requestserviceproxy_fuzzer/requestserviceproxy_fuzzer_common.h"

using namespace OHOS::Request;

namespace OHOS {

void RequestManagerImplFuzzTestSetMode(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    Mode mode = modes[provider.ConsumeIntegralInRange<size_t>(0, modes.size() - 1)];
    RequestManagerImpl::GetInstance()->SetMode(tid, mode);
}

void RequestManagerImplFuzzTestDisableTaskNotification(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<ExceptionErrorCode> err = convertToVectorExceptionErrorCode(provider);
    RequestManagerImpl::GetInstance()->DisableTaskNotification(tids, err);
}

void RequestManagerImplFuzzTestCreateTasks(FuzzedDataProvider &provider)
{
    std::vector<Config> configs = convertToVectorConfig(provider);
    std::vector<TaskRet> rets = convertToVectorTaskRet(provider);
    RequestManagerImpl::GetInstance()->CreateTasks(configs, rets);
}

void RequestManagerImplFuzzTestStartTasks(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<ExceptionErrorCode> err = convertToVectorExceptionErrorCode(provider);
    RequestManagerImpl::GetInstance()->StartTasks(tids, err);
}

void RequestManagerImplFuzzTestStopTasks(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<ExceptionErrorCode> err = convertToVectorExceptionErrorCode(provider);
    RequestManagerImpl::GetInstance()->StopTasks(tids, err);
}

} // namespace OHOS

extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size)
{
    FuzzedDataProvider provider(data, size);
    OHOS::RequestManagerImplFuzzTestSetMode(provider);
    OHOS::RequestManagerImplFuzzTestDisableTaskNotification(provider);
    OHOS::RequestManagerImplFuzzTestCreateTasks(provider);
    OHOS::RequestManagerImplFuzzTestStartTasks(provider);
    OHOS::RequestManagerImplFuzzTestStopTasks(provider);
    return 0;
}
