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

#include "download_task_napi.h"

#include <mutex>
#include <uv.h>
#include <regex>
#include <unistd.h>

#include "ability.h"
#include "async_call.h"
#include "constant.h"
#include "download_event.h"
#include "download_manager.h"
#include "download_pause.h"
#include "download_query.h"
#include "download_query_mimetype.h"
#include "download_remove.h"
#include "download_resume.h"
#include "log.h"
#include "napi_utils.h"
#include "legacy/download_manager.h"
#include "napi_base_context.h"

static constexpr const char *FUNCTION_ON = "on";
static constexpr const char *FUNCTION_OFF = "off";

static constexpr const char *PARAM_KEY_URI = "url";
static constexpr const char *PARAM_KEY_HEADER = "header";
static constexpr const char *PARAM_KEY_METERED = "enableMetered";
static constexpr const char *PARAM_KEY_ROAMING = "enableRoaming";
static constexpr const char *PARAM_KEY_DESCRIPTION = "description";
static constexpr const char *PARAM_KEY_NETWORKTYPE = "networkType";
static constexpr const char *PARAM_KEY_FILE_PATH = "filePath";
static constexpr const char *PARAM_KEY_TITLE = "title";
static constexpr const char *PARAM_KEY_BACKGROUND = "background";

namespace OHOS::Request::Download {
__thread napi_ref DownloadTaskNapi::globalCtor = nullptr;
namespace {
constexpr const std::uint32_t CONFIG_PARAM_AT_FIRST = 0;
constexpr const std::uint32_t CONFIG_PARAM_AT_SECOND = 1;
std::mutex mutex_;
}
napi_value DownloadTaskNapi::JsMain(napi_env env, napi_callback_info info)
{
    DOWNLOAD_HILOGD("Enter download JsMain.");
    if (Legacy::DownloadManager::IsLegacy(env, info)) {
        DOWNLOAD_HILOGD("Enter download legacy.");
        return Legacy::DownloadManager::Download(env, info);
    }
    if (!DownloadManager::GetInstance()->LoadDownloadServer()) {
        DOWNLOAD_HILOGE("load download server fail");
        return nullptr;
    }
    if (!DownloadManager::GetInstance()->CheckPermission()) {
        DOWNLOAD_HILOGE("no permission to access download service");
        return nullptr;
    }
    struct ContextInfo {
        napi_ref ref = nullptr;
    };
    auto ctxInfo = std::make_shared<ContextInfo>();
    auto input = [ctxInfo](napi_env env, size_t argc, napi_value *argv, napi_value self) -> napi_status {
        DOWNLOAD_HILOGD("download parser to native params %{public}d!", static_cast<int>(argc));
        NAPI_ASSERT_BASE(env, (argc > 0) && (argc <= 3), " need 2 or 3 parameters!", napi_invalid_arg);
        napi_value proxy = nullptr;
        napi_status status = napi_new_instance(env, GetCtor(env), argc, argv, &proxy);
        if ((proxy == nullptr) || (status != napi_ok)) {
            DOWNLOAD_HILOGE("Failed to create download task");
            return napi_generic_failure;
        }
        napi_create_reference(env, proxy, 1, &(ctxInfo->ref));
        return napi_ok;
    };
    auto output = [ctxInfo](napi_env env, napi_value *result) -> napi_status {
        napi_status status = napi_get_reference_value(env, ctxInfo->ref, result);
        napi_delete_reference(env, ctxInfo->ref);
        return status;
    };
    auto context = std::make_shared<AsyncCall::Context>(input, output);
    AsyncCall asyncCall(env, info, context, "", ASYNC_DEFAULT_POS);
    return asyncCall.Call(env);
}

napi_value DownloadTaskNapi::GetCtor(napi_env env)
{
    std::lock_guard<std::mutex> lock(mutex_);
    napi_value cons;
    if (globalCtor != nullptr) {
        NAPI_CALL(env, napi_get_reference_value(env, globalCtor, &cons));
        return cons;
    }

    napi_property_descriptor clzDes[] = {
        {FUNCTION_ON, 0, DownloadEvent::On, 0, 0, 0, napi_default, 0},
        {FUNCTION_OFF, 0, DownloadEvent::Off, 0, 0, 0, napi_default, 0},
        {FUNCTION_PAUSE, 0, DownloadPause::Pause, 0, 0, 0, napi_default, 0},
        {FUNCTION_QUERY, 0, DownloadQuery::Query, 0, 0, 0, napi_default, 0},
        {FUNCTION_QUERY_MIME_TYPE, 0, DownloadQueryMimeType::QueryMimeType, 0, 0, 0, napi_default, 0},
        {FUNCTION_REMOVE, 0, DownloadRemove::Remove, 0, 0, 0, napi_default, 0},
        {FUNCTION_RESUME, 0, DownloadResume::Resume, 0, 0, 0, napi_default, 0},
    };
    NAPI_CALL(env, napi_define_class(env, "DownloadTaskNapi", NAPI_AUTO_LENGTH, Initialize, nullptr,
                       sizeof(clzDes) / sizeof(napi_property_descriptor), clzDes, &cons));
    NAPI_CALL(env, napi_create_reference(env, cons, 1, &globalCtor));
    return cons;
}

napi_value DownloadTaskNapi::Initialize(napi_env env, napi_callback_info info)
{
    DOWNLOAD_HILOGD("constructor download task!");
    napi_value self = nullptr;
    int parametersPosition = CONFIG_PARAM_AT_FIRST;
    size_t argc = NapiUtils::MAX_ARGC;
    napi_value argv[NapiUtils::MAX_ARGC] = {nullptr};
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, argv, &self, nullptr));

    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;
    napi_status getStatus = GetContext(env, argv[0], parametersPosition, context);
    if (getStatus != napi_ok) {
        DOWNLOAD_HILOGE("Initialize. GetContext fail.");
        NAPI_ASSERT(env, false, "Initialize. GetContext fail.");
        return nullptr;
    }
    if (context->GetApplicationInfo() == nullptr) {
        DOWNLOAD_HILOGE("ApplicationInfo is null");
        NAPI_ASSERT(env, false, "ApplicationInfo is null");
        return nullptr;
    }
    DownloadConfig config;
    if (!ParseConfig(env, argv[parametersPosition], config)) {
        DOWNLOAD_HILOGE("download config has wrong type");
        NAPI_ASSERT(env, false, "download config has wrong type");
        return nullptr;
    }
    config.SetBundleName(context->GetBundleName());
    config.SetApplicationInfoUid(static_cast<int32_t>(getuid()));
    ExceptionError err;
    auto *task = DownloadManager::GetInstance()->EnqueueTask(config, err);
    if (task == nullptr) {
        DOWNLOAD_HILOGE("download task fail: %{public}s", err.errInfo.c_str());
        NapiUtils::ThrowError(env, err.code, err.errInfo);
        return nullptr;
    }
    auto finalize = [](napi_env env, void *data, void *hint) {
        DOWNLOAD_HILOGD("destructed download task");
        DownloadTask *task = reinterpret_cast<DownloadTask *>(data);
        delete task;
    };
    if (napi_wrap(env, self, task, finalize, nullptr, nullptr) != napi_ok) {
        finalize(env, task, nullptr);
        NAPI_ASSERT(env, false, "napi_wrap fail");
        return nullptr;
    }
    DOWNLOAD_HILOGD("Succeed to allocate download task");
    return self;
}

