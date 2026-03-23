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

#ifndef TEST_FUZZTEST_REQUESTSERVICEPROXY_FUZZER_COMMON_H
#define TEST_FUZZTEST_REQUESTSERVICEPROXY_FUZZER_COMMON_H

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

using namespace Request;

constexpr std::array<ExceptionErrorCode, 19> exceptionErrorCodes = {
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

constexpr std::array<Version, 3> versions = {
    Version::API8,
    Version::API9,
    Version::API10,
};

constexpr std::array<Mode, 3> modes = {
    Mode::BACKGROUND,
    Mode::FOREGROUND,
    Mode::ANY,
};

constexpr std::array<WaitingReason, 4> waitingReasons = {
    WaitingReason::TaskQueueFull,
    WaitingReason::NetworkNotMatch,
    WaitingReason::AppBackground,
    WaitingReason::UserInactivated,
};

constexpr std::array<SubscribeType, 10> subscribeTypes = {
    SubscribeType::COMPLETED,
    SubscribeType::FAILED,
    SubscribeType::HEADER_RECEIVE,
    SubscribeType::PAUSE,
    SubscribeType::PROGRESS,
    SubscribeType::REMOVE,
    SubscribeType::RESUME,
    SubscribeType::RESPONSE,
    SubscribeType::FAULT_OCCUR,
    SubscribeType::WAIT,
};

constexpr std::array<State, 11> states = {
    State::INITIALIZED,
    State::WAITING,
    State::RUNNING,
    State::RETRYING,
    State::PAUSED,
    State::STOPPED,
    State::COMPLETED,
    State::FAILED,
    State::REMOVED,
    State::DEFAULT,
    State::ANY,
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

} // namespace OHOS

#endif // TEST_FUZZTEST_REQUESTSERVICEPROXYFUZZER_COMMON_H
