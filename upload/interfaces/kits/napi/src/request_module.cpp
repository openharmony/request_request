/*
 * Copyright (c) 2022 Huawei Device Co., Ltd.
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

#include "napi/native_api.h"
#include "napi/native_node_api.h"
#include "upload_task_napi.h"
#include "js_util.h"
#include "download_task_napi.h"

using namespace OHOS::Request::UploadNapi;
using namespace OHOS::Request::Upload;
using namespace OHOS::Request::Download;

static napi_value Init(napi_env env, napi_value exports)
{
    napi_property_descriptor desc[] = {
        DECLARE_NAPI_METHOD("download", DownloadTaskNapi::JsMain),
        DECLARE_NAPI_METHOD("upload", UploadTaskNapi::JsUpload),
    };

    napi_status status = napi_define_properties(env, exports, sizeof(desc) / sizeof(napi_property_descriptor), desc);
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "init upload %{public}d", status);
    return exports;
}

static __attribute__((constructor)) void RegisterModule()
{
    static napi_module module = {
        .nm_version = 1,
        .nm_flags = 0,
        .nm_filename = nullptr,
        .nm_register_func = Init,
        .nm_modname = "request",
        .nm_priv = ((void *)0),
        .reserved = { 0 }
    };
    napi_module_register(&module);
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "module register request");
}