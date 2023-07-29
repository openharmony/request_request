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

#include <unistd.h>
#include <fcntl.h>
#include <cstdio>
#include <vector>
#include <string>
#include <climits>
#include <cinttypes>
#include "common_timer_errors.h"
#include "upload_task.h"
#include "upload_hilog_wrapper.h"
#include "hitrace_meter.h"
#include "hisysevent.h"
#include "curl_adp.h"

namespace OHOS::Request::Upload {
CUrlAdp::CUrlAdp(std::vector<FileData> &fileDatas, std::shared_ptr<UploadConfig> &config)
    : fileDatas_(fileDatas), timer_("uploadTimer")
{
    config_ = config;
    isCurlGlobalInit_ = false;
    isReadAbort_ = false;
    curlMulti_ = nullptr;
    timerId_ = 0;
}

CUrlAdp::~CUrlAdp()
{
    UPLOAD_HILOGI(UPLOAD_MODULE_FRAMEWORK, "~CUrlAdp()");
}

uint32_t CUrlAdp::DoUpload(std::shared_ptr<IUploadTask> task)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "upload start");
    if (task != nullptr) {
        uploadTask_ = task;
    }

    uint32_t successCount = 0;
    for (auto &vmem : fileDatas_) {
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "read abort stat: %{public}d file index: %{public}u",
                      IsReadAbort(), vmem.fileIndex);
        if (IsReadAbort()) {
            vmem.result = UPLOAD_TASK_REMOVED;
            continue;
        }

        mfileData_ = vmem;
        mfileData_.adp = shared_from_this();
        vmem.result = static_cast<uint32_t>(UploadOneFile());
        if (vmem.result == UPLOAD_OK) {
            successCount++;
        }
        mfileData_.responseHead.clear();
        if (mfileData_.list) {
            curl_slist_free_all(mfileData_.list);
            mfileData_.list = nullptr;
        }
        ClearCurlResource();
        usleep(FILE_UPLOAD_INTERVEL);
    }
    mfileData_.adp = nullptr;
    uploadTask_ = nullptr;
    return (IsSuccess(successCount, fileDatas_.size())) ? UPLOAD_OK : UPLOAD_ERRORCODE_UPLOAD_FAIL;
}

bool CUrlAdp::IsSuccess(const uint32_t count, const uint32_t size)
{
    if (count == 0) {
        return false;
    }
    return (count == size);
}

bool CUrlAdp::MultiAddHandle(CURLM *curlMulti, std::vector<CURL *> &curlArray)
{
    CURL *curl = curl_easy_init();
    if (curl == nullptr) {
        return false;
    }

    SetCurlOpt(curl);
    curlArray.push_back(curl);
    curl_multi_add_handle(curlMulti, curl);
    return true;
}

void CUrlAdp::SetHeadData(CURL *curl)
{
    bool hasContentType = false;
    for (auto &headerData : config_->header) {
        if (headerData.find("Content-Type:") != std::string::npos) {
            hasContentType = true;
        }
        mfileData_.list = curl_slist_append(mfileData_.list, headerData.c_str());
    }

    if (!hasContentType) {
        std::string str = config_->method == PUT ? "Content-Type:application/octet-stream" :
            "Content-Type:multipart/form-data";
        mfileData_.list = curl_slist_append(mfileData_.list, str.c_str());
    }
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, mfileData_.list);
}

void CUrlAdp::SetBehaviorOpt(CURL *curl)
{
    curl_easy_setopt(curl, CURLOPT_VERBOSE, 1L);
    curl_easy_setopt(curl, CURLOPT_NOPROGRESS, 0L);
    curl_easy_setopt(curl, CURLOPT_NOSIGNAL, 1L);
}

void CUrlAdp::SetCallbackOpt(CURL *curl)
{
    curl_easy_setopt(curl, CURLOPT_HEADERDATA, &mfileData_);
    curl_easy_setopt(curl, CURLOPT_HEADERFUNCTION, HeaderCallback);
    curl_easy_setopt(curl, CURLOPT_XFERINFOFUNCTION, ProgressCallback);
    curl_easy_setopt(curl, CURLOPT_XFERINFODATA, &mfileData_);
}

