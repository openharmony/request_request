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

#ifndef LEGACY_DOWNLOAD_TASK_H
#define LEGACY_DOWNLOAD_TASK_H

#include <cstdio>
#include <functional>
#include <thread>
#include <vector>
#include "curl/curl.h"

namespace OHOS::Request::Download::Legacy {
class DownloadTask {
public:
    struct DownloadOption {
        std::string url_;
        std::string filename_;
        std::string fileDir_;
        std::vector<std::string> header_;
    };

    using DoneFunc = std::function<void(const std::string&, bool, const std::string&)>;
    DownloadTask(const std::string &token, const DownloadOption &option, const DoneFunc &callback);

    ~DownloadTask();

    void Start();

    void Run();

    bool DoDownload();

    void SetResumeFromLarge(CURL *curl, uint64_t pos);
private:
    FILE *OpenDownloadFile() const;

    uint32_t GetLocalFileSize();

    bool SetOption(CURL *handle, curl_slist *&headers);

    void NotifyDone(bool successful, const std::string& errMsg = "");

    bool GetFileSize(uint32_t &result);

    std::string taskId_;
    DownloadOption option_;
    DoneFunc callback_;
    std::thread *thread_ {};
    FILE *filp_ {};
    char *errorBuffer_ {};
    static bool isCurlGlobalInited_;

    uint32_t totalSize_;
    bool hasFileSize_;
};
}
#endif // LEGACY_DOWNLOAD_TASK_H