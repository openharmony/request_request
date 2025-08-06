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

#include <cstdint>
#include <string>

#include "constant.h"
#include "iremote_broker.h"
#include "notify_interface.h"
#include "request_common.h"

namespace OHOS::Request {
class RequestServiceInterface : public IRemoteBroker {
public:
    DECLARE_INTERFACE_DESCRIPTOR(u"OHOS.Download.RequestServiceInterface");
    virtual ExceptionErrorCode CreateTasks(const std::vector<Config> &configs, std::vector<TaskRet> &rets) = 0;
    virtual ExceptionErrorCode StartTasks(
        const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets) = 0;
    virtual ExceptionErrorCode StopTasks(
        const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets) = 0;
    virtual ExceptionErrorCode ResumeTasks(
        const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets) = 0;
    virtual ExceptionErrorCode RemoveTasks(
        const std::vector<std::string> &tids, const Version version, std::vector<ExceptionErrorCode> &rets) = 0;
    virtual ExceptionErrorCode PauseTasks(
        const std::vector<std::string> &tids, const Version version, std::vector<ExceptionErrorCode> &rets) = 0;
    virtual ExceptionErrorCode QueryTasks(const std::vector<std::string> &tids, std::vector<TaskInfoRet> &rets) = 0;
    virtual ExceptionErrorCode ShowTasks(const std::vector<std::string> &tids, std::vector<TaskInfoRet> &rets) = 0;
    virtual ExceptionErrorCode TouchTasks(
        const std::vector<TaskIdAndToken> &tidTokens, std::vector<TaskInfoRet> &rets) = 0;
    virtual ExceptionErrorCode SetMaxSpeeds(
        const std::vector<SpeedConfig> &speedConfig, std::vector<ExceptionErrorCode> &rets) = 0;

    virtual ExceptionErrorCode SetMode(const std::string &tid, const Mode mode) = 0;
    virtual ExceptionErrorCode DisableTaskNotification(
        const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets) = 0;

    virtual int32_t Create(const Config &config, std::string &taskId) = 0;
    virtual int32_t GetTask(const std::string &tid, const std::string &token, Config &config) = 0;
    virtual int32_t Start(const std::string &tid) = 0;
    virtual int32_t Pause(const std::string &tid, const Version version) = 0;
    virtual int32_t QueryMimeType(const std::string &tid, std::string &mimeType) = 0;
    virtual int32_t Remove(const std::string &tid, const Version version) = 0;
    virtual int32_t Resume(const std::string &tid) = 0;
    virtual int32_t SetMaxSpeed(const std::string &tid, const int64_t maxSpeed) = 0;

    virtual int32_t Stop(const std::string &tid) = 0;
    virtual int32_t Query(const std::string &tid, TaskInfo &info) = 0;
    virtual int32_t Touch(const std::string &tid, const std::string &token, TaskInfo &info) = 0;
    virtual int32_t Search(const Filter &filter, std::vector<std::string> &tids) = 0;
    virtual int32_t Show(const std::string &tid, TaskInfo &info) = 0;

    virtual int32_t OpenChannel(int32_t &sockFd) = 0;
    virtual int32_t Subscribe(const std::string &taskId) = 0;
    virtual int32_t Unsubscribe(const std::string &taskId) = 0;
    virtual int32_t SubRunCount(const sptr<NotifyInterface> &listener) = 0;
    virtual int32_t UnsubRunCount() = 0;
    virtual int32_t CreateGroup(
        std::string &gid, const bool gauge, Notification &notification) = 0;
    virtual int32_t AttachGroup(const std::string &gid, const std::vector<std::string> &tids) = 0;
    virtual int32_t DeleteGroup(const std::string &gid) = 0;
};
} // namespace OHOS::Request
#endif // DOWNLOAD_SERVICE_INTERFACE_H