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
#include "request_service_proxy.h"

#include <fcntl.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

#include <cstdint>
#include <ctime>
#include <optional>

#include "constant.h"
#include "download_server_ipc_interface_code.h"
#include "iremote_broker.h"
#include "log.h"
#include "message_parcel.h"
#include "parcel_helper.h"
#include "request_common.h"
#include "request_running_task_count.h"
#include "sys_event.h"

namespace OHOS::Request {
using namespace OHOS::HiviewDFX;

RequestServiceProxy::RequestServiceProxy(const sptr<IRemoteObject> &object)
    : IRemoteProxy<RequestServiceInterface>(object)
{
}

ExceptionErrorCode RequestServiceProxy::CreateTasks(const std::vector<Config> &configs, std::vector<TaskRet> &rets)
{
    uint32_t len = static_cast<uint32_t>(configs.size());
    rets.resize(len, {
                         .code = ExceptionErrorCode::E_OTHER,
                     });
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteUint32(len);
    for (auto &config : configs) {
        WriteConfigData(config, data);
    }
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_REQUEST), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request CreateTasks, failed: %{public}d", ret);
        if (ret != REMOTE_DIED_ERROR) {
            SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, std::to_string(ret));
        }
        return ExceptionErrorCode::E_SERVICE_ERROR;
    }
    ExceptionErrorCode code = static_cast<ExceptionErrorCode>(reply.ReadInt32());
    if (code != ExceptionErrorCode::E_OK) {
        SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_01, std::to_string(code));
        REQUEST_HILOGE("End Request CreateTasks, failed: %{public}d", code);
        return code;
    }
    for (uint32_t i = 0; i < len; i++) {
        rets[i].code = static_cast<ExceptionErrorCode>(reply.ReadInt32());
        rets[i].tid = std::to_string(reply.ReadInt32());
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestServiceProxy::StartTasks(
    const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets)
{
    uint32_t len = static_cast<uint32_t>(tids.size());
    rets.resize(len, ExceptionErrorCode::E_OTHER);
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteUint32(len);
    for (const std::string &tid : tids) {
        data.WriteString(tid);
    }
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_START), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request StartTasks, failed: %{public}d", ret);
        if (ret != REMOTE_DIED_ERROR) {
            SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, std::to_string(ret));
        }
        return ExceptionErrorCode::E_SERVICE_ERROR;
    }
    ExceptionErrorCode code = static_cast<ExceptionErrorCode>(reply.ReadInt32());
    if (code != ExceptionErrorCode::E_OK) {
        REQUEST_HILOGE("End Request StartTasks, failed: %{public}d", code);
        SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_01, std::to_string(code));
        return code;
    }
    for (uint32_t i = 0; i < len; i++) {
        rets[i] = static_cast<ExceptionErrorCode>(reply.ReadInt32());
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestServiceProxy::StopTasks(
    const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets)
{
    uint32_t len = static_cast<uint32_t>(tids.size());
    rets.resize(len, ExceptionErrorCode::E_OTHER);
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteUint32(len);
    for (const std::string &tid : tids) {
        data.WriteString(tid);
    }
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_STOP), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request StopTasks, failed: %{public}d", ret);
        if (ret != REMOTE_DIED_ERROR) {
            SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, std::to_string(ret));
        }
        return ExceptionErrorCode::E_SERVICE_ERROR;
    }
    ExceptionErrorCode code = static_cast<ExceptionErrorCode>(reply.ReadInt32());
    if (code != ExceptionErrorCode::E_OK) {
        REQUEST_HILOGE("End Request StopTasks, failed: %{public}d", code);
        SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_01, std::to_string(code));
        return code;
    }
    for (uint32_t i = 0; i < len; i++) {
        rets[i] = static_cast<ExceptionErrorCode>(reply.ReadInt32());
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestServiceProxy::ResumeTasks(
    const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets)
{
    uint32_t len = static_cast<uint32_t>(tids.size());
    rets.resize(len, ExceptionErrorCode::E_OTHER);
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteUint32(len);
    for (const std::string &tid : tids) {
        data.WriteString(tid);
    }
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_RESUME), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request ResumeTasks, failed: %{public}d", ret);
        if (ret != REMOTE_DIED_ERROR) {
            SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, std::to_string(ret));
        }
        return ExceptionErrorCode::E_SERVICE_ERROR;
    }
    ExceptionErrorCode code = static_cast<ExceptionErrorCode>(reply.ReadInt32());
    if (code != ExceptionErrorCode::E_OK) {
        REQUEST_HILOGE("End Request ResumeTasks, failed: %{public}d", code);
        SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_01, std::to_string(code));
        return code;
    }
    for (uint32_t i = 0; i < len; i++) {
        rets[i] = static_cast<ExceptionErrorCode>(reply.ReadInt32());
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestServiceProxy::PauseTasks(
    const std::vector<std::string> &tids, const Version version, std::vector<ExceptionErrorCode> &rets)
{
    uint32_t len = static_cast<uint32_t>(tids.size());
    rets.resize(len, ExceptionErrorCode::E_OTHER);
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteUint32(static_cast<uint32_t>(version));
    data.WriteUint32(len);
    for (const std::string &tid : tids) {
        data.WriteString(tid);
    }
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_PAUSE), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request PauseTasks, failed: %{public}d", ret);
        if (ret != REMOTE_DIED_ERROR) {
            SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, std::to_string(ret));
        }
        return ExceptionErrorCode::E_SERVICE_ERROR;
    }
    ExceptionErrorCode code = static_cast<ExceptionErrorCode>(reply.ReadInt32());
    if (code != ExceptionErrorCode::E_OK) {
        REQUEST_HILOGE("End Request PauseTasks, failed: %{public}d", code);
        SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_01, std::to_string(code));
        return code;
    }
    for (uint32_t i = 0; i < len; i++) {
        rets[i] = static_cast<ExceptionErrorCode>(reply.ReadInt32());
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestServiceProxy::RemoveTasks(
    const std::vector<std::string> &tids, const Version version, std::vector<ExceptionErrorCode> &rets)
{
    uint32_t len = static_cast<uint32_t>(tids.size());
    rets.resize(len, ExceptionErrorCode::E_OTHER);
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteUint32(static_cast<uint32_t>(version));
    data.WriteUint32(len);
    for (const std::string &tid : tids) {
        data.WriteString(tid);
    }
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_REMOVE), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request RemoveTasks, failed: %{public}d", ret);
        if (ret != REMOTE_DIED_ERROR) {
            SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, std::to_string(ret));
        }
        return ExceptionErrorCode::E_SERVICE_ERROR;
    }
    ExceptionErrorCode code = static_cast<ExceptionErrorCode>(reply.ReadInt32());
    if (code != ExceptionErrorCode::E_OK) {
        REQUEST_HILOGE("End Request RemoveTasks, failed: %{public}d", code);
        SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_01, std::to_string(code));
        return code;
    }
    for (uint32_t i = 0; i < len; i++) {
        rets[i] = static_cast<ExceptionErrorCode>(reply.ReadInt32());
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestServiceProxy::QueryTasks(const std::vector<std::string> &tids, std::vector<TaskInfoRet> &rets)
{
    TaskInfoRet infoRet{ .code = ExceptionErrorCode::E_OTHER };
    uint32_t len = static_cast<uint32_t>(tids.size());
    rets.resize(len, infoRet);
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(RequestServiceProxy::GetDescriptor());
    data.WriteUint32(len);
    for (const std::string &tid : tids) {
        data.WriteString(tid);
    }
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_QUERY), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request QueryTasks, failed: %{public}d", ret);
        if (ret != REMOTE_DIED_ERROR) {
            SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, std::to_string(ret));
        }
        return ExceptionErrorCode::E_SERVICE_ERROR;
    }
    ExceptionErrorCode code = static_cast<ExceptionErrorCode>(reply.ReadInt32());
    if (code != ExceptionErrorCode::E_OK) {
        REQUEST_HILOGE("End Request QueryTasks, failed: %{public}d", code);
        SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_01, std::to_string(code));
        return code;
    }
    for (uint32_t i = 0; i < len; i++) {
        rets[i].code = static_cast<ExceptionErrorCode>(reply.ReadInt32());
        TaskInfo info;
        ParcelHelper::UnMarshal(reply, info);
        rets[i].info = info;
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestServiceProxy::ShowTasks(const std::vector<std::string> &tids, std::vector<TaskInfoRet> &rets)
{
    TaskInfoRet infoRet{ .code = ExceptionErrorCode::E_OTHER };
    uint32_t len = static_cast<uint32_t>(tids.size());
    rets.resize(len, infoRet);
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(RequestServiceProxy::GetDescriptor());
    data.WriteUint32(len);
    for (const std::string &tid : tids) {
        data.WriteString(tid);
    }
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_SHOW), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request ShowTasks, failed: %{public}d", ret);
        if (ret != REMOTE_DIED_ERROR) {
            SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, std::to_string(ret));
        }
        return ExceptionErrorCode::E_SERVICE_ERROR;
    }
    ExceptionErrorCode code = static_cast<ExceptionErrorCode>(reply.ReadInt32());
    if (code != ExceptionErrorCode::E_OK) {
        REQUEST_HILOGE("End Request ShowTasks, failed: %{public}d", code);
        SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_01, std::to_string(code));
        return code;
    }
    for (uint32_t i = 0; i < len; i++) {
        rets[i].code = static_cast<ExceptionErrorCode>(reply.ReadInt32());
        TaskInfo info;
        ParcelHelper::UnMarshal(reply, info);
        rets[i].info = info;
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestServiceProxy::TouchTasks(
    const std::vector<TaskIdAndToken> &tids, std::vector<TaskInfoRet> &rets)
{
    TaskInfoRet infoRet{ .code = ExceptionErrorCode::E_OTHER };
    uint32_t len = static_cast<uint32_t>(tids.size());
    rets.resize(len, infoRet);
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(RequestServiceProxy::GetDescriptor());
    data.WriteUint32(len);
    for (const auto &it : tids) {
        data.WriteString(it.tid);
        data.WriteString(it.token);
    }
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_TOUCH), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request TouchTasks, failed: %{public}d", ret);
        if (ret != REMOTE_DIED_ERROR) {
            SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, std::to_string(ret));
        }
        return ExceptionErrorCode::E_SERVICE_ERROR;
    }
    ExceptionErrorCode code = static_cast<ExceptionErrorCode>(reply.ReadInt32());
    if (ret != ExceptionErrorCode::E_OK) {
        REQUEST_HILOGE("End Request TouchTasks, failed: %{public}d", code);
        SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_01, std::to_string(code));
        return code;
    }
    for (uint32_t i = 0; i < len; i++) {
        rets[i].code = static_cast<ExceptionErrorCode>(reply.ReadInt32());
        TaskInfo info;
        ParcelHelper::UnMarshal(reply, info);
        rets[i].info = info;
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestServiceProxy::SetMaxSpeeds(
    const std::vector<SpeedConfig> &speedConfig, std::vector<ExceptionErrorCode> &rets)
{
    uint32_t len = static_cast<uint32_t>(speedConfig.size());
    rets.resize(len, ExceptionErrorCode::E_OTHER);
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteUint32(len);
    for (auto &tid_and_speed : speedConfig) {
        data.WriteString(tid_and_speed.tid);
        data.WriteInt64(tid_and_speed.maxSpeed);
    }
    int32_t ret =
        Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_SET_MAX_SPEED), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request SetMaxSpeeds, failed: %{public}d", ret);
        if (ret != REMOTE_DIED_ERROR) {
            SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, std::to_string(ret));
        }
        return ExceptionErrorCode::E_SERVICE_ERROR;
    }
    ExceptionErrorCode code = static_cast<ExceptionErrorCode>(reply.ReadInt32());
    if (code != ExceptionErrorCode::E_OK) {
        REQUEST_HILOGE("End Request SetMaxSpeeds, failed: %{public}d", code);
        SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_01, std::to_string(code));
        return code;
    }
    for (uint32_t i = 0; i < len; i++) {
        rets[i] = static_cast<ExceptionErrorCode>(reply.ReadInt32());
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestServiceProxy::SetMode(const std::string &tid, const Mode mode)
{
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteString(tid);
    data.WriteUint32(static_cast<uint32_t>(mode));
    int32_t ret =
        Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_SET_MODE), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End send SetMode request, failed: %{public}d", ret);
        if (ret != REMOTE_DIED_ERROR) {
            SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, std::to_string(ret));
        }
        return ExceptionErrorCode::E_SERVICE_ERROR;
    }
    ExceptionErrorCode code = static_cast<ExceptionErrorCode>(reply.ReadInt32());
    if (code != ExceptionErrorCode::E_OK) {
        REQUEST_HILOGE("End Request SetMode, failed: %{public}d", code);
        SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_01, std::to_string(code));
        return code;
    }
    return code;
}

