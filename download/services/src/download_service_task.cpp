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

#include "download_service_task.h"

#include <algorithm>
#include <cinttypes>
#include <mutex>
#include <unistd.h>
#include <fstream>
#include <sstream>
#include <ios>
#include <map>
#include <memory>
#include <string>
#include <ostream>
#include <utility>
#include <vector>
#include <curl/easy.h>
#include "log.h"
#include "network_adapter.h"
#include "hitrace_meter.h"
#include "task_statistics.h"
#include "task_fault.h"
#include "download_background_notification.h"

namespace OHOS::Request::Download {
namespace {
static constexpr const char *URL_HTTPS = "https";
static constexpr const char *TLS_VERSION_DEFAULT = "CURL_SSLVERSION_TLSv1_2";
static constexpr const char *HTTP_GET_METHOD = "GET";
static constexpr uint32_t HUNDRED_PERCENT = 100;
static constexpr uint32_t TEN_PERCENT_THRESHOLD = 10;
static constexpr uint32_t NOTIFICATION_FREQUENCY = 2000;
static constexpr uint32_t RETRY_TIME_MAX = 10;
static constexpr int CURL_REMOVE_TASK = 1;
static constexpr int64_t INVALID_LEN = -1;
}
DownloadServiceTask::DownloadServiceTask(uint32_t taskId, const DownloadConfig &config)
    : taskId_(taskId), config_(config), status_(SESSION_UNKNOWN), code_(ERROR_UNKNOWN), reason_(PAUSED_UNKNOWN),
      mimeType_(""), totalSize_(INVALID_LEN), downloadSize_(INVALID_LEN), isPartialMode_(false), forceStop_(false),
      isRemoved_(false), retryTime_(RETRY_TIME_MAX), eventCb_(nullptr), hasFileSize_(false), isOnline_(true),
      prevSize_(INVALID_LEN)
{
}

DownloadServiceTask::~DownloadServiceTask(void)
{
    DOWNLOAD_HILOGD("Destructed download service task [%{public}d]", taskId_);
    if (config_.GetFD() > 0) {
        close(config_.GetFD());
        config_.SetFD(-1);
    }
}

uint32_t DownloadServiceTask::GetId() const
{
    return taskId_;
}

bool DownloadServiceTask::Run()
{
    DOWNLOAD_HILOGI("Task[%{public}d] start", taskId_);
    if (HandleFileError()) {
        return false;
    }

    if (!NetworkAdapter::GetInstance().IsOnline()) {
        DOWNLOAD_HILOGI("network is offline");
        SetStatus(SESSION_FAILED, ERROR_OFFLINE, PAUSED_UNKNOWN);
        return false;
    }

    uint32_t retryTime = 0;
    bool result = false;
    bool enableTimeout = false;
    SetStatus(SESSION_RUNNING);
    
    do {
        if (!IsSatisfiedConfiguration()) {
            DOWNLOAD_HILOGI("networktype not Satisfied Configuration");
            ForceStopRunning();
            SetStatus(SESSION_FAILED, ERROR_UNSUPPORTED_NETWORK_TYPE, PAUSED_UNKNOWN);
            break;
        }
        enableTimeout = false;
        if (status_ != SESSION_RUNNING && status_ != SESSION_PENDING) {
            break;
        }
        if (!GetFileSize(totalSize_)) {
            SetStatus(SESSION_FAILED, ERROR_HTTP_DATA_ERROR, PAUSED_UNKNOWN);
            DOWNLOAD_HILOGI("get file size fail");
            return false;
        }

        result = ExecHttp();
        DumpStatus();
        DumpErrorCode();
        DumpPausedReason();

        // HTTP timeout occurs, retry
        if (status_ == SESSION_PENDING) {
            enableTimeout = true;
            retryTime++;
        }
    } while (!result && enableTimeout && retryTime < retryTime_);
    if (retryTime >= retryTime_) {
        SetStatus(SESSION_PAUSED, ERROR_UNKNOWN, PAUSED_WAITING_TO_RETRY);
    }
    return result;
}

bool DownloadServiceTask::Pause()
{
    DOWNLOAD_HILOGI("Status [%{public}d], Code [%{public}d], Reason [%{public}d]", status_, code_, reason_);
    if (status_ != SESSION_RUNNING && status_ != SESSION_PENDING) {
        return false;
    }
    ForceStopRunning();

    SetStatus(SESSION_PAUSED, ERROR_UNKNOWN, PAUSED_BY_USER);
    return true;
}

bool DownloadServiceTask::Resume()
{
    DOWNLOAD_HILOGI("Status [%{public}d], Code [%{public}d], Reason [%{public}d]", status_, code_, reason_);
    if (status_ == SESSION_PAUSED || (status_ == SESSION_FAILED && code_ == ERROR_CANNOT_RESUME)) {
        forceStop_ = false;
        if (!CheckResumeCondition()) {
            SetStatus(SESSION_FAILED, ERROR_CANNOT_RESUME, PAUSED_UNKNOWN);
        } else {
            // reset status
            SetStatus(SESSION_UNKNOWN, ERROR_UNKNOWN, PAUSED_UNKNOWN);
        }
        return true;
    }
    return false;
}

bool DownloadServiceTask::Remove()
{
    DOWNLOAD_HILOGI("Task[%{public}d], Status [%{public}d], Code [%{public}d], Reason [%{public}d]",  taskId_,
                    status_, code_, reason_);
    isRemoved_ = true;
    ForceStopRunning();
    if (eventCb_ != nullptr) {
        eventCb_("remove", taskId_, 0, 0, true);
    }
    return true;
}

bool DownloadServiceTask::Query(DownloadInfo &info)
{
    DOWNLOAD_HILOGD("Query Task[%{public}d], current status is %{public}d", taskId_, status_);
    info.SetDescription(config_.GetDescription());
    info.SetDownloadedBytes(downloadSize_);
    info.SetDownloadId(taskId_);
    info.SetFailedReason(code_);
    std::string fileName = config_.GetFilePath().substr(config_.GetFilePath().rfind('/') + 1);
    std::string filePath = config_.GetFilePath().substr(0, config_.GetFilePath().rfind('/'));
    info.SetFileName(fileName);
    info.SetFilePath(filePath);
    info.SetPausedReason(reason_);
    info.SetStatus(status_);
    info.SetTargetURI(config_.GetUrl());
    info.SetDownloadTitle(config_.GetTitle());
    info.SetDownloadTotalBytes(totalSize_);
    info.SetNetworkType(config_.GetNetworkType());
    info.SetMetered(config_.IsMetered());
    info.SetRoaming(config_.IsRoaming());
    return true;
}

bool DownloadServiceTask::QueryMimeType(std::string &mimeType)
{
    DOWNLOAD_HILOGD("Query Mime Type of Task[%{public}d], current status is %{public}d", taskId_, status_);
    mimeType = mimeType_;
    return true;
}

void DownloadServiceTask::InstallCallback(DownloadTaskCallback cb)
{
    eventCb_ = cb;
}

void DownloadServiceTask::GetRunResult(DownloadStatus &status, ErrorCode &code, PausedReason &reason)
{
    status = status_;
    code = code_;
    reason = reason_;
}

void DownloadServiceTask::SetRetryTime(uint32_t retryTime)
{
    retryTime_ = retryTime;
}

void DownloadServiceTask::SetNetworkStatus(bool isOnline)
{
    std::lock_guard<std::recursive_mutex> autoLock(mutex_);
    isOnline_ = isOnline;
    if (status_ == SESSION_PAUSED && reason_ == PAUSED_WAITING_TO_RETRY && !isOnline_) {
        reason_ = PAUSED_WAITING_FOR_NETWORK;
    }
}

void DownloadServiceTask::SetStatus(DownloadStatus status, ErrorCode code, PausedReason reason)
{
    auto stateChange = [this](DownloadStatus status, ErrorCode code, PausedReason reason) -> bool {
        std::lock_guard<std::recursive_mutex> autoLock(mutex_);
        bool isChanged = false;
        if (status != this->status_) {
            this->status_ = status;
            isChanged = true;
        }
        if (code != this->code_) {
            this->code_ = code;
            isChanged = true;
        }
        if (this->reason_ != PAUSED_BY_USER) {
            if (!isOnline_ && reason == PAUSED_WAITING_TO_RETRY) {
                reason = PAUSED_WAITING_FOR_NETWORK;
            }
            if (reason != this->reason_) {
                this->reason_ = reason;
                isChanged = true;
            }
        }

        return isChanged;
    };
    DOWNLOAD_HILOGI("Status [%{public}d], Code [%{public}d], Reason [%{public}d]", status, code, reason);
    if (!stateChange(status, code, reason)) {
        return;
    }
    if (eventCb_ != nullptr) {
        std::lock_guard<std::recursive_mutex> autoLock(mutex_);
        switch (status_) {
            case SESSION_SUCCESS:
                eventCb_("complete", taskId_, 0, 0, true);
                break;

            case SESSION_PAUSED:
                eventCb_("pause", taskId_, 0, 0, true);
                break;

            case SESSION_FAILED:
                eventCb_("fail", taskId_, code_, 0, true);
                break;

            default:
                break;
        }
    }
}

void DownloadServiceTask::SetStatus(DownloadStatus status)
{
    auto stateChange = [this](DownloadStatus status) -> bool {
        std::lock_guard<std::recursive_mutex> autoLock(mutex_);
        if (status == this->status_) {
            DOWNLOAD_HILOGD("ignore same status");
            return false;
        }
        this->status_ = status;
        return true;
    };
    DOWNLOAD_HILOGI("Status [%{public}d]", status);
    if (!stateChange(status)) {
        return;
    }
    if (eventCb_ != nullptr) {
        std::lock_guard<std::recursive_mutex> autoLock(mutex_);
        switch (status_) {
            case SESSION_SUCCESS:
                eventCb_("complete", taskId_, 0, 0, true);
                break;

            case SESSION_PAUSED:
                eventCb_("pause", taskId_, 0, 0, true);
                break;

            case SESSION_FAILED:
                eventCb_("fail", taskId_, code_, 0, true);
                break;

            default:
                break;
        }
    }
}

void DownloadServiceTask::SetError(ErrorCode code)
{
    DOWNLOAD_HILOGI("Code [%{public}d]", code);
    std::lock_guard<std::recursive_mutex> autoLock(mutex_);
    if (code == code_) {
        DOWNLOAD_HILOGD("ignore same error code");
        return;
    }
    code_ = code;
}

void DownloadServiceTask::SetReason(PausedReason reason)
{
    DOWNLOAD_HILOGI("Reason [%{public}d]", reason);
    std::lock_guard<std::recursive_mutex> autoLock(mutex_);

    if (reason_ != PAUSED_BY_USER) {
        if (!isOnline_ && reason == PAUSED_WAITING_TO_RETRY) {
            reason = PAUSED_WAITING_FOR_NETWORK;
        }
        if (reason == reason_) {
            DOWNLOAD_HILOGD("ignore same paused reason");
            return;
        }
        reason_ = reason;
    }
}

void DownloadServiceTask::DumpStatus()
{
    switch (status_) {
        case SESSION_SUCCESS:
            DOWNLOAD_HILOGD("status:	SESSION_SUCCESS");
            break;

        case SESSION_RUNNING:
            DOWNLOAD_HILOGD("status:	SESSION_RUNNING");
            break;

        case SESSION_PENDING:
            DOWNLOAD_HILOGD("status:	SESSION_PENDING");
            break;

        case SESSION_PAUSED:
            DOWNLOAD_HILOGD("status:	SESSION_PAUSED");
            break;

        case SESSION_FAILED:
            DOWNLOAD_HILOGD("status:	SESSION_FAILED");
            break;

        case SESSION_UNKNOWN:
            DOWNLOAD_HILOGD("status:	SESSION_UNKNOWN");
            break;

        default:
            DOWNLOAD_HILOGD("status:	SESSION_UNKNOWN");
            break;
    }
}

void DownloadServiceTask::DumpErrorCode()
{
    switch (code_) {
        case ERROR_CANNOT_RESUME:
            DOWNLOAD_HILOGD("error code:	ERROR_CANNOT_RESUME");
            break;

        case ERROR_DEVICE_NOT_FOUND:
            DOWNLOAD_HILOGD("error code:	ERROR_DEVICE_NOT_FOUND");
            break;

        case ERROR_INSUFFICIENT_SPACE:
            DOWNLOAD_HILOGD("error code:	ERROR_INSUFFICIENT_SPACE");
            break;

        case ERROR_FILE_ALREADY_EXISTS:
            DOWNLOAD_HILOGD("error code:	ERROR_FILE_ALREADY_EXISTS");
            break;

        case ERROR_FILE_ERROR:
            DOWNLOAD_HILOGD("error code:	ERROR_FILE_ERROR");
            break;

        case ERROR_HTTP_DATA_ERROR:
            DOWNLOAD_HILOGD("error code:	ERROR_HTTP_DATA_ERROR");
            break;

        case ERROR_TOO_MANY_REDIRECTS:
            DOWNLOAD_HILOGD("error code:	ERROR_TOO_MANY_REDIRECTS");
            break;

        case ERROR_UNHANDLED_HTTP_CODE:
            DOWNLOAD_HILOGD("error code:	ERROR_UNHANDLED_HTTP_CODE");
            break;

        case ERROR_UNKNOWN:
            DOWNLOAD_HILOGD("error code:	ERROR_UNKNOWN");
            break;

        default:
            DOWNLOAD_HILOGD("error code:	SESSION_UNKNOWN");
            break;
    }
}

void DownloadServiceTask::DumpPausedReason()
{
    switch (reason_) {
        case PAUSED_QUEUED_FOR_WIFI:
            DOWNLOAD_HILOGD("paused reason:	PAUSED_QUEUED_FOR_WIFI");
            break;

        case PAUSED_WAITING_FOR_NETWORK:
            DOWNLOAD_HILOGD("paused reason:	PAUSED_WAITING_FOR_NETWORK");
            break;

        case PAUSED_WAITING_TO_RETRY:
            DOWNLOAD_HILOGD("paused reason:	PAUSED_WAITING_TO_RETRY");
            break;

        case PAUSED_BY_USER:
            DOWNLOAD_HILOGD("paused reason:	PAUSED_BY_USER");
            break;

        case PAUSED_UNKNOWN:
            DOWNLOAD_HILOGD("paused reason:	PAUSED_UNKNOWN");
            break;

        default:
            DOWNLOAD_HILOGD("paused reason:	PAUSED_UNKNOWN");
            break;
    }
}

size_t DownloadServiceTask::WriteCallback(void *buffer, size_t size, size_t num, void *param)
{
    size_t result = 0;
    DownloadServiceTask *this_ = static_cast<DownloadServiceTask *>(param);
    if (this_ != nullptr && this_->config_.GetFD() > 0 && this_->hasFileSize_) {
        result = static_cast<size_t>(write(this_->config_.GetFD(), buffer, size * num));
        if (result < size * num) {
            DOWNLOAD_HILOGE("origin size = %{public}zu, write size = %{public}zu", size * num, result);
        }
        this_->downloadSize_ += static_cast<int64_t>(result);
    }
    return result;
}

int DownloadServiceTask::ProgressCallback(void *pParam, double dltotal, double dlnow, double ultotal, double ulnow)
{
    DownloadServiceTask *task = static_cast<DownloadServiceTask *>(pParam);
    if (task != nullptr && task->hasFileSize_) {
        if (task->isRemoved_) {
            DOWNLOAD_HILOGI("download task has been removed");
            return CURL_REMOVE_TASK;
        }
        if (task->forceStop_) {
            DOWNLOAD_HILOGI("Pause issued by user");
            return HTTP_FORCE_STOP;
        }
        if (task->eventCb_ == nullptr) {
            return 0;
        }
        if (task->prevSize_ != task->downloadSize_) {
            std::lock_guard<std::recursive_mutex> autoLock(task->mutex_);
            if (task->status_ != SESSION_PAUSED) {
                task->eventCb_("progress", task->taskId_, task->downloadSize_, task->totalSize_,  task->isNotifyApp_);
                task->PublishNotification(task->config_.IsBackground(), task->prevSize_,
                                          task->downloadSize_, task->totalSize_);
                task->prevSize_ = task->downloadSize_;
            }
        }
        // calc the download speed
    }
    return 0;
}

bool DownloadServiceTask::ExecHttp()
{
    HitraceScoped traceStart(HITRACE_TAG_MISC, "download file");
    std::unique_ptr<CURL, decltype(&curl_easy_cleanup)> handle(curl_easy_init(), curl_easy_cleanup);
    if (!handle) {
        DOWNLOAD_HILOGE("Failed to create fetch task");
        return false;
    }
    DOWNLOAD_HILOGI("final url: %{public}s", config_.GetUrl().c_str());
    std::vector<std::string> vec;
    std::for_each(
        config_.GetHeader().begin(), config_.GetHeader().end(), [&vec](const std::pair<std::string, std::string> &p) {
            vec.emplace_back(p.first + HTTP_HEADER_SEPARATOR + p.second);
        });
    std::unique_ptr<struct curl_slist, decltype(&curl_slist_free_all)> header(MakeHeaders(vec), curl_slist_free_all);
    if (!SetOption(handle.get(), header.get())) {
        DOWNLOAD_HILOGE("set option failed");
        return false;
    }
    if (config_.GetFD() > 0) {
        DOWNLOAD_HILOGD("Succeed to open download file");
        off_t pos = lseek64(config_.GetFD(), 0, SEEK_END);
        downloadSize_ = 0;
        if (pos > 0) {
            if (totalSize_ == INVALID_LEN || pos < static_cast<off_t>(totalSize_)) {
                SetResumeFromLarge(handle.get(), pos);
            } else {
                downloadSize_ = totalSize_;
                DOWNLOAD_HILOGI("Download task has already completed");
                SetStatus(SESSION_SUCCESS);
                PublishNotification(config_.IsBackground(), HUNDRED_PERCENT);
                return true;
            }
        }
        prevSize_ = downloadSize_;
    } else {
        DOWNLOAD_HILOGD("Failed to open download file");
    }
    PublishNotification(config_.IsBackground(), prevSize_, downloadSize_, totalSize_);
    CURLcode code = CurlPerformFileTransfer(handle.get());
    int32_t httpCode;
    curl_easy_getinfo(handle.get(), CURLINFO_RESPONSE_CODE, &httpCode);
    HandleResponseCode(code, httpCode);
    HandleCleanup(status_);
    RecordTaskEvent(httpCode);
    return code == CURLE_OK;
}

void DownloadServiceTask::RecordTaskEvent(int32_t httpCode)
{
    DOWNLOAD_HILOGI("in RecordTaskEvent");
    if (status_ == SESSION_SUCCESS) {
        TaskStatistics::GetInstance().ReportTasksSize(totalSize_);
        TaskStatistics::GetInstance().ReportTasksNumber();
    } else {
        TaskFault::GetInstance().ReportTaskFault(httpCode);
    }
}
   
CURLcode DownloadServiceTask::CurlPerformFileTransfer(CURL *handle) const
{
    std::string traceParam = "name:" + config_.GetFilePath() + " size:" + std::to_string(totalSize_) +
                             " downloaded size:" + std::to_string(prevSize_);
    HitraceScoped trace(HITRACE_TAG_MISC, "download file" + traceParam);
    return curl_easy_perform(handle);
}

bool DownloadServiceTask::SetFileSizeOption(CURL *curl, struct curl_slist *requestHeader)
{
    curl_easy_setopt(curl, CURLOPT_URL, config_.GetUrl().c_str());
    curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, HTTP_GET_METHOD);

