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

#ifndef OHOS_REQUEST_ACTION_H
#define OHOS_REQUEST_ACTION_H

#include "constant.h"
#include "context.h"
#include "request_common.h"
#include "request_manager.h"
#include "task_builder.h"

namespace OHOS::Request {

static const std::string DOWNLOAD_PERMISSION = "ohos.permission.DOWNLOAD_SESSION_MANAGER";
static const std::string UPLOAD_PERMISSION = "ohos.permission.UPLOAD_SESSION_MANAGER";

class RequestAction {
public:
    static const std::unique_ptr<RequestAction> &GetInstance();
    int32_t Create(TaskBuilder &builder, std::string &tid);
    int32_t Start(const std::string &tid);
    int32_t Stop(const std::string &tid);
    int32_t Touch(const std::string &tid, const std::string &token, TaskInfo &info);
    int32_t Show(const std::string &tid, TaskInfo &info);
    int32_t Pause(const std::string &tid);
    int32_t Remove(const std::string &tid);
    int32_t Resume(const std::string &tid);
    int32_t SetMaxSpeed(const std::string &tid, const int64_t maxSpeed);

    ExceptionErrorCode CreateTasks(std::vector<TaskBuilder> &builders, std::vector<TaskRet> &rets);
    ExceptionErrorCode StartTasks(
        const std::vector<std::string> &tids, std::unordered_map<std::string, ExceptionErrorCode> &rets);
    ExceptionErrorCode StopTasks(
        const std::vector<std::string> &tids, std::unordered_map<std::string, ExceptionErrorCode> &rets);
    ExceptionErrorCode ResumeTasks(
        const std::vector<std::string> &tids, std::unordered_map<std::string, ExceptionErrorCode> &rets);
    ExceptionErrorCode RemoveTasks(
        const std::vector<std::string> &tids, std::unordered_map<std::string, ExceptionErrorCode> &rets);
    ExceptionErrorCode PauseTasks(
        const std::vector<std::string> &tids, std::unordered_map<std::string, ExceptionErrorCode> &rets);
    ExceptionErrorCode ShowTasks(
        const std::vector<std::string> &tids, std::unordered_map<std::string, TaskInfoRet> &rets);
    ExceptionErrorCode TouchTasks(
        const std::vector<TaskIdAndToken> &tidTokens, std::unordered_map<std::string, TaskInfoRet> &rets);
    ExceptionErrorCode SetMaxSpeeds(
        const std::vector<SpeedConfig> &speedConfig, std::unordered_map<std::string, ExceptionErrorCode> &rets);
    ExceptionErrorCode SetMode(std::string &tid, Mode mode);
    ExceptionErrorCode DisableTaskNotification(
        const std::vector<std::string> &tids, std::unordered_map<std::string, ExceptionErrorCode> &rets);

private:
    static bool CreateDirs(const std::vector<std::string> &pathDirs);
    static bool FileToWhole(
        const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config, std::string &path);
    static bool BaseToWhole(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, std::string &path);
    static bool CacheToWhole(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, std::string &path);
    static bool StandardizePath(
        const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config, std::string &path);
    static void StringSplit(const std::string &str, const char delim, std::vector<std::string> &elems);
    static bool PathVecToNormal(const std::vector<std::string> &in, std::vector<std::string> &out);
    static bool WholeToNormal(std::string &path, std::vector<std::string> &out);
    static bool GetAppBaseDir(std::string &baseDir);
    static bool CheckBelongAppBaseDir(const std::string &filepath, std::string &baseDir);
    static bool FindAreaPath(const std::string &filepath);
    static bool GetSandboxPath(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config,
        std::string &path, std::vector<std::string> &pathVec);
    static bool CheckDownloadFilePath(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config);
    static bool InterceptData(const std::string &str, const std::string &in, std::string &out);
    static void StandardizeFileSpec(FileSpec &file);
    static bool IsPathValid(const std::string &filePath);
    static bool GetInternalPath(
        const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config, std::string &path);
    static bool FindDir(const std::string &pathDir);
    static ExceptionErrorCode GetFdDownload(const std::string &path, const Config &config);
    static ExceptionErrorCode CheckDownloadFile(
        const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config);
    static bool IsUserFile(const std::string &path);
    static ExceptionErrorCode CheckUserFileSpec(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context,
        const Config &config, FileSpec &file, bool isUpload);
    static bool CheckPathIsFile(const std::string &path);
    static ExceptionErrorCode GetFdUpload(const std::string &path, const Config &config);
    static ExceptionErrorCode CheckUploadFileSpec(
        const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config, FileSpec &file);
    static ExceptionErrorCode CheckUploadFiles(
        const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config);
    static ExceptionErrorCode CheckUploadBodyFiles(const std::string &filePath, Config &config);
    static bool SetDirsPermission(std::vector<std::string> &dirs);
    static ExceptionErrorCode CheckFilePath(Config &config);
    static void RemoveFile(const std::string &filePath);
    static void RemoveDirsPermission(const std::vector<std::string> &dirs);
    static bool ClearTaskTemp(const std::string &tid);
};

} // namespace OHOS::Request
#endif // OHOS_REQUEST_ACTION_H