void CUrlAdp::SetNetworkOpt(CURL *curl)
{
    curl_easy_setopt(curl, CURLOPT_URL, config_->url.c_str());
}

void CUrlAdp::SetConnectionOpt(CURL *curl)
{
    curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, 30L);
}

void CUrlAdp::SetSslOpt(CURL *curl)
{
    curl_easy_setopt(curl, CURLOPT_SSL_VERIFYPEER, 0L);
    curl_easy_setopt(curl, CURLOPT_SSL_VERIFYHOST, 0L);
}

void CUrlAdp::SetCurlOpt(CURL *curl)
{
    SetHeadData(curl);
    SetNetworkOpt(curl);
    SetConnectionOpt(curl);
    SetSslOpt(curl);
    SetBehaviorOpt(curl);
    SetCallbackOpt(curl);
    if (config_->method == PUT) {
        SetHttpPut(curl);
    } else {
        SetMimePost(curl);
    }
}

void CUrlAdp::SetMimePost(CURL *curl)
{
    curl_mimepart *part;
    curl_mime *mime = curl_mime_init(curl);
    if (config_->data.size()) {
        for (auto &vdata : config_->data) {
            part = curl_mime_addpart(mime);
            curl_mime_name(part, vdata.name.c_str());
            curl_mime_data(part, vdata.value.c_str(), vdata.value.size());
        }
    }
    part = curl_mime_addpart(mime);
    if (!mfileData_.name.empty()) {
        curl_mime_name(part, mfileData_.name.c_str());
    } else {
        curl_mime_name(part, "file");
    }
    curl_mime_type(part, mfileData_.type.c_str());
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===> MultiAddHandle mfileData_.type=%{public}s",
        mfileData_.type.c_str());
    curl_mime_filename(part, mfileData_.filename.c_str());
    curl_mime_data_cb(part, mfileData_.totalsize, ReadCallback, NULL, NULL, &mfileData_);
    curl_easy_setopt(curl, CURLOPT_MIMEPOST, mime);
}

void CUrlAdp::SetHttpPut(CURL *curl)
{
    curl_easy_setopt(curl, CURLOPT_UPLOAD, 1);
    curl_easy_setopt(curl, CURLOPT_READFUNCTION, ReadCallback);
    curl_easy_setopt(curl, CURLOPT_READDATA, &mfileData_);
    curl_easy_setopt(curl, CURLOPT_INFILESIZE, mfileData_.totalsize);
}

int32_t CUrlAdp::UploadOneFile()
{
    std::string traceParam = "name:" + mfileData_.filename + "index" + std::to_string(mfileData_.fileIndex) +
                             "size:" + std::to_string(mfileData_.totalsize);
    HitraceScoped trace(HITRACE_TAG_MISC, "upload file " + traceParam);

    CurlGlobalInit();
    curlMulti_ = curl_multi_init();
    if (curlMulti_ == nullptr) {
        CurlGlobalCleanup();
        return UPLOAD_ERRORCODE_UPLOAD_LIB_ERROR;
    }

    bool ret = MultiAddHandle(curlMulti_, curlArray_);
    if (ret == false) {
        return UPLOAD_ERRORCODE_UPLOAD_LIB_ERROR;
    }

    int isRuning = 0;
    curl_multi_perform(curlMulti_, &isRuning);
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "isRuning = %{public}d", isRuning);
    do {
        int numfds = 0;
        int res = curl_multi_wait(curlMulti_, NULL, 0, TRANS_TIMEOUT_MS, &numfds);
        if (res != CURLM_OK) {
            return res;
        }
        curl_multi_perform(curlMulti_, &isRuning);
    } while (isRuning);

    return CheckUploadStatus(curlMulti_);
}

void CUrlAdp::CurlGlobalInit()
{
    std::lock_guard<std::mutex> guard(globalMutex_);
    if (!isCurlGlobalInit_) {
        isCurlGlobalInit_ = true;
    }
}

