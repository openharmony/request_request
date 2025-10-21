/*
 * Copyright (c) 2024 Huawei Device Co., Ltd.
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

#include "response_message_receiver.h"

#include <unistd.h>

#include <cstdint>
#include <cstdlib>
#include <sstream>
#include <string>
#include <vector>

#include "log.h"
#include "request_common.h"
#include "sys_event.h"

namespace OHOS::Request {

static constexpr int32_t INT64_SIZE = 8;
static constexpr int32_t INT32_SIZE = 4;
static constexpr int32_t INT16_SIZE = 2;
// static constexpr int32_t INT8_SIZE = 1;

std::shared_ptr<OHOS::AppExecFwk::EventHandler> serviceHandler_;

int32_t ResponseMessageReceiver::Int64FromParcel(int64_t &num, char *&parcel, int32_t &size)
{
    if (size < INT64_SIZE) {
        REQUEST_HILOGE("message not complete");
        return -1;
    }
    num = *reinterpret_cast<int64_t *>(parcel);
    parcel += INT64_SIZE;
    size -= INT64_SIZE;
    return 0;
}

int32_t ResponseMessageReceiver::Uint64FromParcel(uint64_t &num, char *&parcel, int32_t &size)
{
    if (size < INT64_SIZE) {
        REQUEST_HILOGE("message not complete");
        return -1;
    }
    num = *reinterpret_cast<uint64_t *>(parcel);
    parcel += INT64_SIZE;
    size -= INT64_SIZE;
    return 0;
}

int32_t ResponseMessageReceiver::Int32FromParcel(int32_t &num, char *&parcel, int32_t &size)
{
    if (size < INT32_SIZE) {
        REQUEST_HILOGE("message not complete");
        return -1;
    }
    num = *reinterpret_cast<int32_t *>(parcel);
    parcel += INT32_SIZE;
    size -= INT32_SIZE;
    return 0;
}

int32_t ResponseMessageReceiver::Uint32FromParcel(uint32_t &num, char *&parcel, int32_t &size)
{
    if (size < INT32_SIZE) {
        REQUEST_HILOGE("message not complete");
        return -1;
    }
    num = *reinterpret_cast<uint32_t *>(parcel);
    parcel += INT32_SIZE;
    size -= INT32_SIZE;
    return 0;
}

int32_t ResponseMessageReceiver::Int16FromParcel(int16_t &num, char *&parcel, int32_t &size)
{
    if (size < INT16_SIZE) {
        REQUEST_HILOGE("message not complete");
        return -1;
    }
    num = *reinterpret_cast<int16_t *>(parcel);
    parcel += INT16_SIZE;
    size -= INT16_SIZE;
    return 0;
}

int32_t ResponseMessageReceiver::StateFromParcel(State &state, char *&parcel, int32_t &size)
{
    uint32_t temp;
    if (Uint32FromParcel(temp, parcel, size) || temp > static_cast<uint32_t>(State::ANY)) {
        return -1;
    }
    state = static_cast<State>(temp);
    return 0;
}

int32_t ResponseMessageReceiver::ActionFromParcel(Action &action, char *&parcel, int32_t &size)
{
    uint32_t temp;
    if (Uint32FromParcel(temp, parcel, size) || temp > static_cast<uint32_t>(Action::ANY)) {
        return -1;
    }
    action = static_cast<Action>(temp);
    return 0;
}

int32_t ResponseMessageReceiver::VersionFromParcel(Version &version, char *&parcel, int32_t &size)
{
    uint32_t temp;
    if (Uint32FromParcel(temp, parcel, size) || temp > static_cast<uint32_t>(Version::API10)) {
        return -1;
    }
    version = static_cast<Version>(temp);
    return 0;
}

int32_t ResponseMessageReceiver::SubscribeTypeFromParcel(SubscribeType &type, char *&parcel, int32_t &size)
{
    uint32_t temp;
    if (Uint32FromParcel(temp, parcel, size) || temp > static_cast<uint32_t>(SubscribeType::BUTT)) {
        return -1;
    }
    type = static_cast<SubscribeType>(temp);
    return 0;
}

int32_t ResponseMessageReceiver::ReasonFromParcel(Reason &reason, char *&parcel, int32_t &size)
{
    uint32_t temp;
    if (Uint32FromParcel(temp, parcel, size)) {
        return -1;
    }
    reason = static_cast<Reason>(temp);
    return 0;
}

int32_t ResponseMessageReceiver::StringFromParcel(std::string &str, char *&parcel, int32_t &size)
{
    int32_t i = 0;

    while (i < size && parcel[i] != '\0') {
        ++i;
    }

    if (i < size) {
        str.assign(parcel, i);
        parcel += (i + 1);
        size -= (i + 1);
        return 0;
    } else {
        REQUEST_HILOGE("message not complete");
        return -1;
    }
}

int32_t ResponseMessageReceiver::ResponseHeaderFromParcel(
    std::map<std::string, std::vector<std::string>> &headers, char *&parcel, int32_t &size)
{
    std::string s(parcel, size);
    std::stringstream ss(s);
    std::string line;
    while (std::getline(ss, line, '\n')) {
        std::stringstream keyValue(line);
        std::string key;
        std::string valueLine;
        std::getline(keyValue, key, ':');
        std::getline(keyValue, valueLine);
        std::stringstream values(valueLine);
        std::string value;
        while (getline(values, value, ',')) {
            headers[key].push_back(value);
        }
    }
    return 0;
}

int32_t ResponseMessageReceiver::ProgressExtrasFromParcel(
    std::map<std::string, std::string> &extras, char *&parcel, int32_t &size)
{
    uint32_t length;
    if (Uint32FromParcel(length, parcel, size)) {
        return -1;
    }

    for (uint32_t i = 0; i < length; ++i) {
        std::string key;
        std::string value;
        if (StringFromParcel(key, parcel, size) != 0) {
            return -1;
        }
        if (StringFromParcel(value, parcel, size) != 0) {
            return -1;
        }
        extras[key] = value;
    }

    return 0;
}

int32_t ResponseMessageReceiver::VecInt64FromParcel(std::vector<int64_t> &vec, char *&parcel, int32_t &size)
{
    uint32_t length;
    if (Uint32FromParcel(length, parcel, size)) {
        return -1;
    }
    for (uint32_t i = 0; i < length; ++i) {
        int64_t value;
        if (Int64FromParcel(value, parcel, size)) {
            return -1;
        }
        vec.push_back(value);
    }

    return 0;
}

ResponseMessageReceiver::ResponseMessageReceiver(IResponseMessageHandler *handler, int32_t sockFd)
    : handler_(handler), sockFd_(sockFd)
{
}

void ResponseMessageReceiver::BeginReceive()
{
    std::shared_ptr<OHOS::AppExecFwk::EventRunner> runner = OHOS::AppExecFwk::EventRunner::GetMainEventRunner();
    if (!runner) {
        SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_10, "GetMainEventRunner failed");
    }
    serviceHandler_ = std::make_shared<OHOS::AppExecFwk::EventHandler>(runner);
    {
        std::lock_guard<std::mutex> lock(sockFdMutex_);
        auto err = serviceHandler_->AddFileDescriptorListener(
            sockFd_, OHOS::AppExecFwk::FILE_DESCRIPTOR_INPUT_EVENT, shared_from_this(), "subscribe");
        if (err != ERR_OK) {
            REQUEST_HILOGE("handler addlisterner err: %{public}d", err);
            SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_11, "handler addlisterner err");
        }
    }
}

// ret 0 if success, ret < 0 if fail
int32_t ResponseMessageReceiver::MsgHeaderParcel(
    int32_t &msgId, int16_t &msgType, int16_t &bodySize, char *&parcel, int32_t &size)
{
    int32_t magicNum = 0;
    if (Int32FromParcel(magicNum, parcel, size) != 0) {
        return -1;
    }
    if (magicNum != ResponseMessageReceiver::RESPONSE_MAGIC_NUM) {
        REQUEST_HILOGE("Bad magic num, %{public}d", magicNum);
        return -1;
    }

    if (Int32FromParcel(msgId, parcel, size) != 0) {
        return -1;
    }
    if (Int16FromParcel(msgType, parcel, size) != 0) {
        return -1;
    }
    if (Int16FromParcel(bodySize, parcel, size) != 0) {
        return -1;
    }
    return 0;
}

int32_t ResponseMessageReceiver::ReasonDataFromParcel(std::shared_ptr<int32_t> &tid,
    std::shared_ptr<SubscribeType> &type, std::shared_ptr<Reason> &reason, char *&parcel, int32_t &size)
{
    if (Int32FromParcel(*tid, parcel, size) != 0) {
        REQUEST_HILOGE("Bad tid");
        return -1;
    }

    if (SubscribeTypeFromParcel(*type, parcel, size) != 0) {
        REQUEST_HILOGE("Bad Subscribe Type");
        return -1;
    }

    if (ReasonFromParcel(*reason, parcel, size) != 0) {
        REQUEST_HILOGE("Bad reason");
        return -1;
    }
    return 0;
}

int32_t ResponseMessageReceiver::ResponseFromParcel(std::shared_ptr<Response> &response, char *&parcel, int32_t &size)
{
    int32_t tid;
    if (Int32FromParcel(tid, parcel, size) != 0) {
        REQUEST_HILOGE("Bad tid");
        return -1;
    }
    response->taskId = std::to_string(tid);

    if (StringFromParcel(response->version, parcel, size) != 0) {
        REQUEST_HILOGE("Bad version");
        return -1;
    }

    if (Int32FromParcel(response->statusCode, parcel, size) != 0) {
        REQUEST_HILOGE("Bad statusCode");
        return -1;
    }

    if (StringFromParcel(response->reason, parcel, size) != 0) {
        REQUEST_HILOGE("Bad reason");
        return -1;
    }

    ResponseHeaderFromParcel(response->headers, parcel, size);
    return 0;
}

int32_t ResponseMessageReceiver::TaskStatesFromParcel(std::vector<TaskState> &taskStates, char *&parcel, int32_t &size)
{
    uint32_t length;
    if (Uint32FromParcel(length, parcel, size) != 0) {
        REQUEST_HILOGE("Bad type");
        return -1;
    }
    for (uint32_t i = 0; i < length; ++i) {
        TaskState taskState;
        if (StringFromParcel(taskState.path, parcel, size) != 0) {
            REQUEST_HILOGE("Bad path");
            return -1;
        }
        if (Uint32FromParcel(taskState.responseCode, parcel, size) != 0) {
            REQUEST_HILOGE("Bad responseCode");
            return -1;
        }
        if (StringFromParcel(taskState.message, parcel, size) != 0) {
            REQUEST_HILOGE("Bad message");
            return -1;
        }
        taskStates.push_back(taskState);
    }
    return 0;
}

int32_t ResponseMessageReceiver::NotifyDataFromParcel(
    std::shared_ptr<NotifyData> &notifyData, char *&parcel, int32_t &size)
{
    if (SubscribeTypeFromParcel(notifyData->type, parcel, size) != 0) {
        REQUEST_HILOGE("Bad type");
        return -1;
    }
    if (Uint32FromParcel(notifyData->taskId, parcel, size) != 0) {
        REQUEST_HILOGE("Bad tid");
        return -1;
    }
    if (StateFromParcel(notifyData->progress.state, parcel, size) != 0) {
        REQUEST_HILOGE("Bad state");
        return -1;
    }
    if (Uint32FromParcel(notifyData->progress.index, parcel, size) != 0) {
        REQUEST_HILOGE("Bad index");
        return -1;
    }
    if (Uint64FromParcel(notifyData->progress.processed, parcel, size) != 0) {
        REQUEST_HILOGE("Bad processed");
        return -1;
    }
    if (Uint64FromParcel(notifyData->progress.totalProcessed, parcel, size) != 0) {
        REQUEST_HILOGE("Bad totalProcessed");
        return -1;
    }
    if (VecInt64FromParcel(notifyData->progress.sizes, parcel, size) != 0) {
        REQUEST_HILOGE("Bad sizes");
        return -1;
    }
    if (ProgressExtrasFromParcel(notifyData->progress.extras, parcel, size) != 0) {
        REQUEST_HILOGE("Bad extras");
        return -1;
    }

    if (ActionFromParcel(notifyData->action, parcel, size) != 0) {
        REQUEST_HILOGE("Bad action");
        return -1;
    }
    if (VersionFromParcel(notifyData->version, parcel, size) != 0) {
        REQUEST_HILOGE("Bad version");
        return -1;
    }
    if (TaskStatesFromParcel(notifyData->taskStates, parcel, size) != 0) {
        REQUEST_HILOGE("Bad taskStates");
        return -1;
    }
    return 0;
}

bool ResponseMessageReceiver::ReadUdsData(char *buffer, int32_t readSize, int32_t &length)
{
    std::lock_guard<std::mutex> lock(sockFdMutex_);
    if (sockFd_ < 0) {
        REQUEST_HILOGE("OnReadable errfd: %{public}d", sockFd_);
        return false;
    }
    length = read(sockFd_, buffer, readSize);
    if (length <= 0) {
        REQUEST_HILOGE("read message error: %{public}d, %{public}d", length, errno);
        return false;
    }
    REQUEST_HILOGD("read message: %{public}d", length);

    char lenBuf[4];
    *reinterpret_cast<uint32_t *>(lenBuf) = length;
    int32_t ret = write(sockFd_, lenBuf, 4);
    if (ret <= 0) {
        REQUEST_HILOGE("send length back failed: %{public}d, %{public}d", ret, errno);
        SysEventLog::SendSysEventLog(FAULT_EVENT, UDS_FAULT_02, "write" + std::to_string(ret));
    }
    return true;
}

void ResponseMessageReceiver::OnReadable(int32_t fd)
{
    int readSize = ResponseMessageReceiver::RESPONSE_MAX_SIZE;
    char buffer[readSize];
    int32_t length = 0;
    if (!ReadUdsData(buffer, readSize, length)) {
        std::lock_guard<std::mutex> lock(sockFdMutex_);
        REQUEST_HILOGE("ReadUdsData err: %{public}d,%{public}d, %{public}d", sockFd_, fd, length);
        return;
    };

    char *leftBuf = buffer;
    int32_t leftLen = length;
    int32_t msgId = -1;
    int16_t msgType = -1;
    int16_t headerSize = -1;
    MsgHeaderParcel(msgId, msgType, headerSize, leftBuf, leftLen);
    if (msgId != messageId_) {
        REQUEST_HILOGE("Bad messageId, expect %{public}d = %{public}d", msgId, messageId_);
    }
    if (headerSize != static_cast<int16_t>(length)) {
        REQUEST_HILOGE("Bad headerSize, %{public}d, %{public}d", length, headerSize);
    }
    ++messageId_;

    if (msgType == MessageType::HTTP_RESPONSE) {
        HandResponseData(leftBuf, leftLen);
    } else if (msgType == MessageType::NOTIFY_DATA) {
        HandNotifyData(leftBuf, leftLen);
    } else if (msgType == MessageType::FAULTS) {
        HandFaultsData(leftBuf, leftLen);
    } else if (msgType == MessageType::WAIT) {
        HandWaitData(leftBuf, leftLen);
    }
}

void ResponseMessageReceiver::HandResponseData(char *&leftBuf, int32_t &leftLen)
{
    std::shared_ptr<Response> response = std::make_shared<Response>();
    if (ResponseFromParcel(response, leftBuf, leftLen) == 0) {
        this->handler_->OnResponseReceive(response);
    } else {
        REQUEST_HILOGE("Bad Response");
        SysEventLog::SendSysEventLog(FAULT_EVENT, UDS_FAULT_01, "Bad Response");
    }
}

void ResponseMessageReceiver::HandNotifyData(char *&leftBuf, int32_t &leftLen)
{
    std::shared_ptr<NotifyData> notifyData = std::make_shared<NotifyData>();
    if (NotifyDataFromParcel(notifyData, leftBuf, leftLen) == 0) {
        this->handler_->OnNotifyDataReceive(notifyData);
    } else {
        REQUEST_HILOGE("Bad NotifyData");
        SysEventLog::SendSysEventLog(FAULT_EVENT, UDS_FAULT_01, "Bad NotifyData");
    }
}

void ResponseMessageReceiver::HandFaultsData(char *&leftBuf, int32_t &leftLen)
{
    std::shared_ptr<int32_t> tid = std::make_shared<int32_t>();
    std::shared_ptr<Reason> reason = std::make_shared<Reason>();
    std::shared_ptr<SubscribeType> type = std::make_shared<SubscribeType>();
    if (ReasonDataFromParcel(tid, type, reason, leftBuf, leftLen) == 0) {
        this->handler_->OnFaultsReceive(tid, type, reason);
    } else {
        REQUEST_HILOGE("Bad faults");
        SysEventLog::SendSysEventLog(FAULT_EVENT, UDS_FAULT_01, "Bad faults");
    }
}

void ResponseMessageReceiver::HandWaitData(char *&leftBuf, int32_t &leftLen)
{
    int32_t taskId;
    if (Int32FromParcel(taskId, leftBuf, leftLen) != 0) {
        REQUEST_HILOGE("Bad taskId");
        return;
    }
    uint32_t reason;
    if (Uint32FromParcel(reason, leftBuf, leftLen) != 0) {
        REQUEST_HILOGE("Bad reason");
        return;
    }
    this->handler_->OnWaitReceive(taskId, static_cast<WaitingReason>(reason));
}

void ResponseMessageReceiver::OnShutdown(int32_t fd)
{
    ShutdownChannel();
}

void ResponseMessageReceiver::OnException(int32_t fd)
{
    ShutdownChannel();
}

void ResponseMessageReceiver::Shutdown()
{
    ShutdownChannel();
}

void ResponseMessageReceiver::ShutdownChannel()
{
    {
        std::lock_guard<std::mutex> lock(sockFdMutex_);
        REQUEST_HILOGI("uds ShutdownChannel, %{public}d", sockFd_);
        if (sockFd_ > 0) {
            serviceHandler_->RemoveFileDescriptorListener(sockFd_);
            fdsan_close_with_tag(sockFd_, REQUEST_FDSAN_TAG);
        }
        sockFd_ = -1;
    }
    this->handler_->OnChannelBroken();
}

} // namespace OHOS::Request