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

#include <iomanip>
#include "download_service_manager.h"
#include "dump_task_info.h"

namespace OHOS::Request::Download {
bool DumpTaskInfo::Dump(int fd, const std::vector<std::string> &args)
{
    int32_t argsNum = args.size();
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

void DumpTaskInfo::FormatSummaryContent(const std::shared_ptr<DownloadInfo> taskInfo, std::ostringstream &buffer) const
{
    for (const auto &it: dumpSummaryCfg_) {
        buffer << std::setw(it.first) << it.second(taskInfo);
    }
}

void DumpTaskInfo::FormatDetailContent(const std::shared_ptr<DownloadInfo> taskInfo, std::ostringstream &buffer) const
{
    for (const auto &it: dumpDetailCfg_) {
        buffer << std::setw(it.first) << it.second(taskInfo);
    }
}

bool DumpTaskInfo::DumpAllTask(int fd) const
{
    std::map<uint32_t, std::shared_ptr<DownloadInfo>> taskMap;
    DownloadServiceManager::GetInstance().QueryAllTask(taskMap);
    dprintf(fd, "task num: %u\n", taskMap.size());
    if (taskMap.size() == 0) {
        return true;
    }

    DumpAllTaskTile(fd);
    for (const auto &iter: taskMap) {
        std::ostringstream buffer;
        buffer << std::left;
        FormatSummaryContent(iter.second, buffer);
        dprintf(fd, "%s\n", buffer.str().c_str());
    }
    return true;
}

bool DumpTaskInfo::DumpTaskDetailInfo(int fd, uint32_t taskId) const
{
    std::shared_ptr<DownloadInfo> downloadInfo = std::make_shared<DownloadInfo>();
    bool ret = DownloadServiceManager::GetInstance().Query(taskId, *downloadInfo);
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

const std::string DumpTaskInfo::DumpTaskID(std::shared_ptr<DownloadInfo> taskInfo)
{
    return std::to_string(taskInfo->GetDownloadId());
}

const std::string DumpTaskInfo::DumpTaskType(std::shared_ptr<DownloadInfo> taskInfo)
{
    return "download";
}

const std::string DumpTaskInfo::DumpTaskStatus(std::shared_ptr<DownloadInfo> taskInfo)
{
    DownloadStatus status = taskInfo->GetStatus();
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

const std::string DumpTaskInfo::DumpFileName(std::shared_ptr<DownloadInfo> taskInfo)
{
    return taskInfo->GetFileName();
}

const std::string DumpTaskInfo::DumpRoaming(std::shared_ptr<DownloadInfo> taskInfo)
{
    return std::to_string(taskInfo->GetRoaming());
}

const std::string DumpTaskInfo::DumpNetworkType(std::shared_ptr<DownloadInfo> taskInfo)
{
    return std::to_string(taskInfo->GetNetworkType());
}

const std::string DumpTaskInfo::DumpMetered(std::shared_ptr<DownloadInfo> taskInfo)
{
    return std::to_string(taskInfo->GetMetered());
}

const std::string DumpTaskInfo::DumpFileSize(std::shared_ptr<DownloadInfo> taskInfo)
{
    return std::to_string(taskInfo->GetDownloadTotalBytes());
}

const std::string DumpTaskInfo::DumpTransferredSize(std::shared_ptr<DownloadInfo> taskInfo)
{
    return std::to_string(taskInfo->GetDownloadedBytes());
}
}