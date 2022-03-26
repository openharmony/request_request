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

#ifndef CURLADP_H
#define CURLADP_H

#include <vector>
#include <mutex>
#include "curl/curl.h"
#include "curl/easy.h"
#include "upload_common.h"
#include "upload_config.h"
#include "i_upload_task.h"
#include "upload_timer_info.h"

namespace OHOS::Request::Upload {
class CUrlAdp {
public:
    CUrlAdp(std::vector<FileData>& fileArray, std::shared_ptr<UploadConfig>& config);
    virtual ~CUrlAdp();
    void DoUpload(IUploadTask *task);
    bool Remove();
    void FailNotify(unsigned int error);
    bool IsReadAbort()
    {
        return isReadAbort_;
    }

protected:
    bool RemoveInner();
    static int ProgressCallback(void *clientp,
        curl_off_t dltotal, curl_off_t dlnow, curl_off_t ultotal, curl_off_t ulnow);
    static size_t HeaderCallback(char *buffer, size_t size, size_t nitems, void *userdata);
    static size_t ReadCallback(char *buffer, size_t size, size_t nitems, void *arg);

private:
    bool MultiAddHandle(CURLM *curlMulti, std::vector<CURL*>& curlArray);
    void UploadFile();
    void SetCurlOpt(CURL *curl);
    void CheckUploadStatus(CURLM *curlMulti);
    void CurlGlobalInit();
    void CurlGlobalCleanup();
    void InitTimerInfo();
    void StartTimer();
    void StopTimer();

private:
    uint64_t timerId_;
    std::shared_ptr<UploadTimerInfo> timerInfo_;
    IUploadTask *uploadTask_;
    std::vector<FileData> fileArray_;
    FileData  mfileData_;
    std::shared_ptr<UploadConfig> config_;
    std::mutex mutex_;
    std::mutex curlMutex_;
    std::mutex setAbortMutex_;
    std::mutex readMutex_;
    bool isCurlGlobalInit_;
    bool isReadAbort_;
    CURLM *curlMulti_;
    std::vector<CURL*> curlArray_;
};
} // end of OHOS::Request::Upload
#endif