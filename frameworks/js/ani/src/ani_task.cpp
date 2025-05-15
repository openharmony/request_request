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

#include <ani.h>
#include <iostream>
#include <filesystem>
#include "constant.h"
#include "log.h"
#include "ani_js_initialize.h"
#include "ani_utils.h"
#include "ani_task.h"
#include "storage_acl.h"
#include "request_manager.h"

using namespace OHOS::Request;
using namespace OHOS::AniUtil;
using OHOS::StorageDaemon::AclSetAccess;

namespace fs = std::filesystem;
std::mutex AniTask::pathMutex_;
std::map<std::string, int32_t> AniTask::pathMap_;
std::map<std::string, int32_t> AniTask::fileMap_;

static constexpr int ACL_SUCC = 0;
static const std::string SA_PERMISSION_RWX = "g:3815:rwx";
static const std::string SA_PERMISSION_X = "g:3815:x";
static const std::string SA_PERMISSION_CLEAN = "g:3815:---";

static constexpr const char *EVENT_COMPLETED = "completed";
static constexpr const char *EVENT_COMPLETE = "complete";
static constexpr const char *EVENT_RESPONSE = "response";
static constexpr const char *EVENT_REMOVE = "remove";

std::map<std::string, SubscribeType> AniTask::supportEventsAni_ = {
    { EVENT_COMPLETE, SubscribeType::COMPLETED },
    { EVENT_COMPLETED, SubscribeType::COMPLETED },
    { EVENT_REMOVE, SubscribeType::REMOVE },
    { EVENT_RESPONSE, SubscribeType::RESPONSE },
};

AniTask* AniTask::Create([[maybe_unused]] ani_env* env, Config config)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("AniTask::Create: seq: %{public}d", seq);
    RequestManager::GetInstance()->LoadRequestServer();

    std::string tid = "";
    int32_t ret = RequestManager::GetInstance()->Create(config, seq, tid);
    REQUEST_HILOGI("Create return: tid: [%{public}s]", tid.c_str());
    if (ret != E_OK) {
        REQUEST_HILOGE("End create task in Create, seq: %{public}d, failed: %{public}d", seq, ret);
        return new AniTask(tid);
    }

    ani_vm *vm = nullptr;
    env->GetVM(&vm);
    auto notifyDataListener = std::make_shared<NotifyDataListener>(vm, tid, SubscribeType::REMOVE);
    RequestManager::GetInstance()->AddListener(tid, SubscribeType::REMOVE, notifyDataListener);

    return new AniTask(tid);
}

void AniTask::Start()
{
    REQUEST_HILOGI("Enter AniTask::Start");

    int32_t ret = RequestManager::GetInstance()->Start(tid_);
    if (ret == E_OK) {
        REQUEST_HILOGI("AniTask::Start success");
    }
    REQUEST_HILOGI("AniTask::Start end");
}

void NotifyDataListener::OnNotifyDataReceive(const std::shared_ptr<NotifyData> &notifyData)
{
    REQUEST_HILOGI("OnNotifyDataReceive enter");
    ani_env *workerEnv = nullptr;
    ani_options aniArgs {0, nullptr};
    auto status = vm_->AttachCurrentThread(&aniArgs, ANI_VERSION_1, &workerEnv);
    if (status == ANI_ERROR) {
        status = vm_->GetEnv(ANI_VERSION_1, &workerEnv);
    }

    AniLocalScopeGuard guard(workerEnv, 0X16);
    if (workerEnv == nullptr) {
        REQUEST_HILOGE("%{public}s: env_ == nullptr.", __func__);
        return;
    }
    ani_object Progress = AniObjectUtils::Create(workerEnv, "L@ohos/request/request;", "Lagent;", "LProgressImpl;",
        static_cast<ani_double>(notifyData->progress.state), static_cast<ani_double>(notifyData->progress.index),
        static_cast<ani_double>(notifyData->progress.processed));
    std::vector<ani_ref> args = {Progress};
    
    OnMessageReceive(workerEnv, args);
    status = vm_->DetachCurrentThread();
    REQUEST_HILOGI("OnNotifyDataReceive end");
}

void NotifyDataListener::AddListener(ani_ref &callback)
{
    AddListenerInner(callback);
    
    if (this->validCbNum == 1) {
        RequestManager::GetInstance()->AddListener(this->tid_, this->type_, shared_from_this());
    }
}

void ResponseListener::OnResponseReceive(const std::shared_ptr<Response> &response)
{
    REQUEST_HILOGI("OnResponseReceive enter");
    ani_env *workerEnv = nullptr;
    ani_options aniArgs {0, nullptr};
    auto status = vm_->AttachCurrentThread(&aniArgs, ANI_VERSION_1, &workerEnv);
    if (status == ANI_ERROR) {
        status = vm_->GetEnv(ANI_VERSION_1, &workerEnv);
    }

    AniLocalScopeGuard guard(workerEnv, 0X16);
    if (workerEnv == nullptr) {
        REQUEST_HILOGE("%{public}s: env_ == nullptr.", __func__);
        return;
    }
    ani_object httpResponse = AniObjectUtils::Create(workerEnv, "L@ohos/request/request;", "Lagent;",
        "LHttpResponseImpl;", AniStringUtils::ToAni(workerEnv, response->version),
        static_cast<ani_double>(response->statusCode), AniStringUtils::ToAni(workerEnv, response->reason));
    std::vector<ani_ref> args = {httpResponse};
    OnMessageReceive(workerEnv, args);
    status = vm_->DetachCurrentThread();
}

