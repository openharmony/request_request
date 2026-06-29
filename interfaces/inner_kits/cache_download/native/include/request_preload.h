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

#ifndef REQUEST_PRE_DOWNLOAD_H
#define REQUEST_PRE_DOWNLOAD_H

#include <cstdint>
#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <tuple>
#include <vector>

namespace rust {
inline namespace cxxbridge1 {
template<typename T> class Box;
template<typename T> class Slice;
} // namespace cxxbridge1
} // namespace rust

namespace OHOS::Request {
struct RustData;
struct TaskHandle;
struct CacheDownloadService;
struct CacheDownloadError;
struct RustDownloadInfo;

/**
 * @brief Running state of a preload task.
 */
enum class PreloadState {
    /// Created but not yet started
    INIT,
    /// Performing network request and download
    RUNNING,
    /// Successfully completed and written to cache
    SUCCESS,
    /// Failed (network error, timeout, etc.)
    FAIL,
    /// Actively cancelled
    CANCEL,
};

/**
 * @brief SSL/TLS protocol type used by a preload request.
 */
enum SslType {
    /// Use the system default protocol
    DEFAULT,
    /// Force the use of the TLS protocol
    TLS,
    /// Use the national cipher TLCP protocol
    TLCP,
};

/**
 * @brief Cache write strategy.
 */
enum class CacheStrategy : uint32_t {
    /// Download and cache immediately
    FORCE = 0,
    /// Download on demand / lazy load
    LAZY = 1,
};

/**
 * Task retry configuration.
 */
struct RetryOptions {
    /**
     * Maximum number of retry attempts.
     * The default value is 1.
     * The minimum value is 0.
     * The maximum value is 10.
     * When set to 0, no retries will be performed.
     */
    int32_t maxRetryCount;
};

/**
 * Task timeout configuration.
 */
struct TimeoutOptions {
    /**
     * Network availability check timeout, in seconds.
     * The default value is 20.
     * The minimum value is 0.
     * The maximum value is 20.
     * When set to 0, no check will be performed.
     */
    int32_t networkCheckTimeout;
    /**
     * Complete HTTP request-response cycle timeout, in seconds.
     * The default value is 60.
     * The minimum value is 1.
     */
    int32_t httpTotalTimeout;
};

/**
 * @brief Safe slice wrapper template across the Rust/C++ boundary, accessing
 * contiguous memory by element count.
 */
template<typename T> class Slice {
public:
    Slice(std::unique_ptr<rust::Slice<T>> &&slice);
    ~Slice();
    /// Get the underlying data pointer
    T *data() const noexcept;
    /// Number of elements
    std::size_t size() const noexcept;
    /// Number of elements, equivalent to size()
    std::size_t length() const noexcept;
    /// Whether it contains no elements
    bool empty() const noexcept;
    /// Access an element by index
    T &operator[](std::size_t n) const noexcept;

private:
    std::unique_ptr<rust::Slice<T>> slice_;
};

/**
 * @brief Holder of preload downloaded data, bridging Rust-side RustData.
 *
 * Move-only. It provides two access styles: a C++ byte slice and a raw Rust
 * slice, used to safely pass downloaded binary content across language
 * boundaries.
 */
class Data {
public:
    Data(rust::Box<RustData> &&data);
    Data(Data &&) noexcept;
    ~Data();
    Data &operator=(Data &&) &noexcept;

    /// Return the byte content as a C++ slice
    Slice<const uint8_t> bytes() const;
    /// Return the byte content as a Rust slice
    rust::Slice<const uint8_t> rustSlice() const;

private:
    RustData *data_;
};

/**
 * @brief Category of preload errors.
 */
enum ErrorKind {
    /// HTTP protocol layer error (e.g. abnormal status code)
    HTTP,
    /// Input/output error
    IO,
    /// Domain name resolution error
    DNS,
    /// TCP connection or transport error
    TCP,
    /// SSL/TLS handshake or certificate error
    SSL,
    /// Other uncategorized errors
    OTHERS
};

/**
 * @brief Statistics of a single download (move-only), bridging Rust-side
 * RustDownloadInfo.
 *
 * Provides metrics such as the duration of each download phase and the target
 * address, used for performance analysis and issue triage.
 */
class CppDownloadInfo {
public:
    CppDownloadInfo(rust::Box<RustDownloadInfo> rust_info);
    CppDownloadInfo(CppDownloadInfo &&other) noexcept;
    CppDownloadInfo &operator=(CppDownloadInfo &&other) noexcept;

    CppDownloadInfo(const CppDownloadInfo &) = delete;
    CppDownloadInfo &operator=(const CppDownloadInfo &) = delete;

    ~CppDownloadInfo();

    /// DNS resolution duration (seconds)
    double dns_time() const;
    /// TCP connection establishment duration (seconds)
    double connect_time() const;
    /// TLS/SSL handshake duration (seconds)
    double tls_time() const;
    /// Duration from connection ready to first request sent (seconds)
    double first_send_time() const;
    /// Duration from request sent to first byte received (seconds)
    double first_recv_time() const;
    /// Cumulative duration of handling HTTP redirects (seconds)
    double redirect_time() const;
    /// Total duration of the whole download flow (seconds)
    double total_time() const;
    /// Resource size in bytes
    int64_t resource_size() const;
    /// Actual connected server address
    std::string server_addr() const;
    /// List of DNS servers used during resolution
    std::vector<std::string> dns_servers() const;

private:
    RustDownloadInfo *rust_info_;
};

/**
 * @brief Preload failure information (move-only), aggregating the error code,
 * description, category and download statistics.
 */
class PreloadError {
public:
    PreloadError(rust::Box<CacheDownloadError> &&error, rust::Box<RustDownloadInfo> &&rust_info);
    PreloadError(PreloadError &&) noexcept;
    PreloadError &operator=(PreloadError &&) &noexcept;
    ~PreloadError();

