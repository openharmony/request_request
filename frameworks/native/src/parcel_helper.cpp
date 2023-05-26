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
namespace OHOS {
namespace Request {
void ParcelHelper::UnMarshal(MessageParcel &data, TaskInfo &info)
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
    info.ctime = data.ReadString();
    info.mtime = data.ReadString();
    info.data = data.ReadString();
    uint32_t size = data.ReadUint32();
    for (uint32_t i = 0; i < size; i++) {
        FormItem form;
        form.name = data.ReadString();
        form.value = data.ReadString();
        info.forms.push_back(form);
    }
    size = data.ReadUint32();
    for (uint32_t i = 0; i < size; i++) {
        FileSpec file;
        file.name = data.ReadString();
        file.uri = data.ReadString();
        file.filename = data.ReadString();
        file.type = data.ReadString();
        info.files.push_back(file);
    }
    info.progress.state = static_cast<State>(data.ReadUint32());
    info.progress.index = data.ReadUint32();
    info.progress.processed = data.ReadInt64();
    data.ReadInt64Vector(&info.progress.sizes);
    size = data.ReadUint32();
    for (uint32_t i = 0; i < size; i++) {
        info.progress.extras[data.ReadString()] = data.ReadString();
    }
    size = data.ReadUint32();
    for (uint32_t i = 0; i < size; i++) {
        info.extras[data.ReadString()] = data.ReadString();
    }
}
} // namespace Request
} // namespace OHOS