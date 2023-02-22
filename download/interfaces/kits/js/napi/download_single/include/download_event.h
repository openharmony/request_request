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

#ifndef DOWNLOAD_EVENT_H
#define DOWNLOAD_EVENT_H

#include <string>
#include "download_task.h"
#include "napi/native_api.h"
#include "napi_utils.h"
#include "noncopyable.h"

namespace OHOS::Request::Download {
class DownloadEvent final {
public:
    ACE_DISALLOW_COPY_AND_MOVE(DownloadEvent);

    DownloadEvent() = default;
    ~DownloadEvent() = default;

    static napi_value On(napi_env env, napi_callback_info info);
    static napi_value Off(napi_env env, napi_callback_info info);
    static uint32_t GetParamNumber(const std::string &type);

private:
    static sptr<DownloadNotifyInterface> CreateNotify(napi_env env, const std::string &type, napi_ref callbackRef);
    static void GetCallbackParams(
        napi_env env, const std::string &type, bool result, napi_value (&params)[NapiUtils::TWO_ARG]);
    static std::string Convert2String(napi_env env, napi_value jsString);
};
} // namespace OHOS::Request::Download

#endif // DOWNLOAD_EVENT_H