ExceptionErrorCode RequestServiceProxy::DisableTaskNotification(
    const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets)
{
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteStringVector(tids);
    size_t length = tids.size();
    int32_t ret = Remote()->SendRequest(
        static_cast<uint32_t>(RequestInterfaceCode::CMD_DISABLE_TASK_NOTIFICATIONS), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End send SetMode request, failed: %{public}d", ret);
        if (ret != REMOTE_DIED_ERROR) {
            SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, std::to_string(ret));
        }
        return ExceptionErrorCode::E_SERVICE_ERROR;
    }

    for (size_t i = 0; i < length; i++) {
        rets.push_back(static_cast<ExceptionErrorCode>(reply.ReadInt32()));
    }
    return ExceptionErrorCode::E_OK;
}

void SerializeNotification(MessageParcel &data, const Notification &notification)
{
    if (notification.title != std::nullopt) {
        data.WriteBool(true);
        data.WriteString(*notification.title);
    } else {
        data.WriteBool(false);
    }
    if (notification.text != std::nullopt) {
        data.WriteBool(true);
        data.WriteString(*notification.text);
    } else {
        data.WriteBool(false);
    }
    if (notification.wantAgent != std::nullopt) {
        data.WriteBool(true);
        data.WriteString(*notification.wantAgent);
    } else {
        data.WriteBool(false);
    }
    data.WriteBool(notification.disable);
    data.WriteUint32(static_cast<uint32_t>(notification.visibility));
}

