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
#ifndef ASYNC_CALL_H
#define ASYNC_CALL_H

#include <functional>
#include <memory>
#include <string>
#include "napi/native_api.h"

namespace OHOS::Request {
class AsyncCall final {
public:
    class Context {
    public:
        using InputAction = std::function<napi_status(size_t, napi_value *, napi_value)>;
        using OutputAction = std::function<napi_status(napi_value *)>;
        using ExecAction = std::function<void()>;
        using ErrorCreator = std::function<napi_value(bool withErrCode, uint32_t innerErrCode)>;
        Context() = default;
        virtual ~Context()
        {
            napi_delete_async_work(env_, work_);
            napi_delete_reference(env_, self_);
        };
        inline Context &SetInput(InputAction action)
        {
            input_ = std::move(action);
            return *this;
        }
        inline Context &SetOutput(OutputAction action)
        {
            output_ = std::move(action);
            return *this;
        }
        inline Context &SetExec(ExecAction action)
        {
            exec_ = std::move(action);
            return *this;
        }
        inline Context &SetErrorCreator(ErrorCreator creator)
        {
            creator_ = std::move(creator);
            return *this;
        }

        InputAction input_ = nullptr;
        OutputAction output_ = nullptr;
        ExecAction exec_ = nullptr;
        ErrorCreator creator_ = nullptr;

        napi_env env_;
        napi_ref callbackRef_ = nullptr;
        napi_ref self_ = nullptr;
        napi_deferred defer_ = nullptr;
        napi_async_work work_ = nullptr;

        int32_t innerCode_;
        bool withErrCode_;
    };

    AsyncCall(napi_env env, napi_callback_info info, const std::shared_ptr<Context> &context);
    ~AsyncCall();
    napi_value Call(const std::shared_ptr<Context> &context, const std::string &resourceName = "AsyncCall");

private:
    enum {
        ARG_ERROR,
        ARG_DATA,
        ARG_BUTT
    };

    struct WorkData {
        std::shared_ptr<Context> ctx = nullptr;
    };
    static void OnExecute(napi_env env, void *data);
    static void OnComplete(napi_env env, napi_status status, void *data);
};
} // namespace OHOS::Request
#endif // ASYNC_CALL_H
