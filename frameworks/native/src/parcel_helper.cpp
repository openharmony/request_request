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

#include "parcel_helper.h"

#include "log.h"

namespace OHOS {
namespace Request {
void ParcelHelper::UnMarshal(MessageParcel &data, TaskInfo &info)
{
    UnMarshalBase(data, info);
    if (!UnMarshalFormItem(data, info)) {
        return;
    }
    if (!UnMarshalFileSpec(data, info)) {
        return;
    }
    UnMarshalProgress(data, info);
    if (!UnMarshalMapProgressExtras(data, info)) {
        return;
    }
    if (!UnMarshalMapExtras(data, info)) {
        return;
    }
    info.version = static_cast<Version>(data.ReadUint32());
    if (!UnMarshalTaskState(data, info)) {
        return;
    }
}

void ParcelHelper::UnMarshalBase(MessageParcel &data, TaskInfo &info)
{
    info.gauge = data.ReadBool();
    info.retry = data.ReadBool();
    info.action = static_cast<Action>(data.ReadUint32());
    info.mode = static_cast<Mode>(data.ReadUint32());
    info.code = static_cast<Reason>(data.ReadUint32());
    info.tries = data.ReadUint32();
    info.uid = data.ReadString();
    info.bundle = data.ReadString();
    info.url = data.ReadString();
    info.tid = data.ReadString();
    info.title = data.ReadString();
    info.mimeType = data.ReadString();
    info.ctime = data.ReadUint64();
    info.mtime = data.ReadUint64();
    info.data = data.ReadString();
    info.description = data.ReadString();
}

bool ParcelHelper::UnMarshalFormItem(MessageParcel &data, TaskInfo &info)
{
    uint32_t size = data.ReadUint32();
    if (size > data.GetReadableBytes()) {
        REQUEST_HILOGE("Size exceeds the upper limit, size = %{public}u", size);
        return false;
    }
    for (uint32_t i = 0; i < size; i++) {
        FormItem form;
        form.name = data.ReadString();
        form.value = data.ReadString();
        info.forms.push_back(form);
    }
    return true;
}

bool ParcelHelper::UnMarshalFileSpec(MessageParcel &data, TaskInfo &info)
{
    uint32_t size = data.ReadUint32();
    if (size > data.GetReadableBytes()) {
        REQUEST_HILOGE("Size exceeds the upper limit, size = %{public}u", size);
        return false;
    }
    for (uint32_t i = 0; i < size; i++) {
        FileSpec file;
        file.name = data.ReadString();
        file.uri = data.ReadString();
        file.filename = data.ReadString();
        file.type = data.ReadString();
        info.files.push_back(file);
    }
    return true;
}

void ParcelHelper::UnMarshalProgress(MessageParcel &data, TaskInfo &info)
{
    info.progress.state = static_cast<State>(data.ReadUint32());
    info.progress.index = data.ReadUint32();
    info.progress.processed = data.ReadUint64();
    info.progress.totalProcessed = data.ReadUint64();
    data.ReadInt64Vector(&info.progress.sizes);
}

bool ParcelHelper::UnMarshalMapProgressExtras(MessageParcel &data, TaskInfo &info)
{
    uint32_t size = data.ReadUint32();
    if (size > data.GetReadableBytes()) {
        REQUEST_HILOGE("Size exceeds the upper limit, size = %{public}u", size);
        return false;
    }
    for (uint32_t i = 0; i < size; i++) {
        std::string key = data.ReadString();
        info.progress.extras[key] = data.ReadString();
    }
    return true;
}

bool ParcelHelper::UnMarshalMapExtras(MessageParcel &data, TaskInfo &info)
{
    uint32_t size = data.ReadUint32();
    if (size > data.GetReadableBytes()) {
        REQUEST_HILOGE("Size exceeds the upper limit, size = %{public}u", size);
        return false;
    }
    for (uint32_t i = 0; i < size; i++) {
        std::string key = data.ReadString();
        info.extras[key] = data.ReadString();
    }
    return true;
}

bool ParcelHelper::UnMarshalTaskState(MessageParcel &data, TaskInfo &info)
{
    uint32_t size = data.ReadUint32();
    if (size > data.GetReadableBytes()) {
        REQUEST_HILOGE("Size exceeds the upper limit, size = %{public}u", size);
        return false;
    }
    for (uint32_t i = 0; i < size; i++) {
        TaskState taskState;
        taskState.path = data.ReadString();
        taskState.responseCode = data.ReadUint32();
        taskState.message = data.ReadString();
        info.taskStates.push_back(taskState);
    }
    return true;
}
} // namespace Request
} // namespace OHOS