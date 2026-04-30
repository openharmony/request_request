/*
 * Copyright (c) 2026 Huawei Device Co., Ltd.
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

#include "cj_failed_listener.h"

#include "cj_request_common.h"
#include "cj_request_task.h"
#include "log.h"
#include "request_manager.h"

namespace OHOS::CJSystemapi::Request {
using OHOS::Request::Action;
using OHOS::Request::DownloadErrorCode;
using OHOS::Request::RequestManager;
using OHOS::Request::Version;

static std::map<Reason, DownloadErrorCode> g_failMap = {
    {Reason::REASON_OK, DownloadErrorCode::ERROR_FILE_ALREADY_EXISTS},
    {Reason::IO_ERROR, DownloadErrorCode::ERROR_FILE_ERROR},
    {Reason::INSUFFICIENT_SPACE, DownloadErrorCode::ERROR_INSUFFICIENT_SPACE},
    {Reason::REDIRECT_ERROR, DownloadErrorCode::ERROR_TOO_MANY_REDIRECTS},
    {Reason::OTHERS_ERROR, DownloadErrorCode::ERROR_UNKNOWN},
    {Reason::NETWORK_OFFLINE, DownloadErrorCode::ERROR_OFFLINE},
    {Reason::UNSUPPORTED_NETWORK_TYPE, DownloadErrorCode::ERROR_UNSUPPORTED_NETWORK_TYPE},
    {Reason::UNSUPPORT_RANGE_REQUEST, DownloadErrorCode::ERROR_UNKNOWN},
};

bool CJFailedListener::IsListenerAdded(void *cb)
{
    if (cb == nullptr) {
        return true;
    }
    for (auto it = this->allCb_.begin(); it != this->allCb_.end(); it++) {
        if (it->second->cbId_ == cb) {
            return it->first;
        }
    }
    return false;
}

void CJFailedListener::AddListener(std::function<void(int32_t)> cb, CFunc cbId)
{
    std::lock_guard<std::recursive_mutex> lock(allCbMutex_);
    if (this->IsListenerAdded(cbId)) {
        return;
    }

    this->allCb_.push_back(std::make_pair(true, std::make_shared<CJFailedCallBackInfo>(cb, cbId)));
    ++this->validCbNum;
    if (this->validCbNum == 1) {
        RequestManager::GetInstance()->AddListener(this->taskId_, SubscribeType::FAILED, shared_from_this());
    }
}

void CJFailedListener::RemoveListenerInner(CFunc cbId)
{
    if (this->validCbNum == 0) {
        return;
    }

    if (cbId == nullptr) {
        for (auto it = this->allCb_.begin(); it != this->allCb_.end(); it++) {
            it->first = false;
        }
        this->validCbNum = 0;
        return;
    }

    for (auto it = this->allCb_.begin(); it != this->allCb_.end(); it++) {
        if (it->second->cbId_ != cbId) {
            continue;
        }
        if (it->first) {
            it->first = false;
            --this->validCbNum;
        }
        break;
    }
}

void CJFailedListener::RemoveListener(CFunc cbId)
{
    std::lock_guard<std::recursive_mutex> lock(allCbMutex_);
    this->RemoveListenerInner(cbId);
    if (this->validCbNum == 0) {
        RequestManager::GetInstance()->RemoveListener(this->taskId_, SubscribeType::FAILED, shared_from_this());
    }
}

int32_t CJFailedListener::ConvertToErrCode(const std::shared_ptr<NotifyData> &notifyData)
{
    if (notifyData->taskStates.empty()) {
        return static_cast<int32_t>(DownloadErrorCode::ERROR_UNKNOWN);
    }
    auto it = g_failMap.find(static_cast<Reason>(notifyData->taskStates[0].responseCode));
    if (it != g_failMap.end()) {
        return static_cast<int32_t>(it->second);
    }
    return static_cast<int32_t>(DownloadErrorCode::ERROR_UNKNOWN);
}

void CJFailedListener::OnNotifyDataReceive(const std::shared_ptr<NotifyData> &notifyData)
{
    int32_t errCode = ConvertToErrCode(notifyData);
    std::lock_guard<std::recursive_mutex> lock(allCbMutex_);
    for (auto it = this->allCb_.begin(); it != this->allCb_.end();) {
        if (it->first == false) {
            it = this->allCb_.erase(it);
            continue;
        }
        it->second->cb_(errCode);
        it++;
    }

    if (notifyData->version == Version::API10) {
        if (notifyData->type == SubscribeType::FAILED) {
            CJRequestTask::ClearTaskTemp(std::to_string(notifyData->taskId), true, false, false);
        }
    }
}

void CJFailedListener::OnFaultsReceive(const std::shared_ptr<int32_t> &tid,
    const std::shared_ptr<SubscribeType> &type, const std::shared_ptr<Reason> &reason)
{
    return;
}

void CJFailedListener::OnWaitReceive(std::int32_t taskId, OHOS::Request::WaitingReason reason)
{
    return;
}

} // namespace OHOS::CJSystemapi::Request
