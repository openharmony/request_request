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

#include "requestserviceproxymanager4_fuzzer.h"
#include "../requestserviceproxy_fuzzer/requestserviceproxy_fuzzer_common.h"

using namespace OHOS::Request;

namespace OHOS {

void RequestManagerImplFuzzTestSetMaxSpeed(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    int64_t maxSpeed = provider.ConsumeIntegral<int64_t>();
    RequestManagerImpl::GetInstance()->SetMaxSpeed(tid, maxSpeed);
    RequestManagerImpl::GetInstance()->OnChannelBroken();
}

void RequestManagerImplFuzzTestOnResponseReceive(FuzzedDataProvider &provider)
{
    Response resp;
    resp.taskId = provider.ConsumeRandomLengthString(MAX_LENGTH);
    resp.version = provider.ConsumeRandomLengthString(MAX_LENGTH);
    resp.statusCode = provider.ConsumeIntegral<int32_t>();
    resp.reason = provider.ConsumeRandomLengthString(MAX_LENGTH);
    int headerCount = provider.ConsumeIntegralInRange<int>(0, MAX_NUM);
    for (int i = 0; i < headerCount; i++) {
        std::string key = provider.ConsumeRandomLengthString(MAX_LENGTH);
        std::vector<std::string> values;
        int valueCount = provider.ConsumeIntegralInRange<int>(1, MAX_NUM);
        for (int j = 0; j < valueCount; j++) {
            values.push_back(provider.ConsumeRandomLengthString(MAX_LENGTH));
        }
        resp.headers[key] = values;
    }
    auto respptr = std::make_shared<Response>(resp);
    RequestManagerImpl::GetInstance()->OnResponseReceive(respptr);
}

void RequestManagerImplFuzzTestOnNotifyDataReceive(FuzzedDataProvider &provider)
{
    NotifyData notifyData;
    notifyData.type = subscribeTypes[provider.ConsumeIntegralInRange<size_t>(0, subscribeTypes.size() - 1)];
    notifyData.taskId = provider.ConsumeIntegral<uint32_t>();
    notifyData.progress.state = states[provider.ConsumeIntegralInRange<size_t>(0, states.size() - 1)];
    notifyData.progress.index = provider.ConsumeIntegral<uint32_t>();
    notifyData.progress.processed = provider.ConsumeIntegral<uint64_t>();
    notifyData.progress.totalProcessed = provider.ConsumeIntegral<uint64_t>();
    notifyData.action = actions[provider.ConsumeIntegralInRange<size_t>(0, actions.size() - 1)];
    notifyData.version = versions[provider.ConsumeIntegralInRange<size_t>(0, versions.size() - 1)];
    notifyData.mode = modes[provider.ConsumeIntegralInRange<size_t>(0, modes.size() - 1)];
    int taskStateCount = provider.ConsumeIntegralInRange<int>(0, MAX_NUM);
    for (int i = 0; i < taskStateCount; i++) {
        TaskState taskState;
        taskState.path = provider.ConsumeRandomLengthString(MAX_LENGTH);
        taskState.responseCode = provider.ConsumeIntegral<uint32_t>();
        taskState.message = provider.ConsumeRandomLengthString(MAX_LENGTH);
        notifyData.taskStates.push_back(taskState);
    }
    auto notifyDataptr = std::make_shared<NotifyData>(notifyData);
    RequestManagerImpl::GetInstance()->OnNotifyDataReceive(notifyDataptr);
}

void RequestManagerImplFuzzTestOnFaultsReceive(FuzzedDataProvider &provider)
{
    int32_t num32 = provider.ConsumeIntegral<int32_t>();
    auto num32ptr = std::make_shared<int32_t>(num32);
    SubscribeType type;
    auto typeptr = std::make_shared<SubscribeType>(type);
    Reason reason;
    auto reasonptr = std::make_shared<Reason>(reason);
    RequestManagerImpl::GetInstance()->OnFaultsReceive(num32ptr, typeptr, reasonptr);
}

void RequestManagerImplFuzzTestOnWaitReceive(FuzzedDataProvider &provider)
{
    int32_t num32 = provider.ConsumeIntegral<int32_t>();
    WaitingReason waitingReas = waitingReasons[provider.ConsumeIntegralInRange<size_t>(0, waitingReasons.size() - 1)];
    RequestManagerImpl::GetInstance()->OnWaitReceive(num32, waitingReas);
}

} // namespace OHOS

extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size)
{
    FuzzedDataProvider provider(data, size);
    OHOS::RequestManagerImplFuzzTestSetMaxSpeed(provider);
    OHOS::RequestManagerImplFuzzTestOnResponseReceive(provider);
    OHOS::RequestManagerImplFuzzTestOnNotifyDataReceive(provider);
    OHOS::RequestManagerImplFuzzTestOnFaultsReceive(provider);
    OHOS::RequestManagerImplFuzzTestOnWaitReceive(provider);
    return 0;
}