    if (requestHeader != nullptr) {
        curl_easy_setopt(curl, CURLOPT_HTTPHEADER, requestHeader);
    }
    // Some servers don't like requests that are made without a user-agent field, so we provide one
    curl_easy_setopt(curl, CURLOPT_USERAGENT, HTTP_DEFAULT_USER_AGENT);
#if 1
    curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 1L);

    /* first #undef CURL_DISABLE_COOKIES in curl config */
    curl_easy_setopt(curl, CURLOPT_COOKIEFILE, "");

#ifdef DOWNLOAD_USE_PROXY
    curl_easy_setopt(curl, CURLOPT_PROXY, HTTP_PROXY_URL_PORT);
    curl_easy_setopt(curl, CURLOPT_PROXYTYPE, HTTP_PROXY_TYPE);
    curl_easy_setopt(curl, CURLOPT_HTTPPROXYTUNNEL, 1L);
#ifdef DOWNLOAD_PROXY_PASS
    curl_easy_setopt(curl, CURLOPT_PROXYUSERPWD, HTTP_PROXY_PASS);
#endif // DOWNLOAD_PROXY_PASS
#endif // DOWNLOAD_USE_PROXY
    if (!SetCertificationOption(curl)) {
        return false;
    }

    curl_easy_setopt(curl, CURLOPT_NOSIGNAL, 1L);
