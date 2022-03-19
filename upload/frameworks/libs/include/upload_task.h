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

#ifndef UPLOAD_TASK_
#define UPLOAD_TASK_

#include <cstdio>
#include <thread>
#include <pthread.h>
#include "curl/curl.h"
#include "curl/easy.h"
#include "upload_common.h"
#include "i_header_receive_callback.h"
#include "i_progress_callback.h"
#include "i_fail_callback.h"
#include "i_upload_task.h"
#include "upload_config.h"
#include "curl_adp.h"
#include "obtain_file.h"
#include "upload_hilog_wrapper.h"

#include "context.h"
#include "ability_context.h"
#include "data_ability_helper.h"

namespace OHOS::Request::Upload {
enum UploadTaskState {
    STATE_INIT,
    STATE_RUNNING,
    STATE_SUCCESS,
    STATE_FAILURE,
};

enum UploadErrorCode {
    UPLOAD_ERRORCODE_NO_ERROR = 0,
    UPLOAD_ERRORCODE_UNSUPPORT_URI,
    UPLOAD_ERRORCODE_GET_FILE_ERROR,
    UPLOAD_ERRORCODE_CONFIG_ERROR,
    UPLOAD_ERRORCODE_UPLOAD_LIB_ERROR,
    UPLOAD_ERRORCODE_UPLOAD_FAIL,
};

class UploadTask : public IUploadTask {
public:
    UPLOAD_API UploadTask(std::shared_ptr<UploadConfig>& uploadConfig);
    UPLOAD_API virtual ~UploadTask();
    UPLOAD_API virtual bool Remove();
    UPLOAD_API virtual void On(Type type, void *callback);
    UPLOAD_API virtual void Off(Type type, void *callback);
    UPLOAD_API void ExecuteTask();
    static void Run(void *arg);
    virtual void OnRun();

    UPLOAD_API virtual void SetCallback(Type type, void *callback);
    UPLOAD_API virtual void SetContext(std::shared_ptr<OHOS::AppExecFwk::Context> context);
    virtual void OnProgress(curl_off_t dltotal, curl_off_t dlnow, curl_off_t ultotal, curl_off_t ulnow);
    virtual void OnHeaderReceive(char *buffer, size_t size, size_t nitems);
    virtual void OnFail(unsigned int error);
    std::vector<std::string> StringSplit(const std::string& str, char delim);

protected:
    std::vector<FileData>& GetFileArray();
    void ClearFileArray();
private:
    std::shared_ptr<UploadConfig> uploadConfig_;
    std::unique_ptr<std::thread> thread_;

    IProgressCallback* progressCallback_;
    IHeaderReceiveCallback* headerReceiveCallback_;
    IFailCallback* failCallback_;
    std::shared_ptr<CUrlAdp> curlAdp_;
    std::shared_ptr<ObtainFile> obtainFile_;
    std::shared_ptr<OHOS::AppExecFwk::Context> context_;
    int64_t uploadedSize_;
    int64_t totalSize_;
    unsigned int error_;
    std::string header_;
    std::vector<FileData> fileArray_;
    UploadTaskState state_;
    std::mutex mutex_;
    std::thread::native_handle_type thread_handle_;
};
} // end of OHOS::Request::Upload
#endif