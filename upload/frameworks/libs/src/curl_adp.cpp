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
#include <sys/stat.h>
#include <cstdio>
#include <climits>
#include "upload_hilog_wrapper.h"
#include "upload_task.h"
#include "upload_timer_info.h"
#include "time_service_client.h"

namespace OHOS::Request::Upload {
const int TRANS_TIMEOUT_MS = 300 * 1000;
const int READFILE_TIMEOUT_MS = 30 * 1000;
const int TIMEOUTTYPE = 1;
const int SLEEP = 1000;

CUrlAdp::CUrlAdp(std::vector<FileData>& fileArray, std::shared_ptr<UploadConfig>& config)
{
    fileArray_ = fileArray;
    config_ = config;
    isCurlGlobalInit_ = false;
    isReadAbort_ = false;
    curlMulti_ = nullptr;
    timerId_ = 0;
    timerInfo_ = nullptr;
}

CUrlAdp::~CUrlAdp()
{
}

void CUrlAdp::DoUpload(IUploadTask *task)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "upload start");
    uploadTask_ = task;

    if (config_ == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "config_ is null");
        FailNotify(UPLOAD_ERRORCODE_CONFIG_ERROR);
        return;
    }

    if (config_->url.empty()) {
        UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "URL is empty");
        FailNotify(UPLOAD_ERRORCODE_CONFIG_ERROR);
        return;
    }
    UPLOAD_HILOGI(UPLOAD_MODULE_FRAMEWORK, "URL is %{public}s", config_->url.c_str());

    if (fileArray_.empty()) {
        UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "fileArray_ is empty");
        FailNotify(UPLOAD_ERRORCODE_GET_FILE_ERROR);
        return;
    }

    if (curlMulti_) {
        UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "DoUpload was multi called");
        return;
    }

    InitTimerInfo();
    for (auto &vmem : fileArray_) {
        vmem.upsize = 0;
        vmem.totalsize = 0;
        vmem.fileIndex = -1;
    }
    int index = -1;
    for (auto &vmem : fileArray_) {
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>vmem : fileArray_ isReadAbort is %{public}d", IsReadAbort());
        if (IsReadAbort()) {
            return;
        }
        index++;
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>fileArray index %{public}d", index);
        mfileData_ = vmem;
        mfileData_.fileIndex = index;
        UploadFile();
        RemoveInner();
        usleep(SLEEP);
    }

    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "upload end");
}

bool CUrlAdp::MultiAddHandle(CURLM *curlMulti, std::vector<CURL*>& curlArray)
{
    curl_mime *mime;
    curl_mimepart *part;

    struct stat fileInfo;
    if (mfileData_.fp == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "file ptr is null");
        FailNotify(UPLOAD_ERRORCODE_UPLOAD_LIB_ERROR);
        return false;
    }
    /* to get the file size */
    if (fstat(fileno(mfileData_.fp), &fileInfo) != 0) {
        UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "get the file info fail");
        FailNotify(UPLOAD_ERRORCODE_UPLOAD_LIB_ERROR);
        return false;
    }
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "fileInfo.st_size %{public}lld", fileInfo.st_size);
    CURL *curl = curl_easy_init();
    if (curl == nullptr) {
        FailNotify(UPLOAD_ERRORCODE_UPLOAD_LIB_ERROR);
        return false;
    }
    curlArray.push_back(curl);
    mime = curl_mime_init(curl);
    part = curl_mime_addpart(mime);
    curl_mime_name(part, "upload");
    curl_mime_filename(part, mfileData_.name.c_str());
    mfileData_.adp = this;
    mfileData_.totalsize = fileInfo.st_size;
    curl_mime_data_cb(part, fileInfo.st_size, ReadCallback, NULL, NULL, &mfileData_);
    curl_easy_setopt(curl, CURLOPT_MIMEPOST, mime);
    SetCurlOpt(curl);
    curl_multi_add_handle(curlMulti, curl);

    return true;
}

