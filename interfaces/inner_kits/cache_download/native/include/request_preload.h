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

enum class PreloadState {
    INIT,
    RUNNING,
    SUCCESS,
    FAIL,
    CANCEL,
};

enum SslType {
    DEFAULT,
    TLS,
    TLCP,
};

enum class CacheStrategy : uint32_t {
    FORCE = 0,
    LAZY = 1,
};

template<typename T> class Slice {
public:
    Slice(std::unique_ptr<rust::Slice<T>> &&slice);
    ~Slice();
    T *data() const noexcept;
    std::size_t size() const noexcept;
    std::size_t length() const noexcept;
    bool empty() const noexcept;
    T &operator[](std::size_t n) const noexcept;

private:
    std::unique_ptr<rust::Slice<T>> slice_;
};

class Data {
public:
    Data(rust::Box<RustData> &&data);
    Data(Data &&) noexcept;
    ~Data();
    Data &operator=(Data &&) &noexcept;

    Slice<const uint8_t> bytes() const;
    rust::Slice<const uint8_t> rustSlice() const;

private:
    RustData *data_;
};

enum ErrorKind {
    HTTP,
    IO,
    DNS,
    TCP,
    SSL,
    OTHERS
};

class CppDownloadInfo {
public:
    CppDownloadInfo(rust::Box<RustDownloadInfo> rust_info);
    CppDownloadInfo(CppDownloadInfo &&other) noexcept;
    CppDownloadInfo &operator=(CppDownloadInfo &&other) noexcept;

    CppDownloadInfo(const CppDownloadInfo &) = delete;
    CppDownloadInfo &operator=(const CppDownloadInfo &) = delete;

    ~CppDownloadInfo();

    double dns_time() const;
    double connect_time() const;
    double tls_time() const;
    double first_send_time() const;
    double first_recv_time() const;
    double redirect_time() const;
    double total_time() const;
    int64_t resource_size() const;
    std::string server_addr() const;
    std::vector<std::string> dns_servers() const;

private:
    RustDownloadInfo *rust_info_;
};

class PreloadError {
public:
    PreloadError(rust::Box<CacheDownloadError> &&error, rust::Box<RustDownloadInfo> &&rust_info);
    PreloadError(PreloadError &&) noexcept;
    PreloadError &operator=(PreloadError &&) &noexcept;
    ~PreloadError();

    int32_t GetCode() const;
    std::string GetMessage() const;
    ErrorKind GetErrorKind() const;
    std::shared_ptr<CppDownloadInfo> GetDownloadInfo() const;

private:
    CacheDownloadError *error_;
    std::shared_ptr<CppDownloadInfo> download_info_;
};

struct PreloadCallback {
    std::function<void(const std::shared_ptr<Data> &&, const std::string &TaskId)> OnSuccess;
    std::function<void(const PreloadError &, const std::string &TaskId)> OnFail;
    std::function<void()> OnCancel;
    std::function<void(uint64_t current, uint64_t total)> OnProgress;
};

class PreloadHandle {
public:
    PreloadHandle(PreloadHandle &&) noexcept;
    PreloadHandle(rust::Box<TaskHandle>);
    PreloadHandle &operator=(PreloadHandle &&) &noexcept;

    ~PreloadHandle();
    void Cancel();
    std::string GetTaskId();
    bool IsFinish();
    PreloadState GetState();

private:
    TaskHandle *handle_;
};

struct PreloadOptions {
    std::vector<std::tuple<std::string, std::string>> headers;
    SslType sslType;
    std::string caPath;
};

class Preload {
public:
    Preload();
    static Preload *GetInstance();
    virtual ~Preload() = default;
    void Cancel(std::string const &url);
    void Remove(std::string const &url);
    bool Contains(std::string const &url);

    void SetRamCacheSize(uint64_t size);
    void SetFileCacheSize(uint64_t size);
    void SetDownloadInfoListSize(uint16_t size);
    static void SetFileCachePath(const std::string &path);

    void ClearMemoryCache();
    void ClearFileCache();

    std::shared_ptr<PreloadHandle> load(std::string const &url, std::unique_ptr<PreloadCallback>,
        std::unique_ptr<PreloadOptions> options = nullptr, bool update = false);

    std::optional<Data> fetch(std::string const &url);
    std::optional<CppDownloadInfo> GetDownloadInfo(std::string const &url);

private:
    const CacheDownloadService *agent_;
};

} // namespace OHOS::Request

#endif // REQUEST_PRE_DOWNLOAD_H