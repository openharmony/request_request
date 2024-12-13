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

#include "request_preload.h"

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

PreloadError::PreloadError(rust::Box<CacheDownloadError> error)
{
    this->_error = error.into_raw();
}

PreloadError::~PreloadError()
{
    rust::Box<CacheDownloadError>::from_raw(this->_error);
}

int32_t PreloadError::GetCode() const
{
    return this->_error->code();
}

std::string PreloadError::GetMessage() const
{
    return std::string(this->_error->message());
}

ErrorKind PreloadError::GetErrorKind() const
{
    return static_cast<ErrorKind>(this->_error->ffi_kind());
}

Preload::Preload()
{
    this->_agent = cache_download_service();
}

std::shared_ptr<PreloadHandle> Preload::load(std::string const &url, std::unique_ptr<PreloadCallback> callback,
    std::unique_ptr<PreloadOptions> options, bool update)
{
    auto callback_wrapper = std::make_unique<PreloadCallbackWrapper>(callback);

    std::shared_ptr<PreloadProgressCallbackWrapper> progress_callback_wrapper = nullptr;
    if (callback != nullptr && callback->OnProgress != nullptr) {
        progress_callback_wrapper = std::make_shared<PreloadProgressCallbackWrapper>(callback);
    }

    FfiPredownloadOptions ffiOptions = { .headers = rust::Vec<rust::str>() };
    if (options != nullptr) {
        for (auto header : options->headers) {
            ffiOptions.headers.push_back(std::get<0>(header));
            ffiOptions.headers.push_back(std::get<1>(header));
        }
    }
    auto taskHandle = this->_agent->ffi_preload(
        rust::str(url), std::move(callback_wrapper), std::move(progress_callback_wrapper), update, ffiOptions);
    return std::make_shared<PreloadHandle>(std::move(taskHandle));
}

void Preload::SetRamCacheSize(uint64_t size)
{
    this->_agent->set_ram_cache_size(size);
}
void Preload::SetFileCacheSize(uint64_t size)
{
    this->_agent->set_file_cache_size(size);
}

void Preload::Cancel(std::string const &url)
{
    this->_agent->cancel(rust::str(url));
}

void Preload::Remove(std::string const &url)
{
    this->_agent->remove(rust::str(url));
}

bool Preload::Contains(const std::string &url)
{
    return this->_agent->contains(rust::str(url));
}

Preload *Preload::GetInstance()
{
    static Preload agent;
    return &agent;
}

PreloadHandle::PreloadHandle(rust::Box<TaskHandle> handle)
{
    this->_handle = handle.into_raw();
}

PreloadHandle::~PreloadHandle()
{
    rust::Box<TaskHandle>::from_raw(this->_handle);
}

void PreloadHandle::Cancel()
{
    this->_handle->cancel();
}

std::string PreloadHandle::GetTaskId()
{
    return std::string(this->_handle->task_id());
}

bool PreloadHandle::IsFinish()
{
    return this->_handle->is_finish();
}

PreloadState PreloadHandle::GetState()
{
    return static_cast<PreloadState>(this->_handle->state());
}

} // namespace OHOS::Request