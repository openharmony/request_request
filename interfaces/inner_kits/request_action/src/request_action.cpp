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
    return RequestManager::GetInstance()->Create(config, seq, tid);
}
int32_t RequestAction::GetTask(const std::string &tid, const std::string &token, Config &config)
{
    return RequestManager::GetInstance()->GetTask(tid, token, config);
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

int32_t RequestAction::StartTasks(const std::vector<std::string> &tids, std::vector<int32_t> &rets)
{
    return RequestManager::GetInstance()->StartTasks(tids, rets);
}

int32_t RequestAction::StopTasks(const std::vector<std::string> &tids, std::vector<int32_t> &rets)
{
    return RequestManager::GetInstance()->StopTasks(tids, rets);
}

int32_t RequestAction::ResumeTasks(const std::vector<std::string> &tids, std::vector<int32_t> &rets)
{
    return RequestManager::GetInstance()->ResumeTasks(tids, rets);
}

int32_t RequestAction::RemoveTasks(const std::vector<std::string> &tids, std::vector<int32_t> &rets)
{
    return RequestManager::GetInstance()->RemoveTasks(tids, Version::API10, rets);
}

int32_t RequestAction::PauseTasks(const std::vector<std::string> &tids, std::vector<int32_t> &rets)
{
    return RequestManager::GetInstance()->PauseTasks(tids, Version::API10, rets);
}

int32_t RequestAction::ShowTasks(const std::vector<std::string> &tids, std::vector<std::pair<int32_t, TaskInfo>> &rets)
{
    return RequestManager::GetInstance()->ShowTasks(tids, rets);
}

int32_t RequestAction::TouchTasks(
    const std::vector<std::pair<std::string, std::string>> &tids, std::vector<std::pair<int32_t, TaskInfo>> &rets)
{
    return RequestManager::GetInstance()->TouchTasks(tids, rets);
}
} // namespace OHOS::Request