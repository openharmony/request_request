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
#include <fuzzer/FuzzedDataProvider.h>

#include <securec.h>

#include <cstddef>
#include <cstdint>

#include "accesstoken_kit.h"
#include "message_parcel.h"
#include "nativetoken_kit.h"
#include "request.h"
#include "request_common.h"
#include "request_manager.h"
#include "request_manager_impl.h"
#include "request_running_task_count.h"
#include "request_service_interface.h"
#include "runcount_notify_stub.h"
#include "running_task_count.h"
#include "system_ability_definition.h"
#include "token_setproc.h"

using namespace OHOS::Request;
using namespace OHOS::Security::AccessToken;

#undef private
#undef protected

const int MAX_NUM = 20;
const int MAX_LENGTH = 50;

namespace OHOS {

constexpr std::array<OHOS::Request::ExceptionErrorCode, 19> exceptionErrorCodes = {
    E_OK, E_UNLOADING_SA, E_IPC_SIZE_TOO_LARGE, E_MIMETYPE_NOT_FOUND, E_TASK_INDEX_TOO_LARGE,
    E_CHANNEL_NOT_OPEN, E_PERMISSION, E_NOT_SYSTEM_APP, E_PARAMETER_CHECK, E_UNSUPPORTED,
    E_FILE_IO, E_FILE_PATH, E_SERVICE_ERROR, E_OTHER, E_TASK_QUEUE, E_TASK_MODE,
    E_TASK_NOT_FOUND, E_TASK_STATE, E_GROUP_NOT_FOUND
};

constexpr std::array<Action, 3> actions = {
    Action::DOWNLOAD,
    Action::UPLOAD,
    Action::ANY,
};

constexpr std::array<OHOS::Request::Version, 3> versions = {
    Version::API8,
    Version::API9,
    Version::API10,
};

constexpr std::array<OHOS::Request::Mode, 3> modes = {
    Mode::BACKGROUND,
    Mode::FOREGROUND,
    Mode::ANY,
};

Filter convertToFilter(FuzzedDataProvider &provider)
{
    std::string bundle = provider.ConsumeRandomLengthString(MAX_LENGTH);
    int64_t before = provider.ConsumeIntegral<int64_t>();
    int64_t after = provider.ConsumeIntegral<int64_t>();
    Filter filter;
    filter.bundle = bundle;
    filter.before = before;
    filter.after = after;
    return filter;
}

std::vector<TaskIdAndToken> convertToVectorTaskIdAndToken(FuzzedDataProvider &provider)
{
    std::vector<TaskIdAndToken> result;
    int len = provider.ConsumeIntegralInRange<int>(1, MAX_NUM);
    for (int i = 0; i < len; i++) {
        std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
        std::string token = provider.ConsumeRandomLengthString(MAX_LENGTH);
        TaskIdAndToken taskIdAndToken;
        taskIdAndToken.tid = tid;
        taskIdAndToken.token = token;
        result.push_back(taskIdAndToken);
    }
    return result;
}

std::vector<TaskInfoRet> convertToVectorTaskInfoRet(FuzzedDataProvider &provider)
{
    std::vector<TaskInfoRet> result;
    int len = provider.ConsumeIntegralInRange<int>(1, MAX_NUM);
    for (int i = 0; i < len; i++) {
        size_t index = provider.ConsumeIntegralInRange<size_t>(0, exceptionErrorCodes.size() - 1);
        ExceptionErrorCode code = exceptionErrorCodes[index];
        TaskInfoRet infoRet{ .code = code };
        result.push_back(infoRet);
    }
    return result;
}

std::vector<FormItem> convertToVectorFormItem(FuzzedDataProvider &provider)
{
    std::vector<FormItem> result;
    int len = provider.ConsumeIntegralInRange<int>(1, MAX_NUM);
    for (int i = 0; i < len; i++) {
        std::string name = provider.ConsumeRandomLengthString(MAX_LENGTH);
        std::string value = provider.ConsumeRandomLengthString(MAX_LENGTH);
        FormItem forItem;
        forItem.name = name;
        forItem.value = value;
        result.push_back(forItem);
    }
    return result;
}

std::vector<FileSpec> convertToVectorFileSpec(FuzzedDataProvider &provider)
{
    std::vector<FileSpec> result;
    int len = provider.ConsumeIntegralInRange<int>(1, MAX_NUM);
    for (int i = 0; i < len; i++) {
        std::string name = provider.ConsumeRandomLengthString(MAX_LENGTH);
        std::string uri = provider.ConsumeRandomLengthString(MAX_LENGTH);
        std::string filename = provider.ConsumeRandomLengthString(MAX_LENGTH);
        std::string type = provider.ConsumeRandomLengthString(MAX_LENGTH);
        FileSpec fileSpec;
        fileSpec.name = name;
        fileSpec.uri = uri;
        fileSpec.filename = filename;
        fileSpec.type = type;
        result.push_back(fileSpec);
    }
    return result;
}

std::vector<std::string> convertToVectorString(FuzzedDataProvider &provider)
{
    std::vector<std::string> result;
    int len = provider.ConsumeIntegralInRange<int>(1, MAX_NUM);
    for (int i = 0; i < len; i++) {
        std::string name = provider.ConsumeRandomLengthString(MAX_LENGTH);
        result.push_back(name);
    }
    return result;
}

std::map<std::string, std::string> convertToMapString(FuzzedDataProvider &provider)
{
    std::map<std::string, std::string> result;
    int len = provider.ConsumeIntegralInRange<int>(1, MAX_NUM);
    for (int i = 0; i < len; i++) {
        std::string key = provider.ConsumeRandomLengthString(MAX_LENGTH);
        std::string value = provider.ConsumeRandomLengthString(MAX_LENGTH);
        result.insert({key, value});
    }
    return result;
}

std::vector<TaskRet> convertToVectorTaskRet(FuzzedDataProvider &provider)
{
    std::vector<TaskRet> result;
    int len = provider.ConsumeIntegralInRange<int>(1, MAX_NUM);
    for (int i = 0; i < len; i++) {
        size_t index = provider.ConsumeIntegralInRange<size_t>(0, exceptionErrorCodes.size() - 1);
        ExceptionErrorCode code = exceptionErrorCodes[index];
        std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
        TaskRet task;
        task.code = code;
        task.tid = tid;
        result.push_back(task);
    }
    return result;
}

std::vector<ExceptionErrorCode> convertToVectorExceptionErrorCode(FuzzedDataProvider &provider)
{
    std::vector<ExceptionErrorCode> result;
    int len = provider.ConsumeIntegralInRange<int>(1, MAX_NUM);
    for (int i = 0; i < len; i++) {
        size_t index = provider.ConsumeIntegralInRange<size_t>(0, exceptionErrorCodes.size() - 1);
        ExceptionErrorCode code = exceptionErrorCodes[index];
        result.push_back(code);
    }
    return result;
}

Config convertToConfig(FuzzedDataProvider &provider)
{
    size_t actionIndex = provider.ConsumeIntegralInRange<size_t>(0, actions.size() - 1);
    Action action = actions[actionIndex];
    std::string url = provider.ConsumeRandomLengthString(MAX_LENGTH);
    std::vector<std::string> certsPath = convertToVectorString(provider);
    size_t versionIndex = provider.ConsumeIntegralInRange<size_t>(0, versions.size() - 1);
    Version version = versions[versionIndex];
    std::string bundleName = provider.ConsumeRandomLengthString(MAX_LENGTH);
    std::string title = provider.ConsumeRandomLengthString(MAX_LENGTH);
    std::string saveas = provider.ConsumeRandomLengthString(MAX_LENGTH);
    std::string method = provider.ConsumeRandomLengthString(MAX_LENGTH);
    std::string description = provider.ConsumeRandomLengthString(MAX_LENGTH);
    std::string data = provider.ConsumeRandomLengthString(MAX_LENGTH);
    std::string proxy = provider.ConsumeRandomLengthString(MAX_LENGTH);
    std::string certificatePins = provider.ConsumeRandomLengthString(MAX_LENGTH);
    std::map<std::string, std::string> headers = convertToMapString(provider);
    std::vector<FormItem> forms = convertToVectorFormItem(provider);
    std::vector<FileSpec> files = convertToVectorFileSpec(provider);
    std::vector<std::string> bodyFileNames = convertToVectorString(provider);
    std::map<std::string, std::string> extras = convertToMapString(provider);
    Config config;
    config.action = action;
    config.url = url;
    config.certsPath = certsPath;
    config.version = version;
    config.bundleName = bundleName;
    config.title = title;
    config.saveas = saveas;
    config.method = method;
    config.description = description;
    config.data = data;
    config.proxy = proxy;
    config.certificatePins = certificatePins;
    config.headers = headers;
    config.forms = forms;
    config.files = files;
    config.bodyFileNames = bodyFileNames;
    config.extras = extras;
    return config;
}

std::vector<Config> convertToVectorConfig(FuzzedDataProvider &provider)
{
    std::vector<Config> result;
    int len = provider.ConsumeIntegralInRange<int>(1, MAX_NUM);
    for (int i = 0; i < len; i++) {
        Config config = convertToConfig(provider);
        result.push_back(config);
    }
    return result;
}

std::vector<SpeedConfig> convertToVectorSpeedConfig(FuzzedDataProvider &provider)
{
    std::vector<SpeedConfig> result;
    int len = provider.ConsumeIntegralInRange<int>(1, MAX_NUM);
    for (int i = 0; i < len; i++) {
        std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
        int32_t speed = provider.ConsumeIntegral<int32_t>();
        SpeedConfig speedConfig;
        speedConfig.tid = tid;
        speedConfig.maxSpeed = speed;
        result.push_back(speedConfig);
    }
    return result;
}

std::vector<uint8_t> convertToVectorUint8_t(FuzzedDataProvider &provider)
{
    std::vector<uint8_t> result;
    int len = provider.ConsumeIntegralInRange<int>(1, MAX_NUM);
    for (int i = 0; i < len; i++) {
        uint8_t value = provider.ConsumeIntegral<uint8_t>();
        result.push_back(value);
    }
    return result;
}

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

// @tc.name: ut_create_request_fuzzer
// @tc.desc: Fuzz test for creating a request
// @tc.precon: NA
// @tc.step: 1. Convert input data to task ID string
// 2. Grant native permission
// 3. Get next sequence number
// 4. Call Create method with config and task ID
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void CreateRequestFuzzTest(const uint8_t *data, size_t size)
{
    Config config;
    std::string tid(reinterpret_cast<const char *>(data), size);

    GrantNativePermission();
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    RequestManager::GetInstance()->Create(config, seq, tid);
}

// @tc.name: ut_start_request_fuzzer
// @tc.desc: Fuzz test for starting a request
// @tc.precon: NA
// @tc.step: 1. Convert input data to task ID string
// 2. Grant native permission
// 3. Call Start method with task ID
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void StartRequestFuzzTest(const uint8_t *data, size_t size)
{
    std::string tid(reinterpret_cast<const char *>(data), size);
    GrantNativePermission();
    RequestManager::GetInstance()->Start(tid);
}

// @tc.name: ut_stop_request_fuzzer
// @tc.desc: Fuzz test for stopping a request
// @tc.precon: NA
// @tc.step: 1. Convert input data to task ID string
// 2. Grant native permission
// 3. Call Stop method with task ID
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void StopRequestFuzzTest(const uint8_t *data, size_t size)
{
    std::string tid(reinterpret_cast<const char *>(data), size);
    GrantNativePermission();
    RequestManager::GetInstance()->Stop(tid);
}

// @tc.name: ut_show_request_fuzzer
// @tc.desc: Fuzz test for showing request information
// @tc.precon: NA
// @tc.step: 1. Create TaskInfo object
// 2. Convert input data to task ID string
// 3. Grant native permission
// 4. Call Show method with task ID and TaskInfo
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void ShowRequestFuzzTest(const uint8_t *data, size_t size)
{
    TaskInfo info;
    std::string tid(reinterpret_cast<const char *>(data), size);
    GrantNativePermission();
    RequestManager::GetInstance()->Show(tid, info);
}

// @tc.name: ut_touch_request_fuzzer
// @tc.desc: Fuzz test for touching a request
// @tc.precon: NA
// @tc.step: 1. Create TaskInfo object
// 2. Convert input data to task ID string
// 3. Create token from input data
// 4. Grant native permission
// 5. Call Touch method with task ID, token and TaskInfo
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void TouchRequestFuzzTest(FuzzedDataProvider &provider)
{
    TaskInfo info;

    std::string tid = provider.ConsumeRandomLengthString();
    std::string token = provider.ConsumeRandomLengthString();
    GrantNativePermission();
    RequestManager::GetInstance()->Touch(tid, token, info);
}

// @tc.name: ut_search_request_fuzzer
// @tc.desc: Fuzz test for searching requests
// @tc.precon: NA
// @tc.step: 1. Create Filter object
// 2. Create vector for task IDs
// 3. Convert input data to string and add to vector
// 4. Grant native permission
// 5. Call Search method with filter and task IDs vector
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void SearchRequestFuzzTest(const uint8_t *data, size_t size)
{
    Filter filter;
    std::vector<std::string> tids;
    std::string str(reinterpret_cast<const char *>(data), size);
    tids.push_back(str);
    GrantNativePermission();
    RequestManager::GetInstance()->Search(filter, tids);
}

// @tc.name: ut_pause_request_fuzzer
// @tc.desc: Fuzz test for pausing a request
// @tc.precon: NA
// @tc.step: 1. Convert input data to Version enum
// 2. Convert input data to task ID string
// 3. Grant native permission
// 4. Call Pause method with task ID and version
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void PauseRequestFuzzTest(const uint8_t *data, size_t size)
{
    Version version = static_cast<Version>(ConvertToUint32(data, size));
    std::string tid(reinterpret_cast<const char *>(data), size);
    GrantNativePermission();
    RequestManager::GetInstance()->Pause(tid, version);
}

// @tc.name: ut_query_mimetype_request_fuzzer
// @tc.desc: Fuzz test for querying MIME type
// @tc.precon: NA
// @tc.step: 1. Convert input data to task ID string
// 2. Create MIME type string from input data
// 3. Grant native permission
// 4. Call QueryMimeType method with task ID and MIME type
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void QueryMimeTypeRequestFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString();
    std::string mimeType = provider.ConsumeRandomLengthString();
    GrantNativePermission();
    RequestManager::GetInstance()->QueryMimeType(tid, mimeType);
}

