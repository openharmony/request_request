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

/**
 * @brief External action execution entry of the request service (singleton).
 *
 * After permission verification and path standardization, it forwards single
 * and batch task operations from the upper layer to RequestManager for
 * execution. It is the unified action contract exposed by inner_kits.
 */
class RequestAction {
public:
    /**
     * @brief Get the reference to the RequestAction singleton.
     * @return Const reference to the singleton, unique within the process.
     */
    static const std::unique_ptr<RequestAction> &GetInstance();
    /**
     * @brief Create a single task and return its task ID.
     * @param builder Task builder with the configuration already assembled.
     * @param tid [out] Outputs the unique identifier of the newly created task.
     * @return 0 on success, other values are error codes.
     */
    int32_t Create(TaskBuilder &builder, std::string &tid);
    /**
     * @brief Start the specified task.
     * @param tid Target task ID.
     * @return 0 on success, other values are error codes.
     */
    int32_t Start(const std::string &tid);
    /**
     * @brief Stop the specified task (stop transfer but keep the task record).
     * @param tid Target task ID.
     * @return 0 on success, other values are error codes.
     */
    int32_t Stop(const std::string &tid);
    /**
     * @brief Query task information and verify the access token.
     * @param tid Target task ID.
     * @param token Access token held by the caller, used for authentication.
     * @param info [out] Outputs the current task information.
     * @return 0 on success, other values are error codes.
     */
    int32_t Touch(const std::string &tid, const std::string &token, TaskInfo &info);
    /**
     * @brief Query task information (without token verification).
     * @param tid Target task ID.
     * @param info [out] Outputs the current task information.
     * @return 0 on success, other values are error codes.
     */
    int32_t Show(const std::string &tid, TaskInfo &info);
    /**
     * @brief Pause the specified task.
     * @param tid Target task ID.
     * @return 0 on success, other values are error codes.
     */
    int32_t Pause(const std::string &tid);
    /**
     * @brief Remove the specified task and its associated resources.
     * @param tid Target task ID.
     * @return 0 on success, other values are error codes.
     */
    int32_t Remove(const std::string &tid);
    /**
     * @brief Resume a paused or stopped task.
     * @param tid Target task ID.
     * @return 0 on success, other values are error codes.
     */
    int32_t Resume(const std::string &tid);
    /**
     * @brief Set the maximum transfer rate of a single task.
     * @param tid Target task ID.
     * @param maxSpeed Maximum rate in bytes per second; 0 means no limit.
     * @return 0 on success, other values are error codes.
     */
    int32_t SetMaxSpeed(const std::string &tid, const int64_t maxSpeed);

    /**
     * @brief Create tasks in batch.
     * @param builders Collection of task builders to be created.
     * @param rets [out] Outputs the creation result of each task.
     * @return Overall error code; per-task results are in rets.
     */
    ExceptionErrorCode CreateTasks(std::vector<TaskBuilder> &builders, std::vector<TaskRet> &rets);
    /**
     * @brief Start tasks in batch.
     * @param tids Collection of task IDs to be started.
     * @param rets [out] Outputs the error code for each task ID.
     * @return Overall error code.
     */
    ExceptionErrorCode StartTasks(
        const std::vector<std::string> &tids, std::unordered_map<std::string, ExceptionErrorCode> &rets);
    /**
     * @brief Stop tasks in batch.
     * @param tids Collection of task IDs to be stopped.
     * @param rets [out] Outputs the error code for each task ID.
     * @return Overall error code.
     */
    ExceptionErrorCode StopTasks(
        const std::vector<std::string> &tids, std::unordered_map<std::string, ExceptionErrorCode> &rets);
    /**
     * @brief Resume tasks in batch.
     * @param tids Collection of task IDs to be resumed.
     * @param rets [out] Outputs the error code for each task ID.
     * @return Overall error code.
     */
    ExceptionErrorCode ResumeTasks(
        const std::vector<std::string> &tids, std::unordered_map<std::string, ExceptionErrorCode> &rets);
    /**
     * @brief Remove tasks in batch.
     * @param tids Collection of task IDs to be removed.
     * @param rets [out] Outputs the error code for each task ID.
     * @return Overall error code.
     */
    ExceptionErrorCode RemoveTasks(
        const std::vector<std::string> &tids, std::unordered_map<std::string, ExceptionErrorCode> &rets);
    /**
     * @brief Pause tasks in batch.
     * @param tids Collection of task IDs to be paused.
     * @param rets [out] Outputs the error code for each task ID.
     * @return Overall error code.
     */
    ExceptionErrorCode PauseTasks(
        const std::vector<std::string> &tids, std::unordered_map<std::string, ExceptionErrorCode> &rets);
    /**
     * @brief Query task information in batch.
     * @param tids Collection of task IDs to be queried.
     * @param rets [out] Outputs the task information result for each task ID.
     * @return Overall error code.
     */
    ExceptionErrorCode ShowTasks(
        const std::vector<std::string> &tids, std::unordered_map<std::string, TaskInfoRet> &rets);
    /**
     * @brief Query task information in batch and verify access tokens.
     * @param tidTokens Collection of task ID and token pairs to be queried.
     * @param rets [out] Outputs the task information result for each task ID.
     * @return Overall error code.
     */
    ExceptionErrorCode TouchTasks(
        const std::vector<TaskIdAndToken> &tidTokens, std::unordered_map<std::string, TaskInfoRet> &rets);
    /**
     * @brief Set the maximum transfer rate of tasks in batch.
     * @param speedConfig Collection of rate configs for tasks to be rate-limited.
     * @param rets [out] Outputs the error code for each task ID.
     * @return Overall error code.
     */
    ExceptionErrorCode SetMaxSpeeds(
        const std::vector<SpeedConfig> &speedConfig, std::unordered_map<std::string, ExceptionErrorCode> &rets);
    /**
     * @brief Set the running mode of a task (foreground/background, etc.).
     * @param tid Target task ID.
     * @param mode Target running mode.
     * @return Overall error code.
     */
    ExceptionErrorCode SetMode(std::string &tid, Mode mode);
    /**
     * @brief Disable task notifications in batch.
     * @param tids Collection of task IDs whose notifications are to be disabled.
     * @param rets [out] Outputs the error code for each task ID.
     * @return Overall error code.
     */
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