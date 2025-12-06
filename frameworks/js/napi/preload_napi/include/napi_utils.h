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
#include "request_preload.h"
namespace OHOS::Request {
static constexpr int32_t NO_ARG = 0;
static constexpr int32_t ONE_ARG = 1;
static constexpr int32_t TWO_ARG = 2;
static constexpr int32_t THE_ARG = 3;
napi_valuetype GetValueType(napi_env env, napi_value value);
napi_value GetNamedProperty(napi_env env, napi_value object, const std::string &propertyName);
std::string GetStringValueWithDefault(napi_env env, napi_value value);

size_t GetStringLength(napi_env env, napi_value value);
std::string GetValueString(napi_env env, napi_value value, size_t length);
std::vector<std::string> GetPropertyNames(napi_env env, napi_value object);
std::string GetPropertyValue(napi_env env, napi_value object, const std::string &propertyName);
int64_t GetValueNum(napi_env env, napi_value value);
void ThrowError(napi_env env, int32_t code, const std::string &msg);
void SetStringPropertyUtf8(napi_env env, napi_value object, const std::string &name, const std::string &value);
void SetUint32Property(napi_env env, napi_value object, const std::string &name, uint32_t value);
napi_value Convert2JSValue(napi_env env, const std::string &str);
napi_value Convert2JSValue(napi_env env, uint32_t code);
} // namespace OHOS::Request
#endif
