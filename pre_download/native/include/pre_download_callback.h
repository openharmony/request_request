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
#include "request_pre_download.h"

namespace OHOS::Request {

class PreloadCallbackWrapper {
public:
    PreloadCallbackWrapper(std::unique_ptr<PreloadCallback> callback);
    ~PreloadCallbackWrapper() = default;

    void OnSuccess(const std::shared_ptr<Data> data,  rust::str TaskId) const;
    void OnFail(rust::Box<DownloadError> error,  rust::str TaskId) const;
    void OnCancel() const;
    void OnProgress(uint64_t current, uint64_t total) const;

private:
    std::unique_ptr<PreloadCallback> _callback;
};

std::shared_ptr<Data> BuildSharedData(rust::Box<RustData> data);

} // namespace OHOS::Request

#endif // REQUEST_PRE_DOWNLOAD_CALLBACK_H