void CUrlAdp::UploadFile()
{
    int isRuning = 0;
    bool ret = false;

    CurlGlobalInit();
    curlMulti_ = curl_multi_init();
    if (curlMulti_ == nullptr) {
        FailNotify(UPLOAD_ERRORCODE_UPLOAD_LIB_ERROR);
        CurlGlobalCleanup();
        return;
    }

    ret = MultiAddHandle(curlMulti_, curlArray_);
    if (ret == false) {
        FailNotify(UPLOAD_ERRORCODE_UPLOAD_LIB_ERROR);
        return;
    }
    curl_multi_perform(curlMulti_, &isRuning);
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "isRuning = %{public}d", isRuning);
    do {
        int numfds = 0;
        int res = curl_multi_wait(curlMulti_, NULL, 0, TRANS_TIMEOUT_MS, &numfds);
        if (res != CURLM_OK) {
            FailNotify(UPLOAD_ERRORCODE_UPLOAD_LIB_ERROR);
            return;
        }
        curl_multi_perform(curlMulti_, &isRuning);
    } while (isRuning);
    CheckUploadStatus(curlMulti_);
}

void CUrlAdp::CurlGlobalInit()
{
    std::lock_guard<std::mutex> guard(curlMutex_);
    if (!isCurlGlobalInit_) {
        isCurlGlobalInit_ = true;
    }
}

void CUrlAdp::CurlGlobalCleanup()
{
    std::lock_guard<std::mutex> guard(curlMutex_);
    if (isCurlGlobalInit_) {
        isCurlGlobalInit_ = false;
    }
}

void CUrlAdp::SetCurlOpt(CURL *curl)
{
    curl_easy_setopt(curl, CURLOPT_URL, config_->url.c_str());
    curl_easy_setopt(curl, CURLOPT_VERBOSE, 1L);
    curl_easy_setopt(curl, CURLOPT_HEADERFUNCTION, HeaderCallback);
    curl_easy_setopt(curl, CURLOPT_HEADERDATA, this);
    curl_easy_setopt(curl, CURLOPT_XFERINFOFUNCTION, ProgressCallback);
    curl_easy_setopt(curl, CURLOPT_XFERINFODATA, &mfileData_);
    curl_easy_setopt(curl, CURLOPT_NOPROGRESS, 0L);
    curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, 30L);
    curl_easy_setopt(curl, CURLOPT_UPLOAD_BUFFERSIZE, 8192L);
    curl_easy_setopt(curl, CURLOPT_NOSIGNAL, 1L);
    curl_easy_setopt(curl, CURLOPT_SSL_VERIFYPEER, 0L);
    curl_easy_setopt(curl, CURLOPT_SSL_VERIFYHOST, 0L);
}

void CUrlAdp::CheckUploadStatus(CURLM *curlMulti)
{
    int msgsLeft = 0;
    CURLMsg* msg = NULL;
    while ((msg = curl_multi_info_read(curlMulti, &msgsLeft))) {
        CURL *eh = NULL;
        if (msg->msg == CURLMSG_DONE) {
            eh = msg->easy_handle;
            int returnCode = msg->data.result;
            if (returnCode != CURLE_OK) {
                FailNotify(UPLOAD_ERRORCODE_UPLOAD_FAIL);
                UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "Curl error code = %{public}d", msg->data.result);
                continue;
            }

            int statusCode = 0;
            char *szUrl = NULL;
            curl_easy_getinfo(eh, CURLINFO_RESPONSE_CODE, &statusCode);
            curl_easy_getinfo(eh, CURLINFO_PRIVATE, &szUrl);
            UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "statusCode is %{public}d, Url is %{public}s", statusCode, szUrl);
        }
    }
}

bool CUrlAdp::Remove()
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "remove");
    isReadAbort_ = true;
    return true;
}

bool CUrlAdp::RemoveInner()
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

