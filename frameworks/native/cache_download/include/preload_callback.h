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

#ifndef REQUEST_PRE_DOWNLOAD_CALLBACK_H
#define REQUEST_PRE_DOWNLOAD_CALLBACK_H
#include <memory>

#include "cxx.h"
#include "request_preload.h"

namespace OHOS::Request {

class PreloadCallbackWrapper {
public:
    PreloadCallbackWrapper(std::unique_ptr<PreloadCallback> &callback);
    ~PreloadCallbackWrapper() = default;
    PreloadCallbackWrapper(const PreloadCallbackWrapper &) = delete;
    PreloadCallbackWrapper &operator=(const PreloadCallbackWrapper &) = delete;

    void OnSuccess(const std::shared_ptr<Data> data, rust::str TaskId) const;
    void OnFail(rust::Box<CacheDownloadError> error, rust::Box<RustDownloadInfo> info, rust::str TaskId) const;
    void OnCancel() const;

private:
    std::function<void(const std::shared_ptr<Data> &&, const std::string &TaskId)> onSuccess_;
    std::function<void(const PreloadError &, const std::string &TaskId)> onFail_;
    std::function<void()> onCancel_;
};

class PreloadProgressCallbackWrapper {
public:
    PreloadProgressCallbackWrapper(std::unique_ptr<PreloadCallback> &callback);
    ~PreloadProgressCallbackWrapper() = default;
    PreloadProgressCallbackWrapper(const PreloadProgressCallbackWrapper &) = delete;
    PreloadProgressCallbackWrapper &operator=(const PreloadProgressCallbackWrapper &) = delete;

    void OnProgress(uint64_t current, uint64_t total) const;

private:
    std::function<void(uint64_t, uint64_t)> onProgress_;
};

std::shared_ptr<Data> SharedData(rust::Box<RustData> data);
std::unique_ptr<Data> UniqueData(rust::Box<RustData> data);
std::unique_ptr<CppDownloadInfo> UniqueInfo(rust::Box<RustDownloadInfo> info);
std::shared_ptr<PreloadHandle> ShareTaskHandle(rust::Box<TaskHandle> handle);
} // namespace OHOS::Request

#endif // REQUEST_PRE_DOWNLOAD_CALLBACK_H
