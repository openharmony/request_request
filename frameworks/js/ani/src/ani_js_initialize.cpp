/*
 * Copyright (C) 2025 Huawei Device Co., Ltd.
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
#include "ani_js_initialize.h"
#include "ani_task.h"
#include <fcntl.h>
#include <securec.h>
#include <sys/stat.h>

#include <algorithm>
#include <cstdio>
#include <cstring>
#include <filesystem>
#include <fstream>
#include <new>
#include <regex>
#include <string>
#include <system_error>
#include "ani_utils.h"
#include "log.h"
#include "net_conn_client.h"
#include "parameter.h"
#include "request_common.h"
#include "request_manager.h"
#include "sys_event.h"

static const std::string AREA1 = "/data/storage/el1/base";
static const std::string AREA2 = "/data/storage/el2/base";
static const std::string AREA5 = "/data/storage/el5/base";
static constexpr uint32_t MAX_UPLOAD_ON15_FILES = 100;

using namespace OHOS::AniUtil;
namespace OHOS::Request {
bool IsPathValid(const std::string &filePath)
{
    auto path = filePath.substr(0, filePath.rfind('/'));
    char resolvedPath[PATH_MAX] = { 0 };
    if (path.length() > PATH_MAX || realpath(path.c_str(), resolvedPath) == nullptr
        || strncmp(resolvedPath, path.c_str(), path.length()) != 0) {
        REQUEST_HILOGE("invalid file path!");
        return false;
    }
    return true;
}

std::shared_ptr<OHOS::AbilityRuntime::Context> JsInitialize::GetContext(ani_env *env, ani_object object)
{
    if (env == nullptr) {
        REQUEST_HILOGE("env is nullptr");
        return nullptr;
    }

    ani_long nativeContextLong;
    ani_status status = env->Object_GetFieldByName_Long(object, "nativeContext", &nativeContextLong);
    if (status != ANI_OK) {
        REQUEST_HILOGE("Object_GetField_Long failed, status : %{public}d", status);
        return nullptr;
    }
    REQUEST_HILOGI("in GetStageModeContext nativeContext is %{public}lld.", static_cast<long long>(nativeContextLong));

    auto weakContext = reinterpret_cast<std::weak_ptr<OHOS::AbilityRuntime::Context>*>(nativeContextLong);
    if (weakContext == nullptr) {
        REQUEST_HILOGE("into GetStageModeContext, weakContext is nullptr");
    }
    auto ret =  weakContext != nullptr ? weakContext->lock() : nullptr;
    if (ret == nullptr) {
        REQUEST_HILOGE("into GetStageModeContext, ret is nullptr");
        return ret;
    }
    REQUEST_HILOGI("into GetStageModeContext, ret is not nullptr");
    return ret;
}

bool JsInitialize::GetAppBaseDir(std::string &baseDir)
{
    auto context = AbilityRuntime::Context::GetApplicationContext();
    if (context == nullptr) {
        REQUEST_HILOGE("AppContext is null.");
        SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_02, "AppContext is null");
        return false;
    }
    baseDir = context->GetBaseDir();
    if (baseDir.empty()) {
        REQUEST_HILOGE("Base dir not found.");
        SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_07, "Base dir not found");
        return false;
    }
    return true;
}

bool JsInitialize::CheckFilePath(
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config, ExceptionError &error)
{
    if (config.action == Action::DOWNLOAD) {
        if (!CheckDownloadFile(context, config, error)) {
            SysEventLog::SendSysEventLog(STATISTIC_EVENT, APP_ERROR_00, config.bundleName, "", error.errInfo);
            return false;
        }
    } else {
        if (!CheckUploadFiles(context, config, error)) {
            SysEventLog::SendSysEventLog(STATISTIC_EVENT, APP_ERROR_01, config.bundleName, "", error.errInfo);
            return false;
        }
        std::string filePath = context->GetCacheDir();
        if (!CheckUploadBodyFiles(filePath, config, error)) {
            SysEventLog::SendSysEventLog(STATISTIC_EVENT, APP_ERROR_02, config.bundleName, "", error.errInfo);
            return false;
        }
    }
    if (!AniTask::SetDirsPermission(config.certsPath)) {
        error.code = E_FILE_IO;
        error.errInfo = "set files of directors permission fail";
        SysEventLog::SendSysEventLog(FAULT_EVENT, TASK_FAULT_02, config.bundleName, "", error.errInfo);
        return false;
    }
    return true;
}

bool JsInitialize::CheckUploadBodyFiles(const std::string &filePath, Config &config, ExceptionError &error)
{
    size_t len = config.files.size();
    if (config.multipart) {
        len = 1;
    }

    for (size_t i = 0; i < len; i++) {
        if (filePath.empty()) {
            REQUEST_HILOGE("internal to cache error");
            error.code = E_PARAMETER_CHECK;
            error.errInfo = "Parameter verification failed, UploadBodyFiles error empty path";
            return false;
        }
        auto now = std::chrono::high_resolution_clock::now();
        auto timestamp = std::chrono::duration_cast<std::chrono::nanoseconds>(now.time_since_epoch()).count();
        std::string path = filePath + "/tmp_body_" + std::to_string(i) + "_" + std::to_string(timestamp);
        if (!IsPathValid(path)) {
            REQUEST_HILOGE("Upload IsPathValid error");
            error.code = E_PARAMETER_CHECK;
            error.errInfo = "Parameter verification failed, UploadBodyFiles error fail path";
            return false;
        }

        FILE *bodyFile = fopen(path.c_str(), "w+");
        if (bodyFile == NULL) {
            error.code = E_FILE_IO;
            error.errInfo = "UploadBodyFiles failed to open file errno " + std::to_string(errno);
            SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_00, config.bundleName, "", error.errInfo);
            return false;
        }
        int32_t ret = chmod(path.c_str(), S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH);
        if (ret != 0) {
            REQUEST_HILOGE("body chmod fail: %{public}d", ret);
            SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_01, config.bundleName, "", std::to_string(ret));
        };

        bool setRes = AniTask::SetPathPermission(path);
        int32_t retClose = fclose(bodyFile);
        if (retClose != 0) {
            REQUEST_HILOGE("upload body fclose fail: %{public}d", ret);
            SysEventLog::SendSysEventLog(
                FAULT_EVENT, STANDARD_FAULT_02, config.bundleName, "", std::to_string(retClose));
        }
        if (!setRes) {
            error.code = E_FILE_IO;
            error.errInfo = "UploadBodyFiles set body path permission fail";
            return false;
        }
        config.bodyFileNames.push_back(path);
    }
    return true;
}

bool JsInitialize::CheckPathIsFile(const std::string &path, ExceptionError &error)
{
    std::error_code err;
    if (!std::filesystem::exists(path, err)) {
        error.code = E_FILE_IO;
        error.errInfo = "Path not exists: " + err.message();
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_03, error.errInfo);
        return false;
    }
    if (std::filesystem::is_directory(path, err)) {
        error.code = E_FILE_IO;
        error.errInfo = "Path not File: " + err.message();
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_04, error.errInfo);
        return false;
    }
    return true;
}

bool JsInitialize::GetFdDownload(const std::string &path, const Config &config, ExceptionError &error)
{
    // File is exist.
    if (JsInitialize::FindDir(path)) {
        if (config.firstInit && !config.overwrite) {
            error.code = config.version == Version::API10 ? E_FILE_IO : E_FILE_PATH;
            error.errInfo = "GetFd File exists and other error";
            SysEventLog::SendSysEventLog(STATISTIC_EVENT, APP_ERROR_00, config.bundleName, "", error.errInfo);
            return false;
        }
    }

    FILE *file = NULL;
    if (!IsPathValid(path)) {
        REQUEST_HILOGE("GetFdDownload IsPathValid error");
        error.code = E_PARAMETER_CHECK;
        error.errInfo = "Parameter verification failed, GetFdDownload error fail path";
        return false;
    }
    if (config.firstInit) {
        file = fopen(path.c_str(), "w+");
    } else {
        file = fopen(path.c_str(), "a+");
    }

    if (file == NULL) {
        error.code = E_FILE_IO;
        error.errInfo = "GetFd failed to open file errno " + std::to_string(errno);
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_00, config.bundleName, "", error.errInfo);
        return false;
    }

    int32_t ret = chmod(path.c_str(), S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH);
    if (ret != 0) {
        REQUEST_HILOGE("download file chmod fail: %{public}d", ret);
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_01, config.bundleName, "", std::to_string(ret));
    };

    int32_t retClose = fclose(file);
    if (retClose != 0) {
        REQUEST_HILOGE("download fclose fail: %{public}d", ret);
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_02, config.bundleName, "", std::to_string(retClose));
    }
    return true;
}

bool JsInitialize::InterceptData(const std::string &str, const std::string &in, std::string &out)
{
    std::size_t position = in.find_last_of(str);
    // when the str at last index, will error.
    if (position == std::string::npos || position + 1 >= in.size()) {
        return false;
    }
    out = std::string(in, position + 1);
    return true;
}

bool JsInitialize::GetFdUpload(const std::string &path, const Config &config, ExceptionError &error)
{
    if (!JsInitialize::CheckPathIsFile(path, error)) {
        error.code = config.version == Version::API10 ? E_FILE_IO : E_FILE_PATH;
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_03, config.bundleName, "", error.errInfo);
        return false;
    }
    if (!IsPathValid(path)) {
        REQUEST_HILOGE("GetFdUpload IsPathValid error");
        error.code = E_PARAMETER_CHECK;
        error.errInfo = "Parameter verification failed, GetFdUpload error fail path";
        return false;
    }
    FILE *file = fopen(path.c_str(), "r");
    if (file == NULL) {
        error.code = config.version == Version::API10 ? E_FILE_IO : E_FILE_PATH;
        error.errInfo = "GetFd failed to open file errno " + std::to_string(errno);
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_00, config.bundleName, "", error.errInfo);
        return false;
    }
    REQUEST_HILOGD("upload file fopen ok");
    int32_t ret = chmod(path.c_str(), S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH);
    if (ret != 0) {
        REQUEST_HILOGE("upload file chmod fail: %{public}d", ret);
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_01, config.bundleName, "", std::to_string(ret));
    }
    int32_t retClose = fclose(file);
    if (retClose != 0) {
        REQUEST_HILOGE("upload fclose fail: %{public}d", ret);
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_02, config.bundleName, "", std::to_string(retClose));
    }
    return true;
}

bool JsInitialize::GetInternalPath(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config,
    std::string &path, std::string &errInfo)
{
    std::string fileName;
    std::string pattern = "internal://cache/";
    size_t pos = path.find(pattern);
    if (pos != 0) {
        fileName = path;
    } else {
        fileName = path.substr(pattern.size(), path.size());
    }
    REQUEST_HILOGE("11 fileName %{public}s", fileName.c_str());
    if (fileName.empty()) {
        errInfo = "Parameter verification failed, GetInternalPath failed, fileName is empty";
        return false;
    }
    path = context->GetCacheDir();
    if (path.empty()) {
        REQUEST_HILOGE("internal to cache error");
        errInfo = "Parameter verification failed, GetInternalPath failed, cache path is empty";
        return false;
    }
    path += "/" + fileName;
    if (!IsPathValid(path)) {
        REQUEST_HILOGE("IsPathValid error %{public}s", path.c_str());
        errInfo = "Parameter verification failed, GetInternalPath failed, filePath is not valid";
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
    size_t posEnd = std::min({ tempUrl.find(':', posStart), tempUrl.find('/', posStart), tempUrl.find('?', posStart) });
    if (posEnd != std::string::npos) {
        return tempUrl.substr(posStart, posEnd - posStart);
    }
    return tempUrl.substr(posStart);
}

bool JsInitialize::CheckUserFileSpec(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context,
    const Config &config, FileSpec &file, ExceptionError &error)
{
    if (config.mode != Mode::FOREGROUND) {
        error.code = E_PARAMETER_CHECK;
        error.errInfo = "Parameter verification failed, user file can only for Mode::FOREGROUND";
        return false;
    }
    std::shared_ptr<Uri> uri = std::make_shared<Uri>(file.uri);
    std::shared_ptr<AppExecFwk::DataAbilityHelper> dataAbilityHelper =
        AppExecFwk::DataAbilityHelper::Creator(context, uri);
    if (dataAbilityHelper == nullptr) {
        REQUEST_HILOGE("dataAbilityHelper null");
        error.code = E_PARAMETER_CHECK;
        error.errInfo = "Parameter verification failed, dataAbilityHelper null";
        SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_07, config.bundleName, "", error.errInfo);
        return false;
    }
    file.fd = dataAbilityHelper->OpenFile(*uri, "r");
    if (file.fd < 0) {
        REQUEST_HILOGE("Failed to open user file, fd: %{public}d", file.fd);
        error.code = E_FILE_IO;
        error.errInfo = "Failed to open user file";
        SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_09, config.bundleName, "", error.errInfo);
        return false;
    }
    StandardizeFileSpec(file);
    return true;
}

bool JsInitialize::CheckUploadFiles(
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config, ExceptionError &error)
{
    int32_t sdkVersion = GetSdkApiVersion();
    constexpr const int32_t uploadVersion = 15;
    if (config.version == Version::API10 && sdkVersion >= uploadVersion
        && config.files.size() > MAX_UPLOAD_ON15_FILES) {
        error.code = E_PARAMETER_CHECK;
        error.errInfo = "Parameter verification failed, upload by multipart file so many";
        return false;
    }
    // need reconstruction.
    for (auto &file : config.files) {
        if (IsUserFile(file.uri)) {
            file.isUserFile = true;
            if (config.version == Version::API9) {
                error.code = E_PARAMETER_CHECK;
                error.errInfo = "Parameter verification failed, user file can only for request.agent.";
                return false;
            }
            if (!CheckUserFileSpec(context, config, file, error)) {
                return false;
            }
            StandardizeFileSpec(file);
            continue;
        }

        if (!CheckUploadFileSpec(context, config, file, error)) {
            return false;
        }
    }
    return true;
}

bool JsInitialize::CheckUploadFileSpec(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config,
    FileSpec &file, ExceptionError &error)
{
    file.isUserFile = false;
    std::string path = file.uri;
    if (config.version == Version::API9) {
        if (!GetInternalPath(context, config, path, error.errInfo)) {
            error.code = E_PARAMETER_CHECK;
            return false;
        }
    } else {
        std::vector<std::string> pathVec;
        if (!GetSandboxPath(context, config, path, pathVec, error.errInfo)) {
            error.code = E_PARAMETER_CHECK;
            return false;
        }
    }
    REQUEST_HILOGI("CheckUploadFileSpec path: %{public}s", path.c_str());
    file.uri = path;
    if (!GetFdUpload(path, config, error)) {
        return false;
    }
    if (!AniTask::SetPathPermission(file.uri)) {
        error.code = E_FILE_IO;
        error.errInfo = "set path permission fail";
        return false;
    }
    StandardizeFileSpec(file);
    return true;
}

bool JsInitialize::CheckDownloadFile(
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config, ExceptionError &error)
{
    if (config.version == Version::API9) {
        std::string path = config.saveas;
        if (config.saveas.find('/') == 0) {
            // API9 do not check.
        } else if (!GetInternalPath(context, config, path, error.errInfo)) {
            error.code = E_PARAMETER_CHECK;
            return false;
        }
        config.saveas = path;
    } else {
        if (!CheckDownloadFilePath(context, config, error.errInfo)) {
            error.code = E_PARAMETER_CHECK;
            return false;
        }
    }
    FileSpec file = { .uri = config.saveas, .isUserFile = false };
    StandardizeFileSpec(file);
    config.files.push_back(file);
    if (!GetFdDownload(file.uri, config, error)) {
        return false;
    }
    if (!AniTask::SetPathPermission(config.saveas)) {
        error.code = E_FILE_IO;
        error.errInfo = "set path permission fail, download";
        return false;
    }
    return true;
}

bool JsInitialize::CheckDownloadFilePath(
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config, std::string &errInfo)
{
    std::string path = config.saveas;
    std::vector<std::string> pathVec;
    if (!GetSandboxPath(context, config, path, pathVec, errInfo)) {
        return false;
    }
    // pop filename.
    pathVec.pop_back();
    if (!JsInitialize::CreateDirs(pathVec)) {
        REQUEST_HILOGE("CreateDirs Err");
        errInfo = "Parameter verification failed, this is fail saveas path";
        return false;
    }
    config.saveas = path;
    return true;
}

bool JsInitialize::CreateDirs(const std::vector<std::string> &pathDirs)
{
    std::string path;
    std::error_code err;
    for (auto elem : pathDirs) {
        path += "/" + elem;
        if (std::filesystem::exists(path, err)) {
            continue;
        }
        err.clear();
        // create_directory noexcept.
        if (!std::filesystem::create_directory(path, err)) {
            REQUEST_HILOGE("Create Dir Err: %{public}d, %{public}s", err.value(), err.message().c_str());
            SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_05, err.message());
            return false;
        }
    }
    return true;
}

bool JsInitialize::FindDir(const std::string &pathDir)
{
    std::error_code err;
    return std::filesystem::exists(pathDir, err);
}

bool JsInitialize::IsUserFile(const std::string &path)
{
    return path.find("file://docs/") == 0 || path.find("file://media/") == 0;
}

void JsInitialize::StandardizeFileSpec(FileSpec &file)
{
    if (file.filename.empty()) {
        InterceptData("/", file.uri, file.filename);
    }
    // Does not have "contentType" field or API9 "type" empty.
    if (!file.hasContentType) {
        InterceptData(".", file.filename, file.type);
    }
    if (file.name.empty()) {
        file.name = "file";
    }
    return;
}

bool JsInitialize::GetSandboxPath(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config,
    std::string &path, std::vector<std::string> &pathVec, std::string &errInfo)
{
    if (!StandardizePath(context, config, path)) {
        REQUEST_HILOGE("StandardizePath Err");
        errInfo = "Parameter verification failed, GetSandboxPath failed, StandardizePath fail";
        return false;
    };
    if (!WholeToNormal(path, pathVec) || pathVec.empty()) {
        REQUEST_HILOGE("WholeToNormal Err");
        errInfo = "Parameter verification failed, GetSandboxPath failed, WholeToNormal path fail";
        return false;
    };
    std::string baseDir;
    if (!CheckBelongAppBaseDir(path, baseDir)) {
        REQUEST_HILOGE("CheckBelongAppBaseDir Err");
        errInfo = "Parameter verification failed, GetSandboxPath failed, path not belong app base dir";
        return false;
    };
    return true;
}

// Must not user file.
bool JsInitialize::StandardizePath(
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config, std::string &path)
{
    std::string WHOLE_PREFIX = "/";
    std::string FILE_PREFIX = "file://";
    std::string INTERNAL_PREFIX = "internal://";
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
        return BaseToWhole(context, path);
    }
    if (path.find(CURRENT_PREFIX) == 0) {
        path.erase(0, CURRENT_PREFIX.size());
        return CacheToWhole(context, path);
    }
    return CacheToWhole(context, path);
}

// BaseDir is following context.
bool JsInitialize::BaseToWhole(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, std::string &path)
{
    std::string base = context->GetBaseDir();
    if (base.empty()) {
        REQUEST_HILOGE("GetBaseDir error.");
        SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_06, "GetCacheDir error");
        return false;
    }
    path = base + "/" + path;
    return true;
}

bool JsInitialize::CacheToWhole(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, std::string &path)
{
    std::string cache = context->GetCacheDir();
    if (cache.empty()) {
        REQUEST_HILOGE("GetCacheDir error.");
        SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_05, "GetCacheDir error");
        return false;
    }
    path = cache + "/" + path;
    return true;
}

bool JsInitialize::FileToWhole(
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

bool JsInitialize::WholeToNormal(std::string &path, std::vector<std::string> &out)
{
    std::string normalPath;
    std::vector<std::string> elems;
    StringSplit(path, '/', elems);
    if (!PathVecToNormal(elems, out)) {
        return false;
    }
    for (auto elem : out) {
        normalPath += "/" + elem;
    }
    path = normalPath;
    return true;
}

bool JsInitialize::PathVecToNormal(const std::vector<std::string> &in, std::vector<std::string> &out)
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

void JsInitialize::StringSplit(const std::string &str, const char delim, std::vector<std::string> &elems)
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

void JsInitialize::StringTrim(std::string &str)
{
    if (str.empty()) {
        return;
    }
    str.erase(0, str.find_first_not_of(" "));
    str.erase(str.find_last_not_of(" ") + 1);
    return;
}

bool JsInitialize::CheckBelongAppBaseDir(const std::string &filepath, std::string &baseDir)
{
    if (!JsInitialize::GetAppBaseDir(baseDir)) {
        return false;
    }
    if ((filepath.find(AREA1) == 0) || filepath.find(AREA2) == 0 || filepath.find(AREA5) == 0) {
        return true;
    } else {
        REQUEST_HILOGE("File dir not include base dir: %{public}s, path dir: %{public}s",
            baseDir.c_str(), filepath.c_str());
        return false;
    }
}

bool JsInitialize::Convert2FileSpec(ani_env *env, ani_object aniValue, const std::string &name, FileSpec &file)
{
    REQUEST_HILOGI("Convert2FileSpec in");
    file.name = name;
    ani_ref pathRef;
    if (ANI_OK != env->Object_GetPropertyByName_Ref(aniValue, "path", &pathRef)) {
        REQUEST_HILOGE("Object_GetFieldByName_Ref value from data Faild");
        return false;
    }
    file.uri = AniStringUtils::ToStd(env, static_cast<ani_string>(pathRef));
    StringTrim(file.uri);
    if (file.uri.empty()) {
        return false;
    }
    file.filename = "";
    ani_ref fileNameRef;
    if (env->Object_GetPropertyByName_Ref(aniValue, "filename", &fileNameRef) == ANI_OK) {
        file.filename = AniStringUtils::ToStd(env, static_cast<ani_string>(fileNameRef));
    }
    return true;
}

bool JsInitialize::Convert2FileSpecs(ani_env *env, ani_object aniValue, const std::string &name,
    std::vector<FileSpec> &files)
{
    UnionAccessor unionAccessor(env, aniValue);
    std::vector<ani_ref> arrayValues = {};
    if (!unionAccessor.TryConvertArray<ani_ref>(arrayValues) || arrayValues.empty()) {
        return false;
    }
    bool ret = false;
    for (uint16_t i = 0; i < arrayValues.size(); i++) {
        ani_object data = static_cast<ani_object>(arrayValues[i]);
        FileSpec file;
        ret = Convert2FileSpec(env, data, name, file);
        if (!ret) {
            return false;
        }
        files.push_back(file);
    }
    return true;
}

} // namespace OHOS::Request
