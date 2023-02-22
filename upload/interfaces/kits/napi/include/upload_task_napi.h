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

#ifndef REQUEST_NAPI_H
#define REQUEST_NAPI_H

#include <string>
#include <vector>
#include "upload_config.h"
#include "upload_task.h"
#include "upload_common.h"
#include "async_call.h"
#include "progress_callback.h"
#include "header_receive_callback.h"
#include "notify_callback.h"
#include "context.h"

namespace OHOS::Request::UploadNapi {
using namespace OHOS::Request::Upload;
class UploadTaskNapi {
public:
    static napi_value JsUpload(napi_env env, napi_callback_info info);
    static napi_value JsOn(napi_env env, napi_callback_info info);
    static napi_value JsOff(napi_env env, napi_callback_info info);
    static napi_value JsRemove(napi_env env, napi_callback_info info);

    UploadTaskNapi &operator=(std::shared_ptr<Upload::UploadTask> &&uploadTask);
    bool operator==(const std::shared_ptr<Upload::UploadTask> &uploadTask);
    static napi_status GetContext(napi_env env, napi_value *argv, int &paramPosition,
        std::shared_ptr<OHOS::AbilityRuntime::Context> &context);

private:
    static napi_value GetCtor(napi_env env);
    static napi_value Initialize(napi_env env, napi_callback_info info);

    struct RemoveContextInfo : public AsyncCall::Context {
        UploadTaskNapi *proxy = nullptr;
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

    std::shared_ptr<Upload::IProgressCallback> onProgress_ = nullptr;
    std::shared_ptr<Upload::IHeaderReceiveCallback> onHeaderReceive_ = nullptr;
    std::shared_ptr<Upload::INotifyCallback> onFail_ = nullptr;
    std::shared_ptr<Upload::INotifyCallback> onComplete_ = nullptr;
    std::shared_ptr<Upload::UploadTask> napiUploadTask_ = nullptr;
    std::shared_ptr<Upload::UploadConfig> napiUploadConfig_ = nullptr;
};
} // namespace OHOS::Request::UploadNapi
#endif // REQUEST_NAPI_H