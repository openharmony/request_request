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

#include "preload_callback.h"

#include <memory>

#include "cxx.h"
#include "request_preload.h"
namespace OHOS::Request {

PreloadCallbackWrapper::PreloadCallbackWrapper(std::unique_ptr<PreloadCallback> &callback)
{
    if (callback != nullptr) {
        this->onSuccess_ = callback->OnSuccess;
        this->onCancel_ = callback->OnCancel;
        this->onFail_ = callback->OnFail;
    }
}

void PreloadCallbackWrapper::OnSuccess(const std::shared_ptr<Data> data, rust::str taskId) const
{
    if (this->onSuccess_ != nullptr) {
        this->onSuccess_(std::move(data), std::string(taskId));
    }
}

void PreloadCallbackWrapper::OnFail(
    rust::Box<CacheDownloadError> error, rust::Box<RustDownloadInfo> info, rust::str taskId) const
{
    if (this->onFail_ != nullptr) {
        PreloadError preloadError(std::move(error), std::move(info));
        this->onFail_(preloadError, std::string(taskId));
    }
}

void PreloadCallbackWrapper::OnCancel() const
{
    if (this->onCancel_ != nullptr) {
        this->onCancel_();
    }
}

PreloadProgressCallbackWrapper::PreloadProgressCallbackWrapper(std::unique_ptr<PreloadCallback> &callback)
{
    if (callback != nullptr) {
        this->onProgress_ = callback->OnProgress;
    }
}

void PreloadProgressCallbackWrapper::OnProgress(uint64_t current, uint64_t total) const
{
    if (this->onProgress_ != nullptr) {
        this->onProgress_(current, total);
    }
}

std::shared_ptr<Data> SharedData(rust::Box<RustData> data)
{
    return std::make_shared<Data>(std::move(data));
}

std::unique_ptr<Data> UniqueData(rust::Box<RustData> data)
{
    return std::make_unique<Data>(std::move(data));
}

std::unique_ptr<CppDownloadInfo> UniqueInfo(rust::Box<RustDownloadInfo> info)
{
    return std::make_unique<CppDownloadInfo>(std::move(info));
}

std::shared_ptr<PreloadHandle> ShareTaskHandle(rust::Box<TaskHandle> handle)
{
    return std::make_shared<PreloadHandle>(std::move(handle));
}

} // namespace OHOS::Request