// @tc.name: ut_remove_request_fuzzer
// @tc.desc: Fuzz test for removing a request
// @tc.precon: NA
// @tc.step: 1. Convert input data to Version enum
// 2. Convert input data to task ID string
// 3. Grant native permission
// 4. Call Remove method with task ID and version
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void RemoveRequestFuzzTest(const uint8_t *data, size_t size)
{
    Version version = static_cast<Version>(ConvertToUint32(data, size));
    std::string tid(reinterpret_cast<const char *>(data), size);
    GrantNativePermission();
    RequestManager::GetInstance()->Remove(tid, version);
}

// @tc.name: ut_resume_request_fuzzer
// @tc.desc: Fuzz test for resuming a request
// @tc.precon: NA
// @tc.step: 1. Convert input data to task ID string
// 2. Grant native permission
// 3. Call Resume method with task ID
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void ResumeRequestFuzzTest(const uint8_t *data, size_t size)
{
    std::string tid(reinterpret_cast<const char *>(data), size);
    GrantNativePermission();
    RequestManager::GetInstance()->Resume(tid);
}

// @tc.name: ut_get_task_request_fuzzer
// @tc.desc: Fuzz test for getting task information
// @tc.precon: NA
// @tc.step: 1. Create Config object
// 2. Convert input data to task ID string
// 3. Create token from input data
// 4. Grant native permission
// 5. Call GetTask method with task ID, token and config
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void GetTaskRequestFuzzTest(FuzzedDataProvider &provider)
{
    Config config;

    std::string tid = provider.ConsumeRandomLengthString();
    std::string token = provider.ConsumeRandomLengthString();

    GrantNativePermission();
    RequestManager::GetInstance()->GetTask(tid, token, config);
}

