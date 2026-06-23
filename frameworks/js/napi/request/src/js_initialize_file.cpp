/*
 * Copyright (C) 2023 Huawei Device Co., Ltd.
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

#include "js_initialize.h"

#include <cerrno>
#include <fcntl.h>
#include <securec.h>
#include <sys/stat.h>

#include <chrono>
#include <cstdio>
#include <filesystem>
#include <new>
#include <string>

#include "file_uri.h"
#include "log.h"
#include "napi_utils.h"
#include "request_common.h"
#include "sys_event.h"

static constexpr uint32_t FILE_PERMISSION = 0644;
static constexpr uint32_t MAX_UPLOAD_ON15_FILES = 100;

namespace OHOS::Request {

static std::string GetErrnoAppendMessage(int32_t errNum)
{
    switch (errNum) {
        case ENOENT:
            return ", File not found";
        case EACCES:
            return ", Permission denied";
        case EISDIR:
            return ", Path is a directory, not a file";
        case ENOSPC:
            return ", Insufficient storage space on device";
        case EROFS:
            return ", Read-only file system";
        default:
            return "";
    }
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
    if (!JsTask::SetDirsPermission(config.certsPath)) {
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
        if (!NapiUtils::IsPathValid(path)) {
            REQUEST_HILOGE("Upload IsPathValid error");
            error.code = E_PARAMETER_CHECK;
            error.errInfo = "Parameter verification failed, UploadBodyFiles error fail path";
            return false;
        }
        FILE *bodyFile = fopen(path.c_str(), "w+");
        if (bodyFile == NULL) {
            error.code = E_FILE_IO;
            error.errInfo = "UploadBodyFiles failed to open file errno " + std::to_string(errno)
                + GetErrnoAppendMessage(errno);
            SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_00, config.bundleName, "", error.errInfo);
            return false;
        }
        int32_t retClose = fclose(bodyFile);
        if (retClose != 0) {
            REQUEST_HILOGE("upload body fclose fail: %{public}d", retClose);
            SysEventLog::SendSysEventLog(
                FAULT_EVENT, STANDARD_FAULT_02, config.bundleName, "", std::to_string(retClose));
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
            error.errInfo = "GetFd File exists and other error, set overwrite=true to replace";
            SysEventLog::SendSysEventLog(STATISTIC_EVENT, APP_ERROR_00, config.bundleName, "", error.errInfo);
            return false;
        }
    }

    FILE *file = NULL;
    if (config.firstInit) {
        file = fopen(path.c_str(), "w+");
    } else {
        file = fopen(path.c_str(), "a+");
    }

    if (file == NULL) {
        error.code = E_FILE_IO;
        error.errInfo = "GetFd failed to open file errno " + std::to_string(errno)
            + GetErrnoAppendMessage(errno);
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_00, config.bundleName, "", error.errInfo);
        return false;
    }
    int32_t retClose = fclose(file);
    if (retClose != 0) {
        REQUEST_HILOGE("download fclose fail: %{public}d", retClose);
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_02, config.bundleName, "", std::to_string(retClose));
    }
    return true;
}

bool JsInitialize::GetFdUpload(const std::string &path, const Config &config, ExceptionError &error)
{
    if (!JsInitialize::CheckPathIsFile(path, error)) {
        error.code = config.version == Version::API10 ? E_FILE_IO : E_FILE_PATH;
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_03, config.bundleName, "", error.errInfo);
        return false;
    }
    FILE *file = fopen(path.c_str(), "r");
    if (file == NULL) {
        error.code = config.version == Version::API10 ? E_FILE_IO : E_FILE_PATH;
        error.errInfo = "GetFd failed to open file errno " + std::to_string(errno)
            + GetErrnoAppendMessage(errno);
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_00, config.bundleName, "", error.errInfo);
        return false;
    }
    REQUEST_HILOGD("upload file fopen ok");
    int32_t retClose = fclose(file);
    if (retClose != 0) {
        REQUEST_HILOGE("upload fclose fail: %{public}d", retClose);
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_02, config.bundleName, "", std::to_string(retClose));
    }
    return true;
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

bool JsInitialize::CheckUserFileSpec(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context,
    const Config &config, FileSpec &file, ExceptionError &error, bool isUpload)
{
    if (config.mode != Mode::FOREGROUND) {
        error.code = E_PARAMETER_CHECK;
        error.errInfo = "Parameter verification failed, user file can only for Mode::FOREGROUND";
        return false;
    }
    if (isUpload) {
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
    } else {
        std::shared_ptr<AppFileService::ModuleFileUri::FileUri> fileUri =
            std::make_shared<AppFileService::ModuleFileUri::FileUri>(file.uri);
        std::string realPath = fileUri->GetRealPath();
        if (config.firstInit) {
            file.fd = open(realPath.c_str(), O_RDWR | O_TRUNC | O_CLOEXEC);
        } else {
            file.fd = open(realPath.c_str(), O_RDWR | O_APPEND | O_CLOEXEC);
        }
    }
    if (file.fd < 0) {
        REQUEST_HILOGE("Failed to open user file, fd: %{public}d", file.fd);
        error.code = E_FILE_IO;
        error.errInfo = "Failed to open user file, errno " + std::to_string(errno)
            + GetErrnoAppendMessage(errno);
        SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_09, config.bundleName, "", error.errInfo);
        return false;
    }
    fdsan_exchange_owner_tag(file.fd, 0, REQUEST_FDSAN_TAG);
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
            if (!CheckUserFileSpec(context, config, file, error, true)) {
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
        StandardizePathApi9(path);
    } else {
        std::vector<std::string> pathVec;
        if (!GetSandboxPath(context, config, path, pathVec, error.errInfo)) {
            error.code = E_PARAMETER_CHECK;
            return false;
        }
    }
    REQUEST_HILOGD("CheckUploadFileSpec path");
    file.uri = path;
    if (!GetFdUpload(path, config, error)) {
        return false;
    }
    StandardizeFileSpec(file);
    return true;
}

void JsInitialize::StandardizePathApi9(std::string &path)
{
    std::vector<std::string> pathVec;
    if (!JsInitialize::WholeToNormal(path, pathVec) || pathVec.empty()) {
        REQUEST_HILOGE("WholeToNormal Err api9");
    };
}

bool JsInitialize::CheckDownloadFile(
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config, ExceptionError &error)
{
    if (IsUserFile(config.saveas)) {
        if (config.version == Version::API9) {
            error.code = E_PARAMETER_CHECK;
            error.errInfo = "Parameter verification failed, user file can only for request.agent.";
            return false;
        }
        if (!config.overwrite) {
            error.code = E_PARAMETER_CHECK;
            error.errInfo = "Parameter verification failed, download to user file must support overrite.";
            return false;
        }
        FileSpec file = { .uri = config.saveas, .isUserFile = true };
        if (!CheckUserFileSpec(context, config, file, error, false)) {
            return false;
        }
        config.files.push_back(file);
        return true;
    }
    if (config.version == Version::API9) {
        std::string path = config.saveas;
        if (config.saveas.find('/') == 0) {
        } else if (!GetInternalPath(context, config, path, error.errInfo)) {
            error.code = E_PARAMETER_CHECK;
            return false;
        }
        StandardizePathApi9(path);
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

} // namespace OHOS::Request
