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

#include "napi_utils.h"

#include <cstdint>
#include <mutex>
#include <string>

#include "base/request/request/common/include/constant.h"
#include "js_native_api.h"
#include "js_native_api_types.h"
#include "napi/native_common.h"

namespace OHOS::Request {
napi_value CreateBusinessError(napi_env env, int32_t errorCode, const std::string &errorMessage)
{
    napi_value error = nullptr;
    napi_value msg = nullptr;
    NAPI_CALL(env, napi_create_string_utf8(env, errorMessage.c_str(), errorMessage.length(), &msg));
    NAPI_CALL(env, napi_create_error(env, nullptr, msg, &error));
    napi_value code = nullptr;
    NAPI_CALL(env, napi_create_uint32(env, static_cast<uint32_t>(errorCode), &code));
    napi_set_named_property(env, error, "code", code);
    return error;
}

void ThrowError(napi_env env, int32_t code, const std::string &msg)
{
    napi_value error = CreateBusinessError(env, code, msg);
    napi_throw(env, error);
}

napi_valuetype GetValueType(napi_env env, napi_value value)
{
    if (value == nullptr) {
        return napi_undefined;
    }
    napi_valuetype valueType = napi_undefined;
    NAPI_CALL_BASE(env, napi_typeof(env, value, &valueType), napi_undefined);
    return valueType;
}

size_t GetStringLength(napi_env env, napi_value value)
{
    size_t length;
    NAPI_CALL_BASE(env, napi_get_value_string_utf8(env, value, nullptr, 0, &length), 0);
    return length;
}

std::string GetValueString(napi_env env, napi_value value, size_t length)
{
    char chars[length + 1];
    NAPI_CALL(env, napi_get_value_string_utf8(env, value, chars, sizeof(chars), &length));
    return std::string(chars);
}

int64_t GetValueNum(napi_env env, napi_value value)
{
    int64_t ret;
    NAPI_CALL_BASE(env, napi_get_value_int64(env, value, &ret), 0);
    return ret;
}

std::vector<std::string> GetPropertyNames(napi_env env, napi_value object)
{
    std::vector<std::string> ret;
    napi_value names = nullptr;
    NAPI_CALL_BASE(env, napi_get_property_names(env, object, &names), ret);
    uint32_t length = 0;
    NAPI_CALL_BASE(env, napi_get_array_length(env, names, &length), ret);
    for (uint32_t index = 0; index < length; ++index) {
        napi_value name = nullptr;
        if (napi_get_element(env, names, index, &name) != napi_ok) {
            continue;
        }
        if (GetValueType(env, name) != napi_string) {
            continue;
        }
        size_t propertyLength = GetStringLength(env, name);
        ret.emplace_back(GetValueString(env, name, propertyLength));
    }
    return ret;
}

bool HasNamedProperty(napi_env env, napi_value object, const std::string &propertyName)
{
    bool hasProperty = false;
    NAPI_CALL_BASE(env, napi_has_named_property(env, object, propertyName.c_str(), &hasProperty), false);
    return hasProperty;
}

napi_value GetNamedProperty(napi_env env, napi_value object, const std::string &propertyName)
{
    napi_value value = nullptr;
    bool hasProperty = false;
    NAPI_CALL(env, napi_has_named_property(env, object, propertyName.c_str(), &hasProperty));
    if (!hasProperty) {
        return value;
    }
    NAPI_CALL(env, napi_get_named_property(env, object, propertyName.c_str(), &value));
    return value;
}

std::string GetStringValueWithDefault(napi_env env, napi_value value)
{
    if (GetValueType(env, value) != napi_string) {
        return "";
    }
    auto length = GetStringLength(env, value);
    return GetValueString(env, value, length);
}

std::string GetPropertyValue(napi_env env, napi_value object, const std::string &propertyName)
{
    if (!HasNamedProperty(env, object, propertyName)) {
        return "";
    }
    napi_value value = GetNamedProperty(env, object, propertyName);
    return GetStringValueWithDefault(env, value);
}

napi_value Convert2JSValue(napi_env env, const std::string &str)
{
    napi_value value = nullptr;
    if (napi_create_string_utf8(env, str.c_str(), strlen(str.c_str()), &value) != napi_ok) {
        return nullptr;
    }
    return value;
}

napi_value Convert2JSValue(napi_env env, uint32_t code)
{
    napi_value value = nullptr;
    if (napi_create_uint32(env, code, &value) != napi_ok) {
        return nullptr;
    }
    return value;
}

void SetStringPropertyUtf8(napi_env env, napi_value object, const std::string &name, const std::string &value)
{
    napi_value jsValue = Convert2JSValue(env, value);
    if (GetValueType(env, jsValue) != napi_string) {
        return;
    }
    napi_status status = napi_set_named_property(env, object, name.c_str(), jsValue);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "preload failed to set named property");
    }
}

void SetUint32Property(napi_env env, napi_value object, const std::string &name, uint32_t value)
{
    napi_value jsValue = Convert2JSValue(env, value);
    if (GetValueType(env, jsValue) != napi_number) {
        return;
    }
    napi_status status = napi_set_named_property(env, object, name.c_str(), jsValue);
    if (status != napi_ok) {
        napi_throw_error(env, nullptr, "preload failed to set named property");
    }
}

} // namespace OHOS::Request