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

#include <memory>

#include "cxx.h"

namespace OHOS::Request {
struct DownloadAgent;

class PreDownloadCallback {
public:
    PreDownloadCallback() = default;
    virtual ~PreDownloadCallback();
    virtual void OnSuccess() const = 0;
    virtual void OnFail() const = 0;
    virtual void OnCancel() const = 0;
};

class PreDownloadAgent {
public:
    PreDownloadAgent();
    void preDownload(std::string url, std::unique_ptr<PreDownloadCallback> callback) const;

private:
    DownloadAgent *_agent;
};

} // namespace OHOS::Request

#endif // REQUEST_PRE_DOWNLOAD_H