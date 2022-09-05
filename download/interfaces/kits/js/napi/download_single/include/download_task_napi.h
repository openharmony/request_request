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

#ifndef DOWNLOAD_TASK_NAPI_H
#define DOWNLOAD_TASK_NAPI_H

#include <string>
#include <vector>

#include "async_call.h"
#include "download_config.h"

#include "data_ability_helper.h"

namespace OHOS::Request::Download {
class DownloadTaskNapi {
public:
    static napi_value JsMain(napi_env env, napi_callback_info info);

private:
    static napi_value GetCtor(napi_env env);
    static napi_value Initialize(napi_env env, napi_callback_info info);
    static bool ParseConfig(napi_env env, napi_value configValue, DownloadConfig &config);
    static bool ParseHeader(napi_env env, napi_value configValue, DownloadConfig &config);
    static bool ParseUrl(napi_env env, napi_value configValue, DownloadConfig &config);
    static napi_status OnHeaderReceive(
        napi_env env, size_t argc, napi_value *argv, napi_value self, napi_value *result);
    static std::shared_ptr<OHOS::AppExecFwk::DataAbilityHelper> GetDataAbilityHelper(napi_env env);

private:
    static __thread napi_ref globalCtor;
};
} // namespace OHOS::Request::Download
#endif // DOWNLOAD_TASK_NAPI_H
