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

#include "request_action.h"

#include <fcntl.h>
#include <securec.h>
#include <sys/stat.h>

#include <filesystem>
#include <fstream>
#include <memory>
#include <string>
#include <unordered_map>
#include <vector>

#include "access_token.h"
#include "accesstoken_kit.h"
#include "application_context.h"
#include "constant.h"
#include "data_ability_helper.h"
#include "ffrt.h"
#include "file_uri.h"
#include "log.h"
#include "path_control.h"
#include "request_common.h"
#include "request_manager.h"
#include "storage_acl.h"

namespace OHOS::Request {
using namespace OHOS::Security::AccessToken;
using namespace OHOS::StorageDaemon;
namespace fs = std::filesystem;

static std::mutex taskMutex_;
static std::map<std::string, Config> taskMap_;

const std::unique_ptr<RequestAction> &RequestAction::GetInstance()
{
    static std::unique_ptr<RequestAction> instance = std::make_unique<RequestAction>();
    return instance;
}

int32_t RequestAction::Start(const std::string &tid)
{
    return RequestManager::GetInstance()->Start(tid);
}
int32_t RequestAction::Stop(const std::string &tid)
{
    return RequestManager::GetInstance()->Stop(tid);
}

int32_t RequestAction::Touch(const std::string &tid, const std::string &token, TaskInfo &info)
{
    return RequestManager::GetInstance()->Touch(tid, token, info);
}

int32_t RequestAction::Show(const std::string &tid, TaskInfo &info)
{
    return RequestManager::GetInstance()->Show(tid, info);
}

int32_t RequestAction::Pause(const std::string &tid)
{
    return RequestManager::GetInstance()->Pause(tid, Version::API10);
}

int32_t RequestAction::Resume(const std::string &tid)
{
    return RequestManager::GetInstance()->Resume(tid);
}

int32_t RequestAction::SetMaxSpeed(const std::string &tid, const int64_t maxSpeed)
{
    return RequestManager::GetInstance()->SetMaxSpeed(tid, maxSpeed);
}

ExceptionErrorCode RequestAction::StartTasks(
    const std::vector<std::string> &tids, std::unordered_map<std::string, ExceptionErrorCode> &rets)
{
    rets.clear();
    std::vector<ExceptionErrorCode> vec;
    ExceptionErrorCode code = RequestManager::GetInstance()->StartTasks(tids, vec);
    if (code != ExceptionErrorCode::E_OK) {
        return code;
    }
    uint32_t len = static_cast<uint32_t>(tids.size());
    for (uint32_t i = 0; i < len; i++) {
        rets.insert_or_assign(tids[i], vec[i]);
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestAction::StopTasks(
    const std::vector<std::string> &tids, std::unordered_map<std::string, ExceptionErrorCode> &rets)
{
    rets.clear();
    std::vector<ExceptionErrorCode> vec;
    ExceptionErrorCode code = RequestManager::GetInstance()->StopTasks(tids, vec);
    if (code != ExceptionErrorCode::E_OK) {
        return code;
    }
    uint32_t len = static_cast<uint32_t>(tids.size());
    for (uint32_t i = 0; i < len; i++) {
        rets.insert_or_assign(tids[i], vec[i]);
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestAction::ResumeTasks(
    const std::vector<std::string> &tids, std::unordered_map<std::string, ExceptionErrorCode> &rets)
{
    rets.clear();
    std::vector<ExceptionErrorCode> vec;
    ExceptionErrorCode code = RequestManager::GetInstance()->ResumeTasks(tids, vec);
    if (code != ExceptionErrorCode::E_OK) {
        return code;
    }
    uint32_t len = static_cast<uint32_t>(tids.size());
    for (uint32_t i = 0; i < len; i++) {
        rets.insert_or_assign(tids[i], vec[i]);
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestAction::PauseTasks(
    const std::vector<std::string> &tids, std::unordered_map<std::string, ExceptionErrorCode> &rets)
{
    rets.clear();
    std::vector<ExceptionErrorCode> vec;
    ExceptionErrorCode code = RequestManager::GetInstance()->PauseTasks(tids, Version::API10, vec);
    if (code != ExceptionErrorCode::E_OK) {
        return code;
    }
    uint32_t len = static_cast<uint32_t>(tids.size());
    for (uint32_t i = 0; i < len; i++) {
        rets.insert_or_assign(tids[i], vec[i]);
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestAction::ShowTasks(
    const std::vector<std::string> &tids, std::unordered_map<std::string, TaskInfoRet> &rets)
{
    rets.clear();
    std::vector<TaskInfoRet> vec;
    ExceptionErrorCode code = RequestManager::GetInstance()->ShowTasks(tids, vec);
    if (code != ExceptionErrorCode::E_OK) {
        return code;
    }
    uint32_t len = static_cast<uint32_t>(tids.size());
    for (uint32_t i = 0; i < len; i++) {
        rets.insert_or_assign(tids[i], vec[i]);
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestAction::TouchTasks(
    const std::vector<TaskIdAndToken> &tidTokens, std::unordered_map<std::string, TaskInfoRet> &rets)
{
    rets.clear();
    std::vector<TaskInfoRet> vec;
    ExceptionErrorCode code = RequestManager::GetInstance()->TouchTasks(tidTokens, vec);
    if (code != ExceptionErrorCode::E_OK) {
        return code;
    }
    uint32_t len = static_cast<uint32_t>(tidTokens.size());
    for (uint32_t i = 0; i < len; i++) {
        rets.insert_or_assign(tidTokens[i].tid, vec[i]);
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestAction::SetMaxSpeeds(
    const std::vector<SpeedConfig> &speedConfig, std::unordered_map<std::string, ExceptionErrorCode> &rets)
{
    rets.clear();
    std::vector<ExceptionErrorCode> vec;
    ExceptionErrorCode code = RequestManager::GetInstance()->SetMaxSpeeds(speedConfig, vec);
    if (code != ExceptionErrorCode::E_OK) {
        return code;
    }
    uint32_t len = static_cast<uint32_t>(speedConfig.size());
    for (uint32_t i = 0; i < len; i++) {
        rets.insert_or_assign(speedConfig[i].tid, vec[i]);
    }
    return ExceptionErrorCode::E_OK;
}

ExceptionErrorCode RequestAction::SetMode(std::string &tid, Mode mode)
{
    return RequestManager::GetInstance()->SetMode(tid, mode);
}

ExceptionErrorCode RequestAction::DisableTaskNotification(
    const std::vector<std::string> &tids, std::unordered_map<std::string, ExceptionErrorCode> &rets)
{
    rets.clear();
    std::vector<ExceptionErrorCode> vec;
    ExceptionErrorCode code = RequestManager::GetInstance()->DisableTaskNotification(tids, vec);
    for (size_t i = 0; i < tids.size() && i < vec.size(); i++) {
        rets.insert_or_assign(tids[i], vec[i]);
    }
    return code;
}

bool RequestAction::CreateDirs(const std::vector<std::string> &pathDirs)
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
            return false;
        }
    }
    return true;
}

bool RequestAction::FileToWhole(
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

bool RequestAction::BaseToWhole(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, std::string &path)
{
    std::string base = context->GetBaseDir();
    if (base.empty()) {
        REQUEST_HILOGE("GetBaseDir error.");
        return false;
    }
    path = base + "/" + path;
    return true;
}

bool RequestAction::CacheToWhole(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, std::string &path)
{
    std::string cache = context->GetCacheDir();
    if (cache.empty()) {
        REQUEST_HILOGE("GetCacheDir error.");
        return false;
    }
    path = cache + "/" + path;
    return true;
}

bool RequestAction::StandardizePath(
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config, std::string &path)
{
    std::string wholePrefix = "/";
    std::string filePrefix = "file://";
    std::string internalPrefix = "internal://";
    std::string currentPrefix = "./";

    if (path.find(wholePrefix) == 0) {
        return true;
    }
    if (path.find(filePrefix) == 0) {
        path.erase(0, filePrefix.size());
        return FileToWhole(context, config, path);
    }
    if (path.find(internalPrefix) == 0) {
        path.erase(0, internalPrefix.size());
        return BaseToWhole(context, path);
    }
    if (path.find(currentPrefix) == 0) {
        path.erase(0, currentPrefix.size());
        return CacheToWhole(context, path);
    }
    return CacheToWhole(context, path);
}

void RequestAction::StringSplit(const std::string &str, const char delim, std::vector<std::string> &elems)
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

bool RequestAction::PathVecToNormal(const std::vector<std::string> &in, std::vector<std::string> &out)
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

bool RequestAction::WholeToNormal(std::string &path, std::vector<std::string> &out)
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

bool RequestAction::GetAppBaseDir(std::string &baseDir)
{
    auto context = AbilityRuntime::Context::GetApplicationContext();
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

bool RequestAction::CheckBelongAppBaseDir(const std::string &filepath, std::string &baseDir)
{
    if (!GetAppBaseDir(baseDir)) {
        return false;
    }
    return FindAreaPath(filepath);
}

bool RequestAction::FindAreaPath(const std::string &filepath)
{
    if (PathControl::CheckBelongAppBaseDir(filepath)) {
        return true;
    } else {
        REQUEST_HILOGE("File dir not include base dir");
        return false;
    }
}

bool RequestAction::GetSandboxPath(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config,
    std::string &path, std::vector<std::string> &pathVec)
{
    if (!StandardizePath(context, config, path)) {
        REQUEST_HILOGE("StandardizePath Err");
        return false;
    };
    if (!WholeToNormal(path, pathVec) || pathVec.empty()) {
        REQUEST_HILOGE("WholeToNormal Err");
        return false;
    };
    std::string baseDir;
    if (!CheckBelongAppBaseDir(path, baseDir)) {
        REQUEST_HILOGE("CheckBelongAppBaseDir Err");
        return false;
    };
    return true;
}

bool RequestAction::CheckDownloadFilePath(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config)
{
    std::string path = config.saveas;
    std::vector<std::string> pathVec;
    if (!GetSandboxPath(context, config, path, pathVec)) {
        return false;
    }
    // pop filename.
    pathVec.pop_back();
    if (!CreateDirs(pathVec)) {
        REQUEST_HILOGE("CreateDirs Err");
        return false;
    }
    config.saveas = path;
    return true;
}

bool RequestAction::InterceptData(const std::string &str, const std::string &in, std::string &out)
{
    std::size_t position = in.find_last_of(str);
    // when the str at last index, will error.
    if (position == std::string::npos || position + 1 >= in.size()) {
        return false;
    }
    out = std::string(in, position + 1);
    return true;
}

void RequestAction::StandardizeFileSpec(FileSpec &file)
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

bool RequestAction::IsPathValid(const std::string &filePath)
{
    auto path = filePath.substr(0, filePath.rfind('/'));
    char resolvedPath[PATH_MAX + 1] = { 0 };
    if (path.length() > PATH_MAX || realpath(path.c_str(), resolvedPath) == nullptr
        || strncmp(resolvedPath, path.c_str(), path.length()) != 0) {
        REQUEST_HILOGE("invalid file path!");
        return false;
    }
    return true;
}

bool RequestAction::GetInternalPath(
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config, std::string &path)
{
    std::string fileName;
    std::string pattern = "internal://cache/";
    size_t pos = path.find(pattern);
    if (pos != 0) {
        fileName = path;
    } else {
        fileName = path.substr(pattern.size(), path.size());
    }
    if (fileName.empty()) {
        return false;
    }
    path = context->GetCacheDir();
    if (path.empty()) {
        REQUEST_HILOGE("internal to cache error");
        return false;
    }
    path += "/" + fileName;
    if (!IsPathValid(path)) {
        REQUEST_HILOGE("IsPathValid error");
        return false;
    }
    return true;
}

bool RequestAction::FindDir(const std::string &pathDir)
{
    std::error_code err;
    return std::filesystem::exists(pathDir, err);
}

ExceptionErrorCode RequestAction::GetFdDownload(const std::string &path, const Config &config)
{
    // File is exist.
    if (FindDir(path)) {
        if (config.firstInit && !config.overwrite) {
            return config.version == Version::API10 ? E_FILE_IO : E_FILE_PATH;
        }
    }

    FILE *file = nullptr;
    if (config.firstInit) {
        file = fopen(path.c_str(), "w+");
    } else {
        file = fopen(path.c_str(), "a+");
    }

    if (file == nullptr) {
        return E_FILE_IO;
    }

    int32_t ret = chmod(path.c_str(), S_IRUSR | S_IWUSR | S_IRGRP);
    if (ret != 0) {
        REQUEST_HILOGE("download file chmod fail: %{public}d", ret);
    };

    int32_t retClose = fclose(file);
    if (retClose != 0) {
        REQUEST_HILOGE("download fclose fail: %{public}d", ret);
    }
    return E_OK;
}

ExceptionErrorCode RequestAction::CheckDownloadFile(
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config)
{
    ExceptionErrorCode ret;
    if (IsUserFile(config.saveas)) {
        if (config.version == Version::API9 || !config.overwrite) {
            return E_PARAMETER_CHECK;
        }
        FileSpec file = { .uri = config.saveas, .isUserFile = true };
        ret = CheckUserFileSpec(context, config, file, false);
        if (ret == ExceptionErrorCode::E_OK) {
            config.files.push_back(file);
        }
        return ret;
    }
    if (config.version == Version::API9) {
        std::string path = config.saveas;
        if (config.saveas.find('/') == 0) {
            // API9 do not check.
        } else if (!GetInternalPath(context, config, path)) {
            return E_PARAMETER_CHECK;
        }
        config.saveas = path;
    } else {
        if (!CheckDownloadFilePath(context, config)) {
            return E_PARAMETER_CHECK;
        }
    }
    FileSpec file = { .uri = config.saveas, .isUserFile = false };
    StandardizeFileSpec(file);
    config.files.push_back(file);
    ret = GetFdDownload(file.uri, config);
    if (ret != ExceptionErrorCode::E_OK) {
        return ret;
    }
    if (!PathControl::AddPathsToMap(config.saveas)) {
        return ExceptionErrorCode::E_FILE_IO;
    }
    return ExceptionErrorCode::E_OK;
}

bool RequestAction::IsUserFile(const std::string &path)
{
    return path.find("file://docs/") == 0 || path.find("file://media/") == 0;
}

ExceptionErrorCode RequestAction::CheckUserFileSpec(
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config, FileSpec &file, bool isUpload)
{
    if (config.mode != Mode::FOREGROUND) {
        return E_PARAMETER_CHECK;
    }
    if (isUpload) {
        std::shared_ptr<Uri> uri = std::make_shared<Uri>(file.uri);
        std::shared_ptr<AppExecFwk::DataAbilityHelper> dataAbilityHelper =
            AppExecFwk::DataAbilityHelper::Creator(context, uri);
        if (dataAbilityHelper == nullptr) {
            REQUEST_HILOGE("dataAbilityHelper null");
            return E_PARAMETER_CHECK;
        }
        file.fd = dataAbilityHelper->OpenFile(*uri, "r");
    } else {
        std::shared_ptr<AppFileService::ModuleFileUri::FileUri> fileUri =
            std::make_shared<AppFileService::ModuleFileUri::FileUri>(file.uri);
        std::string realPath = fileUri->GetRealPath();
        if (config.firstInit) {
            file.fd = open(realPath.c_str(), O_RDWR | O_TRUNC);
        } else {
            file.fd = open(realPath.c_str(), O_RDWR | O_APPEND);
        }
    }
    if (file.fd < 0) {
        REQUEST_HILOGE("Failed to open user file, fd: %{public}d", file.fd);
        return E_FILE_IO;
    }
    fdsan_exchange_owner_tag(file.fd, 0, REQUEST_FDSAN_TAG);
    StandardizeFileSpec(file);
    return E_OK;
}

bool RequestAction::CheckPathIsFile(const std::string &path)
{
    std::error_code err;
    if (!std::filesystem::exists(path, err)) {
        return false;
    }
    if (std::filesystem::is_directory(path, err)) {
        return false;
    }
    return true;
}

ExceptionErrorCode RequestAction::GetFdUpload(const std::string &path, const Config &config)
{
    if (!CheckPathIsFile(path)) {
        return config.version == Version::API10 ? E_FILE_IO : E_FILE_PATH;
    }
    FILE *file = fopen(path.c_str(), "r");
    if (file == nullptr) {
        return config.version == Version::API10 ? E_FILE_IO : E_FILE_PATH;
    }
    REQUEST_HILOGD("upload file fopen ok");
    int32_t ret = chmod(path.c_str(), S_IRUSR | S_IWUSR | S_IRGRP);
    if (ret != 0) {
        REQUEST_HILOGE("upload file chmod fail: %{public}d", ret);
    }
    int32_t retClose = fclose(file);
    if (retClose != 0) {
        REQUEST_HILOGE("upload fclose fail: %{public}d", ret);
    }
    return E_OK;
}

ExceptionErrorCode RequestAction::CheckUploadFileSpec(
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config, FileSpec &file)
{
    ExceptionErrorCode ret;
    file.isUserFile = false;
    std::string path = file.uri;
    if (config.version == Version::API9) {
        if (!GetInternalPath(context, config, path)) {
            return E_PARAMETER_CHECK;
        }
    } else {
        std::vector<std::string> pathVec;
        if (!GetSandboxPath(context, config, path, pathVec)) {
            return E_PARAMETER_CHECK;
        }
    }
    REQUEST_HILOGD("CheckUploadFileSpec path");
    file.uri = path;
    ret = GetFdUpload(path, config);
    if (ret != E_OK) {
        return ret;
    }
    if (!PathControl::AddPathsToMap(file.uri)) {
        return E_FILE_IO;
    }
    StandardizeFileSpec(file);
    return E_OK;
}

ExceptionErrorCode RequestAction::CheckUploadFiles(
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config)
{
    // need reconstruction.
    ExceptionErrorCode ret;
    for (auto &file : config.files) {
        if (IsUserFile(file.uri)) {
            file.isUserFile = true;
            if (config.version == Version::API9) {
                return E_PARAMETER_CHECK;
            }
            ret = CheckUserFileSpec(context, config, file, true);
            if (ret != ExceptionErrorCode::E_OK) {
                return ret;
            }
            StandardizeFileSpec(file);
            continue;
        }

        ret = CheckUploadFileSpec(context, config, file);
        if (ret != ExceptionErrorCode::E_OK) {
            return ret;
        }
    }
    return E_OK;
}

ExceptionErrorCode RequestAction::CheckUploadBodyFiles(const std::string &filePath, Config &config)
{
    size_t len = config.files.size();
    if (config.multipart) {
        len = 1;
    }

    for (size_t i = 0; i < len; i++) {
        if (filePath.empty()) {
            REQUEST_HILOGE("internal to cache error");
            return E_PARAMETER_CHECK;
        }
        auto now = std::chrono::high_resolution_clock::now();
        auto timestamp = std::chrono::duration_cast<std::chrono::nanoseconds>(now.time_since_epoch()).count();
        std::string path = filePath + "/tmp_body_" + std::to_string(i) + "_" + std::to_string(timestamp);
        if (!IsPathValid(path)) {
            REQUEST_HILOGE("Upload IsPathValid error");
            return E_PARAMETER_CHECK;
        }
        FILE *bodyFile = fopen(path.c_str(), "w+");
        if (bodyFile == nullptr) {
            return E_FILE_IO;
        }
        int32_t ret = chmod(path.c_str(), S_IRUSR | S_IWUSR | S_IRGRP);
        if (ret != 0) {
            REQUEST_HILOGE("body chmod fail: %{public}d", ret);
        };

        bool setRes = PathControl::AddPathsToMap(path);
        int32_t retClose = fclose(bodyFile);
        if (retClose != 0) {
            REQUEST_HILOGE("upload body fclose fail: %{public}d", ret);
        }
        if (!setRes) {
            return E_FILE_IO;
        }
        config.bodyFileNames.push_back(path);
    }
    return E_OK;
}

bool RequestAction::SetDirsPermission(std::vector<std::string> &dirs)
{
    if (dirs.empty()) {
        return true;
    }
    std::string newPath = "/data/storage/el2/base/.ohos/.request/.certs";
    std::vector<std::string> dirElems;
    StringSplit(newPath, '/', dirElems);
    if (!CreateDirs(dirElems)) {
        REQUEST_HILOGE("CreateDirs Error");
        return false;
    }

    for (const auto &folderPath : dirs) {
        fs::path folder = folderPath;
        if (!(fs::exists(folder) && fs::is_directory(folder))) {
            return false;
        }
        for (const auto &entry : fs::directory_iterator(folder)) {
            fs::path path = entry.path();
            std::string existfilePath = folder.string() + "/" + path.filename().string();
            std::string newfilePath = newPath + "/" + path.filename().string();
            if (!fs::exists(newfilePath)) {
                fs::copy(existfilePath, newfilePath);
            }
            if (chmod(newfilePath.c_str(), S_IRUSR | S_IWUSR | S_IRGRP) != 0) {
                REQUEST_HILOGD("File add OTH access Failed.");
            }
            if (!PathControl::AddPathsToMap(newfilePath)) {
                REQUEST_HILOGE("Set path permission fail.");
                return false;
            }
        }
    }
    if (!dirs.empty()) {
        dirs.clear();
        dirs.push_back(newPath);
    }
    return true;
}

ExceptionErrorCode RequestAction::CheckFilePath(Config &config)
{
    auto context = AbilityRuntime::Context::GetApplicationContext();
    ExceptionErrorCode ret;
    if (context == nullptr) {
        REQUEST_HILOGE("AppContext is null.");
        return E_FILE_IO;
    }
    if (config.action == Action::DOWNLOAD) {
        ret = CheckDownloadFile(context, config);
        if (ret != ExceptionErrorCode::E_OK) {
            return ret;
        }
    } else {
        ret = CheckUploadFiles(context, config);
        if (ret != ExceptionErrorCode::E_OK) {
            return ret;
        }
        std::string filePath = context->GetCacheDir();
        ret = CheckUploadBodyFiles(filePath, config);
        if (ret != ExceptionErrorCode::E_OK) {
            return ret;
        }
    }
    if (!SetDirsPermission(config.certsPath)) {
        return ExceptionErrorCode::E_FILE_IO;
    }
    return ExceptionErrorCode::E_OK;
}

int32_t RequestAction::Create(TaskBuilder &builder, std::string &tid)
{
    auto ret = builder.build();
    if (ret.second != ExceptionErrorCode::E_OK) {
        return ret.second;
    }
    int32_t err = CheckFilePath(ret.first);
    if (err != ExceptionErrorCode::E_OK) {
        return err;
    }

    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGD("Begin Create, seq: %{public}d", seq);
    err = RequestManager::GetInstance()->Create(ret.first, seq, tid);
    if (err == 0) {
        std::lock_guard<std::mutex> lockGuard(taskMutex_);
        taskMap_.emplace(tid, ret.first);
    }
    return err;
}

ExceptionErrorCode RequestAction::CreateTasks(std::vector<TaskBuilder> &builders, std::vector<TaskRet> &rets)
{
    std::vector<Config> configs;
    size_t len = builders.size();
    rets.resize(len, {
                         .code = ExceptionErrorCode::E_OTHER,
                     });
    for (size_t i = 0; i < len; i++) {
        auto ret = builders[i].build();
        if (ret.second != ExceptionErrorCode::E_OK) {
            rets[i].code = ret.second;
            continue;
        }
        ExceptionErrorCode err = CheckFilePath(ret.first);
        if (err != ExceptionErrorCode::E_OK) {
            rets[i].code = err;
            continue;
        }
        // If config is invalid, do not add it to configs.
        configs.push_back(ret.first);
    }
    PathControl::InsureMapAcl();

    std::vector<TaskRet> temp_rets;
    ExceptionErrorCode ret = RequestManager::GetInstance()->CreateTasks(configs, temp_rets);
    if (ret == ExceptionErrorCode::E_OK) {
        size_t ret_index = 0;
        size_t temp_index = 0;
        std::lock_guard<std::mutex> lockGuard(taskMutex_);
        while (ret_index < len) {
            if (rets[ret_index].code != ExceptionErrorCode::E_OTHER) {
                ++ret_index;
                continue;
            }
            rets[ret_index] = temp_rets[temp_index];
            if (rets[ret_index].code == ExceptionErrorCode::E_OK) {
                taskMap_[rets[ret_index].tid] = configs[temp_index];
            }
            ++ret_index;
            ++temp_index;
        }
    }
    return ret;
}

void RequestAction::RemoveFile(const std::string &filePath)
{
    auto removeFile = [filePath]() -> void {
        std::remove(filePath.c_str());
        return;
    };
    ffrt::submit(removeFile, {}, {}, ffrt::task_attr().name("Os_Request_Rm").qos(ffrt::qos_default));
}

void RequestAction::RemoveDirsPermission(const std::vector<std::string> &dirs)
{
    for (const auto &folderPath : dirs) {
        fs::path folder = folderPath;
        for (const auto &entry : fs::directory_iterator(folder)) {
            fs::path path = entry.path();
            std::string filePath = folder.string() + "/" + path.filename().string();
            PathControl::SubPathsToMap(filePath);
        }
    }
}

bool RequestAction::ClearTaskTemp(const std::string &tid)
{
    Config config;
    {
        std::lock_guard<std::mutex> lockGuard(taskMutex_);
        auto it = taskMap_.find(tid);
        if (it == taskMap_.end()) {
            REQUEST_HILOGD("Clear task tmp files, not in taskMap_");
            return false;
        }
        config = it->second;
        taskMap_.erase(it);
    }

    auto bodyFileNames = config.bodyFileNames;
    for (auto &filePath : bodyFileNames) {
        std::error_code err;
        if (!std::filesystem::exists(filePath, err)) {
            continue;
        }
        err.clear();
        PathControl::SubPathsToMap(filePath);
        RemoveFile(filePath);
    }

    // Reset Acl permission
    for (auto &file : config.files) {
        PathControl::SubPathsToMap(file.uri);
    }

    RemoveDirsPermission(config.certsPath);
    return true;
}

int32_t RequestAction::Remove(const std::string &tid)
{
    RequestAction::ClearTaskTemp(tid);
    return RequestManager::GetInstance()->Remove(tid, Version::API10);
}

ExceptionErrorCode RequestAction::RemoveTasks(
    const std::vector<std::string> &tids, std::unordered_map<std::string, ExceptionErrorCode> &rets)
{
    for (auto &tid : tids) {
        RequestAction::ClearTaskTemp(tid);
    }
    rets.clear();
    std::vector<ExceptionErrorCode> vec;
    ExceptionErrorCode code = RequestManager::GetInstance()->RemoveTasks(tids, Version::API10, vec);
    if (code != ExceptionErrorCode::E_OK) {
        return code;
    }
    uint32_t len = static_cast<uint32_t>(tids.size());
    for (uint32_t i = 0; i < len; i++) {
        rets.insert_or_assign(tids[i], vec[i]);
    }
    return ExceptionErrorCode::E_OK;
}

} // namespace OHOS::Request