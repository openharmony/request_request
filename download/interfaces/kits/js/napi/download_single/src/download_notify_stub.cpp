/*
 * Copyright (c) 2022 Huawei Device Co., Ltd.
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

#include "download_notify_stub.h"

#include "log.h"
#include "download_common.h"

namespace OHOS::Request::Download {
int32_t DownloadNotifyStub::OnRemoteRequest(
    uint32_t code, MessageParcel &data, MessageParcel &reply, MessageOption &option)
{
    auto descriptorToken = data.ReadInterfaceToken();
    if (descriptorToken != GetDescriptor()) {
        DOWNLOAD_HILOGE("Remote descriptor not the same as local descriptor.");
        return E_DOWNLOAD_TRANSACT_ERROR;
    }
    DOWNLOAD_HILOGD("DownloadNotifyStub  code----> %{public}u", code);
    switch (code) {
        case DOWNLOAD_NOTIFY: {
            OnCallBack(data);
            break;
        }
        default: {
            return OHOS::UNKNOWN_TRANSACTION;
        }
    }
    return E_DOWNLOAD_OK;
}

void DownloadNotifyStub::OnCallBack(MessageParcel &data)
{
    DOWNLOAD_HILOGD("Receive callback");
    std::vector<int64_t> params;
    if (!data.ReadInt64Vector(&params) || params.empty()) {
        DOWNLOAD_HILOGE("read params fail");
        return;
    }
    CallBack(params);
}

} // namespace OHOS::Request::Download
