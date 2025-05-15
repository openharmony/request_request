/*
 * Copyright (C) 2025 Huawei Device Co., Ltd.
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

#include "ani_utils.h"
#include "listener_list.h"
#include "log.h"
using namespace OHOS::AniUtil;
namespace OHOS::Request {

ani_status ListenerList::AddListenerInner(ani_ref cb)
{
    if (cb == nullptr) {
        return ANI_OK;
    }
    REQUEST_HILOGI("AddListenerInner begin");
    std::lock_guard<std::recursive_mutex> lock(allCbMutex_);
    this->allCb_.push_back(std::make_pair(true, cb));
    ++this->validCbNum;
    REQUEST_HILOGI("AddListenerInner end");

    return ANI_OK;
}

void ListenerList::OnMessageReceive(ani_env* env, std::vector<ani_ref> &args)
{
    REQUEST_HILOGI("OnMessageReceive begin");
    if (args.size() == 0) {
        REQUEST_HILOGE("%{public}s: args size is zero", __func__);
        return;
    }
    if (env == nullptr) {
        REQUEST_HILOGE("%{public}s: env is nullptr.", __func__);
        return;
    }

    std::lock_guard<std::recursive_mutex> lock(allCbMutex_);
    for (auto it = this->allCb_.begin(); it != this->allCb_.end();) {
        auto fnObj = reinterpret_cast<ani_fn_object>(it->second);
        ani_ref result;
        if (fnObj != nullptr && ANI_OK != env->FunctionalObject_Call(fnObj, 1, args.data(), &result)) {
            REQUEST_HILOGI("%{public}s: FunctionalObject_Call failed", __func__);
        }
        it++;
    }
    REQUEST_HILOGI("OnMessageReceive end");
}

} // namespace OHOS::Request