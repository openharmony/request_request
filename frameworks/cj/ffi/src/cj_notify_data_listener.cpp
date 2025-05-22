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

#include "cj_notify_data_listener.h"

#include <numeric>
#include "cj_request_common.h"
#include "cj_request_task.h"
#include "log.h"
#include "request_manager.h"

namespace OHOS::CJSystemapi::Request {
using OHOS::Request::Action;
using OHOS::Request::RequestManager;
using OHOS::Request::State;
using OHOS::Request::Version;

void CJNotifyDataListener::AddListener(std::function<void(CProgress)> cb, CFunc cbId)
{
    this->AddListenerInner(cb, cbId);
    /* remove listener must be subscribed to free task */
    if (this->validCbNum == 1 && this->type_ != SubscribeType::REMOVE) {
        RequestManager::GetInstance()->AddListener(this->taskId_, this->type_, shared_from_this());
    }
}

void CJNotifyDataListener::RemoveListener(CFunc cbId)
{
    this->RemoveListenerInner(cbId);
    if (this->validCbNum == 0 && this->type_ != SubscribeType::REMOVE) {
        RequestManager::GetInstance()->RemoveListener(this->taskId_, this->type_, shared_from_this());
    }
}

bool CJNotifyDataListener::IsHeaderReceive(const std::shared_ptr<NotifyData> &notifyData)
{
    if (notifyData->version == Version::API10 && notifyData->action == Action::UPLOAD &&
        notifyData->progress.state == State::COMPLETED &&
        (notifyData->type == SubscribeType::PROGRESS || notifyData->type == SubscribeType::COMPLETED)) {
        return true;
    }
    return false;
}

void CJNotifyDataListener::ProcessHeaderReceive(const std::shared_ptr<NotifyData> &notifyData)
{
    CJRequestTask *task = nullptr;
    {
        std::lock_guard<std::mutex> lockGuard(CJRequestTask::taskMutex_);
        auto item = CJRequestTask::taskMap_.find(std::to_string(notifyData->taskId));
        if (item == CJRequestTask::taskMap_.end()) {
            REQUEST_HILOGE("CJRequestTask ID not found");
            return;
        }
        task = item->second;
    }

    uint32_t index = notifyData->progress.index;
    size_t len = task->config_.bodyFileNames.size();
    if (index < len) {
        std::string &filePath = task->config_.bodyFileNames[index];
        ReadBytesFromFile(filePath, notifyData->progress.bodyBytes);
        // Waiting for "complete" to read and delete.
        if (!(notifyData->version == Version::API10 && index + 1 == len &&
              notifyData->type == SubscribeType::PROGRESS)) {
            RemoveFile(filePath);
        }
    }
}

void CJNotifyDataListener::NotifyDataProcess(const std::shared_ptr<NotifyData> &notifyData)
{
    if (IsHeaderReceive(notifyData)) {
        ProcessHeaderReceive(notifyData);
    }
}

static void RemoveJSTask(const std::shared_ptr<NotifyData> &notifyData)
{
    std::string tid = std::to_string(notifyData->taskId);
    if (notifyData->version == Version::API10) {
        if (notifyData->type == SubscribeType::REMOVE) {
            CJRequestTask::ClearTaskTemp(tid, true, true, true);
            CJRequestTask::ClearTaskMap(tid);
            REQUEST_HILOGD("jstask %{public}s removed", tid.c_str());
        } else if (notifyData->type == SubscribeType::COMPLETED || notifyData->type == SubscribeType::FAILED) {
            CJRequestTask::ClearTaskTemp(tid, true, false, false);
        }
    }
}

void CJNotifyDataListener::OnNotifyDataReceive(const std::shared_ptr<NotifyData> &notifyData)
{
    this->NotifyDataProcess(notifyData);
    this->OnMessageReceive(notifyData);
    RemoveJSTask(notifyData);
}

void CJNotifyDataListener::OnFaultsReceive(const std::shared_ptr<int32_t> &tid,
    const std::shared_ptr<SubscribeType> &type, const std::shared_ptr<Reason> &reason)
{
    return;
}

void CJNotifyDataListener::OnWaitReceive(std::int32_t taskId, OHOS::Request::WaitingReason reason)
{
    // unimplemented
    return;
}

} // namespace OHOS::CJSystemapi::Request