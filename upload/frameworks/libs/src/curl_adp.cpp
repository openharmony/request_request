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
#include <cinttypes>
#include "upload_task.h"
#include "upload_hilog_wrapper.h"
#include "time_service_client.h"
#include "curl_adp.h"

namespace OHOS::Request::Upload {
constexpr int TRANS_TIMEOUT_MS = 300 * 1000;
constexpr int READFILE_TIMEOUT_MS = 30 * 1000;
constexpr int TIMEOUTTYPE = 1;
constexpr int SLEEP = 1000;
constexpr int COLLECT_DO_FLAG = 1;
constexpr int COLLECT_END_FLAG = 2;

CUrlAdp::CUrlAdp(std::vector<FileData>& fileArray, std::shared_ptr<UploadConfig>& config)
{
    fileArray_ = fileArray;
    config_ = config;
    isCurlGlobalInit_ = false;
    isReadAbort_ = false;
    curlMulti_ = nullptr;
    timerId_ = 0;
    timerInfo_ = nullptr;
    for (auto &vmem : fileArray_) {
        vmem.upsize = 0;
        vmem.totalsize = 0;
        vmem.fileIndex = 0;
        vmem.mcurl = nullptr;
        vmem.headSendFlag = 0;
        vmem.httpCode = 0;
        vmem.list = nullptr;
    }
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
    int index = 0;
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
        mfileData_.responseHead.clear();
        if (mfileData_.list) {
            curl_slist_free_all(mfileData_.list);
            mfileData_.list = nullptr;
        }
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
    if (fstat(fileno(mfileData_.fp), &fileInfo) != 0) {
        UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "get the file info fail");
        FailNotify(UPLOAD_ERRORCODE_UPLOAD_LIB_ERROR);
        return false;
    }
    CURL *curl = curl_easy_init();
    if (curl == nullptr) {
        FailNotify(UPLOAD_ERRORCODE_UPLOAD_LIB_ERROR);
        return false;
    }
    SetHeadData(curl);
    curlArray.push_back(curl);
    mime = curl_mime_init(curl);
    if (config_->data.size()) {
        for (auto &vdata : config_->data) {
            part = curl_mime_addpart(mime);
            curl_mime_name(part, vdata.name.c_str());
            curl_mime_data(part, vdata.value.c_str(), vdata.value.size());
        }
    }
    part = curl_mime_addpart(mime);
    if (mfileData_.name.size()) {
        curl_mime_name(part, mfileData_.name.c_str());
    } else {
        curl_mime_name(part, "file");
    }
    curl_mime_type(part, mfileData_.type.c_str());
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===> MultiAddHandle mfileData_.type.c_str() %{public}s",
        mfileData_.type.c_str());
    curl_mime_filename(part, mfileData_.filename.c_str());
    mfileData_.adp = this;
    mfileData_.totalsize = fileInfo.st_size;
    curl_mime_data_cb(part, fileInfo.st_size, ReadCallback, NULL, NULL, &mfileData_);
    curl_easy_setopt(curl, CURLOPT_MIMEPOST, mime);
    SetCurlOpt(curl);
    curl_multi_add_handle(curlMulti, curl);
    return true;
}

