/*
 * Copyright (C) 2024 Huawei Device Co., Ltd.
 * Licensed under the Apache License, Version 2.0 (the "License")
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

#include "cj_request_event.h"

#include "cj_initialize.h"
#include "log.h"
#include "request_manager.h"

namespace OHOS::CJSystemapi::Request {
using OHOS::Request::Action;
using OHOS::Request::Config;
using OHOS::Request::FileSpec;
using OHOS::Request::FUNCTION_PAUSE;
using OHOS::Request::FUNCTION_RESUME;
using OHOS::Request::FUNCTION_START;
using OHOS::Request::FUNCTION_STOP;
using OHOS::Request::RequestManager;
using OHOS::Request::Version;

static constexpr const char *EVENT_COMPLETED = "completed";
static constexpr const char *EVENT_FAILED = "failed";
static constexpr const char *EVENT_PAUSE = "pause";
static constexpr const char *EVENT_RESUME = "resume";
static constexpr const char *EVENT_REMOVE = "remove";
static constexpr const char *EVENT_PROGRESS = "progress";
static constexpr const char *EVENT_RESPONSE = "response";
static constexpr const char *EVENT_FAULT_OCCUR = "faultOccur";

std::map<std::string, SubscribeType> CJRequestEvent::supportEventsV10_ = {
    {EVENT_PROGRESS, SubscribeType::PROGRESS}, {EVENT_COMPLETED, SubscribeType::COMPLETED},
    {EVENT_FAILED, SubscribeType::FAILED},     {EVENT_PAUSE, SubscribeType::PAUSE},
    {EVENT_RESUME, SubscribeType::RESUME},     {EVENT_REMOVE, SubscribeType::REMOVE},
    {EVENT_RESPONSE, SubscribeType::RESPONSE}, {EVENT_FAULT_OCCUR, SubscribeType::FAULT_OCCUR},
};

SubscribeType CJRequestEvent::StringToSubscribeType(const std::string &type)
{
    if (supportEventsV10_.find(type) != supportEventsV10_.end()) {
        return supportEventsV10_[type];
    }

    return SubscribeType::BUTT;
}

std::map<std::string, CJRequestEvent::Event> CJRequestEvent::requestEvent_ = {
    {FUNCTION_PAUSE, CJRequestEvent::PauseExec},
    {FUNCTION_RESUME, CJRequestEvent::ResumeExec},
    {FUNCTION_START, CJRequestEvent::StartExec},
    {FUNCTION_STOP, CJRequestEvent::StopExec},
};

ExceptionErrorCode CJRequestEvent::Exec(std::string execType, const CJRequestTask *task)
{
    auto handle = requestEvent_.find(execType);
    if (handle == requestEvent_.end()) {
        return ExceptionErrorCode::E_PARAMETER_CHECK;
    }

    return (ExceptionErrorCode)handle->second(task);
}

ExceptionErrorCode CJRequestEvent::StartExec(const CJRequestTask *task)
{
    REQUEST_HILOGD("RequestEvent::StartExec in");
    Config config = task->config_;
    // Rechecks file path.
    if (config.files.size() == 0) {
        return ExceptionErrorCode::E_FILE_IO;
    }
    FileSpec file = config.files[0];
    if (CJInitialize::FindDir(file.uri) && config.action == Action::DOWNLOAD) {
        REQUEST_HILOGD("Found the downloaded file: %{public}s.", file.uri.c_str());
        if (chmod(file.uri.c_str(), S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH) != 0) {
            REQUEST_HILOGD("File add OTH access Failed.");
        }
        if (!CJRequestTask::SetPathPermission(file.uri)) {
            REQUEST_HILOGE("Set path permission fail.");
            return ExceptionErrorCode::E_FILE_IO;
        }
    }

    return (ExceptionErrorCode)RequestManager::GetInstance()->Start(task->GetTidStr());
}

ExceptionErrorCode CJRequestEvent::StopExec(const CJRequestTask *task)
{
    return (ExceptionErrorCode)RequestManager::GetInstance()->Stop(task->GetTidStr());
}

ExceptionErrorCode CJRequestEvent::PauseExec(const CJRequestTask *task)
{
    return (ExceptionErrorCode)RequestManager::GetInstance()->Pause(task->GetTidStr(), Version::API10);
}

ExceptionErrorCode CJRequestEvent::ResumeExec(const CJRequestTask *task)
{
    return (ExceptionErrorCode)RequestManager::GetInstance()->Resume(task->GetTidStr());
}

} // namespace OHOS::CJSystemapi::Request