// @tc.name: ut_subscribe_request_fuzzer
// @tc.desc: Fuzz test for subscribing to a request
// @tc.precon: NA
// @tc.step: 1. Convert input data to task ID string
// 2. Grant native permission
// 3. Call Subscribe method with task ID
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void SubscribeRequestFuzzTest(const uint8_t *data, size_t size)
{
    std::string tid(reinterpret_cast<const char *>(data), size);
    GrantNativePermission();
    RequestManager::GetInstance()->Subscribe(tid);
}

// @tc.name: ut_unsubscribe_request_fuzzer
// @tc.desc: Fuzz test for unsubscribing from a request
// @tc.precon: NA
// @tc.step: 1. Convert input data to task ID string
// 2. Grant native permission
// 3. Call Unsubscribe method with task ID
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void UnsubscribeRequestFuzzTest(const uint8_t *data, size_t size)
{
    std::string tid(reinterpret_cast<const char *>(data), size);
    GrantNativePermission();
    RequestManager::GetInstance()->Unsubscribe(tid);
}

// @tc.name: ut_is_sa_ready_request_fuzzer
// @tc.desc: Fuzz test for checking SA readiness
// @tc.precon: NA
// @tc.step: 1. Grant native permission
// 2. Call IsSaReady method
// 3. Convert input data to task ID string
// 4. Call Start method with task ID
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void IsSaReadyRequestFuzzTest(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    RequestManager::GetInstance()->IsSaReady();
    std::string tid(reinterpret_cast<const char *>(data), size);
    RequestManager::GetInstance()->Start(tid);
}

// @tc.name: ut_reopen_channel_request_fuzzer
// @tc.desc: Fuzz test for reopening channel
// @tc.precon: NA
// @tc.step: 1. Grant native permission
// 2. Call ReopenChannel method
// 3. Convert input data to task ID string
// 4. Call Start method with task ID
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void ReopenChannelRequestFuzzTest(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    RequestManager::GetInstance()->ReopenChannel();
    std::string tid(reinterpret_cast<const char *>(data), size);
    RequestManager::GetInstance()->Start(tid);
}