int CUrlAdp::ProgressCallback(void *clientp, curl_off_t dltotal, curl_off_t dlnow, curl_off_t ultotal, curl_off_t ulnow)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>ProgressCallback thread id is %{public}lu", pthread_self());
    FileData *fData = (FileData *) clientp;
    CUrlAdp *url = (CUrlAdp *) fData->adp;
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>ProgressCallback ulnow is %{public}lld", ulnow);
    fData->upsize = ulnow;
    int64_t totalulnow = 0;
    if (url && url->uploadTask_) {
        for (auto &vmem : url->fileArray_) {
            UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>ProgressCallback vmem.name is %{public}s", vmem.name.c_str());
            if (fData->name == vmem.name) {
                vmem.upsize = fData->upsize;
            }
            totalulnow += vmem.upsize;
            UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>ProgressCallback vmem.upsize is %{public}lld", vmem.upsize);
        }
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>ProgressCallback totalulnow is %{public}lld", totalulnow);
        url->uploadTask_->OnProgress(dltotal, dlnow, ultotal, totalulnow);
    }
    return 0;
}

size_t CUrlAdp::HeaderCallback(char *buffer, size_t size, size_t nitems, void *userdata)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "size is %{public}u, nitems is %{public}u", size, nitems);
    CUrlAdp* url = (CUrlAdp*) userdata;
    if (url && url->uploadTask_) {
        url->uploadTask_->OnHeaderReceive(buffer, size, nitems);
    }
    return size * nitems;
}

size_t CUrlAdp::ReadCallback(char *buffer, size_t size, size_t nitems, void *arg)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "size is %{public}u, nitems is %{public}u.", size, nitems);
    FileData *read = (FileData *) arg;
    CUrlAdp *adp = (CUrlAdp *) read->adp;
	if (adp == nullptr) {
        UPLOAD_HILOGI(UPLOAD_MODULE_FRAMEWORK, "adp is null");
        return CURL_READFUNC_ABORT;
    }
    std::lock_guard<std::mutex> guard(adp->readMutex_);
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "isReadAbort is %{public}d", adp->IsReadAbort());
    if (ferror(read->fp) || adp->IsReadAbort()) {
        UPLOAD_HILOGI(UPLOAD_MODULE_FRAMEWORK, "read abort or ferror");
        return CURL_READFUNC_ABORT;
    }
    adp->StartTimer();
    size_t readSize = fread(buffer, size, nitems, read->fp);
    adp->StopTimer();

    return readSize;
}

void CUrlAdp::FailNotify(unsigned int error)
{
    if (uploadTask_) {
        uploadTask_->OnFail(error);
    }
}

void CUrlAdp::InitTimerInfo()
{
    timerInfo_ = std::make_shared<UploadTimerInfo>();
    timerInfo_->SetType(TIMEOUTTYPE);
    timerInfo_->SetRepeat(false);
    timerInfo_->SetInterval(READFILE_TIMEOUT_MS);
    timerInfo_->SetWantAgent(nullptr);

    timerInfo_->SetCallbackInfo([this]() {
        this->isReadAbort_ = true;
        this->FailNotify(UPLOAD_ERRORCODE_UPLOAD_OUTTIME);
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "OutTime error");
        });
}

void CUrlAdp::StartTimer()
{
    timerId_ = MiscServices::TimeServiceClient::GetInstance()->CreateTimer(timerInfo_);
    if (timerId_ == 0) {
        UPLOAD_HILOGI(UPLOAD_MODULE_FRAMEWORK, "Create Timer error");
        return;
    }

    bool ret = MiscServices::TimeServiceClient::GetInstance()->StartTimer(timerId_, READFILE_TIMEOUT_MS);
    if (ret != true) {
        UPLOAD_HILOGI(UPLOAD_MODULE_FRAMEWORK, "Start Timer error");
        MiscServices::TimeServiceClient::GetInstance()->DestroyTimer(timerId_);
        timerId_ = 0;
    }

    return;
}

void CUrlAdp::StopTimer()
{
    bool ret = MiscServices::TimeServiceClient::GetInstance()->StopTimer(timerId_);
    ret = MiscServices::TimeServiceClient::GetInstance()->DestroyTimer(timerId_);
    return;
}
} // namespace OHOS::Request::Upload