void CUrlAdp::CurlGlobalCleanup()
{
    std::lock_guard<std::mutex> guard(globalMutex_);
    if (isCurlGlobalInit_) {
        isCurlGlobalInit_ = false;
    }
}

int CUrlAdp::CheckUploadStatus(CURLM *curlMulti)
{
    int msgsLeft = 0;
    int returnCode = UPLOAD_ERRORCODE_UPLOAD_FAIL;
    CURLMsg* msg = NULL;
    if (IsReadAbort()) {
        UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "CheckUploadStatus  IsReadAbort is %{public}d", IsReadAbort());
        return returnCode;
    }
    while ((msg = curl_multi_info_read(curlMulti, &msgsLeft))) {
        if (msg->msg != CURLMSG_DONE) {
            continue;
        }
        CURL *eh = NULL;
        eh = msg->easy_handle;
        returnCode = msg->data.result;
        if (returnCode != CURLE_OK) {
            UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "upload fail curl error %{public}d", returnCode);
            return UPLOAD_ERRORCODE_UPLOAD_LIB_ERROR;
        }

        long respCode = 0;
        curl_easy_getinfo(eh, CURLINFO_RESPONSE_CODE, &respCode);
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "upload http code %{public}ld", respCode);
        if (respCode != HTTP_SUCCESS) {
            returnCode = respCode;
            UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "upload fail http error %{public}d", returnCode);
            return UPLOAD_ERRORCODE_UPLOAD_FAIL;
        }
        returnCode = UPLOAD_OK;
    }
    return returnCode;
}

bool CUrlAdp::Remove()
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "remove");
    std::lock_guard<std::mutex> guard(curlMutex_);
    isReadAbort_ = true;
    return true;
}

bool CUrlAdp::ClearCurlResource()
{
    std::lock_guard<std::mutex> guard(mutex_);
    for (auto url : curlArray_) {
        curl_multi_remove_handle(curlMulti_, url);
        curl_easy_cleanup(url);
    }
    curlArray_.clear();
    if (curlMulti_) {
        curl_multi_cleanup(curlMulti_);
        curlMulti_ = nullptr;
    }
    CurlGlobalCleanup();
    return true;
}

bool CUrlAdp::CheckCUrlAdp(FileData *fData)
{
    if (fData == nullptr || fData->adp == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "CheckCUrlAdp url == nullptr");
        return false;
    }
    std::lock_guard<std::mutex> lock(fData->adp->curlMutex_);
    if (fData->adp->IsReadAbort()) {
        UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "CheckCUrlAdp url->IsReadAbort()");
        return false;
    }
    return true;
}

int CUrlAdp::ProgressCallback(void *clientp, curl_off_t dltotal, curl_off_t dlnow, curl_off_t ultotal, curl_off_t ulnow)
{
    FileData *fData = (FileData *)clientp;
    if (!CheckCUrlAdp(fData)) {
        return UPLOAD_ERRORCODE_UPLOAD_FAIL;
    }

    std::shared_ptr<CUrlAdp> url = fData->adp;
    std::lock_guard<std::mutex> lock(url->curlMutex_);
    if (ulnow > 0) {
        fData->upsize = fData->totalsize - (ultotal - ulnow);
    } else {
        fData->upsize = ulnow;
    }

    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "progress upload total: %{public}" PRIu64 " upload now: %{public}" PRIu64
        " upload size: %{public}" PRIu64 " total size: %{public}" PRIu64 " thread:%{public}lu",
        ultotal, ulnow, fData->upsize, fData->totalsize, pthread_self());

    if (url->uploadTask_) {
        int64_t totalulnow = 0;
        for (auto &vmem : url->fileDatas_) {
            if (fData->filename == vmem.filename) {
                vmem.upsize = fData->upsize;
            }
            totalulnow += vmem.upsize;
        }
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "report progress total upload size: %{public}" PRIu64
            " upload now: %{public}" PRIu64, totalulnow, ultotal);
        url->uploadTask_->OnProgress(totalulnow);
    }
    return 0;
}