// @tc.name: ut_subscribe_sa_request_fuzzer
// @tc.desc: Fuzz test for subscribing and unsubscribing SA
// @tc.precon: NA
// @tc.step: 1. Grant native permission
// 2. Call SubscribeSA method
// 3. Call UnsubscribeSA method
// 4. Convert input data to task ID string
// 5. Call Start method with task ID
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void SubscribeSARequestFuzzTest(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    RequestManager::GetInstance()->SubscribeSA();
    RequestManager::GetInstance()->UnsubscribeSA();
    std::string tid(reinterpret_cast<const char *>(data), size);
    RequestManager::GetInstance()->Start(tid);
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
    void OnFaultsReceive(const std::shared_ptr<int32_t> &tid, const std::shared_ptr<SubscribeType> &type,
        const std::shared_ptr<Reason> &reason) override
    {
    }
    void OnWaitReceive(std::int32_t taskId, WaitingReason reason) override
    {
    }
};

// @tc.name: ut_add_remove_listener_request_fuzzer
// @tc.desc: Fuzz test for adding and removing listeners
// @tc.precon: NA
// @tc.step: 1. Convert input data to task ID string
// 2. Grant native permission
// 3. Create response listener and add to request
// 4. Remove response listener
// 5. Create notify data listener and add to request
// 6. Remove notify data listener
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
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

// @tc.name: ut_remove_all_listeners_request_fuzzer
// @tc.desc: Fuzz test for removing all listeners
// @tc.precon: NA
// @tc.step: 1. Convert input data to task ID string
// 2. Grant native permission
// 3. Add response listener
// 4. Add notify data listener
// 5. Call RemoveAllListeners method
// 6. Call RestoreListener with nullptr
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
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

// @tc.name: ut_restore_listener_request_fuzzer
// @tc.desc: Fuzz test for restoring listeners
// @tc.precon: NA
// @tc.step: 1. Grant native permission
// 2. Convert input data to task ID string
// 3. Call Start method with task ID
// 4. Call RestoreListener with TestFunc
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void RestoreListenerRequestFuzzTest(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    std::string tid(reinterpret_cast<const char *>(data), size);
    RequestManager::GetInstance()->Start(tid);
    RequestManager::GetInstance()->RestoreListener(TestFunc);
}

// @tc.name: ut_query_request_fuzzer
// @tc.desc: Fuzz test for querying request information
// @tc.precon: NA
// @tc.step: 1. Convert input data to task ID string
// 2. Create TaskInfo object
// 3. Grant native permission
// 4. Call Query method with task ID and TaskInfo
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void QueryRequestFuzzTest(const uint8_t *data, size_t size)
{
    std::string tid(reinterpret_cast<const char *>(data), size);
    TaskInfo taskinfo;
    GrantNativePermission();
    RequestManager::GetInstance()->Query(tid, taskinfo);
}

// @tc.name: ut_request_get_id_fuzzer
// @tc.desc: Fuzz test for Request getId method
// @tc.precon: NA
// @tc.step: 1. Convert input data to task ID string
// 2. Create Request object with task ID
// 3. Call getId method
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void RequestFuzzTestGetId(const uint8_t *data, size_t size)
{
    std::string tid(reinterpret_cast<const char *>(data), size);
    GrantNativePermission();
    auto request = OHOS::Request::Request(tid);
    request.getId();
}

// @tc.name: ut_request_has_listener_fuzzer
// @tc.desc: Fuzz test for Request HasListener method
// @tc.precon: NA
// @tc.step: 1. Convert input data to task ID string
// 2. Create Request object with task ID
// 3. Create response listener
// 4. Add listener to request
// 5. Call HasListener method
// 6. Remove listener
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
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

// @tc.name: ut_request_on_notify_data_receive_fuzzer
// @tc.desc: Fuzz test for Request OnNotifyDataReceive method
// @tc.precon: NA
// @tc.step: 1. Convert input data to task ID string
// 2. Create Request object with task ID
// 3. Create and configure NotifyData object
// 4. Call OnNotifyDataReceive without listener
// 5. Add notify data listener
// 6. Call OnNotifyDataReceive again with listener
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
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

// @tc.name: ut_request_add_remove_listener_fuzzer
// @tc.desc: Fuzz test for Request AddListener and RemoveListener methods
// @tc.precon: NA
// @tc.step: 1. Convert input data to task ID string
// 2. Create Request object with task ID
// 3. Create and configure NotifyData object
// 4. Call OnNotifyDataReceive without listener
// 5. Add notify data listener
// 6. Remove notify data listener
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
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

// @tc.name: ut_request_on_response_receive_fuzzer
// @tc.desc: Fuzz test for Request OnResponseReceive method
// @tc.precon: NA
// @tc.step: 1. Convert input data to task ID string
// 2. Create Request object with task ID
// 3. Create Response object
// 4. Call OnResponseReceive without listener
// 5. Add response listener
// 6. Call OnResponseReceive again with listener
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
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