void CUrlAdp::SetHeadData(CURL *curl)
{
    if (config_->header.size()) {
        for (auto &headerData : config_->header) {
            mfileData_.list = curl_slist_append(mfileData_.list, headerData.c_str());
        }
    }
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, mfileData_.list);
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
    curl_easy_setopt(curl, CURLOPT_HEADERDATA, &mfileData_);
    if (config_->protocolVersion == "L5") {
        curl_easy_setopt(curl, CURLOPT_HEADERFUNCTION, HeaderCallbackL5);
    } else {
        curl_easy_setopt(curl, CURLOPT_HEADERFUNCTION, HeaderCallback);
        curl_easy_setopt(curl, CURLOPT_XFERINFOFUNCTION, ProgressCallback);
        curl_easy_setopt(curl, CURLOPT_XFERINFODATA, &mfileData_);
    }
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
        if (msg->msg != CURLMSG_DONE) {
            continue;
        }
        eh = msg->easy_handle;
        int returnCode = msg->data.result;
        if (returnCode != CURLE_OK) {
            if (config_->protocolVersion != "L5") {
                FailNotify(UPLOAD_ERRORCODE_UPLOAD_FAIL);
                UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "Curl error code = %{public}d", msg->data.result);
            }
            continue;
        }
        int statusCode = 0;
        char *szUrl = NULL;
        curl_easy_getinfo(eh, CURLINFO_RESPONSE_CODE, &statusCode);
        curl_easy_getinfo(eh, CURLINFO_PRIVATE, &szUrl);
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "statusCode is %{public}d, Url is %{public}s", statusCode, szUrl);
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

int CUrlAdp::OnDebug(CURL *curl, curl_infotype itype, char *pData, size_t size, void *lpvoid)
{
    if (itype == CURLINFO_TEXT) {
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>OnDebug CURLINFO_TEXT is %{public}s", pData);
    } else if (itype == CURLINFO_HEADER_IN) {
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>OnDebug CURLINFO_HEADER_IN is %{public}s", pData);
    } else if (itype == CURLINFO_HEADER_OUT) {
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>OnDebug CURLINFO_HEADER_OUT is %{public}s", pData);
    } else if (itype == CURLINFO_DATA_IN) {
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>OnDebug CURLINFO_DATA_IN is %{public}s", pData);
    } else if (itype == CURLINFO_DATA_OUT) {
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>OnDebug CURLINFO_DATA_OUT is %{public}s", pData);
    }
    return (int)itype;
}
int CUrlAdp::ProgressCallback(void *clientp, curl_off_t dltotal, curl_off_t dlnow, curl_off_t ultotal, curl_off_t ulnow)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>ProgressCallback thread id is %{public}lu", pthread_self());
    FileData *fData = (FileData *) clientp;
    CUrlAdp *url = (CUrlAdp *) fData->adp;
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>ProgressCallback ultotal is %{public}" PRIu64, ultotal);
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>ProgressCallback ulnow is %{public}" PRIu64, ulnow);
    UPLOAD_HILOGD(
        UPLOAD_MODULE_FRAMEWORK, "===>ProgressCallback fData->totalsize is %{public}" PRIu64, fData->totalsize);
    if (ulnow > 0) {
        fData->upsize = fData->totalsize - (ultotal - ulnow);
    } else {
        fData->upsize = ulnow;
    }

    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>ProgressCallback fData->totalsize - (ultotal - ulnow) is %{public}lld",
        (long long)fData->upsize);
    int64_t totalulnow = 0;
    if (url && url->uploadTask_) {
        for (auto &vmem : url->fileArray_) {
            UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>ProgressCallback vmem.filename is %{public}s",
                vmem.filename.c_str());
            if (fData->filename == vmem.filename) {
                vmem.upsize = fData->upsize;
            }
            totalulnow += vmem.upsize;
            UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>ProgressCallback vmem.upsize is %{public}lld",
                (long long)vmem.upsize);
        }
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>ProgressCallback totalulnow is %{public}lld",
            (long long)totalulnow);
        url->uploadTask_->OnProgress(dltotal, dlnow, ultotal, totalulnow);
    }
    return 0;
}