int32_t RequestServiceProxy::Create(const Config &config, std::string &tid)
{
    REQUEST_HILOGD("Request Create, tid: %{public}s", tid.c_str());
    std::vector<Config> configs = { config };
    std::vector<TaskRet> rets;
    int32_t ret = RequestServiceProxy::CreateTasks(configs, rets);
    if (ret != ExceptionErrorCode::E_OK) {
        REQUEST_HILOGE("End Request Create failed: %{public}d", ret);
        return ret;
    }
    if (rets[0].code == ExceptionErrorCode::E_OK || rets[0].code == ExceptionErrorCode::E_CHANNEL_NOT_OPEN) {
        REQUEST_HILOGD("End Request Create ok, tid: %{public}s", tid.c_str());
        tid = rets[0].tid;
    }
    return rets[0].code;
}

void RequestServiceProxy::WriteConfigData(const Config &config, MessageParcel &data)
{
    data.WriteUint32(static_cast<uint32_t>(config.action));
    data.WriteUint32(static_cast<uint32_t>(config.version));
    data.WriteUint32(static_cast<uint32_t>(config.mode));
    data.WriteUint32(static_cast<uint32_t>(config.bundleType));
    data.WriteBool(config.overwrite);
    data.WriteUint32(static_cast<uint32_t>(config.network));
    data.WriteBool(config.metered);
    data.WriteBool(config.roaming);
    data.WriteBool(config.retry);
    data.WriteBool(config.redirect);
    data.WriteBool(config.background);
    data.WriteBool(config.multipart);
    data.WriteUint32(config.index);
    data.WriteInt64(config.begins);
    data.WriteInt64(config.ends);
    data.WriteBool(config.gauge);
    data.WriteBool(config.precise);
    data.WriteUint32(config.priority);
    data.WriteInt64(config.minSpeed.speed);
    data.WriteInt64(config.minSpeed.duration);
    data.WriteUint64(config.timeout.connectionTimeout);
    data.WriteUint64(config.timeout.totalTimeout);
    data.WriteString(config.url);
    data.WriteString(config.title);
    data.WriteString(config.method);
    data.WriteString(config.token);
    data.WriteString(config.description);
    data.WriteString(config.data);
    data.WriteString(config.proxy);
    data.WriteString(config.certificatePins);
    GetVectorData(config, data);
    SerializeNotification(data, config.notification);
}

