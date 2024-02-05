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
#include "c_event_handler.h"

#include "event_handler.h"
#include "event_runner.h"
#include "inner_event.h"
#include "log.h"

std::shared_ptr<OHOS::AppExecFwk::EventHandler> serviceHandler_ = nullptr;
const std::int64_t INIT_INTERVAL = 5000L;

void RequestInitServiceHandler(void)
{
    REQUEST_HILOGD("RequestInitServiceHandler started.");
    if (serviceHandler_ != nullptr) {
        REQUEST_HILOGE("RequestInitServiceHandler already init.");
        return;
    }
    std::shared_ptr<OHOS::AppExecFwk::EventRunner> runner = OHOS::AppExecFwk::EventRunner::Create("DownloadServiceAbil"
                                                                                                  "ity");
    serviceHandler_ = std::make_shared<OHOS::AppExecFwk::EventHandler>(runner);
    REQUEST_HILOGD("RequestInitServiceHandler succeeded.");
}

void RequestPostTask(fun f)
{
    REQUEST_HILOGD("RequestPostTask");
    if (serviceHandler_ == nullptr) {
        REQUEST_HILOGE("serviceHandler_ is null");
        return;
    }
    auto callback = [f]() { f(); };
    serviceHandler_->PostTask(callback, INIT_INTERVAL);
    REQUEST_HILOGE("DownloadServiceAbility Init failed. Try again 5s later");
}