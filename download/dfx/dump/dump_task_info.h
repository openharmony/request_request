/*
 * Copyright (c) 2022 Huawei Device Co., Ltd.
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
#include <memory>
#include <sstream>
#include "i_dumper.h"

namespace OHOS::Request::Download {
class DownloadInfo;
class DumpTaskInfo : public IDumper {
public:
    DumpTaskInfo() = default;
    ~DumpTaskInfo() override {};

    bool Dump(int fd, const std::vector<std::string> &args) override;
public:
    static const std::string DumpTaskID(std::shared_ptr<DownloadInfo> taskInfo);
    static const std::string DumpTaskType(std::shared_ptr<DownloadInfo> taskInfo);
    static const std::string DumpTaskStatus(std::shared_ptr<DownloadInfo> taskInfo);
    static const std::string DumpFileName(std::shared_ptr<DownloadInfo> taskInfo);
    static const std::string DumpRoaming(std::shared_ptr<DownloadInfo> taskInfo);
    static const std::string DumpNetworkType(std::shared_ptr<DownloadInfo> taskInfo);
    static const std::string DumpMetered(std::shared_ptr<DownloadInfo> taskInfo);
    static const std::string DumpFileSize(std::shared_ptr<DownloadInfo> taskInfo);
    static const std::string DumpTransferredSize(std::shared_ptr<DownloadInfo> taskInfo);
private:
    void DumpAllTaskTile(int fd) const;
    bool DumpAllTask(int fd) const;
    void DumpTaskDetailInfoTile(int fd) const;
    bool DumpTaskDetailInfo(int fd, uint32_t taskId) const;
    void FormatSummaryTitle(std::ostringstream &buffer) const;
    void FormatDetailTitle(std::ostringstream &buffer) const;
    void FormatSummaryContent(const std::shared_ptr<DownloadInfo> taskInfo, std::ostringstream &buffer) const;
    void FormatDetailContent(const std::shared_ptr<DownloadInfo> taskInfo, std::ostringstream &buffer) const;
private:
    static const int32_t columnWidthInt = 12;
    static const int32_t columnWidthShort = 8;
    static const int32_t columnWidthFileName = 256;

    using columnDumpFunc = std::function<const std::string (std::shared_ptr<DownloadInfo>)>;
    std::vector<std::pair<int32_t, std::string>> summaryColumnTitle_ = {
        {columnWidthInt, "id"},
        {columnWidthInt, "type"},
        {columnWidthInt, "status"},
    };

    std::vector<std::pair<int32_t, columnDumpFunc>> dumpSummaryCfg_ = {
        {columnWidthInt, DumpTaskID},
        {columnWidthInt, DumpTaskType},
        {columnWidthInt, DumpTaskStatus},
    };

    std::vector<std::pair<int32_t, std::string>> detailColumnTitle_ = {
        {columnWidthShort, "roaming"},
        {columnWidthShort, "network"},
        {columnWidthShort, "meter"},
        {columnWidthInt, "file_size"},
        {columnWidthInt, "tran_size "},
        {columnWidthFileName, "file_name"},
    };

    std::vector<std::pair<int32_t, columnDumpFunc>> dumpDetailCfg_ = {
        {columnWidthShort, DumpRoaming},
        {columnWidthShort, DumpNetworkType},
        {columnWidthShort, DumpMetered},
        {columnWidthInt, DumpFileSize},
        {columnWidthInt, DumpTransferredSize},
        {columnWidthFileName, DumpFileName},
    };
};
}
#endif // DUMP_TASK_INFO_H
