// Copyright (C) 2025 Huawei Device Co., Ltd.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include "subscribe.h"

#include <memory>

#include "request.h"
#include "response_message_receiver.h"
#include "wrapper.rs.h"

namespace OHOS {
namespace RequestAni {

void UdsListener::OnChannelBroken()
{
}

void UdsListener::OnResponseReceive(const std::shared_ptr<Request::Response> &response)
{
    auto res = Response{
        .taskId = response->taskId,
        .version = response->version,
        .statusCode = response->statusCode,
        .reason = response->reason,
    };
    on_response(res);
}

void UdsListener::OnNotifyDataReceive(const std::shared_ptr<Request::NotifyData> &notifyData)
{
    auto res = Response{
        .taskId = std::to_string(notifyData->taskId),
        .version = "st",
        .statusCode = 123,
        .reason = "aaa",
    };
    on_response(res);
}

void OpenChannel(int32_t fd)
{
    static UdsListener listener;
    auto uds = std::make_shared<Request::ResponseMessageReceiver>(&listener, fd);
    uds->BeginReceive();
};

} // namespace RequestAni
} // namespace OHOS