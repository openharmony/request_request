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

#include "preload_callback_test.h"

#include <vector>

#include "js_native_api.h"
#include "js_native_api_types.h"
#include "napi/native_common.h"
#include "preload_napi.h"
#include "request_preload.h"

using namespace OHOS::Request;
TestCallback::TestCallback()
{
    auto flagS = std::make_shared<std::atomic_bool>(false);
    auto flagF = std::make_shared<std::atomic_bool>(false);
    auto flagInfo = std::make_shared<std::atomic_bool>(false);
    auto flagC = std::make_shared<std::atomic_bool>(false);
    auto flagP = std::make_shared<std::atomic_bool>(false);
    this->callback = PreloadCallback{
        .OnSuccess = [flagS](const std::shared_ptr<Data> &&data, const std::string &taskId) { flagS->store(true); },
        .OnCancel = [flagC]() { flagC->store(true); },
        .OnFail =
            [flagF, flagInfo](const PreloadError &error, const std::string &taskId) {
                std::shared_ptr<CppDownloadInfo> info = error.GetDownloadInfo();
                if (info == nullptr) {
                    flagF->store(true);
                    return;
                }
                napi_value result = BuildDownloadInfo(nullptr, *info.get());
                if (result == nullptr) {
                    flagInfo->store(true);
                }
                flagF->store(true);
            },
        .OnProgress = [flagP](uint64_t current, uint64_t total) { flagP->store(true); },
    };
    this->flagS = flagS;
    this->flagF = flagF;
    this->flagC = flagC;
    this->flagP = flagP;
    this->flagInfo = flagInfo;
}