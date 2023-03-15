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

#ifndef UPLOAD_TASK_NAPIV5_H
#define UPLOAD_TASK_NAPIV5_H

#include <string>
#include <vector>

#include "context.h"
#include "data_ability_helper.h"
#include "header_receive_callback.h"
#include "notify_callback.h"
#include "progress_callback.h"
#include "upload_config.h"
#include "upload_task.h"


namespace OHOS::Request::Upload {
using namespace OHOS::Request::UploadNapi;
class UploadTaskNapiV5 : public std::enable_shared_from_this<UploadTaskNapiV5> {
public:
    struct SystemFailCallback {
        std::string data;
        int32_t code;
        napi_env env;
        napi_ref ref;
    };

    struct SystemSuccessCallback {
        napi_env env;
        napi_ref ref;
        Upload::UploadResponse response;
    };

    struct SystemCompleteCallback {
        napi_env env;
        napi_ref ref;
    };

    struct RecycleRef {
        napi_env env;
        napi_ref successRef;
        napi_ref failRef;
        napi_ref completeRef;
    };

    UploadTaskNapiV5(napi_env env) : env_(env){};
    ~UploadTaskNapiV5();
    static void OnSystemSuccess(napi_env env, napi_ref ref, Upload::UploadResponse &response);
    static void OnSystemFail(napi_env env, napi_ref ref, std::string &response, int32_t &code);
    static void OnSystemComplete(napi_env env, napi_ref ref);
    napi_value JsUpload(napi_env env, napi_callback_info info);
    bool ParseCallback(napi_env env, napi_callback_info info);
    void AddCallbackToConfig(napi_env env, std::shared_ptr<UploadConfig> &config);
    inline void SetEnv(napi_env env)
    {
        env_ = env;
    }

private:
    std::shared_ptr<Upload::UploadTask> uploadTask_ = nullptr;
    napi_ref success_ = nullptr;
    napi_ref fail_ = nullptr;
    napi_ref complete_ = nullptr;
    napi_env env_ = nullptr;
};
} // namespace OHOS::Request::Upload

#endif // UPLOAD_TASK_NAPIV5_H