#if HTTP_CURL_PRINT_VERBOSE
    curl_easy_setopt(curl, CURLOPT_VERBOSE, 1L, context);
#endif
    curl_easy_setopt(curl, CURLOPT_LOW_SPEED_TIME, DEFAULT_READ_TIMEOUT);
    curl_easy_setopt(curl, CURLOPT_LOW_SPEED_LIMIT, 1L);
    curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, DEFAULT_CONNECT_TIMEOUT);
#endif
    return true;
}

bool DownloadServiceTask::SetOption(CURL *curl, struct curl_slist *requestHeader)
{
    curl_easy_setopt(curl, CURLOPT_URL, config_.GetUrl().c_str());
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, WriteCallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, this);

    curl_easy_setopt(curl, CURLOPT_NOPROGRESS, 0);
    curl_easy_setopt(curl, CURLOPT_PROGRESSFUNCTION, ProgressCallback);
    curl_easy_setopt(curl, CURLOPT_PROGRESSDATA, this);

    if (requestHeader != nullptr) {
        curl_easy_setopt(curl, CURLOPT_HTTPHEADER, requestHeader);
    }
    // Some servers don't like requests that are made without a user-agent field, so we provide one
    curl_easy_setopt(curl, CURLOPT_USERAGENT, HTTP_DEFAULT_USER_AGENT);