size_t CUrlAdp::HeaderCallback(char *buffer, size_t size, size_t nitems, void *userdata)
{
    FileData *fData = (FileData *) userdata;
    CUrlAdp *url = (CUrlAdp *) fData->adp;
    std::string stmp(buffer, size * nitems);
    uint32_t isize = 1;
    const int32_t codeOk = 200;
    const std::string headEndFlag = "\r\n";

    if (std::string::npos != stmp.find("HTTP")) {
        fData->headSendFlag = COLLECT_DO_FLAG;
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>HeaderCallback collect begin  is %{public}s", stmp.c_str());
        const int codeLen = 3;
        std::string::size_type position = stmp.find_first_of(" ");
        std::string scode(stmp, position + 1, codeLen);
        fData->httpCode = std::stol(scode);
    } else if (stmp == headEndFlag) {
        fData->headSendFlag = COLLECT_END_FLAG;
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>HeaderCallback collect end  is %{public}s", stmp.c_str());
    }
    if (COLLECT_DO_FLAG == fData->headSendFlag || COLLECT_END_FLAG == fData->headSendFlag) {
        fData->responseHead.push_back(stmp);
    }
    if (url && url->uploadTask_ && COLLECT_END_FLAG == fData->headSendFlag) {
        std::string stoatalHead = "";
        for (auto &smem : fData->responseHead) {
            UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>HeaderCallback smem is %{public}s", smem.c_str());
            stoatalHead += smem;
        }
        char sbuff[stoatalHead.length()];
        (void)memset_s(sbuff, stoatalHead.length(), 0, stoatalHead.length());
        (void)memcpy_s(sbuff, stoatalHead.length(), stoatalHead.c_str(), stoatalHead.length());
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>HeaderCallback stoatalHead is %{public}s", stoatalHead.c_str());
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>HeaderCallback stoatalHead.length() is %{public}zu",
            stoatalHead.length());
        nitems = stoatalHead.length();
        size = isize;
        if (codeOk == fData->httpCode) {
            if (url->fileArray_.size() == fData->fileIndex) {
                url->uploadTask_->OnHeaderReceive(sbuff, size, nitems);
            }
        } else {
            url->uploadTask_->OnHeaderReceive(sbuff, size, nitems);
        }
        fData->responseHead.clear();
        fData->httpCode = 0;
    }
    return size * nitems;
}

size_t CUrlAdp::HeaderCallbackL5(char *buffer, size_t size, size_t nitems, void *userdata)
{
    FileData *fData = (FileData *) userdata;
    CUrlAdp *url = (CUrlAdp *) fData->adp;
    std::string stmp(buffer, size * nitems);
    uint32_t isize = 1;
    const int32_t codeOk = 200;
    UploadResponse resData;
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
    if (COLLECT_DO_FLAG == fData->headSendFlag || COLLECT_END_FLAG == fData->headSendFlag) {
        fData->responseHead.push_back(stmp);
    }
    if (url && url->uploadTask_ && COLLECT_END_FLAG == fData->headSendFlag) {
        std::string stoatalHead = "";
        for (auto &smem : fData->responseHead) {
            stoatalHead += smem;
        }
        size = isize;
        nitems = stoatalHead.length();
        if (codeOk == fData->httpCode) {
            if (url->fileArray_.size() == fData->fileIndex && url->config_->fsuccess != nullptr) {
                resData.headers = stoatalHead;
                resData.code = fData->httpCode;
                UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>HeaderCallbackL5 success response head is %{public}s",
                    resData.headers.c_str());
                url->config_->fsuccess(resData);
            }
        } else {
            if (url->config_->ffail) {
                url->config_->ffail(stoatalHead, fData->httpCode);
            }
            UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "===>HeaderCallbackL5 fail Data.code is %{public}d", resData.code);
        }
        fData->responseHead.clear();
        fData->httpCode = 0;
    }
    return size * nitems;
}

size_t CUrlAdp::ReadCallback(char *buffer, size_t size, size_t nitems, void *arg)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "size is %{public}zu, nitems is %{public}zu.", size, nitems);
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
        if (config_->protocolVersion != "L5") {
            uploadTask_->OnFail(error);
        }
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
        this->FailNotify(UPLOAD_ERRORCODE_UPLOAD_OUTTIME);
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "OutTime error");
        this->isReadAbort_ = true;
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