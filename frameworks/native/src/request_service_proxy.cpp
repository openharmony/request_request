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
    data.WriteString(config.url);
    data.WriteString(config.bundleName);
    data.WriteString(config.title);
    data.WriteString(config.method);
    data.WriteString(config.token);
    data.WriteString(config.description);
    data.WriteString(config.data);
    GetVectorData(config, data);
    data.WriteRemoteObject(listener->AsObject().GetRefPtr());
    int32_t ret = Remote()->SendRequest(CMD_REQUEST, data, reply, option);
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
        data.WriteFileDescriptor(file.fd);
        data.WriteInt32(static_cast<int32_t>(errno));
    }

    for (const auto &file : config.files) {
        if (file.fd > 0) {
            close(file.fd);
        }
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

int32_t RequestServiceProxy::Start(const std::string &tid)
{
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteString(tid);
    REQUEST_HILOGD("Start.");
    int32_t ret = Remote()->SendRequest(CMD_START, data, reply, option);
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
    int32_t ret = Remote()->SendRequest(CMD_STOP, data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    return reply.ReadInt32();
}

int32_t RequestServiceProxy::Show(const std::string &tid, TaskInfo &info)
{
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(RequestServiceProxy::GetDescriptor());
    data.WriteUint32(static_cast<uint32_t>(Version::API10));
    data.WriteString(tid);
    REQUEST_HILOGD("Show");
    int32_t ret = Remote()->SendRequest(CMD_SHOW, data, reply, option);
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
    int32_t ret = Remote()->SendRequest(CMD_TOUCH, data, reply, option);
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
    data.WriteString(filter.before);
    data.WriteString(filter.after);
    data.WriteUint32(static_cast<uint32_t>(filter.state));
    data.WriteUint32(static_cast<uint32_t>(filter.action));
    data.WriteUint32(static_cast<uint32_t>(filter.mode));
    int32_t ret = Remote()->SendRequest(CMD_TOUCH, data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    int32_t errCode = reply.ReadInt32();
    if (errCode != E_OK) {
        return errCode;
    }
    uint32_t size = reply.ReadUint32();
    for (uint32_t i = 0; i < size; i++) {
        tids.push_back(reply.ReadString());
    }
    return E_OK;
}

int32_t RequestServiceProxy::Clear(const std::vector<std::string> &tids, std::vector<std::string> &res)
{
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(RequestServiceProxy::GetDescriptor());
    for (auto tid : tids) {
        data.WriteString(tid);
    }
    int32_t ret = Remote()->SendRequest(CMD_CLEAR, data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    int32_t errCode = reply.ReadInt32();
    if (errCode != E_OK) {
        return errCode;
    }
      
    for (uint32_t i = 0; i < reply.ReadUint32(); i++) {
        res.push_back(reply.ReadString());
    }
    return ret;
}

int32_t RequestServiceProxy::Query(const std::string &tid, TaskInfo &info, Version version)
{
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(RequestServiceProxy::GetDescriptor());
    data.WriteUint32(static_cast<uint32_t>(version));
    data.WriteString(tid);
    REQUEST_HILOGD("RequestServiceProxy Query started.");
    int32_t ret = Remote()->SendRequest(CMD_QUERY, data, reply, option);
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
    int32_t ret =  Remote()->SendRequest(CMD_PAUSE, data, reply, option);
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
    int32_t ret = Remote()->SendRequest(CMD_QUERYMIMETYPE, data, reply, option);
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
    int32_t ret = Remote()->SendRequest(CMD_REMOVE, data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    return reply.ReadInt32();
}

int32_t RequestServiceProxy::Resume(const std::string &tid)
{
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(RequestServiceProxy::GetDescriptor());
    data.WriteString(tid);
    REQUEST_HILOGD("RequestServiceProxy Resume started.");
    int32_t ret = Remote()->SendRequest(CMD_RESUME, data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    return reply.ReadInt32();
}

int32_t RequestServiceProxy::On(const std::string &type, const std::string &tid,
    const sptr<NotifyInterface> &listener)
{
    REQUEST_HILOGD("On");
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteString(type);
    data.WriteString(tid);
    data.WriteRemoteObject(listener->AsObject().GetRefPtr());
    int32_t ret = Remote()->SendRequest(CMD_ON, data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    return E_OK;
}

int32_t RequestServiceProxy::Off(const std::string &type, const std::string &tid)
{
    REQUEST_HILOGD("Off");
    MessageParcel data, reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteString(type);
    data.WriteString(tid);
    int32_t ret = Remote()->SendRequest(CMD_OFF, data, reply, option);
    if (ret != ERR_NONE) {
        REQUEST_HILOGE("send request ret code is %{public}d", ret);
        return E_SERVICE_ERROR;
    }
    return E_OK;
}
} // namespace OHOS::Request