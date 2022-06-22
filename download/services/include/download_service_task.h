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

#ifndef DOWNLOAD_TASK_H
#define DOWNLOAD_TASK_H

#include <mutex>
#include <string>
#include <vector>

#include "constant.h"
#include "curl/curl.h"
#include "download_config.h"
#include "download_info.h"

namespace OHOS::Request::Download {
    using DownloadTaskCallback = void(*)(const std::string& type, uint32_t taskId, uint32_t argv1, uint32_t argv2);

class DownloadServiceTask {
public:
    DownloadServiceTask(uint32_t taskId, const DownloadConfig &config);
    ~DownloadServiceTask(void);

    uint32_t GetId() const;
    bool Run();
    bool Pause();
    bool Resume();
    bool Remove();
    bool Query(DownloadInfo &info);
    bool QueryMimeType(std::string &mimeType);

    void InstallCallback(DownloadTaskCallback cb);
    void GetRunResult(DownloadStatus &status, ErrorCode &code, PausedReason &reason);

    void SetRetryTime(uint32_t retryTime);
    void SetNetworkStatus(bool isOnline);
    bool IsSatisfiedConfiguration();
private:
    void SetStatus(DownloadStatus status, ErrorCode code, PausedReason reason);
    void SetStatus(DownloadStatus status);
    void SetError(ErrorCode code);
    void SetReason(PausedReason reason);

    void DumpStatus();
    void DumpErrorCode();
    void DumpPausedReason();

    bool ExecHttp();
    bool SetFileSizeOption(CURL *curl, struct curl_slist *requestHeader);
    bool SetOption(CURL *curl, struct curl_slist *requestHeader);
    struct curl_slist *MakeHeaders(const std::vector<std::string> &vec);

    void SetResumeFromLarge(CURL *curl, long long pos);

    bool GetFileSize(uint32_t &result);
    std::string GetTmpPath();
    void HandleResponseCode(CURLcode code, int32_t httpCode);
    void HandleCleanup(DownloadStatus status);

    static size_t WriteCallback(void *buffer, size_t size, size_t num, void *param);
    static size_t HeaderCallback(void *buffer, size_t size, size_t num, void *param);
    static int ProgressCallback(void *param, double dltotal, double dlnow, double ultotal, double ulnow);

    bool CheckResumeCondition();
    void ForceStopRunning();
    bool HandleFileError();

private:
    uint32_t taskId_;
    DownloadConfig config_;

    DownloadStatus status_;
    ErrorCode code_;
    PausedReason reason_;
    std::string mimeType_;
    uint32_t totalSize_;
    uint32_t downloadSize_;
    bool isPartialMode_;

    bool forceStop_;
    bool isRemoved_;
    uint32_t retryTime_;

    DownloadTaskCallback eventCb_;
    std::recursive_mutex mutex_;
    bool hasFileSize_;
    bool isOnline_;
    uint32_t prevSize_;
};
} // namespace OHOS::Request::Download
#endif // DOWNLOAD_TASK_H
