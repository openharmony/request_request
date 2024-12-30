/*
 * Copyright (c) 2023 Huawei Device Co., Ltd.
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

#include "common.h"

#include "request_preload.h"
using namespace OHOS::Request;
TestCallback::TestCallback(size_t size)
{
    auto flagS = std::make_shared<std::atomic_bool>(false);
    auto flagF = std::make_shared<std::atomic_bool>(false);
    auto flagC = std::make_shared<std::atomic_bool>(false);
    auto flagP = std::make_shared<std::atomic_bool>(false);
    this->callback = PreloadCallback{
        .OnSuccess =
            [flagS, size](const std::shared_ptr<Data> &&data, const std::string &taskId) {
                if (size == 0 || data->bytes().length() == size) {
                    flagS->store(true);
                }
            },
        .OnCancel = [flagC]() { flagC->store(true); },
        .OnFail = [flagF](const PreloadError &error, const std::string &taskId) { flagF->store(true); },
        .OnProgress = [flagP](uint64_t current, uint64_t total) { flagP->store(true); },
    };
    this->flagS = flagS;
    this->flagF = flagF;
    this->flagC = flagC;
    this->flagP = flagP;
}