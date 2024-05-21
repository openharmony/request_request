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

#include "request_manager_impl.h"

namespace OHOS::Request {

const std::unique_ptr<RequestManager> &RequestManager::GetInstance()
{
    static std::unique_ptr<RequestManager> instance(new RequestManager());
    return instance;
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

int32_t RequestManager::Pause(const std::string &tid, Version version)
{
    return RequestManagerImpl::GetInstance()->Pause(tid, version);
}

int32_t RequestManager::QueryMimeType(const std::string &tid, std::string &mimeType)
{
    return RequestManagerImpl::GetInstance()->QueryMimeType(tid, mimeType);
}

int32_t RequestManager::Remove(const std::string &tid, Version version)
{
    return RequestManagerImpl::GetInstance()->Remove(tid, version);
}

int32_t RequestManager::Resume(const std::string &tid)
{
    return RequestManagerImpl::GetInstance()->Resume(tid);
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

bool RequestManager::LoadRequestServer()
{
    return RequestManagerImpl::GetInstance()->LoadRequestServer();
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

} // namespace OHOS::Request