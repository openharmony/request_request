/*
* Copyright (C) 2024 Huawei Device Co., Ltd.
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

#include "request_pre_download.h"

#include <cstdint>
#include <memory>

#include "cxx.h"
#include "wrapper.rs.h"

namespace OHOS::Request {
Data::Data(rust::Box<RustData> data)
{
    this->_data = data.into_raw();
}
Data::~Data()
{
    rust::Box<RustData>::from_raw(this->_data);
}

rust::Slice<const uint8_t> Data::bytes()
{
    return this->_data->bytes();
}

PreDownloadError::PreDownloadError(rust::Box<DownloadError> error)
{
    this->_error = error.into_raw();
}

PreDownloadError::~PreDownloadError()
{
    rust::Box<DownloadError>::from_raw(this->_error);
}

int32_t PreDownloadError::GetCode() const
{
    return this->_error->code();
}

std::string PreDownloadError::GetMessage() const
{
    return std::string(this->_error->message());
}

ErrorKind PreDownloadError::GetErrorKind() const
{
    return static_cast<ErrorKind>(this->_error->ffi_kind());
}

PreDownloadAgent::PreDownloadAgent()
{
    this->_agent = download_agent();
}

std::shared_ptr<PreDownloadHandle> PreDownloadAgent::Download(
    std::string const &url, std::unique_ptr<DownloadCallback> callback, std::unique_ptr<PreDownloadOptions> options)
{
    auto callback_wrapper = std::make_unique<DownloadCallbackWrapper>(std::move(callback));
    FfiPredownloadOptions ffiOptions = { .headers = rust::Vec<rust::str>() };
    if (options != nullptr) {
        for (auto header : options->headers) {
            ffiOptions.headers.push_back(std::get<0>(header));
            ffiOptions.headers.push_back(std::get<1>(header));
        }
    }
    auto taskHandle = this->_agent->ffi_pre_download(rust::str(url), std::move(callback_wrapper), false, ffiOptions);
    return std::make_shared<PreDownloadHandle>(std::move(taskHandle));
}

void PreDownloadAgent::SetRamCacheSize(uint64_t size)
{
    this->_agent->set_ram_cache_size(size);
}
void PreDownloadAgent::SetFileCacheSize(uint64_t size)
{
    this->_agent->set_file_cache_size(size);
}

void PreDownloadAgent::Cancel(std::string const &url)
{
    this->_agent->cancel(rust::str(url));
}

void PreDownloadAgent::Remove(std::string const &url)
{
    this->_agent->remove(rust::str(url));
}

PreDownloadAgent *PreDownloadAgent::GetInstance()
{
    static PreDownloadAgent agent;
    return &agent;
}

PreDownloadHandle::PreDownloadHandle(rust::Box<TaskHandle> handle)
{
    this->_handle = handle.into_raw();
}

PreDownloadHandle::~PreDownloadHandle()
{
    rust::Box<TaskHandle>::from_raw(this->_handle);
}

void PreDownloadHandle::Cancel()
{
    this->_handle->cancel();
}

std::string PreDownloadHandle::GetTaskId()
{
    return std::string(this->_handle->task_id());
}

bool PreDownloadHandle::IsFinish()
{
    return this->_handle->is_finish();
}

PreDownloadState PreDownloadHandle::GetState()
{
    return static_cast<PreDownloadState>(this->_handle->state());
}

} // namespace OHOS::Request