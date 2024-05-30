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
    info.priority = data.ReadUint32();
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

void ParcelHelper::UnMarshalConfig(MessageParcel &data, Config &config)
{
    config.action = static_cast<Action>(data.ReadUint32());
    config.mode = static_cast<Mode>(data.ReadUint32());
    config.overwrite = data.ReadBool();
    config.network = static_cast<Network>(data.ReadUint32());
    config.metered = data.ReadBool();
    config.roaming = data.ReadBool();
    config.retry = data.ReadBool();
    config.redirect = data.ReadBool();
    config.index = data.ReadUint32();
    config.begins = data.ReadInt64();
    config.ends = data.ReadInt64();
    config.gauge = data.ReadBool();
    config.precise = data.ReadBool();
    config.priority = data.ReadUint32();
    config.background = data.ReadBool();
    config.bundleName = data.ReadString();
    config.url = data.ReadString();
    config.title = data.ReadString();
    config.description = data.ReadString();
    config.method = data.ReadString();
    // read headers
    if (!UnMarshalConfigHeaders(data, config)) {
        return;
    }
    config.data = data.ReadString();
    config.token = data.ReadString();
    // read extras
    if (!UnMarshalConfigExtras(data, config)) {
        return;
    }
    config.version = static_cast<Version>(data.ReadUint32());
    // read form_items
    if (!UnMarshalConfigFormItem(data, config)) {
        return;
    }
    // read file_specs
    if (!UnMarshalConfigFileSpec(data, config)) {
        return;
    }
    // read body_file_names
    if (!UnMarshalConfigBodyFileName(data, config)) {
        return;
    }
}

bool ParcelHelper::UnMarshalConfigHeaders(MessageParcel &data, Config &config)
{
    uint32_t headerLen = data.ReadUint32();
    if (headerLen > data.GetReadableBytes()) {
        REQUEST_HILOGE("Size exceeds the upper limit, size = %{public}u", headerLen);
        return false;
    }
    for (uint32_t i = 0; i < headerLen; i++) {
        std::string key = data.ReadString();
        config.headers[key] = data.ReadString();
    }
    return true;
}

bool ParcelHelper::UnMarshalConfigExtras(MessageParcel &data, Config &config)
{
    uint32_t extraLen = data.ReadUint32();
    if (extraLen > data.GetReadableBytes()) {
        REQUEST_HILOGE("Size exceeds the upper limit, size = %{public}u", extraLen);
        return false;
    }
    for (uint32_t i = 0; i < extraLen; i++) {
        std::string key = data.ReadString();
        config.extras[key] = data.ReadString();
    }
    return true;
}

bool ParcelHelper::UnMarshalConfigFormItem(MessageParcel &data, Config &config)
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
        config.forms.push_back(form);
    }
    return true;
}

bool ParcelHelper::UnMarshalConfigFileSpec(MessageParcel &data, Config &config)
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
        config.files.push_back(file);
    }
    return true;
}

bool ParcelHelper::UnMarshalConfigBodyFileName(MessageParcel &data, Config &config)
{
    uint32_t size = data.ReadUint32();
    if (size > data.GetReadableBytes()) {
        REQUEST_HILOGE("Size exceeds the upper limit, size = %{public}u", size);
        return false;
    }
    for (uint32_t i = 0; i < size; i++) {
        std::string name = data.ReadString();
        config.bodyFds.push_back(0);
        config.bodyFileNames.push_back(name);
    }
    return true;
}
} // namespace Request
} // namespace OHOS