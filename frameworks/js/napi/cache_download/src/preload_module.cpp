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

#include "preload_module.h"

#include <dlfcn.h>
#include <unistd.h>

#include <cstdint>
#include <memory>

#include "access_token.h"
#include "accesstoken_kit.h"
#include "base/request/request/common/include/constant.h"
#include "base/request/request/common/include/log.h"
#include "ipc_skeleton.h"
#include "js_native_api.h"
#include "js_native_api_types.h"
#include "napi/native_common.h"
#include "napi/native_node_api.h"
#include "napi_utils.h"
#include "request_preload.h"

namespace OHOS::Request {
using namespace Security::AccessToken;

constexpr const size_t MAX_UTL_LENGTH = 8192;

constexpr int64_t MAX_MEM_SIZE = 1073741824;
constexpr int64_t MAX_FILE_SIZE = 4294967296;
const std::string INTERNET_PERMISSION = "ohos.permission.INTERNET";

bool CheckInternetPermission()
{
    static bool hasPermission = []() {
        uint64_t tokenId = IPCSkeleton::GetCallingFullTokenID();
        TypeATokenTypeEnum tokenType = AccessTokenKit::GetTokenTypeFlag(static_cast<AccessTokenID>(tokenId));
        if (tokenType == TOKEN_INVALID) {
            return false;
        }
        int result = AccessTokenKit::VerifyAccessToken(tokenId, INTERNET_PERMISSION);
        return result == PERMISSION_GRANTED;
    }();
    return hasPermission;
}

napi_value download(napi_env env, napi_callback_info info)
{
    size_t argc = 2;
    napi_value args[2] = { nullptr };
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, args, nullptr, nullptr));
    if (GetValueType(env, args[0]) != napi_string || GetValueType(env, args[1]) != napi_object) {
        ThrowError(env, E_PARAMETER_CHECK, "parameter error");
        return nullptr;
    }
    size_t urlLength = GetStringLength(env, args[0]);
    if (urlLength > MAX_UTL_LENGTH) {
        ThrowError(env, E_PARAMETER_CHECK, "url exceeds the maximum length");
        return nullptr;
    }
    std::string url = GetValueString(env, args[0], urlLength);
    std::unique_ptr<PreloadOptions> options = std::make_unique<PreloadOptions>();
    napi_value headers = nullptr;
    if (napi_get_named_property(env, args[1], "headers", &headers) == napi_ok
        && GetValueType(env, headers) == napi_valuetype::napi_object) {
        auto names = GetPropertyNames(env, headers);
        for (auto name : names) {
            auto value = GetPropertyValue(env, headers, name);
            options->headers.emplace_back(std::make_pair(name, value));
        }
    }
    if (!CheckInternetPermission()) {
        ThrowError(env, E_PERMISSION, "internet permission denied");
        REQUEST_HILOGI("internet permission denied");
        return nullptr;
    }
    Preload::GetInstance()->load(url, nullptr, std::move(options), true);
    return nullptr;
}

napi_value cancel(napi_env env, napi_callback_info info)
{
    size_t argc = 1;
    napi_value args[1] = { nullptr };
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, args, nullptr, nullptr));
    if (GetValueType(env, args[0]) != napi_string) {
        ThrowError(env, E_PARAMETER_CHECK, "parameter error");
        return nullptr;
    }
    size_t urlLength = GetStringLength(env, args[0]);
    if (urlLength > MAX_UTL_LENGTH) {
        ThrowError(env, E_PARAMETER_CHECK, "url exceeds the maximum length");
        return nullptr;
    }
    std::string url = GetValueString(env, args[0], urlLength);
    Preload::GetInstance()->Cancel(url);
    return nullptr;
}

napi_value setMemoryCacheSize(napi_env env, napi_callback_info info)
{
    size_t argc = 1;
    napi_value args[1] = { nullptr };
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, args, nullptr, nullptr));

    if (GetValueType(env, args[0]) != napi_number) {
        ThrowError(env, E_PARAMETER_CHECK, "parameter error");
        return nullptr;
    }
    int64_t size = GetValueNum(env, args[0]);
    if (size > MAX_MEM_SIZE) {
        ThrowError(env, E_PARAMETER_CHECK, "memory cache size exceeds the maximum value");
        return nullptr;
    }
    Preload::GetInstance()->SetRamCacheSize(size);
    return nullptr;
}

napi_value setFileCacheSize(napi_env env, napi_callback_info info)
{
    size_t argc = 1;
    napi_value args[1] = { nullptr };
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, args, nullptr, nullptr));

    if (GetValueType(env, args[0]) != napi_number) {
        ThrowError(env, E_PARAMETER_CHECK, "parameter error");
        return nullptr;
    }
    int64_t size = GetValueNum(env, args[0]);
    if (size > MAX_FILE_SIZE) {
        ThrowError(env, E_PARAMETER_CHECK, "file cache size exceeds the maximum value");
        return nullptr;
    }
    Preload::GetInstance()->SetFileCacheSize(size);
    return nullptr;
}

static napi_value registerFunc(napi_env env, napi_value exports)
{
    napi_property_descriptor desc[]{
        DECLARE_NAPI_FUNCTION("download", download),
        DECLARE_NAPI_FUNCTION("cancel", cancel),
        DECLARE_NAPI_FUNCTION("setMemoryCacheSize", setMemoryCacheSize),
        DECLARE_NAPI_FUNCTION("setFileCacheSize", setFileCacheSize),
    };
    NAPI_CALL(env, napi_define_properties(env, exports, sizeof(desc) / sizeof(napi_property_descriptor), desc));
    return exports;
}

} // namespace OHOS::Request

static __attribute__((constructor)) void RegisterModule()
{
    static napi_module module = { .nm_version = 1,
        .nm_flags = 0,
        .nm_filename = nullptr,
        .nm_register_func = OHOS::Request::registerFunc,
        .nm_modname = "request.cacheDownload",
        .nm_priv = ((void *)0),
        .reserved = { 0 } };
    napi_module_register(&module);
}
