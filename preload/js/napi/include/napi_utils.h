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

#ifndef REQUEST_PRE_DOWNLOAD_NAPI_UTILS_H
#define REQUEST_PRE_DOWNLOAD_NAPI_UTILS_H

#include <vector>

#include "js_native_api.h"
#include "js_native_api_types.h"
#include "napi/native_common.h"
namespace OHOS::Request {
napi_valuetype GetValueType(napi_env env, napi_value value);
std::string GetValueString(napi_env env, napi_value value);
std::vector<std::string> GetPropertyNames(napi_env env, napi_value object);
std::string GetPropertyValue(napi_env env, napi_value object, const std::string &propertyName);
uint32_t GetValueNum(napi_env env, napi_value value);
napi_value NapiOk(napi_env env);
} // namespace OHOS::Request
#endif
