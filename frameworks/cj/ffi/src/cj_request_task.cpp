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
#include <sys/stat.h>
#include "securec.h"
#include "application_context.h"
#include "storage_acl.h"
#include "constant.h"
#include "request_manager.h"
#include "cj_app_state_callback.h"
#include "cj_initialize.h"
#include "cj_lambda.h"
#include "cj_request_common.h"
#include "cj_request_event.h"
#include "cj_request_log.h"

namespace OHOS::CJSystemapi::Request {
namespace fs = std::filesystem;
using OHOS::AbilityRuntime::Context;
using OHOS::Request::Version;
using OHOS::Request::ExceptionErrorCode;
using OHOS::Request::Action;
using OHOS::Request::RequestManager;
using OHOS::StorageDaemon::AclSetAccess;

std::mutex CJTask::taskMutex_;
std::map<std::string, CJTask*> CJTask::taskMap_;

std::mutex CJTask::pathMutex_;
std::map<std::string, int32_t> CJTask::pathMap_;

bool CJTask::register_ = false;

static constexpr int ACL_SUCC = 0;
static const std::string SA_PERMISSION_RWX = "g:3815:rwx";
static const std::string SA_PERMISSION_X = "g:3815:x";
static const std::string SA_PERMISSION_CLEAN = "g:3815:---";

CJTask::CJTask()
{
    config_.version = Version::API10;
    config_.action = Action::ANY;
    REQUEST_HILOGI("construct CJTask()");
}

CJTask::~CJTask()
{
    REQUEST_HILOGI("~CJTask()");
    RequestManager::GetInstance()->RemoveAllListeners(GetTidStr());
}

std::string CJTask::GetTidStr() const
{
    return tid_;
}

void CJTask::SetTid()
{
    tid_ = std::to_string(taskId_);
}

void CJTask::AddTaskMap(const std::string &key, CJTask *task)
{
    std::lock_guard<std::mutex> lockGuard(CJTask::taskMutex_);
    CJTask::taskMap_[key] = task;
}

CJTask* CJTask::FindTaskById(int32_t taskId)
{
    CJTask *task = nullptr;
    {
        std::lock_guard<std::mutex> lockGuard(CJTask::taskMutex_);
        auto item = CJTask::taskMap_.find(std::to_string(taskId));
        if (item == CJTask::taskMap_.end()) {
            return nullptr;
        }
        task = item->second;
    }
    return task;
}

CJTask* CJTask::ClearTaskMap(const std::string &key)
{
    std::lock_guard<std::mutex> lockGuard(CJTask::taskMutex_);
    auto it = taskMap_.find(key);
    if (it == taskMap_.end()) {
        return nullptr;
    }
    taskMap_.erase(it);
    return it->second;
}

bool CJTask::SetPathPermission(const std::string &filepath)
{
    std::string baseDir;
    if (!CJInitialize::GetBaseDir(baseDir) || filepath.find(baseDir) == std::string::npos) {
        REQUEST_HILOGE("File dir not found.");
        return false;
    }

    AddPathMap(filepath, baseDir);
    for (auto it : pathMap_) {
        if (it.second <= 0) {
            continue;
        }
        if (AclSetAccess(it.first, SA_PERMISSION_X) != ACL_SUCC) {
            REQUEST_HILOGE("AclSetAccess Parent Dir Failed.");
            return false;
        }
    }
    
    std::string childDir = filepath.substr(0, filepath.rfind("/"));
    if (AclSetAccess(childDir, SA_PERMISSION_RWX) != ACL_SUCC) {
        REQUEST_HILOGE("AclSetAccess Child Dir Failed.");
        return false;
    }
    return true;
}

bool CJTask::SetDirsPermission(std::vector<std::string> &dirs)
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
            if (!CJTask::SetPathPermission(newfilePath)) {
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


void CJTask::AddPathMap(const std::string &filepath, const std::string &baseDir)
{
    std::string childDir(filepath);
    std::string parentDir;
    while (childDir.length() > baseDir.length()) {
        parentDir = childDir.substr(0, childDir.rfind("/"));
        std::lock_guard<std::mutex> lockGuard(CJTask::pathMutex_);
        auto it = pathMap_.find(parentDir);
        if (it == pathMap_.end()) {
            pathMap_[parentDir] = 1;
        } else {
            pathMap_[parentDir] += 1;
        }
        childDir = parentDir;
    }
}

void CJTask::ResetDirAccess(const std::string &filepath)
{
    int ret = AclSetAccess(filepath, SA_PERMISSION_CLEAN);
    if (ret != ACL_SUCC) {
        REQUEST_HILOGE("AclSetAccess Reset Dir Failed: %{public}s", filepath.c_str());
    }
}

void CJTask::RemovePathMap(const std::string &filepath)
{
    std::string baseDir;
    if (!CJInitialize::GetBaseDir(baseDir) || filepath.find(baseDir) == std::string::npos) {
        REQUEST_HILOGE("File dir not found.");
        return;
    }

    if (chmod(filepath.c_str(), S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH) != 0) {
        REQUEST_HILOGE("File remove WOTH access Failed.");
    }

    std::string childDir(filepath);
    std::string parentDir;
    while (childDir.length() > baseDir.length()) {
        parentDir = childDir.substr(0, childDir.rfind("/"));
        std::lock_guard<std::mutex> lockGuard(CJTask::pathMutex_);
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

void CJTask::RemoveDirsPermission(const std::vector<std::string> &dirs)
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

void CJTask::RegisterForegroundResume()
{
    if (register_) {
        return;
    }
    register_ = true;
    auto context = AbilityRuntime::ApplicationContext::GetInstance();
    if (context == nullptr) {
        REQUEST_HILOGE("Get ApplicationContext failed");
        return;
    }
    context->RegisterAbilityLifecycleCallback(std::make_shared<CJAppStateCallback>());
    REQUEST_HILOGD("Register foreground resume callback success");
}

ExceptionError CJTask::Create(Context* context, Config &config)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("Begin task create, seq: %{public}d", seq);
    config_ = config;
    RequestManager::GetInstance()->RestoreListener(CJTask::ReloadListener);
    if (!RequestManager::GetInstance()->LoadRequestServer()) {
        return {.code = ExceptionErrorCode::E_SERVICE_ERROR};
    }

    if (config.mode == Mode::FOREGROUND) {
        RegisterForegroundResume();
    }

    int32_t err = RequestManager::GetInstance()->Create(config_, seq, taskId_);
    if (err != ExceptionErrorCode::E_OK) {
        REQUEST_HILOGE("Create task failed, in");
        return {.code = (ExceptionErrorCode)err};
    }

    SetTid();
    listenerMutex_.lock();
    notifyDataListenerMap_[SubscribeType::REMOVE] =
        std::make_shared<CJNotifyDataListener>(GetTidStr(), SubscribeType::REMOVE);
    listenerMutex_.unlock();
    RequestManager::GetInstance()->AddListener(
        GetTidStr(), SubscribeType::REMOVE, notifyDataListenerMap_[SubscribeType::REMOVE]);

    AddTaskMap(GetTidStr(), this);

    return {.code = ExceptionErrorCode::E_OK};
}

ExceptionError CJTask::Remove(const std::string &tid)
{
    int result = RequestManager::GetInstance()->Remove(tid, Version::API10);
    if (result != ExceptionErrorCode::E_OK) {
        return ConvertError(result);
    }

    return {
        .code = ExceptionErrorCode::E_OK
    };
}

void CJTask::ReloadListener()
{
    REQUEST_HILOGD("ReloadListener in");
    std::lock_guard<std::mutex> lockGuard(CJTask::taskMutex_);
    RequestManager::GetInstance()->ReopenChannel();
    for (const auto &it : taskMap_) {
        RequestManager::GetInstance()->Subscribe(it.first);
    }
}

ExceptionError CJTask::On(std::string type, int32_t taskId, void (*callback)(CProgress progress))
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("Begin task on, seq: %{public}d", seq);

    SubscribeType subscribeType = CJRequestEvent::StringToSubscribeType(type);
    if (subscribeType == SubscribeType::BUTT) {
        return { .code = ExceptionErrorCode::E_PARAMETER_CHECK, .errInfo = "First parameter error" };
    }

    listenerMutex_.lock();
    auto listener = notifyDataListenerMap_.find(subscribeType);
    if (listener == notifyDataListenerMap_.end()) {
        notifyDataListenerMap_[subscribeType] =
            std::make_shared<CJNotifyDataListener>(GetTidStr(), subscribeType);
    }
    listenerMutex_.unlock();
    notifyDataListenerMap_[subscribeType]->AddListener(CJLambda::Create(callback),
        (CFunc)callback);

    REQUEST_HILOGI("End task on event %{public}s successfully, seq: %{public}d, tid: %{public}s", type.c_str(), seq,
        GetTidStr().c_str());

    return {.code = ExceptionErrorCode::E_OK};
}

ExceptionError CJTask::Off(std::string event, CFunc callback)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("Begin task off, seq: %{public}d", seq);

    SubscribeType subscribeType = CJRequestEvent::StringToSubscribeType(event);
    if (subscribeType == SubscribeType::BUTT) {
        return { .code = ExceptionErrorCode::E_PARAMETER_CHECK, .errInfo = "First parameter error" };
    }

    listenerMutex_.lock();
    auto listener = notifyDataListenerMap_.find(subscribeType);
    if (listener == notifyDataListenerMap_.end()) {
        notifyDataListenerMap_[subscribeType] =
            std::make_shared<CJNotifyDataListener>(GetTidStr(), subscribeType);
    }
    listenerMutex_.unlock();
    notifyDataListenerMap_[subscribeType]->RemoveListener((CFunc)callback);

    return {.code = ExceptionErrorCode::E_OK};
}

void CJTask::ClearTaskTemp(const std::string &tid, bool isRmFiles, bool isRmAcls, bool isRmCertsAcls)
{
    std::lock_guard<std::mutex> lockGuard(CJTask::taskMutex_);
    auto item = CJTask::taskMap_.find(tid);
    if (item == CJTask::taskMap_.end()) {
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

} //namespace OHOS::CJSystemapi::Request
