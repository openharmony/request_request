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

#include <securec.h>

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
#include "request_sync_load_callback.h"
#include "runcount_notify_stub.h"
#include "running_task_count.h"
#include "system_ability_definition.h"
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

void SubscribeSARequestFuzzTest(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    RequestManager::GetInstance()->SubscribeSA();
    RequestManager::GetInstance()->UnsubscribeSA();
}

class FuzzResponseListenerImpl : public IResponseListener {
public:
    ~FuzzResponseListenerImpl(){};
    void OnResponseReceive(const std::shared_ptr<Response> &response) override
    {
        (void)response;
        return;
    }
};

class FuzzNotifyDataListenerImpl : public INotifyDataListener {
public:
    ~FuzzNotifyDataListenerImpl(){};
    void OnNotifyDataReceive(const std::shared_ptr<NotifyData> &notifyData) override
    {
        (void)notifyData;
        return;
    }
};

void AddAndRemoveListenerRequestFuzzTest(const uint8_t *data, size_t size)
{
    std::string taskId(reinterpret_cast<const char *>(data), size);
    GrantNativePermission();
    SubscribeType type = SubscribeType::RESPONSE;
    std::shared_ptr<FuzzResponseListenerImpl> listener = std::make_shared<FuzzResponseListenerImpl>();
    RequestManager::GetInstance()->AddListener(taskId, type, listener);
    RequestManager::GetInstance()->RemoveListener(taskId, type, listener);
    type = SubscribeType::COMPLETED;
    std::shared_ptr<FuzzNotifyDataListenerImpl> listener2 = std::make_shared<FuzzNotifyDataListenerImpl>();
    RequestManager::GetInstance()->AddListener(taskId, type, listener2);
    RequestManager::GetInstance()->RemoveListener(taskId, type, listener2);
}

void RemoveAllListenersRequestFuzzTest(const uint8_t *data, size_t size)
{
    std::string taskId(reinterpret_cast<const char *>(data), size);
    GrantNativePermission();
    SubscribeType type = SubscribeType::RESPONSE;
    std::shared_ptr<FuzzResponseListenerImpl> listener = std::make_shared<FuzzResponseListenerImpl>();
    RequestManager::GetInstance()->AddListener(taskId, type, listener);
    type = SubscribeType::COMPLETED;
    std::shared_ptr<FuzzNotifyDataListenerImpl> listener2 = std::make_shared<FuzzNotifyDataListenerImpl>();
    RequestManager::GetInstance()->AddListener(taskId, type, listener2);
    RequestManager::GetInstance()->RemoveAllListeners(taskId);
    RequestManager::GetInstance()->RestoreListener(nullptr);
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
    request.getId();
}

void RequestFuzzTestHasListener(const uint8_t *data, size_t size)
{
    std::string tid(reinterpret_cast<const char *>(data), size);
    SubscribeType type = SubscribeType::RESPONSE;
    auto request = OHOS::Request::Request(tid);
    std::shared_ptr<FuzzResponseListenerImpl> listenerPtr = std::make_shared<FuzzResponseListenerImpl>();
    GrantNativePermission();
    request.AddListener(type, listenerPtr);
    request.HasListener();
    request.RemoveListener(type, listenerPtr);
}

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
    std::shared_ptr<FuzzNotifyDataListenerImpl> listenerPtr = std::make_shared<FuzzNotifyDataListenerImpl>();
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
    std::shared_ptr<FuzzNotifyDataListenerImpl> listenerPtr = std::make_shared<FuzzNotifyDataListenerImpl>();
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
    std::shared_ptr<FuzzResponseListenerImpl> listenerPtr = std::make_shared<FuzzResponseListenerImpl>();
    request.AddListener(type, listenerPtr);
    request.OnResponseReceive(response);
}

class FuzzFwkTestOberver : public IRunningTaskObserver {
public:
    void OnRunningTaskCountUpdate(int count) override;
    ~FuzzFwkTestOberver() = default;
    FuzzFwkTestOberver() = default;
};

