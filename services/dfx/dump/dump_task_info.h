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

#ifndef DUMP_TASK_INFO_H
#define DUMP_TASK_INFO_H

#include <vector>
#include <string>
#include <iosfwd>
#include "i_dumper.h"

namespace OHOS::Request::Download {
class DownloadInfo;
class DumpTaskInfo : public IDumper {
public:
    DumpTaskInfo() = default;
    ~DumpTaskInfo() override {};

    bool Dump(int fd, const std::vector<std::string> &args) override;
private:
    void DumpAllTaskTile(int fd) const;
    bool DumpAllTask(int fd) const;
    void DumpTaskDetailInfoTile(int fd) const;
    bool DumpTaskDetailInfo(int fd, uint32_t taskId) const;
    void FormatSummaryTitle(std::ostringstream &buffer) const;
    void FormatDetailTitle(std::ostringstream &buffer) const;
    void FormatSummaryContent(const DownloadInfo &taskInfo, std::ostringstream &buffer) const;
    void FormatDetailContent(const DownloadInfo &taskInfo, std::ostringstream &buffer) const;
private:
    std::string DumpTaskID(const DownloadInfo &taskInfo) const;
    std::string DumpTaskType(const DownloadInfo &taskInfo) const;
    std::string DumpTaskStatus(const DownloadInfo &taskInfo) const;
    std::string DumpFileName(const DownloadInfo &taskInfo) const;
    std::string DumpRoaming(const DownloadInfo &taskInfo) const;
    std::string DumpNetworkType(const DownloadInfo &taskInfo) const;
    std::string DumpMetered(const DownloadInfo &taskInfo) const;
    std::string DumpFileSize(const DownloadInfo &taskInfo) const;
    std::string DumpTransferredSize(const DownloadInfo &taskInfo) const;
private:
    const int32_t columnWidthInt = 12;
    const int32_t columnWidthShort = 8;
    const int32_t columnWidthFileName = 256;

    using ColumnDumpFunc = std::string (DumpTaskInfo::*)(const DownloadInfo &taskInfo) const;
    std::vector<std::pair<int32_t, std::string>> summaryColumnTitle_ = {
        {columnWidthInt, "id"},
        {columnWidthInt, "type"},
        {columnWidthInt, "status"},
    };

    std::vector<std::pair<int32_t, ColumnDumpFunc>> dumpSummaryCfg_ = {
        {columnWidthInt, &DumpTaskInfo::DumpTaskID},
        {columnWidthInt, &DumpTaskInfo::DumpTaskType},
        {columnWidthInt, &DumpTaskInfo::DumpTaskStatus},
    };

    std::vector<std::pair<int32_t, std::string>> detailColumnTitle_ = {
        {columnWidthShort, "roaming"},
        {columnWidthShort, "network"},
        {columnWidthShort, "meter"},
        {columnWidthInt, "file_size"},
        {columnWidthInt, "tran_size "},
        {columnWidthFileName, "file_name"},
    };

    std::vector<std::pair<int32_t, ColumnDumpFunc>> dumpDetailCfg_ = {
        {columnWidthShort, &DumpTaskInfo::DumpRoaming},
        {columnWidthShort, &DumpTaskInfo::DumpNetworkType},
        {columnWidthShort, &DumpTaskInfo::DumpMetered},
        {columnWidthInt, &DumpTaskInfo::DumpFileSize},
        {columnWidthInt, &DumpTaskInfo::DumpTransferredSize},
        {columnWidthFileName, &DumpTaskInfo::DumpFileName},
    };
};
}
#endif // DUMP_TASK_INFO_H
