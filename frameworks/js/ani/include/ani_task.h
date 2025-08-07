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

#ifndef ANI_TASK_H
#define ANI_TASK_H

#include <ani.h>
#include "i_response_listener.h"
#include "i_notify_data_listener.h"
#include "listener_list.h"

namespace OHOS::Request {

class ResponseListener :
    public IResponseListener,
    public std::enable_shared_from_this<ResponseListener>,
    public ListenerList {
public:
    virtual ~ResponseListener() = default;
    ResponseListener(ani_vm* vm, std::string tid, SubscribeType type) : vm_(vm), tid_(tid), type_(type)
    {
    }

    virtual void OnResponseReceive(const std::shared_ptr<Response> &response);
    void AddListener(ani_ref &callback);

private:
    ani_vm *vm_ = nullptr;
    std::string tid_ = "";
    SubscribeType type_ = SubscribeType::FAILED;
};

class NotifyDataListener :
    public INotifyDataListener,
    public std::enable_shared_from_this<NotifyDataListener>,
    public ListenerList {
public:
    virtual ~NotifyDataListener() = default;
    NotifyDataListener(ani_vm *vm, std::string tid, SubscribeType type) : vm_(vm), tid_(tid), type_(type)
    {
    }

    virtual void OnNotifyDataReceive(const std::shared_ptr<NotifyData> &notifyData);
    virtual void OnFaultsReceive(const std::shared_ptr<int32_t> &tid, const std::shared_ptr<SubscribeType> &type,
        const std::shared_ptr<Reason> &reason)
    {
    }
    virtual void OnWaitReceive(std::int32_t taskId, WaitingReason reason)
    {
    }
    void AddListener(ani_ref &callback);

private:
    ani_vm *vm_ = nullptr;
    std::string tid_ = "";
    SubscribeType type_ = SubscribeType::FAILED;

    std::shared_ptr<Response> response_ = nullptr;
};

class AniTask {
public:
    AniTask(const std::string &tid) : tid_(tid) {
    }

    ~AniTask();

    static AniTask* Create([[maybe_unused]] ani_env* env, Config config);

    void Start(ani_env *env);
    void On([[maybe_unused]] ani_env *env, std::string, ani_ref callback);

    std::string GetTid()
    {
        return tid_;
    }

    void SetTid(std::string &tid)
    {
        tid_ = tid;
    }

    static bool SetDirsPermission(std::vector<std::string> &dirs);
    static bool SetPathPermission(const std::string &filepath);
    static void RemoveDirsPermission(const std::vector<std::string> &dirs);
    static void AddPathMap(const std::string &filepath, const std::string &baseDir);
    static void ResetDirAccess(const std::string &filepath);
    static void RemovePathMap(const std::string &filepath);
    static void AddTaskMap(const std::string &key, AniTask *task);
    static void ClearTaskMap(const std::string &key);

    Config config_ = {.action = Action::DOWNLOAD, .version = Version::API8};
    bool isGetPermission = false;

    static std::mutex pathMutex_;
    static std::mutex taskMutex_;
    static std::map<std::string, AniTask *> taskMap_;
    static std::map<std::string, int32_t> pathMap_;
    static std::map<std::string, int32_t> fileMap_;

private:
    std::string tid_ = "";
    SubscribeType type_ = SubscribeType::FAILED;
    static std::map<std::string, SubscribeType> supportEventsAni_;

    std::mutex listenerMutex_;
    std::shared_ptr<ResponseListener> responseListener_;
    std::map<SubscribeType, std::shared_ptr<NotifyDataListener>> notifyDataListenerMap_;
};

} // namespace OHOS::Request

#endif // ANI_TASK_H
