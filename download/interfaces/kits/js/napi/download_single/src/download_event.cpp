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

#include "download_event.h"

#include "download_base_notify.h"
#include "download_manager.h"
#include "download_task.h"
#include "log.h"

namespace OHOS::Request::Download {
napi_value DownloadEvent::On(napi_env env, napi_callback_info info)
{
    DOWNLOAD_HILOGD("on Enter ---->");
    if (!DownloadManager::GetInstance()->CheckPermission()) {
        DOWNLOAD_HILOGD("no permission to access download service");
        return nullptr;
    }
    napi_value result = nullptr;
    size_t argc = NapiUtils::MAX_ARGC;
    napi_value argv[NapiUtils::MAX_ARGC] = {nullptr};
    napi_value thisVal = nullptr;
    void *data = nullptr;
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, argv, &thisVal, &data));
    if (argc != NapiUtils::TWO_ARG) {
        DOWNLOAD_HILOGE("Wrong number of arguments, requires 1");
        return result;
    }

    napi_valuetype valuetype;
    NAPI_CALL(env, napi_typeof(env, argv[NapiUtils::FIRST_ARGV], &valuetype));
    NAPI_ASSERT(env, valuetype == napi_string, "type is not a string");
    char event[NapiUtils::MAX_LEN] = {0};
    size_t len = 0;
    napi_get_value_string_utf8(env, argv[NapiUtils::FIRST_ARGV], event, NapiUtils::MAX_LEN, &len);
    std::string type = event;
    DOWNLOAD_HILOGD("type : %{public}s", type.c_str());

    valuetype = napi_undefined;
    napi_typeof(env, argv[NapiUtils::SECOND_ARGV], &valuetype);
    NAPI_ASSERT(env, valuetype == napi_function, "callback is not a function");

    DownloadTask *task;
    NAPI_CALL(env, napi_unwrap(env, thisVal, reinterpret_cast<void **>(&task)));
    if (task == nullptr || !task->IsSupportType(type)) {
        DOWNLOAD_HILOGD("Event On type : %{public}s not support", type.c_str());
        return result;
    }
    napi_ref callbackRef = nullptr;
    napi_create_reference(env, argv[argc - 1], 1, &callbackRef);

    sptr<DownloadNotifyInterface> listener = CreateNotify(env, type, callbackRef);
    if (listener == nullptr) {
        DOWNLOAD_HILOGD("DownloadPause create callback object fail");
        return result;
    }
    task->AddListener(type, listener);
    DownloadManager::GetInstance()->On(task->GetId(), type, listener);
    return result;
}

napi_value DownloadEvent::Off(napi_env env, napi_callback_info info)
{
    DOWNLOAD_HILOGD("off Enter ---->");
    if (!DownloadManager::GetInstance()->CheckPermission()) {
        DOWNLOAD_HILOGE("no permission to access download service");
        return nullptr;
    }
    napi_value result = nullptr;
    size_t argc = NapiUtils::MAX_ARGC;
    napi_value argv[NapiUtils::MAX_ARGC] = {nullptr};
    napi_value thisVal = nullptr;
    void *data = nullptr;
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, argv, &thisVal, &data));
    if (argc != NapiUtils::ONE_ARG && argc != NapiUtils::TWO_ARG) {
        DOWNLOAD_HILOGE("Wrong number of arguments, requires 1 or 2");
        return result;
    }
    napi_value callback = nullptr;
    if (argc == NapiUtils::TWO_ARG) {
        callback = argv[NapiUtils::SECOND_ARGV];
    }
    napi_valuetype valuetype = napi_null;
    NAPI_CALL(env, napi_typeof(env, argv[NapiUtils::FIRST_ARGV], &valuetype));
    NAPI_ASSERT(env, valuetype == napi_string, "type is not a string");
    std::string eventType = Convert2String(env, argv[NapiUtils::FIRST_ARGV]);
    DOWNLOAD_HILOGD("eventType : %{public}s", eventType.c_str());

    DownloadTask *task = nullptr;
    NAPI_CALL(env, napi_unwrap(env, thisVal, reinterpret_cast<void **>(&task)));
    if (task == nullptr) {
        DOWNLOAD_HILOGE("Unwrap DownloadTsk failed.");
        return result;
    }
    if (!task->IsSupportType(eventType)) {
        DOWNLOAD_HILOGE("Unkown event type.");
        return result;
    }
    bool isSuccess = DownloadManager::GetInstance()->Off(task->GetId(), eventType);
    if (isSuccess) {
        task->RemoveListener(eventType);
    }
    if (callback == nullptr) {
        return result;
    }
    napi_value params[NapiUtils::TWO_ARG] = { 0 };
    GetCallbackParams(env, eventType, isSuccess, params);
    napi_value returnValue = nullptr;
    NAPI_CALL(env, napi_call_function(env, nullptr, callback, NapiUtils::TWO_ARG, params, &returnValue));
    return result;
}

void DownloadEvent::GetCallbackParams(
    napi_env env, const std::string &type, bool result, napi_value (&params)[NapiUtils::TWO_ARG])
{
    if (type == EVENT_PROGRESS || type == EVENT_FAIL) {
        int ret = 0;
        if (!result) {
            ret = -1;
        }
        DOWNLOAD_HILOGD("ret is:%{public}d", ret);
        params[NapiUtils::FIRST_ARGV] = NapiUtils::CreateInt32(env, ret);
        if (type == EVENT_PROGRESS) {
            params[NapiUtils::SECOND_ARGV] = NapiUtils::CreateInt32(env, ret);
        }
    }
}

std::string DownloadEvent::Convert2String(napi_env env, napi_value jsString)
{
    size_t maxLen = NapiUtils::MAX_LEN;
    napi_status status = napi_get_value_string_utf8(env, jsString, NULL, 0, &maxLen);
    if (status != napi_ok) {
        GET_AND_THROW_LAST_ERROR((env));
        maxLen = NapiUtils::MAX_LEN;
    }
    if (maxLen == 0) {
        return std::string();
    }
    char *buf = new char[maxLen + 1];
    if (buf == nullptr) {
        return std::string();
    }
    size_t len = 0;
    status = napi_get_value_string_utf8(env, jsString, buf, maxLen + 1, &len);
    if (status != napi_ok) {
        GET_AND_THROW_LAST_ERROR((env));
    }
    buf[len] = 0;
    std::string value(buf);
    delete[] buf;
    return value;
}

uint32_t DownloadEvent::GetParamNumber(const std::string &type)
{
    if (type == EVENT_PROGRESS) {
        return TWO_PARAMETER;
    } else if (type == EVENT_FAIL) {
        return ONE_PARAMETER;
    }
    return NO_PARAMETER;
}

sptr<DownloadNotifyInterface> DownloadEvent::CreateNotify(napi_env env, const std::string &type, napi_ref callbackRef)
{
    uint32_t paramNumber = GetParamNumber(type);
    return new (std::nothrow) DownloadBaseNotify(env, paramNumber, callbackRef);
}
} // namespace OHOS::Request::Download