#if 1
    curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 1L);

    /* first #undef CURL_DISABLE_COOKIES in curl config */
    curl_easy_setopt(curl, CURLOPT_COOKIEFILE, "");

#ifdef DOWNLOAD_USE_PROXY
    curl_easy_setopt(curl, CURLOPT_PROXY, HTTP_PROXY_URL_PORT);
    curl_easy_setopt(curl, CURLOPT_PROXYTYPE, HTTP_PROXY_TYPE);
    curl_easy_setopt(curl, CURLOPT_HTTPPROXYTUNNEL, 1L);
#ifdef DOWNLOAD_PROXY_PASS
    curl_easy_setopt(curl, CURLOPT_PROXYUSERPWD, HTTP_PROXY_PASS);
#endif // DOWNLOAD_PROXY_PASS
#endif // DOWNLOAD_USE_PROXY
    if (!SetCertificationOption(curl)) {
        return false;
    }

    curl_easy_setopt(curl, CURLOPT_NOSIGNAL, 1L);
#if HTTP_CURL_PRINT_VERBOSE
    curl_easy_setopt(curl, CURLOPT_VERBOSE, 1L, context);
#endif
    curl_easy_setopt(curl, CURLOPT_LOW_SPEED_TIME, DEFAULT_READ_TIMEOUT);
    curl_easy_setopt(curl, CURLOPT_LOW_SPEED_LIMIT, 1L);
    curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, DEFAULT_CONNECT_TIMEOUT);
