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

#ifndef DOWNLOAD_SERVICE_INTERFACE_H
#define DOWNLOAD_SERVICE_INTERFACE_H

#include <string>

#include "download_config.h"
#include "download_info.h"
#include "download_notify_interface.h"
#include "iremote_broker.h"
#include "constant.h"

namespace OHOS::Request::Download {
class DownloadServiceInterface : public IRemoteBroker {
public:
    DECLARE_INTERFACE_DESCRIPTOR(u"OHOS.Download.DownloadServiceInterface");
    virtual int32_t Request(const DownloadConfig &config, ExceptionError &error) = 0;
    virtual bool Pause(uint32_t taskId) = 0;
    virtual bool Query(uint32_t taskId, DownloadInfo &info) = 0;
    virtual bool QueryMimeType(uint32_t taskId, std::string &mimeType) = 0;
    virtual bool Remove(uint32_t taskId) = 0;
    virtual bool Resume(uint32_t taskId) = 0;
    virtual bool On(uint32_t taskId, const std::string &type, const sptr<DownloadNotifyInterface> &listener) = 0;
    virtual bool Off(uint32_t taskId, const std::string &type) = 0;
    virtual bool CheckPermission() = 0;
};

enum {
    CMD_REQUEST,
    CMD_PAUSE,
    CMD_QUERY,
    CMD_QUERYMIMETYPE,
    CMD_REMOVE,
    CMD_RESUME,
    CMD_ON,
    CMD_OFF,
    CMD_CHECKPERMISSION,
};
} // namespace OHOS::Request::Download
#endif // DOWNLOAD_SERVICE_INTERFACE_H