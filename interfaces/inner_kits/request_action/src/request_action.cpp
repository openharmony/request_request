/*
 * Copyright (C) 2023 Huawei Device Co., Ltd.
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

#include "request_action.h"

#include <memory>
#include <string>
#include <unordered_map>
#include <vector>

#include "constant.h"
#include "log.h"
#include "request_common.h"

namespace OHOS::Request {
using namespace OHOS::Security::AccessToken;

const std::unique_ptr<RequestAction> &RequestAction::GetInstance()
{
    static std::unique_ptr<RequestAction> instance = std::make_unique<RequestAction>();
    return instance;
}

int32_t RequestAction::Create(const Config &config, int32_t seq, std::string &tid)
{
    return E_PARAMETER_CHECK;
}
int32_t RequestAction::Start(const std::string &tid)
{
    return RequestManager::GetInstance()->Start(tid);
}
int32_t RequestAction::Stop(const std::string &tid)
{
    return RequestManager::GetInstance()->Stop(tid);
}

int32_t RequestAction::Touch(const std::string &tid, const std::string &token, TaskInfo &info)
{
    return RequestManager::GetInstance()->Touch(tid, token, info);
}

int32_t RequestAction::Show(const std::string &tid, TaskInfo &info)
{
    return RequestManager::GetInstance()->Show(tid, info);
}

int32_t RequestAction::Pause(const std::string &tid)
{
    return RequestManager::GetInstance()->Pause(tid, Version::API10);
}

int32_t RequestAction::Remove(const std::string &tid)
{
    return RequestManager::GetInstance()->Remove(tid, Version::API10);
}

int32_t RequestAction::Resume(const std::string &tid)
{
    return RequestManager::GetInstance()->Resume(tid);
}

int32_t RequestAction::SetMaxSpeed(const std::string &tid, const int64_t maxSpeed)
{
    return RequestManager::GetInstance()->SetMaxSpeed(tid, maxSpeed);
}

ExceptionErrorCode RequestAction::StartTasks(
    const std::vector<std::string> &tids, std::unordered_map<std::string, ExceptionErrorCode> &rets)
{
    rets.clear();
    std::vector<ExceptionErrorCode> vec;
    ExceptionErrorCode code = RequestManager::GetInstance()->StartTasks(tids, vec);
    if (code != ExceptionErrorCode::E_OK) {
        return code;
    }
    uint32_t len = static_cast<uint32_t>(tids.size());
    for (uint32_t i = 0; i < len; i++) {
        rets.insert_or_assign(tids[i], vec[i]);
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestAction::StopTasks(
    const std::vector<std::string> &tids, std::unordered_map<std::string, ExceptionErrorCode> &rets)
{
    rets.clear();
    std::vector<ExceptionErrorCode> vec;
    ExceptionErrorCode code = RequestManager::GetInstance()->StopTasks(tids, vec);
    if (code != ExceptionErrorCode::E_OK) {
        return code;
    }
    uint32_t len = static_cast<uint32_t>(tids.size());
    for (uint32_t i = 0; i < len; i++) {
        rets.insert_or_assign(tids[i], vec[i]);
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestAction::ResumeTasks(
    const std::vector<std::string> &tids, std::unordered_map<std::string, ExceptionErrorCode> &rets)
{
    rets.clear();
    std::vector<ExceptionErrorCode> vec;
    ExceptionErrorCode code = RequestManager::GetInstance()->ResumeTasks(tids, vec);
    if (code != ExceptionErrorCode::E_OK) {
        return code;
    }
    uint32_t len = static_cast<uint32_t>(tids.size());
    for (uint32_t i = 0; i < len; i++) {
        rets.insert_or_assign(tids[i], vec[i]);
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestAction::RemoveTasks(
    const std::vector<std::string> &tids, std::unordered_map<std::string, ExceptionErrorCode> &rets)
{
    rets.clear();
    std::vector<ExceptionErrorCode> vec;
    ExceptionErrorCode code = RequestManager::GetInstance()->RemoveTasks(tids, Version::API10, vec);
    if (code != ExceptionErrorCode::E_OK) {
        return code;
    }
    uint32_t len = static_cast<uint32_t>(tids.size());
    for (uint32_t i = 0; i < len; i++) {
        rets.insert_or_assign(tids[i], vec[i]);
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestAction::PauseTasks(
    const std::vector<std::string> &tids, std::unordered_map<std::string, ExceptionErrorCode> &rets)
{
    rets.clear();
    std::vector<ExceptionErrorCode> vec;
    ExceptionErrorCode code = RequestManager::GetInstance()->PauseTasks(tids, Version::API10, vec);
    if (code != ExceptionErrorCode::E_OK) {
        return code;
    }
    uint32_t len = static_cast<uint32_t>(tids.size());
    for (uint32_t i = 0; i < len; i++) {
        rets.insert_or_assign(tids[i], vec[i]);
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestAction::ShowTasks(
    const std::vector<std::string> &tids, std::unordered_map<std::string, TaskInfoRet> &rets)
{
    rets.clear();
    std::vector<TaskInfoRet> vec;
    ExceptionErrorCode code = RequestManager::GetInstance()->ShowTasks(tids, vec);
    if (code != ExceptionErrorCode::E_OK) {
        return code;
    }
    uint32_t len = static_cast<uint32_t>(tids.size());
    for (uint32_t i = 0; i < len; i++) {
        rets.insert_or_assign(tids[i], vec[i]);
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestAction::TouchTasks(
    const std::vector<TaskIdAndToken> &tidTokens, std::unordered_map<std::string, TaskInfoRet> &rets)
{
    rets.clear();
    std::vector<TaskInfoRet> vec;
    ExceptionErrorCode code = RequestManager::GetInstance()->TouchTasks(tidTokens, vec);
    if (code != ExceptionErrorCode::E_OK) {
        return code;
    }
    uint32_t len = static_cast<uint32_t>(tidTokens.size());
    for (uint32_t i = 0; i < len; i++) {
        rets.insert_or_assign(tidTokens[i].tid, vec[i]);
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestAction::SetMaxSpeeds(
    const std::vector<SpeedConfig> &speedConfig, std::unordered_map<std::string, ExceptionErrorCode> &rets)
{
    rets.clear();
    std::vector<ExceptionErrorCode> vec;
    ExceptionErrorCode code = RequestManager::GetInstance()->SetMaxSpeeds(speedConfig, vec);
    if (code != ExceptionErrorCode::E_OK) {
        return code;
    }
    uint32_t len = static_cast<uint32_t>(speedConfig.size());
    for (uint32_t i = 0; i < len; i++) {
        rets.insert_or_assign(speedConfig[i].tid, vec[i]);
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestAction::SetMode(std::string &tid, Mode mode)
{
    return RequestManager::GetInstance()->SetMode(tid, mode);
}
} // namespace OHOS::Request