void RequestServiceProxy::GetVectorData(const Config &config, MessageParcel &data)
{
    data.WriteUint32(config.certsPath.size());
    for (const auto &cert : config.certsPath) {
        data.WriteString(cert);
    }

    data.WriteUint32(config.forms.size());
    for (const auto &form : config.forms) {
        data.WriteString(form.name);
        data.WriteString(form.value);
    }
    data.WriteUint32(config.files.size());
    for (const auto &file : config.files) {
        data.WriteString(file.name);
        data.WriteString(file.uri);
        data.WriteString(file.filename);
        data.WriteString(file.type);
        data.WriteBool(file.isUserFile);
        if (file.isUserFile) {
            data.WriteFileDescriptor(file.fd);
        }
    }

    // Response Body files.
    data.WriteUint32(config.bodyFileNames.size());
    for (const auto &name : config.bodyFileNames) {
        data.WriteString(name);
    }

    data.WriteUint32(config.headers.size());
    for (const auto &header : config.headers) {
        data.WriteString(header.first);
        data.WriteString(header.second);
    }
    data.WriteUint32(config.extras.size());
    for (const auto &extra : config.extras) {
        data.WriteString(extra.first);
        data.WriteString(extra.second);
    }
}

int32_t RequestServiceProxy::GetTask(const std::string &tid, const std::string &token, Config &config)
{
    REQUEST_HILOGD("Request GetTask, tid: %{public}s", tid.c_str());
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteString(tid);
    data.WriteString(token);
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_GETTASK), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request GetTask, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, config.bundleName, "", std::to_string(ret));
        return E_SERVICE_ERROR;
    }
    int32_t errCode = reply.ReadInt32();
    if (errCode != E_OK && errCode != E_CHANNEL_NOT_OPEN) {
        REQUEST_HILOGE("End Request GetTask, failed: %{public}d", errCode);
        SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_01, config.bundleName, "", std::to_string(errCode));
        return errCode;
    }
    ParcelHelper::UnMarshalConfig(reply, config);
    REQUEST_HILOGD("End Request GetTask ok, tid: %{public}s", tid.c_str());
    return errCode;
}

