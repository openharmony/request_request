/*
 * Copyright (c) 2024 Huawei Device Co., Ltd.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "cj_request_task.h"
#include <cstring>
#include <fcntl.h>
#include <filesystem>
#include <fstream>
#include <regex>
#include <string>
#include <sys/stat.h>
#include "application_context.h"
#include "cj_app_state_callback.h"
#include "cj_application_context.h"
#include "cj_initialize.h"
#include "cj_lambda.h"
#include "cj_request_common.h"
#include "cj_request_event.h"
#include "cj_response_listener.h"
#include "constant.h"
#include "request_common.h"
#include "log.h"
#include "request_manager.h"
#include "securec.h"
#include "storage_acl.h"

namespace OHOS::CJSystemapi::Request {
namespace fs = std::filesystem;
using OHOS::AbilityRuntime::Context;
using OHOS::Request::Action;
using OHOS::Request::ExceptionErrorCode;
using OHOS::Request::RequestManager;
using OHOS::Request::TaskInfo;
using OHOS::Request::Version;
using OHOS::StorageDaemon::AclSetAccess;

std::mutex CJRequestTask::taskMutex_;
std::map<std::string, CJRequestTask *> CJRequestTask::taskMap_;

std::mutex CJRequestTask::pathMutex_;
std::map<std::string, int32_t> CJRequestTask::pathMap_;

bool CJRequestTask::register_ = false;

static constexpr int ACL_SUCC = 0;
static const std::string SA_PERMISSION_RWX = "g:3815:rwx";
static const std::string SA_PERMISSION_X = "g:3815:x";
static const std::string SA_PERMISSION_CLEAN = "g:3815:---";

CJRequestTask::CJRequestTask()
{
    config_.version = Version::API10;
    config_.action = Action::ANY;
    REQUEST_HILOGD("construct CJRequestTask()");
}

CJRequestTask::~CJRequestTask()
{
    REQUEST_HILOGD("~CJRequestTask()");
    RequestManager::GetInstance()->RemoveAllListeners(GetTidStr());
}

std::string CJRequestTask::GetTidStr() const
{
    return tid_;
}

void CJRequestTask::SetTid()
{
    tid_ = taskId_;
}

void CJRequestTask::AddTaskMap(const std::string &key, CJRequestTask *task)
{
    std::lock_guard<std::mutex> lockGuard(CJRequestTask::taskMutex_);
    CJRequestTask::taskMap_[key] = task;
}

CJRequestTask *CJRequestTask::FindTaskById(std::string &taskId)
{
    CJRequestTask *task = nullptr;
    {
        std::lock_guard<std::mutex> lockGuard(CJRequestTask::taskMutex_);
        auto item = CJRequestTask::taskMap_.find(taskId);
        if (item == CJRequestTask::taskMap_.end()) {
            return nullptr;
        }
        task = item->second;
    }
    return task;
}

CJRequestTask *CJRequestTask::ClearTaskMap(const std::string &key)
{
    std::lock_guard<std::mutex> lockGuard(CJRequestTask::taskMutex_);
    auto it = taskMap_.find(key);
    if (it == taskMap_.end()) {
        return nullptr;
    }
    taskMap_.erase(it);
    return it->second;
}

bool CJRequestTask::SetPathPermission(const std::string &filepath)
{
    std::string baseDir;
    if (CheckApiVersionAfter19()) {
        if (!CJInitialize::CheckBelongAppBaseDir(filepath, baseDir)) {
            return false;
        }
    } else {
        if (!CJInitialize::GetBaseDir(baseDir) || filepath.find(baseDir) == std::string::npos) {
            REQUEST_HILOGE("File dir not found.");
            return false;
        }
    }

    AddPathMap(filepath, baseDir);
    {
        std::lock_guard<std::mutex> lockGuard(pathMutex_);
        for (auto it : pathMap_) {
            if (it.second <= 0) {
                continue;
            }
            if (AclSetAccess(it.first, SA_PERMISSION_X) != ACL_SUCC) {
                REQUEST_HILOGE("AclSetAccess Parent Dir Failed.");
            }
        }
    }

    if (AclSetAccess(filepath, SA_PERMISSION_RWX) != ACL_SUCC) {
        REQUEST_HILOGE("AclSetAccess Child Dir Failed.");
        return false;
    }
    return true;
}

bool CJRequestTask::SetDirsPermission(std::vector<std::string> &dirs)
{
    if (dirs.empty()) {
        return true;
    }
    std::string newPath = "/data/storage/el2/base/.ohos/.request/.certs";
    std::vector<std::string> dirElems;
    CJInitialize::StringSplit(newPath, '/', dirElems);
    if (!CJInitialize::CreateDirs(dirElems)) {
        REQUEST_HILOGE("CreateDirs Err: %{public}s", newPath.c_str());
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
            if (chmod(newfilePath.c_str(), S_IRWXU | S_IRWXG | S_IROTH | S_IXOTH) != 0) {
                REQUEST_HILOGD("File add OTH access Failed.");
            }
            REQUEST_HILOGD("current filePath is %{public}s", newfilePath.c_str());
            if (!CJRequestTask::SetPathPermission(newfilePath)) {
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

void CJRequestTask::AddPathMap(const std::string &filepath, const std::string &baseDir)
{
    std::string childDir(filepath);
    std::string parentDir;
    while (childDir.length() > baseDir.length()) {
        parentDir = childDir.substr(0, childDir.rfind("/"));
        std::lock_guard<std::mutex> lockGuard(CJRequestTask::pathMutex_);
        auto it = pathMap_.find(parentDir);
        if (it == pathMap_.end()) {
            pathMap_[parentDir] = 1;
        } else {
            pathMap_[parentDir] += 1;
        }
        childDir = parentDir;
    }
}

void CJRequestTask::ResetDirAccess(const std::string &filepath)
{
    int ret = AclSetAccess(filepath, SA_PERMISSION_CLEAN);
    if (ret != ACL_SUCC) {
        REQUEST_HILOGE("AclSetAccess Reset Dir Failed: %{public}s", filepath.c_str());
    }
}

void CJRequestTask::RemovePathMap(const std::string &filepath)
{
    std::string baseDir;
    if (!CJInitialize::GetBaseDir(baseDir) || filepath.find(baseDir) == std::string::npos) {
        REQUEST_HILOGE("File dir not found.");
        return;
    }

    if (chmod(filepath.c_str(), S_IRUSR | S_IWUSR | S_IRGRP) != 0) {
        REQUEST_HILOGE("File remove WOTH access Failed.");
    }

    std::string childDir(filepath);
    std::string parentDir;
    while (childDir.length() > baseDir.length()) {
        parentDir = childDir.substr(0, childDir.rfind("/"));
        std::lock_guard<std::mutex> lockGuard(CJRequestTask::pathMutex_);
        auto it = pathMap_.find(parentDir);
        if (it != pathMap_.end()) {
            if (pathMap_[parentDir] <= 1) {
                pathMap_.erase(parentDir);
                ResetDirAccess(parentDir);
            } else {
                pathMap_[parentDir] -= 1;
            }
        }
        childDir = parentDir;
    }
}

void CJRequestTask::RemoveDirsPermission(const std::vector<std::string> &dirs)
{
    for (const auto &folderPath : dirs) {
        fs::path folder = folderPath;
        for (const auto &entry : fs::directory_iterator(folder)) {
            fs::path path = entry.path();
            std::string filePath = folder.string() + "/" + path.filename().string();
            RemovePathMap(filePath);
        }
    }
}

void CJRequestTask::RegisterForegroundResume()
{
    if (register_) {
        return;
    }
    register_ = true;
    auto context = ApplicationContextCJ::CJApplicationContext::GetInstance();
    if (context == nullptr) {
        REQUEST_HILOGE("Get ApplicationContext failed");
        return;
    }
    context->RegisterAbilityLifecycleCallback(std::make_shared<CJAppStateCallback>());
    REQUEST_HILOGD("Register foreground resume callback success");
}

ExceptionError CJRequestTask::Create(Context *context, Config &config)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("Begin task create, seq: %{public}d", seq);
    config_ = config;
    ExceptionError err;
    RequestManager::GetInstance()->RestoreListener(CJRequestTask::ReloadListener);

    if (config.mode == Mode::FOREGROUND) {
        RegisterForegroundResume();
    }

    int32_t ret = RequestManager::GetInstance()->Create(config_, seq, taskId_);
    if (ret != ExceptionErrorCode::E_OK) {
        REQUEST_HILOGE("Create task failed, in");
        err.code = static_cast<ExceptionErrorCode>(ret);
        return err;
    }

    SetTid();
    {
        std::unique_lock<std::recursive_mutex> lock(listenerMutex_);
        notifyDataListenerMap_[SubscribeType::REMOVE] =
            std::make_shared<CJNotifyDataListener>(GetTidStr(), SubscribeType::REMOVE);
        RequestManager::GetInstance()->AddListener(GetTidStr(), SubscribeType::REMOVE,
            notifyDataListenerMap_[SubscribeType::REMOVE]);
    }
    AddTaskMap(GetTidStr(), this);

    return err;
}

ExceptionError CJRequestTask::GetTask(OHOS::AbilityRuntime::Context *context, std::string &taskId, std::string &token,
                                      Config &config)
{
    ExceptionError err;
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("Begin get task, seq: %{public}d", seq);

    CJRequestTask *task = CJRequestTask::FindTaskById(taskId);
    if (task != nullptr) {
        if (task->config_.token != token) {
            return ConvertError(ExceptionErrorCode::E_TASK_NOT_FOUND);
        }
        config = task->config_;
        return err;
    }

    int32_t result = RequestManager::GetInstance()->GetTask(taskId, token, config);
    if (result != ExceptionErrorCode::E_OK) {
        return ConvertError(result);
    }
    return err;
}

ExceptionError CJRequestTask::Remove(const std::string &tid)
{
    int32_t result = RequestManager::GetInstance()->Remove(tid, Version::API10);
    if (result != ExceptionErrorCode::E_OK) {
        return ConvertError(result);
    }

    return ExceptionError();
}

ExceptionError CJRequestTask::Touch(const std::string &tid, TaskInfo &task, const std::string &token)
{
    ExceptionError err;

    int32_t result = RequestManager::GetInstance()->Touch(tid, token, task);
    if (result != ExceptionErrorCode::E_OK) {
        return ConvertError(result);
    }
    return err;
}

ExceptionError CJRequestTask::Search(const Filter &filter, std::vector<std::string> &tids)
{
    ExceptionError err;

    int32_t result = RequestManager::GetInstance()->Search(filter, tids);
    if (result != ExceptionErrorCode::E_OK) {
        return ConvertError(result);
    }

    return err;
}

void CJRequestTask::ReloadListener()
{
    REQUEST_HILOGD("ReloadListener in");
    std::lock_guard<std::mutex> lockGuard(CJRequestTask::taskMutex_);
    RequestManager::GetInstance()->ReopenChannel();
    for (const auto &it : taskMap_) {
        RequestManager::GetInstance()->Subscribe(it.first);
    }
}

ExceptionError CJRequestTask::On(std::string type, std::string &taskId, void *callback)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("Begin task on, seq: %{public}d", seq);

    ExceptionError err;
    SubscribeType subscribeType = CJRequestEvent::StringToSubscribeType(type);
    if (subscribeType == SubscribeType::BUTT) {
        err.code = ExceptionErrorCode::E_PARAMETER_CHECK;
        err.errInfo = "First parameter error";
        return err;
    }

    if (subscribeType == SubscribeType::RESPONSE) {
        {
            std::unique_lock<std::recursive_mutex> lock(listenerMutex_);
            if (responseListener_ == nullptr) {
                responseListener_ = std::make_shared<CJResponseListener>(GetTidStr());
            }
        }
        responseListener_->AddListener(CJLambda::Create((void (*)(CResponse progress))callback), callback);
    } else {
        std::unique_lock<std::recursive_mutex> lock(listenerMutex_);
        auto listener = notifyDataListenerMap_.find(subscribeType);
        if (listener == notifyDataListenerMap_.end()) {
            notifyDataListenerMap_[subscribeType] = std::make_shared<CJNotifyDataListener>(GetTidStr(), subscribeType);
        }
        notifyDataListenerMap_[subscribeType]->AddListener(CJLambda::Create((void (*)(CProgress progress))callback),
            (CFunc)callback);
    }

    REQUEST_HILOGI("End task on event %{public}s successfully, seq: %{public}d, tid: %{public}s", type.c_str(), seq,
        GetTidStr().c_str());

    return err;
}

ExceptionError CJRequestTask::Off(std::string event, CFunc callback)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("Begin task off, seq: %{public}d", seq);

    ExceptionError err;
    SubscribeType subscribeType = CJRequestEvent::StringToSubscribeType(event);
    if (subscribeType == SubscribeType::BUTT) {
        err.code = ExceptionErrorCode::E_PARAMETER_CHECK;
        err.errInfo = "First parameter error";
        return err;
    }

    if (subscribeType == SubscribeType::RESPONSE) {
        {
            std::unique_lock<std::recursive_mutex> lock(listenerMutex_);
            if (responseListener_ == nullptr) {
                responseListener_ = std::make_shared<CJResponseListener>(GetTidStr());
            }
        }
        responseListener_->RemoveListener((CFunc)callback);
    } else {
        std::unique_lock<std::recursive_mutex> lock(listenerMutex_);
        if (notifyDataListenerMap_.find(subscribeType) == notifyDataListenerMap_.end()) {
            notifyDataListenerMap_[subscribeType] = std::make_shared<CJNotifyDataListener>(GetTidStr(), subscribeType);
        }
        notifyDataListenerMap_[subscribeType]->RemoveListener((CFunc)callback);
    }
    return err;
}

void CJRequestTask::ClearTaskTemp(const std::string &tid, bool isRmFiles, bool isRmAcls, bool isRmCertsAcls)
{
    std::lock_guard<std::mutex> lockGuard(CJRequestTask::taskMutex_);
    auto item = CJRequestTask::taskMap_.find(tid);
    if (item == CJRequestTask::taskMap_.end()) {
        REQUEST_HILOGD("Clear task tmp files, not find task");
        return;
    }
    auto task = item->second;
    if (isRmFiles) {
        auto bodyFileNames = task->config_.bodyFileNames;
        for (auto &filePath : bodyFileNames) {
            RemovePathMap(filePath);
            RemoveFile(filePath);
        }
    }
    if (isRmAcls) {
        // Reset Acl permission
        for (auto &file : task->config_.files) {
            RemovePathMap(file.uri);
        }
    }
    if (isRmCertsAcls) {
        RemoveDirsPermission(task->config_.certsPath);
    }
}

} // namespace OHOS::CJSystemapi::Request