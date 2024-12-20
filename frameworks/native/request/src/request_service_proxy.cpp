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

#include "download_server_ipc_interface_code.h"
#include "iremote_broker.h"
#include "log.h"
#include "parcel_helper.h"
#include "request_running_task_count.h"

namespace OHOS::Request {
using namespace OHOS::HiviewDFX;

RequestServiceProxy::RequestServiceProxy(const sptr<IRemoteObject> &object)
    : IRemoteProxy<RequestServiceInterface>(object)
{
}

int32_t RequestServiceProxy::Create(const Config &config, std::string &tid)
{
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
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
    data.WriteUint32(config.index);
    data.WriteInt64(config.begins);
    data.WriteInt64(config.ends);
    data.WriteBool(config.gauge);
    data.WriteBool(config.precise);
    data.WriteUint32(config.priority);
    data.WriteString(config.url);
    data.WriteString(config.title);
    data.WriteString(config.method);
    data.WriteString(config.token);
    data.WriteString(config.description);
    data.WriteString(config.data);
    data.WriteString(config.proxy);
    data.WriteString(config.certificatePins);
    GetVectorData(config, data);
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_REQUEST), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End send create request, failed: %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    int32_t errCode = reply.ReadInt32();
    if (errCode != E_OK && errCode != E_CHANNEL_NOT_OPEN) {
        REQUEST_HILOGE("End send create request, failed: %{public}d", errCode);
        return errCode;
    }
    tid = std::to_string(reply.ReadInt32());
    return errCode;
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

    for (const auto &file : config.files) {
        if (file.fd > 0) {
            close(file.fd);
        }
    }
    // Response Bodys fds.
    data.WriteUint32(config.bodyFds.size());
    for (const auto &fd : config.bodyFds) {
        if (fd > 0) {
            close(fd);
        }
    }
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
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteString(tid);
    data.WriteString(token);
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_GETTASK), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request GetTask, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        return E_SERVICE_ERROR;
    }
    int32_t errCode = reply.ReadInt32();
    if (errCode != E_OK && errCode != E_CHANNEL_NOT_OPEN) {
        REQUEST_HILOGE("End Request GetTask, failed: %{public}d", errCode);
        return errCode;
    }
    ParcelHelper::UnMarshalConfig(reply, config);
    REQUEST_HILOGD("End Request GetTask ok, tid: %{public}s", tid.c_str());
    return errCode;
}

int32_t RequestServiceProxy::Start(const std::string &tid)
{
    REQUEST_HILOGD("Request Start, tid: %{public}s", tid.c_str());
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteString(tid);

    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_START), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request Start, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        return E_SERVICE_ERROR;
    }
    REQUEST_HILOGD("End Request Start ok, tid: %{public}s", tid.c_str());
    return reply.ReadInt32();
}

int32_t RequestServiceProxy::Stop(const std::string &tid)
{
    REQUEST_HILOGD("Request Stop, tid: %{public}s", tid.c_str());
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteString(tid);
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_STOP), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request Stop, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        return E_SERVICE_ERROR;
    }
    REQUEST_HILOGD("End Request Stop ok, tid: %{public}s", tid.c_str());
    return reply.ReadInt32();
}

int32_t RequestServiceProxy::Query(const std::string &tid, TaskInfo &info)
{
    REQUEST_HILOGD("Request Query, tid: %{public}s", tid.c_str());
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(RequestServiceProxy::GetDescriptor());
    data.WriteString(tid);
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_QUERY), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request Query, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        return E_SERVICE_ERROR;
    }
    int32_t errCode = reply.ReadInt32();
    if (errCode != E_OK) {
        REQUEST_HILOGE("End Request Query, tid: %{public}s, failed: %{public}d", tid.c_str(), errCode);
        return errCode;
    }
    ParcelHelper::UnMarshal(reply, info);
    REQUEST_HILOGD("End Request Query ok, tid: %{public}s", tid.c_str());
    return E_OK;
}

int32_t RequestServiceProxy::Touch(const std::string &tid, const std::string &token, TaskInfo &info)
{
    REQUEST_HILOGD("Request Touch, tid: %{public}s", tid.c_str());
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(RequestServiceProxy::GetDescriptor());
    data.WriteString(tid);
    data.WriteString(token);
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_TOUCH), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request Touch, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        return E_SERVICE_ERROR;
    }
    int32_t errCode = reply.ReadInt32();
    if (errCode != E_OK) {
        REQUEST_HILOGE("End Request Touch, tid: %{public}s, failed: %{public}d", tid.c_str(), errCode);
        return errCode;
    }
    ParcelHelper::UnMarshal(reply, info);
    REQUEST_HILOGD("End Request Touch ok, tid: %{public}s", tid.c_str());
    return E_OK;
}

int32_t RequestServiceProxy::Search(const Filter &filter, std::vector<std::string> &tids)
{
    REQUEST_HILOGD("Request Search");
    MessageParcel data, reply;
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
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(RequestServiceProxy::GetDescriptor());
    data.WriteString(tid);
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_SHOW), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request Show, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        return E_SERVICE_ERROR;
    }
    int32_t errCode = reply.ReadInt32();
    if (errCode != E_OK) {
        REQUEST_HILOGE("End Request Show, tid: %{public}s, failed: %{public}d", tid.c_str(), errCode);
        return errCode;
    }
    ParcelHelper::UnMarshal(reply, info);
    REQUEST_HILOGD("End Request Show ok, tid: %{public}s", tid.c_str());
    return E_OK;
}

