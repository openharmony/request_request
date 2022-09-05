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

#include <uv.h>
#include <regex>

#include "ability.h"
#include "async_call.h"
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

static constexpr const char *FUNCTION_ON = "on";
static constexpr const char *FUNCTION_OFF = "off";
static constexpr const char *FUNCTION_PAUSE = "pause";
static constexpr const char *FUNCTION_QUERY = "query";
static constexpr const char *FUNCTION_QUERYMIMETYPE = "queryMimeType";
static constexpr const char *FUNCTION_REMOVE = "remove";
static constexpr const char *FUNCTION_RESUME = "resume";

static constexpr const char *PARAM_KEY_URI = "url";
static constexpr const char *PARAM_KEY_HEADER = "header";
static constexpr const char *PARAM_KEY_METERED = "enableMetered";
static constexpr const char *PARAM_KEY_ROAMING = "enableRoaming";
static constexpr const char *PARAM_KEY_DESCRIPTION = "description";
static constexpr const char *PARAM_KEY_NETWORKTYPE = "networkType";
static constexpr const char *PARAM_KEY_FILE_PATH = "filePath";
static constexpr const char *PARAM_KEY_TITLE = "title";

namespace OHOS::Request::Download {
__thread napi_ref DownloadTaskNapi::globalCtor = nullptr;
napi_value DownloadTaskNapi::JsMain(napi_env env, napi_callback_info info)
{
    DOWNLOAD_HILOGD("Enter download JsMain.");
    if (Legacy::DownloadManager::IsLegacy(env, info)) {
        DOWNLOAD_HILOGD("Enter download legacy.");
        return Legacy::DownloadManager::Download(env, info);
    }

    if (!DownloadManager::GetInstance()->CheckPermission()) {
        DOWNLOAD_HILOGD("no permission to access download service");
        return nullptr;
    }
    struct ContextInfo {
        napi_ref ref = nullptr;
    };
    auto ctxInfo = std::make_shared<ContextInfo>();
    auto input = [ctxInfo](napi_env env, size_t argc, napi_value *argv, napi_value self) -> napi_status {
        DOWNLOAD_HILOGD("download parser to native params %{public}d!", static_cast<int>(argc));
        NAPI_ASSERT_BASE(env, (argc > 0) && (argc <= 2), " need 1 or 2 parameters!", napi_invalid_arg);
        napi_value proxy = nullptr;
        napi_status status = napi_new_instance(env, GetCtor(env), argc, argv, &proxy);
        if ((proxy == nullptr) || (status != napi_ok)) {
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
    AsyncCall asyncCall(env, info, context, 1);
    return asyncCall.Call(env);
}

napi_status DownloadTaskNapi::OnHeaderReceive(
    napi_env env, size_t argc, napi_value *argv, napi_value self, napi_value *result)
{
    DOWNLOAD_HILOGD("Enter OnHeaderReceive.");
    return napi_ok;
}

napi_value DownloadTaskNapi::GetCtor(napi_env env)
{
    napi_value cons;
    if (globalCtor != nullptr) {
        NAPI_CALL(env, napi_get_reference_value(env, globalCtor, &cons));
        return cons;
    }

    napi_property_descriptor clzDes[] = {
        {FUNCTION_ON, 0, DownloadEvent::On, 0, 0, 0, napi_default, 0},
        {FUNCTION_OFF, 0, DownloadEvent::Off, 0, 0, 0, napi_default, 0},
        {FUNCTION_PAUSE, 0, DownloadPause::Exec, 0, 0, 0, napi_default, 0},
        {FUNCTION_QUERY, 0, DownloadQuery::Exec, 0, 0, 0, napi_default, 0},
        {FUNCTION_QUERYMIMETYPE, 0, DownloadQueryMimeType::Exec, 0, 0, 0, napi_default, 0},
        {FUNCTION_REMOVE, 0, DownloadRemove::Exec, 0, 0, 0, napi_default, 0},
        {FUNCTION_RESUME, 0, DownloadResume::Exec, 0, 0, 0, napi_default, 0},
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
    size_t argc = NapiUtils::MAX_ARGC;
    napi_value argv[NapiUtils::MAX_ARGC] = {nullptr};
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, argv, &self, nullptr));

    DownloadConfig config;
    if (!ParseConfig(env, argv[0], config)) {
        DOWNLOAD_HILOGE("download config has wrong type");
        return nullptr;
    }
    DownloadManager::GetInstance()->SetDataAbilityHelper(GetDataAbilityHelper(env));
    auto *task = DownloadManager::GetInstance()->EnqueueTask(config);

    auto finalize = [](napi_env env, void *data, void *hint) {
        DOWNLOAD_HILOGD("destructed download task");
        DownloadTask *task = reinterpret_cast<DownloadTask *>(data);
        delete task;
    };
    if (napi_wrap(env, self, task, finalize, nullptr, nullptr) != napi_ok) {
        finalize(env, task, nullptr);
        return nullptr;
    }
    return self;
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
        DOWNLOAD_HILOGD("No header present, ignore it");
        return true;
    }
    napi_value header = NapiUtils::GetNamedProperty(env, configValue, PARAM_KEY_HEADER);
    if (NapiUtils::GetValueType(env, header) != napi_object) {
        return false;
    }
    auto names = NapiUtils::GetPropertyNames(env, header);
    std::vector<std::string>::iterator iter;
    DOWNLOAD_HILOGD("current name list size = %{public}d", names.size());
    for (iter = names.begin(); iter != names.end(); ++iter) {
        auto value = NapiUtils::GetStringPropertyUtf8(env, header, *iter);
        if (!value.empty()) {
            config.SetHeader(NapiUtils::ToLower(*iter), value);
        }
    }
    return true;
}

std::shared_ptr<OHOS::AppExecFwk::DataAbilityHelper> dataAbilityHelper_ = nullptr;

std::shared_ptr<OHOS::AppExecFwk::DataAbilityHelper> DownloadTaskNapi::GetDataAbilityHelper(napi_env env)
{
    napi_value global = nullptr;
    NAPI_CALL(env, napi_get_global(env, &global));
    napi_value abilityObj = nullptr;
    NAPI_CALL(env, napi_get_named_property(env, global, "ability", &abilityObj));
    if (abilityObj == nullptr) {
        DOWNLOAD_HILOGE("abilityObj is nullptr!");
    }
    OHOS::AppExecFwk::Ability *ability = nullptr;
    NAPI_CALL(env, napi_get_value_external(env, abilityObj, (void **)&ability));
    if (ability == nullptr) {
        DOWNLOAD_HILOGE("ability is nullptr!");
    }
    std::shared_ptr<OHOS::Uri> uriPtr = std::make_shared<OHOS::Uri>("dataability:///com.ohos.download");
    if (dataAbilityHelper_ == nullptr) {
        DOWNLOAD_HILOGE("Try to get dataAbilityHelper_!");
        dataAbilityHelper_ = OHOS::AppExecFwk::DataAbilityHelper::Creator(ability->GetContext(), uriPtr);
    }
    if (dataAbilityHelper_ != nullptr) {
        DOWNLOAD_HILOGE("dataAbilityHelper_ is not nullptr!");
    }
    return dataAbilityHelper_;
}
} // namespace OHOS::Request::Download
