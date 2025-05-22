/*
 * Copyright (C) 2024 Huawei Device Co., Ltd.
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

#ifndef OHOS_REQUEST_RESPONSE_MESSAGE_RECEIVER_H
#define OHOS_REQUEST_RESPONSE_MESSAGE_RECEIVER_H

#include "event_handler.h"
#include "event_runner.h"
#include "i_response_message_handler.h"

namespace OHOS::Request {

enum MessageType {
    HTTP_RESPONSE = 0,
    NOTIFY_DATA,
    FAULTS,
    WAIT,
};

class ResponseMessageReceiver
    : public OHOS::AppExecFwk::FileDescriptorListener
    , public std::enable_shared_from_this<ResponseMessageReceiver> {
public:
    static constexpr uint32_t RESPONSE_MAX_SIZE = 16 * 1024;
    static constexpr uint32_t RESPONSE_MAGIC_NUM = 0x43434646;

    ResponseMessageReceiver(IResponseMessageHandler *handler, int32_t sockFd);
    void BeginReceive();
    void Shutdown(void);

private:
    static int32_t Int64FromParcel(int64_t &num, char *&parcel, int32_t &size);
    static int32_t Uint64FromParcel(uint64_t &num, char *&parcel, int32_t &size);
    static int32_t Int32FromParcel(int32_t &num, char *&parcel, int32_t &size);
    static int32_t Uint32FromParcel(uint32_t &num, char *&parcel, int32_t &size);
    static int32_t Int16FromParcel(int16_t &num, char *&parcel, int32_t &size);
    static int32_t StateFromParcel(State &state, char *&parcel, int32_t &size);
    static int32_t ActionFromParcel(Action &action, char *&parcel, int32_t &size);
    static int32_t VersionFromParcel(Version &version, char *&parcel, int32_t &size);
    static int32_t SubscribeTypeFromParcel(SubscribeType &type, char *&parcel, int32_t &size);
    static int32_t StringFromParcel(std::string &str, char *&parcel, int32_t &size);
    static int32_t ResponseHeaderFromParcel(
        std::map<std::string, std::vector<std::string>> &headers, char *&parcel, int32_t &size);
    static int32_t ProgressExtrasFromParcel(std::map<std::string, std::string> &extras, char *&parcel, int32_t &size);
    void OnReadable(int32_t fd) override;
    bool ReadUdsData(char *buffer, int32_t readSize, int32_t &length);
    static int32_t VecInt64FromParcel(std::vector<int64_t> &vec, char *&parcel, int32_t &size);
    static int32_t MsgHeaderParcel(int32_t &msgId, int16_t &msgType, int16_t &bodySize, char *&parcel, int32_t &size);
    static int32_t ResponseFromParcel(std::shared_ptr<Response> &response, char *&parcel, int32_t &size);
    static int32_t TaskStatesFromParcel(std::vector<TaskState> &taskStates, char *&parcel, int32_t &size);
    static int32_t NotifyDataFromParcel(std::shared_ptr<NotifyData> &notifyData, char *&parcel, int32_t &size);
    static int32_t ReasonFromParcel(Reason &reason, char *&parcel, int32_t &size);
    static int32_t ReasonDataFromParcel(std::shared_ptr<int32_t> &tid, std::shared_ptr<SubscribeType> &type,
        std::shared_ptr<Reason> &reason, char *&parcel, int32_t &size);
    void HandResponseData(char *&leftBuf, int32_t &leftLen);
    void HandNotifyData(char *&leftBuf, int32_t &leftLen);
    void HandFaultsData(char *&leftBuf, int32_t &leftLen);
    void HandWaitData(char *&leftBuf, int32_t &leftLen);
    void OnShutdown(int32_t fd) override;
    void OnException(int32_t fd) override;
    void ShutdownChannel();

private:
    IResponseMessageHandler *handler_;
    int32_t messageId_{ 1 };
    int32_t sockFd_{ -1 };
    std::mutex sockFdMutex_;
};

} // namespace OHOS::Request

#endif // OHOS_REQUEST_RESPONSE_MESSAGE_RECEIVER_H