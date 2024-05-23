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

#include "cj_initialize.h"

#include <regex>
#include <sys/stat.h>
#include <algorithm>
#include <cstring>
#include <filesystem>
#include <fcntl.h>
#include <fstream>
#include "securec.h"
#include "constant.h"
#include "js_common.h"
#include "cj_request_log.h"
#include "request_manager.h"
#include "cj_request_common.h"
#include "net_conn_client.h"
#include "cj_request_task.h"

namespace OHOS::CJSystemapi::Request {

using OHOS::Request::Action;
using OHOS::Request::Version;
using OHOS::Request::FileSpec;
using OHOS::Request::FormItem;
using OHOS::Request::ExceptionErrorCode;
using OHOS::AbilityRuntime::Context;

static constexpr uint32_t TOKEN_MAX_BYTES = 2048;
static constexpr uint32_t TOKEN_MIN_BYTES = 8;
static constexpr uint32_t URL_MAXIMUM = 2048;
static constexpr uint32_t TITLE_MAXIMUM = 256;
static constexpr uint32_t DESCRIPTION_MAXIMUM = 1024;

static constexpr uint32_t FILE_PERMISSION = 0644;

static const std::string AREA1 = "el1";
static const std::string AREA2 = "el2";

ExceptionError CJInitialize::ParseBundleName(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context,
    std::string &bundleName)
{
    if (context->GetApplicationInfo() == nullptr) {
        return {
            .code = ExceptionErrorCode::E_OTHER,
            .errInfo = "ApplicationInfo is null"
        };
    }

    bundleName = context->GetBundleName();

    return {
        .code = ExceptionErrorCode::E_OK
    };
}

bool CJInitialize::ParseUrl(std::string &url)
{
    if (url.size() > URL_MAXIMUM) {
        REQUEST_HILOGE("The URL exceeds the maximum length of 2048");
        return false;
    }
    if (!std::regex_match(url, std::regex("^http(s)?:\\/\\/.+"))) {
        REQUEST_HILOGE("ParseUrl error");
        return false;
    }

    return true;
}

bool CJInitialize::ParseCertsPath(std::string &url, std::vector<std::string> &certsPath)
{
    if (url.size() > URL_MAXIMUM) {
        REQUEST_HILOGE("The URL exceeds the maximum length of 2048");
        return false;
    }
    if (!regex_match(url, std::regex("^http(s)?:\\/\\/.+"))) {
        REQUEST_HILOGE("ParseUrl error");
        return false;
    }

    typedef std::string::const_iterator iter_t;
    iter_t urlEnd = url.end();
    iter_t protocolStart = url.cbegin();
    iter_t protocolEnd = std::find(protocolStart, urlEnd, ':');
    std::string protocol = std::string(protocolStart, protocolEnd);
    if (protocol != "https") {
        REQUEST_HILOGD("Using Http");
        return true;
    }
    if (protocolEnd != urlEnd) {
        std::string afterProtocol = &*(protocolEnd);
        // 3 is the num of ://
        if ((afterProtocol.length() > 3) && (afterProtocol.substr(0, 3) == "://")) {
            // 3 means go beyound :// in protocolEnd
            protocolEnd += 3;
        } else {
            protocolEnd = url.cbegin();
        }
    } else {
        protocolEnd = url.cbegin();
    }
    iter_t hostStart = protocolEnd;
    iter_t pathStart = std::find(hostStart, urlEnd, '/');
    iter_t queryStart = std::find(url.cbegin(), urlEnd, '?');
    iter_t hostEnd = std::find(protocolEnd, (pathStart != urlEnd) ? pathStart : queryStart, ':');
    std::string hostname = std::string(hostStart, hostEnd);
    REQUEST_HILOGD("Hostname is %{public}s", hostname.c_str());
    NetManagerStandard::NetConnClient::GetInstance().GetTrustAnchorsForHostName(hostname, certsPath);

    return true;
}

bool CJInitialize::Convert2FileSpec(const CFileSpec *cFile, const char *name, FileSpec &file)
{
    file.name = name;

    if (cFile->path == nullptr) {
        return false;
    }
    file.uri = cFile->path;
    if (file.uri.empty()) {
        return false;
    }
    if (cFile->filename != nullptr) {
        file.filename = cFile->filename;
    }

    if (cFile->mimeType != nullptr) {
        file.type = cFile->mimeType;
    }

    return true;
}

bool CJInitialize::Convert2FileSpecs(const CFileSpecArr *cFiles, const char *name, std::vector<FileSpec> &files)
{
    for (int i = 0; i < cFiles->size; ++i) {
        FileSpec file;
        if (!Convert2FileSpec(&cFiles->head[i], name, file)) {
            return false;
        }
        files.push_back(file);
    }
    return true;
}

bool CJInitialize::ParseFormItems(const CFormItemArr *cForms, std::vector<FormItem> &forms,
    std::vector<FileSpec> &files)
{
    for (int i = 0; i < cForms->size; ++i) {
        CFormItem *cForm = &cForms->head[i];
        if (cForm->value.str != nullptr) {
            FormItem form;
            form.name = cForm->name;
            form.value = cForm->value.str;
            forms.push_back(form);
        } else if (cForm->value.file.path != nullptr) {
            FileSpec file;
            if (!Convert2FileSpec(&cForm->value.file, cForm->name, file)) {
                REQUEST_HILOGE("Convert2FileSpec failed");
                return false;
            }
            files.push_back(file);
        } else if (cForm->value.files.size > 0) {
            if (!Convert2FileSpecs(&cForm->value.files, cForm->name, files)) {
                return false;
            }
        } else {
            REQUEST_HILOGE("value type is error");
            return false;
        }
    }

    return true;
}

bool CJInitialize::ParseData(const CConfig *config, Config &out)
{
    REQUEST_HILOGD("formItems.size is %{public}" PRIi64 ", data.str is %{public}p", config->data.formItems.size,
        config->data.str);
    if (config->data.str == nullptr && config->data.formItems.size <= 0) {
        return true;
    }

    if (out.action == Action::UPLOAD && config->data.formItems.size > 0) {
        return ParseFormItems(&config->data.formItems, out.forms, out.files);
    } else if (out.action == Action::DOWNLOAD && config->data.str != nullptr) {
        out.data = config->data.str;
    } else {
        REQUEST_HILOGE("data type is error");
        return false;
    }

    return true;
}

bool CJInitialize::ParseIndex(Config &config)
{
    if (config.action == Action::DOWNLOAD) {
        config.index = 0;
        return true;
    }
    if (config.files.size() <= config.index) {
        REQUEST_HILOGE("files.size is %{public}zu, index is %{public}d", config.files.size(), config.index);
        return false;
    }
    return true;
}

int64_t CJInitialize::ParseBegins(int64_t &begins)
{
    return begins >= 0 ? begins : 0;
}

bool CJInitialize::ParseTitle(Config &config)
{
    if (config.title.size() > TITLE_MAXIMUM) {
        return false;
    }

    if (config.title.empty()) {
        config.title = config.action == Action::UPLOAD ? "upload" : "download";
    }

    return true;
}

bool CJInitialize::ParseToken(Config &config)
{
    if (config.token.empty()) {
        config.token = "null";
        return true;
    }
    size_t len = config.token.length();
    if (len < TOKEN_MIN_BYTES || len > TOKEN_MAX_BYTES) {
        return false;
    }

    config.token = SHA256(config.token.c_str(), len);
    return true;
}

bool CJInitialize::ParseDescription(std::string &description)
{
    return description.size() <= DESCRIPTION_MAXIMUM;
}

bool CJInitialize::ParseSaveas(Config &config)
{
    if (config.action != Action::DOWNLOAD) {
        config.saveas = "";
        return true;
    }

    std::string temp = config.saveas;
    if (temp.empty() || temp == "./") {
        return InterceptData("/", config.url, config.saveas);
    }
    temp = std::string(temp, 0, temp.find_last_not_of(' ') + 1);
    if (temp[temp.size() - 1] == '/') {
        return false;
    }
    config.saveas = temp;
    return true;
}

void CJInitialize::ParseMethod(Config &config)
{
    std::string method = config.method;
    config.method = config.action == Action::UPLOAD ? "PUT" : "GET";
    if (!method.empty()) {
        transform(method.begin(), method.end(), method.begin(), ::toupper);
        if (config.action == Action::UPLOAD && (method == "POST" || method == "PUT")) {
            config.method = method;
        }
        if (config.action == Action::DOWNLOAD && (method == "POST" || method == "GET")) {
            config.method = method;
        }
    }
}

void CJInitialize::ParseNetwork(Network &network)
{
    if (network != Network::ANY && network != Network::WIFI && network != Network::CELLULAR) {
        network = Network::ANY;
    }
}

void CJInitialize::ParseBackGround(Mode mode, bool &background)
{
    background = mode == Mode::BACKGROUND;
}

void CJInitialize::StringSplit(const std::string &str, const char delim, std::vector<std::string> &elems)
{
    std::stringstream stream(str);
    std::string item;
    while (std::getline(stream, item, delim)) {
        if (!item.empty()) {
            elems.push_back(item);
        }
    }
    return;
}

bool CJInitialize::GetBaseDir(std::string &baseDir)
{
    auto context = OHOS::AbilityRuntime::Context::GetApplicationContext();
    if (context == nullptr) {
        REQUEST_HILOGE("AppContext is null.");
        return false;
    }

    baseDir = context->GetBaseDir();
    if (baseDir.empty()) {
        REQUEST_HILOGE("Base dir not found.");
        return false;
    }
    return true;
}

bool CJInitialize::CheckPathBaseDir(const std::string &filepath, std::string &baseDir)
{
    if (!CJInitialize::GetBaseDir(baseDir)) {
        return false;
    }
    if (filepath.find(baseDir) == 0) {
        return true;
    }
    // check baseDir replaced with el2
    if (baseDir.find(AREA1) != std::string::npos) {
        baseDir = baseDir.replace(baseDir.find(AREA1), AREA1.length(), AREA2);
        if (filepath.find(baseDir) == 0) {
            return true;
        }
        REQUEST_HILOGE("File dir not include base dir: %{public}s", baseDir.c_str());
        return false;
    }
    // check baseDir replaced with el1
    if (baseDir.find(AREA2) != std::string::npos) {
        baseDir = baseDir.replace(baseDir.find(AREA2), AREA2.length(), AREA1);
        if (filepath.find(baseDir) == 0) {
            return true;
        }
        REQUEST_HILOGE("File dir not include base dir: %{public}s", baseDir.c_str());
        return false;
    }
    return false;
}

bool CJInitialize::CreateDirs(const std::vector<std::string> &pathDirs)
{
    std::string path;
    for (auto elem : pathDirs) {
        path += "/" + elem;
        std::error_code err;
        if (std::filesystem::exists(path, err)) {
            continue;
        }
        err.clear();
        // create_directory noexcept.
        if (!std::filesystem::create_directory(path, err)) {
            REQUEST_HILOGE("Create Dir Err: %{public}d, %{public}s", err.value(), err.message().c_str());
            return false;
        }
    }
    return true;
}

bool CJInitialize::CheckDownloadFilePath(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config,
    std::string &errInfo)
{
    std::string path = config.saveas;
    if (!StandardizePath(context, config, path)) {
        REQUEST_HILOGE("StandardizePath Err: %{public}s", path.c_str());
        errInfo = "this is fail saveas path";
        return false;
    };
    std::string normalPath;
    std::vector<std::string> pathVec;
    if (!WholeToNormal(path, normalPath, pathVec) || pathVec.empty()) {
        REQUEST_HILOGE("WholeToNormal Err: %{public}s", path.c_str());
        errInfo = "this is fail saveas path";
        return false;
    };
    std::string baseDir;
    if (!CheckPathBaseDir(normalPath, baseDir)) {
        REQUEST_HILOGE("CheckPathBaseDir Err: %{public}s", normalPath.c_str());
        errInfo = "this is fail saveas path";
        return false;
    };
    // pop filename.
    pathVec.pop_back();
    if (!CreateDirs(pathVec)) {
        REQUEST_HILOGE("CreateDirs Err: %{public}s", normalPath.c_str());
        errInfo = "this is fail saveas path";
        return false;
    }
    config.saveas = normalPath;
    return true;
}

bool CJInitialize::FileToWhole(
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config, std::string &path)
{
    std::string bundleName = path.substr(0, path.find("/"));
    if (bundleName != config.bundleName) {
        REQUEST_HILOGE("path bundleName error.");
        return false;
    }
    path.erase(0, bundleName.size());
    return true;
}

bool CJInitialize::CacheToWhole(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, std::string &path)
{
    std::string cache = context->GetCacheDir();
    if (cache.empty()) {
        REQUEST_HILOGE("GetCacheDir error.");
        return false;
    }
    path = cache + "/" + path;
    return true;
}

bool CJInitialize::StandardizePath(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config,
    std::string &path)
{
    std::string WHOLE_PREFIX = "/";
    std::string FILE_PREFIX = "file://";
    std::string INTERNAL_PREFIX = "internal://cache/";
    std::string CURRENT_PREFIX = "./";

    if (path.find(WHOLE_PREFIX) == 0) {
        return true;
    }
    if (path.find(FILE_PREFIX) == 0) {
        path.erase(0, FILE_PREFIX.size());
        return FileToWhole(context, config, path);
    }
    if (path.find(INTERNAL_PREFIX) == 0) {
        path.erase(0, INTERNAL_PREFIX.size());
        return CacheToWhole(context, path);
    }
    if (path.find(CURRENT_PREFIX) == 0) {
        path.erase(0, CURRENT_PREFIX.size());
        return CacheToWhole(context, path);
    }
    return CacheToWhole(context, path);
}

bool CJInitialize::PathVecToNormal(const std::vector<std::string> &in, std::vector<std::string> &out)
{
    for (auto elem : in) {
        if (elem == "..") {
            if (out.size() > 0) {
                out.pop_back();
            } else {
                return false;
            }
        } else {
            out.push_back(elem);
        }
    }
    return true;
}

bool CJInitialize::WholeToNormal(const std::string &wholePath, std::string &normalPath, std::vector<std::string> &out)
{
    std::vector<std::string> elems;
    StringSplit(wholePath, '/', elems);
    if (!PathVecToNormal(elems, out)) {
        return false;
    }
    for (auto elem : out) {
        normalPath += "/" + elem;
    }
    return true;
}

ExceptionError CJInitialize::UploadBodyFileProc(std::string &fileName, Config &config)
{
    int32_t bodyFd = open(fileName.c_str(), O_TRUNC | O_RDWR);
    if (bodyFd < 0) {
        bodyFd = open(fileName.c_str(), O_CREAT | O_RDWR, FILE_PERMISSION);
        if (bodyFd < 0) {
            return {
                .code = ExceptionErrorCode::E_FILE_IO,
                .errInfo = "Failed to open file errno " + std::to_string(errno)
            };
        }
    }

    if (bodyFd >= 0) {
        chmod(fileName.c_str(), S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH | S_IWOTH);
    }

    config.bodyFds.push_back(bodyFd);
    config.bodyFileNames.push_back(fileName);

    return {
        .code = ExceptionErrorCode::E_OK
    };
}

ExceptionError CJInitialize::CheckUploadBodyFiles(Config &config, const std::string &filePath)
{
    size_t len = config.files.size();
    for (size_t i = 0; i < len; i++) {
        if (filePath.empty()) {
            REQUEST_HILOGE("internal to cache error");
            return {
                .code = ExceptionErrorCode::E_PARAMETER_CHECK,
                .errInfo = "IsPathValid error empty path"
            };
        }
        auto now = std::chrono::high_resolution_clock::now();
        auto timestamp = std::chrono::duration_cast<std::chrono::nanoseconds>(now.time_since_epoch()).count();
        std::string fileName = filePath + "/tmp_body_" + std::to_string(i) + "_" + std::to_string(timestamp);
        REQUEST_HILOGD("Create upload body file, %{public}s", fileName.c_str());
        if (!IsPathValid(fileName)) {
            REQUEST_HILOGE("IsPathValid error %{public}s", fileName.c_str());
            return {
                .code = ExceptionErrorCode::E_PARAMETER_CHECK,
                .errInfo = "IsPathValid error fail path"
            };
        }
        ExceptionError ret = UploadBodyFileProc(fileName, config);
        if (ret.code != ExceptionErrorCode::E_OK) {
            return ret;
        }
    }
    return {
        .code = ExceptionErrorCode::E_OK
    };
}

bool CJInitialize::InterceptData(const std::string &str, const std::string &in, std::string &out)
{
    std::string tmpStr = std::string(in, 0, in.find_last_not_of(' ') + 1);
    std::size_t position = tmpStr.find_last_of(str);
    // when the str at last index, will error.
    if (position == std::string::npos || position >= tmpStr.size() - 1) {
        return false;
    }
    out = std::string(tmpStr, position + 1);
    return true;
}

ExceptionError CJInitialize::GetFD(const std::string &path, const Config &config, int32_t &fd)
{
    ExceptionError error = { .code = ExceptionErrorCode::E_OK };
    fd = config.action == Action::UPLOAD ? open(path.c_str(), O_RDONLY) : open(path.c_str(), O_TRUNC | O_RDWR);
    if (fd >= 0) {
        REQUEST_HILOGD("File already exists");
        if (config.action == Action::UPLOAD) {
            chmod(path.c_str(), S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH);
            return error;
        } else {
            chmod(path.c_str(), S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH | S_IWOTH);
        }

        if (config.overwrite) {
            return error;
        }
        if (!config.firstInit) {
            REQUEST_HILOGD("CJTask config is not firstInit");
            return error;
        }
        ExceptionErrorCode code = ExceptionErrorCode::E_FILE_IO;
        return {
            .code = code,
            .errInfo = "Download File already exists"
        };
    } else {
        if (config.action == Action::UPLOAD) {
            ExceptionErrorCode code = ExceptionErrorCode::E_FILE_IO;
            return {
                .code = code,
                .errInfo = "Failed to open file errno " + std::to_string(errno)
            };
        }
        fd = open(path.c_str(), O_CREAT | O_RDWR, FILE_PERMISSION);
        if (fd < 0) {
            return {
                .code = ExceptionErrorCode::E_FILE_IO,
                .errInfo = "Failed to open file errno " +std::to_string(errno)
            };
        }
        chmod(path.c_str(), S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH | S_IWOTH);
    }
    return error;
}

bool CJInitialize::GetInternalPath(const std::string &fileUri,
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config, std::string &filePath)
{
    if (config.action == Action::DOWNLOAD && fileUri.find('/') == 0) {
        filePath = fileUri;
        return true;
    }
    std::string fileName;
    std::string pattern = "./";
    size_t pos = fileUri.find(pattern);
    if (pos != 0) {
        fileName = fileUri;
    } else {
        fileName = fileUri.substr(pattern.size(), fileUri.size());
    }
    if (fileName.empty()) {
        return false;
    }
    filePath = context->GetCacheDir();
    if (filePath.empty()) {
        REQUEST_HILOGE("internal to cache error");
        return false;
    }

    filePath += "/" + fileName;
    if (!IsPathValid(filePath)) {
        REQUEST_HILOGE("IsPathValid error %{public}s", filePath.c_str());
        return false;
    }
    return true;
}

ExceptionError CJInitialize::CheckFileSpec(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context,
    Config &config)
{
    ExceptionError err = { .code = ExceptionErrorCode::E_OK };
    for (auto &file : config.files) {
        std::string path;
        if (!GetInternalPath(file.uri, context, config, path)) {
            return {
                .code = ExceptionErrorCode::E_PARAMETER_CHECK,
                .errInfo = "this is fail path"
            };
        }
        file.uri = path;
        if (file.filename.empty()) {
            InterceptData("/", file.uri, file.filename);
        }
        if (file.type.empty()) {
            InterceptData(".", file.filename, file.type);
        }
        if (file.name.empty()) {
            file.name = "file";
        }

        if (!CJTask::SetPathPermission(file.uri)) {
            return {
                .code = ExceptionErrorCode::E_FILE_IO,
                .errInfo = "set path permission fail"
            };
        }

        err = GetFD(path, config, file.fd);
        if (err.code != ExceptionErrorCode::E_OK) {
            return err;
        }
    }
    return err;
}

ExceptionError CJInitialize::CheckFilePath(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context,
    Config &config)
{
    ExceptionError err = { .code = ExceptionErrorCode::E_OK };
    if (config.action == Action::DOWNLOAD) {
        if (!CheckDownloadFilePath(context, config, err.errInfo)) {
            err.code = ExceptionErrorCode::E_PARAMETER_CHECK;
            return err;
        }

        FileSpec file = { .uri = config.saveas };
        config.files.push_back(file);
    }

    err = CheckFileSpec(context, config);
    if (err.code != ExceptionErrorCode::E_OK) {
        return err;
    }

    if (!CJTask::SetDirsPermission(config.certsPath)) {
        return {
            .code = ExceptionErrorCode::E_FILE_IO,
            .errInfo = "set files of directors permission fail"
        };
    }

    if (config.action == Action::UPLOAD) {
        std::string filePath = context->GetCacheDir();
        err = CheckUploadBodyFiles(config, filePath);
    }

    return err;
}

ExceptionError CJInitialize::ParseConfig(OHOS::AbilityRuntime::Context *stageContext, const CConfig *ffiConfig,
    Config &config)
{
    config.action = (OHOS::Request::Action)ffiConfig->action;
    config.withErrCode = true;
    config.version = Version::API10; // CJ only support API10

    if (stageContext == nullptr) {
        return {
            .code = ExceptionErrorCode::E_PARAMETER_CHECK,
            .errInfo = "Get context fail"
        };
    }

    std::shared_ptr<OHOS::AbilityRuntime::Context> context = stageContext->shared_from_this();
    ExceptionError ret = ParseBundleName(context, config.bundleName);
    if (ret.code != 0) {
        return ret;
    }
    ret.code = ExceptionErrorCode::E_PARAMETER_CHECK;
    if (!ParseUrl(config.url)) {
        ret.errInfo = "parse url error";
        return ret;
    }

    if (!ParseCertsPath(config.url, config.certsPath)) {
        ret.errInfo = "parse certs path error";
        return ret;
    }

    if (!ParseData(ffiConfig, config)) {
        ret.errInfo = "parse data error";
        return ret;
    }

    if (!ParseIndex(config)) {
        ret.errInfo = "Index exceeds file list";
        return ret;
    }

    if (!ParseTitle(config) ||
        !ParseToken(config) ||
        !ParseDescription(config.description)) {
        ret.errInfo = "Exceeding maximum length";
        return ret;
    }

    if (!ParseSaveas(config)) {
        ret.errInfo = "parse saveas error";
        return ret;
    }

    ParseMethod(config);
    ParseNetwork(config.network);
    ParseBackGround(config.mode, config.background);
    config.begins = ParseBegins(config.begins);

    return CheckFilePath(context, config);
}

bool CJInitialize::FindDir(const std::string &pathDir)
{
    std::error_code err;
    return std::filesystem::exists(pathDir, err);
}


} //namespace OHOS::CJSystemapi::Request
