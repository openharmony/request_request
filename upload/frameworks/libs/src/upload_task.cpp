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

#include "upload_task.h"

#include <thread>

#include "curl/curl.h"
#include "curl/easy.h"
#include "hisysevent.h"
#include "hitrace_meter.h"

namespace OHOS::Request::Upload {
UploadTask::UploadTask(std::shared_ptr<UploadConfig> &uploadConfig)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "UploadTask. In.");
    uploadConfig_ = uploadConfig;
    curlAdp_ = nullptr;
    state_ = STATE_INIT;
    uploadedSize_ = 0;
    totalSize_ = 0;
    progressCallback_ = nullptr;
    headerReceiveCallback_ = nullptr;
    failCallback_ = nullptr;
    completeCallback_ = nullptr;
    context_ = nullptr;
    isRemoved_ = false;
}

UploadTask::~UploadTask()
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "~UploadTask. In.");
    std::lock_guard<std::mutex> guard(mutex_);
    progressCallback_ = nullptr;
    headerReceiveCallback_ = nullptr;
    failCallback_ = nullptr;
    completeCallback_ = nullptr;
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

void UploadTask::On(Type type, void *callback)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "On. In.");
    std::lock_guard<std::mutex> guard(mutex_);
    SetCallback(type, callback);
}

void UploadTask::Off(Type type, void *callback)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "Off. In.");

    std::lock_guard<std::mutex> guard(mutex_);
    if (callback == nullptr) {
        return;
    }

    if (type == TYPE_PROGRESS_CALLBACK && progressCallback_ != nullptr) {
        (static_cast<IProgressCallback *>(callback))->Progress(uploadedSize_, totalSize_);
    }
    if (type == TYPE_HEADER_RECEIVE_CALLBACK && headerReceiveCallback_ != nullptr) {
        (static_cast<IHeaderReceiveCallback *>(callback))->HeaderReceive(header_);
    }
    if (type == TYPE_COMPLETE_CALLBACK && completeCallback_ != nullptr) {
        (static_cast<INotifyCallback *>(callback))->Notify(taskStates_);
    }
    if (type == TYPE_FAIL_CALLBACK && failCallback_ != nullptr) {
        (static_cast<INotifyCallback *>(callback))->Notify(taskStates_);
    }
    SetCallback(type, nullptr);
}

void UploadTask::SetCallback(Type type, void *callback)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "SetCallback. In.");
    if (type == TYPE_PROGRESS_CALLBACK) {
        progressCallback_ = (IProgressCallback *)callback;
        if (progressCallback_ && uploadedSize_ > 0) {
            progressCallback_->Progress(uploadedSize_, totalSize_);
        }
    } else if (type == TYPE_HEADER_RECEIVE_CALLBACK) {
        headerReceiveCallback_ = (IHeaderReceiveCallback *)callback;
        if (headerReceiveCallback_ && headerArray_.empty() == false) {
            for (auto header : headerArray_) {
                if (header.length() > 0) {
                    headerReceiveCallback_->HeaderReceive(header);
                }
            }
            headerArray_.clear();
        }
    } else if (type == TYPE_FAIL_CALLBACK) {
        failCallback_ = (INotifyCallback *)callback;
        if (failCallback_ && state_ == STATE_FAILURE) {
            failCallback_->Notify(taskStates_);
        }
    } else if (type == TYPE_COMPLETE_CALLBACK) {
        completeCallback_ = (INotifyCallback *)callback;
        if (completeCallback_ && state_ == STATE_SUCCESS) {
            completeCallback_->Notify(taskStates_);
        }
    } else {
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "SetCallback. type[%{public}d] not match.", type);
    }
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
    usleep(USLEEP_INTERVEL_BEFOR_RUN);
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
    if (task->uploadConfig_->protocolVersion == API5) {
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

std::string UploadTask::GetCodeMessage(uint32_t code)
{
    std::vector<std::pair<UploadErrorCode, std::string>> codeMap = {
        { UPLOAD_OK, "file uploaded successfully" },
        { UPLOAD_ERRORCODE_UNSUPPORT_URI, "file path error" },
        { UPLOAD_ERRORCODE_GET_FILE_ERROR, "failed to get file" },
        { UPLOAD_ERRORCODE_CONFIG_ERROR, "upload configuration error" },
        { UPLOAD_ERRORCODE_UPLOAD_LIB_ERROR, "libcurl return error" },
        { UPLOAD_ERRORCODE_UPLOAD_FAIL, "upload failed" },
        { UPLOAD_ERRORCODE_UPLOAD_OUTTIME, "upload timeout" },
        { UPLOAD_TASK_REMOVED, "upload task removed"}
    };

    for (const auto &it : codeMap) {
        if (it.first == code) {
            return it.second;
        }
    }
    return "unknown";
}

void UploadTask::OnRun()
{
    std::string traceParam = "url:" + uploadConfig_->url + "file num:" + std::to_string(uploadConfig_->files.size());
    HitraceScoped trace(HITRACE_TAG_MISC, "exec upload task " + traceParam);
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "OnRun. In.");
    state_ = STATE_RUNNING;
    uint32_t ret = StartUploadFile();
    std::lock_guard<std::mutex> guard(removeMutex_);
    if (!isRemoved_) {
        if (ret != UPLOAD_OK) {
            UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "ret != UPLOAD_OK");
            OnFail();
            ReportTaskFault(ret);
        } else {
            OnComplete();
        }
        ClearFileArray();
    }
    totalSize_ = 0;
}

