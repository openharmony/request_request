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

#ifndef UPLOAD_TASK_NAPIV9_H
#define UPLOAD_TASK_NAPIV9_H

#include <string>
#include <vector>
#include "upload_config.h"
#include "upload_task.h"
#include "upload_common.h"
#include "async_call.h"
#include "progress_callback.h"
#include "header_receive_callback.h"
#include "notify_callback.h"
#include "i_progress_callback.h"
#include "i_header_receive_callback.h"
#include "i_notify_callback.h"
#include "context.h"
#include "ability_context.h"
#include "data_ability_helper.h"

namespace OHOS::Request::UploadNapi {
using namespace OHOS::Request::Upload;
class UploadTaskNapiV9 {
public:
    static napi_value JsUploadFile(napi_env env, napi_callback_info info);
    static napi_value JsOn(napi_env env, napi_callback_info info);
    static napi_value JsOff(napi_env env, napi_callback_info info);
    static napi_value JsDelete(napi_env env, napi_callback_info info);

    UploadTaskNapiV9 &operator=(std::shared_ptr<Upload::UploadTask> &&uploadTask);
    bool operator==(const std::shared_ptr<Upload::UploadTask> &uploadTask);

private:
    static napi_value GetCtor(napi_env env);
    static napi_value Initialize(napi_env env, napi_callback_info info);
    static napi_status InitParam(napi_env env, napi_callback_info info, napi_value &self, UploadTaskNapiV9 *proxy);
    static napi_status GetContext(napi_env env, napi_value *argv,
        std::shared_ptr<OHOS::AbilityRuntime::Context>& context);
    static uint32_t InitFileArray(const std::shared_ptr<Upload::UploadConfig> &config,
        std::shared_ptr<OHOS::AbilityRuntime::Context> &context, int64_t &totalSize, std::vector<FileData> &fileDatas);

    std::shared_ptr<Upload::UploadTask> napiUploadTask_ = nullptr;
    std::shared_ptr<Upload::UploadConfig> napiUploadConfig_ = nullptr;

    struct RemoveContextInfo : public AsyncCall::Context {
        UploadTaskNapiV9 *proxy = nullptr;
        bool removeStatus = false;
        napi_status status = napi_generic_failure;
        RemoveContextInfo() : Context(nullptr, nullptr) {};
        RemoveContextInfo(InputAction input, OutputAction output) : Context(std::move(input), std::move(output)) {};
        virtual ~RemoveContextInfo() {};

        napi_status operator()(napi_env env, size_t argc, napi_value *argv, napi_value self) override
        {
            NAPI_ASSERT_BASE(env, self != nullptr, "self is nullptr", napi_invalid_arg);
            NAPI_CALL_BASE(env, napi_unwrap(env, self, reinterpret_cast<void **>(&proxy)), napi_invalid_arg);
            NAPI_ASSERT_BASE(env, proxy != nullptr, "there is no native upload task", napi_invalid_arg);
            return Context::operator()(env, argc, argv, self);
        }
        napi_status operator()(napi_env env, napi_value *result) override
        {
            if (status != napi_ok) {
                return status;
            }
            return Context::operator()(env, result);
        }
    };

    struct JsParam {
        std::string type;
        napi_value callback;
        napi_value self;
    };

    using Exec = std::function<napi_status(napi_env, napi_value, napi_value)>;
    static std::map<std::string, Exec> onTypeHandlers_;
    static std::map<std::string, Exec> offTypeHandlers_;
    static napi_status OnProgress(napi_env env, napi_value callback, napi_value self);
    static napi_status OnHeaderReceive(napi_env env, napi_value callback, napi_value self);
    static napi_status OnFail(napi_env env, napi_value callback, napi_value self);
    static napi_status OnComplete(napi_env env, napi_value callback, napi_value self);
    static napi_status OffProgress(napi_env env, napi_value callback, napi_value self);
    static napi_status OffHeaderReceive(napi_env env, napi_value callback, napi_value self);
    static napi_status OffFail(napi_env env, napi_value callback, napi_value self);
    static napi_status OffComplete(napi_env env, napi_value callback, napi_value self);
    static napi_status ParseParam(napi_env env, napi_callback_info info, bool IsRequiredParam, JsParam &jsParam);
    static uint32_t CheckFilePath(const std::shared_ptr<Upload::UploadConfig> &config,
    std::shared_ptr<OHOS::AbilityRuntime::Context> &context, std::vector<Upload::TaskState> &taskStates);
    static uint32_t CheckAbilityPath(const std::string &fileUri,
       std::shared_ptr<OHOS::AbilityRuntime::Context> &context);
    static uint32_t CheckInternalPath(const std::string &fileUri,
        std::shared_ptr<OHOS::AbilityRuntime::Context> &context);

    std::shared_ptr<Upload::IProgressCallback> onProgress_ = nullptr;
    std::shared_ptr<Upload::IHeaderReceiveCallback> onHeaderReceive_ = nullptr;
    std::shared_ptr<Upload::INotifyCallback> onFail_ = nullptr;
    std::shared_ptr<Upload::INotifyCallback> onComplete_ = nullptr;
    std::shared_ptr<OHOS::AbilityRuntime::Context> context_ = nullptr;
};
} // namespace OHOS::Request::UploadNapi
#endif // UPLOAD_TASK_NAPIV9_H