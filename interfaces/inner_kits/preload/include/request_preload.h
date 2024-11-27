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
#include <tuple>
#include <vector>

#include "cxx.h"

namespace OHOS::Request {
struct RustData;
struct TaskHandle;
struct CacheDownloadService;
struct CacheDownloadError;

enum class PreloadState {
    INIT,
    RUNNING,
    SUCCESS,
    FAIL,
    CANCEL,
};

class Data {
public:
    Data(rust::Box<RustData> data);
    Data &operator=(const Data &) = delete;

    ~Data();
    rust::Slice<const uint8_t> bytes();

private:
    RustData *_data;
};

enum ErrorKind {
    HTTP,
    IO,
    CACHE,
};

class PreloadError {
public:
    PreloadError(rust::Box<CacheDownloadError> error);
    PreloadError &operator=(const PreloadError &) = delete;
    ~PreloadError();

    int32_t GetCode() const;
    std::string GetMessage() const;
    ErrorKind GetErrorKind() const;

private:
    CacheDownloadError *_error;
};

struct PreloadCallback {
    std::function<void(const std::shared_ptr<Data> &&, const std::string &TaskId)> OnSuccess;
    std::function<void(const PreloadError &, const std::string &TaskId)> OnFail;
    std::function<void()> OnCancel;
    std::function<void(uint64_t current, uint64_t total)> OnProgress;
};

class PreloadHandle {
public:
    PreloadHandle(rust::Box<TaskHandle>);
    PreloadError &operator=(const PreloadError &) = delete;

    ~PreloadHandle();
    void Cancel();
    std::string GetTaskId();
    bool IsFinish();
    PreloadState GetState();

private:
    TaskHandle *_handle;
};

struct PreloadOptions {
    std::vector<std::tuple<std::string, std::string>> headers;
};

class Preload {
public:
    Preload();
    static Preload *GetInstance();
    virtual ~Preload() = default;
    void Cancel(std::string const &url);
    void Remove(std::string const &url);

    void SetRamCacheSize(uint64_t size);
    void SetFileCacheSize(uint64_t size);

    std::shared_ptr<PreloadHandle> load(
        std::string const &url, std::unique_ptr<PreloadCallback>, std::unique_ptr<PreloadOptions> options = nullptr);

private:
    const CacheDownloadService *_agent;
};

} // namespace OHOS::Request

#endif // REQUEST_PRE_DOWNLOAD_H