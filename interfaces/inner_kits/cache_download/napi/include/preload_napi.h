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

#ifndef REQUEST_PRE_DOWNLOAD_PRELOAD_NAPI_H
#define REQUEST_PRE_DOWNLOAD_PRELOAD_NAPI_H

#include <vector>

#include "js_native_api.h"
#include "js_native_api_types.h"
#include "napi/native_common.h"
#include "request_preload.h"
namespace OHOS::Request {
/**
 * @brief Convert the C++ download statistics object to an ArkTS/JS napi_value.
 *
 * Serializes the CppDownloadInfo collected by the Preload module (DNS,
 * connection, durations, resource size, etc.) into an externally readable JS
 * object, returned to the application layer by the NAPI interface.
 * @param env napi calling environment.
 * @param result Download statistics to convert.
 * @return The converted napi_value object.
 */
napi_value BuildDownloadInfo(napi_env env, const CppDownloadInfo &result);
} // namespace OHOS::Request
#endif