int32_t RequestServiceProxy::Pause(const std::string &tid, Version version)
{
    REQUEST_HILOGD("Request Pause, tid: %{public}s", tid.c_str());
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteUint32(static_cast<uint32_t>(version));
    data.WriteString(tid);
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_PAUSE), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request Pause, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        return E_SERVICE_ERROR;
    }
    REQUEST_HILOGD("End Request Pause ok, tid: %{public}s", tid.c_str());
    return reply.ReadInt32();
}

int32_t RequestServiceProxy::QueryMimeType(const std::string &tid, std::string &mimeType)
{
    REQUEST_HILOGD("Request QueryMimeType, tid: %{public}s", tid.c_str());
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(RequestServiceProxy::GetDescriptor());
    data.WriteString(tid);
    int32_t ret =
        Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_QUERYMIMETYPE), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request QueryMimeType, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        return E_SERVICE_ERROR;
    }
    int32_t errCode = reply.ReadInt32();
    if (errCode != E_OK) {
        REQUEST_HILOGE("End Request QueryMimeType, tid: %{public}s, failed: %{public}d", tid.c_str(), errCode);
        return errCode;
    }
    mimeType = reply.ReadString();
    REQUEST_HILOGD("End Request QueryMimeType ok, tid: %{public}s", tid.c_str());
    return E_OK;
}

int32_t RequestServiceProxy::Remove(const std::string &tid, Version version)
{
    REQUEST_HILOGD("Request Remove, tid: %{public}s", tid.c_str());
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(RequestServiceProxy::GetDescriptor());
    data.WriteUint32(static_cast<uint32_t>(version));
    data.WriteString(tid);
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_REMOVE), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request Remove, tid: %{public}s failed: %{public}d", tid.c_str(), ret);
        return E_SERVICE_ERROR;
    }

    // API9 or lower will not return E_TASK_NOT_FOUND.
    int32_t result = reply.ReadInt32();
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
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(RequestServiceProxy::GetDescriptor());
    data.WriteString(tid);
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_RESUME), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request Resume, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        return E_SERVICE_ERROR;
    }
    REQUEST_HILOGD("End Request Resume ok, tid: %{public}s", tid.c_str());
    return reply.ReadInt32();
}

int32_t RequestServiceProxy::OpenChannel(int32_t &sockFd)
{
    REQUEST_HILOGD("Request OpenChannel");
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    int32_t ret =
        Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_OPENCHANNEL), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request OpenChannel, failed: %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    int32_t errCode = reply.ReadInt32();
    if (errCode != E_OK) {
        REQUEST_HILOGE("End Request OpenChannel, failed: %{public}d", errCode);
        return errCode;
    }
    sockFd = reply.ReadFileDescriptor();

    REQUEST_HILOGD("End Request OpenChannel ok, fd: %{public}d", sockFd);
    return E_OK;
}

int32_t RequestServiceProxy::Subscribe(const std::string &tid)
{
    REQUEST_HILOGD("Request Subscribe, tid: %{public}s", tid.c_str());
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteString(tid);
    int32_t ret =
        Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_SUBSCRIBE), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request Subscribe, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        return E_SERVICE_ERROR;
    }
    REQUEST_HILOGD("End Request Subscribe ok, tid: %{public}s", tid.c_str());
    int32_t errCode = reply.ReadInt32();
    return errCode;
}

int32_t RequestServiceProxy::Unsubscribe(const std::string &tid)
{
    REQUEST_HILOGD("Request Unsubscribe, tid: %{public}s", tid.c_str());
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteString(tid);
    int32_t ret =
        Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_UNSUBSCRIBE), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request Unsubscribe, tid: %{public}s, failed: %{public}d", tid.c_str(), ret);
        return E_SERVICE_ERROR;
    }
    REQUEST_HILOGD("End Request Unsubscribe ok, tid: %{public}s", tid.c_str());
    return E_OK;
}

int32_t RequestServiceProxy::SubRunCount(const sptr<NotifyInterface> &listener)
{
    REQUEST_HILOGD("Request SubRunCount");
    FwkRunningTaskCountManager::GetInstance()->SetSaStatus(true);
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteRemoteObject(listener->AsObject());
    int32_t ret =
        Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_SUB_RUNCOUNT), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request SubRunCount, failed: %{public}d", ret);
        return ret;
    }
    int32_t errCode = reply.ReadInt32();
    if (errCode != E_OK) {
        REQUEST_HILOGE("End Request SubRunCount, failed: %{public}d", errCode);
        return errCode;
    }
    REQUEST_HILOGD("End Request SubRunCount ok");
    return E_OK;
}

int32_t RequestServiceProxy::UnsubRunCount()
{
    REQUEST_HILOGD("Request UnubRunCount");
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    int32_t ret =
        Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_UNSUB_RUNCOUNT), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("End Request UnubRunCount, failed: %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    REQUEST_HILOGD("End Request UnubRunCount ok");
    return E_OK;
}

} // namespace OHOS::Request