#endif
    return true;
}

struct curl_slist *DownloadServiceTask::MakeHeaders(const std::vector<std::string> &vec)
{
    struct curl_slist *header = nullptr;
    std::for_each(vec.begin(), vec.end(), [&header](const std::string &s) {
        if (!s.empty()) {
            header = curl_slist_append(header, s.c_str());
        }
    });
    return header;
}

void DownloadServiceTask::SetResumeFromLarge(CURL *curl, long long pos)
{
    isPartialMode_ = true;
    downloadSize_ = static_cast<int64_t>(pos);
    curl_easy_setopt(curl, CURLOPT_RESUME_FROM_LARGE, pos);
}

bool DownloadServiceTask::GetFileSize(int64_t &result)
{
    if (hasFileSize_) {
        DOWNLOAD_HILOGI("Already get file size");
        return true;
    }
    double size = -1;
    std::unique_ptr<CURL, decltype(&curl_easy_cleanup)> handle(curl_easy_init(), curl_easy_cleanup);
    if (!handle) {
        DOWNLOAD_HILOGE("Failed to create download service task");
        return false;
    }

    std::vector<std::string> vec;
    std::for_each(
        config_.GetHeader().begin(), config_.GetHeader().end(), [&vec](const std::pair<std::string, std::string> &p) {
            vec.emplace_back(p.first + HTTP_HEADER_SEPARATOR + p.second);
        });
    std::unique_ptr<struct curl_slist, decltype(&curl_slist_free_all)> header(MakeHeaders(vec), curl_slist_free_all);

    if (!SetFileSizeOption(handle.get(), header.get())) {
        DOWNLOAD_HILOGE("set option failed");
        return false;
    }

    curl_easy_setopt(handle.get(), CURLOPT_NOBODY, 1L);
    CURLcode code = curl_easy_perform(handle.get());
    long respCode = 0;
    curl_easy_getinfo(handle.get(), CURLINFO_RESPONSE_CODE, &respCode);
    if ((code == CURLE_OK) && (respCode == HTTP_OK)) {
        curl_easy_getinfo(handle.get(), CURLINFO_CONTENT_LENGTH_DOWNLOAD, &size);
        result = static_cast<int64_t>(size);
        char *ct = nullptr;
        curl_easy_getinfo(handle.get(), CURLINFO_CONTENT_TYPE, &ct);
        mimeType_ = ct;
        hasFileSize_ = true;
    }
    DOWNLOAD_HILOGI("file size: %{public}" PRId64 "curl code: %{public}d, http resp: %{public}ld",
        result, code, respCode);
    return hasFileSize_;
}

