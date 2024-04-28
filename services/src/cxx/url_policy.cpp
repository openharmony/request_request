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

#include "url_policy.h"

namespace OHOS::Request {
static std::shared_ptr<UrlPolicy> singletonUrlPolicy = nullptr;
static std::once_flag singleFlag;

std::string UrlPolicy::DOMAIN_TYPE_HTTP_REQUEST = "httpRequest";
std::string UrlPolicy::DOMAIN_TYPE_WEB_SOCKET = "webSocket";
std::string UrlPolicy::DOMAIN_TYPE_DOWNLOAD = "download";
std::string UrlPolicy::DOMAIN_TYPE_UPLOAD = "upload";
std::string UrlPolicy::DOMAIN_TYPE_WEBVIEW = "webView";
int32_t UrlPolicy::RESULT_ACCEPT = 0;
int32_t UrlPolicy::RESULT_REJECT = 1;

std::shared_ptr<UrlPolicy> UrlPolicy::GetInstance()
{
    std::call_once(singleFlag, [&] { singletonUrlPolicy = std::make_shared<UrlPolicy>(); });
    return singletonUrlPolicy;
}

UrlPolicy::UrlPolicy()
{
    if (this->isInit) {
        REQUEST_HILOGD("Policy so is loaded");
        return;
    }
    if (this->libHandle) {
        REQUEST_HILOGD("lib handle is loaded");
        return;
    }
    const std::string LIB_API_POLICY_PATH = "/system/lib64/platformsdk/libapipolicy_client.z.so";
    this->libHandle = dlopen(LIB_API_POLICY_PATH.c_str(), RTLD_NOW);
    if (!this->libHandle) {
        const char *err = dlerror();
        REQUEST_HILOGE("Policy so dlopen failed: %{public}s", err ? err : "unknown");
        return;
    }
    REQUEST_HILOGI("Policy so success");
    this->isInit = true;
    this->checkUrlFunc = reinterpret_cast<CheckUrlFunc>(dlsym(this->libHandle, "CheckUrl"));
    if (this->checkUrlFunc == nullptr) {
        const char *err = dlerror();
        REQUEST_HILOGE("Policy so dlsym CheckUrl failed: %{public}s", err ? err : "unknown");
        return;
    }
}

UrlPolicy::~UrlPolicy()
{
    if (this->libHandle) {
        dlclose(this->libHandle);
        this->libHandle = nullptr;
        REQUEST_HILOGI("Policy so dalete");
    }
    this->isInit = false;
    this->checkUrlFunc = nullptr;
}

int32_t UrlPolicy::CheckUrlDomain(std::string app_id, std::string domain_type, std::string url)
{
    int32_t res = -1;
    if (!this->isInit || !this->libHandle) {
        REQUEST_HILOGE("Policy so handle is not init");
        return res;
    }
    if (!this->checkUrlFunc) {
        REQUEST_HILOGE("Policy checkUrlFunc is nullptr");
        return res;
    }
    res = this->checkUrlFunc(app_id, domain_type, url);
    return res;
}
} // namespace OHOS::Request

using namespace OHOS::Request;
int32_t PolicyCheckUrlDomain(CStringWrapper app_id, CStringWrapper domain_type, CStringWrapper url)
{
    std::string app(app_id.cStr, app_id.len);
    std::string domain(domain_type.cStr, domain_type.len);
    std::string urlStr(url.cStr, url.len);
    return UrlPolicy::GetInstance()->CheckUrlDomain(app, domain, urlStr);
}