void ResponseListener::AddListener(ani_ref &callback)
{
    AddListenerInner(callback);
    if (this->validCbNum == 1 && this->type_ != SubscribeType::REMOVE) {
        RequestManager::GetInstance()->AddListener(this->tid_, this->type_, shared_from_this());
    }
}

void AniTask::On([[maybe_unused]] ani_env* env, std::string event, ani_ref callback)
{
    REQUEST_HILOGI("Enter AniTask::On %{public}s", event.c_str());
    if (supportEventsAni_.find(event) == supportEventsAni_.end()) {
        REQUEST_HILOGE("event not find!");
        return;
    }
    ani_vm *vm = nullptr;
    env->GetVM(&vm);
    this->type_ = supportEventsAni_[event];

    if (this->type_ == SubscribeType::RESPONSE) {
        if (responseListener_ == nullptr) {
            responseListener_ = std::make_shared<ResponseListener>(vm, this->tid_, this->type_);
        }
        responseListener_->AddListener(callback);
    } else {
        if (notifyDataListenerMap_.find(this->type_) == notifyDataListenerMap_.end()) {
            notifyDataListenerMap_[this->type_] = std::make_shared<NotifyDataListener>(vm, this->tid_, this->type_);
        }
        notifyDataListenerMap_[this->type_]->AddListener(callback);
    }
    REQUEST_HILOGI("End AniTask::On");
}


bool AniTask::SetDirsPermission(std::vector<std::string> &dirs)
{
    if (dirs.empty()) {
        return true;
    }
    std::string newPath = "/data/storage/el2/base/.ohos/.request/.certs";
    std::vector<std::string> dirElems;
    JsInitialize::StringSplit(newPath, '/', dirElems);
    if (!JsInitialize::CreateDirs(dirElems)) {
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
            if (!AniTask::SetPathPermission(newfilePath)) {
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

bool AniTask::SetPathPermission(const std::string &filepath)
{
    std::string baseDir;
    if (!JsInitialize::CheckBelongAppBaseDir(filepath, baseDir)) {
        return false;
    }

    AddPathMap(filepath, baseDir);
    {
        std::lock_guard<std::mutex> lockGuard(AniTask::pathMutex_);
        for (auto it : pathMap_) {
            if (it.second <= 0) {
                continue;
            }
            if (AclSetAccess(it.first, SA_PERMISSION_X) != ACL_SUCC) {
                REQUEST_HILOGD("AclSetAccess Parent Dir Failed: %{public}s", it.first.c_str());
            }
        }
    }

    std::string childDir = filepath.substr(0, filepath.rfind("/"));
    if (AclSetAccess(childDir, SA_PERMISSION_RWX) != ACL_SUCC) {
        REQUEST_HILOGE("AclSetAccess Child Dir Failed: %{public}s", childDir.c_str());
        return false;
    }
    return true;
}

void AniTask::AddPathMap(const std::string &filepath, const std::string &baseDir)
{
    {
        std::lock_guard<std::mutex> lockGuard(AniTask::pathMutex_);
        auto it = fileMap_.find(filepath);
        if (it == fileMap_.end()) {
            fileMap_[filepath] = 1;
        } else {
            fileMap_[filepath] += 1;
        }
    }

    std::string childDir(filepath);
    std::string parentDir;
    while (childDir.length() > baseDir.length()) {
        parentDir = childDir.substr(0, childDir.rfind("/"));
        std::lock_guard<std::mutex> lockGuard(AniTask::pathMutex_);
        auto it = pathMap_.find(parentDir);
        if (it == pathMap_.end()) {
            pathMap_[parentDir] = 1;
        } else {
            pathMap_[parentDir] += 1;
        }
        childDir = parentDir;
    }
}

void AniTask::ResetDirAccess(const std::string &filepath)
{
    int ret = AclSetAccess(filepath, SA_PERMISSION_CLEAN);
    if (ret != ACL_SUCC) {
        REQUEST_HILOGD("AclSetAccess Reset Dir Failed: %{public}s", filepath.c_str());
    }
}

void AniTask::RemovePathMap(const std::string &filepath)
{
    std::string baseDir;
    if (!JsInitialize::CheckBelongAppBaseDir(filepath, baseDir)) {
        return;
    }

    {
        std::lock_guard<std::mutex> lockGuard(AniTask::pathMutex_);
        auto it = fileMap_.find(filepath);
        if (it != fileMap_.end()) {
            if (fileMap_[filepath] <= 1) {
                fileMap_.erase(filepath);
                if (chmod(filepath.c_str(), S_IRUSR | S_IWUSR | S_IRGRP) != 0) {
                    REQUEST_HILOGE("File remove OTH access Failed: %{public}s", filepath.c_str());
                }
            } else {
                fileMap_[filepath] -= 1;
            }
        } else {
            return;
        }
    }

    std::string childDir(filepath);
    std::string parentDir;
    while (childDir.length() > baseDir.length()) {
        parentDir = childDir.substr(0, childDir.rfind("/"));
        std::lock_guard<std::mutex> lockGuard(AniTask::pathMutex_);
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

void AniTask::RemoveDirsPermission(const std::vector<std::string> &dirs)
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