std::string DownloadServiceTask::GetTmpPath()
{
    return config_.GetFilePath() + "_" + std::to_string(taskId_);
}

void DownloadServiceTask::HandleResponseCode(CURLcode code, int32_t httpCode)
{
    if (isRemoved_) {
        DOWNLOAD_HILOGI("download task has been removed");
        return;
    }
    DOWNLOAD_HILOGI("Current CURLcode is %{public}d, httpCode is %{public}d", code, httpCode);
    if (status_ == SESSION_PAUSED && reason_ == PAUSED_BY_USER) {
        DOWNLOAD_HILOGI("Pause By User:ignore status changed caused by libcurl");
        return;
    }
    
    switch (code) {
        case CURLE_OK:
            if (httpCode == HTTP_OK || (isPartialMode_ && httpCode == HTTP_PARIAL_FILE)) {
                SetStatus(SESSION_SUCCESS);
                PublishNotification(config_.IsBackground(), HUNDRED_PERCENT);
                return;
            }
            break;
            
        case CURLE_ABORTED_BY_CALLBACK:
            if (httpCode == HTTP_OK || (isPartialMode_ && httpCode == HTTP_PARIAL_FILE)) {
                SetStatus(SESSION_PAUSED, ERROR_UNKNOWN, PAUSED_BY_USER);
                return;
            }
            break;

        case CURLE_WRITE_ERROR:
            if (httpCode == HTTP_OK || (isPartialMode_ && httpCode == HTTP_PARIAL_FILE)) {
                SetStatus(SESSION_FAILED, ERROR_HTTP_DATA_ERROR, PAUSED_UNKNOWN);
                return;
            }
            break;
            
        case CURLE_TOO_MANY_REDIRECTS:
            SetStatus(SESSION_FAILED, ERROR_TOO_MANY_REDIRECTS, PAUSED_UNKNOWN);
            return;

        case CURLE_COULDNT_RESOLVE_PROXY:
        case CURLE_COULDNT_RESOLVE_HOST:
        case CURLE_COULDNT_CONNECT:
        case CURLE_OPERATION_TIMEDOUT:
            SetStatus(SESSION_PENDING);
            return;

        default:
            break;
    }
    SetStatus(SESSION_FAILED, ERROR_UNHANDLED_HTTP_CODE, PAUSED_UNKNOWN);
}

