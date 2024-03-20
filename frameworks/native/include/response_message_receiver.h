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
};

class ResponseMessageReceiver
    : public OHOS::AppExecFwk::FileDescriptorListener
    , public std::enable_shared_from_this<ResponseMessageReceiver> {
public:
    static constexpr uint32_t RESPONSE_MAX_SIZE = 8 * 1024;
    static constexpr uint32_t RESPONSE_MAGIC_NUM = 0x43434646;

    ResponseMessageReceiver(IResponseMessageHandler *handler, int32_t sockFd);
    void BeginReceive();
    void Shutdown(void);

private:
    void OnReadable(int32_t fd) override;
    void OnShutdown(int32_t fd) override;
    void OnException(int32_t fd) override;

private:
    IResponseMessageHandler *handler_;
    int32_t sockFd_{ -1 };
    int32_t messageId_{ 1 };
};

} // namespace OHOS::Request

#endif // OHOS_REQUEST_RESPONSE_MESSAGE_RECEIVER_H