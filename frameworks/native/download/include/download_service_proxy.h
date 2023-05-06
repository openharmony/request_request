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

#ifndef DOWNLOAD_SERVICE_PROXY_H
#define DOWNLOAD_SERVICE_PROXY_H

#include "iremote_proxy.h"

#include "download_notify_interface.h"
#include "download_service_interface.h"

namespace OHOS::Request::Download {
class DownloadServiceProxy : public IRemoteProxy<DownloadServiceInterface> {
public:
    explicit DownloadServiceProxy(const sptr<IRemoteObject> &object);
    ~DownloadServiceProxy() = default;
    DISALLOW_COPY_AND_MOVE(DownloadServiceProxy);
    int32_t Request(const DownloadConfig &config, ExceptionError &error) override;
    bool Pause(uint32_t taskId) override;
    bool Query(uint32_t taskId, DownloadInfo &info) override;
    bool QueryMimeType(uint32_t taskId, std::string &mimeType) override;
    bool Remove(uint32_t taskId) override;
    bool Resume(uint32_t taskId) override;

    bool On(uint32_t taskId, const std::string &type, const sptr<DownloadNotifyInterface> &listener) override;
    bool Off(uint32_t taskId, const std::string &type) override;
    bool CheckPermission() override;

private:
    static bool IsPathValid(const std::string &filePath);
    static inline BrokerDelegator<DownloadServiceProxy> delegator_;
};
} // namespace OHOS::Request::Download
#endif // DOWNLOAD_SERVICE_PROXY_H
