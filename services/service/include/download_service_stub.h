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

#ifndef DOWNLOAD_SERVICE_STUB_H
#define DOWNLOAD_SERVICE_STUB_H

#include <stdint.h>
#include "message_option.h"
#include "message_parcel.h"
#include "refbase.h"
#include "iremote_stub.h"
#include "download_service_interface.h"

namespace OHOS::Request::Download {
class DownloadServiceStub : public IRemoteStub<DownloadServiceInterface> {
public:
    int32_t OnRemoteRequest(uint32_t code, MessageParcel &data, MessageParcel &reply, MessageOption &option) override;

private:
    bool OnRequest(MessageParcel &data, MessageParcel &reply);
    bool OnPause(MessageParcel &data, MessageParcel &reply);
    bool OnQuery(MessageParcel &data, MessageParcel &reply);
    bool OnQueryMimeType(MessageParcel &data, MessageParcel &reply);
    bool OnRemove(MessageParcel &data, MessageParcel &reply);
    bool OnResume(MessageParcel &data, MessageParcel &reply);
    bool OnEventOn(MessageParcel &data, MessageParcel &reply);
    bool OnEventOff(MessageParcel &data, MessageParcel &reply);
    bool OnCheckPermission(MessageParcel &data, MessageParcel &reply);
};
} // namespace OHOS::Request::Download
#endif // DOWNLOAD_SERVICE_STUB_H
