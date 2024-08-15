/*
* Copyright (C) 2024 Huawei Device Co., Ltd.
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

#include "cj_app_state_callback.h"

#include "js_common.h"
#include "cj_request_task.h"
#include "log.h"
#include "request_manager.h"

namespace OHOS::CJSystemapi::Request {
using OHOS::Request::RequestManager;
using OHOS::Request::Mode;

void CJAppStateCallback::OnAbilityForeground(const std::shared_ptr<NativeReference> &ability)
{
    if (RequestManager::GetInstance()->IsSaReady()) {
        return;
    }
    for (auto task = CJTask::taskMap_.begin(); task != CJTask::taskMap_.end(); ++task) {
        if (task->second->config_.mode == Mode::FOREGROUND) {
            RequestManager::GetInstance()->LoadRequestServer();
            return;
        }
    }
    CJTask::register_ = false;
    auto context = AbilityRuntime::ApplicationContext::GetInstance();
    if (context == nullptr) {
        REQUEST_HILOGE("Get ApplicationContext failed");
        return;
    }
    context->UnregisterAbilityLifecycleCallback(std::make_shared<CJAppStateCallback>());
    REQUEST_HILOGD("Unregister foreground resume callback success");
}
} // namespace OHOS::CJSystemapi::Request