void FuzzFwkTestOberver::OnRunningTaskCountUpdate(int count)
{
}

void RunningTaskCountFuzzTestSubscribeRunningTaskCount(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy();
    if (proxy == nullptr) {
        std::shared_ptr<IRunningTaskObserver> ob = std::make_shared<FuzzFwkTestOberver>();
        SubscribeRunningTaskCount(ob);
        UnsubscribeRunningTaskCount(ob);
    }
    std::shared_ptr<IRunningTaskObserver> ob1 = std::make_shared<FuzzFwkTestOberver>();
    SubscribeRunningTaskCount(ob1);
    std::shared_ptr<IRunningTaskObserver> ob2 = std::make_shared<FuzzFwkTestOberver>();
    FwkRunningTaskCountManager::GetInstance()->AttachObserver(ob2);
    SubscribeRunningTaskCount(ob2);
    FwkRunningTaskCountManager::GetInstance()->DetachObserver(ob1);
    FwkRunningTaskCountManager::GetInstance()->DetachObserver(ob2);
}

void RunningTaskCountFuzzTestUnubscribeRunning(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    std::shared_ptr<IRunningTaskObserver> ob1 = std::make_shared<FuzzFwkTestOberver>();
    FwkRunningTaskCountManager::GetInstance()->AttachObserver(ob1);

    std::shared_ptr<IRunningTaskObserver> ob2 = std::make_shared<FuzzFwkTestOberver>();
    UnsubscribeRunningTaskCount(ob2);
    UnsubscribeRunningTaskCount(ob1);
}

void RunningTaskCountFuzzTestGetAndSetCount(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    int old = FwkRunningTaskCountManager::GetInstance()->GetCount();
    int except = 1; // 10 is except count num
    FwkRunningTaskCountManager::GetInstance()->SetCount(except);
    FwkRunningTaskCountManager::GetInstance()->GetCount();
    FwkRunningTaskCountManager::GetInstance()->SetCount(old);
    FwkRunningTaskCountManager::GetInstance()->GetCount();
}

void RunningTaskCountFuzzTestUpdateRunningTaskCount(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    std::shared_ptr<IRunningTaskObserver> ob = std::make_shared<FuzzFwkTestOberver>();
    FwkIRunningTaskObserver runningOb = FwkIRunningTaskObserver(ob);
    runningOb.UpdateRunningTaskCount();
}

void RunningTaskCountFuzzTestNotifyAllObservers(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    std::shared_ptr<IRunningTaskObserver> ob1 = std::make_shared<FuzzFwkTestOberver>();
    FwkRunningTaskCountManager::GetInstance()->AttachObserver(ob1);
    FwkRunningTaskCountManager::GetInstance()->NotifyAllObservers();
    FwkRunningTaskCountManager::GetInstance()->DetachObserver(ob1);
}

void RunCountNotifyStubFuzzTestGetInstance(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    RunCountNotifyStub::GetInstance();
}

void RunCountNotifyStubFuzzTestCallBack(const uint8_t *data, size_t size)
{
    Notify notify;
    GrantNativePermission();
    RunCountNotifyStub::GetInstance()->CallBack(notify);
}

void RunCountNotifyStubFuzzTestDone(const uint8_t *data, size_t size)
{
    TaskInfo taskInfo;
    GrantNativePermission();
    RunCountNotifyStub::GetInstance()->Done(taskInfo);
}

void RunCountNotifyStubFuzzTestOnCallBack(const uint8_t *data, size_t size)
{
    int64_t except = 10; // 10 is except value
    int old = FwkRunningTaskCountManager::GetInstance()->GetCount();
    OHOS::MessageParcel parcel;
    parcel.WriteInt64(except);
    GrantNativePermission();
    RunCountNotifyStub::GetInstance()->OnCallBack(parcel);
    FwkRunningTaskCountManager::GetInstance()->GetCount();
    FwkRunningTaskCountManager::GetInstance()->SetCount(old);
    FwkRunningTaskCountManager::GetInstance()->GetCount();
}

