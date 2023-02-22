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
#include <vector>
#include <pthread.h>
#include "curl/curl.h"
#include "curl/easy.h"
#include "upload_common.h"
#include "i_header_receive_callback.h"
#include "i_progress_callback.h"
#include "i_notify_callback.h"
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

class UploadTaskNapiV5;
class UploadTask : public IUploadTask, public std::enable_shared_from_this<UploadTask> {
public:
    UPLOAD_API UploadTask(std::shared_ptr<UploadConfig>& uploadConfig);
    UPLOAD_API ~UploadTask();
    UPLOAD_API bool Remove();
    UPLOAD_API void On(Type type, void *callback);
    UPLOAD_API void Off(Type type, void *callback);
    UPLOAD_API void ExecuteTask();
    static void Run(std::shared_ptr<Upload::UploadTask> task);
    void OnRun();

    UPLOAD_API void SetCallback(Type type, void *callback);
    UPLOAD_API void SetContext(std::shared_ptr<OHOS::AbilityRuntime::Context> context);
    UPLOAD_API void SetUploadProxy(std::shared_ptr<UploadTaskNapiV5> proxy);
    void OnProgress(curl_off_t ulnow) override;
    void OnHeaderReceive(const std::string &header) override;
    void OnFail() override;
    void OnComplete() override;

protected:
    uint32_t InitFileArray();
    void ClearFileArray();
    std::string GetCodeMessage(uint32_t code);
    std::vector<TaskState> GetTaskStates();
private:
    void ReportTaskFault(uint32_t ret) const;
    uint32_t StartUploadFile();

    std::shared_ptr<UploadConfig> uploadConfig_;
    std::unique_ptr<std::thread> thread_;
    static constexpr const char *REQUEST_TASK_FAULT = "REQUEST_TASK_FAULT";
    static constexpr const char *TASKS_TYPE = "TASKS_TYPE";
    static constexpr const char *UPLOAD = "UPLOAD";
    static constexpr const char *TOTAL_FILE_NUM = "TOTAL_FILE_NUM";
    static constexpr const char *FAIL_FILE_NUM = "FAIL_FILE_NUM";
    static constexpr const char *SUCCESS_FILE_NUM = "SUCCESS_FILE_NUM";
    static constexpr const char *ERROR_INFO = "ERROR_INFO";

    IProgressCallback* progressCallback_;
    IHeaderReceiveCallback* headerReceiveCallback_;
    INotifyCallback* failCallback_;
    INotifyCallback* completeCallback_;

    std::shared_ptr<CUrlAdp> curlAdp_;
    std::shared_ptr<UploadTaskNapiV5> uploadProxy_;
    std::shared_ptr<OHOS::AbilityRuntime::Context> context_;
    int64_t uploadedSize_;
    int64_t totalSize_;
    std::vector<std::string> headerArray_;
    std::string header_;
    std::vector<FileData> fileDatas_;
    std::vector<TaskState> taskStates_;
    UploadTaskState state_;
    std::mutex mutex_;
    bool isRemoved_ {false};
    std::mutex removeMutex_;
    std::thread::native_handle_type thread_handle_;
    static constexpr int USLEEP_INTERVEL_BEFOR_RUN = 50 * 1000;
};
} // end of OHOS::Request::Upload
#endif