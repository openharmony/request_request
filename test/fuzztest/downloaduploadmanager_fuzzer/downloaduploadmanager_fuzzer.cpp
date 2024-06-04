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
#define private public
#define protected public

#include "downloaduploadmanager_fuzzer.h"

#include <cstddef>
#include <cstdint>

#include "accesstoken_kit.h"
#include "js_common.h"
#include "message_parcel.h"
#include "nativetoken_kit.h"
#include "request.h"
#include "request_manager.h"
#include "request_manager_impl.h"
#include "request_running_task_count.h"
#include "request_service_interface.h"
#include "running_task_count.h"
#include "token_setproc.h"

using namespace OHOS::Request;
using namespace OHOS::Security::AccessToken;

#undef private
#undef protected

namespace OHOS {

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
    auto tid = std::to_string(size);

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

void RequestFuzzTestGetId(const uint8_t *data, size_t size)
{
    std::string tid(reinterpret_cast<const char *>(data), size);
    GrantNativePermission();
    auto request = OHOS::Request::Request(tid);
}

class RTResponseListenerImpl : public IResponseListener {
public:
    ~RTResponseListenerImpl(){};
    void OnResponseReceive(const std::shared_ptr<Response> &response) override
    {
        (void)response;
        return;
    }
};

void RequestFuzzTestHasListener(const uint8_t *data, size_t size)
{
    std::string tid(reinterpret_cast<const char *>(data), size);
    SubscribeType type = SubscribeType::RESPONSE;
    auto request = OHOS::Request::Request(tid);
    std::shared_ptr<RTResponseListenerImpl> listenerPtr = std::make_shared<RTResponseListenerImpl>();
    GrantNativePermission();
    request.AddListener(type, listenerPtr);
    request.RemoveListener(type, listenerPtr);
}

class RTNotifyDataListenerImpl : public INotifyDataListener {
public:
    ~RTNotifyDataListenerImpl(){};
    void OnNotifyDataReceive(const std::shared_ptr<NotifyData> &notifyData) override
    {
        (void)notifyData;
        return;
    }
};

void RequestFuzzTestOnNotifyDataReceive(const uint8_t *data, size_t size)
{
    std::string tid(reinterpret_cast<const char *>(data), size);
    SubscribeType type = SubscribeType::COMPLETED;
    auto request = OHOS::Request::Request(tid);
    std::shared_ptr<NotifyData> notifyData = std::make_shared<NotifyData>();
    notifyData->type = type;
    notifyData->version = Version::API9;
    GrantNativePermission();
    request.OnNotifyDataReceive(notifyData);
    std::shared_ptr<RTNotifyDataListenerImpl> listenerPtr = std::make_shared<RTNotifyDataListenerImpl>();
    request.AddListener(type, listenerPtr);
    request.OnNotifyDataReceive(notifyData);
}

void RequestFuzzTestAddAndRemoveListener(const uint8_t *data, size_t size)
{
    std::string tid(reinterpret_cast<const char *>(data), size);
    SubscribeType type = SubscribeType::COMPLETED;
    GrantNativePermission();
    auto request = OHOS::Request::Request(tid);
    std::shared_ptr<NotifyData> notifyData = std::make_shared<NotifyData>();
    notifyData->type = type;
    notifyData->version = Version::API9;

    request.OnNotifyDataReceive(notifyData);
    std::shared_ptr<RTNotifyDataListenerImpl> listenerPtr = std::make_shared<RTNotifyDataListenerImpl>();
    request.AddListener(type, listenerPtr);
    request.RemoveListener(type, listenerPtr);
}

void RequestFuzzTestOnResponseReceive(const uint8_t *data, size_t size)
{
    std::string tid(reinterpret_cast<const char *>(data), size);
    SubscribeType type = SubscribeType::RESPONSE;
    std::shared_ptr<Response> response = std::make_shared<Response>();
    GrantNativePermission();
    auto request = OHOS::Request::Request(tid);
    request.OnResponseReceive(response);
    std::shared_ptr<RTResponseListenerImpl> listenerPtr = std::make_shared<RTResponseListenerImpl>();
    request.AddListener(type, listenerPtr);
    request.OnResponseReceive(response);
}

class FwkTestOberver : public IRunningTaskObserver {
public:
    void OnRunningTaskCountUpdate(int count) override;
    ~FwkTestOberver() = default;
    FwkTestOberver() = default;
};

void FwkTestOberver::OnRunningTaskCountUpdate(int count)
{
}

void RunningTaskCountFuzzTestSubscribeRunningTaskCount(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy();
    if (proxy == nullptr) {
        std::shared_ptr<IRunningTaskObserver> ob = std::make_shared<FwkTestOberver>();
        SubscribeRunningTaskCount(ob);
        UnsubscribeRunningTaskCount(ob);
    }
    std::shared_ptr<IRunningTaskObserver> ob1 = std::make_shared<FwkTestOberver>();
    int32_t ret = SubscribeRunningTaskCount(ob1);
    std::shared_ptr<IRunningTaskObserver> ob2 = std::make_shared<FwkTestOberver>();
    FwkRunningTaskCountManager::GetInstance()->AttachObserver(ob2);
    ret = SubscribeRunningTaskCount(ob2);
    FwkRunningTaskCountManager::GetInstance()->DetachObserver(ob1);
    FwkRunningTaskCountManager::GetInstance()->DetachObserver(ob2);
}

void RunningTaskCountFuzzTestUnubscribeRunning(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    std::shared_ptr<IRunningTaskObserver> ob1 = std::make_shared<FwkTestOberver>();
    FwkRunningTaskCountManager::GetInstance()->AttachObserver(ob1);

    std::shared_ptr<IRunningTaskObserver> ob2 = std::make_shared<FwkTestOberver>();
    UnsubscribeRunningTaskCount(ob2);
    UnsubscribeRunningTaskCount(ob1);
}

void RunningTaskCountFuzzTestGetAndSetCount(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    int old = FwkRunningTaskCountManager::GetInstance()->GetCount();
    int except = 10; // 10 is except count num
    FwkRunningTaskCountManager::GetInstance()->SetCount(except);
    int count = FwkRunningTaskCountManager::GetInstance()->GetCount();
    FwkRunningTaskCountManager::GetInstance()->SetCount(old);
    count = FwkRunningTaskCountManager::GetInstance()->GetCount();
}

void RunningTaskCountFuzzTestUpdateRunningTaskCount(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    std::shared_ptr<IRunningTaskObserver> ob = std::make_shared<FwkTestOberver>();
    FwkIRunningTaskObserver runningOb = FwkIRunningTaskObserver(ob);
    runningOb.UpdateRunningTaskCount();
}

void RunningTaskCountFuzzTestNotifyAllObservers(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    std::shared_ptr<IRunningTaskObserver> ob1 = std::make_shared<FwkTestOberver>();
    FwkRunningTaskCountManager::GetInstance()->AttachObserver(ob1);
    FwkRunningTaskCountManager::GetInstance()->NotifyAllObservers();
    FwkRunningTaskCountManager::GetInstance()->DetachObserver(ob1);
}

} // namespace OHOS

/* Fuzzer entry point */
extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size)
{
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
    OHOS::RequestFuzzTestGetId(data, size);
    OHOS::RequestFuzzTestHasListener(data, size);
    OHOS::RequestFuzzTestOnNotifyDataReceive(data, size);
    OHOS::RequestFuzzTestAddAndRemoveListener(data, size);
    OHOS::RequestFuzzTestOnResponseReceive(data, size);
    OHOS::RunningTaskCountFuzzTestSubscribeRunningTaskCount(data, size);
    OHOS::RunningTaskCountFuzzTestUnubscribeRunning(data, size);
    OHOS::RunningTaskCountFuzzTestGetAndSetCount(data, size);
    OHOS::RunningTaskCountFuzzTestUpdateRunningTaskCount(data, size);
    OHOS::RunningTaskCountFuzzTestNotifyAllObservers(data, size);
    return 0;
}
