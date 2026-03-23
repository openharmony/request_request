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

#include "requestserviceproxysubscribe_fuzzer.h"
#include "../requestserviceproxy_fuzzer/requestserviceproxy_fuzzer_common.h"

using namespace OHOS::Request;

namespace OHOS {

bool SubscribeFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->Subscribe(tid);

    return true;
}

bool UnSubscribeFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->Unsubscribe(tid);

    return true;
}

bool SubRunCountFuzzTest(FuzzedDataProvider &provider)
{
    sptr<NotifyInterface> listener = RunCountNotifyStub::GetInstance();
    bool trigger = provider.ConsumeBool();

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(trigger);
    if (proxy == nullptr) {
        return true;
    }
    proxy->SubRunCount(listener);

    return true;
}

bool UnsubRunCountFuzzTest(FuzzedDataProvider &provider)
{
    sptr<NotifyInterface> listener = RunCountNotifyStub::GetInstance();
    bool trigger = provider.ConsumeBool();

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(trigger);
    if (proxy == nullptr) {
        return true;
    }
    proxy->UnsubRunCount();

    return true;
}

bool AttachGroupFuzzTest(FuzzedDataProvider &provider)
{
    std::string gid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    std::vector<std::string> tids = convertToVectorString(provider);
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->AttachGroup(gid, tids);

    return true;
}

} // namespace OHOS

extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size)
{
    FuzzedDataProvider provider(data, size);
    OHOS::SubscribeFuzzTest(provider);
    OHOS::UnSubscribeFuzzTest(provider);
    OHOS::SubRunCountFuzzTest(provider);
    OHOS::UnsubRunCountFuzzTest(provider);
    OHOS::AttachGroupFuzzTest(provider);
    return 0;
}
