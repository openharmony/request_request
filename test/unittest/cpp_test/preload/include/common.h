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

#ifndef REQUEST_PRELOAD_TEST_COMMON_H
#define REQUEST_PRELOAD_TEST_COMMON_H

#include <atomic>
#include <memory>

#include "request_preload.h"
struct TestCallback {
    TestCallback(size_t size = 0);
    std::shared_ptr<std::atomic_bool> flagS;
    std::shared_ptr<std::atomic_bool> flagF;
    std::shared_ptr<std::atomic_bool> flagC;
    std::shared_ptr<std::atomic_bool> flagP;
    OHOS::Request::PreloadCallback callback;
};

#endif