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
#ifndef UV_QUEUE_H
#define UV_QUEUE_H

#include <iostream>

#include "napi/native_api.h"
#include "uv.h"

namespace OHOS::Request {
struct UvCallbackData {
    napi_env env;
    napi_ref ref;
};
class UvQueue {
public:
    static bool Call(napi_env env, void *data, uv_after_work_cb afterCallback);
    static void DeleteRef(napi_env env, napi_ref ref);
    static void UvDelete(uv_work_t *work, int status);
};
} // namespace OHOS::Request
#endif // UV_QUEUE_H