int32_t RequestServiceProxy::Start(const std::string &tid)
{
    REQUEST_HILOGD("Request Start, tid: %{public}s", tid.c_str());
    std::vector<std::string> tids = { tid };
    std::vector<ExceptionErrorCode> rets = { E_OTHER };
    int32_t ret = RequestServiceProxy::StartTasks(tids, rets);
    if (ret != ExceptionErrorCode::E_OK) {
        REQUEST_HILOGE("End Request Start, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        return ret;
    }
    REQUEST_HILOGD("End Request Start ok, tid: %{public}s", tid.c_str());
    return rets[0];
}

int32_t RequestServiceProxy::Stop(const std::string &tid)
{
    REQUEST_HILOGD("Request Stop, tid: %{public}s", tid.c_str());
    std::vector<std::string> tids = { tid };
    std::vector<ExceptionErrorCode> rets = { E_OTHER };
    int32_t ret = RequestServiceProxy::StopTasks(tids, rets);
    if (ret != ExceptionErrorCode::E_OK) {
        REQUEST_HILOGE("End Request Stop, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        return ret;
    }
    REQUEST_HILOGD("End Request Stop ok, tid: %{public}s", tid.c_str());
    return rets[0];
}

int32_t RequestServiceProxy::Query(const std::string &tid, TaskInfo &info)
{
    REQUEST_HILOGD("Request Query, tid: %{public}s", tid.c_str());
    std::vector<std::string> tids = { tid };
    TaskInfoRet infoRet{ .code = ExceptionErrorCode::E_OTHER };
    std::vector<TaskInfoRet> rets = { infoRet };

    int32_t ret = RequestServiceProxy::QueryTasks(tids, rets);
    if (ret != ExceptionErrorCode::E_OK) {
        REQUEST_HILOGE("End Request Query err, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        return ret;
    }

    int32_t errCode = rets[0].code;
    if (errCode != E_OK) {
        REQUEST_HILOGE("End Request Query, tid: %{public}s, failed: %{public}d", tid.c_str(), errCode);
        return errCode;
    }
    info = rets[0].info;
    REQUEST_HILOGD("End Request Query ok, tid: %{public}s", tid.c_str());
    return E_OK;
}

int32_t RequestServiceProxy::Touch(const std::string &tid, const std::string &token, TaskInfo &info)
{
    REQUEST_HILOGD("Request Touch, tid: %{public}s", tid.c_str());
    TaskIdAndToken idToken{ .tid = tid, .token = token };
    std::vector<TaskIdAndToken> tidTokens = { idToken };
    TaskInfoRet infoRet{ .code = ExceptionErrorCode::E_OTHER };
    std::vector<TaskInfoRet> rets = { infoRet };

    int32_t ret = RequestServiceProxy::TouchTasks(tidTokens, rets);
    if (ret != ExceptionErrorCode::E_OK) {
        REQUEST_HILOGE("End Request Touch err, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        return ret;
    }
    int32_t errCode = rets[0].code;
    if (errCode != E_OK) {
        REQUEST_HILOGE("End Request Touch, tid: %{public}s, failed: %{public}d", tid.c_str(), errCode);
        return errCode;
    }
    info = rets[0].info;
    REQUEST_HILOGD("End Request Touch ok, tid: %{public}s", tid.c_str());
    return E_OK;
}

int32_t RequestServiceProxy::Search(const Filter &filter, std::vector<std::string> &tids)
{
    REQUEST_HILOGD("Request Search");
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteString(filter.bundle);
    data.WriteInt64(filter.before);
    data.WriteInt64(filter.after);
    data.WriteUint32(static_cast<uint32_t>(filter.state));
    data.WriteUint32(static_cast<uint32_t>(filter.action));
    data.WriteUint32(static_cast<uint32_t>(filter.mode));
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_SEARCH), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request Search, failed: %{public}d", ret);
        if (ret != REMOTE_DIED_ERROR) {
            SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, std::to_string(ret));
        }
        return E_SERVICE_ERROR;
    }
    uint32_t size = reply.ReadUint32();
    for (uint32_t i = 0; i < size; i++) {
        tids.push_back(reply.ReadString());
    }
    REQUEST_HILOGD("End Request Search ok");
    return E_OK;
}

int32_t RequestServiceProxy::Show(const std::string &tid, TaskInfo &info)
{
    REQUEST_HILOGD("Request Show, tid: %{public}s", tid.c_str());
    std::vector<std::string> tids = { tid };
    TaskInfoRet infoRet{ .code = ExceptionErrorCode::E_OTHER };
    std::vector<TaskInfoRet> rets = { infoRet };

    int32_t ret = RequestServiceProxy::ShowTasks(tids, rets);
    if (ret != ExceptionErrorCode::E_OK) {
        REQUEST_HILOGE("End Request Show err, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        return ret;
    }

    int32_t errCode = rets[0].code;
    if (errCode != E_OK) {
        REQUEST_HILOGE("End Request Show, tid: %{public}s, failed: %{public}d", tid.c_str(), errCode);
        return errCode;
    }
    info = rets[0].info;
    REQUEST_HILOGD("End Request Show ok, tid: %{public}s", tid.c_str());
    return E_OK;
}

int32_t RequestServiceProxy::Pause(const std::string &tid, const Version version)
{
    REQUEST_HILOGD("Request Pause, tid: %{public}s", tid.c_str());
    std::vector<std::string> tids = { tid };
    std::vector<ExceptionErrorCode> rets = { E_OTHER };
    int32_t ret = RequestServiceProxy::PauseTasks(tids, version, rets);
    if (ret != ExceptionErrorCode::E_OK) {
        REQUEST_HILOGE("End Request Pause, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        return ret;
    }
    REQUEST_HILOGD("End Request Pause ok, tid: %{public}s", tid.c_str());
    return rets[0];
}

int32_t RequestServiceProxy::QueryMimeType(const std::string &tid, std::string &mimeType)
{
    REQUEST_HILOGD("Request QueryMimeType, tid: %{public}s", tid.c_str());
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(RequestServiceProxy::GetDescriptor());
    data.WriteString(tid);
    int32_t ret =
        Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_QUERYMIMETYPE), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request QueryMimeType, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        if (ret != REMOTE_DIED_ERROR) {
            SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, std::to_string(ret));
        }
        return E_SERVICE_ERROR;
    }
    int32_t errCode = reply.ReadInt32();
    if (errCode != E_OK) {
        REQUEST_HILOGE("End Request QueryMimeType, tid: %{public}s, failed: %{public}d", tid.c_str(), errCode);
        SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_01, std::to_string(errCode));
        return errCode;
    }
    mimeType = reply.ReadString();
    REQUEST_HILOGD("End Request QueryMimeType ok, tid: %{public}s", tid.c_str());
    return E_OK;
}

