/*
 * Copyright (C) 2022 Huawei Device Co., Ltd.
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
#include "download_notify_proxy.h"

#include "log.h"
#include <stdint.h>
#include "message_option.h"
#include "message_parcel.h"

namespace OHOS::Request::Download {
DownloadNotifyProxy::DownloadNotifyProxy(const sptr<IRemoteObject> &impl) : IRemoteProxy<DownloadNotifyInterface>(impl)
{
}

void DownloadNotifyProxy::OnCallBack(MessageParcel &data)
{
    DOWNLOAD_HILOGD("DownloadNotifyProxy::OnCallBack Start");
    DOWNLOAD_HILOGD("data should be filled within service module");
    MessageParcel realData;
    MessageParcel reply;
    MessageOption option;

    if (!realData.WriteInterfaceToken(DownloadNotifyProxy::GetDescriptor())) {
        DOWNLOAD_HILOGE("write descriptor failed");
        return;
    }
    uint32_t argv1 = data.ReadUint32();
    uint32_t argv2 = data.ReadUint32();
    DOWNLOAD_HILOGD("notification's argument:[%{public}d, %{public}d]", argv1, argv2);
    realData.WriteUint32(argv1);
    realData.WriteUint32(argv2);

    int error = Remote()->SendRequest(DOWNLOAD_NOTIFY, realData, reply, option);
    if (error != 0) {
        DOWNLOAD_HILOGE("SendRequest failed, error %{public}d", error);
    }
    DOWNLOAD_HILOGD("DownloadNotifyProxy::OnCallBack End");
}

/*
void DownloadNotifyProxy::OnCallBack(const std::string &event)
{
    DOWNLOAD_HILOGD("DownloadNotifyProxy::OnCallBack Start");
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    if (!data.WriteInterfaceToken(DownloadNotifyProxy::GetDescriptor())) {
        DOWNLOAD_HILOGE("write descriptor failed");
        return;
    }
    if (!data.WriteString(event)) {
        DOWNLOAD_HILOGE("write string failed");
        return;
    }
    int error = Remote()->SendRequest(ONCALLBACK_VOID, data, reply, option);
    if (error != 0) {
        DOWNLOAD_HILOGE("SendRequest failed, error %{public}d", error);
    }
    DOWNLOAD_HILOGD("DownloadNotifyProxy::OnCallBack End");
}

void DownloadNotifyProxy::OnCallBack(const std::string &event, int result)
{
    DOWNLOAD_HILOGD("DownloadNotifyProxy::OnCallBack Start");
    DOWNLOAD_HILOGD("event =%{public}s, result = %{public}d", event.c_str(), result);
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    if (!data.WriteInterfaceToken(DownloadNotifyProxy::GetDescriptor())) {
        DOWNLOAD_HILOGE("write descriptor failed");
        return;
    }
    DOWNLOAD_HILOGD("DownloadNotifyProxy::OnCallBack Start1");
    if (!data.WriteString(event)) {
        DOWNLOAD_HILOGE("write string failed");
        return;
    }
    DOWNLOAD_HILOGD("DownloadNotifyProxy::OnCallBack Start2");
    if (!data.WriteInt32(result)) {
        DOWNLOAD_HILOGE("write bool failed");
        return;
    }
    DOWNLOAD_HILOGD("DownloadNotifyProxy::OnCallBack Start3");
    int error = Remote()->SendRequest(ONCALLBACK_INT, data, reply, option);
    if (error != 0) {
        DOWNLOAD_HILOGE("SendRequest failed, error %{public}d", error);
    }
    DOWNLOAD_HILOGD("DownloadNotifyProxy::OnCallBack End");
}
*/
} // namespace OHOS::Request::Download
