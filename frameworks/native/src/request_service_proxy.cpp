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

#include <ctime>

#include "download_server_ipc_interface_code.h"
#include "iremote_broker.h"
#include "log.h"
#include "parcel_helper.h"

namespace OHOS::Request {
using namespace OHOS::HiviewDFX;

RequestServiceProxy::RequestServiceProxy(const sptr<IRemoteObject> &object)
    : IRemoteProxy<RequestServiceInterface>(object)
{
}

int32_t RequestServiceProxy::Create(const Config &config, int32_t &tid, sptr<NotifyInterface> listener)
{
    REQUEST_HILOGD("Create");
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteUint32(static_cast<uint32_t>(config.action));
    data.WriteUint32(static_cast<uint32_t>(config.version));
    data.WriteUint32(static_cast<uint32_t>(config.mode));
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
    GetVectorData(config, data);
    data.WriteRemoteObject(listener->AsObject().GetRefPtr());
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_REQUEST), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("SendRequest ret : %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    int32_t errCode = reply.ReadInt32();
    if (errCode != E_OK) {
        return errCode;
    }
    tid = reply.ReadInt32();
    return E_OK;
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
        data.WriteInt32(static_cast<int32_t>(errno));
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
    REQUEST_HILOGD("GetTask");
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteString(tid);
    data.WriteString(token);
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_GETTASK), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    int32_t errCode = reply.ReadInt32();
    if (errCode != E_OK) {
        return errCode;
    }
    ParcelHelper::UnMarshalConfig(reply, config);
    return E_OK;
}

int32_t RequestServiceProxy::Start(const std::string &tid)
{
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteString(tid);
    REQUEST_HILOGD("Start.");
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_START), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    return reply.ReadInt32();
}

int32_t RequestServiceProxy::Stop(const std::string &tid)
{
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteString(tid);
    REQUEST_HILOGD("Stop");
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_STOP), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    return reply.ReadInt32();
}

int32_t RequestServiceProxy::Query(const std::string &tid, TaskInfo &info)
{
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(RequestServiceProxy::GetDescriptor());
    data.WriteString(tid);
    REQUEST_HILOGD("Query");
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_QUERY), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    int32_t errCode = reply.ReadInt32();
    if (errCode != E_OK) {
        return errCode;
    }
    ParcelHelper::UnMarshal(reply, info);
    return E_OK;
}

int32_t RequestServiceProxy::Touch(const std::string &tid, const std::string &token, TaskInfo &info)
{
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(RequestServiceProxy::GetDescriptor());
    data.WriteString(tid);
    data.WriteString(token);
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_TOUCH), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    int32_t errCode = reply.ReadInt32();
    if (errCode != E_OK) {
        return errCode;
    }
    ParcelHelper::UnMarshal(reply, info);
    return E_OK;
}

int32_t RequestServiceProxy::Search(const Filter &filter, std::vector<std::string> &tids)
{
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
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    uint32_t size = reply.ReadUint32();
    for (uint32_t i = 0; i < size; i++) {
        tids.push_back(reply.ReadString());
    }
    return E_OK;
}

int32_t RequestServiceProxy::Show(const std::string &tid, TaskInfo &info)
{
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(RequestServiceProxy::GetDescriptor());
    data.WriteString(tid);
    REQUEST_HILOGD("RequestServiceProxy Show started.");
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_SHOW), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    int32_t errCode = reply.ReadInt32();
    if (errCode != E_OK) {
        return errCode;
    }
    ParcelHelper::UnMarshal(reply, info);
    return E_OK;
}

int32_t RequestServiceProxy::Pause(const std::string &tid, Version version)
{
    REQUEST_HILOGD("Pause");
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteUint32(static_cast<uint32_t>(version));
    data.WriteString(tid);
    REQUEST_HILOGD("RequestServiceProxy Pause started.");
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_PAUSE), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    return reply.ReadInt32();
}

int32_t RequestServiceProxy::QueryMimeType(const std::string &tid, std::string &mimeType)
{
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(RequestServiceProxy::GetDescriptor());
    data.WriteString(tid);
    REQUEST_HILOGD("RequestServiceProxy QueryMimeType started.");
    int32_t ret =
        Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_QUERYMIMETYPE), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    int32_t errCode = reply.ReadInt32();
    if (errCode != E_OK) {
        return errCode;
    }
    mimeType = reply.ReadString();
    return E_OK;
}

int32_t RequestServiceProxy::Remove(const std::string &tid, Version version)
{
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(RequestServiceProxy::GetDescriptor());
    data.WriteUint32(static_cast<uint32_t>(version));
    data.WriteString(tid);
    REQUEST_HILOGD("RequestServiceProxy Remove started.");
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_REMOVE), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }

    // API9 or lower will not return E_TASK_NOT_FOUND.
    int32_t result = reply.ReadInt32();
    if (version == Version::API9) {
        result = E_OK;
    }
    return result;
}

int32_t RequestServiceProxy::Resume(const std::string &tid)
{
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(RequestServiceProxy::GetDescriptor());
    data.WriteString(tid);
    REQUEST_HILOGD("RequestServiceProxy Resume started.");
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_RESUME), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    return reply.ReadInt32();
}

int32_t RequestServiceProxy::On(
    const std::string &type, const std::string &tid, const sptr<NotifyInterface> &listener, Version version)
{
    REQUEST_HILOGD("On");
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteUint32(static_cast<uint32_t>(version));
    data.WriteString(type);
    data.WriteString(tid);
    data.WriteRemoteObject(listener->AsObject().GetRefPtr());
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_ON), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    return E_OK;
}

int32_t RequestServiceProxy::Off(const std::string &type, const std::string &tid, Version version)
{
    REQUEST_HILOGD("Off");
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteUint32(static_cast<uint32_t>(version));
    data.WriteString(type);
    data.WriteString(tid);
    int32_t ret = Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_OFF), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    return E_OK;
}

int32_t RequestServiceProxy::OpenChannel(int32_t &sockFd)
{
    REQUEST_HILOGD("OpenChannel");
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    int32_t ret =
        Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_OPENCHANNEL), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    int32_t errCode = reply.ReadInt32();
    if (errCode != E_OK) {
        return errCode;
    }
    sockFd = reply.ReadFileDescriptor();
    REQUEST_HILOGD("OpenChannel sockFd: %{public}d", sockFd);
    return E_OK;
}

int32_t RequestServiceProxy::Subscribe(const std::string &taskId, int32_t cbType)
{
    REQUEST_HILOGD("Subscribe");
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteString(taskId);
    data.WriteInt32(cbType);
    int32_t ret =
        Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_SUBSCRIBE), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    return E_OK;
}

int32_t RequestServiceProxy::Unsubscribe(const std::string &taskId, int32_t cbType)
{
    REQUEST_HILOGD("Unsubscribe");
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteString(taskId);
    data.WriteInt32(cbType);
    int32_t ret =
        Remote()->SendRequest(static_cast<uint32_t>(RequestInterfaceCode::CMD_UNSUBSCRIBE), data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    return E_OK;
}

} // namespace OHOS::Request
