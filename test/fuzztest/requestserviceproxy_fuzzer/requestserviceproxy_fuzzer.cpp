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

using namespace OHOS::Request;

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

bool CreateTasksFuzzTest(FuzzedDataProvider &provider)
{
    std::vector<Config> configs = convertToVectorConfig(provider);
    std::vector<TaskRet> rets = convertToVectorTaskRet(provider);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->CreateTasks(configs, rets);

    return true;
}

bool StartTasksFuzzTest(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<ExceptionErrorCode> rets = convertToVectorExceptionErrorCode(provider);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->StartTasks(tids, rets);

    return true;
}

bool StopTasksFuzzTest(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<ExceptionErrorCode> rets = convertToVectorExceptionErrorCode(provider);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->StopTasks(tids, rets);

    return true;
}

bool ResumeTasksFuzzTest(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<ExceptionErrorCode> rets = convertToVectorExceptionErrorCode(provider);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
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
    proxy->RemoveTasks(tids, version, rets);

    return true;
}

bool DisableTaskNotificationFuzzTest(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<ExceptionErrorCode> rets = convertToVectorExceptionErrorCode(provider);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->DisableTaskNotification(tids, rets);

    return true;
}

bool StartFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->Start(tid);

    return true;
}

bool StopFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->Stop(tid);

    return true;
}

bool PauseFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    size_t versionIndex = provider.ConsumeIntegralInRange<size_t>(0, versions.size() - 1);
    Version version = versions[versionIndex];

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->Pause(tid, version);

    return true;
}

bool QueryMimeTypeFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    std::string mimeType = provider.ConsumeRandomLengthString(MAX_LENGTH);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->QueryMimeType(tid, mimeType);

    return true;
}

bool RemoveFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    size_t versionIndex = provider.ConsumeIntegralInRange<size_t>(0, versions.size() - 1);
    Version version = versions[versionIndex];

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->Remove(tid, version);

    return true;
}

bool ResumeFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->Resume(tid);

    return true;
}

bool SetMaxSpeedFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    int64_t maxSpeed = provider.ConsumeIntegral<int64_t>();

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->SetMaxSpeed(tid, maxSpeed);

    return true;
}

bool OpenChannelFuzzTest(FuzzedDataProvider &provider)
{
    int32_t sockFd = provider.ConsumeIntegral<int32_t>();

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->OpenChannel(sockFd);

    return true;
}

bool SubscribeFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->Subscribe(tid);

    return true;
}

bool UnSubscribeFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->Unsubscribe(tid);

    return true;
}

bool SubRunCountFuzzTest(FuzzedDataProvider &provider)
{
    sptr<NotifyInterface> listener = RunCountNotifyStub::GetInstance();

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->SubRunCount(listener);

    return true;
}

bool UnsubRunCountFuzzTest(FuzzedDataProvider &provider)
{
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->UnsubRunCount();

    return true;
}

bool AttachGroupFuzzTest(FuzzedDataProvider &provider)
{
    std::string gid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    std::vector<std::string> tids = convertToVectorString(provider);
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->AttachGroup(gid, tids);

    return true;
}

bool DeleteGroupFuzzTest(FuzzedDataProvider &provider)
{
    std::string gid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->DeleteGroup(gid);

    return true;
}

bool QueryTasksFuzzTest(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<TaskInfoRet> rets = convertToVectorTaskInfoRet(provider);
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->QueryTasks(tids, rets);

    return true;
}

bool ShowTasksFuzzTest(FuzzedDataProvider &provider)
{
    std::vector<std::string> tids = convertToVectorString(provider);
    std::vector<TaskInfoRet> rets = convertToVectorTaskInfoRet(provider);
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->ShowTasks(tids, rets);

    return true;
}

bool TouchTasksFuzzTest(FuzzedDataProvider &provider)
{
    std::vector<TaskIdAndToken> tids = convertToVectorTaskIdAndToken(provider);
    std::vector<TaskInfoRet> rets = convertToVectorTaskInfoRet(provider);
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
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
    proxy->Touch(tid, token, infoRet.info);

    return true;
}

bool SetModeFuzzTest(FuzzedDataProvider &provider)
{
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    size_t index = provider.ConsumeIntegralInRange<size_t>(0, modes.size() - 1);
    Mode mode = modes[index];

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
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
    proxy->Show(tid, infoRet.info);

    return true;
}

bool CreateGroupFuzzTest(FuzzedDataProvider &provider)
{
    std::string gid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    bool gauge = provider.ConsumeBool();
    Notification notification;

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->CreateGroup(gid, gauge, notification);

    return true;
}

bool CreateFuzzTest(FuzzedDataProvider &provider)
{
    Config config = convertToConfig(provider);
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->Create(config, tid);

    return true;
}

bool GetTaskFuzzTest(FuzzedDataProvider &provider)
{
    Config config = convertToConfig(provider);
    std::string tid = provider.ConsumeRandomLengthString(MAX_LENGTH);
    std::string token = provider.ConsumeRandomLengthString(MAX_LENGTH);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->GetTask(tid, token, config);

    return true;
}

bool SearchFuzzTest(FuzzedDataProvider &provider)
{
    Filter filter = convertToFilter(provider);
    std::vector<std::string> tids = convertToVectorString(provider);

    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    proxy->Search(filter, tids);

    return true;
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
    return 0;
}