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

#include "legacy/download_task.h"
#include "log.h"

namespace OHOS::Request::Download::Legacy {
bool DownloadTask::isCurlGlobalInited_ = false;

DownloadTask::DownloadTask(const std::string &token, const DownloadOption &option, const DoneFunc &callback)
    : token_(token), option_(option), callback_(callback)
{
    DOWNLOAD_HILOGI("constructor");
}

DownloadTask::~DownloadTask()
{
    DOWNLOAD_HILOGI("destroy");
    if (filp_ != nullptr) {
        fclose(filp_);
    }
    delete[] errorBuffer_;
    delete thread_;
}

FILE *DownloadTask::OpenDownloadFile() const
{
    auto downloadFile = option_.fileDir_ + '/' + option_.filename_;
    FILE *filp = fopen(downloadFile.c_str(), "w+");
    if (filp == nullptr) {
        DOWNLOAD_HILOGE("open download file failed");
    }
    return filp;
}

void DownloadTask::NotifyDone(bool successful, const std::string &errMsg)
{
    if (callback_) {
        callback_(token_, successful, errMsg);
    }
}

bool DownloadTask::SetOption(CURL *handle, curl_slist *&headers)
{
    filp_ = OpenDownloadFile();
    if (filp_ == nullptr) {
        return false;
    }
    curl_easy_setopt(handle, CURLOPT_WRITEDATA, filp_);

    errorBuffer_ = new(std::nothrow) char[CURL_ERROR_SIZE];
    if (errorBuffer_ == nullptr) {
        return false;
    }
    curl_easy_setopt(handle, CURLOPT_ERRORBUFFER, errorBuffer_);

    curl_easy_setopt(handle, CURLOPT_URL, option_.url_.c_str());
    curl_easy_setopt(handle, CURLOPT_SSL_VERIFYHOST, 0L);
    curl_easy_setopt(handle, CURLOPT_SSL_VERIFYPEER, 0L);

    if (!option_.header_.empty()) {
        for (const auto& head : option_.header_) {
            headers = curl_slist_append(headers, head.c_str());
        }
        curl_easy_setopt(handle, CURLOPT_HTTPHEADER, headers);
    }
    return true;
}

void DownloadTask::DoDownload()
{
    curl_slist *headers {};
    std::shared_ptr<CURL> handle(curl_easy_init(), [headers](CURL* handle) {
        if (headers) {
            curl_slist_free_all(headers);
        }
        curl_easy_cleanup(handle);
    });

    if (handle == nullptr) {
        NotifyDone(false, "curl failed");
        return;
    }

    if (!SetOption(handle.get(), headers)) {
        NotifyDone(false, "curl set option failed");
        return;
    }

    auto code = curl_easy_perform(handle.get());
    DOWNLOAD_HILOGI("code=%{public}d, %{public}s", code, errorBuffer_);
    NotifyDone(code == CURLE_OK, errorBuffer_);
}

void DownloadTask::Start()
{
    DOWNLOAD_HILOGD("token=%{public}s url=%{public}s file=%{public}s dir=%{public}s",
                    token_.c_str(), option_.url_.c_str(), option_.filename_.c_str(), option_.fileDir_.c_str());
    if (!isCurlGlobalInited_) {
        curl_global_init(CURL_GLOBAL_ALL);
        isCurlGlobalInited_ = true;
    }

    thread_ = new(std::nothrow) std::thread(&DownloadTask::DoDownload, this);
    if (thread_ == nullptr) {
        NotifyDone(false, "create download thread failed");
        return;
    }
    thread_->detach();
}
}