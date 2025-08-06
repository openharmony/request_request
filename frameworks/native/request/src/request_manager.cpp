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

#include "request_manager.h"

#include <optional>
#include <vector>

#include "request_common.h"
#include "request_manager_impl.h"

namespace OHOS::Request {

const std::unique_ptr<RequestManager> &RequestManager::GetInstance()
{
    static std::unique_ptr<RequestManager> instance(new RequestManager());
    return instance;
}

ExceptionErrorCode RequestManager::CreateTasks(const std::vector<Config> &configs, std::vector<TaskRet> &rets)
{
    return RequestManagerImpl::GetInstance()->CreateTasks(configs, rets);
}

ExceptionErrorCode RequestManager::StartTasks(
    const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets)
{
    return RequestManagerImpl::GetInstance()->StartTasks(tids, rets);
}

ExceptionErrorCode RequestManager::StopTasks(
    const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets)
{
    return RequestManagerImpl::GetInstance()->StopTasks(tids, rets);
}

ExceptionErrorCode RequestManager::ResumeTasks(
    const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets)
{
    return RequestManagerImpl::GetInstance()->ResumeTasks(tids, rets);
}

ExceptionErrorCode RequestManager::RemoveTasks(
    const std::vector<std::string> &tids, const Version version, std::vector<ExceptionErrorCode> &rets)
{
    return RequestManagerImpl::GetInstance()->RemoveTasks(tids, version, rets);
}

ExceptionErrorCode RequestManager::PauseTasks(
    const std::vector<std::string> &tids, const Version version, std::vector<ExceptionErrorCode> &rets)
{
    return RequestManagerImpl::GetInstance()->PauseTasks(tids, version, rets);
}

ExceptionErrorCode RequestManager::ShowTasks(const std::vector<std::string> &tids, std::vector<TaskInfoRet> &rets)
{
    return RequestManagerImpl::GetInstance()->ShowTasks(tids, rets);
}

ExceptionErrorCode RequestManager::TouchTasks(
    const std::vector<TaskIdAndToken> &tidTokens, std::vector<TaskInfoRet> &rets)
{
    return RequestManagerImpl::GetInstance()->TouchTasks(tidTokens, rets);
}

ExceptionErrorCode RequestManager::SetMaxSpeeds(
    const std::vector<SpeedConfig> &speedConfig, std::vector<ExceptionErrorCode> &rets)
{
    return RequestManagerImpl::GetInstance()->SetMaxSpeeds(speedConfig, rets);
}

ExceptionErrorCode RequestManager::SetMode(const std::string &tid, const Mode mode)
{
    return RequestManagerImpl::GetInstance()->SetMode(tid, mode);
}

ExceptionErrorCode RequestManager::DisableTaskNotification(
    const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets)
{
    return RequestManagerImpl::GetInstance()->DisableTaskNotification(tids, rets);
}

int32_t RequestManager::Create(const Config &config, int32_t seq, std::string &tid)
{
    return RequestManagerImpl::GetInstance()->Create(config, seq, tid);
}
int32_t RequestManager::GetTask(const std::string &tid, const std::string &token, Config &config)
{
    return RequestManagerImpl::GetInstance()->GetTask(tid, token, config);
}
int32_t RequestManager::Start(const std::string &tid)
{
    return RequestManagerImpl::GetInstance()->Start(tid);
}
int32_t RequestManager::Stop(const std::string &tid)
{
    return RequestManagerImpl::GetInstance()->Stop(tid);
}

int32_t RequestManager::Query(const std::string &tid, TaskInfo &info)
{
    return RequestManagerImpl::GetInstance()->Query(tid, info);
}

int32_t RequestManager::Touch(const std::string &tid, const std::string &token, TaskInfo &info)
{
    return RequestManagerImpl::GetInstance()->Touch(tid, token, info);
}

int32_t RequestManager::Search(const Filter &filter, std::vector<std::string> &tids)
{
    return RequestManagerImpl::GetInstance()->Search(filter, tids);
}

int32_t RequestManager::Show(const std::string &tid, TaskInfo &info)
{
    return RequestManagerImpl::GetInstance()->Show(tid, info);
}

int32_t RequestManager::Pause(const std::string &tid, const Version version)
{
    return RequestManagerImpl::GetInstance()->Pause(tid, version);
}

int32_t RequestManager::QueryMimeType(const std::string &tid, std::string &mimeType)
{
    return RequestManagerImpl::GetInstance()->QueryMimeType(tid, mimeType);
}

int32_t RequestManager::Remove(const std::string &tid, const Version version)
{
    return RequestManagerImpl::GetInstance()->Remove(tid, version);
}

int32_t RequestManager::Resume(const std::string &tid)
{
    return RequestManagerImpl::GetInstance()->Resume(tid);
}

int32_t RequestManager::SetMaxSpeed(const std::string &tid, const int64_t maxSpeed)
{
    return RequestManagerImpl::GetInstance()->SetMaxSpeed(tid, maxSpeed);
}

int32_t RequestManager::Subscribe(const std::string &taskId)
{
    return RequestManagerImpl::GetInstance()->Subscribe(taskId);
}

int32_t RequestManager::Unsubscribe(const std::string &taskId)
{
    return RequestManagerImpl::GetInstance()->Unsubscribe(taskId);
}

void RequestManager::RestoreListener(void (*callback)())
{
    return RequestManagerImpl::GetInstance()->RestoreListener(callback);
}

void RequestManager::LoadRequestServer()
{
    RequestManagerImpl::GetInstance()->LoadRequestServer();
}

bool RequestManager::SubscribeSA()
{
    return RequestManagerImpl::GetInstance()->SubscribeSA();
}

bool RequestManager::UnsubscribeSA()
{
    return RequestManagerImpl::GetInstance()->UnsubscribeSA();
}

bool RequestManager::IsSaReady()
{
    return RequestManagerImpl::GetInstance()->IsSaReady();
}

void RequestManager::ReopenChannel()
{
    return RequestManagerImpl::GetInstance()->ReopenChannel();
}

int32_t RequestManager::AddListener(
    const std::string &taskId, const SubscribeType &type, const std::shared_ptr<IResponseListener> &listener)
{
    return RequestManagerImpl::GetInstance()->AddListener(taskId, type, listener);
}

int32_t RequestManager::RemoveListener(
    const std::string &taskId, const SubscribeType &type, const std::shared_ptr<IResponseListener> &listener)
{
    return RequestManagerImpl::GetInstance()->RemoveListener(taskId, type, listener);
}

int32_t RequestManager::AddListener(
    const std::string &taskId, const SubscribeType &type, const std::shared_ptr<INotifyDataListener> &listener)
{
    return RequestManagerImpl::GetInstance()->AddListener(taskId, type, listener);
}

int32_t RequestManager::RemoveListener(
    const std::string &taskId, const SubscribeType &type, const std::shared_ptr<INotifyDataListener> &listener)
{
    return RequestManagerImpl::GetInstance()->RemoveListener(taskId, type, listener);
}

void RequestManager::RemoveAllListeners(const std::string &taskId)
{
    RequestManagerImpl::GetInstance()->RemoveAllListeners(taskId);
}

int32_t RequestManager::GetNextSeq()
{
    return RequestManagerImpl::GetInstance()->GetNextSeq();
}

int32_t RequestManager::CreateGroup(std::string &gid, const bool gauge, Notification &notification)
{
    return RequestManagerImpl::GetInstance()->CreateGroup(gid, gauge, notification);
}
int32_t RequestManager::AttachGroup(const std::string &gid, const std::vector<std::string> &tids)
{
    return RequestManagerImpl::GetInstance()->AttachGroup(gid, tids);
}
int32_t RequestManager::DeleteGroup(const std::string &gid)
{
    return RequestManagerImpl::GetInstance()->DeleteGroup(gid);
}

} // namespace OHOS::Request
