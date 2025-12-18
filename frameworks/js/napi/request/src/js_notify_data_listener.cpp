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

#include <numeric>

#include "js_task.h"
#include "log.h"
#include "napi/native_node_api.h"
#include "napi_utils.h"
#include "request_event.h"
#include "request_manager.h"

namespace OHOS::Request {

napi_status JSNotifyDataListener::AddListener(napi_value cb)
{
    napi_status ret = this->AddListenerInner(cb);
    if (ret != napi_ok) {
        return ret;
    }
    /* remove listener must be subscribed to free task */
    if (this->validCbNum == 1 && this->type_ != SubscribeType::REMOVE) {
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
    if (this->validCbNum == 0 && this->type_ != SubscribeType::REMOVE) {
        RequestManager::GetInstance()->RemoveListener(this->taskId_, this->type_, shared_from_this());
    }
    return napi_ok;
}

bool JSNotifyDataListener::IsHeaderReceive(const std::shared_ptr<NotifyData> &notifyData)
{
    if (notifyData->version == Version::API9 && notifyData->action == Action::UPLOAD
        && notifyData->type == SubscribeType::HEADER_RECEIVE) {
        return true;
    } else if (notifyData->version == Version::API10 && notifyData->action == Action::UPLOAD
               && notifyData->progress.state == State::COMPLETED
               && (notifyData->type == SubscribeType::PROGRESS || notifyData->type == SubscribeType::COMPLETED)) {
        return true;
    }
    return false;
}

void JSNotifyDataListener::ProcessHeaderReceive(const std::shared_ptr<NotifyData> &notifyData)
{
    uint32_t index = notifyData->progress.index;
    size_t len = 0;
    std::string filePath;
    {
        std::lock_guard<std::mutex> lockGuard(JsTask::taskMutex_);
        auto it = JsTask::taskContextMap_.find(std::to_string(notifyData->taskId));
        if (it == JsTask::taskContextMap_.end() || it->second->task == nullptr) {
            REQUEST_HILOGE("Task ID not found");
            return;
        }
        JsTask *task = it->second->task;
        if (task->config_.multipart) {
            index = 0;
        }
        len = task->config_.bodyFileNames.size();
        if (index >= len) {
            return;
        }
        filePath = task->config_.bodyFileNames[index];
    }

    NapiUtils::ReadBytesFromFile(filePath, notifyData->progress.bodyBytes);
    // Waiting for "complete" to read and delete.
    if (!(notifyData->version == Version::API10 && index + 1 == len && notifyData->type == SubscribeType::PROGRESS)) {
        NapiUtils::RemoveFile(filePath);
    }
}

void JSNotifyDataListener::NotifyDataProcess(
    const std::shared_ptr<NotifyData> &notifyData, napi_value *value, uint32_t &paramNumber)
{
    if (IsHeaderReceive(notifyData)) {
        ProcessHeaderReceive(notifyData);
    }

    if (notifyData->version == Version::API10) {
        REQUEST_HILOGD("Receive API10 callback");
        value[0] = NapiUtils::Convert2JSValue(this->env_, notifyData->progress);
        return;
    }

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
            int64_t totalSize =
                std::accumulate(notifyData->progress.sizes.begin(), notifyData->progress.sizes.end(), 0);
            value[0] = NapiUtils::Convert2JSValue(this->env_, notifyData->progress.totalProcessed);
            value[1] = NapiUtils::Convert2JSValue(this->env_, totalSize);
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
        case SubscribeType::FAULT_OCCUR:
            return "faultOccur";
        case SubscribeType::WAIT:
            return "wait";
        case SubscribeType::BUTT:
            return "butt";
    }
}

static RemoveTaskChecker CheckRemoveJSTask(const std::shared_ptr<NotifyData> &notifyData, const std::string &tid)
{
    if (notifyData->version == Version::API9
        && (notifyData->type == SubscribeType::COMPLETED || notifyData->type == SubscribeType::FAILED
            || notifyData->type == SubscribeType::REMOVE)) {
        return RemoveTaskChecker::ClearFileAndRemoveTask;
    } else if (notifyData->version == Version::API10) {
        if (notifyData->type == SubscribeType::REMOVE) {
            return RemoveTaskChecker::ClearFileAndRemoveTask;
        } else if (notifyData->type == SubscribeType::COMPLETED || notifyData->type == SubscribeType::FAILED) {
            return RemoveTaskChecker::ClearFile;
        }
    }
    return RemoveTaskChecker::DoNothing;
}

void JSNotifyDataListener::DoJSTask(const std::shared_ptr<NotifyData> &notifyData)
{
    std::string tid = std::to_string(notifyData->taskId);
    uint32_t paramNumber = NapiUtils::ONE_ARG;
    napi_value values[NapiUtils::TWO_ARG] = { nullptr };
    // Data from file to memory.
    this->NotifyDataProcess(notifyData, values, paramNumber);
    RemoveTaskChecker checkDo = CheckRemoveJSTask(notifyData, tid);
    if (checkDo == RemoveTaskChecker::DoNothing) {
        this->OnMessageReceive(values, paramNumber);
    } else if (checkDo == RemoveTaskChecker::ClearFile) {
        JsTask::ClearTaskTemp(tid, true, false, false);
        REQUEST_HILOGD("jstask %{public}s clear file", tid.c_str());
        this->OnMessageReceive(values, paramNumber);
    } else if (checkDo == RemoveTaskChecker::ClearFileAndRemoveTask) {
        JsTask::ClearTaskTemp(tid, true, true, true);
        REQUEST_HILOGD("jstask %{public}s clear file", tid.c_str());
        this->OnMessageReceive(values, paramNumber);
        JsTask::RemoveTaskContext(tid);
        REQUEST_HILOGD("jstask %{public}s removed", tid.c_str());
    }
}

void JSNotifyDataListener::OnNotifyDataReceive(const std::shared_ptr<NotifyData> &notifyData)
{
    NotifyDataPtr *ptr = new (std::nothrow) NotifyDataPtr;
    if (ptr == nullptr) {
        REQUEST_HILOGE("NotifyDataPtr new failed");
        return;
    }
    ptr->listener = shared_from_this();
    ptr->notifyData = notifyData;
    REQUEST_HILOGI(
        "cb %{public}s %{public}d", SubscribeTypeToString(notifyData->type).c_str(), notifyData->taskId);
    int32_t ret = napi_send_event(
        this->env_,
        [ptr]() {
            uint32_t paramNumber = NapiUtils::ONE_ARG;
            napi_handle_scope scope = nullptr;
            napi_status status = napi_open_handle_scope(ptr->listener->env_, &scope);
            if (status != napi_ok || scope == nullptr) {
                REQUEST_HILOGE("OnNotifyDataReceive napi_scope failed");
                delete ptr;
                return;
            }
            if (ptr->notifyData->type == SubscribeType::COMPLETED || ptr->notifyData->type == SubscribeType::FAILED) {
                REQUEST_HILOGD("DoJSTask: %{public}s tid %{public}d",
                    SubscribeTypeToString(ptr->notifyData->type).c_str(), ptr->notifyData->taskId);
            }
            ptr->listener->DoJSTask(ptr->notifyData);
            napi_close_handle_scope(ptr->listener->env_, scope);
            delete ptr;
        },
        napi_eprio_high,
        "request:download|downloadfile|upload|uploadfile|agent.create");
    if (ret != napi_ok) {
        REQUEST_HILOGE("napi_send_event failed: %{public}d", ret);
        delete ptr;
    }
}

void JSNotifyDataListener::OnFaultsReceive(const std::shared_ptr<int32_t> &tid,
    const std::shared_ptr<SubscribeType> &type, const std::shared_ptr<Reason> &reason)
{
    ReasonDataPtr *ptr = new (std::nothrow) ReasonDataPtr;
    if (ptr == nullptr) {
        REQUEST_HILOGE("ReasonDataPtr new failed");
        return;
    }
    ptr->listener = shared_from_this();
    ptr->reason = reason;
    ptr->tid = tid;
    int32_t ret = napi_send_event(
        this->env_,
        [ptr, this]() {
            uint32_t paramNumber = NapiUtils::ONE_ARG;
            napi_handle_scope scope = nullptr;
            napi_status status = napi_open_handle_scope(ptr->listener->env_, &scope);
            if (status != napi_ok || scope == nullptr) {
                REQUEST_HILOGE("OnFaultsReceive napi_scope failed");
                delete ptr;
                return;
            }
            napi_value value = NapiUtils::Convert2JSValue(ptr->listener->env_, *(ptr->reason));
            JsTask::ClearTaskTemp(std::to_string(*(ptr->tid)), true, false, false);
            this->OnMessageReceive(&value, paramNumber);
            napi_close_handle_scope(ptr->listener->env_, scope);
            delete ptr;
        },
        napi_eprio_high,
        "request:task.on");
    if (ret != napi_ok) {
        REQUEST_HILOGE("napi_send_event failed: %{public}d", ret);
        delete ptr;
    }
}

void JSNotifyDataListener::OnWaitReceive(std::int32_t taskId, WaitingReason reason)
{
    REQUEST_HILOGI(
        "Notify wait, tid %{public}d, reason: %{public}d", taskId, static_cast<int32_t>(reason));
    int32_t ret = napi_send_event(
        this->env_,
        [me = shared_from_this(), taskId, reason]() {
            uint32_t paramNumber = NapiUtils::ONE_ARG;
            napi_handle_scope scope = nullptr;
            napi_status status = napi_open_handle_scope(me->env_, &scope);
            if (status != napi_ok || scope == nullptr) {
                REQUEST_HILOGE("OnWaitReceive napi_scope failed");
                return;
            }
            napi_value value = NapiUtils::Convert2JSValue(me->env_, reason);
            me->OnMessageReceive(&value, paramNumber);
            napi_close_handle_scope(me->env_, scope);
        },
        napi_eprio_high,
        "request:task.on");
    if (ret != napi_ok) {
        REQUEST_HILOGE("napi_send_event failed: %{public}d", ret);
    }
}

} // namespace OHOS::Request