// @tc.name: ut_running_task_count_subscribe_fuzzer
// @tc.desc: Fuzz test for subscribing to running task count
// @tc.precon: NA
// @tc.step: 1. Grant native permission
// 2. Try to get service proxy
// 3. Create observer and test subscription functionality
// 4. Test attaching/detaching observers
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void RunningTaskCountFuzzTestSubscribeRunningTaskCount(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        std::shared_ptr<IRunningTaskObserver> ob = std::make_shared<FuzzFwkTestOberver>();
        ob->OnRunningTaskCountUpdate(static_cast<int>(*data));
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

// @tc.name: ut_running_task_count_unsubscribe_fuzzer
// @tc.desc: Fuzz test for unsubscribing from running task count
// @tc.precon: NA
// @tc.step: 1. Grant native permission
// 2. Create observer and attach it
// 3. Test OnRunningTaskCountUpdate
// 4. Test unsubscribe with different observers
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void RunningTaskCountFuzzTestUnubscribeRunning(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    std::shared_ptr<IRunningTaskObserver> ob1 = std::make_shared<FuzzFwkTestOberver>();
    FwkRunningTaskCountManager::GetInstance()->AttachObserver(ob1);
    ob1->OnRunningTaskCountUpdate(static_cast<int>(*data));

    std::shared_ptr<IRunningTaskObserver> ob2 = std::make_shared<FuzzFwkTestOberver>();
    UnsubscribeRunningTaskCount(ob2);
    UnsubscribeRunningTaskCount(ob1);
}

// @tc.name: ut_running_task_count_get_set_fuzzer
// @tc.desc: Fuzz test for getting and setting running task count
// @tc.precon: NA
// @tc.step: 1. Grant native permission
// 2. Convert input data to integer value
// 3. Set count to expected value
// 4. Test GetCount method
// 5. Restore original count
// 6. Test GetCount again
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void RunningTaskCountFuzzTestGetAndSetCount(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    int old = static_cast<int>(*data);
    int except = 1; // 10 is except count num
    FwkRunningTaskCountManager::GetInstance()->SetCount(except);
    FwkRunningTaskCountManager::GetInstance()->GetCount();
    FwkRunningTaskCountManager::GetInstance()->SetCount(old);
    FwkRunningTaskCountManager::GetInstance()->GetCount();
}

// @tc.name: ut_running_task_count_update_fuzzer
// @tc.desc: Fuzz test for updating running task count
// @tc.precon: NA
// @tc.step: 1. Grant native permission
// 2. Create observer
// 3. Test OnRunningTaskCountUpdate
// 4. Create FwkIRunningTaskObserver and test UpdateRunningTaskCount
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void RunningTaskCountFuzzTestUpdateRunningTaskCount(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    std::shared_ptr<IRunningTaskObserver> ob = std::make_shared<FuzzFwkTestOberver>();
    ob->OnRunningTaskCountUpdate(static_cast<int>(*data));
    FwkIRunningTaskObserver runningOb = FwkIRunningTaskObserver(ob);
    runningOb.UpdateRunningTaskCount();
}

// @tc.name: ut_running_task_count_notify_observers_fuzzer
// @tc.desc: Fuzz test for notifying all observers
// @tc.precon: NA
// @tc.step: 1. Grant native permission
// 2. Create observer and attach it
// 3. Call NotifyAllObservers
// 4. Detach observer
// 5. Test OnRunningTaskCountUpdate
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void RunningTaskCountFuzzTestNotifyAllObservers(const uint8_t *data, size_t size)
{
    GrantNativePermission();
    std::shared_ptr<IRunningTaskObserver> ob1 = std::make_shared<FuzzFwkTestOberver>();
    FwkRunningTaskCountManager::GetInstance()->AttachObserver(ob1);
    FwkRunningTaskCountManager::GetInstance()->NotifyAllObservers();
    FwkRunningTaskCountManager::GetInstance()->DetachObserver(ob1);
    ob1->OnRunningTaskCountUpdate(static_cast<int>(*data));
}

// @tc.name: ut_run_count_notify_stub_instance_callback_fuzzer
// @tc.desc: Fuzz test for RunCountNotifyStub getInstance, Done and CallBack methods
// @tc.precon: NA
// @tc.step: 1. Create TaskInfo and configure with input data
// 2. Create Notify object
// 3. Grant native permission
// 4. Test GetInstance, Done and CallBack methods
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void RunCountNotifyStubFuzzTestGetInstanceDoneCallBack(const uint8_t *data, size_t size)
{
    TaskInfo taskInfo;
    taskInfo.tid = std::string(reinterpret_cast<const char *>(data), size);
    Notify notify;
    GrantNativePermission();

    RunCountNotifyStub::GetInstance();
    RunCountNotifyStub::GetInstance()->Done(taskInfo);
    RunCountNotifyStub::GetInstance()->CallBack(notify);
}

// @tc.name: ut_run_count_notify_stub_on_callback_fuzzer
// @tc.desc: Fuzz test for RunCountNotifyStub OnCallBack method
// @tc.precon: NA
// @tc.step: 1. Convert input data to int64 value
// 2. Save current count
// 3. Create and configure MessageParcel
// 4. Grant native permission
// 5. Call OnCallBack method
// 6. Test GetCount and restore original count
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void RunCountNotifyStubFuzzTestOnCallBack(const uint8_t *data, size_t size)
{
    int64_t except = static_cast<int64_t>(*data);
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

// @tc.name: ut_response_message_int64_from_parcel_fuzzer
// @tc.desc: Fuzz test for ResponseMessageReceiver Int64FromParcel method
// @tc.precon: NA
// @tc.step: 1. Convert input data to int64 value
// 2. Create parcel pointer
// 3. Test Int64FromParcel with INT32_SIZE
// 4. Test Int64FromParcel with INT64_SIZE
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void ResponseMessageFuzzTestInt64FromParcel(const uint8_t *data, size_t size)
{
    int64_t except = static_cast<int64_t>(*data);
    char *parcel = reinterpret_cast<char *>(&except);
    int64_t num;
    int testSize = INT32_SIZE;
    ResponseMessageReceiver::Int64FromParcel(num, parcel, testSize);
    testSize = INT64_SIZE;
    ResponseMessageReceiver::Int64FromParcel(num, parcel, testSize);
}

// @tc.name: ut_response_message_uint64_from_parcel_fuzzer
// @tc.desc: Fuzz test for ResponseMessageReceiver Uint64FromParcel method
// @tc.precon: NA
// @tc.step: 1. Convert input data to uint64 value
// 2. Create parcel pointer
// 3. Test Uint64FromParcel with INT32_SIZE
// 4. Test Uint64FromParcel with INT64_SIZE
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void ResponseMessageFuzzTestUint64FromParcel(const uint8_t *data, size_t size)
{
    uint64_t except = static_cast<uint64_t>(*data);
    char *parcel = reinterpret_cast<char *>(&except);
    uint64_t num;
    int testSize = INT32_SIZE;
    ResponseMessageReceiver::Uint64FromParcel(num, parcel, testSize);
    testSize = INT64_SIZE;
    ResponseMessageReceiver::Uint64FromParcel(num, parcel, testSize);
}

// @tc.name: ut_response_message_int32_from_parcel_fuzzer
// @tc.desc: Fuzz test for ResponseMessageReceiver Int32FromParcel method
// @tc.precon: NA
// @tc.step: 1. Convert input data to int32 value
// 2. Create parcel pointer
// 3. Test Int32FromParcel with INT16_SIZE
// 4. Test Int32FromParcel with INT32_SIZE
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void ResponseMessageFuzzTestInt32FromParcel(const uint8_t *data, size_t size)
{
    int32_t except = static_cast<int32_t>(*data);
    char *parcel = reinterpret_cast<char *>(&except);
    int32_t num;
    int testSize = INT16_SIZE;
    ResponseMessageReceiver::Int32FromParcel(num, parcel, testSize);
    testSize = INT32_SIZE;
    ResponseMessageReceiver::Int32FromParcel(num, parcel, testSize);
}

// @tc.name: ut_response_message_uint32_from_parcel_fuzzer
// @tc.desc: Fuzz test for ResponseMessageReceiver Uint32FromParcel method
// @tc.precon: NA
// @tc.step: 1. Convert input data to uint32 value
// 2. Create parcel pointer
// 3. Test Uint32FromParcel with INT16_SIZE
// 4. Test Uint32FromParcel with INT32_SIZE
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void ResponseMessageFuzzTestUint32FromParcel(const uint8_t *data, size_t size)
{
    uint32_t except = static_cast<uint32_t>(*data);
    char *parcel = reinterpret_cast<char *>(&except);
    uint32_t num;
    int testSize = INT16_SIZE;
    ResponseMessageReceiver::Uint32FromParcel(num, parcel, testSize);
    testSize = INT32_SIZE;
    ResponseMessageReceiver::Uint32FromParcel(num, parcel, testSize);
}

// @tc.name: ut_response_message_int16_from_parcel_fuzzer
// @tc.desc: Fuzz test for ResponseMessageReceiver Int16FromParcel method
// @tc.precon: NA
// @tc.step: 1. Convert input data to int16 value
// 2. Create parcel pointer
// 3. Test Int16FromParcel with size 0
// 4. Test Int16FromParcel with INT16_SIZE
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void ResponseMessageFuzzTestInt16FromParcel(const uint8_t *data, size_t size)
{
    int16_t except = static_cast<int16_t>(*data);
    char *parcel = reinterpret_cast<char *>(&except);
    int16_t num;
    int testSize = 0;
    ResponseMessageReceiver::Int16FromParcel(num, parcel, testSize);
    testSize = INT16_SIZE;
    ResponseMessageReceiver::Int16FromParcel(num, parcel, testSize);
}

// @tc.name: ut_response_message_state_from_parcel_fuzzer
// @tc.desc: Fuzz test for ResponseMessageReceiver StateFromParcel method
// @tc.precon: NA
// @tc.step: 1. Convert input data to State enum
// 2. Test with invalid state value
// 3. Test with valid state value (State::ANY)
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void ResponseMessageFuzzTestStateFromParcel(const uint8_t *data, size_t size)
{
    State state = static_cast<State>(*data);
    uint32_t except = static_cast<uint32_t>(State::ANY) + 1;
    char *parcel = reinterpret_cast<char *>(&except);
    int testSize = INT32_SIZE;
    ResponseMessageReceiver::StateFromParcel(state, parcel, testSize);
    except = static_cast<uint32_t>(State::ANY);
    parcel = reinterpret_cast<char *>(&except);
    testSize = INT32_SIZE;
    ResponseMessageReceiver::StateFromParcel(state, parcel, testSize);
}

// @tc.name: ut_response_message_action_from_parcel_fuzzer
// @tc.desc: Fuzz test for ResponseMessageReceiver ActionFromParcel method
// @tc.precon: NA
// @tc.step: 1. Convert input data to Action enum
// 2. Test with invalid action value
// 3. Test with valid action value (Action::ANY)
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void ResponseMessageFuzzTestActionFromParcel(const uint8_t *data, size_t size)
{
    Action action = static_cast<Action>(*data);
    uint32_t except = static_cast<uint32_t>(Action::ANY) + 1;
    char *parcel = reinterpret_cast<char *>(&except);
    int testSize = INT32_SIZE;
    ResponseMessageReceiver::ActionFromParcel(action, parcel, testSize);
    except = static_cast<uint32_t>(Action::ANY);
    parcel = reinterpret_cast<char *>(&except);
    testSize = INT32_SIZE;
    ResponseMessageReceiver::ActionFromParcel(action, parcel, testSize);
}

// @tc.name: ut_response_message_version_from_parcel_fuzzer
// @tc.desc: Fuzz test for ResponseMessageReceiver VersionFromParcel method
// @tc.precon: NA
// @tc.step: 1. Convert input data to Version enum
// 2. Test with invalid version value
// 3. Test with valid version value (Version::API10)
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void ResponseMessageFuzzTestVersionFromParcel(const uint8_t *data, size_t size)
{
    Version version = static_cast<Version>(*data);
    uint32_t except = static_cast<uint32_t>(Version::API10) + 1;
    char *parcel = reinterpret_cast<char *>(&except);
    int testSize = INT32_SIZE;
    ResponseMessageReceiver::VersionFromParcel(version, parcel, testSize);
    except = static_cast<uint32_t>(Version::API10);
    parcel = reinterpret_cast<char *>(&except);
    testSize = INT32_SIZE;
    ResponseMessageReceiver::VersionFromParcel(version, parcel, testSize);
}

// @tc.name: ut_response_message_subscribe_type_from_parcel_fuzzer
// @tc.desc: Fuzz test for ResponseMessageReceiver SubscribeTypeFromParcel method
// @tc.precon: NA
// @tc.step: 1. Convert input data to SubscribeType enum
// 2. Test with invalid type value
// 3. Test with valid type value (SubscribeType::BUTT)
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void ResponseMessageFuzzTestSubscribeTypeFromParcel(const uint8_t *data, size_t size)
{
    SubscribeType type = static_cast<SubscribeType>(*data);
    uint32_t except = static_cast<uint32_t>(SubscribeType::BUTT) + 1;
    char *parcel = reinterpret_cast<char *>(&except);
    int testSize = INT32_SIZE;
    ResponseMessageReceiver::SubscribeTypeFromParcel(type, parcel, testSize);
    except = static_cast<uint32_t>(SubscribeType::BUTT);
    parcel = reinterpret_cast<char *>(&except);
    testSize = INT32_SIZE;
    ResponseMessageReceiver::SubscribeTypeFromParcel(type, parcel, testSize);
}

// @tc.name: ut_response_message_string_from_parcel_fuzzer
// @tc.desc: Fuzz test for ResponseMessageReceiver StringFromParcel method
// @tc.precon: NA
// @tc.step: 1. Convert input data to string
// 2. Create parcel pointer from string
// 3. Test StringFromParcel with size-1
// 4. Test StringFromParcel with size+1
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void ResponseMessageFuzzTestStringFromParcel(const uint8_t *data, size_t size)
{
    std::string str;
    std::string except(reinterpret_cast<const char *>(data), size);
    char *parcel = const_cast<char *>(except.c_str());
    int testSize = except.size() - 1;
    ResponseMessageReceiver::StringFromParcel(str, parcel, testSize);
    testSize = except.size() + 1;
    ResponseMessageReceiver::StringFromParcel(str, parcel, testSize);
}

// @tc.name: ut_response_message_fuzz_test_response_header_from_parcel
// @tc.desc: Fuzz test for ResponseMessageReceiver ResponseHeaderFromParcel method
// @tc.precon: NA
// @tc.step: 1. Create a new Parcel object
// 2. Write random data to the parcel
// 3. Create ResponseMessageReceiver instance
// 4. Call ResponseHeaderFromParcel method with parcel
// @tc.expect: Function should handle various parcel data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void ResponseMessageFuzzTestResponseHeaderFromParcel(const uint8_t *data, size_t size)
{
    std::map<std::string, std::vector<std::string>> headers;
    std::string except = "header:aaa,bbb,ccc\n";
    char *parcel = const_cast<char *>(except.c_str());
    int testSize = except.size();
    ResponseMessageReceiver::ResponseHeaderFromParcel(headers, parcel, testSize);
    std::string str(reinterpret_cast<const char *>(data), size);
    parcel = const_cast<char *>(str.c_str());
    testSize = except.size();
    ResponseMessageReceiver::ResponseHeaderFromParcel(headers, parcel, testSize);
}

void ResponseMessageFuzzTestProgressExtrasFromParcel(const uint8_t *data, size_t size)
{
    int arraySize = 64; // 64 is char array length
    char except[arraySize];
    uint32_t length = static_cast<uint32_t>(*data);
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

// @tc.name: ut_response_message_fuzz_test_vec_int64_from_parcel
// @tc.desc: Fuzz test for ResponseMessageReceiver VecInt64FromParcel method
// @tc.precon: NA
// @tc.step: 1. Create test array with INT32_SIZE + INT64_SIZE
// 2. Copy input data to test array as length
// 3. Copy test value to the array
// 4. Test VecInt64FromParcel with INT16_SIZE
// 5. Test VecInt64FromParcel with INT64_SIZE
// 6. Test VecInt64FromParcel with full array size
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void ResponseMessageFuzzTestVecInt64FromParcel(const uint8_t *data, size_t size)
{
    int arraySize = INT32_SIZE + INT64_SIZE;
    char except[arraySize];
    uint32_t length = static_cast<uint32_t>(*data);
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

// @tc.name: ut_response_message_fuzz_test_response_message_receiver
// @tc.desc: Fuzz test for ResponseMessageReceiver constructor with various input data
// @tc.precon: NA
// @tc.step: 1. Create a null IResponseMessageHandler pointer
// 2. Extract socket file descriptor from fuzz data
// 3. Create ResponseMessageReceiver instance with the handler and socket descriptor
// @tc.expect: No crash occurs during object creation
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 1
void ResponseMessageFuzzTestResponseMessageReceiver(const uint8_t *data, size_t size)
{
    IResponseMessageHandler *handler = nullptr;
    int32_t sockFd = static_cast<int32_t>(*data);
    ResponseMessageReceiver receiver = ResponseMessageReceiver(handler, sockFd);
}

// @tc.name: ut_response_message_fuzz_test_msg_header_parcel
// @tc.desc: Fuzz test for ResponseMessageReceiver MsgHeaderParcel method
// @tc.precon: NA
// @tc.step: 1. Create MessageHeader object
// 2. Convert input data to various header fields
// 3. Test MsgHeaderParcel with different input sizes and values
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
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
    int32_t msgId = static_cast<int32_t>(*data);
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

// @tc.name: ut_response_message_fuzz_test_response_from_parcel
// @tc.desc: Fuzz test for ResponseMessageReceiver ResponseFromParcel method
// @tc.precon: NA
// @tc.step: 1. Create shared_ptr<Response> object
// 2. Create test array with various data fields
// 3. Copy task ID, version, status code, reason, headers to test array
// 4. Test ResponseFromParcel with INT16_SIZE
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void ResponseMessageFuzzTestResponseFromParcel(const uint8_t *data, size_t size)
{
    std::shared_ptr<Response> response = std::make_shared<Response>();
    int pos = 0;
    int32_t tid = static_cast<int32_t>(*data);
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

// @tc.name: ut_response_message_fuzz_test_task_states_from_parcel
// @tc.desc: Fuzz test for ResponseMessageReceiver TaskStatesFromParcel method
// @tc.precon: NA
// @tc.step: 1. Create vector<TaskState> object
// 2. Create test array with INT32_SIZE length and other data
// 3. Copy length, path, response code, message to test array
// 4. Test TaskStatesFromParcel with various test sizes
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void ResponseMessageFuzzTestTaskStatesFromParcel(const uint8_t *data, size_t size)
{
    std::vector<TaskState> taskStates;
    int pos = 0;
    int32_t length = static_cast<int32_t>(*data);
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

// @tc.name: ut_response_message_fuzz_test_notify_data_from_parcel
// @tc.desc: Fuzz test for ResponseMessageReceiver NotifyDataFromParcel method
// @tc.precon: NA
// @tc.step: 1. Create shared_ptr<NotifyData> object
// 2. Create test array with various data fields
// 3. Copy subscribe type, task ID, state, index, processed, etc. to test array
// 4. Test NotifyDataFromParcel with various parameters
// @tc.expect: Function should handle various input data without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
void ResponseMessageFuzzTestNotifyDataFromParcel(const uint8_t *data, size_t size)
{
    std::shared_ptr<NotifyData> notifyData = std::make_shared<NotifyData>();
    int pos = 0;
    int32_t length = static_cast<int32_t>(*data);
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

void RequestManagerFuzzTestCreateTasks(FuzzedDataProvider &provider)
{
    std::vector<Config> configs = convertToVectorConfig(provider);
    std::vector<TaskRet> rets = convertToVectorTaskRet(provider);
    RequestManager::GetInstance()->CreateTasks(configs, rets);
}

void RequestManagerFuzzTestStartTasks(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<ExceptionErrorCode> err = convertToVectorExceptionErrorCode(provider);
    RequestManager::GetInstance()->StartTasks(tids, err);
}

void RequestManagerFuzzTestStopTasks(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<ExceptionErrorCode> err = convertToVectorExceptionErrorCode(provider);
    RequestManager::GetInstance()->StopTasks(tids, err);
}

void RequestManagerFuzzTestResumeTasks(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<ExceptionErrorCode> err = convertToVectorExceptionErrorCode(provider);
    RequestManager::GetInstance()->ResumeTasks(tids, err);
}

void RequestManagerFuzzTestRemoveTasks(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<ExceptionErrorCode> err = convertToVectorExceptionErrorCode(provider);
    Version version = versions[provider.ConsumeIntegralInRange<size_t>(0, versions.size() - 1)];
    RequestManager::GetInstance()->RemoveTasks(tids, version, err);
}

void RequestManagerFuzzTestPauseTasks(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<ExceptionErrorCode> err = convertToVectorExceptionErrorCode(provider);
    Version version = versions[provider.ConsumeIntegralInRange<size_t>(0, versions.size() - 1)];
    RequestManager::GetInstance()->PauseTasks(tids, version, err);
}

void RequestManagerFuzzTestShowTasks(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<TaskInfoRet> tasks = convertToVectorTaskInfoRet(provider);
    RequestManager::GetInstance()->ShowTasks(tids, tasks);
}

void RequestManagerFuzzTestTouchTasks(FuzzedDataProvider &provider)
{
    std::vector<TaskIdAndToken> taskid = convertToVectorTaskIdAndToken(provider);
    std::vector<TaskInfoRet> taskinfo = convertToVectorTaskInfoRet(provider);
    RequestManager::GetInstance()->TouchTasks(taskid, taskinfo);
}

void RequestManagerFuzzTestSetMaxSpeeds(FuzzedDataProvider &provider)
{
    std::vector<SpeedConfig> speedconfig = convertToVectorSpeedConfig(provider);
    std::vector<ExceptionErrorCode> err = convertToVectorExceptionErrorCode(provider);
    RequestManager::GetInstance()->SetMaxSpeeds(speedconfig, err);
}

void RequestManagerFuzzTestSetMode(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    Mode mode = modes[provider.ConsumeIntegralInRange<size_t>(0, modes.size() - 1)];
    RequestManager::GetInstance()->SetMode(tid, mode);
}

void RequestManagerFuzzTestDisableTaskNotification(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<ExceptionErrorCode> err = convertToVectorExceptionErrorCode(provider);
    RequestManager::GetInstance()->DisableTaskNotification(tids, err);
}

void RequestManagerFuzzTestSetMaxSpeed(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    int64_t maxSpeed = provider.ConsumeIntegral<int64_t>();
    RequestManager::GetInstance()->SetMaxSpeed(tid, maxSpeed);
    RequestManager::GetInstance()->LoadRequestServer();
}

void RequestManagerFuzzTestCreateGroup(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    bool gauge = provider.ConsumeBool();
    Notification notification;
    RequestManager::GetInstance()->CreateGroup(tid, gauge, notification);
}

void RequestManagerFuzzTestAttachGroup(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    RequestManager::GetInstance()->AttachGroup(tid, tids);
}

void RequestManagerFuzzTestDeleteGroup(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    RequestManager::GetInstance()->DeleteGroup(tid);
}

void RequestManagerFuzzTest(FuzzedDataProvider &provider)
{
    OHOS::RequestManagerFuzzTestCreateTasks(provider);
    OHOS::RequestManagerFuzzTestStartTasks(provider);
    OHOS::RequestManagerFuzzTestStopTasks(provider);
    OHOS::RequestManagerFuzzTestResumeTasks(provider);
    OHOS::RequestManagerFuzzTestRemoveTasks(provider);
    OHOS::RequestManagerFuzzTestPauseTasks(provider);
    OHOS::RequestManagerFuzzTestShowTasks(provider);
    OHOS::RequestManagerFuzzTestTouchTasks(provider);
    OHOS::RequestManagerFuzzTestSetMaxSpeeds(provider);
    OHOS::RequestManagerFuzzTestSetMode(provider);
    OHOS::RequestManagerFuzzTestDisableTaskNotification(provider);
    OHOS::RequestManagerFuzzTestSetMaxSpeed(provider);
    OHOS::RequestManagerFuzzTestCreateGroup(provider);
    OHOS::RequestManagerFuzzTestAttachGroup(provider);
    OHOS::RequestManagerFuzzTestDeleteGroup(provider);
}

class FuzzRemoteObjectImpl : public OHOS::IRemoteObject {};

} // namespace OHOS

// @tc.name: ut_llvm_fuzzer_test_one_input
// @tc.desc: Fuzzer entry point function
// @tc.precon: NA
// @tc.step: 1. Call all fuzz test functions with the same input data
// 2. Functions include request management, response handling, and notification tests
// @tc.expect: Entry point should execute all fuzz tests without crashes
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 3
/* Fuzzer entry point */
extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size)
{
    /* Run your code on data */
    OHOS::CreateRequestFuzzTest(data, size);
    OHOS::StartRequestFuzzTest(data, size);
    OHOS::StopRequestFuzzTest(data, size);
    OHOS::ShowRequestFuzzTest(data, size);
    OHOS::SearchRequestFuzzTest(data, size);
    OHOS::PauseRequestFuzzTest(data, size);
    OHOS::RemoveRequestFuzzTest(data, size);
    OHOS::ResumeRequestFuzzTest(data, size);
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
    OHOS::RunCountNotifyStubFuzzTestGetInstanceDoneCallBack(data, size);
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
    FuzzedDataProvider provider(data, size);
    OHOS::RequestManagerFuzzTest(provider);
    OHOS::GetTaskRequestFuzzTest(provider);
    OHOS::QueryMimeTypeRequestFuzzTest(provider);
    OHOS::TouchRequestFuzzTest(provider);
    return 0;
}