int32_t RequestServiceProxy::Remove(const std::string &tid, const Version version)
{
    REQUEST_HILOGD("Request Remove, tid: %{public}s", tid.c_str());
    std::vector<std::string> tids = { tid };
    std::vector<ExceptionErrorCode> rets = { E_OTHER };
    int32_t ret = RequestServiceProxy::RemoveTasks(tids, version, rets);
    if (ret != ExceptionErrorCode::E_OK) {
        REQUEST_HILOGE("End Request Remove, tid: %{public}s failed: %{public}d", tid.c_str(), ret);
        return ret;
    }
    // API9 or lower will not return E_TASK_NOT_FOUND.
    int32_t result = rets[0];
    if (version == Version::API9) {
        REQUEST_HILOGD("End Request Remove ok, tid: %{public}s", tid.c_str());
        result = E_OK;
    }
    REQUEST_HILOGD("End Request Remove ok, tid: %{public}s, result: %{public}d", tid.c_str(), result);
    return result;
}

int32_t RequestServiceProxy::Resume(const std::string &tid)
{
    REQUEST_HILOGD("Request Resume, tid: %{public}s", tid.c_str());
    std::vector<std::string> tids = { tid };
    std::vector<ExceptionErrorCode> rets = { E_OTHER };
    int32_t ret = RequestServiceProxy::ResumeTasks(tids, rets);
    if (ret != ExceptionErrorCode::E_OK) {
        REQUEST_HILOGE("End Resume Resume, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        return ret;
    }

    REQUEST_HILOGD("End Resume Resume ok, tid: %{public}s", tid.c_str());
    return rets[0];
}

int32_t RequestServiceProxy::SetMaxSpeed(const std::string &tid, const int64_t maxSpeed)
{
    REQUEST_HILOGD("Request SetMaxSpeed, tid: %{public}s", tid.c_str());
    std::vector<SpeedConfig> speedConfig = { { tid, maxSpeed } };
    std::vector<ExceptionErrorCode> rets = { E_OTHER };
    int32_t ret = RequestServiceProxy::SetMaxSpeeds(speedConfig, rets);
    if (ret != ExceptionErrorCode::E_OK) {
        REQUEST_HILOGE("End SetMaxSpeed, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        return ret;
    }

    REQUEST_HILOGD("End SetMaxSpeed ok, tid: %{public}s", tid.c_str());
    return rets[0];
}

int32_t RequestServiceProxy::OpenChannel(int32_t &sockFd)
{
    REQUEST_HILOGD("Request OpenChannel");
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    int32_t ret =
        Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_OPENCHANNEL), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request OpenChannel, failed: %{public}d", ret);
        if (ret != REMOTE_DIED_ERROR) {
            SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, std::to_string(ret));
        }
        return E_SERVICE_ERROR;
    }
    int32_t errCode = reply.ReadInt32();
    if (errCode != E_OK) {
        REQUEST_HILOGE("End Request OpenChannel, failed: %{public}d", errCode);
        SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_01, std::to_string(errCode));
        return errCode;
    }
    sockFd = reply.ReadFileDescriptor();
    REQUEST_HILOGD("End Request OpenChannel ok, fd: %{public}d", sockFd);
    return E_OK;
}