static constexpr int32_t ARRAY_LEN = 256; // 128 is array length
static constexpr int32_t INT64_SIZE = 8;  // 8 is int64 and uint64 num length
static constexpr int32_t INT32_SIZE = 4;  // 4 is int32 and uint32 num length
static constexpr int32_t INT16_SIZE = 2;  // 2 is int16 and uint16 num length

void ResponseMessageFuzzTestInt64FromParcel(const uint8_t *data, size_t size)
{
    int64_t except = 123456; // 123456 is except num
    char *parcel = reinterpret_cast<char *>(&except);
    int64_t num;
    int testSize = INT32_SIZE;
    ResponseMessageReceiver::Int64FromParcel(num, parcel, testSize);
    testSize = INT64_SIZE;
    ResponseMessageReceiver::Int64FromParcel(num, parcel, testSize);
}

void ResponseMessageFuzzTestUint64FromParcel(const uint8_t *data, size_t size)
{
    uint64_t except = 123456; // 123456 is except num
    char *parcel = reinterpret_cast<char *>(&except);
    uint64_t num;
    int testSize = INT32_SIZE;
    ResponseMessageReceiver::Uint64FromParcel(num, parcel, testSize);
    testSize = INT64_SIZE;
    ResponseMessageReceiver::Uint64FromParcel(num, parcel, testSize);
}

void ResponseMessageFuzzTestInt32FromParcel(const uint8_t *data, size_t size)
{
    int32_t except = 123456; // 123456 is except num
    char *parcel = reinterpret_cast<char *>(&except);
    int32_t num;
    int testSize = INT16_SIZE;
    ResponseMessageReceiver::Int32FromParcel(num, parcel, testSize);
    testSize = INT32_SIZE;
    ResponseMessageReceiver::Int32FromParcel(num, parcel, testSize);
}

void ResponseMessageFuzzTestUint32FromParcel(const uint8_t *data, size_t size)
{
    uint32_t except = 123456; // 123456 is except num
    char *parcel = reinterpret_cast<char *>(&except);
    uint32_t num;
    int testSize = INT16_SIZE;
    ResponseMessageReceiver::Uint32FromParcel(num, parcel, testSize);
    testSize = INT32_SIZE;
    ResponseMessageReceiver::Uint32FromParcel(num, parcel, testSize);
}

void ResponseMessageFuzzTestInt16FromParcel(const uint8_t *data, size_t size)
{
    int16_t except = 123; // 123 is except num
    char *parcel = reinterpret_cast<char *>(&except);
    int16_t num;
    int testSize = 0;
    ResponseMessageReceiver::Int16FromParcel(num, parcel, testSize);
    testSize = INT16_SIZE;
    ResponseMessageReceiver::Int16FromParcel(num, parcel, testSize);
}

void ResponseMessageFuzzTestStateFromParcel(const uint8_t *data, size_t size)
{
    State state;
    uint32_t except = static_cast<uint32_t>(State::ANY) + 1;
    char *parcel = reinterpret_cast<char *>(&except);
    int testSize = INT32_SIZE;
    ResponseMessageReceiver::StateFromParcel(state, parcel, testSize);
    except = static_cast<uint32_t>(State::ANY);
    parcel = reinterpret_cast<char *>(&except);
    testSize = INT32_SIZE;
    ResponseMessageReceiver::StateFromParcel(state, parcel, testSize);
}

void ResponseMessageFuzzTestActionFromParcel(const uint8_t *data, size_t size)
{
    Action action;
    uint32_t except = static_cast<uint32_t>(Action::ANY) + 1;
    char *parcel = reinterpret_cast<char *>(&except);
    int testSize = INT32_SIZE;
    ResponseMessageReceiver::ActionFromParcel(action, parcel, testSize);
    except = static_cast<uint32_t>(Action::ANY);
    parcel = reinterpret_cast<char *>(&except);
    testSize = INT32_SIZE;
    ResponseMessageReceiver::ActionFromParcel(action, parcel, testSize);
}

