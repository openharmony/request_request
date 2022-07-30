/*
 * Copyright (C) 2021-2022 Huawei Device Co., Ltd.
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

#include <iomanip>
#include <vector>
#include <string>
#include <sstream>
#include <ostream>
#include <ios>
#include "constant.h"
#include "download_info.h"
#include "download_service_manager.h"
#include "dump_task_info.h"

namespace OHOS::Request::Download {
bool DumpTaskInfo::Dump(int fd, const std::vector<std::string> &args)
{
    uint32_t argsNum = args.size();
    if (argsNum > 1) {
        dprintf(fd, "too many args, -t accept no arg or one arg \n");
        return false;
    }

    if (argsNum == 0) {
        DumpAllTask(fd);
    } else {
        DumpTaskDetailInfo(fd, std::stoul(args[0]));
    }

    return true;
}

void DumpTaskInfo::DumpAllTaskTile(int fd) const
{
    std::ostringstream buffer;
    buffer << std::left;
    FormatSummaryTitle(buffer);
    dprintf(fd, "%s\n", buffer.str().c_str());
}

void DumpTaskInfo::FormatSummaryTitle(std::ostringstream &buffer) const
{
    for (const auto &it: summaryColumnTitle_) {
        buffer << std::setw(it.first) << it.second;
    }
}

void DumpTaskInfo::FormatDetailTitle(std::ostringstream &buffer) const
{
    for (const auto &it: detailColumnTitle_) {
        buffer << std::setw(it.first) << it.second;
    }
}

void DumpTaskInfo::DumpTaskDetailInfoTile(int fd) const
{
    std::ostringstream buffer;
    buffer << std::left;
    FormatSummaryTitle(buffer);
    FormatDetailTitle(buffer);
    dprintf(fd, "%s\n", buffer.str().c_str());
}

void DumpTaskInfo::FormatSummaryContent(const DownloadInfo &taskInfo, std::ostringstream &buffer) const
{
    for (const auto &it: dumpSummaryCfg_) {
        auto columnFormatFun = it.second;
        buffer << std::setw(it.first) << (this->*columnFormatFun)(taskInfo);
    }
}

void DumpTaskInfo::FormatDetailContent(const DownloadInfo &taskInfo, std::ostringstream &buffer) const
{
    for (const auto &it: dumpDetailCfg_) {
        auto columnFormatFun = it.second;
        buffer << std::setw(it.first) << (this->*columnFormatFun)(taskInfo);
    }
}

bool DumpTaskInfo::DumpAllTask(int fd) const
{
    std::vector<DownloadInfo> taskVector;
    auto instance = DownloadServiceManager::GetInstance();
    if (instance == nullptr) {
        dprintf(fd, "not enough memory\n");
        return false;
    }
    instance->QueryAllTask(taskVector);
    dprintf(fd, "task num: %lu\n", taskVector.size());
    if (taskVector.empty()) {
        return true;
    }

    DumpAllTaskTile(fd);
    for (const auto &iter: taskVector) {
        std::ostringstream buffer;
        buffer << std::left;
        FormatSummaryContent(iter, buffer);
        dprintf(fd, "%s\n", buffer.str().c_str());
    }
    taskVector.clear();
    return true;
}

bool DumpTaskInfo::DumpTaskDetailInfo(int fd, uint32_t taskId) const
{
    DownloadInfo downloadInfo;
    auto instance = DownloadServiceManager::GetInstance();
    if (instance == nullptr) {
        dprintf(fd, "not enough memory\n");
        return false;
    }
    bool ret = instance->Query(taskId, downloadInfo);
    if (!ret) {
        dprintf(fd, "invalid task id %u\n", taskId);
        return false;
    }

    DumpTaskDetailInfoTile(fd);
    std::ostringstream buffer;
    buffer << std::left;
    FormatSummaryContent(downloadInfo, buffer);
    FormatDetailContent(downloadInfo, buffer);
    dprintf(fd, "%s\n", buffer.str().c_str());
    return true;
}

std::string DumpTaskInfo::DumpTaskID(const DownloadInfo &taskInfo) const
{
    return std::to_string(taskInfo.GetDownloadId());
}

std::string DumpTaskInfo::DumpTaskType(const DownloadInfo &taskInfo) const
{
    return taskInfo.GetTaskType();
}

std::string DumpTaskInfo::DumpTaskStatus(const DownloadInfo &taskInfo) const
{
    DownloadStatus status = taskInfo.GetStatus();
    std::vector<std::pair<DownloadStatus, std::string>> mapping = {
        {SESSION_SUCCESS, "complete"},
        {SESSION_RUNNING, "running"},
        {SESSION_PENDING, "pending"},
        {SESSION_PAUSED,  "pause"},
        {SESSION_FAILED,  "failed"},
        {SESSION_UNKNOWN, "unknown"},
    };

    for (const auto &it: mapping) {
        if (it.first == status) {
            return it.second;
        }
    }
    return "unknown";
}

std::string DumpTaskInfo::DumpFileName(const DownloadInfo &taskInfo) const
{
    return taskInfo.GetFileName();
}

std::string DumpTaskInfo::DumpRoaming(const DownloadInfo &taskInfo) const
{
    return std::to_string(taskInfo.GetRoaming());
}

std::string DumpTaskInfo::DumpNetworkType(const DownloadInfo &taskInfo) const
{
    return std::to_string(taskInfo.GetNetworkType());
}

std::string DumpTaskInfo::DumpMetered(const DownloadInfo &taskInfo) const
{
    return std::to_string(taskInfo.GetMetered());
}

std::string DumpTaskInfo::DumpFileSize(const DownloadInfo &taskInfo) const
{
    return std::to_string(taskInfo.GetDownloadTotalBytes());
}

std::string DumpTaskInfo::DumpTransferredSize(const DownloadInfo &taskInfo) const
{
    return std::to_string(taskInfo.GetDownloadedBytes());
}
}