/*
* Copyright (c) 2024 Huawei Device Co., Ltd.
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

#ifndef REQUEST_URL_POLICY_H
#define REQUEST_URL_POLICY_H

#include <dlfcn.h>

#include <iostream>
#include <memory>
#include <mutex>

#include "c_string_wrapper.h"
#include "log.h"

namespace OHOS::Request {
class UrlPolicy {
public:
    static std::string DOMAIN_TYPE_HTTP_REQUEST;
    static std::string DOMAIN_TYPE_WEB_SOCKET;
    static std::string DOMAIN_TYPE_DOWNLOAD;
    static std::string DOMAIN_TYPE_UPLOAD;
    static std::string DOMAIN_TYPE_WEBVIEW;
    static int32_t RESULT_ACCEPT;
    static int32_t RESULT_REJECT;

    static std::shared_ptr<UrlPolicy> GetInstance();

    int32_t CheckUrlDomain(std::string app_id, std::string domain_type, std::string url);
    UrlPolicy();
    ~UrlPolicy();

private:
    bool isInit = false;
    void *libHandle = nullptr;
    using CheckUrlFunc = int32_t (*)(std::string, std::string, std::string);
    CheckUrlFunc checkUrlFunc = nullptr;
};
} // namespace OHOS::Request

#ifdef __cplusplus
extern "C" {
#endif

int32_t PolicyCheckUrlDomain(CStringWrapper app_id, CStringWrapper domain_type, CStringWrapper url);

#ifdef __cplusplus
}
#endif

#endif // REQUEST_URL_POLICY_H