void ResponseMessageFuzzTestVersionFromParcel(const uint8_t *data, size_t size)
{
    Version version;
    uint32_t except = static_cast<uint32_t>(Version::API10) + 1;
    char *parcel = reinterpret_cast<char *>(&except);
    int testSize = INT32_SIZE;
    ResponseMessageReceiver::VersionFromParcel(version, parcel, testSize);
    except = static_cast<uint32_t>(Version::API10);
    parcel = reinterpret_cast<char *>(&except);
    testSize = INT32_SIZE;
    ResponseMessageReceiver::VersionFromParcel(version, parcel, testSize);
}

void ResponseMessageFuzzTestSubscribeTypeFromParcel(const uint8_t *data, size_t size)
{
    SubscribeType type;
    uint32_t except = static_cast<uint32_t>(SubscribeType::BUTT) + 1;
    char *parcel = reinterpret_cast<char *>(&except);
    int testSize = INT32_SIZE;
    ResponseMessageReceiver::SubscribeTypeFromParcel(type, parcel, testSize);
    except = static_cast<uint32_t>(SubscribeType::BUTT);
    parcel = reinterpret_cast<char *>(&except);
    testSize = INT32_SIZE;
    ResponseMessageReceiver::SubscribeTypeFromParcel(type, parcel, testSize);
}

void ResponseMessageFuzzTestStringFromParcel(const uint8_t *data, size_t size)
{
    std::string str;
    std::string except = "except string";
    char *parcel = const_cast<char *>(except.c_str());
    int testSize = except.size() - 1;
    ResponseMessageReceiver::StringFromParcel(str, parcel, testSize);
    testSize = except.size() + 1;
    ResponseMessageReceiver::StringFromParcel(str, parcel, testSize);
}

void ResponseMessageFuzzTestResponseHeaderFromParcel(const uint8_t *data, size_t size)
{
    std::map<std::string, std::vector<std::string>> headers;
    std::string except = "header:aaa,bbb,ccc\n";
    std::vector<std::string> header;
    char *parcel = const_cast<char *>(except.c_str());
    int testSize = except.size();
    ResponseMessageReceiver::ResponseHeaderFromParcel(headers, parcel, testSize);
}

void ResponseMessageFuzzTestProgressExtrasFromParcel(const uint8_t *data, size_t size)
{
    int arraySize = 64; // 64 is char array length
    char except[arraySize];
    uint32_t length = 1;
    int ret = memcpy_s(except, static_cast<size_t>(arraySize), reinterpret_cast<void *>(&length), sizeof(length));
    if (ret != 0) {
        return;
    }
    char keyValue[] = "key\0value\0";
    ret = memcpy_s(except + sizeof(length), static_cast<size_t>(arraySize - sizeof(length)), keyValue,
        9); // 9 is keyValue length
    if (ret != 0) {
        return;
    }
    std::map<std::string, std::string> extras;
    char *parcel = except;
    int testSize = INT16_SIZE;
    ResponseMessageReceiver::ProgressExtrasFromParcel(extras, parcel, testSize);
    parcel = except;
    testSize = sizeof(length) + 1;
    ResponseMessageReceiver::ProgressExtrasFromParcel(extras, parcel, testSize);
    parcel = except;
    testSize = sizeof(length) + 6; // 6 make except testSize between the keyValue
    ResponseMessageReceiver::ProgressExtrasFromParcel(extras, parcel, testSize);
    parcel = except;
    testSize = arraySize;
    ResponseMessageReceiver::ProgressExtrasFromParcel(extras, parcel, testSize);
}

