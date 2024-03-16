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

#include "js_notify_data_listener.h"

#include "js_task.h"
#include "log.h"
#include "napi_utils.h"
#include "request_event.h"
#include "request_manager.h"
#include "uv_queue.h"

namespace OHOS::Request {

napi_status JSNotifyDataListener::AddListener(napi_value cb)
{
    napi_status ret = this->AddListenerInner(cb);
    if (ret != napi_ok) {
        return ret;
    }
    if (this->validCbNum == 1) {
        RequestManager::GetInstance()->AddListener(this->taskId_, this->type_, shared_from_this());
    }
    return napi_ok;
}

napi_status JSNotifyDataListener::RemoveListener(napi_value cb)
{
    napi_status ret = this->RemoveListenerInner(cb);
    if (ret != napi_ok) {
        return ret;
    }
    if (this->validCbNum == 0) {
        RequestManager::GetInstance()->RemoveListener(this->taskId_, this->type_, shared_from_this());
    }
    return napi_ok;
}

void JSNotifyDataListener::ProcessHeaderReceive(const std::shared_ptr<NotifyData> &notifyData)
{
    if (!(notifyData->version == Version::API9 && notifyData->action == Action::UPLOAD &&
        notifyData->type == SubscribeType::HEADER_RECEIVE)) {
        return;
    } else if (!(notifyData->version == Version::API10 && notifyData->action == Action::UPLOAD &&
               notifyData->progress.state == State::COMPLETED &&
               (notifyData->type == SubscribeType::PROGRESS || notifyData->type == SubscribeType::COMPLETED))) {
        return;
    }

    JsTask *task = nullptr;
    {
        std::lock_guard<std::mutex> lockGuard(JsTask::taskMutex_);
        auto item = JsTask::taskMap_.find(std::to_string(notifyData->taskId));
        if (item == JsTask::taskMap_.end()) {
            REQUEST_HILOGE("Task ID not found");
            return;
        }
        task = item->second;
    }

    uint32_t index = notifyData->progress.index;
    size_t len = task->config_.bodyFileNames.size();
    if (index < len) {
        std::string &filePath = task->config_.bodyFileNames[index];
        NapiUtils::ReadBytesFromFile(filePath, notifyData->progress.bodyBytes);
        // Waiting for "complete" to read and delete.
        if (!(notifyData->version == Version::API10 && index == len - 1 &&
              notifyData->type == SubscribeType::PROGRESS)) {
            NapiUtils::RemoveFile(filePath);
        }
    }
}

void JSNotifyDataListener::NotifyDataProcess(
    const std::shared_ptr<NotifyData> &notifyData, napi_value *value, uint32_t &paramNumber)
{
    if (notifyData->version == Version::API10) {
        REQUEST_HILOGD("Receive API10 callback");
        value[0] = NapiUtils::Convert2JSValue(this->env_, notifyData->progress);
        return;
    }

    ProcessHeaderReceive(notifyData);

    if (notifyData->action == Action::DOWNLOAD) {
        if (notifyData->type == SubscribeType::PROGRESS) {
            value[0] = NapiUtils::Convert2JSValue(this->env_, notifyData->progress.processed);
            if (!notifyData->progress.sizes.empty()) {
                value[1] = NapiUtils::Convert2JSValue(this->env_, notifyData->progress.sizes[0]);
                paramNumber = NapiUtils::TWO_ARG;
            }
        } else if (notifyData->type == SubscribeType::FAILED) {
            if (notifyData->taskStates.empty()) {
                paramNumber = 0;
                return;
            }
            int64_t failedReason;
            auto it = RequestEvent::failMap_.find(static_cast<Reason>(notifyData->taskStates[0].responseCode));
            if (it != RequestEvent::failMap_.end()) {
                failedReason = it->second;
            } else {
                failedReason = static_cast<int64_t>(ERROR_UNKNOWN);
            }
            value[0] = NapiUtils::Convert2JSValue(this->env_, failedReason);
        }
    } else if (notifyData->action == Action::UPLOAD) {
        if (notifyData->type == SubscribeType::COMPLETED || notifyData->type == SubscribeType::FAILED) {
            value[0] = NapiUtils::Convert2JSValue(env_, notifyData->taskStates);
        } else if (notifyData->type == SubscribeType::PROGRESS) {
            int64_t totalSize = 0;
            for (const auto &size : notifyData->progress.sizes) {
                totalSize += size;
            }
            value[0] = NapiUtils::Convert2JSValue(this->env_, notifyData->progress.totalProcessed);
            value[0] = NapiUtils::Convert2JSValue(this->env_, totalSize);
            paramNumber = NapiUtils::TWO_ARG;
        } else if (notifyData->type == SubscribeType::HEADER_RECEIVE) {
            value[0] = NapiUtils::Convert2JSHeadersAndBody(
                env_, notifyData->progress.extras, notifyData->progress.bodyBytes, true);
        }
    }
}

static std::string SubscribeTypeToString(SubscribeType type)
{
    switch (type) {
        case SubscribeType::COMPLETED:
            return "completed";
        case SubscribeType::FAILED:
            return "failed";
        case SubscribeType::HEADER_RECEIVE:
            return "header_receive";
        case SubscribeType::PAUSE:
            return "pause";
        case SubscribeType::PROGRESS:
            return "progress";
        case SubscribeType::REMOVE:
            return "remove";
        case SubscribeType::RESUME:
            return "resume";
        case SubscribeType::RESPONSE:
            return "response";
        case SubscribeType::BUTT:
            return "butt";
    }
}

void JSNotifyDataListener::OnNotifyDataReceive(const std::shared_ptr<NotifyData> &notifyData)
{
    REQUEST_HILOGI("OnNotifyDataReceive type is %{public}s, tid is %{public}d",
        SubscribeTypeToString(notifyData->type).c_str(), notifyData->taskId);
    uv_loop_s *loop = nullptr;
    napi_get_uv_event_loop(this->env_, &loop);
    if (loop == nullptr) {
        REQUEST_HILOGE("napi_get_uv_event_loop failed");
        return;
    }
    uv_work_t *work = new (std::nothrow) uv_work_t;
    if (work == nullptr) {
        REQUEST_HILOGE("uv_work_t new failed");
        return;
    }
    {
        std::lock_guard<std::mutex> lock(this->notifyDataMutex_);
        this->notifyData_ = notifyData;
    }
    work->data = reinterpret_cast<void *>(this);
    uv_queue_work(
        loop, work, [](uv_work_t *work) {},
        [](uv_work_t *work, int status) {
            napi_value values[NapiUtils::TWO_ARG] = { nullptr };
            uint32_t paramNumber = NapiUtils::ONE_ARG;
            JSNotifyDataListener *listener = static_cast<JSNotifyDataListener *>(work->data);
            std::lock_guard<std::mutex> lock(listener->notifyDataMutex_);
            napi_handle_scope scope = nullptr;
            napi_open_handle_scope(listener->env_, &scope);
            listener->NotifyDataProcess(listener->notifyData_, values, paramNumber);
            listener->OnMessageReceive(values, paramNumber);
            napi_close_handle_scope(listener->env_, scope);
            std::string tid = std::to_string(listener->notifyData_->taskId);
            if (listener->notifyData_->version == Version::API9 &&
                (listener->notifyData_->type == SubscribeType::COMPLETED ||
                 listener->notifyData_->type == SubscribeType::FAILED ||
                 listener->notifyData_->type == SubscribeType::REMOVE)) {
                JsTask::ClearTaskContext(tid);
                JsTask::ClearTaskMap(tid);
            } else if (listener->notifyData_->version == Version::API10 &&
                       listener->notifyData_->type == SubscribeType::REMOVE) {
                JsTask::ClearTaskContext(tid);
                JsTask::ClearTaskMap(tid);
            }
            delete work;
        });
}

} // namespace OHOS::Request