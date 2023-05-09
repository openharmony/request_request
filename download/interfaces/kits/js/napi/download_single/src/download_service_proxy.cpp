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
#include "download_service_proxy.h"
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>

#include "iremote_broker.h"
#include "log.h"
#include "download_common.h"

static constexpr uint32_t FILE_PERMISSION = 0644;

namespace OHOS::Request::Download {
using namespace OHOS::HiviewDFX;

DownloadServiceProxy::DownloadServiceProxy(const sptr<IRemoteObject> &object)
    : IRemoteProxy<DownloadServiceInterface>(object)
{
}

bool DownloadServiceProxy::IsPathValid(const std::string &filePath)
{
    auto path = filePath.substr(0, filePath.rfind('/'));
    char resolvedPath[PATH_MAX + 1] = { 0 };
    if (path.length() > PATH_MAX || realpath(path.c_str(), resolvedPath) == nullptr
        || strncmp(resolvedPath, path.c_str(), path.length()) != 0) {
        DOWNLOAD_HILOGE("invalid file path!");
        return false;
    }
    return true;
}

int32_t DownloadServiceProxy::Request(const DownloadConfig &config, ExceptionError &error)
{
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    int32_t fd = -1;
    if (!IsPathValid(config.GetFilePath())) {
        error.code = EXCEPTION_FILE_PATH;
        error.errInfo = "Download File Path Valid";
        return -1;
    }
    fd = open(config.GetFilePath().c_str(), O_RDWR);
    DOWNLOAD_HILOGE("fd: %{public}d start", fd);
    if (fd >= 0) {
        error.code = EXCEPTION_FILE_PATH;
        error.errInfo = "Download File already exists";
        DOWNLOAD_HILOGE("%{public}s", error.errInfo.c_str());
        close(fd);
        return -1;
    } else {
        fd = open(config.GetFilePath().c_str(), O_CREAT | O_RDWR, FILE_PERMISSION);
        if (fd < 0) {
            error.code = EXCEPTION_FILE_IO;
            error.errInfo = "Failed to open file errno " + std::to_string(errno);
            DOWNLOAD_HILOGE("%{public}s", error.errInfo.c_str());
            return -1;
        }
    }
    DOWNLOAD_HILOGE("fd: %{public}d end", fd);
    data.WriteFileDescriptor(fd);
    if (fd > 0) {
        close(fd);
    }
    data.WriteInt32(static_cast<int32_t>(errno));
    data.WriteString(config.GetUrl());
    data.WriteBool(config.IsMetered());
    data.WriteBool(config.IsRoaming());
    data.WriteString(config.GetDescription());
    data.WriteUint32(config.GetNetworkType());
    data.WriteString(config.GetFilePath());
    data.WriteString(config.GetTitle());
    data.WriteString(config.GetDescription());
    data.WriteBool(config.IsBackground());
    data.WriteString(config.GetBundleName());
    data.WriteInt32(config.GetApplicationInfoUid());
    data.WriteUint32(config.GetHeader().size());
    std::map<std::string, std::string>::const_iterator iter;
    for (iter = config.GetHeader().begin(); iter != config.GetHeader().end(); ++iter) {
        data.WriteString(iter->first);
        data.WriteString(iter->second);
    }
    config.Dump();
    bool ret = Remote()->SendRequest(CMD_REQUEST, data, reply, option);
    if (ret != ERR_NONE) {
        error.code = EXCEPTION_SERVICE_ERROR;
        error.errInfo = "ipc request  ret = " + std::to_string(ret);
        DOWNLOAD_HILOGE("%{public}s", error.errInfo.c_str());
        return -1;
    }
    int32_t taskId = reply.ReadInt32();
    if (taskId < 0) {
        error.code = EXCEPTION_SERVICE_ERROR;
        error.errInfo = "taskId: " + std::to_string(taskId);
        DOWNLOAD_HILOGE("%{public}s", error.errInfo.c_str());
        return -1;
    }
    return taskId;
}

bool DownloadServiceProxy::Pause(uint32_t taskId)
{
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(GetDescriptor());
    data.WriteUint32(taskId);
    DOWNLOAD_HILOGD("DownloadServiceProxy Pause started.");
    bool ret = Remote()->SendRequest(CMD_PAUSE, data, reply, option);
    if (ret != ERR_NONE) {
        DOWNLOAD_HILOGE("Pause, ret = %{public}d", ret);
        return false;
    }
    DOWNLOAD_HILOGD("DownloadServiceProxy Pause succeeded.");
    return true;
}

bool DownloadServiceProxy::Query(uint32_t taskId, DownloadInfo &info)
{
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(DownloadServiceProxy::GetDescriptor());
    data.WriteInt32(taskId);
    DOWNLOAD_HILOGD("DownloadServiceProxy Query started.");
    bool ret = Remote()->SendRequest(CMD_QUERY, data, reply, option);
    if (ret != ERR_NONE) {
        DOWNLOAD_HILOGE("Query, ret = %{public}d", ret);
        return false;
    }
    DOWNLOAD_HILOGD("DownloadServiceProxy Query succeeded.");
    info.SetDescription(reply.ReadString());
    info.SetDownloadedBytes(reply.ReadUint32());
    info.SetDownloadId(reply.ReadInt64());
    info.SetFailedReason(static_cast<ErrorCode>(reply.ReadUint32()));
    info.SetFileName(reply.ReadString());
    info.SetFilePath(reply.ReadString());
    info.SetPausedReason(static_cast<PausedReason>(reply.ReadUint32()));
    info.SetStatus(static_cast<DownloadStatus>(reply.ReadUint32()));
    info.SetTargetURI(reply.ReadString());
    info.SetDownloadTitle(reply.ReadString());
    info.SetDownloadTotalBytes(reply.ReadInt64());
    info.Dump();
    return true;
}

bool DownloadServiceProxy::QueryMimeType(uint32_t taskId, std::string &mimeType)
{
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(DownloadServiceProxy::GetDescriptor());
    data.WriteInt32(taskId);
    DOWNLOAD_HILOGD("DownloadServiceProxy QueryMimeType started.");
    bool ret = Remote()->SendRequest(CMD_QUERYMIMETYPE, data, reply, option);
    if (ret != ERR_NONE) {
        DOWNLOAD_HILOGE("QueryMimeType, ret = %{public}d", ret);
        return false;
    }
    mimeType = reply.ReadString();
    DOWNLOAD_HILOGD("DownloadServiceProxy QueryMimeType succeeded.");
    return true;
}

bool DownloadServiceProxy::Remove(uint32_t taskId)
{
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(DownloadServiceProxy::GetDescriptor());
    data.WriteInt32(taskId);
    DOWNLOAD_HILOGD("DownloadServiceProxy Remove started.");
    bool ret = Remote()->SendRequest(CMD_REMOVE, data, reply, option);
    if (ret != ERR_NONE) {
        DOWNLOAD_HILOGE("Remove, ret = %{public}d", ret);
        return false;
    }
    DOWNLOAD_HILOGD("DownloadServiceProxy Remove succeeded.");
    return true;
}

bool DownloadServiceProxy::Resume(uint32_t taskId)
{
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    data.WriteInterfaceToken(DownloadServiceProxy::GetDescriptor());
    data.WriteInt32(taskId);
    DOWNLOAD_HILOGD("DownloadServiceProxy Resume started.");
    bool ret = Remote()->SendRequest(CMD_RESUME, data, reply, option);
    if (ret != ERR_NONE) {
        DOWNLOAD_HILOGE("Resume, ret = %{public}d", ret);
        return false;
    }
    DOWNLOAD_HILOGD("DownloadServiceProxy Resume succeeded.");
    return true;
}

bool DownloadServiceProxy::On(uint32_t taskId, const std::string &type, const sptr<DownloadNotifyInterface> &listener)
{
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    if (!data.WriteInterfaceToken(GetDescriptor())) {
        DOWNLOAD_HILOGE(" Failed to write parcelable ");
        return false;
    }
    if (listener == nullptr) {
        DOWNLOAD_HILOGE("listener is nullptr");
        return false;
    }

    if (!data.WriteUint32(taskId)) {
        DOWNLOAD_HILOGE("write taskId=%{public}d fail", taskId);
        return false;
    }
    
    DOWNLOAD_HILOGD("DownloadServiceProxy::On type=%{public}s", type.c_str());
    if (type.empty()) {
        DOWNLOAD_HILOGE("DownloadServiceProxy::On type is null.");
        return false;
    }
    if (!data.WriteString(type)) {
        DOWNLOAD_HILOGE("write type failed.");
        return false;
    }
    if (!data.WriteRemoteObject(listener->AsObject().GetRefPtr())) {
        DOWNLOAD_HILOGE("write parcel failed.");
        return false;
    }
    int32_t result = Remote()->SendRequest(CMD_ON, data, reply, option);
    if (result != ERR_NONE) {
        DOWNLOAD_HILOGE(" DownloadServiceProxy::On fail, result = %{public}d ", result);
        return false;
    }
    bool ret = reply.ReadBool();
    DOWNLOAD_HILOGD("DownloadServiceProxy::On out [result: %{public}d]", ret);
    return ret;
}

bool DownloadServiceProxy::Off(uint32_t taskId, const std::string &type)
{
    DOWNLOAD_HILOGD("DownloadServiceProxy::Off in");
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    if (!data.WriteInterfaceToken(GetDescriptor())) {
        DOWNLOAD_HILOGE(" Failed to write parcelable ");
        return false;
    }
    
    if (!data.WriteUint32(taskId)) {
        DOWNLOAD_HILOGE("write taskId=%{public}d fail", taskId);
        return false;
    }
    
    if (!data.WriteString(type)) {
        DOWNLOAD_HILOGE("write type=%{public}s fail", type.c_str());
        return false;
    }
    int32_t result = Remote()->SendRequest(CMD_OFF, data, reply, option);
    if (result != ERR_NONE) {
        DOWNLOAD_HILOGE(" DownloadServiceProxy::Off fail, ret = %{public}d ", result);
        return false;
    }
    bool ret = reply.ReadBool();
    DOWNLOAD_HILOGD("DownloadServiceProxy::Off out [ret: %{public}d]", ret);
    return ret;
}

bool DownloadServiceProxy::CheckPermission()
{
    DOWNLOAD_HILOGD("DownloadServiceProxy::CheckPermission in");
    MessageParcel data;
    MessageParcel reply;
    MessageOption option;
    if (!data.WriteInterfaceToken(GetDescriptor())) {
        DOWNLOAD_HILOGE(" Failed to write parcelable ");
        return false;
    }
    
    int32_t result = Remote()->SendRequest(CMD_CHECKPERMISSION, data, reply, option);
    if (result != ERR_NONE) {
        DOWNLOAD_HILOGE(" DownloadServiceProxy::CheckPermission fail, ret = %{public}d ", result);
        return false;
    }
    bool ret = reply.ReadBool();
    DOWNLOAD_HILOGD("DownloadServiceProxy::CheckPermission out [ret: %{public}d]", ret);
    return ret;
}
} // namespace OHOS::Request::Download
