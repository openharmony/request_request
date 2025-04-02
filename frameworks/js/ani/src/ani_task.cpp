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

#include "constant.h"
#include "log.h"
#include "ani_utils.h"
#include "ani_task.h"
#include "request_manager.h"

using namespace OHOS::Request;

AniTask* AniTask::Create([[maybe_unused]] ani_env* env, Config config)
{
    int32_t seq = RequestManager::GetInstance()->GetNextSeq();
    REQUEST_HILOGI("AniTask::Create: seq: %{public}d", seq);
    RequestManager::GetInstance()->LoadRequestServer();

    std::string tid = "temp";
    int32_t ret = RequestManager::GetInstance()->Create(config, seq, tid);
    REQUEST_HILOGI("Create return: tid: [%{public}s]", tid.c_str());
    if (ret != E_OK) {
        REQUEST_HILOGE("End create task in Create, seq: %{public}d, failed: %{public}d", seq, ret);
        return new AniTask(tid);
    }

    auto notifyDataListener = std::make_shared<NotifyDataListener>(env, tid, SubscribeType::REMOVE);
    RequestManager::GetInstance()->AddListener(tid, SubscribeType::REMOVE, notifyDataListener);

    return new AniTask(tid);
}

void AniTask::Start()
{
    REQUEST_HILOGI("Enter AniTask::Start");

    int32_t ret = RequestManager::GetInstance()->Start(tid_);
    REQUEST_HILOGI("RequestManager::GetInstance()->Start(tid_) failed: %{public}d", ret);
    if (ret == E_OK) {
        REQUEST_HILOGI("AniTask::Start success");
    }
    REQUEST_HILOGI("AniTask::Start end");
}

void NotifyDataListener::OnNotifyDataReceive(const std::shared_ptr<NotifyData> &notifyData)
{
    REQUEST_HILOGI("OnNotifyDataReceive enter, type_:%{public}d, tid_:%{public}s", type_, tid_.c_str());
    if (env_ == nullptr) {
        return;
    }
}

void ResponseListener::OnResponseReceive(const std::shared_ptr<Response> &response)
{
    AniLocalScopeGuard guard(env_, 16);  // AniLocalScopeGuard size_t: 16
    ani_string version = AniStringUtils::ToAni(env_, response->version);
    ani_int statusCode = static_cast<ani_int>(response->statusCode);
    ani_string reason = AniStringUtils::ToAni(env_, response->reason);
    ani_ref httpResponse = AniObjectUtils::Create(env_, "L@ohos/request/request;", "Lagent;",
        "LHttpResponseImpl;", version, statusCode, reason);
    auto fnObj = reinterpret_cast<ani_fn_object>(callbackRef_);
    std::vector<ani_ref> args = {httpResponse};
    ani_ref result;
    REQUEST_HILOGI("%{public}s: Begin to call FunctionalObject_Call", __func__);
    if (fnObj == nullptr || args.size() == 0) {
        REQUEST_HILOGI("%{public}s: fnObj == nullptr", __func__);
        return;
    }
    if (ANI_OK != env_->FunctionalObject_Call(fnObj, 1, args.data(), &result)) {
        REQUEST_HILOGI("%{public}s: FunctionalObject_Call failed", __func__);
        return;
    }
    REQUEST_HILOGI("%{public}s: FunctionalObject_Call success", __func__);
}

void AniTask::On([[maybe_unused]] ani_env* env, std::string, ani_ref callback)
{
    REQUEST_HILOGI("Enter AniTask::On");

    auto responseListener = std::make_shared<ResponseListener>(env, callback);
    RequestManager::GetInstance()->AddListener(tid_, type_, responseListener);
    REQUEST_HILOGI("End AniTask::On");
}