void ResponseMessageFuzzTestVecInt64FromParcel(const uint8_t *data, size_t size)
{
    int arraySize = INT32_SIZE + INT64_SIZE;
    char except[arraySize];
    uint32_t length = 1;
    int ret = memcpy_s(except, static_cast<size_t>(arraySize), reinterpret_cast<void *>(&length), sizeof(length));
    if (ret != 0) {
        return;
    }
    int64_t value = 123456; // 123456 is except num
    ret = memcpy_s(except + sizeof(length), static_cast<size_t>(arraySize - sizeof(length)),
        reinterpret_cast<void *>(&value), sizeof(value));
    if (ret != 0) {
        return;
    }
    std::vector<int64_t> vec;
    char *parcel = except;
    int testSize = INT16_SIZE;
    ResponseMessageReceiver::VecInt64FromParcel(vec, parcel, testSize);
    parcel = except;
    testSize = INT64_SIZE;
    ResponseMessageReceiver::VecInt64FromParcel(vec, parcel, testSize);
    parcel = except;
    testSize = arraySize; // 6 make except testSize between the keyValue
    ResponseMessageReceiver::VecInt64FromParcel(vec, parcel, testSize);
}

void ResponseMessageFuzzTestResponseMessageReceiver(const uint8_t *data, size_t size)
{
    IResponseMessageHandler *handler = nullptr;
    int32_t sockFd = -1;
    ResponseMessageReceiver receiver = ResponseMessageReceiver(handler, sockFd);
}

void ResponseMessageFuzzTestMsgHeaderParcel(const uint8_t *data, size_t size)
{
    uint32_t magicNum = ResponseMessageReceiver::RESPONSE_MAGIC_NUM - 1;
    int pos = 0;
    int arraySize = INT32_SIZE + INT64_SIZE;
    char except[arraySize];
    int ret = memcpy_s(except, static_cast<size_t>(arraySize), reinterpret_cast<void *>(&magicNum), sizeof(magicNum));
    if (ret != 0) {
        return;
    }
    pos += sizeof(magicNum);
    int32_t msgId = 123456; // 123456 is except num
    ret = memcpy_s(except + pos, static_cast<size_t>(arraySize - pos), reinterpret_cast<void *>(&msgId), sizeof(msgId));
    if (ret != 0) {
        return;
    }
    pos += sizeof(msgId);
    int16_t msgType = 123; // 123 is except num
    ret = memcpy_s(
        except + pos, static_cast<size_t>(arraySize - pos), reinterpret_cast<void *>(&msgType), sizeof(msgType));
    if (ret != 0) {
        return;
    }
    pos += sizeof(msgType);
    int16_t bodySize = 456; // 456 is except num
    ret = memcpy_s(
        except + pos, static_cast<size_t>(arraySize - pos), reinterpret_cast<void *>(&bodySize), sizeof(bodySize));
    if (ret != 0) {
        return;
    }
    pos += sizeof(bodySize);
    msgId = 0;
    msgType = 0;
    bodySize = 0;
    char *parcel = except;
    int testSize = INT16_SIZE;
    ResponseMessageReceiver::MsgHeaderParcel(msgId, msgType, bodySize, parcel, testSize);
    parcel = except;
    testSize = INT32_SIZE;
    ResponseMessageReceiver::MsgHeaderParcel(msgId, msgType, bodySize, parcel, testSize);
    parcel = except;
    testSize = INT32_SIZE;
    magicNum = ResponseMessageReceiver::RESPONSE_MAGIC_NUM;
    ret = memcpy_s(except, static_cast<size_t>(arraySize), reinterpret_cast<void *>(&magicNum), sizeof(magicNum));
    if (ret != 0) {
        return;
    }
    ResponseMessageReceiver::MsgHeaderParcel(msgId, msgType, bodySize, parcel, testSize);
    parcel = except;
    testSize = INT32_SIZE + INT16_SIZE;
    ResponseMessageReceiver::MsgHeaderParcel(msgId, msgType, bodySize, parcel, testSize);
    parcel = except;
    testSize = INT64_SIZE;
    ResponseMessageReceiver::MsgHeaderParcel(msgId, msgType, bodySize, parcel, testSize); // 123456 is except num
    parcel = except;
    testSize = INT64_SIZE + INT16_SIZE;
    ResponseMessageReceiver::MsgHeaderParcel(msgId, msgType, bodySize, parcel, testSize);
    parcel = except;
    testSize = arraySize;
    ResponseMessageReceiver::MsgHeaderParcel(msgId, msgType, bodySize, parcel, testSize);
}