bool DownloadServiceTask::CheckResumeCondition()
{
    if (!isOnline_) {
        return false;
    }
    return true;
}

void DownloadServiceTask::ForceStopRunning()
{
    forceStop_ = true;
}

void DownloadServiceTask::HandleCleanup(DownloadStatus status)
{
    switch (status) {
        case SESSION_SUCCESS:
            // rename download to target file name
            if (config_.GetFD() > 0) {
                close(config_.GetFD());
                config_.SetFD(-1);
            }
            break;

        case SESSION_FAILED:
            break;

        default:
            break;
    }
}

bool DownloadServiceTask::HandleFileError()
{
    ErrorCode code = ERROR_UNKNOWN;
    if (config_.GetFD() < 0) {
        switch (config_.GetFDError()) {
            case 0:
                DOWNLOAD_HILOGD("Download File already exists");
                code = ERROR_FILE_ALREADY_EXISTS;
                break;
                
            case ENODEV:
                code = ERROR_DEVICE_NOT_FOUND;
                break;

            default:
                code = ERROR_FILE_ERROR;
                break;
        }
        SetStatus(SESSION_FAILED, code, PAUSED_UNKNOWN);
        return true;
    }
    return false;
}

bool DownloadServiceTask::IsSatisfiedConfiguration()
{
    // Compatible does not support downloading network task configuration version
    if (config_.GetNetworkType() == NETWORK_INVALID) {
        return true;
    }
    auto networkInfo = NetworkAdapter::GetInstance().GetNetworkInfo();
    DOWNLOAD_HILOGD("isRoaming_: %{public}d, isMetered_: %{public}d, networkType_: %{public}u",
                    networkInfo.isRoaming_, networkInfo.isMetered_, networkInfo.networkType_);
    DOWNLOAD_HILOGD("config_ { isRoaming_: %{public}d,isMetered_: %{public}d, networkType_: %{public}u}",
                    config_.IsRoaming(), config_.IsMetered(), config_.GetNetworkType());
    if (networkInfo.isRoaming_ && !config_.IsRoaming()) {
        return false;
    }
    if (networkInfo.isMetered_ && !config_.IsMetered()) {
        return false;
    }
    if ((networkInfo.networkType_ & config_.GetNetworkType()) == NETWORK_INVALID) {
        return false;
    }
    return true;
}

bool DownloadServiceTask::SetCertificationOption(CURL *curl)
{
    return (IsHttpsURL() ? SetHttpsCertificationOption(curl) : SetHttpCertificationOption(curl));
}

