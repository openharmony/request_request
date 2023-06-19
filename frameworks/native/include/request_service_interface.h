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

#ifndef DOWNLOAD_SERVICE_INTERFACE_H
#define DOWNLOAD_SERVICE_INTERFACE_H

#include <string>

#include "notify_interface.h"
#include "iremote_broker.h"
#include "constant.h"
#include "js_common.h"

namespace OHOS::Request {
class RequestServiceInterface : public IRemoteBroker {
public:
    DECLARE_INTERFACE_DESCRIPTOR(u"OHOS.Download.RequestServiceInterface");
    virtual int32_t Create(const Config &config, int32_t &taskId, sptr<NotifyInterface> listener) = 0;
    virtual int32_t Start(const std::string &tid) = 0;
    virtual int32_t Pause(const std::string &tid, Version version) = 0;
    virtual int32_t QueryMimeType(const std::string &tid, std::string &mimeType) = 0;
    virtual int32_t Remove(const std::string &tid, Version version) = 0;
    virtual int32_t Resume(const std::string &tid) = 0;

    virtual int32_t Stop(const std::string &tid) = 0;
    virtual int32_t Show(const std::string &tid, TaskInfo &info) = 0;
    virtual int32_t Touch(const std::string &tid, const std::string &token, TaskInfo &info) = 0;
    virtual int32_t Search(const Filter &filter, std::vector<std::string> &tids) = 0;
    virtual int32_t Query(const std::string &tid, TaskInfo &info, Version version) = 0;
    virtual int32_t Clear(const std::vector<std::string> &tids, std::vector<std::string> &res) =0;

    virtual int32_t On(const std::string &type, const std::string &tid,
        const sptr<NotifyInterface> &listener) = 0;
    virtual int32_t Off(const std::string &type, const std::string &tid) = 0;
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
    CMD_START,
    CMD_STOP,
    CMD_SHOW,
    CMD_TOUCH,
    CMD_SEARCH,
    CMD_CLEAR,
};
} // namespace OHOS::Request
#endif // DOWNLOAD_SERVICE_INTERFACE_H