void ResponseMessageFuzzTestResponseFromParcel(const uint8_t *data, size_t size)
{
    std::shared_ptr<Response> response = std::make_shared<Response>();
    int pos = 0;
    int32_t tid = 123; // 123 is except tid
    std::string version = "version";
    int32_t statusCode = 456; // 456 is except statusCode
    std::string reason = "reason";
    std::string headers = "header:aaa,bbb,ccc\n";
    char except[ARRAY_LEN];
    int ret = memcpy_s(except, static_cast<size_t>(ARRAY_LEN), reinterpret_cast<void *>(&tid), sizeof(tid));
    if (ret != 0) {
        return;
    }
    pos += sizeof(tid);
    ret = memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), version.c_str(), version.size() + 1);
    if (ret != 0) {
        return;
    }
    pos += (version.size() + 1);
    ret = memcpy_s(
        except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&statusCode), sizeof(statusCode));
    if (ret != 0) {
        return;
    }
    pos += sizeof(statusCode);
    ret = memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), reason.c_str(), reason.size() + 1);
    if (ret != 0) {
        return;
    }
    pos += (reason.size() + 1);
    ret = memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), headers.c_str(), headers.size() + 1);
    if (ret != 0) {
        return;
    }
    pos += (headers.size() + 1);
    char *parcel = except;
    int testSize = INT16_SIZE;
    ResponseMessageReceiver::ResponseFromParcel(response, parcel, testSize);
    parcel = except;
    testSize = INT32_SIZE;
    ResponseMessageReceiver::ResponseFromParcel(response, parcel, testSize);
    parcel = except;
    testSize = INT32_SIZE + version.size() + 1;
    ResponseMessageReceiver::ResponseFromParcel(response, parcel, testSize);
    parcel = except;
    testSize = INT64_SIZE + version.size() + 1;
    ResponseMessageReceiver::ResponseFromParcel(response, parcel, testSize);
    parcel = except;
    testSize = ARRAY_LEN;
    ResponseMessageReceiver::ResponseFromParcel(response, parcel, testSize);
}

void ResponseMessageFuzzTestTaskStatesFromParcel(const uint8_t *data, size_t size)
{
    std::vector<TaskState> taskStates;
    int pos = 0;
    int32_t length = 1;
    std::string path = "path";
    int32_t responseCode = NETWORK_OFFLINE;
    std::string message = "message";
    char except[ARRAY_LEN];
    int ret = memcpy_s(except, static_cast<size_t>(ARRAY_LEN), reinterpret_cast<void *>(&length), sizeof(length));
    if (ret != 0) {
        return;
    }
    pos += sizeof(length);
    ret = memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), path.c_str(), path.size() + 1);
    if (ret != 0) {
        return;
    }
    pos += (path.size() + 1);
    ret = memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&responseCode),
        sizeof(responseCode));
    if (ret != 0) {
        return;
    }
    pos += sizeof(responseCode);
    ret = memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), message.c_str(), message.size() + 1);
    if (ret != 0) {
        return;
    }
    pos += (message.size() + 1);
    char *parcel = except;
    int testSize = INT16_SIZE;
    ResponseMessageReceiver::TaskStatesFromParcel(taskStates, parcel, testSize);
    parcel = except;
    testSize = INT32_SIZE;
    ResponseMessageReceiver::TaskStatesFromParcel(taskStates, parcel, testSize);
    parcel = except;
    testSize = INT32_SIZE + path.size() + 1;
    ResponseMessageReceiver::TaskStatesFromParcel(taskStates, parcel, testSize);
    parcel = except;
    testSize = INT64_SIZE + path.size() + 1;
    ResponseMessageReceiver::TaskStatesFromParcel(taskStates, parcel, testSize);
    parcel = except;
    testSize = ARRAY_LEN;
    ResponseMessageReceiver::TaskStatesFromParcel(taskStates, parcel, testSize);
}

