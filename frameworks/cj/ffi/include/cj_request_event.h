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

#ifndef OHOS_REQUEST_CJ_EVENT_H
#define OHOS_REQUEST_CJ_EVENT_H

#include <string>
#include <unordered_set>
#include "cj_request_task.h"

namespace OHOS::CJSystemapi::Request {
using OHOS::Request::ExceptionErrorCode;

class CJRequestEvent final {
public:
    CJRequestEvent() = default;
    ~CJRequestEvent() = default;
    CJRequestEvent(CJRequestEvent const &) = delete;
    void operator=(CJRequestEvent const &) = delete;
    CJRequestEvent(CJRequestEvent &&) = delete;
    CJRequestEvent &operator=(CJRequestEvent &&) = delete;

    static ExceptionErrorCode Exec(std::string execType, const CJRequestTask *task);
    static SubscribeType StringToSubscribeType(const std::string &type);

private:
    using Event = std::function<int32_t(const CJRequestTask *)>;
    static std::map<std::string, Event> requestEvent_;

    static ExceptionErrorCode StartExec(const CJRequestTask *task);
    static ExceptionErrorCode StopExec(const CJRequestTask *task);
    static ExceptionErrorCode PauseExec(const CJRequestTask *task);
    static ExceptionErrorCode ResumeExec(const CJRequestTask *task);
    static std::map<std::string, SubscribeType> supportEventsV10_;
};

} // namespace OHOS::CJSystemapi::Request

#endif // OHOS_REQUEST_CJ_EVENT_H