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
#include "constant.h"
#include "log.h"

namespace OHOS::Request::Download {
DownloadServiceTask::DownloadServiceTask(uint32_t taskId, const DownloadConfig &config)
    : taskId_(taskId), config_(config), status_(SESSION_UNKNOWN), code_(ERROR_UNKNOWN), reason_(PAUSED_UNKNOWN),
      mimeType_(""), file_(nullptr), totalSize_(0), downloadSize_(0), isPartialMode_(false), forceStop_(false),
      isRemoved_(false), retryTime_(10), eventCb_(nullptr) {
}

DownloadServiceTask::~DownloadServiceTask(void)
{
    DOWNLOAD_HILOGD("Destructed download service task [%{public}d]", taskId_);
    if (file_ != nullptr) {
        fflush(file_);
        fclose(file_);
    }
}

uint32_t DownloadServiceTask::GetId() const
{
    return taskId_;
}

bool DownloadServiceTask::Run()
{
    DOWNLOAD_HILOGD("Task[%{public}d] start.", taskId_);
    std::lock_guard<std::recursive_mutex> autoLock(mutex_);
    if (!GetFileSize(totalSize_)) {
        SetStatus(SESSION_PENDING, ERROR_UNKNOWN, reason_);
        return false;
    }
    uint32_t retryTime = 0;
    bool result = true;
    bool enableTimeout;
    do {
        enableTimeout = false;
        SetStatus(SESSION_RUNNING);
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
    return result;
}

bool DownloadServiceTask::Pause()
{
    DOWNLOAD_HILOGD("Pause Task[%{public}d], current status is %{public}d\n", taskId_, status_);
    if (status_ != SESSION_RUNNING && status_ != SESSION_PENDING) {
        return false;
    }
    ForceStopRunning();

    SetStatus(SESSION_PAUSED);
    return true;
}

bool DownloadServiceTask::Resume()
{
    DOWNLOAD_HILOGD("Resume Task[%{public}d], current status is %{public}d\n", taskId_, status_);
    if (status_ != SESSION_PAUSED) {
        return false;
    }
    ForceStopRunning();
    if (!CheckResumeCondition()) {
        SetStatus(SESSION_FAILED, ERROR_CANNOT_RESUME, reason_);
        return true;
    }
    SetStatus(SESSION_UNKNOWN, code_, reason_);
    return true;
}
bool DownloadServiceTask::Remove()
{
    DOWNLOAD_HILOGD("Remove Task[%{public}d], current status is %{public}d\n", taskId_, status_);
    isRemoved_ = true;
    ForceStopRunning();
    if (eventCb_ != nullptr) {
        eventCb_("remove", taskId_, 0, 0);
    }
    return true;
}

bool DownloadServiceTask::Query(DownloadInfo &info)
{
    DOWNLOAD_HILOGD("Query Task[%{public}d], current status is %{public}d\n", taskId_, status_);
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
    return true;
}

bool DownloadServiceTask::QueryMimeType(std::string &mimeType)
{
    DOWNLOAD_HILOGD("Query Mime Type of Task[%{public}d], current status is %{public}d\n", taskId_, status_);
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

void DownloadServiceTask::SetStatus(DownloadStatus status, ErrorCode code, PausedReason reason)
{
    auto stateChange = [this](DownloadStatus status, ErrorCode code, PausedReason reason) -> bool {
        std::lock_guard<std::recursive_mutex> autoLock(mutex_);
        this->forceStop_ = false;
        bool isChanged = false;
        if (status != this->status_) {
            this->status_ = status;
            isChanged = true;
        }
        if (code != this->code_) {
            this->code_ = code;
            isChanged = true;
        }
        if (reason != this->reason_) {
            this->reason_ = reason;
            isChanged = true;
        }
        return true;
    };
    if (!stateChange(status, code, reason)) {
        return;
    }
    if (eventCb_ != nullptr) {
        switch (status_) {
            case SESSION_SUCCESS:
                eventCb_("complete", taskId_, 0, 0);
                break;

            case SESSION_PAUSED:
                eventCb_("pause", taskId_, 0, 0);
                break;

            case SESSION_FAILED:
                eventCb_("fail", taskId_, code_, 0);
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
        this->forceStop_ = false;
        if (status == this->status_) {
            DOWNLOAD_HILOGD("ignore same status");
            return false;
        }
        this->status_ = status;
        return true;
    };
    if (!stateChange(status)) {
        return;
    }
    if (eventCb_ != nullptr) {
        switch (status_) {
            case SESSION_SUCCESS:
                eventCb_("complete", taskId_, 0, 0);
                break;

            case SESSION_PAUSED:
                eventCb_("pause", taskId_, 0, 0);
                break;

            case SESSION_FAILED:
                eventCb_("fail", taskId_, code_, 0);
                break;

            default:
                break;
        }
    }
}

void DownloadServiceTask::SetError(ErrorCode code)
{
    std::lock_guard<std::recursive_mutex> autoLock(mutex_);
    this->forceStop_ = false;
    if (code == code_) {
        DOWNLOAD_HILOGD("ignore same error code");
        return;
    }
    code_ = code;
}

void DownloadServiceTask::SetReason(PausedReason reason)
{
    std::lock_guard<std::recursive_mutex> autoLock(mutex_);
    this->forceStop_ = false;
    if (reason == reason_) {
        DOWNLOAD_HILOGD("ignore same paused reason");
        return;
    }
    reason_ = reason;
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
    if (this_ != nullptr && this_->file_) {
        result = fwrite(buffer, size, num, this_->file_);
        this_->downloadSize_ += static_cast<uint32_t>(result);
    }
    return result;
}

size_t DownloadServiceTask::HeaderCallback(void *buffer, size_t size, size_t num, void *param)
{
    DownloadServiceTask *this_ = static_cast<DownloadServiceTask *>(param);
    std::string recvHeader = static_cast<char *>(buffer);
    if (this_ != nullptr && recvHeader.find(HTTP_CONTENT_TYPE) != std::string::npos) {
        std::string mimeType = recvHeader.substr(recvHeader.find(HTTP_HEADER_SEPARATOR) + 2);
        mimeType = mimeType.substr(0, mimeType.find(HTTP_LINE_SEPARATOR));
        this_->mimeType_ = mimeType;
    }
    return size * num;
}

int DownloadServiceTask::ProgressCallback(void *pParam, double dltotal, double dlnow, double ultotal, double ulnow)
{
    DownloadServiceTask *this_ = static_cast<DownloadServiceTask *>(pParam);
    if (this_ != nullptr) {
        if (this_->isRemoved_) {
            DOWNLOAD_HILOGD("download task has been removed\n");
        }
        if (this_->eventCb_ != nullptr && !this_->isRemoved_) {
            this_->eventCb_("progress",  this_->taskId_, this_->downloadSize_, this_->totalSize_);
        }
        if (this_->forceStop_) {
            DOWNLOAD_HILOGD("Pause issued by user\n");
            return HTTP_FORCE_STOP;
        }
        // calc the download speed
    }
    return 0;
}

bool DownloadServiceTask::ExecHttp()
{
    std::unique_ptr<CURL, decltype(&curl_easy_cleanup)> handle(curl_easy_init(), curl_easy_cleanup);

    if (!handle) {
        DOWNLOAD_HILOGD("Failed to create fetch task");
        return false;
    }

    DOWNLOAD_HILOGD("final url: %{public}s\n", config_.GetUrl().c_str());

    std::vector<std::string> vec;
    std::for_each(
        config_.GetHeader().begin(), config_.GetHeader().end(), [&vec](const std::pair<std::string, std::string> &p) {
            vec.emplace_back(p.first + HTTP_HEADER_SEPARATOR + p.second);
        });
    std::unique_ptr<struct curl_slist, decltype(&curl_slist_free_all)> header(MakeHeaders(vec), curl_slist_free_all);

    if (!SetOption(handle.get(), header.get())) {
        DOWNLOAD_HILOGD("set option failed");
        return false;
    }
    std::string tmpFileName = GetTmpPath().c_str();
    file_ = fopen(tmpFileName.c_str(), "ab");
    if (file_ != nullptr) {
        DOWNLOAD_HILOGD("Succeed to open %{public}s", tmpFileName.c_str());
        fseek(file_, 0, SEEK_END);
        uint32_t pos = ftell(file_);
        if (pos > 0) {
            if (pos < totalSize_) {
                isPartialMode_ = true;
                SetResumeFromLarge(handle.get(), pos);
            } else if (pos == totalSize_) {
                downloadSize_ = totalSize_;
                DOWNLOAD_HILOGD("Download task has already completed");
                SetStatus(SESSION_SUCCESS);
                return true;
            } else {
                DOWNLOAD_HILOGD("Download size exceed the file size, re-download it");
                fclose(file_);
                file_ = fopen(tmpFileName.c_str(), "wb");
            }
        }
    } else {
        DOWNLOAD_HILOGD("Failed to open %{public}s", tmpFileName.c_str());
    }

    CURLcode code = curl_easy_perform(handle.get());

    if (file_ != nullptr) {
        fflush(file_);
        fclose(file_);
    }
    int32_t httpCode;
    curl_easy_getinfo(handle.get(), CURLINFO_RESPONSE_CODE, &httpCode);
    HandleResponseCode(code, httpCode);
    HandleCleanup(status_);
    return code == CURLE_OK;
}

bool DownloadServiceTask::SetOption(CURL *curl, struct curl_slist *requestHeader)
{
    curl_easy_setopt(curl, CURLOPT_URL, config_.GetUrl().c_str());
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, WriteCallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, this);

    curl_easy_setopt(curl, CURLOPT_NOPROGRESS, 0);
    curl_easy_setopt(curl, CURLOPT_PROGRESSFUNCTION, ProgressCallback);
    curl_easy_setopt(curl, CURLOPT_PROGRESSDATA, this);

    curl_easy_setopt(curl, CURLOPT_HEADERFUNCTION, HeaderCallback);
    curl_easy_setopt(curl, CURLOPT_HEADERDATA, this);

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

#ifdef DOWNLOAD_SSL_CERTIFICATION
    curl_easy_setopt(curl, CURLOPT_CAINFO, HTTP_DEFAULT_CA_PATH);
#else
    // NO_SSL_CERTIFICATION
    curl_easy_setopt(curl, CURLOPT_SSL_VERIFYHOST, 0L);
    curl_easy_setopt(curl, CURLOPT_SSL_VERIFYPEER, 0L);
#endif

    curl_easy_setopt(curl, CURLOPT_NOSIGNAL, 1L);
#if HTTP_CURL_PRINT_VERBOSE
    curl_easy_setopt(curl, CURLOPT_VERBOSE, 1L, context);
#endif
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, DEFAULT_READ_TIMEOUT);
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
    curl_easy_setopt(curl, CURLOPT_RESUME_FROM_LARGE, pos);
}

bool DownloadServiceTask::GetFileSize(uint32_t &result)
{
    if (totalSize_) {
        DOWNLOAD_HILOGD("Already get file size");
        return true;
    }
    double size = 0.0;
    std::unique_ptr<CURL, decltype(&curl_easy_cleanup)> handle(curl_easy_init(), curl_easy_cleanup);

    if (!handle) {
        DOWNLOAD_HILOGD("Failed to create download service task");
        return false;
    }

    std::vector<std::string> vec;
    std::for_each(
        config_.GetHeader().begin(), config_.GetHeader().end(), [&vec](const std::pair<std::string, std::string> &p) {
            vec.emplace_back(p.first + HTTP_HEADER_SEPARATOR + p.second);
        });
    std::unique_ptr<struct curl_slist, decltype(&curl_slist_free_all)> header(MakeHeaders(vec), curl_slist_free_all);

    if (!SetOption(handle.get(), header.get())) {
        DOWNLOAD_HILOGD("set option failed");
        return false;
    }

    curl_easy_setopt(handle.get(), CURLOPT_NOBODY, 1L);
    CURLcode res;
    curl_easy_perform(handle.get());
    res = curl_easy_getinfo(handle.get(), CURLINFO_CONTENT_LENGTH_DOWNLOAD, &size);
    if (res == CURLE_OK) {
        result = static_cast<long long>(size);
    }
    if (result == -1) {
        result = 0;
    }
    DOWNLOAD_HILOGD("fetch file size %{public}d", result);
    return true;
}

std::string DownloadServiceTask::GetTmpPath()
{
    return config_.GetFilePath() + "_" + std::to_string(taskId_);
}

void DownloadServiceTask::HandleResponseCode(CURLcode code, int32_t httpCode)
{
    if (isRemoved_) {
        DOWNLOAD_HILOGD("download task has been removed");
        return;
    }
    switch (code) {
        case CURLE_OK:
            if (httpCode == HTTP_OK || (isPartialMode_ && httpCode == HTTP_PARIAL_FILE)) {
                SetStatus(SESSION_SUCCESS);
                return;
            }

        case CURLE_ABORTED_BY_CALLBACK:
            if (httpCode == HTTP_OK || (isPartialMode_ && httpCode == HTTP_PARIAL_FILE)) {
                SetStatus(SESSION_PAUSED, code_, PAUSED_BY_USER);
                return;
            }
        case CURLE_TOO_MANY_REDIRECTS:
            SetStatus(SESSION_FAILED, ERROR_TOO_MANY_REDIRECTS, reason_);
            return;

        case CURLE_OPERATION_TIMEDOUT:
            SetStatus(SESSION_PENDING);
            return;

        default:
            break;
    }
    DOWNLOAD_HILOGD("Current CURLcode is %{public}d, httpCode is %{public}d\n", code, httpCode);
    SetStatus(SESSION_FAILED, ERROR_UNHANDLED_HTTP_CODE, reason_);
}

bool DownloadServiceTask::CheckResumeCondition()
{
    // current paused issued by user
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
            rename(GetTmpPath().c_str(), config_.GetFilePath().c_str());
            break;

        case SESSION_FAILED:
            break;

        default:
            break;
    }
}
} // namespace OHOS::Request::Download