void ResponseMessageFuzzTestNotifyDataFromParcel(const uint8_t *data, size_t size)
{
    std::shared_ptr<NotifyData> notifyData = std::make_shared<NotifyData>();
    int pos = 0;
    int32_t length = 1;
    SubscribeType type = SubscribeType::BUTT;
    uint32_t taskId = 123; // 123 is except tid
    State state = State::ANY;
    uint32_t index = 456;             // 456 is except index
    uint64_t processed = 123456;      // 123456 is except processed
    uint64_t totalProcessed = 111222; // 111222 is except totalProcessed
    int64_t value = 333444;           // 333444 is except num
    int ketValueLen = 10;             //9 is keyValue length
    char keyValue[] = "key\0value\0";
    Action action = Action::UPLOAD;
    Version version = Version::API10;
    std::string path = "path";
    int32_t responseCode = NETWORK_OFFLINE;
    std::string message = "message";
    char except[ARRAY_LEN];
    int ret = memcpy_s(except, static_cast<size_t>(ARRAY_LEN), reinterpret_cast<void *>(&type), sizeof(type));
    if (ret != 0) {
        return;
    }
    pos += sizeof(type);
    ret = memcpy_s(
        except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&taskId), sizeof(taskId));
    if (ret != 0) {
        return;
    }
    pos += sizeof(taskId);
    ret = memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&state), sizeof(state));
    if (ret != 0) {
        return;
    }
    pos += sizeof(state);
    ret = memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&index), sizeof(index));
    if (ret != 0) {
        return;
    }
    pos += sizeof(index);
    ret = memcpy_s(
        except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&processed), sizeof(processed));
    if (ret != 0) {
        return;
    }
    pos += sizeof(processed);
    ret = memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&totalProcessed),
        sizeof(totalProcessed));
    if (ret != 0) {
        return;
    }
    pos += sizeof(totalProcessed);
    ret = memcpy_s(
        except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&length), sizeof(length));
    if (ret != 0) {
        return;
    }
    pos += sizeof(length);
    ret = memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&value), sizeof(value));
    if (ret != 0) {
        return;
    }
    pos += sizeof(value);
    ret = memcpy_s(
        except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&length), sizeof(length));
    if (ret != 0) {
        return;
    }
    pos += sizeof(length);
    ret = memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), keyValue, ketValueLen);
    if (ret != 0) {
        return;
    }
    pos += ketValueLen;
    ret = memcpy_s(
        except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&action), sizeof(action));
    if (ret != 0) {
        return;
    }
    pos += sizeof(action);
    ret = memcpy_s(
        except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&version), sizeof(version));
    if (ret != 0) {
        return;
    }
    pos += sizeof(version);
    ret = memcpy_s(
        except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&length), sizeof(length));
    if (ret != 0) {
        return;
    }
    pos += sizeof(length);
    ret = memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), path.c_str(), path.size() + 1);
    if (ret != 0) {
        return;
    }
    pos += (path.size() + 1);
    ret = memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&responseCode),
        sizeof(responseCode));
    if (ret != 0) {
        return;
    }
    pos += sizeof(responseCode);
    ret = memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), message.c_str(), message.size() + 1);
    if (ret != 0) {
        return;
    }
    pos += (message.size() + 1);
    char *parcel = except;
    int testSize = INT16_SIZE;
    ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, testSize);
    parcel = except;
    testSize = INT32_SIZE;
    int maxLen = testSize;
    ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, testSize);
    parcel = except;
    maxLen += INT32_SIZE;
    testSize = maxLen;
    ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, testSize);
    parcel = except;
    maxLen += INT32_SIZE;
    testSize = maxLen;
    ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, testSize);
    parcel = except;
    maxLen += INT32_SIZE;
    testSize = maxLen;
    ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, testSize);
    parcel = except;
    maxLen += INT64_SIZE;
    testSize = maxLen;
    ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, testSize);
    parcel = except;
    maxLen += INT64_SIZE;
    testSize = maxLen;
    ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, testSize);
    parcel = except;
    maxLen += (sizeof(length) + sizeof(value));
    testSize = maxLen;
    ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, testSize);
    parcel = except;
    maxLen += (sizeof(length) + ketValueLen);
    testSize = maxLen;
    ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, testSize);
    parcel = except;
    maxLen += INT32_SIZE;
    testSize = maxLen;
    ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, testSize);
    parcel = except;
    maxLen += INT32_SIZE;
    testSize = maxLen;
    ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, testSize);
    parcel = except;
    testSize = ARRAY_LEN;
    ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, testSize);
}

