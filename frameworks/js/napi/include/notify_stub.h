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

#ifndef DOWNLOAD_NOTIFY_STUB_H
#define DOWNLOAD_NOTIFY_STUB_H

#include <fstream>
#include <memory>
#include <unistd.h>

#include "iremote_stub.h"
#include "js_common.h"
#include "notify_interface.h"
#include "visibility.h"

namespace OHOS::Request {
class NotifyStub : public IRemoteStub<NotifyInterface> {
public:
    REQUEST_API NotifyStub() = default;
    REQUEST_API ~NotifyStub() override = default;
    REQUEST_API int32_t OnRemoteRequest(
        uint32_t code, MessageParcel &data, MessageParcel &reply, MessageOption &option) override;
    REQUEST_API void RequestCallBack(const std::string &type, const std::string &tid, const NotifyData &notifyData);

private:
    void OnCallBack(MessageParcel &data);
    bool IsHeaderReceive(const std::string &type, const NotifyData &notifyData);
    static void GetDownloadNotify(const std::string &type, const NotifyData &notifyData, Notify &notify);
    static void GetUploadNotify(const std::string &type, const NotifyData &notifyData, Notify &notify);
};
} // namespace OHOS::Request
#endif // DOWNLOAD_NOTIFY_STUB_H