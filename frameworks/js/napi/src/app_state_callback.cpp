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

#include "app_state_callback.h"

#include "js_task.h"
#include "log.h"
#include "request_manager.h"

namespace OHOS {
namespace Request {
void AppStateCallback::OnAbilityForeground(const std::shared_ptr<NativeReference> &ability)
{
    if (RequestManager::GetInstance()->IsSaReady()) {
        return;
    }
    for (auto task = JsTask::taskMap_.begin(); task != JsTask::taskMap_.end(); ++task) {
        if (task->second->config_.mode == Mode::FOREGROUND) {
            RequestManager::GetInstance()->LoadRequestServer();
            return;
        }
    }
    JsTask::register_ = false;
    auto context = AbilityRuntime::ApplicationContext::GetInstance();
    if (context == nullptr) {
        REQUEST_HILOGE("Get ApplicationContext failed");
        return;
    }
    context->UnregisterAbilityLifecycleCallback(std::make_shared<AppStateCallback>());
    REQUEST_HILOGD("Unregister foreground resume callback success");
}
} // namespace Request
} // namespace OHOS