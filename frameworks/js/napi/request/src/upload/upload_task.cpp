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

#include "upload/upload_task.h"

#include <pthread.h>

#include <thread>

#include "curl/curl.h"
#include "curl/easy.h"

namespace OHOS::Request::Upload {
UploadTask::UploadTask(std::shared_ptr<UploadConfig> &uploadConfig)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "UploadTask. In.");
    uploadConfig_ = uploadConfig;
    curlAdp_ = nullptr;
    state_ = STATE_INIT;
    uploadedSize_ = 0;
    totalSize_ = 0;
    context_ = nullptr;
    isRemoved_ = false;
}

UploadTask::~UploadTask()
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "~UploadTask. In.");
    std::lock_guard<std::mutex> guard(mutex_);
    if (!isRemoved_) {
        Remove();
    }
}

bool UploadTask::Remove()
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "Remove. In.");
    std::lock_guard<std::mutex> guard(removeMutex_);
    isRemoved_ = true;
    if (curlAdp_ != nullptr) {
        curlAdp_->Remove();
    }
    ClearFileArray();
    return true;
}

void UploadTask::SetContext(std::shared_ptr<OHOS::AbilityRuntime::Context> context)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "SetContext. In.");
    context_ = context;
}

void UploadTask::SetUploadProxy(std::shared_ptr<UploadTaskNapiV5> proxy)
{
    uploadProxy_ = proxy;
}

void UploadTask::Run(std::shared_ptr<Upload::UploadTask> task)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "Run. In.");
    pthread_setname_np(pthread_self(), "upload_task");
    usleep(USLEEP_INTERVAL_BEFORE_RUN);
    if (task == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "task == nullptr");
        return;
    }
    task->OnRun();
    std::lock_guard<std::mutex> guard(task->removeMutex_);
    if (task->isRemoved_) {
        task->SetUploadProxy(nullptr);
        return;
    }
    if (task->uploadConfig_->protocolVersion == API3) {
        if (task->uploadConfig_->fcomplete) {
            task->uploadConfig_->fcomplete();
            UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "Complete.");
        }
    }
    task->SetUploadProxy(nullptr);
}

uint32_t UploadTask::InitFileArray()
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "InitFileArray. In.");
    unsigned int fileSize = 0;
    FileData data;
    FILE *file;
    totalSize_ = 0;
    uint32_t initResult = UPLOAD_OK;
    ObtainFile obtainFile;
    uint32_t index = 1;
    for (auto f : uploadConfig_->files) {
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "filename is %{public}s", f.filename.c_str());
        data.result = UPLOAD_ERRORCODE_UPLOAD_FAIL;
        uint32_t ret = obtainFile.GetFile(&file, f.uri, fileSize, context_);
        if (ret != UPLOAD_OK) {
            initResult = data.result;
            data.result = ret;
        }

        data.fp = file;
        std::size_t position = f.uri.find_last_of("/");
        if (position != std::string::npos) {
            data.filename = std::string(f.uri, position + 1);
            data.filename.erase(data.filename.find_last_not_of(" ") + 1);
        }
        data.name = f.name;
        data.type = f.type;
        data.fileIndex = index++;
        data.adp = nullptr;
        data.upsize = 0;
        data.totalsize = fileSize;
        data.list = nullptr;
        data.headSendFlag = 0;
        data.httpCode = 0;

        fileDatas_.push_back(data);
        totalSize_ += static_cast<int64_t>(fileSize);
    }

    return initResult;
}

uint32_t UploadTask::StartUploadFile()
{
    if (isRemoved_) {
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "upload task removed");
        return UPLOAD_TASK_REMOVED;
    }
    uint32_t ret = InitFileArray();
    if (ret != UPLOAD_OK) {
        return ret;
    }
    curlAdp_ = std::make_shared<CUrlAdp>(fileDatas_, uploadConfig_);
    return curlAdp_->DoUpload(shared_from_this());
}

void UploadTask::OnRun()
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "OnRun. In.");
    state_ = STATE_RUNNING;
    uint32_t ret = StartUploadFile();
    std::lock_guard<std::mutex> guard(removeMutex_);
    if (!isRemoved_) {
        if (ret != UPLOAD_OK) {
            UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "ret != UPLOAD_OK");
            state_ = STATE_FAILURE;
        } else {
            state_ = STATE_SUCCESS;
        }
        ClearFileArray();
    }
    totalSize_ = 0;
}

void UploadTask::ExecuteTask()
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "ExecuteTask. In.");
    thread_ = std::make_unique<std::thread>(UploadTask::Run, shared_from_this());
    thread_handle_ = thread_->native_handle();
    thread_->detach();
}

void UploadTask::ClearFileArray()
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "ClearFileArray()");
    if (fileDatas_.empty()) {
        return;
    }
    for (auto &file : fileDatas_) {
        if (file.fp != NULL) {
            fclose(file.fp);
        }
        file.name = "";
    }
    fileDatas_.clear();
}
} // namespace OHOS::Request::Upload