napi_status DownloadTaskNapi::GetContext(napi_env env, napi_value value, int& parametersPosition,
    std::shared_ptr<OHOS::AbilityRuntime::Context>& context)
{
    if (!IsStageMode(env, value)) {
        auto ability = OHOS::AbilityRuntime::GetCurrentAbility(env);
        if (ability == nullptr) {
            DOWNLOAD_HILOGE("GetContext. L7. GetCurrentAbility ability == nullptr.");
            return napi_generic_failure;
        }
        context = ability->GetAbilityContext();
        DOWNLOAD_HILOGE("GetContext. L7");
    } else {
        parametersPosition = CONFIG_PARAM_AT_SECOND;
        context = OHOS::AbilityRuntime::GetStageModeContext(env, value);
        if (context == nullptr) {
            DOWNLOAD_HILOGE("GetContext. L8. GetStageModeContext contextRtm == nullptr.");
            return napi_generic_failure;
        }
        DOWNLOAD_HILOGE("GetContext. L8");
    }
    if (context == nullptr) {
        DOWNLOAD_HILOGE("GetContext failed. context is nullptr.");
        return napi_generic_failure;
    }
    return napi_ok;
}

bool DownloadTaskNapi::ParseConfig(napi_env env, napi_value configValue, DownloadConfig &config)
{
    if (!ParseHeader(env, configValue, config)) {
        return false;
    }
    if (!ParseUrl(env, configValue, config)) {
        DOWNLOAD_HILOGE("Input url error");
        return false;
    }
    config.SetMetered(NapiUtils::GetBooleanProperty(env, configValue, PARAM_KEY_METERED));
    config.SetRoaming(NapiUtils::GetBooleanProperty(env, configValue, PARAM_KEY_ROAMING));
    config.SetDescription(NapiUtils::GetStringPropertyUtf8(env, configValue, PARAM_KEY_DESCRIPTION));
    config.SetNetworkType(NapiUtils::GetUint32Property(env, configValue, PARAM_KEY_NETWORKTYPE));
    config.SetFilePath(NapiUtils::GetStringPropertyUtf8(env, configValue, PARAM_KEY_FILE_PATH));
    config.SetTitle(NapiUtils::GetStringPropertyUtf8(env, configValue, PARAM_KEY_TITLE));
    config.SetBackground(NapiUtils::GetBooleanProperty(env, configValue, PARAM_KEY_BACKGROUND));
    return true;
}

