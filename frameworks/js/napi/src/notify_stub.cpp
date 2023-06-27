/*
 * Copyright (c) 2023 Huawei Device Co., Ltd.
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

#include "notify_stub.h"
#include "request_event.h"
#include "log.h"
#include "parcel_helper.h"
#include "download_server_ipc_interface_code.h"

namespace OHOS::Request {
int32_t NotifyStub::OnRemoteRequest(uint32_t code, MessageParcel &data, MessageParcel &reply,
    MessageOption &option)
{
    auto descriptorToken = data.ReadInterfaceToken();
    if (descriptorToken != GetDescriptor()) {
        REQUEST_HILOGE("Remote descriptor not the same as local descriptor.");
        return IPCObjectStub::OnRemoteRequest(code, data, reply, option);
    }
    switch (code) {
        case static_cast<uint32_t>(RequestNotifyInterfaceCode::REQUEST_NOTIFY):
            OnCallBack(data);
            break;
        case static_cast<uint32_t>(RequestNotifyInterfaceCode::REQUEST_DONE_NOTIFY):
            OnDone(data);
            break;
        default:
            REQUEST_HILOGE("Default value received, check needed.");
            return IPCObjectStub::OnRemoteRequest(code, data, reply, option);
    }
    return ERR_NONE;
}

void NotifyStub::OnCallBack(MessageParcel &data)
{
    REQUEST_HILOGD("Receive callback");
    std::string type = data.ReadString();
    std::string tid = data.ReadString();
    NotifyData notifyData;
    notifyData.progress.state = static_cast<State>(data.ReadUint32());
    notifyData.progress.index = data.ReadUint32();
    notifyData.progress.processed = data.ReadUint64();
    notifyData.progress.totalProcessed = data.ReadUint64();
    data.ReadInt64Vector(&notifyData.progress.sizes);
    uint32_t size = data.ReadUint32();
    if (size > data.GetReadableBytes()) {
        REQUEST_HILOGE("Size exceeds the upper limit, size = %{public}d", size);
        return;
    }
    for (uint32_t i = 0; i < size; i++) {
        std::string key = data.ReadString();
        notifyData.progress.extras[key] = data.ReadString();
    }
    notifyData.action = static_cast<Action>(data.ReadUint32());
    notifyData.version = static_cast<Version>(data.ReadUint32());
    size = data.ReadUint32();
    if (size > data.GetReadableBytes()) {
        REQUEST_HILOGE("Size exceeds the upper limit, size = %{public}d", size);
        return;
    }
    for (uint32_t i = 0; i < size; i++) {
        TaskState taskState;
        taskState.path = data.ReadString();
        taskState.responseCode = data.ReadUint32();
        taskState.message = data.ReadString();
        notifyData.taskStates.push_back(taskState);
    }
    RequestCallBack(type, tid, notifyData);
}

void NotifyStub::RequestCallBack(const std::string &type, const std::string &tid, const NotifyData &notifyData)
{
    Notify notify;
    if (notifyData.version != Version::API10) {
        auto func = notifyData.action == Action::DOWNLOAD ? GetDownloadNotify : GetUploadNotify;
        func(type, notifyData, notify);
    } else {
        REQUEST_HILOGD("Receive API10 callback");
        notify.type = PROGRESS_CALLBACK;
        notify.progress = notifyData.progress;
    }

    auto item = JsTask::taskMap_.find(tid);
    if (item == JsTask::taskMap_.end()) {
        REQUEST_HILOGE("Task ID not found");
        return;
    }
    auto task = item->second;
    std::string key = type + tid;
    auto it = task->listenerMap_.find(key);
    if (it == task->listenerMap_.end()) {
        REQUEST_HILOGE("Unregistered %{public}s callback", type.c_str());
        return;
    }
    for (const auto &callback : it->second) {
        callback->CallBack(notify);
    }
}

void NotifyStub::GetDownloadNotify(const std::string &type, const NotifyData &notifyData, Notify &notify)
{
    REQUEST_HILOGD("Get download notify data");
    notify.type = DATA_CALLBACK;
    if (type == "progress") {
        notify.data.push_back(notifyData.progress.processed);
        if (!notifyData.progress.sizes.empty()) {
            notify.data.push_back(notifyData.progress.sizes[0]);
        }
    } else if (type == "fail") {
        if (notifyData.taskStates.empty()) {
            return;
        }
        int64_t failedReason;
        auto it = RequestEvent::failMap_.find(static_cast<Reason>(notifyData.taskStates[0].responseCode));
        if (it != RequestEvent::failMap_.end()) {
            failedReason = it->second;
        } else {
            failedReason = static_cast<int64_t>(ERROR_UNKNOWN);
        }
        notify.data.push_back(failedReason);
    }
}

void NotifyStub::GetUploadNotify(const std::string &type, const NotifyData &notifyData, Notify &notify)
{
    REQUEST_HILOGD("Get upload notify data");
    if (type == "complete" || type == "fail") {
        notify.type = TASK_STATE_CALLBACK;
        notify.taskStates = notifyData.taskStates;
    } else if (type == "progress") {
        notify.type = DATA_CALLBACK;
        int64_t size = 0;
        for (const auto &i : notifyData.progress.sizes) {
            size += i;
        }
        notify.data.push_back(notifyData.progress.totalProcessed);
        notify.data.push_back(size);
    } else {
        notify.type = HEADER_CALLBACK;
        notify.header = notifyData.progress.extras;
    }
}

void NotifyStub::OnDone(MessageParcel &data)
{
    auto taskInfo = std::make_shared<TaskInfo>();
    ParcelHelper::UnMarshal(data, *taskInfo);
    REQUEST_HILOGI("task %{public}s done", taskInfo->tid.c_str());
    RequestEvent::AddCache(taskInfo->tid, taskInfo);
}
} // namespace OHOS::Request