size_t CUrlAdp::HeaderCallback(char *buffer, size_t size, size_t nitems, void *userdata)
{
    FileData *fData = (FileData *)userdata;
    if (!CheckCUrlAdp(fData)) {
        return CURLE_WRITE_ERROR;
    }

    std::shared_ptr<CUrlAdp> url = fData->adp;
    std::lock_guard<std::mutex> lock(url->curlMutex_);
    std::string stmp(buffer, size * nitems);
    url->SplitHttpMessage(stmp, fData);

    if (url->uploadTask_ && fData->headSendFlag == COLLECT_END_FLAG) {
        std::string headers = std::accumulate(fData->responseHead.begin(), fData->responseHead.end(), std::string(""));
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "report head len: %{public}zu, content: %{public}s",
            headers.length(), headers.c_str());
        auto func = (url->config_->protocolVersion == API5) ? NotifyAPI5 : Notify;
        func(fData, headers);
        fData->responseHead.clear();
        fData->httpCode = 0;
    }
    return size * nitems;
}

void CUrlAdp::SplitHttpMessage(const std::string &stmp, FileData* &fData)
{
    const std::string headEndFlag = "\r\n";
    if (std::string::npos != stmp.find("HTTP")) {
        fData->headSendFlag = COLLECT_DO_FLAG;
        const int codeLen = 3;
        std::string::size_type position = stmp.find_first_of(" ");
        std::string scode(stmp, position + 1, codeLen);
        fData->httpCode = std::stol(scode);
    } else if (stmp == headEndFlag) {
        fData->headSendFlag = COLLECT_END_FLAG;
    }
    if (fData->headSendFlag == COLLECT_DO_FLAG || fData->headSendFlag == COLLECT_END_FLAG) {
        fData->responseHead.push_back(stmp);
    }
}

void CUrlAdp::Notify(FileData *fData, std::string &headers)
{
    if (fData->httpCode == HTTP_SUCCESS) {
        if (fData->adp->fileDatas_.size() == fData->fileIndex) {
            fData->adp->uploadTask_->OnHeaderReceive(headers);
        }
    } else {
        fData->adp->uploadTask_->OnHeaderReceive(headers);
    }
}

void CUrlAdp::NotifyAPI5(FileData *fData, std::string &headers)
{
    if (fData->httpCode == HTTP_SUCCESS) {
        if (fData->adp->fileDatas_.size() == fData->fileIndex && fData->adp->config_->fsuccess != nullptr) {
            UploadResponse resData;
            resData.headers = headers;
            resData.code = fData->httpCode;
            fData->adp->config_->fsuccess(resData);
        }
    } else {
        if (fData->adp->config_->ffail) {
            fData->adp->config_->ffail(headers, fData->httpCode);
        }
    }
}

size_t CUrlAdp::ReadCallback(char *buffer, size_t size, size_t nitems, void *arg)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "size is %{public}zu, nitems is %{public}zu.", size, nitems);
    FileData *fData = (FileData *)arg;
    if (!CheckCUrlAdp(fData) || ferror(fData->fp)) {
        return CURL_READFUNC_ABORT;
    }

    std::shared_ptr<CUrlAdp> url = fData->adp;
    std::lock_guard<std::mutex> lock(url->curlMutex_);
    url->StartTimer();
    size_t readSize = fread(buffer, size, nitems, fData->fp);
    url->StopTimer();

    return readSize;
}

void CUrlAdp::StartTimer()
{
    uint32_t ret = timer_.Setup();
    if (ret != Utils::TIMER_ERR_OK) {
        UPLOAD_HILOGI(UPLOAD_MODULE_FRAMEWORK, "Create Timer error");
        return;
    }
    auto TimeOutCallback = [this]() {
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "OutTime error");
        this->isReadAbort_ = true;
    };
    timerId_ = timer_.Register(TimeOutCallback, READFILE_TIMEOUT_MS, true);
}

void CUrlAdp::StopTimer()
{
    timer_.Unregister(timerId_);
    timer_.Shutdown();
}
} // namespace OHOS::Request::Upload