bool DownloadServiceTask::IsHttpsURL()
{
    return config_.GetUrl().find(URL_HTTPS) == 0;
}

bool DownloadServiceTask::SetHttpCertificationOption(CURL *curl)
{
    curl_easy_setopt(curl, CURLOPT_SSL_VERIFYHOST, 0L);
    curl_easy_setopt(curl, CURLOPT_SSL_VERIFYPEER, 0L);
    return true;
}

bool DownloadServiceTask::SetHttpsCertificationOption(CURL *curl)
{
    std::string certInfo = ReadCertification();
    if (certInfo.empty()) {
        DOWNLOAD_HILOGE("Read certinfo failed");
        return false;
    }
    struct curl_blob blob;
    blob.data = const_cast<char*>(certInfo.c_str());
    blob.len = certInfo.size();
    blob.flags = CURL_BLOB_COPY;
    std::string version = TLS_VERSION_DEFAULT;
    std::map<std::string, std::string>::const_iterator iter = config_.GetHeader().find(tlsVersion);
    if (iter != config_.GetHeader().end()) {
        version = iter->second;
        DOWNLOAD_HILOGI("version changes: %{public}s", version.c_str());
    }
    curl_easy_setopt(curl, CURLOPT_SSLVERSION, version.c_str());
    DOWNLOAD_HILOGI("security version is: %{public}s", version.c_str());
    CURLcode code = curl_easy_setopt(curl, CURLOPT_CAINFO_BLOB, &blob);
    if (code != CURLE_OK) {
        return false;
    }
    DOWNLOAD_HILOGI("SetHttpsCertificationOption success");
    return true;
}

std::string DownloadServiceTask::ReadCertification()
{
    std::ifstream inFile(std::string(HTTP_DEFAULT_CA_PATH), std::ios::in | std::ios::binary);
    if (!inFile.is_open()) {
        DOWNLOAD_HILOGE("open cacert.pem failed");
        return "";
    }
    std::stringstream buf;
    buf << inFile.rdbuf();
    std::string certInfo(buf.str());
    inFile.close();
    return certInfo;
}

void DownloadServiceTask::PublishNotification(bool background, uint32_t percent)
{
    if (!background) {
        return;
    }
    pid_t pid = static_cast<pid_t>(config_.GetApplicationInfoUid());
    std::string filePath = config_.GetFilePath();
    DownloadBackgroundNotification::PublishDownloadNotification(taskId_, pid, filePath, percent);
}

void DownloadServiceTask::PublishNotification(bool background, int64_t prevSize, int64_t downloadSize,
    int64_t totalSize)
{
    if (!background || totalSize == -1) {
        return;
    }
    if (prevSize == 0) {
        PublishNotification(background, 0);
        lastTimestamp_ =  GetCurTimestamp();
    } else {
        uint32_t percent = ProgressNotification(prevSize, downloadSize, totalSize);
        if (percent > 0) {
            PublishNotification(background, percent);
        }
    }
}

std::time_t DownloadServiceTask::GetCurTimestamp()
{
    auto tp = std::chrono::time_point_cast<std::chrono::milliseconds>(std::chrono::system_clock::now());
    return tp.time_since_epoch().count();
}

uint32_t DownloadServiceTask::ProgressNotification(int64_t prevSize, int64_t downloadSize, int64_t totalSize)
{
    uint32_t ret = 0;
    if (totalSize != 0) {
        uint32_t percent = static_cast<uint32_t>(downloadSize * 100.0 / totalSize);
        uint32_t lastPercent = static_cast<uint32_t>(prevSize  * 100.0 / totalSize);
        std::time_t curTimestamp = GetCurTimestamp();
        if (curTimestamp < lastTimestamp_) {
            return 0;
        }
        if ((percent - lastPercent) >= TEN_PERCENT_THRESHOLD ||
            (curTimestamp - lastTimestamp_) >= NOTIFICATION_FREQUENCY) {
            ret = percent;
            lastTimestamp_ = curTimestamp;
        }
    }
    return ret;
}

void DownloadServiceTask::SetNotifyApp(bool isNotifyApp)
{
    isNotifyApp_ = isNotifyApp;
}

std::string DownloadServiceTask::GetTaskBundleName() const
{
    return config_.GetBundleName();
}

int32_t DownloadServiceTask::GetTaskApplicationInfoUid() const
{
    return config_.GetApplicationInfoUid();
}

} // namespace OHOS::Request::Download
