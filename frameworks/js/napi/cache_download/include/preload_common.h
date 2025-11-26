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

#ifndef REQUEST_PRE_DOWNLOAD_PRELOAD_COMMON_H
#define REQUEST_PRE_DOWNLOAD_PRELOAD_COMMON_H

#include <vector>

#include "js_native_api.h"
#include "js_native_api_types.h"
#include "napi/native_common.h"
#include "request_preload.h"
namespace OHOS::Request {
bool BuildInfoPerformance(napi_env env, const CppDownloadInfo &result, napi_value &jsInfo);
bool BuildInfoNetwork(napi_env env, const CppDownloadInfo &result, napi_value &jsInfo);
bool BuildInfoResource(napi_env env, const CppDownloadInfo &result, napi_value &jsInfo);
void SetOptionsHeaders(napi_env env, napi_value arg, std::unique_ptr<PreloadOptions> &options);
void SetOptionsSslType(napi_env env, napi_value arg, std::unique_ptr<PreloadOptions> &options);
void GetCacheStrategy(napi_env env, napi_value arg, bool &isUpdate);
inline napi_status SetPerformanceField(napi_env env, napi_value performance, double field_value, const char *js_name);
} // namespace OHOS::Request
#endif