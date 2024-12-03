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

#include <pthread.h>

#include <cstdio>
#include <thread>
#include <vector>

#include "ability_context.h"
#include "context.h"
#include "curl/curl.h"
#include "curl/easy.h"
#include "data_ability_helper.h"
#include "i_upload_task.h"
#include "upload/curl_adp.h"
#include "upload/obtain_file.h"
#include "upload/upload_common.h"
#include "upload/upload_hilog_wrapper.h"
#include "upload_config.h"

namespace OHOS::Request::Upload {
enum UploadTaskState {
    STATE_INIT,
    STATE_RUNNING,
    STATE_SUCCESS,
    STATE_FAILURE,
};
class UploadTaskNapiV5;
class UploadTask
    : public IUploadTask
    , public std::enable_shared_from_this<UploadTask> {
public:
    UPLOAD_API UploadTask(std::shared_ptr<UploadConfig> &uploadConfig);
    UPLOAD_API ~UploadTask();
    UPLOAD_API bool Remove();
    UPLOAD_API void ExecuteTask();
    static void Run(std::shared_ptr<Upload::UploadTask> task);
    void OnRun();

    UPLOAD_API void SetCallback(Type type, void *callback);
    UPLOAD_API void SetContext(std::shared_ptr<OHOS::AbilityRuntime::Context> context);
    UPLOAD_API void SetUploadProxy(std::shared_ptr<UploadTaskNapiV5> proxy);

protected:
    uint32_t InitFileArray();
    void ClearFileArray();

private:
    void OnFail();
    void OnComplete();
    void ReportTaskFault(uint32_t ret) const;
    uint32_t StartUploadFile();

    std::shared_ptr<UploadConfig> uploadConfig_;
    std::unique_ptr<std::thread> thread_;
    std::shared_ptr<CUrlAdp> curlAdp_;
    std::shared_ptr<UploadTaskNapiV5> uploadProxy_;
    std::shared_ptr<OHOS::AbilityRuntime::Context> context_;
    int64_t uploadedSize_;
    int64_t totalSize_;
    std::vector<std::string> headerArray_;
    std::string header_;
    std::vector<FileData> fileDatas_;
    UploadTaskState state_;
    std::mutex mutex_;
    bool isRemoved_{ false };
    std::mutex removeMutex_;
    std::thread::native_handle_type thread_handle_;
    static constexpr int USLEEP_INTERVAL_BEFORE_RUN = 50 * 1000;
};
} // namespace OHOS::Request::Upload
#endif