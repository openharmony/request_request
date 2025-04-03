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

namespace OHOS::Request {

class ResponseListener : public IResponseListener {
public:
    virtual ~ResponseListener() = default;
    ResponseListener(ani_env* env, ani_ref callbackRef) : env_(env), callbackRef_(callbackRef)
    {
    }

    virtual void OnResponseReceive(const std::shared_ptr<Response> &response);

private:
    ani_env *env_;
    ani_ref callbackRef_;
};

class NotifyDataListener : public INotifyDataListener {
public:
    virtual ~NotifyDataListener() = default;
    NotifyDataListener(ani_env* env, std::string tid, SubscribeType type) : env_(env), tid_(tid), type_(type)
    {
    }

    virtual void OnNotifyDataReceive(const std::shared_ptr<NotifyData> &notifyData);

private:
    ani_env *env_;
    std::string tid_;
    SubscribeType type_;
};

class AniTask {
public:
    AniTask(const std::string &tid) : tid_(tid) {
    }

    ~AniTask();

    static AniTask* Create([[maybe_unused]] ani_env* env, Config config);

    void Start();
    void On([[maybe_unused]] ani_env *env, std::string, ani_ref callback);

    std::string GetTid()
    {
        return tid_;
    }

    void SetTid(std::string &tid)
    {
        tid_ = tid;
    }

private:
    std::string tid_;
    SubscribeType type_;
};

} // namespace OHOS::Request

#endif // ANI_TASK_H
