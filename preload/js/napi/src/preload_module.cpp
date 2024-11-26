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

#include <unistd.h>

#include <cstdint>
#include <memory>

#include "js_native_api.h"
#include "js_native_api_types.h"
#include "napi/native_common.h"
#include "napi/native_node_api.h"
#include "napi_utils.h"
#include "request_preload.h"

namespace OHOS::Request {

napi_value download(napi_env env, napi_callback_info info)
{
    size_t argc = 2;
    napi_value args[2] = { nullptr };
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, args, nullptr, nullptr));

    if (GetValueType(env, args[0]) != napi_string || GetValueType(env, args[1]) != napi_object) {
        napi_throw_type_error(env, nullptr, "Unsupported parameter type");
        return nullptr;
    }
    std::string url = GetValueString(env, args[0]);

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
    Preload::GetInstance()->load(url, nullptr, std::move(options));
    return nullptr;
}

napi_value cancel(napi_env env, napi_callback_info info)
{
    size_t argc = 1;
    napi_value args[1] = { nullptr };
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, args, nullptr, nullptr));

    if (GetValueType(env, args[0]) != napi_string) {
        napi_throw_type_error(env, nullptr, "Unsupported parameter type");
        return nullptr;
    }
    std::string url = GetValueString(env, args[0]);
    Preload::GetInstance()->Cancel(url);
    return nullptr;
}

napi_value setMemoryCacheSize(napi_env env, napi_callback_info info)
{
    size_t argc = 1;
    napi_value args[1] = { nullptr };
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, args, nullptr, nullptr));

    if (GetValueType(env, args[0]) != napi_number) {
        napi_throw_type_error(env, nullptr, "Unsupported parameter type");
        return nullptr;
    }
    uint32_t size = GetValueNum(env, args[0]);
    Preload::GetInstance()->SetRamCacheSize(size);
    return nullptr;
}

napi_value setFileCacheSize(napi_env env, napi_callback_info info)
{
    size_t argc = 1;
    napi_value args[1] = { nullptr };
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, args, nullptr, nullptr));

    if (GetValueType(env, args[0]) != napi_number) {
        napi_throw_type_error(env, nullptr, "Unsupported parameter type");
        return nullptr;
    }
    uint32_t size = GetValueNum(env, args[0]);
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
