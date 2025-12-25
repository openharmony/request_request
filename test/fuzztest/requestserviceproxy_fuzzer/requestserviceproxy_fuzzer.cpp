/*
 * Copyright (c) 2025 Huawei Device Co., Ltd.
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

#include "requestserviceproxy_fuzzer.h"
#include <fuzzer/FuzzedDataProvider.h>

#include <securec.h>

#include <cstddef>
#include <cstdint>
#include <vector>
#include <map>

#include "request_common.h"
#include "request_service_proxy.h"
#include "request_manager_impl.h"
#include "runcount_notify_stub.h"
#include "sys_event.h"
#include "utf8_utils.h"
#include "parcel_helper.h"
#include "request_manager.h"
#include "request_manager_impl.h"
#include "request_common.h"

using namespace OHOS::Request;

#define private public
#define protected public

#define SIZE_ONE 1
#define SIZE_TWO 2
#define SIZE_THREE 3
#define SIZE_FOUR 4
#define SIZE_FIVE 5
#define RETRY_TIMES 3

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

constexpr std::array<OHOS::Request::WaitingReason, 4> waitingReasons = {
    WaitingReason::TaskQueueFull,
    WaitingReason::NetworkNotMatch,
    WaitingReason::AppBackground,
    WaitingReason::UserInactivated,
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

bool RemoveTasksFuzzTest(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<ExceptionErrorCode> rets = convertToVectorExceptionErrorCode(provider);
    size_t versionIndex = provider.ConsumeIntegralInRange<size_t>(0, versions.size() - 1);
    Version version = versions[versionIndex];

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->RemoveTasks(tids, version, rets);

    return true;
}

bool DisableTaskNotificationFuzzTest(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<ExceptionErrorCode> rets = convertToVectorExceptionErrorCode(provider);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->DisableTaskNotification(tids, rets);

    return true;
}

bool StartFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->Start(tid);

    return true;
}

bool StopFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->Stop(tid);

    return true;
}

bool PauseFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    size_t versionIndex = provider.ConsumeIntegralInRange<size_t>(0, versions.size() - 1);
    Version version = versions[versionIndex];

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->Pause(tid, version);

    return true;
}

bool QueryMimeTypeFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    std::string mimeType = provider.ConsumeRandomLengthString(MAX_LENGTH);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->QueryMimeType(tid, mimeType);

    return true;
}

bool RemoveFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    size_t versionIndex = provider.ConsumeIntegralInRange<size_t>(0, versions.size() - 1);
    Version version = versions[versionIndex];

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->Remove(tid, version);

    return true;
}

bool ResumeFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->Resume(tid);

    return true;
}

bool SetMaxSpeedFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    int64_t maxSpeed = provider.ConsumeIntegral<int64_t>();

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->SetMaxSpeed(tid, maxSpeed);

    return true;
}

bool OpenChannelFuzzTest(FuzzedDataProvider &provider)
{
    int32_t sockFd = provider.ConsumeIntegral<int32_t>();

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->OpenChannel(sockFd);

    return true;
}

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

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->SubRunCount(listener);

    return true;
}

bool UnsubRunCountFuzzTest(FuzzedDataProvider &provider)
{
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
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

bool TouchFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    std::string token = provider.ConsumeRandomLengthString(MAX_LENGTH);
    size_t index = provider.ConsumeIntegralInRange<size_t>(0, exceptionErrorCodes.size() - 1);
    ExceptionErrorCode code = exceptionErrorCodes[index];
    TaskInfoRet infoRet{ .code = code };

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->Touch(tid, token, infoRet.info);

    return true;
}

bool SetModeFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    size_t index = provider.ConsumeIntegralInRange<size_t>(0, modes.size() - 1);
    Mode mode = modes[index];

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->SetMode(tid, mode);

    return true;
}

bool ShowFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    size_t index = provider.ConsumeIntegralInRange<size_t>(0, exceptionErrorCodes.size() - 1);
    ExceptionErrorCode code = exceptionErrorCodes[index];
    TaskInfoRet infoRet{ .code = code };

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->Show(tid, infoRet.info);

    return true;
}

bool CreateGroupFuzzTest(FuzzedDataProvider &provider)
{
    std::string gid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    bool gauge = provider.ConsumeBool();
    Notification notification;

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->CreateGroup(gid, gauge, notification);

    return true;
}

bool CreateFuzzTest(FuzzedDataProvider &provider)
{
    Config config = convertToConfig(provider);
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        return true;
    }
    proxy->Create(config, tid);

    return true;
}

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
    auto iter = Request::SysEventLog::sysEventMap_.find("EXEC_ERROR");
    if (iter == Request::SysEventLog::sysEventMap_.end()) {
        return true;
    }
    iter = Request::SysEventLog::sysEventMap_.find("EXEC_FAULT");
    if (iter == Request::SysEventLog::sysEventMap_.end()) {
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

bool Utf8UtilsFuzzTestGetNextByte(FuzzedDataProvider &provider)
{
    std::vector<uint8_t> num = convertToVectorUint8_t(provider);
    size_t size = num.size();
    if (size < SIZE_ONE) {
        return true;
    }
    Utf8Utils::RunUtf8Validation(num);
    if (size < SIZE_TWO) {
        return true;
    }
    num[0] = 0x81;
    Utf8Utils::RunUtf8Validation(num);
    if (size < SIZE_THREE) {
        return true;
    }
    num[0] = 0xC2;
    num[1] = 0xA9;
    Utf8Utils::RunUtf8Validation(num);
    if (size < SIZE_FOUR) {
        return true;
    }
    num[0] = 0xE2;
    num[1] = 0x82;
    num[SIZE_TWO] = 0xAC;
    Utf8Utils::RunUtf8Validation(num);
    if (size < SIZE_FIVE) {
        return true;
    }
    num[0] = 0xF0;
    num[1] = 0x9F;
    num[SIZE_TWO] = 0x98;
    num[SIZE_THREE] = 0x80;
    Utf8Utils::RunUtf8Validation(num);
    num[0] = 0x80;
    Utf8Utils::RunUtf8Validation(num);
    num[0] = 0xC0;
    num[1] = 0x80;
    Utf8Utils::RunUtf8Validation(num);
    return true;
}

void MarshalConfigBase(OHOS::MessageParcel &data)
{
    Config config;
    data.WriteUint32(static_cast<uint32_t>(config.action));
    data.WriteUint32(static_cast<uint32_t>(config.mode));
    data.WriteUint32(config.bundleType);
    data.WriteBool(config.overwrite);
    data.WriteUint32(static_cast<uint32_t>(config.network));
    config.metered = data.WriteBool(config.metered);
    data.WriteBool(config.roaming);
    data.WriteBool(config.retry);
    data.WriteBool(config.redirect);
    data.WriteUint32(config.index);
    data.WriteInt64(config.begins);
    data.WriteInt64(config.ends);
    data.WriteBool(config.gauge);
    data.WriteBool(config.precise);
    data.WriteUint32(config.priority);
    data.WriteBool(config.background);
    data.WriteBool(config.multipart);
    data.WriteString("bundleName");
    data.WriteString("url");
    data.WriteString("title");
    data.WriteString("description");
    data.WriteString("method");
}

bool ParcelHelperFuzzTestUnMarshalConfig(FuzzedDataProvider &provider)
{
    std::vector<std::string> string = convertToVectorString(provider);
    std::vector<uint8_t> num = convertToVectorUint8_t(provider);
    Config config;
    OHOS::MessageParcel data;
    MarshalConfigBase(data);
    data.WriteUint32(num[0]);
    data.WriteString(string[0]);
    ParcelHelper::UnMarshalConfig(data, config);
    ParcelHelper::UnMarshalConfigHeaders(data, config);
    ParcelHelper::UnMarshalConfigHeaders(data, config);
    ParcelHelper::UnMarshalConfigExtras(data, config);
    ParcelHelper::UnMarshalConfigFormItem(data, config);
    ParcelHelper::UnMarshalConfigFileSpec(data, config);
    ParcelHelper::UnMarshalConfigBodyFileName(data, config);
    return true;
}

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

void RequestManagerImplFuzzTestTouchTasks(FuzzedDataProvider &provider)
{
    std::vector<TaskIdAndToken> taskid = convertToVectorTaskIdAndToken(provider);
    std::vector<TaskInfoRet> taskinfo = convertToVectorTaskInfoRet(provider);
    RequestManagerImpl::GetInstance()->TouchTasks(taskid, taskinfo);
}

void RequestManagerImplFuzzTestSetMaxSpeeds(FuzzedDataProvider &provider)
{
    std::vector<SpeedConfig> speedconfig = convertToVectorSpeedConfig(provider);
    std::vector<ExceptionErrorCode> err = convertToVectorExceptionErrorCode(provider);
    RequestManagerImpl::GetInstance()->SetMaxSpeeds(speedconfig, err);
}

void RequestManagerImplFuzzTestCreateGroup(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    bool gauge = provider.ConsumeBool();
    Notification notification;
    RequestManagerImpl::GetInstance()->CreateGroup(tid, gauge, notification);
}

void RequestManagerImplFuzzTestAttachGroup(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    std::vector<std::string> tids = convertToVectorString(provider);
    RequestManagerImpl::GetInstance()->AttachGroup(tid, tids);
}

void RequestManagerImplFuzzTestDeleteGroup(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    RequestManagerImpl::GetInstance()->DeleteGroup(tid);
}

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
    auto respptr = std::make_shared<Response>(resp);
    RequestManagerImpl::GetInstance()->OnResponseReceive(respptr);
}

void RequestManagerImplFuzzTestOnNotifyDataReceive(FuzzedDataProvider &provider)
{
    NotifyData notifyData;
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

void RequestManagerImplFuzzTestOnRemoveSystemAbility(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    int32_t num32 = provider.ConsumeIntegral<int32_t>();
    RequestManagerImpl::GetInstance()->LoadRequestServer();
    RequestManagerImpl::SystemAbilityStatusChangeListener listener =
        RequestManagerImpl::SystemAbilityStatusChangeListener();
    listener.OnRemoveSystemAbility(num32, tid);
}

void RequestManagerImplFuzzTest(FuzzedDataProvider &provider)
{
    OHOS::RequestManagerImplFuzzTestSetMode(provider);
    OHOS::RequestManagerImplFuzzTestDisableTaskNotification(provider);
    OHOS::RequestManagerImplFuzzTestCreateTasks(provider);
    OHOS::RequestManagerImplFuzzTestStartTasks(provider);
    OHOS::RequestManagerImplFuzzTestStopTasks(provider);
    OHOS::RequestManagerImplFuzzTestResumeTasks(provider);
    OHOS::RequestManagerImplFuzzTestRemoveTasks(provider);
    OHOS::RequestManagerImplFuzzTestPauseTasks(provider);
    OHOS::RequestManagerImplFuzzTestQueryTasks(provider);
    OHOS::RequestManagerImplFuzzTestShowTasks(provider);
    OHOS::RequestManagerImplFuzzTestTouchTasks(provider);
    OHOS::RequestManagerImplFuzzTestSetMaxSpeeds(provider);
    OHOS::RequestManagerImplFuzzTestCreateGroup(provider);
    OHOS::RequestManagerImplFuzzTestAttachGroup(provider);
    OHOS::RequestManagerImplFuzzTestDeleteGroup(provider);
    OHOS::RequestManagerImplFuzzTestSetMaxSpeed(provider);
    OHOS::RequestManagerImplFuzzTestOnResponseReceive(provider);
    OHOS::RequestManagerImplFuzzTestOnNotifyDataReceive(provider);
    OHOS::RequestManagerImplFuzzTestOnFaultsReceive(provider);
    OHOS::RequestManagerImplFuzzTestOnWaitReceive(provider);
    OHOS::RequestManagerImplFuzzTestOnRemoveSystemAbility(provider);
}
}

extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size)
{
    // /* Run your code on data */
    FuzzedDataProvider provider(data, size);
    OHOS::CreateTasksFuzzTest(provider);
    OHOS::StartTasksFuzzTest(provider);
    OHOS::StopTasksFuzzTest(provider);
    OHOS::ResumeTasksFuzzTest(provider);
    OHOS::PauseTasksFuzzTest(provider);
    OHOS::RemoveTasksFuzzTest(provider);
    OHOS::DisableTaskNotificationFuzzTest(provider);
    OHOS::StartFuzzTest(provider);
    OHOS::StopFuzzTest(provider);
    OHOS::PauseFuzzTest(provider);
    OHOS::QueryMimeTypeFuzzTest(provider);
    OHOS::RemoveFuzzTest(provider);
    OHOS::ResumeFuzzTest(provider);
    OHOS::SetMaxSpeedFuzzTest(provider);
    OHOS::OpenChannelFuzzTest(provider);
    OHOS::SubscribeFuzzTest(provider);
    OHOS::UnSubscribeFuzzTest(provider);
    OHOS::SubRunCountFuzzTest(provider);
    OHOS::UnsubRunCountFuzzTest(provider);
    OHOS::AttachGroupFuzzTest(provider);
    OHOS::DeleteGroupFuzzTest(provider);
    OHOS::QueryTasksFuzzTest(provider);
    OHOS::ShowTasksFuzzTest(provider);
    OHOS::TouchTasksFuzzTest(provider);
    OHOS::QueryFuzzTest(provider);
    OHOS::TouchFuzzTest(provider);
    OHOS::SetModeFuzzTest(provider);
    OHOS::ShowFuzzTest(provider);
    OHOS::CreateGroupFuzzTest(provider);
    OHOS::CreateFuzzTest(provider);
    OHOS::GetTaskFuzzTest(provider);
    OHOS::SearchFuzzTest(provider);
    OHOS::SysEventFuzzTestSendSysEventLog(provider);
    OHOS::SysEventFuzzTestSendStatisticEvent(provider);
    OHOS::Utf8UtilsFuzzTestGetNextByte(provider);
    OHOS::ParcelHelperFuzzTestUnMarshalConfig(provider);
    OHOS::RequestManagerImplFuzzTest(provider);
    return 0;
}