int32_t RequestServiceProxy::Subscribe(const std::string &tid)
{
    REQUEST_HILOGD("Request Subscribe, tid: %{public}s", tid.c_str());
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteString(tid);
    int32_t ret =
        Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_SUBSCRIBE), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request Subscribe, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        if (ret != REMOTE_DIED_ERROR) {
            SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, std::to_string(ret));
        }
        return E_SERVICE_ERROR;
    }
    REQUEST_HILOGD("End Request Subscribe ok, tid: %{public}s", tid.c_str());
    int32_t errCode = reply.ReadInt32();
    return errCode;
}

int32_t RequestServiceProxy::Unsubscribe(const std::string &tid)
{
    REQUEST_HILOGD("Request Unsubscribe, tid: %{public}s", tid.c_str());
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteString(tid);
    int32_t ret =
        Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_UNSUBSCRIBE), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request Unsubscribe, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        if (ret != REMOTE_DIED_ERROR) {
            SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, std::to_string(ret));
        }
        return E_SERVICE_ERROR;
    }
    REQUEST_HILOGD("End Request Unsubscribe ok, tid: %{public}s", tid.c_str());
    return E_OK;
}

int32_t RequestServiceProxy::SubRunCount(const sptr<NotifyInterface> &listener)
{
    REQUEST_HILOGD("Request SubRunCount");
    FwkRunningTaskCountManager::GetInstance()->SetSaStatus(true);
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteRemoteObject(listener->AsObject());
    int32_t ret =
        Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_SUB_RUNCOUNT), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request SubRunCount, failed: %{public}d", ret);
        if (ret != REMOTE_DIED_ERROR) {
            SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, std::to_string(ret));
        }
        return ret;
    }
    int32_t errCode = reply.ReadInt32();
    if (errCode != E_OK) {
        REQUEST_HILOGE("End Request SubRunCount, failed: %{public}d", errCode);
        SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_01, std::to_string(errCode));
        return errCode;
    }
    REQUEST_HILOGD("End Request SubRunCount ok");
    return E_OK;
}