class FuzzRemoteObjectImpl : public OHOS::IRemoteObject {};

void RequestSyncLoadFuzzTestOnLoadSystemAbility(const uint8_t *data, size_t size)
{
    OHOS::sptr<FuzzRemoteObjectImpl> remote;
    RequestSyncLoadCallback requestSyncLoadCallback = RequestSyncLoadCallback();
    requestSyncLoadCallback.OnLoadSystemAbilityFail(OHOS::PRINT_SERVICE_ID);
    requestSyncLoadCallback.OnLoadSystemAbilityFail(OHOS::DOWNLOAD_SERVICE_ID);
    requestSyncLoadCallback.OnLoadSystemAbilitySuccess(OHOS::PRINT_SERVICE_ID, remote);
    requestSyncLoadCallback.OnLoadSystemAbilitySuccess(OHOS::DOWNLOAD_SERVICE_ID, remote);
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
    OHOS::SubscribeSARequestFuzzTest(data, size);
    OHOS::AddAndRemoveListenerRequestFuzzTest(data, size);
    OHOS::RemoveAllListenersRequestFuzzTest(data, size);
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
    OHOS::RunCountNotifyStubFuzzTestGetInstance(data, size);
    OHOS::RunCountNotifyStubFuzzTestCallBack(data, size);
    OHOS::RunCountNotifyStubFuzzTestDone(data, size);
    OHOS::RunCountNotifyStubFuzzTestOnCallBack(data, size);
    OHOS::ResponseMessageFuzzTestInt64FromParcel(data, size);
    OHOS::ResponseMessageFuzzTestUint64FromParcel(data, size);
    OHOS::ResponseMessageFuzzTestInt32FromParcel(data, size);
    OHOS::ResponseMessageFuzzTestUint32FromParcel(data, size);
    OHOS::ResponseMessageFuzzTestInt16FromParcel(data, size);
    OHOS::ResponseMessageFuzzTestStateFromParcel(data, size);
    OHOS::ResponseMessageFuzzTestActionFromParcel(data, size);
    OHOS::ResponseMessageFuzzTestVersionFromParcel(data, size);
    OHOS::ResponseMessageFuzzTestSubscribeTypeFromParcel(data, size);
    OHOS::ResponseMessageFuzzTestStringFromParcel(data, size);
    OHOS::ResponseMessageFuzzTestResponseHeaderFromParcel(data, size);
    OHOS::ResponseMessageFuzzTestProgressExtrasFromParcel(data, size);
    OHOS::ResponseMessageFuzzTestVecInt64FromParcel(data, size);
    OHOS::ResponseMessageFuzzTestResponseMessageReceiver(data, size);
    OHOS::ResponseMessageFuzzTestMsgHeaderParcel(data, size);
    OHOS::ResponseMessageFuzzTestResponseFromParcel(data, size);
    OHOS::ResponseMessageFuzzTestTaskStatesFromParcel(data, size);
    OHOS::ResponseMessageFuzzTestNotifyDataFromParcel(data, size);
    OHOS::RequestSyncLoadFuzzTestOnLoadSystemAbility(data, size);
    return 0;
}
