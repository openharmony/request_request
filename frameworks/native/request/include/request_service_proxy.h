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

#ifndef DOWNLOAD_SERVICE_PROXY_H
#define DOWNLOAD_SERVICE_PROXY_H

#include <cstdint>

#include "iremote_proxy.h"
#include "notify_interface.h"
#include "request_common.h"
#include "request_service_interface.h"

namespace OHOS::Request {

constexpr int REMOTE_DIED_ERROR = 29189;

class RequestServiceProxy : public IRemoteProxy<RequestServiceInterface> {
public:
    explicit RequestServiceProxy(const sptr<IRemoteObject> &object);
    ~RequestServiceProxy() = default;
    DISALLOW_COPY_AND_MOVE(RequestServiceProxy);

    ExceptionErrorCode CreateTasks(const std::vector<Config> &configs, std::vector<TaskRet> &rets) override;
    ExceptionErrorCode StartTasks(const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets) override;
    ExceptionErrorCode StopTasks(const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets) override;
    ExceptionErrorCode ResumeTasks(
        const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets) override;

    ExceptionErrorCode PauseTasks(
        const std::vector<std::string> &tids, const Version version, std::vector<ExceptionErrorCode> &rets) override;
    ExceptionErrorCode RemoveTasks(
        const std::vector<std::string> &tids, const Version version, std::vector<ExceptionErrorCode> &rets) override;

    ExceptionErrorCode QueryTasks(const std::vector<std::string> &tids, std::vector<TaskInfoRet> &rets) override;
    ExceptionErrorCode ShowTasks(const std::vector<std::string> &tids, std::vector<TaskInfoRet> &rets) override;
    ExceptionErrorCode TouchTasks(
        const std::vector<TaskIdAndToken> &tidTokens, std::vector<TaskInfoRet> &rets) override;
    ExceptionErrorCode SetMaxSpeeds(
        const std::vector<SpeedConfig> &speedConfig, std::vector<ExceptionErrorCode> &rets) override;
    ExceptionErrorCode SetMode(const std::string &tid, const Mode mode) override;
    ExceptionErrorCode DisableTaskNotification(
        const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets) override;

    int32_t Create(const Config &config, std::string &tid) override;
    int32_t GetTask(const std::string &tid, const std::string &token, Config &config) override;
    int32_t Start(const std::string &tid) override;
    int32_t Pause(const std::string &tid, const Version version) override;
    int32_t QueryMimeType(const std::string &tid, std::string &mimeType) override;
    int32_t Remove(const std::string &tid, const Version version) override;
    int32_t Resume(const std::string &tid) override;
    int32_t SetMaxSpeed(const std::string &tid, const int64_t maxSpeed) override;

    int32_t Stop(const std::string &tid) override;
    int32_t Query(const std::string &tid, TaskInfo &info) override;
    int32_t Touch(const std::string &tid, const std::string &token, TaskInfo &info) override;
    int32_t Search(const Filter &filter, std::vector<std::string> &tids) override;
    int32_t Show(const std::string &tid, TaskInfo &info) override;

    int32_t OpenChannel(int32_t &sockFd) override;
    int32_t Subscribe(const std::string &tid) override;
    int32_t Unsubscribe(const std::string &tid) override;
    int32_t SubRunCount(const sptr<NotifyInterface> &listener) override;
    int32_t UnsubRunCount() override;

    int32_t CreateGroup(std::string &gid, const bool gauge, Notification &notification) override;
    int32_t AttachGroup(const std::string &gid, const std::vector<std::string> &tids) override;
    int32_t DeleteGroup(const std::string &gid) override;

private:
    static void GetVectorData(const Config &config, MessageParcel &data);
    static void WriteConfigData(const Config &config, MessageParcel &data);
    static inline BrokerDelegator<RequestServiceProxy> delegator_;
};
} // namespace OHOS::Request
#endif // DOWNLOAD_SERVICE_PROXY_H