void UploadTask::ReportTaskFault(uint32_t ret) const
{
    uint32_t successCount = 0;
    uint32_t failCount = 0;
    for (auto &vmem : fileDatas_) {
        if (vmem.result == UPLOAD_OK) {
            successCount++;
        } else {
            failCount++;
        }
    }
    OHOS::HiviewDFX::HiSysEvent::Write(OHOS::HiviewDFX::HiSysEvent::Domain::REQUEST, REQUEST_TASK_FAULT,
        OHOS::HiviewDFX::HiSysEvent::EventType::FAULT, TASKS_TYPE, UPLOAD, TOTAL_FILE_NUM, fileDatas_.size(),
        FAIL_FILE_NUM, failCount, SUCCESS_FILE_NUM, successCount, ERROR_INFO, static_cast<int>(ret));
}

void UploadTask::OnProgress(curl_off_t ulnow)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "OnProgress. In.");
    if (isRemoved_) {
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "OnProgress isRemoved");
        return;
    }
    if (ulnow == uploadedSize_) {
        return;
    }

    std::lock_guard<std::mutex> guard(mutex_);
    uploadedSize_ = ulnow;
    if (uploadedSize_ == totalSize_) {
        state_ = STATE_SUCCESS;
    }
    if (progressCallback_) {
        progressCallback_->Progress(uploadedSize_, totalSize_);
    }
}

void UploadTask::OnHeaderReceive(const std::string &header)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "OnHeaderReceive. In.");
    if (isRemoved_) {
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "OnHeaderReceive isRemoved");
        return;
    }
    std::lock_guard<std::mutex> guard(mutex_);
    header_ = header;
    if (headerReceiveCallback_) {
        headerReceiveCallback_->HeaderReceive(header_);
    } else {
        headerArray_.push_back(header);
    }
}

std::vector<TaskState> UploadTask::GetTaskStates()
{
    std::vector<TaskState> taskStates;
    TaskState taskState;
    for (auto &vmem : fileDatas_) {
        taskState = { vmem.filename, vmem.result, GetCodeMessage(vmem.result) };
        taskStates.push_back(taskState);
    }
    return taskStates;
}
void UploadTask::OnFail()
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "OnFail. In.");
    if (isRemoved_) {
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "OnFail isRemoved");
        return;
    }
    if (uploadConfig_->protocolVersion == API5) {
        return;
    }
    std::lock_guard<std::mutex> guard(mutex_);
    std::vector<TaskState> taskStates = GetTaskStates();
    taskStates_ = taskStates;
    state_ = STATE_FAILURE;
    if (failCallback_) {
        failCallback_->Notify(taskStates);
    }
}

void UploadTask::OnComplete()
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "OnComplete. In.");
    if (isRemoved_) {
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "OnComplete isRemoved");
        return;
    }
    if (uploadConfig_->protocolVersion == API5) {
        return;
    }
    std::lock_guard<std::mutex> guard(mutex_);
    std::vector<TaskState> taskStates = GetTaskStates();
    taskStates_ = taskStates;
    state_ = STATE_SUCCESS;
    if (completeCallback_) {
        completeCallback_->Notify(taskStates);
    }
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