int32_t RequestServiceProxy::UnsubRunCount()
{
    REQUEST_HILOGD("Request UnubRunCount");
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    int32_t ret =
        Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_UNSUB_RUNCOUNT), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request UnubRunCount, failed: %{public}d", ret);
        if (ret != REMOTE_DIED_ERROR) {
            SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, std::to_string(ret));
        }
        return E_SERVICE_ERROR;
    }
    REQUEST_HILOGD("End Request UnubRunCount ok");
    return E_OK;
}

int32_t RequestServiceProxy::CreateGroup(std::string &gid, const bool gauge, Notification &notification)
{
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteBool(gauge);
    if (notification.title != std::nullopt) {
        data.WriteBool(true);
        data.WriteString(*notification.title);
    } else {
        data.WriteBool(false);
    }
    if (notification.text != std::nullopt) {
        data.WriteBool(true);
        data.WriteString(*notification.text);
    } else {
        data.WriteBool(false);
    }
    if (notification.wantAgent != std::nullopt) {
        data.WriteBool(true);
        data.WriteString(*notification.wantAgent);
    } else {
        data.WriteBool(false);
    }
    data.WriteBool(notification.disable);
    data.WriteUint32(static_cast<uint32_t>(notification.visibility));
    int32_t ret =
        Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_CREATE_GROUP), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request AttachGroup, failed: %{public}d", ret);
        if (ret != REMOTE_DIED_ERROR) {
            SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, std::to_string(ret));
        }
        return E_SERVICE_ERROR;
    }
    gid = reply.ReadString();
    return E_OK;
}

int32_t RequestServiceProxy::AttachGroup(const std::string &gid, const std::vector<std::string> &tids)
{
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteString(gid);
    data.WriteStringVector(tids);
    int32_t ret =
        Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_ATTACH_GROUP), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request AttachGroup, failed: %{public}d", ret);
        if (ret != REMOTE_DIED_ERROR) {
            SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, std::to_string(ret));
        }
        return E_SERVICE_ERROR;
    }
    int code = reply.ReadInt32();
    if (code != E_OK) {
        REQUEST_HILOGE("End Request AttachGroup, failed: %{public}d", code);
        SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_01, std::to_string(code));
    }
    return code;
}

int32_t RequestServiceProxy::DeleteGroup(const std::string &gid)
{
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteString(gid);
    int32_t ret =
        Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_DELETE_GROUP), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request AttachGroup, failed: %{public}d", ret);
        if (ret != REMOTE_DIED_ERROR) {
            SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_00, std::to_string(ret));
        }
        return E_SERVICE_ERROR;
    }
    int code = reply.ReadInt32();
    if (code != E_OK) {
        REQUEST_HILOGE("End Request AttachGroup, failed: %{public}d", code);
        SysEventLog::SendSysEventLog(FAULT_EVENT, IPC_FAULT_01, std::to_string(code));
    }
    return code;
}

} // namespace OHOS::Request