bool DownloadTaskNapi::ParseUrl(napi_env env, napi_value configValue, DownloadConfig &config)
{
    std::string url = NapiUtils::GetStringPropertyUtf8(env, configValue, PARAM_KEY_URI);
    if (!regex_match(url, std::regex("^http(s)?:\\/\\/.+"))) {
        return false;
    }
    config.SetUrl(url);
    return true;
}

bool DownloadTaskNapi::ParseHeader(napi_env env, napi_value configValue, DownloadConfig &config)
{
    if (!NapiUtils::HasNamedProperty(env, configValue, PARAM_KEY_HEADER)) {
        DOWNLOAD_HILOGD("No header present, Reassign value");
        config.SetHeader(tlsVersion, TLS_VERSION);
        config.SetHeader(cipherList, TLS_CIPHER);
        return true;
    }
    napi_value header = NapiUtils::GetNamedProperty(env, configValue, PARAM_KEY_HEADER);
    if (NapiUtils::GetValueType(env, header) != napi_object) {
        return false;
    }
    auto names = NapiUtils::GetPropertyNames(env, header);
    std::vector<std::string>::iterator iter;
    DOWNLOAD_HILOGD("current name list size = %{public}zu", names.size());
    iter = find(names.begin(), names.end(), cipherList);
    if (iter == names.end()) {
        config.SetHeader(cipherList, TLS_CIPHER);
        names = NapiUtils::GetPropertyNames(env, header);
    }
    for (iter = names.begin(); iter != names.end(); ++iter) {
        auto value = NapiUtils::GetStringPropertyUtf8(env, header, *iter);
        if (!value.empty()) {
            config.SetHeader(NapiUtils::ToLower(*iter), value);
        }
    }
    return true;
}

bool DownloadTaskNapi::IsStageMode(napi_env env, napi_value value)
{
    bool stageMode = true;
    napi_status status = OHOS::AbilityRuntime::IsStageContext(env, value, stageMode);
    if (status != napi_ok || !stageMode) {
        return false;
    }
    return stageMode;
}
} // namespace OHOS::Request::Download
