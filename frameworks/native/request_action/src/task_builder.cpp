/*
 * Copyright (c) 2025 Huawei Device Co., Ltd.
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

#include "task_builder.h"

#include <regex>

#include "application_context.h"
#include "log.h"
#include "network_security_config.h"

namespace OHOS::Request {

TaskBuilder &TaskBuilder::setAction(Action action)
{
    this->config.action = action;
    return *this;
}

TaskBuilder &TaskBuilder::setUrl(const std::string &url)
{
    this->config.url = url;
    return *this;
}

TaskBuilder &TaskBuilder::setTitle(const std::string &title)
{
    this->config.title = title;
    return *this;
}

TaskBuilder &TaskBuilder::setDescription(const std::string &description)
{
    this->config.description = description;
    return *this;
}

TaskBuilder &TaskBuilder::setMode(Mode mode)
{
    this->config.mode = mode;
    return *this;
}

TaskBuilder &TaskBuilder::setOverwrite(bool overwrite)
{
    this->config.overwrite = overwrite;
    return *this;
}

TaskBuilder &TaskBuilder::setMethod(const std::string &method)
{
    this->config.method = method;
    return *this;
}

TaskBuilder &TaskBuilder::setHeaders(const std::map<std::string, std::string> &headers)
{
    this->config.headers = headers;
    return *this;
}

TaskBuilder &TaskBuilder::setData(const std::string &data)
{
    this->config.data = data;
    return *this;
}

TaskBuilder &TaskBuilder::setData(const std::vector<FormItem> &data)
{
    this->config.forms = data;
    return *this;
}

TaskBuilder &TaskBuilder::setData(const std::vector<FileSpec> &data)
{
    this->config.files = data;
    return *this;
}

TaskBuilder &TaskBuilder::setSaveAs(const std::string &saveas)
{
    this->config.saveas = saveas;
    return *this;
}

TaskBuilder &TaskBuilder::setNetwork(Network network)
{
    this->config.network = network;
    return *this;
}

TaskBuilder &TaskBuilder::setMetered(bool metered)
{
    this->config.metered = metered;
    return *this;
}

TaskBuilder &TaskBuilder::setRoaming(bool roaming)
{
    this->config.roaming = roaming;
    return *this;
}

TaskBuilder &TaskBuilder::setRetry(bool retry)
{
    this->config.retry = retry;
    return *this;
}

TaskBuilder &TaskBuilder::setRedirect(bool redirect)
{
    this->config.redirect = redirect;
    return *this;
}

TaskBuilder &TaskBuilder::setProxy(const std::string &proxy)
{
    this->config.proxy = proxy;
    return *this;
}

TaskBuilder &TaskBuilder::setIndex(uint32_t index)
{
    this->config.index = index;
    return *this;
}

TaskBuilder &TaskBuilder::setBegins(int begins)
{
    this->config.begins = begins;
    return *this;
}

TaskBuilder &TaskBuilder::setEnds(int ends)
{
    this->config.ends = ends;
    return *this;
}

TaskBuilder &TaskBuilder::setGauge(bool gauge)
{
    this->config.gauge = gauge;
    return *this;
}

TaskBuilder &TaskBuilder::setPrecise(bool precise)
{
    this->config.precise = precise;
    return *this;
}

TaskBuilder &TaskBuilder::setToken(const std::string &token)
{
    this->config.token = token;
    return *this;
}

TaskBuilder &TaskBuilder::setPriority(uint32_t priority)
{
    this->config.priority = priority;
    return *this;
}

TaskBuilder &TaskBuilder::setExtras(const std::map<std::string, std::string> &extras)
{
    this->config.extras = extras;
    return *this;
}

std::pair<Config, ExceptionErrorCode> TaskBuilder::build()
{
    if (!this->checkAction()) {
        return { this->config, ExceptionErrorCode::E_PARAMETER_CHECK };
    }
    if (!this->checkUrl()) {
        return { this->config, ExceptionErrorCode::E_PARAMETER_CHECK };
    }
    if (!this->checkData()) {
        return { this->config, ExceptionErrorCode::E_PARAMETER_CHECK };
    }
    if (!this->checkIndex()) {
        return { this->config, ExceptionErrorCode::E_PARAMETER_CHECK };
    }
    if (!this->checkProxy()) {
        return { this->config, ExceptionErrorCode::E_PARAMETER_CHECK };
    }
    if (!this->checkTitle()) {
        return { this->config, ExceptionErrorCode::E_PARAMETER_CHECK };
    }
    if (!this->checkToken()) {
        return { this->config, ExceptionErrorCode::E_PARAMETER_CHECK };
    }
    if (!this->checkDescription()) {
        return { this->config, ExceptionErrorCode::E_PARAMETER_CHECK };
    }
    if (!this->checkSaveas()) {
        return { this->config, ExceptionErrorCode::E_PARAMETER_CHECK };
    }
    if (!this->checkBundle()) {
        return { this->config, ExceptionErrorCode::E_PARAMETER_CHECK };
    }
    this->checkCertsPath();
    this->checkCertificatePins();
    this->checkMethod();
    this->checkOtherConfig();

    return { this->config, ExceptionErrorCode::E_OK };
}

bool TaskBuilder::checkAction()
{
    if (this->config.action != Action::DOWNLOAD && this->config.action != Action::UPLOAD) {
        REQUEST_HILOGE("Must be UPLOAD or DOWNLOAD");
        return false;
    }
    return true;
}
bool TaskBuilder::checkUrl()
{
    constexpr uint32_t URL_MAXIMUM = 8192;
    if (this->config.url.size() > URL_MAXIMUM) {
        REQUEST_HILOGE("The URL exceeds the maximum length of 8192");
        return false;
    }
    if (!regex_match(this->config.url, std::regex("^http(s)?:\\/\\/.+"))) {
        REQUEST_HILOGE("ParseUrl error");
        return false;
    }
    return true;
}

void TaskBuilder::checkCertsPath()
{
    typedef std::string::const_iterator iter_t;

    iter_t urlEnd = this->config.url.end();
    iter_t protocolStart = this->config.url.cbegin();
    iter_t protocolEnd = std::find(protocolStart, urlEnd, ':');
    std::string protocol = std::string(protocolStart, protocolEnd);
    if (protocol != "https") {
        REQUEST_HILOGD("Using Http");
        return;
    }
    if (protocolEnd != urlEnd) {
        std::string afterProtocol = &*(protocolEnd);
        // 3 is the num of ://
        if ((afterProtocol.length() > 3) && (afterProtocol.substr(0, 3) == "://")) {
            // 3 means go beyound :// in protocolEnd
            protocolEnd += 3;
        } else {
            protocolEnd = this->config.url.cbegin();
        }
    } else {
        protocolEnd = this->config.url.cbegin();
    }
    iter_t hostStart = protocolEnd;
    iter_t pathStart = std::find(hostStart, urlEnd, '/');
    iter_t queryStart = std::find(this->config.url.cbegin(), urlEnd, '?');
    iter_t hostEnd = std::find(protocolEnd, (pathStart != urlEnd) ? pathStart : queryStart, ':');
    std::string hostname = std::string(hostStart, hostEnd);
    REQUEST_HILOGD("Hostname is %{public}s", hostname.c_str());
    NetManagerStandard::NetworkSecurityConfig::GetInstance().
        GetTrustAnchorsForHostName(hostname, this->config.certsPath);
}

bool TaskBuilder::checkData()
{
    if (this->config.action == Action::UPLOAD) {
        if (this->config.files.empty()) {
            REQUEST_HILOGE("Missing mandatory parameters, files is empty");
            return false;
        }
        for (auto &file : this->config.files) {
            if (file.uri.empty()) {
                REQUEST_HILOGE("Missing mandatory parameters, uri is empty");
                return false;
            }
        }
    }
    return true;
}

bool TaskBuilder::checkIndex()
{
    if (this->config.action == Action::DOWNLOAD) {
        this->config.index = 0;
    } else if (this->config.files.size() <= config.index) {
        REQUEST_HILOGE("files.size is %{public}zu, index is %{public}d", config.files.size(), config.index);
        return false;
    }

    return true;
}

bool TaskBuilder::checkProxy()
{
    constexpr uint32_t PROXY_MAXIMUM = 512;
    if (this->config.proxy.empty()) {
        return true;
    }
    if (this->config.proxy.size() > PROXY_MAXIMUM) {
        REQUEST_HILOGE("The proxy exceeds the maximum length of 512");
        return false;
    }
    if (!regex_match(this->config.proxy, std::regex("^http:\\/\\/.+:\\d{1,5}$"))) {
        REQUEST_HILOGE("ParseProxy error");
        return false;
    }
    return true;
}

bool TaskBuilder::checkTitle()
{
    static constexpr uint32_t TITLE_MAXIMUM = 256;
    if (config.title.size() > TITLE_MAXIMUM) {
        REQUEST_HILOGE("Parameter verification failed, the length of config title exceeds 256");
        return false;
    }
    if (this->config.title.empty()) {
        this->config.title = this->config.action == Action::UPLOAD ? "upload" : "download";
    }
    return true;
}

bool TaskBuilder::checkToken()
{
    constexpr uint32_t TOKEN_MAX_BYTES = 2048;
    constexpr uint32_t TOKEN_MIN_BYTES = 8;
    if (this->config.token.compare("null") == 0) {
        return true;
    }
    if ((this->config.token.size() < TOKEN_MIN_BYTES || this->config.token.size() > TOKEN_MAX_BYTES)) {
        REQUEST_HILOGE("token error");
        return false;
    }
    return true;
}

bool TaskBuilder::checkDescription()
{
    constexpr uint32_t DESCRIPTION_MAXIMUM = 1024;
    if (this->config.description.size() > DESCRIPTION_MAXIMUM) {
        REQUEST_HILOGE("description error");
        return false;
    }
    return true;
}

bool TaskBuilder::checkSaveas()
{
    if (this->config.action != Action::DOWNLOAD) {
        this->config.saveas = "";
        return true;
    }

    if (!this->config.saveas.empty()) {
        this->config.saveas.erase(0, this->config.saveas.find_first_not_of(" "));
        this->config.saveas.erase(this->config.saveas.find_last_not_of(" ") + 1);
    }

    if (this->config.saveas.empty() || this->config.saveas == "./") {
        std::size_t position = this->config.url.find_last_of("/");
        if (position == std::string::npos || position + 1 >= this->config.url.size()) {
            REQUEST_HILOGE("Parameter verification failed, config.saveas parse error");
            return false;
        }
        this->config.saveas = std::string(this->config.url, position + 1);
        return true;
    }
    if (this->config.saveas.size() == 0 || this->config.saveas[this->config.saveas.size() - 1] == '/') {
        REQUEST_HILOGE("Parameter verification failed, config.saveas parse error");
        return false;
    }
    return true;
}

std::string GetHostnameFromURL(const std::string &url)
{
    if (url.empty()) {
        return "";
    }
    std::string delimiter = "://";
    std::string tempUrl = url;
    std::replace(tempUrl.begin(), tempUrl.end(), '\\', '/');
    size_t posStart = tempUrl.find(delimiter);
    if (posStart != std::string::npos) {
        posStart += delimiter.length();
    } else {
        posStart = 0;
    }
    size_t notSlash = tempUrl.find_first_not_of('/', posStart);
    if (notSlash != std::string::npos) {
        posStart = notSlash;
    }
    size_t posEnd =
        std::min({ tempUrl.find(':', posStart), tempUrl.find('/', posStart), tempUrl.find('?', posStart) });
    if (posEnd != std::string::npos) {
        return tempUrl.substr(posStart, posEnd - posStart);
    }
    return tempUrl.substr(posStart);
}

void TaskBuilder::checkCertificatePins()
{
    auto hostname = GetHostnameFromURL(this->config.url);
    if (OHOS::NetManagerStandard::NetworkSecurityConfig::GetInstance().IsPinOpenMode(hostname)) {
        REQUEST_HILOGI("Pins is openMode");
        return;
    }
    auto ret = OHOS::NetManagerStandard::NetworkSecurityConfig::GetInstance().GetPinSetForHostName(
        hostname, this->config.certificatePins);
    if (ret != 0 || this->config.certificatePins.empty()) {
        REQUEST_HILOGD("Get No pin set by hostname");
    }
}

void TaskBuilder::checkMethod()
{
    if (!this->config.method.empty()) {
        transform(this->config.method.begin(), this->config.method.end(), this->config.method.begin(), ::toupper);
        if (this->config.action == Action::UPLOAD) {
            if ((this->config.method == "POST" || this->config.method == "PUT")) {
                return;
            }
        }
        if (this->config.action == Action::DOWNLOAD) {
            if (this->config.method == "POST" || this->config.method == "GET") {
                return;
            }
        }
    }
    this->config.method = this->config.action == Action::UPLOAD ? "PUT" : "GET";
}

void TaskBuilder::checkOtherConfig()
{
    this->config.version = Version::API10;
    if (this->config.begins < 0) {
        this->config.begins = 0;
    }
    if (this->config.mode == Mode::BACKGROUND) {
        this->config.background = true;
    }
}

bool TaskBuilder::checkBundle()
{
    auto context = AbilityRuntime::Context::GetApplicationContext();
    if (context == nullptr) {
        REQUEST_HILOGE("AppContext is null.");
        return false;
    }
    auto applicationInfo = context->GetApplicationInfo();
    if (applicationInfo == nullptr) {
        REQUEST_HILOGE("AppInfo is null.");
        return false;
    }
    this->config.bundleType = static_cast<u_int32_t>(applicationInfo->bundleType);
    REQUEST_HILOGD("config.bundleType is %{public}d", config.bundleType);
    this->config.bundleName = context->GetBundleName();
    REQUEST_HILOGD("config.bundleName is %{public}s", config.bundleName.c_str());
    return true;
}

} // namespace OHOS::Request