    /// Error code
    int32_t GetCode() const;
    /// Human-readable error description
    std::string GetMessage() const;
    /// Error category
    ErrorKind GetErrorKind() const;
    /// Download statistics on failure, may be empty
    std::shared_ptr<CppDownloadInfo> GetDownloadInfo() const;

private:
    CacheDownloadError *error_;
    std::shared_ptr<CppDownloadInfo> download_info_;
};

/**
 * @brief Set of event callbacks for a preload task, implemented by the caller
 * and passed to load().
 */
struct PreloadCallback {
    /// Download success callback, returns the downloaded data and task ID.
    std::function<void(const std::shared_ptr<Data> &&, const std::string &TaskId)> OnSuccess;
    /// Download failure callback, returns the error information and task ID.
    std::function<void(const PreloadError &, const std::string &TaskId)> OnFail;
    /// Task cancelled callback.
    std::function<void()> OnCancel;
    /// Download progress callback; current is downloaded bytes, total is total resource bytes.
    std::function<void(uint64_t current, uint64_t total)> OnProgress;
};

/**
 * @brief Preload task handle (move-only), used to query state or actively
 * cancel after the task completes.
 */
class PreloadHandle {
public:
    PreloadHandle(PreloadHandle &&) noexcept;
    PreloadHandle(rust::Box<TaskHandle>);
    PreloadHandle &operator=(PreloadHandle &&) &noexcept;

    ~PreloadHandle();
    /// Cancel this preload task
    void Cancel();
    /// Get the unique task identifier
    std::string GetTaskId();
    /// Whether the task has reached a terminal state (success/failure/cancel)
    bool IsFinish();
    /// Get the current task state
    PreloadState GetState();

private:
    TaskHandle *handle_;
};

/**
 * @brief Optional configuration for a single preload request.
 */
struct PreloadOptions {
    /// Custom HTTP request header key-value pairs
    std::vector<std::tuple<std::string, std::string>> headers;
    /// SSL/TLS protocol type
    SslType sslType;
    /// Custom CA certificate path
    std::string caPath;
    /// Retry configuration
    RetryOptions retry;
    /// Timeout configuration
    TimeoutOptions timeout;
};

/**
 * @brief External entry of preload (cache download), as a singleton.
 *
 * Provides capabilities such as starting a preload by URL, cancelling,
 * removing and querying by URL, cache capacity and path configuration,
 * global retry/timeout configuration, and synchronously fetching cached
 * data.
 */
class Preload {
public:
    Preload();
    /**
     * @brief Get the Preload singleton pointer.
     * @return Singleton pointer, unique within the process.
     */
    static Preload *GetInstance();
    virtual ~Preload() = default;
    /// Cancel the preload task for the specified URL
    void Cancel(std::string const &url);
    /// Remove the preload task and its cache for the specified URL
    void Remove(std::string const &url);
    /// Whether a preload task or cache exists for the specified URL
    bool Contains(std::string const &url);

    /// Set the memory cache capacity upper limit (bytes)
    void SetRamCacheSize(uint64_t size);
    /// Set the file cache capacity upper limit (bytes)
    void SetFileCacheSize(uint64_t size);
    /// Set the maximum number of entries in the download info list
    void SetDownloadInfoListSize(uint16_t size);
    /// Set the file cache directory (static, takes effect globally)
    static void SetFileCachePath(const std::string &path);

    /// Clear the memory cache
    void ClearMemoryCache();
    /// Clear the file cache
    void ClearFileCache();

    /// Set the global retry policy
    void SetGlobalRetryOptions(const RetryOptions &options);
    /// Set the global timeout policy
    void SetGlobalTimeoutOptions(const TimeoutOptions &options);

    /**
     * @brief Start a preload for the specified URL.
     * @param url Resource address to be preloaded.
     * @param callback Task event callback, must not be null.
     * @param options Optional configuration for this request; when null, global config is used.
     * @param update Whether to force-refresh cached content (true means re-download even if cached).
     * @return Task handle, which can be used to cancel or query state later.
     */
    std::shared_ptr<PreloadHandle> load(std::string const &url, std::unique_ptr<PreloadCallback>,
        std::unique_ptr<PreloadOptions> options = nullptr, bool update = false);

    /**
     * @brief Synchronously fetch the cached data for the specified URL.
     * @param url Resource address.
     * @return Returns the data if the cache is hit, otherwise an empty optional.
     */
    std::optional<Data> fetch(std::string const &url);
    /**
     * @brief Get the download statistics for the specified URL.
     * @param url Resource address.
     * @return Returns the statistics if a record exists, otherwise an empty optional.
     */
    std::optional<CppDownloadInfo> GetDownloadInfo(std::string const &url);

private:
    const CacheDownloadService *agent_;
};

} // namespace OHOS::Request

#endif // REQUEST_PRE_DOWNLOAD_H