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
#include "message_option.h"
#include "message_parcel.h"

namespace OHOS::Request::Download {
DownloadNotifyProxy::DownloadNotifyProxy(const sptr<IRemoteObject> &impl) : IRemoteProxy<DownloadNotifyInterface>(impl)
{
}

void DownloadNotifyProxy::CallBack(const std::vector<int64_t> &params)
{
    DOWNLOAD_HILOGD("DownloadNotifyProxy::OnCallBack Start");
    DOWNLOAD_HILOGD("data should be filled within service module");
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    if (!data.WriteInterfaceToken(DownloadNotifyProxy::GetDescriptor())) {
        DOWNLOAD_HILOGE("write descriptor failed");
        return;
    }
    data.WriteInt64Vector(params);

    int error = Remote()->SendRequest(DOWNLOAD_NOTIFY, data, reply, option);
    if (error != 0) {
        DOWNLOAD_HILOGE("SendRequest failed, error %{public}d", error);
    }
    DOWNLOAD_HILOGD("DownloadNotifyProxy::OnCallBack End");
}
} // namespace OHOS::Request::Download
