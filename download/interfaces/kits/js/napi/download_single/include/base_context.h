/*
 * Copyright (C) 2021-2022 Huawei Device Co., Ltd.
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

#ifndef DOWNLOAD_BASE_CONTEXT_H
#define DOWNLOAD_BASE_CONTEXT_H

#include <list>

#include "event_manager.h"
#include "napi/native_api.h"
#include "napi/native_common.h"
#include "noncopyable.h"

namespace OHOS::Request::Download {
typedef void (*AsyncWorkExecutor)(napi_env env, void *data);

typedef void (*AsyncWorkCallback)(napi_env env, napi_status status, void *data);

// support no argument
class BaseContext {
public:
    ACE_DISALLOW_COPY_AND_MOVE(BaseContext);

    BaseContext() = delete;

    explicit BaseContext(napi_env env, napi_callback_info info, EventManager *manager);

    virtual ~BaseContext();

    virtual void ParseParams(napi_value *params, size_t paramsCount);

    void SetParseOK(bool parseOK);

    void SetExecOK(bool execOK);

    void SetErrorCode(int32_t errorCode);

    napi_status SetCallback(napi_value callback);

    void DeleteCallback();

    void CreateAsyncWork(const std::string &name, AsyncWorkExecutor executor, AsyncWorkCallback callback);

    void DeleteAsyncWork();

    napi_value CreatePromise();

    [[nodiscard]] bool IsParseOK() const;

    [[nodiscard]] bool IsExecOK() const;

    [[nodiscard]] napi_env GetEnv() const;

    [[nodiscard]] napi_callback_info GetInfo() const;

    [[nodiscard]] int32_t GetErrorCode() const;

    [[nodiscard]] napi_value GetCallback() const;

    [[nodiscard]] napi_deferred GetDeferred() const;

    [[nodiscard]] const std::string &GetAsyncWorkName() const;

    void Emit(const std::string &type, const std::pair<napi_value, napi_value> &argv);

protected:
    EventManager *manager_;

private:
    napi_env env_;

    napi_callback_info info_;

    bool parseOK_;

    bool execOK_;

    int32_t errorCode_;

    napi_ref callback_;

    napi_async_work asyncWork_;

    napi_deferred deferred_;

    std::string asyncWorkName_;
};
} // namespace OHOS::Request::Download

#endif /* DOWNLOAD_BASE_CONTEXT_H */
