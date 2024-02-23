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

#include <stdlib.h>
#include <unistd.h>

#include <sstream>
#include <string>
#include <vector>

#include "log.h"

namespace OHOS::Request {

static constexpr int32_t INT32_SIZE = 4;
static constexpr int32_t INT16_SIZE = 2;

std::shared_ptr<OHOS::AppExecFwk::EventHandler> serviceHandler_;

// retval == 0 means success, < 0 means failed
static int32_t Int32FromParcel(int32_t &num, char *&parcel, int32_t &size)
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

static int16_t Int16FromParcel(int16_t &num, char *&parcel, int32_t &size)
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

static int32_t StringFromParcel(std::string &str, char *&parcel, int32_t &size)
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

static int32_t ResponseHeaderFromParcel(
    std::map<std::string, std::vector<std::string>> &headers, char *&parcel, int32_t &size)
{
    std::string s(parcel, size);
    std::stringstream ss(s);
    std::string line;
    while (std::getline(ss, line, '\n')) {
        std::stringstream keyValue(line);
        std::string key, valueLine;
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

ResponseMessageReceiver::ResponseMessageReceiver(IResponseMessageHandler *handler, int32_t sockFd)
    : handler_(handler), sockFd_(sockFd)
{
}

void ResponseMessageReceiver::BeginReceive()
{
    std::shared_ptr<OHOS::AppExecFwk::EventRunner> runner = OHOS::AppExecFwk::EventRunner::GetMainEventRunner();
    serviceHandler_ = std::make_shared<OHOS::AppExecFwk::EventHandler>(runner);
    serviceHandler_->AddFileDescriptorListener(
        sockFd_, OHOS::AppExecFwk::FILE_DESCRIPTOR_INPUT_EVENT, shared_from_this(), "subscribe");
}

// ret 0 if success, ret < 0 if fail
static int32_t MsgHeaderParcel(int32_t &msgId, int16_t &msgType, int16_t &bodySize, char *&parcel, int32_t &size)
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

static int32_t MsgFromParcel(std::shared_ptr<Response> &response, char *&parcel, int32_t &size)
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

    if (ResponseHeaderFromParcel(response->headers, parcel, size) != 0) {
        REQUEST_HILOGE("Bad headers");
        return -1;
    }
    return 0;
}

void ResponseMessageReceiver::OnReadable(int32_t fd)
{
    int32_t msgId;
    int16_t msgType;
    int16_t headerSize;
    std::shared_ptr<Response> response = std::make_shared<Response>();
    int readSize = ResponseMessageReceiver::RESPONSE_MAX_SIZE;
    char buffer[readSize];

    int32_t length = read(fd, buffer, readSize);
    if (length <= 0) {
        return;
    }

    REQUEST_HILOGD("read response: %{public}d", length);

    char *leftBuf = buffer;
    int32_t leftLen = length;
    MsgHeaderParcel(msgId, msgType, headerSize, leftBuf, leftLen);
    if (msgId != messageId_) {
        REQUEST_HILOGE("Bad messageId");
        return;
    }
    if (headerSize != length) {
        REQUEST_HILOGE("Bad headerSize, %{public}d, %{public}d", length, headerSize);
    }
    ++messageId_;

    if (MsgFromParcel(response, leftBuf, leftLen) == 0) {
        this->handler_->OnResponseReceive(response);
    }
}

void ResponseMessageReceiver::OnShutdown(int32_t fd)
{
    serviceHandler_->RemoveFileDescriptorListener(fd);
    close(fd);
    this->handler_->OnChannelBroken();
}

void ResponseMessageReceiver::OnException(int32_t fd)
{
    serviceHandler_->RemoveFileDescriptorListener(fd);
    close(fd);
    this->handler_->OnChannelBroken();
}

void ResponseMessageReceiver::Shutdown()
{
    serviceHandler_->RemoveFileDescriptorListener(sockFd_);
    close(sockFd_);
    this->handler_->OnChannelBroken();
}

} // namespace OHOS::Request