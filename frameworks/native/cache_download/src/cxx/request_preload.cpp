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
#include "log.h"
#include "utf8_utils.h"
#include "wrapper.rs.h"

namespace OHOS::Request {

/**
 * @class Data
 * @brief Wrapper for Rust data with move semantics
 */
Data::Data(rust::Box<RustData> &&data)
{
    // Take ownership of Rust data by converting to raw pointer
    data_ = data.into_raw();
}

Data::~Data()
{
    // Reconstruct Rust Box to properly deallocate memory
    rust::Box<RustData>::from_raw(data_);
}

/**
 * @brief Move constructor
 * @param other Source Data object to move from
 */
Data::Data(Data &&other) noexcept : data_(other.data_)
{
    other.data_ = nullptr;
}

/**
 * @brief Move assignment operator
 * @param other Source Data object to move from
 * @return Reference to this object
 */
Data &Data::operator=(Data &&other) &noexcept
{
    if (this != &other) {
        if (data_) {
            rust::Box<RustData>::from_raw(data_);
        }
        data_ = other.data_;
        other.data_ = nullptr;
    }
    return *this;
}

/**
 * @class CppDownloadInfo
 * @brief C++ wrapper for Rust download information
 */
CppDownloadInfo::CppDownloadInfo(rust::Box<RustDownloadInfo> rust_info)
{
    // Take ownership of Rust data
    rust_info_ = rust_info.into_raw();
}

CppDownloadInfo::~CppDownloadInfo()
{
    if (rust_info_ != nullptr) {
        rust::Box<RustDownloadInfo>::from_raw(rust_info_);
    }
}

/**
 * @brief Move constructor
 * @param other Source object to move from
 */
CppDownloadInfo::CppDownloadInfo(CppDownloadInfo &&other) noexcept : rust_info_(other.rust_info_)
{
    other.rust_info_ = nullptr;
}

/**
 * @brief Move assignment operator
 * @param other Source object to move from
 * @return Reference to this object
 */
CppDownloadInfo &CppDownloadInfo::operator=(CppDownloadInfo &&other) noexcept
{
    if (this != &other) {
        if (rust_info_) {
            rust::Box<RustDownloadInfo>::from_raw(rust_info_);
        }
        rust_info_ = other.rust_info_;
        other.rust_info_ = nullptr;
    }
    return *this;
}

// Getters for download timing information
double CppDownloadInfo::dns_time() const
{
    return rust_info_->dns_time();
}

double CppDownloadInfo::connect_time() const
{
    return rust_info_->connect_time();
}

double CppDownloadInfo::total_time() const
{
    return rust_info_->total_time();
}

double CppDownloadInfo::tls_time() const
{
    return rust_info_->tls_time();
}

double CppDownloadInfo::first_send_time() const
{
    return rust_info_->first_send_time();
}

double CppDownloadInfo::first_recv_time() const
{
    return rust_info_->first_recv_time();
}

double CppDownloadInfo::redirect_time() const
{
    return rust_info_->redirect_time();
}

/**
 * @brief Get the resource size in bytes
 * @return Resource size as int64_t
 */
int64_t CppDownloadInfo::resource_size() const
{
    return rust_info_->resource_size();
}

/**
 * @brief Get the server address
 * @return Server address as string
 */
std::string CppDownloadInfo::server_addr() const
{
    return std::string(rust_info_->server_addr());
}

/**
 * @brief Get list of DNS servers used
 * @return Vector of DNS server addresses
 */
std::vector<std::string> CppDownloadInfo::dns_servers() const
{
    std::vector<std::string> result;

    const auto &servers = rust_info_->dns_servers();

    for (const auto &server : servers) {
        result.push_back(std::string(server));
    }

    return result;
}

/**
 * @class Slice
 * @brief Template class wrapping Rust slice with common operations
 */
template<typename T> Slice<T>::Slice(std::unique_ptr<rust::Slice<T>> &&slice) : slice_(std::move(slice))
{
}

template<typename T> Slice<T>::~Slice()
{
}

// Slice access methods
template<typename T> T *Slice<T>::data() const noexcept
{
    return slice_->data();
}

template<typename T> std::size_t Slice<T>::size() const noexcept
{
    return slice_->size();
}

template<typename T> std::size_t Slice<T>::length() const noexcept
{
    return slice_->length();
}

template<typename T> bool Slice<T>::empty() const noexcept
{
    return slice_->empty();
}

template<typename T> T &Slice<T>::operator[](std::size_t n) const noexcept
{
    return (*slice_)[n];
}

/**
 * @brief Get data as a byte slice
 * @return Slice of const uint8_t
 */
Slice<const uint8_t> Data::bytes() const
{
    auto bytes = std::make_unique<rust::Slice<const uint8_t>>(data_->bytes());
    return Slice<const uint8_t>(std::move(bytes));
}

/**
 * @brief Get raw Rust slice
 * @return rust::Slice of const uint8_t
 */
rust::Slice<const uint8_t> Data::rustSlice() const
{
    return data_->bytes();
}

/**
 * @class PreloadError
 * @brief Wrapper for Rust download errors
 */
PreloadError::PreloadError(rust::Box<CacheDownloadError> &&error, rust::Box<RustDownloadInfo> &&rust_info)
    : error_(error.into_raw()), download_info_(std::make_shared<CppDownloadInfo>(std::move(rust_info)))
{
}

PreloadError::PreloadError(PreloadError &&other) noexcept
    : error_(other.error_), download_info_(std::move(other.download_info_))
{
    other.error_ = nullptr;
}

PreloadError &PreloadError::operator=(PreloadError &&other) &noexcept
{
    if (this != &other) {
        if (error_ != nullptr) {
            rust::Box<CacheDownloadError>::from_raw(error_);
        }
        error_ = other.error_;
        download_info_ = std::move(other.download_info_);
        other.error_ = nullptr;
    }
    return *this;
}

PreloadError::~PreloadError()
{
    if (error_) {
        rust::Box<CacheDownloadError>::from_raw(error_);
        error_ = nullptr;
    }
}

// Error information accessors
int32_t PreloadError::GetCode() const
{
    return error_->code();
}

std::string PreloadError::GetMessage() const
{
    return std::string(error_->message());
}

ErrorKind PreloadError::GetErrorKind() const
{
    return static_cast<ErrorKind>(error_->ffi_kind());
}
std::shared_ptr<CppDownloadInfo> PreloadError::GetDownloadInfo() const
{
    return download_info_;
}

/**
 * @class Preload
 * @brief Main class for preloading resources
 */
Preload::Preload()
{
    // Initialize with Rust service
    agent_ = cache_download_service();
}

/**
 * @class PreloadHandle
 * @brief Handle for managing individual preload tasks
 */
PreloadHandle::PreloadHandle(PreloadHandle &&other) noexcept : handle_(other.handle_)
{
    other.handle_ = nullptr;
}

PreloadHandle &PreloadHandle::operator=(PreloadHandle &&other) &noexcept
{
    if (this != &other) {
        if (handle_) {
            rust::Box<TaskHandle>::from_raw(handle_);
        }
        handle_ = other.handle_;
        other.handle_ = nullptr;
    }
    return *this;
}

// SSL type to name mapping
const std::unordered_map<SslType, std::string> SslTypeName = {
    { SslType::DEFAULT, "" },
    { SslType::TLS, "TLS" },
    { SslType::TLCP, "TLCP" },
};

/**
 * @brief Start a preload task
 * @param url URL to preload
 * @param callback Callback for task events
 * @param options Additional options for the request
 * @param update Whether to force update cached resource
 * @return Shared pointer to PreloadHandle
 */
std::shared_ptr<PreloadHandle> Preload::load(std::string const &url, std::unique_ptr<PreloadCallback> callback,
    std::unique_ptr<PreloadOptions> options, bool update)
{
    // Wrap C++ callback for Rust
    auto callback_wrapper = std::make_unique<PreloadCallbackWrapper>(callback);

    std::shared_ptr<PreloadProgressCallbackWrapper> progress_callback_wrapper = nullptr;
    if (callback != nullptr && callback->OnProgress != nullptr) {
        progress_callback_wrapper = std::make_shared<PreloadProgressCallbackWrapper>(callback);
    }

    // Prepare options for FFI call
    FfiPredownloadOptions ffiOptions = {
        .headers = rust::Vec<rust::str>(),
    };

    if (options != nullptr) {
        // Validate and set headers
        for (const auto &[key, value] : options->headers) {
            std::vector<uint8_t> key_bytes(key.begin(), key.end());
            std::vector<uint8_t> value_bytes(value.begin(), value.end());

            if (!Utf8Utils::RunUtf8Validation(key_bytes) || !Utf8Utils::RunUtf8Validation(value_bytes)) {
                return nullptr;
            }
            ffiOptions.headers.push_back(rust::str(key));
            ffiOptions.headers.push_back(rust::str(value));
        }

        ffiOptions.ssl_type = rust::str(SslTypeName.at(options->sslType));
        ffiOptions.ca_path = rust::str(options->caPath);
    }

    if (!Utf8Utils::RunUtf8Validation(std::vector<uint8_t>(url.begin(), url.end()))) {
        return nullptr;
    }

    // Start preload task through Rust agent
    auto taskHandle = agent_->ffi_preload(
        rust::str(url), std::move(callback_wrapper), std::move(progress_callback_wrapper), update, ffiOptions);
    return taskHandle;
}

/**
 * @brief Fetch data from cache or network
 * @param url URL to fetch
 * @return Optional containing Data if successful
 */
std::optional<Data> Preload::fetch(std::string const &url)
{
    if (!Utf8Utils::RunUtf8Validation(std::vector<uint8_t>(url.begin(), url.end()))) {
        return std::nullopt;
    }
    std::unique_ptr<Data> data = agent_->ffi_fetch(rust::str(url));
    if (data == nullptr) {
        return std::nullopt;
    }
    return std::move(*data);
}

/**
 * @brief Get download information for a URL
 * @param url URL to query
 * @return Optional containing CppDownloadInfo if available
 */
std::optional<CppDownloadInfo> Preload::GetDownloadInfo(std::string const &url)
{
    if (!Utf8Utils::RunUtf8Validation(std::vector<uint8_t>(url.begin(), url.end()))) {
        return std::nullopt;
    }
    std::unique_ptr<CppDownloadInfo> info = agent_->ffi_get_download_info(rust::str(url));
    if (info == nullptr) {
        return std::nullopt;
    }
    return std::move(*info);
}

// Cache configuration methods
void Preload::SetRamCacheSize(uint64_t size)
{
    agent_->set_ram_cache_size(size);
}
void Preload::SetFileCacheSize(uint64_t size)
{
    agent_->set_file_cache_size(size);
}
void Preload::SetDownloadInfoListSize(uint16_t size)
{
    agent_->set_info_list_size(size);
}

/**
 * @brief Cancel a preload task
 * @param url URL of task to cancel
 */
void Preload::Cancel(std::string const &url)
{
    if (!Utf8Utils::RunUtf8Validation(std::vector<uint8_t>(url.begin(), url.end()))) {
        return;
    }
    agent_->cancel(rust::str(url));
}

/**
 * @brief Remove cached data for URL
 * @param url URL to remove
 */
void Preload::Remove(std::string const &url)
{
    if (!Utf8Utils::RunUtf8Validation(std::vector<uint8_t>(url.begin(), url.end()))) {
        return;
    }
    agent_->remove(rust::str(url));
}

/**
 * @brief Set file cache path
 * @param path Filesystem path for cache
 */
void Preload::SetFileCachePath(const std::string &path)
{
    if (path.empty()) {
        REQUEST_HILOGE("SetFileCachePath fail.");
        return;
    }
    set_file_cache_path(rust::String(path));
}

/**
 * @brief Check if URL is in cache
 * @param url URL to check
 * @return true if cached, false otherwise
 */
bool Preload::Contains(const std::string &url)
{
    if (!Utf8Utils::RunUtf8Validation(std::vector<uint8_t>(url.begin(), url.end()))) {
        return false;
    }
    return agent_->contains(rust::str(url));
}

void Preload::ClearMemoryCache()
{
    agent_->clear_memory_cache();
}

void Preload::ClearFileCache()
{
    agent_->clear_file_cache();
}

/**
 * @brief Get singleton instance
 * @return Pointer to Preload instance
 */
Preload *Preload::GetInstance()
{
    static Preload agent;
    return &agent;
}

/**
 * @brief Construct PreloadHandle from Rust handle
 * @param handle Rust task handle
 */
PreloadHandle::PreloadHandle(rust::Box<TaskHandle> handle)
{
    handle_ = handle.into_raw();
}

PreloadHandle::~PreloadHandle()
{
    rust::Box<TaskHandle>::from_raw(handle_);
}

/**
 * @brief Cancel the associated task
 */
void PreloadHandle::Cancel()
{
    handle_->cancel();
}

/**
 * @brief Get task ID
 * @return Task ID as string
 */
std::string PreloadHandle::GetTaskId()
{
    return std::string(handle_->task_id());
}

/**
 * @brief Check if task is finished
 * @return true if finished, false otherwise
 */
bool PreloadHandle::IsFinish()
{
    return handle_->is_finish();
}

/**
 * @brief Get current task state
 * @return PreloadState enum value
 */
PreloadState PreloadHandle::GetState()
{
    return static_cast<PreloadState>(handle_->state());
}

// Explicit template instantiation
template class Slice<const uint8_t>;

} // namespace OHOS::Request