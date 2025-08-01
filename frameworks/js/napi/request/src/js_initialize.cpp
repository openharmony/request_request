/*
 * Copyright (C) 2023 Huawei Device Co., Ltd.
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

#include "js_initialize.h"

#include <fcntl.h>
#include <securec.h>
#include <sys/stat.h>

#include <algorithm>
#include <cstdio>
#include <cstring>
#include <filesystem>
#include <fstream>
#include <new>
#include <regex>
#include <string>
#include <system_error>

#include "file_uri.h"
#include "log.h"
#include "napi_utils.h"
#include "network_security_config.h"
#include "parameter.h"
#include "request_common.h"
#include "request_manager.h"
#include "sys_event.h"

static constexpr const char *PARAM_KEY_DESCRIPTION = "description";
static constexpr const char *PARAM_KEY_NETWORKTYPE = "networkType";
static constexpr const char *PARAM_KEY_FILE_PATH = "filePath";
static constexpr const char *PARAM_KEY_BACKGROUND = "background";
static constexpr uint32_t FILE_PERMISSION = 0644;
static constexpr uint32_t TITLE_MAXIMUM = 256;
static constexpr uint32_t DESCRIPTION_MAXIMUM = 1024;
static constexpr uint32_t URL_MAXIMUM = 8192;
static constexpr uint32_t NOTIFICATION_TITLE_MAXIMUM = 1024;
static constexpr uint32_t NOTIFICATION_TEXT_MAXIMUM = 3072;
static constexpr uint32_t PROXY_MAXIMUM = 512;
static constexpr uint32_t MAX_UPLOAD_ON15_FILES = 100;
static constexpr uint32_t MIN_TIMEOUT = 1;
static constexpr uint32_t MAX_TIMEOUT = 604800;

namespace OHOS::Request {
napi_value JsInitialize::Initialize(napi_env env, napi_callback_info info, Version version, bool firstInit)
{
    REQUEST_HILOGD("constructor request task!");
    bool withErrCode = version != Version::API8;
    napi_value self = nullptr;
    size_t argc = NapiUtils::MAX_ARGC;
    napi_value argv[NapiUtils::MAX_ARGC] = { nullptr };
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, argv, &self, nullptr));
    int32_t number = version == Version::API8 ? NapiUtils::ONE_ARG : NapiUtils::TWO_ARG;
    if (static_cast<int32_t>(argc) < number) {
        NapiUtils::ThrowError(
            env, E_PARAMETER_CHECK, "Missing mandatory parameters, invalid parameter count", withErrCode);
        return nullptr;
    }

    Config config;
    config.version = version;
    config.withErrCode = withErrCode;
    config.firstInit = firstInit;
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;
    ExceptionError err = InitParam(env, argv, context, config);
    if (err.code != E_OK) {
        REQUEST_HILOGE("err.code : %{public}d, err.errInfo :  %{public}s", err.code, err.errInfo.c_str());
        NapiUtils::ThrowError(env, err.code, err.errInfo, withErrCode);
        return nullptr;
    }

    auto *task = new (std::nothrow) JsTask();
    if (task == nullptr) {
        REQUEST_HILOGE("Create task object failed");
        return nullptr;
    }
    task->config_ = config;
    task->isGetPermission = true;
    RequestManager::GetInstance()->RestoreListener(JsTask::ReloadListener);
    // `finalize` executes on the JS thread
    auto finalize = [](napi_env env, void *data, void *hint) {
        REQUEST_HILOGD("destructed task");
        JsTask *task = reinterpret_cast<JsTask *>(data);
        JsTask::ClearTaskMap(task->GetTid());
        RequestManager::GetInstance()->RemoveAllListeners(task->GetTid());
        delete task;
    };
    if (napi_wrap(env, self, task, finalize, nullptr, nullptr) != napi_ok) {
        finalize(env, task, nullptr);
        return nullptr;
    }
    return self;
}

ExceptionError JsInitialize::InitParam(
    napi_env env, napi_value *argv, std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config)
{
    REQUEST_HILOGD("InitParam in");
    ExceptionError err = { .code = E_OK };
    int parametersPosition = config.version == Version::API8 ? CONFIG_PARAM_AT_FIRST : CONFIG_PARAM_AT_SECOND;

    napi_status getStatus = GetContext(env, argv[0], context);
    if (getStatus != napi_ok) {
        REQUEST_HILOGE("Get context fail");
        err.code = E_PARAMETER_CHECK;
        err.errInfo = "Parameter verification failed, Get context fail";
        return err;
    }
    auto applicationInfo = context->GetApplicationInfo();
    if (applicationInfo == nullptr) {
        err.code = E_OTHER;
        err.errInfo = "ApplicationInfo is null";
        SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_03, err.errInfo);
        return err;
    }
    config.bundleType = static_cast<u_int32_t>(applicationInfo->bundleType);
    REQUEST_HILOGD("config.bundleType is %{public}d", config.bundleType);
    if (!ParseConfig(env, argv[parametersPosition], config, err.errInfo)) {
        err.code = E_PARAMETER_CHECK;
        return err;
    }
    config.bundleName = context->GetBundleName();
    REQUEST_HILOGD("config.bundleName is %{public}s", config.bundleName.c_str());
    CheckFilePath(context, config, err);
    return err;
}

napi_status JsInitialize::GetContext(
    napi_env env, napi_value value, std::shared_ptr<OHOS::AbilityRuntime::Context> &context)
{
    if (!IsStageMode(env, value)) {
        auto ability = OHOS::AbilityRuntime::GetCurrentAbility(env);
        if (ability == nullptr) {
            REQUEST_HILOGE("Get current ability fail");
            SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_04, "Get current ability fail");
            return napi_generic_failure;
        }
        context = ability->GetAbilityContext();
    } else {
        context = OHOS::AbilityRuntime::GetStageModeContext(env, value);
    }
    if (context == nullptr) {
        REQUEST_HILOGE("Get Context failed, context is nullptr.");
        SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_06, "Get Context failed");
        return napi_generic_failure;
    }
    return napi_ok;
}

bool JsInitialize::GetAppBaseDir(std::string &baseDir)
{
    auto context = AbilityRuntime::Context::GetApplicationContext();
    if (context == nullptr) {
        REQUEST_HILOGE("AppContext is null.");
        SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_02, "AppContext is null");
        return false;
    }
    baseDir = context->GetBaseDir();
    if (baseDir.empty()) {
        REQUEST_HILOGE("Base dir not found.");
        SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_07, "Base dir not found");
        return false;
    }
    return true;
}

bool JsInitialize::CheckFilePath(
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config, ExceptionError &error)
{
    if (config.action == Action::DOWNLOAD) {
        if (!CheckDownloadFile(context, config, error)) {
            SysEventLog::SendSysEventLog(STATISTIC_EVENT, APP_ERROR_00, config.bundleName, "", error.errInfo);
            return false;
        }
    } else {
        if (!CheckUploadFiles(context, config, error)) {
            SysEventLog::SendSysEventLog(STATISTIC_EVENT, APP_ERROR_01, config.bundleName, "", error.errInfo);
            return false;
        }
        std::string filePath = context->GetCacheDir();
        if (!CheckUploadBodyFiles(filePath, config, error)) {
            SysEventLog::SendSysEventLog(STATISTIC_EVENT, APP_ERROR_02, config.bundleName, "", error.errInfo);
            return false;
        }
    }
    if (!JsTask::SetDirsPermission(config.certsPath)) {
        error.code = E_FILE_IO;
        error.errInfo = "set files of directors permission fail";
        SysEventLog::SendSysEventLog(FAULT_EVENT, TASK_FAULT_02, config.bundleName, "", error.errInfo);
        return false;
    }
    return true;
}

bool JsInitialize::CheckUploadBodyFiles(const std::string &filePath, Config &config, ExceptionError &error)
{
    size_t len = config.files.size();
    if (config.multipart) {
        len = 1;
    }

    for (size_t i = 0; i < len; i++) {
        if (filePath.empty()) {
            REQUEST_HILOGE("internal to cache error");
            error.code = E_PARAMETER_CHECK;
            error.errInfo = "Parameter verification failed, UploadBodyFiles error empty path";
            return false;
        }
        auto now = std::chrono::high_resolution_clock::now();
        auto timestamp = std::chrono::duration_cast<std::chrono::nanoseconds>(now.time_since_epoch()).count();
        std::string path = filePath + "/tmp_body_" + std::to_string(i) + "_" + std::to_string(timestamp);
        if (!NapiUtils::IsPathValid(path)) {
            REQUEST_HILOGE("Upload IsPathValid error");
            error.code = E_PARAMETER_CHECK;
            error.errInfo = "Parameter verification failed, UploadBodyFiles error fail path";
            return false;
        }
        FILE *bodyFile = fopen(path.c_str(), "w+");
        if (bodyFile == NULL) {
            error.code = E_FILE_IO;
            error.errInfo = "UploadBodyFiles failed to open file errno " + std::to_string(errno);
            SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_00, config.bundleName, "", error.errInfo);
            return false;
        }
        int32_t ret = chmod(path.c_str(), S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH);
        if (ret != 0) {
            REQUEST_HILOGE("body chmod fail: %{public}d", ret);
            SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_01, config.bundleName, "", std::to_string(ret));
        };

        bool setRes = JsTask::SetPathPermission(path);
        int32_t retClose = fclose(bodyFile);
        if (retClose != 0) {
            REQUEST_HILOGE("upload body fclose fail: %{public}d", ret);
            SysEventLog::SendSysEventLog(
                FAULT_EVENT, STANDARD_FAULT_02, config.bundleName, "", std::to_string(retClose));
        }
        if (!setRes) {
            error.code = E_FILE_IO;
            error.errInfo = "UploadBodyFiles set body path permission fail";
            return false;
        }
        config.bodyFileNames.push_back(path);
    }
    return true;
}

bool JsInitialize::CheckPathIsFile(const std::string &path, ExceptionError &error)
{
    std::error_code err;
    if (!std::filesystem::exists(path, err)) {
        error.code = E_FILE_IO;
        error.errInfo = "Path not exists: " + err.message();
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_03, error.errInfo);
        return false;
    }
    if (std::filesystem::is_directory(path, err)) {
        error.code = E_FILE_IO;
        error.errInfo = "Path not File: " + err.message();
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_04, error.errInfo);
        return false;
    }
    return true;
}

bool JsInitialize::GetFdDownload(const std::string &path, const Config &config, ExceptionError &error)
{
    // File is exist.
    if (JsInitialize::FindDir(path)) {
        if (config.firstInit && !config.overwrite) {
            error.code = config.version == Version::API10 ? E_FILE_IO : E_FILE_PATH;
            error.errInfo = "GetFd File exists and other error";
            SysEventLog::SendSysEventLog(STATISTIC_EVENT, APP_ERROR_00, config.bundleName, "", error.errInfo);
            return false;
        }
    }

    FILE *file = NULL;
    if (config.firstInit) {
        file = fopen(path.c_str(), "w+");
    } else {
        file = fopen(path.c_str(), "a+");
    }

    if (file == NULL) {
        error.code = E_FILE_IO;
        error.errInfo = "GetFd failed to open file errno " + std::to_string(errno);
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_00, config.bundleName, "", error.errInfo);
        return false;
    }

    int32_t ret = chmod(path.c_str(), S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH);
    if (ret != 0) {
        REQUEST_HILOGE("download file chmod fail: %{public}d", ret);
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_01, config.bundleName, "", std::to_string(ret));
    };

    int32_t retClose = fclose(file);
    if (retClose != 0) {
        REQUEST_HILOGE("download fclose fail: %{public}d", ret);
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_02, config.bundleName, "", std::to_string(retClose));
    }
    return true;
}

bool JsInitialize::GetFdUpload(const std::string &path, const Config &config, ExceptionError &error)
{
    if (!JsInitialize::CheckPathIsFile(path, error)) {
        error.code = config.version == Version::API10 ? E_FILE_IO : E_FILE_PATH;
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_03, config.bundleName, "", error.errInfo);
        return false;
    }
    FILE *file = fopen(path.c_str(), "r");
    if (file == NULL) {
        error.code = config.version == Version::API10 ? E_FILE_IO : E_FILE_PATH;
        error.errInfo = "GetFd failed to open file errno " + std::to_string(errno);
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_00, config.bundleName, "", error.errInfo);
        return false;
    }
    REQUEST_HILOGD("upload file fopen ok");
    int32_t ret = chmod(path.c_str(), S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH);
    if (ret != 0) {
        REQUEST_HILOGE("upload file chmod fail: %{public}d", ret);
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_01, config.bundleName, "", std::to_string(ret));
    }
    int32_t retClose = fclose(file);
    if (retClose != 0) {
        REQUEST_HILOGE("upload fclose fail: %{public}d", ret);
        SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_02, config.bundleName, "", std::to_string(retClose));
    }
    return true;
}

bool JsInitialize::GetInternalPath(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config,
    std::string &path, std::string &errInfo)
{
    std::string fileName;
    std::string pattern = "internal://cache/";
    size_t pos = path.find(pattern);
    if (pos != 0) {
        fileName = path;
    } else {
        fileName = path.substr(pattern.size(), path.size());
    }
    if (fileName.empty()) {
        errInfo = "Parameter verification failed, GetInternalPath failed, fileName is empty";
        return false;
    }
    path = context->GetCacheDir();
    if (path.empty()) {
        REQUEST_HILOGE("internal to cache error");
        errInfo = "Parameter verification failed, GetInternalPath failed, cache path is empty";
        return false;
    }
    path += "/" + fileName;
    if (!NapiUtils::IsPathValid(path)) {
        REQUEST_HILOGE("IsPathValid error");
        errInfo = "Parameter verification failed, GetInternalPath failed, filePath is not valid";
        return false;
    }
    return true;
}

void JsInitialize::SetParseConfig(napi_env env, napi_value jsConfig, Config &config)
{
    config.overwrite = NapiUtils::Convert2Boolean(env, jsConfig, "overwrite");
    config.metered = NapiUtils::Convert2Boolean(env, jsConfig, "metered");
    config.gauge = NapiUtils::Convert2Boolean(env, jsConfig, "gauge");
    config.precise = NapiUtils::Convert2Boolean(env, jsConfig, "precise");
    config.priority = ParsePriority(env, jsConfig);
    config.begins = ParseBegins(env, jsConfig);
    config.ends = ParseEnds(env, jsConfig);
    config.mode = static_cast<Mode>(NapiUtils::Convert2Uint32(env, jsConfig, "mode"));
    config.headers = ParseMap(env, jsConfig, "headers");
    config.extras = ParseMap(env, jsConfig, "extras");
    config.multipart = NapiUtils::Convert2Boolean(env, jsConfig, "multipart");
    if (config.mode == Mode::BACKGROUND) {
        config.background = true;
    }
}

void JsInitialize::ParseConfigInner(napi_env env, napi_value jsConfig, Config &config)
{
    ParseCertificatePins(env, config.url, config.certificatePins);
    ParseMethod(env, jsConfig, config);
    ParseRoaming(env, jsConfig, config);
    ParseRedirect(env, jsConfig, config.redirect);
    ParseNetwork(env, jsConfig, config.network);
    ParseRetry(env, jsConfig, config.retry);
    SetParseConfig(env, jsConfig, config);
}

bool JsInitialize::ParseConfig(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo)
{
    if (NapiUtils::GetValueType(env, jsConfig) != napi_object) {
        errInfo = "Incorrect parameter type, Wrong config type, expected object";
        return false;
    }
    if (config.version != Version::API10) {
        return ParseConfigV9(env, jsConfig, config, errInfo);
    }

    if (!ParseAction(env, jsConfig, config.action, errInfo)) {
        return false;
    }
    if (!ParseUrl(env, jsConfig, config.url, errInfo)) {
        return false;
    }
    if (!ParseCertsPath(env, jsConfig, config.certsPath, errInfo)) {
        return false;
    }
    if (!ParseData(env, jsConfig, config, errInfo)) {
        return false;
    }
    if (!ParseIndex(env, jsConfig, config, errInfo)) {
        return false;
    }
    if (!ParseProxy(env, jsConfig, config.proxy, errInfo)) {
        return false;
    }
    if (!ParseTitle(env, jsConfig, config, errInfo) || !ParseToken(env, jsConfig, config, errInfo)
        || !ParseDescription(env, jsConfig, config.description, errInfo)) {
        return false;
    }
    if (!ParseSaveas(env, jsConfig, config, errInfo)) {
        return false;
    }
    if (!ParseNotification(env, jsConfig, config, errInfo)) {
        return false;
    }
    if (!ParseMinSpeed(env, jsConfig, config, errInfo)) {
        return false;
    }
    if (!ParseTimeout(env, jsConfig, config, errInfo)) {
        return false;
    }
    ParseConfigInner(env, jsConfig, config);
    return true;
}

void JsInitialize::ParseRoaming(napi_env env, napi_value jsConfig, Config &config)
{
    if (!NapiUtils::HasNamedProperty(env, jsConfig, "roaming")) {
        config.roaming = config.version == Version::API10;
    } else {
        config.roaming = NapiUtils::Convert2Boolean(env, jsConfig, "roaming");
    }
}

bool JsInitialize::ParseNotification(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo)
{
    napi_value notification = NapiUtils::GetNamedProperty(env, jsConfig, "notification");
    if (NapiUtils::GetValueType(env, notification) != napi_undefined) {
        if (NapiUtils::GetValueType(env, NapiUtils::GetNamedProperty(env, notification, "title")) != napi_undefined) {
            config.notification.title = NapiUtils::Convert2String(env, notification, "title");
            if (config.notification.title->size() > NOTIFICATION_TITLE_MAXIMUM) {
                errInfo = "Parameter verification failed, notification.title length exceeds the maximum limit";
                return false;
            }
        }
        if (NapiUtils::GetValueType(env, NapiUtils::GetNamedProperty(env, notification, "text")) != napi_undefined) {
            config.notification.text = NapiUtils::Convert2String(env, notification, "text");
            if (config.notification.text->size() > NOTIFICATION_TEXT_MAXIMUM) {
                errInfo = "Parameter verification failed, notification.text length exceeds the maximum limit";
                return false;
            }
        }
        if (NapiUtils::GetValueType(env, NapiUtils::GetNamedProperty(env, notification, "disable"))
            != napi_undefined) {
            config.notification.disable = NapiUtils::Convert2Boolean(env, notification, "disable");
        }
    }
    return true;
}

bool JsInitialize::ParseMinSpeed(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo)
{
    napi_value minSpeed = NapiUtils::GetNamedProperty(env, jsConfig, "minSpeed");
    if (NapiUtils::GetValueType(env, minSpeed) != napi_undefined) {
        napi_value value = NapiUtils::GetNamedProperty(env, minSpeed, "speed");
        auto ty = NapiUtils::GetValueType(env, value);
        if (ty != napi_undefined) {
            if (ty != napi_number) {
                REQUEST_HILOGE("GetNamedProperty err");
                errInfo = "Incorrect parameter type, minSpeed.speed type is not of napi_number type";
                return false;
            }
            config.minSpeed.speed = NapiUtils::Convert2Int64(env, value);
            if (config.minSpeed.speed < 0) {
                errInfo = "Parameter verification failed, minSpeed.speed must be greater than or equal to 0";
                return false;
            }
        }
        value = NapiUtils::GetNamedProperty(env, minSpeed, "duration");
        ty = NapiUtils::GetValueType(env, value);
        if (ty != napi_undefined) {
            if (ty != napi_number) {
                REQUEST_HILOGE("GetNamedProperty err");
                errInfo = "Incorrect parameter type, minSpeed.duration type is not of napi_number type";
                return false;
            }
            config.minSpeed.duration = NapiUtils::Convert2Int64(env, value);
            if (config.minSpeed.duration < 0) {
                errInfo = "Parameter verification failed, minSpeed.duration must be greater than or equal to 0";
                return false;
            }
        }
    }
    return true;
}

bool JsInitialize::ParseTimeout(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo)
{
    napi_value timeout = NapiUtils::GetNamedProperty(env, jsConfig, "timeout");
    if (NapiUtils::GetValueType(env, timeout) != napi_undefined) {
        if (NapiUtils::GetValueType(env, NapiUtils::GetNamedProperty(env, timeout, "connectionTimeout"))
            != napi_undefined) {
            config.timeout.connectionTimeout =
                static_cast<uint64_t>(NapiUtils::Convert2Int64(env, timeout, "connectionTimeout"));
            if (config.timeout.connectionTimeout < MIN_TIMEOUT) {
                errInfo = "Parameter verification failed, the connectionTimeout is less than minimum";
                return false;
            }
        }
        if (NapiUtils::GetValueType(env, NapiUtils::GetNamedProperty(env, timeout, "totalTimeout"))
            != napi_undefined) {
            config.timeout.totalTimeout =
                static_cast<uint64_t>(NapiUtils::Convert2Int64(env, timeout, "totalTimeout"));
            if (config.timeout.totalTimeout < MIN_TIMEOUT || config.timeout.totalTimeout > MAX_TIMEOUT) {
                errInfo = "Parameter verification failed, the totalTimeout exceeds the limit";
                return false;
            }
        }
    }
    return true;
}

void JsInitialize::ParseNetwork(napi_env env, napi_value jsConfig, Network &network)
{
    network = static_cast<Network>(NapiUtils::Convert2Uint32(env, jsConfig, "network"));
    if (network != Network::ANY && network != Network::WIFI && network != Network::CELLULAR) {
        network = Network::ANY;
    }
}

bool JsInitialize::ParseToken(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo)
{
    char *token = nullptr;
    size_t len = 0;
    if (!NapiUtils::HasNamedProperty(env, jsConfig, "token")) {
        return true;
    }
    napi_value value = NapiUtils::GetNamedProperty(env, jsConfig, "token");
    if (NapiUtils::GetValueType(env, value) != napi_string) {
        return true;
    }
    uint32_t bufferLen = TOKEN_MAX_BYTES + 2;
    token = new (std::nothrow) char[bufferLen];
    if (token == nullptr) {
        return false;
    }
    napi_status status = napi_get_value_string_utf8(env, value, token, bufferLen, &len);
    if (status != napi_ok) {
        REQUEST_HILOGE("napi get value string utf8 failed");
        memset_s(token, bufferLen, 0, bufferLen);
        errInfo = "Parameter verification failed, get parameter config.token failed";
        delete[] token;
        return false;
    }
    if (len < TOKEN_MIN_BYTES || len > TOKEN_MAX_BYTES) {
        memset_s(token, bufferLen, 0, bufferLen);
        errInfo = "Parameter verification failed, the length of token should between 8 and 2048 bytes";
        delete[] token;
        return false;
    }
    config.token = std::string(token, len);
    memset_s(token, bufferLen, 0, bufferLen);
    delete[] token;
    return true;
}

bool JsInitialize::ParseIndex(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo)
{
    config.index = NapiUtils::Convert2Uint32(env, jsConfig, "index");
    if (config.action == Action::DOWNLOAD) {
        config.index = 0;
        return true;
    }
    if (config.files.size() <= config.index) {
        REQUEST_HILOGE("files.size is %{public}zu, index is %{public}d", config.files.size(), config.index);
        errInfo = "Parameter verification failed, config.index exceeds file list";
        return false;
    }
    return true;
}

bool JsInitialize::ParseAction(napi_env env, napi_value jsConfig, Action &action, std::string &errInfo)
{
    if (!NapiUtils::HasNamedProperty(env, jsConfig, "action")) {
        REQUEST_HILOGE("ParseAction err");
        errInfo = "Missing mandatory parameters, can not find property action";
        return false;
    }
    napi_value value = NapiUtils::GetNamedProperty(env, jsConfig, "action");
    if (NapiUtils::GetValueType(env, value) != napi_number) {
        REQUEST_HILOGE("GetNamedProperty err");
        errInfo = "Incorrect parameter type, action type is not of napi_number type";
        return false;
    }
    action = static_cast<Action>(NapiUtils::Convert2Uint32(env, value));
    if (action != Action::DOWNLOAD && action != Action::UPLOAD) {
        REQUEST_HILOGE("Must be UPLOAD or DOWNLOAD");
        errInfo = "Parameter verification failed, action must be UPLOAD or DOWNLOAD";
        return false;
    }
    return true;
}

// Only use for Action::DOWNLOAD.
bool JsInitialize::ParseSaveas(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo)
{
    if (config.action != Action::DOWNLOAD) {
        config.saveas = "";
        return true;
    }
    std::string temp = NapiUtils::Convert2String(env, jsConfig, "saveas");
    StringTrim(temp);
    if (temp.empty() || temp == "./") {
        bool result = InterceptData("/", config.url, config.saveas);
        if (!result) {
            errInfo = "Parameter verification failed, config.saveas parse error";
        }
        return result;
    }
    if (temp.size() == 0 || temp[temp.size() - 1] == '/') {
        errInfo = "Parameter verification failed, config.saveas parse error";
        return false;
    }
    config.saveas = temp;
    return true;
}

int64_t JsInitialize::ParseBegins(napi_env env, napi_value jsConfig)
{
    int64_t size = NapiUtils::Convert2Int64(env, jsConfig, "begins");
    return size >= 0 ? size : 0;
}

int64_t JsInitialize::ParseEnds(napi_env env, napi_value jsConfig)
{
    if (!NapiUtils::HasNamedProperty(env, jsConfig, "ends")) {
        return -1;
    }

    napi_value value = NapiUtils::GetNamedProperty(env, jsConfig, "ends");
    if (NapiUtils::GetValueType(env, value) != napi_number) {
        return -1;
    }
    return NapiUtils::Convert2Int64(env, value);
}

uint32_t JsInitialize::ParsePriority(napi_env env, napi_value jsConfig)
{
    if (!NapiUtils::HasNamedProperty(env, jsConfig, "priority")) {
        return 0;
    }
    return NapiUtils::Convert2Uint32(env, jsConfig, "priority");
}

bool JsInitialize::ParseDescription(napi_env env, napi_value jsConfig, std::string &description, std::string &errInfo)
{
    description = NapiUtils::Convert2String(env, jsConfig, "description");
    if (description.size() > DESCRIPTION_MAXIMUM) {
        errInfo = "Parameter verification failed, the length of config.description exceeds 1024";
        return false;
    }
    return true;
}

std::map<std::string, std::string> JsInitialize::ParseMap(
    napi_env env, napi_value jsConfig, const std::string &propertyName)
{
    std::map<std::string, std::string> result;
    napi_value jsValue = NapiUtils::GetNamedProperty(env, jsConfig, propertyName);
    napi_valuetype jsType = NapiUtils::GetValueType(env, jsValue);
    if (jsType == napi_undefined) {
        return result;
    }
    auto names = NapiUtils::GetPropertyNames(env, jsValue);
    for (auto iter = names.begin(); iter != names.end(); ++iter) {
        // The value of `Header` or `extra` can be empty.
        result[*iter] = NapiUtils::Convert2String(env, jsValue, *iter);
    }
    return result;
}

bool JsInitialize::ParseUrl(napi_env env, napi_value jsConfig, std::string &url, std::string &errInfo)
{
    url = NapiUtils::Convert2String(env, jsConfig, "url");
    if (url.size() > URL_MAXIMUM) {
        REQUEST_HILOGE("The URL exceeds the maximum length of 8192");
        errInfo = "Parameter verification failed, the length of url exceeds 8192";
        return false;
    }
    auto hostname = GetHostnameFromURL(url);
    bool cleartextPermitted = true;
    OHOS::NetManagerStandard::NetworkSecurityConfig::GetInstance().IsCleartextPermitted(hostname, cleartextPermitted);
    if (!cleartextPermitted) {
        if (!regex_match(url, std::regex("^https:\\/\\/.+"))) {
            REQUEST_HILOGE("ParseUrl error");
            errInfo = "Parameter verification failed, clear text transmission to this url is not permitted";
            return false;
        }
    } else {
        if (!regex_match(url, std::regex("^http(s)?:\\/\\/.+"))) {
            REQUEST_HILOGE("ParseUrl error");
            errInfo = "Parameter verification failed, the url should start with http(s)://";
            return false;
        }
    }

    return true;
}

bool JsInitialize::ParseCertsPath(
    napi_env env, napi_value jsConfig, std::vector<std::string> &certsPath, std::string &errInfo)
{
    std::string url = NapiUtils::Convert2String(env, jsConfig, "url");
    if (url.size() > URL_MAXIMUM) {
        REQUEST_HILOGE("The URL exceeds the maximum length of 8192");
        errInfo = "Parameter verification failed, the length of url exceeds 8192";
        return false;
    }
    if (!regex_match(url, std::regex("^http(s)?:\\/\\/.+"))) {
        REQUEST_HILOGE("ParseUrl error");
        errInfo = "Parameter verification failed, the url should start with http(s)://";
        return false;
    }

    typedef std::string::const_iterator iter_t;

    iter_t urlEnd = url.end();
    iter_t protocolStart = url.cbegin();
    iter_t protocolEnd = std::find(protocolStart, urlEnd, ':');
    std::string protocol = std::string(protocolStart, protocolEnd);
    if (protocol != "https") {
        REQUEST_HILOGD("Using Http");
        return true;
    }
    if (protocolEnd != urlEnd) {
        std::string afterProtocol = &*(protocolEnd);
        // 3 is the num of ://
        if ((afterProtocol.length() > 3) && (afterProtocol.substr(0, 3) == "://")) {
            // 3 means go beyound :// in protocolEnd
            protocolEnd += 3;
        } else {
            protocolEnd = url.cbegin();
        }
    } else {
        protocolEnd = url.cbegin();
    }
    iter_t hostStart = protocolEnd;
    iter_t pathStart = std::find(hostStart, urlEnd, '/');
    iter_t queryStart = std::find(url.cbegin(), urlEnd, '?');
    iter_t hostEnd = std::find(protocolEnd, (pathStart != urlEnd) ? pathStart : queryStart, ':');
    std::string hostname = std::string(hostStart, hostEnd);
    REQUEST_HILOGD("Hostname is %{public}s", hostname.c_str());
    NetManagerStandard::NetworkSecurityConfig::GetInstance().GetTrustAnchorsForHostName(hostname, certsPath);
    return true;
}

bool JsInitialize::ParseTitle(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo)
{
    config.title = NapiUtils::Convert2String(env, jsConfig, "title");
    if (config.version == Version::API10 && config.title.size() > TITLE_MAXIMUM) {
        errInfo = "Parameter verification failed, the length of config title exceeds 256";
        return false;
    }
    if (config.title.empty()) {
        config.title = config.action == Action::UPLOAD ? "upload" : "download";
    }
    return true;
}

bool JsInitialize::ParseProxy(napi_env env, napi_value jsConfig, std::string &proxy, std::string &errInfo)
{
    proxy = NapiUtils::Convert2String(env, jsConfig, "proxy");
    if (proxy.empty()) {
        return true;
    }

    if (proxy.size() > PROXY_MAXIMUM) {
        REQUEST_HILOGE("The proxy exceeds the maximum length of 512");
        errInfo = "Parameter verification failed, the length of config.proxy exceeds 512";
        return false;
    }

    if (!regex_match(proxy, std::regex("^http:\\/\\/.+:\\d{1,5}$"))) {
        REQUEST_HILOGE("ParseProxy error");
        errInfo = "Parameter verification failed, the format of proxy is http(s)://<address or domain>:port";
        return false;
    }
    return true;
}

std::string GetHostnameFromURL(const std::string &url)
{
    if (url.empty()) {
        return "";
    }
    std::string delimiter = "://";
    std::string tempUrl = url;
    std::replace(tempUrl.begin(), tempUrl.end(), '\\', '/');
    size_t posStart = tempUrl.find(delimiter);
    if (posStart != std::string::npos) {
        posStart += delimiter.length();
    } else {
        posStart = 0;
    }
    size_t notSlash = tempUrl.find_first_not_of('/', posStart);
    if (notSlash != std::string::npos) {
        posStart = notSlash;
    }
    size_t posEnd =
        std::min({ tempUrl.find(':', posStart), tempUrl.find('/', posStart), tempUrl.find('?', posStart) });
    if (posEnd != std::string::npos) {
        return tempUrl.substr(posStart, posEnd - posStart);
    }
    return tempUrl.substr(posStart);
}

void JsInitialize::ParseCertificatePins(napi_env env, std::string &url, std::string &certificatePins)
{
    auto hostname = GetHostnameFromURL(url);
    if (OHOS::NetManagerStandard::NetworkSecurityConfig::GetInstance().IsPinOpenMode(hostname)) {
        REQUEST_HILOGI("Pins is openMode");
        return;
    }
    auto ret =
        OHOS::NetManagerStandard::NetworkSecurityConfig::GetInstance().GetPinSetForHostName(hostname, certificatePins);
    if (ret != 0 || certificatePins.empty()) {
        REQUEST_HILOGD("Get No pin set by hostname");
    }
}

void JsInitialize::ParseMethod(napi_env env, napi_value jsConfig, Config &config)
{
    if (config.version == Version::API10) {
        config.method = config.action == Action::UPLOAD ? "PUT" : "GET";
    } else {
        config.method = "POST";
    }
    std::string method = NapiUtils::Convert2String(env, jsConfig, "method");
    if (!method.empty()) {
        transform(method.begin(), method.end(), method.begin(), ::toupper);
        if (config.action == Action::UPLOAD && (method == "POST" || method == "PUT")) {
            config.method = method;
        }
        if (config.action == Action::DOWNLOAD && (method == "POST" || method == "GET")) {
            config.method = method;
        }
    }
}

bool JsInitialize::ParseData(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo)
{
    napi_value value = NapiUtils::GetNamedProperty(env, jsConfig, "data");
    if (value == nullptr) {
        return true;
    }

    napi_valuetype valueType = NapiUtils::GetValueType(env, value);
    if (config.action == Action::UPLOAD && valueType == napi_object) {
        return Convert2FormItems(env, value, config.forms, config.files, errInfo);
    } else if (config.action == Action::DOWNLOAD && valueType == napi_string) {
        config.data = NapiUtils::Convert2String(env, value);
    } else {
        REQUEST_HILOGE("data type is error");
        errInfo = "Incorrect parameter type, the config.data parameter type is incorrect";
        return false;
    }
    return true;
}

bool JsInitialize::ParseName(napi_env env, napi_value jsVal, std::string &name)
{
    napi_value value = NapiUtils::GetNamedProperty(env, jsVal, "name");
    if (NapiUtils::GetValueType(env, value) != napi_string) {
        return false;
    }
    name = NapiUtils::Convert2String(env, value);
    return true;
}

bool JsInitialize::GetFormItems(
    napi_env env, napi_value jsVal, std::vector<FormItem> &forms, std::vector<FileSpec> &files)
{
    if (!NapiUtils::HasNamedProperty(env, jsVal, "name") || !NapiUtils::HasNamedProperty(env, jsVal, "value")) {
        return false;
    }

    std::string name;
    if (!ParseName(env, jsVal, name)) {
        return false;
    }
    napi_value value = NapiUtils::GetNamedProperty(env, jsVal, "value");
    if (value == nullptr) {
        REQUEST_HILOGE("Get upload value failed");
        return false;
    }
    bool isArray = false;
    napi_is_array(env, value, &isArray);
    napi_valuetype valueType = NapiUtils::GetValueType(env, value);
    if (valueType == napi_string) {
        FormItem form;
        form.name = name;
        form.value = NapiUtils::Convert2String(env, value);
        forms.push_back(form);
    } else if (valueType == napi_object && !isArray) {
        FileSpec file;
        if (!Convert2FileSpec(env, value, name, file)) {
            REQUEST_HILOGE("Convert2FileSpec failed");
            return false;
        }
        files.push_back(file);
    } else if (isArray) {
        if (!Convert2FileSpecs(env, value, name, files)) {
            return false;
        }
    } else {
        REQUEST_HILOGE("value type is error");
        return false;
    }
    return true;
}

bool JsInitialize::Convert2FormItems(
    napi_env env, napi_value jsValue, std::vector<FormItem> &forms, std::vector<FileSpec> &files, std::string &errInfo)
{
    bool isArray = false;
    napi_is_array(env, jsValue, &isArray);
    NAPI_ASSERT_BASE(env, isArray, "not array", false);
    uint32_t length = 0;
    napi_get_array_length(env, jsValue, &length);
    for (uint32_t i = 0; i < length; ++i) {
        napi_value jsVal = nullptr;
        napi_handle_scope scope = nullptr;
        napi_open_handle_scope(env, &scope);
        if (scope == nullptr) {
            return false;
        }
        napi_get_element(env, jsValue, i, &jsVal);
        if (jsVal == nullptr) {
            REQUEST_HILOGE("Get element jsVal failed");
            errInfo = "Missing mandatory parameters, Get element jsVal failed";
            napi_close_handle_scope(env, scope);
            return false;
        }
        if (!GetFormItems(env, jsVal, forms, files)) {
            REQUEST_HILOGE("Get formItems failed");
            errInfo = "Missing mandatory parameters, Get formItems failed";
            napi_close_handle_scope(env, scope);
            return false;
        }
        napi_close_handle_scope(env, scope);
    }
    if (files.empty()) {
        errInfo = "Missing mandatory parameters, files is empty";
        return false;
    }
    return true;
}

bool JsInitialize::Convert2FileSpecs(
    napi_env env, napi_value jsValue, const std::string &name, std::vector<FileSpec> &files)
{
    REQUEST_HILOGD("Convert2FileSpecs in");
    uint32_t length = 0;
    napi_get_array_length(env, jsValue, &length);
    for (uint32_t i = 0; i < length; ++i) {
        napi_value jsVal = nullptr;
        napi_handle_scope scope = nullptr;
        napi_open_handle_scope(env, &scope);
        if (scope == nullptr) {
            return false;
        }
        napi_get_element(env, jsValue, i, &jsVal);
        if (jsVal == nullptr) {
            napi_close_handle_scope(env, scope);
            return false;
        }
        FileSpec file;
        bool ret = Convert2FileSpec(env, jsVal, name, file);
        if (!ret) {
            napi_close_handle_scope(env, scope);
            return false;
        }
        files.push_back(file);
        napi_close_handle_scope(env, scope);
    }
    return true;
}

// Assert `in` is trimmed.
bool JsInitialize::InterceptData(const std::string &str, const std::string &in, std::string &out)
{
    std::size_t position = in.find_last_of(str);
    // when the str at last index, will error.
    if (position == std::string::npos || position + 1 >= in.size()) {
        return false;
    }
    out = std::string(in, position + 1);
    return true;
}

bool JsInitialize::Convert2FileSpec(napi_env env, napi_value jsValue, const std::string &name, FileSpec &file)
{
    REQUEST_HILOGD("Convert2FileSpec in");
    file.name = name;
    file.uri = NapiUtils::Convert2String(env, jsValue, "path");
    StringTrim(file.uri);
    if (file.uri.empty()) {
        return false;
    }
    file.filename = NapiUtils::Convert2String(env, jsValue, "filename");
    file.hasContentType = NapiUtils::HasNamedProperty(env, jsValue, "contentType");
    if (file.hasContentType) {
        file.type = NapiUtils::Convert2String(env, jsValue, "contentType");
    }
    return true;
}

void JsInitialize::ParseRedirect(napi_env env, napi_value jsConfig, bool &redirect)
{
    if (!NapiUtils::HasNamedProperty(env, jsConfig, "redirect")) {
        redirect = true;
    } else {
        redirect = NapiUtils::Convert2Boolean(env, jsConfig, "redirect");
    }
}

void JsInitialize::ParseRetry(napi_env env, napi_value jsConfig, bool &retry)
{
    if (!NapiUtils::HasNamedProperty(env, jsConfig, "retry")) {
        retry = true;
    } else {
        retry = NapiUtils::Convert2Boolean(env, jsConfig, "retry");
    }
}

bool JsInitialize::IsStageMode(napi_env env, napi_value value)
{
    bool stageMode = true;
    napi_status status = OHOS::AbilityRuntime::IsStageContext(env, value, stageMode);
    if (status != napi_ok || !stageMode) {
        return false;
    }
    return true;
}

bool JsInitialize::ParseConfigV9(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo)
{
    REQUEST_HILOGD("ParseConfigV9 in");
    config.action = NapiUtils::GetRequestAction(env, jsConfig);
    config.headers = ParseMap(env, jsConfig, "header");
    if (!ParseUrl(env, jsConfig, config.url, errInfo)) {
        errInfo = "Parse url error";
        return false;
    }
    auto func = config.action == Action::UPLOAD ? ParseUploadConfig : ParseDownloadConfig;
    if (!func(env, jsConfig, config, errInfo)) {
        return false;
    }
    ParseTitle(env, jsConfig, config, errInfo);
    return true;
}

bool JsInitialize::ParseUploadConfig(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo)
{
    REQUEST_HILOGD("ParseUploadConfig in");
    ParseMethod(env, jsConfig, config);
    napi_value jsFiles = NapiUtils::GetNamedProperty(env, jsConfig, PARAM_KEY_FILES);
    if (jsFiles == nullptr) {
        errInfo = "Parse config files error";
        return false;
    }

    config.files = NapiUtils::Convert2FileVector(env, jsFiles, "API8");
    if (config.files.empty()) {
        errInfo = "Parameter verification failed, Parse config files error";
        return false;
    }

    napi_value jsData = NapiUtils::GetNamedProperty(env, jsConfig, PARAM_KEY_DATA);
    if (jsData == nullptr) {
        errInfo = "Parameter verification failed, Parse config data error";
        return false;
    }
    config.forms = NapiUtils::Convert2RequestDataVector(env, jsData);

    if (!ParseIndex(env, jsConfig, config, errInfo)) {
        return false;
    }

    config.begins = ParseBegins(env, jsConfig);
    config.ends = ParseEnds(env, jsConfig);
    return true;
}

bool JsInitialize::ParseDownloadConfig(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo)
{
    REQUEST_HILOGD("ParseDownloadConfig in");
    config.metered = NapiUtils::Convert2Boolean(env, jsConfig, "enableMetered");
    config.roaming = NapiUtils::Convert2Boolean(env, jsConfig, "enableRoaming");
    config.description = NapiUtils::Convert2String(env, jsConfig, PARAM_KEY_DESCRIPTION);
    uint32_t type = NapiUtils::Convert2Uint32(env, jsConfig, PARAM_KEY_NETWORKTYPE);
    if (type == NETWORK_MOBILE) {
        config.network = Network::CELLULAR;
    } else if (type == NETWORK_WIFI) {
        config.network = Network::WIFI;
    } else {
        config.network = Network::ANY;
    }
    config.saveas = NapiUtils::Convert2String(env, jsConfig, PARAM_KEY_FILE_PATH);
    if (config.saveas.empty()) {
        InterceptData("/", config.url, config.saveas);
    }
    config.background = NapiUtils::Convert2Boolean(env, jsConfig, PARAM_KEY_BACKGROUND);
    config.method = "GET";
    return true;
}

void JsInitialize::CreatProperties(napi_env env, napi_value &self, napi_value config, JsTask *task)
{
    if (task->config_.version == Version::API10) {
        NapiUtils::SetStringPropertyUtf8(env, self, "tid", task->GetTid());
        napi_set_named_property(env, self, "config", config);
    }
}

void JsInitialize::StandardizeFileSpec(FileSpec &file)
{
    if (file.filename.empty()) {
        InterceptData("/", file.uri, file.filename);
    }
    // Does not have "contentType" field or API9 "type" empty.
    if (!file.hasContentType) {
        InterceptData(".", file.filename, file.type);
    }
    if (file.name.empty()) {
        file.name = "file";
    }
    return;
}

bool JsInitialize::CheckUserFileSpec(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context,
    const Config &config, FileSpec &file, ExceptionError &error, bool isUpload)
{
    if (config.mode != Mode::FOREGROUND) {
        error.code = E_PARAMETER_CHECK;
        error.errInfo = "Parameter verification failed, user file can only for Mode::FOREGROUND";
        return false;
    }
    if (isUpload) {
        std::shared_ptr<Uri> uri = std::make_shared<Uri>(file.uri);
        std::shared_ptr<AppExecFwk::DataAbilityHelper> dataAbilityHelper =
            AppExecFwk::DataAbilityHelper::Creator(context, uri);
        if (dataAbilityHelper == nullptr) {
            REQUEST_HILOGE("dataAbilityHelper null");
            error.code = E_PARAMETER_CHECK;
            error.errInfo = "Parameter verification failed, dataAbilityHelper null";
            SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_07, config.bundleName, "", error.errInfo);
            return false;
        }
        file.fd = dataAbilityHelper->OpenFile(*uri, "r");
    } else {
        std::shared_ptr<AppFileService::ModuleFileUri::FileUri> fileUri =
            std::make_shared<AppFileService::ModuleFileUri::FileUri>(file.uri);
        std::string realPath = fileUri->GetRealPath();
        if (config.firstInit) {
            file.fd = open(realPath.c_str(), O_RDWR | O_TRUNC);
        } else {
            file.fd = open(realPath.c_str(), O_RDWR | O_APPEND);
        }
    }
    if (file.fd < 0) {
        REQUEST_HILOGE("Failed to open user file, fd: %{public}d", file.fd);
        error.code = E_FILE_IO;
        error.errInfo = "Failed to open user file";
        SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_09, config.bundleName, "", error.errInfo);
        return false;
    }
    fdsan_exchange_owner_tag(file.fd, 0, REQUEST_FDSAN_TAG);
    StandardizeFileSpec(file);
    return true;
}

bool JsInitialize::CheckUploadFiles(
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config, ExceptionError &error)
{
    int32_t sdkVersion = GetSdkApiVersion();
    constexpr const int32_t uploadVersion = 15;
    if (config.version == Version::API10 && sdkVersion >= uploadVersion
        && config.files.size() > MAX_UPLOAD_ON15_FILES) {
        error.code = E_PARAMETER_CHECK;
        error.errInfo = "Parameter verification failed, upload by multipart file so many";
        return false;
    }
    // need reconstruction.
    for (auto &file : config.files) {
        if (IsUserFile(file.uri)) {
            file.isUserFile = true;
            if (config.version == Version::API9) {
                error.code = E_PARAMETER_CHECK;
                error.errInfo = "Parameter verification failed, user file can only for request.agent.";
                return false;
            }
            if (!CheckUserFileSpec(context, config, file, error, true)) {
                return false;
            }
            StandardizeFileSpec(file);
            continue;
        }

        if (!CheckUploadFileSpec(context, config, file, error)) {
            return false;
        }
    }
    return true;
}

bool JsInitialize::CheckUploadFileSpec(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config,
    FileSpec &file, ExceptionError &error)
{
    file.isUserFile = false;
    std::string path = file.uri;
    if (config.version == Version::API9) {
        if (!GetInternalPath(context, config, path, error.errInfo)) {
            error.code = E_PARAMETER_CHECK;
            return false;
        }
        StandardizePathApi9(path);
    } else {
        std::vector<std::string> pathVec;
        if (!GetSandboxPath(context, config, path, pathVec, error.errInfo)) {
            error.code = E_PARAMETER_CHECK;
            return false;
        }
    }
    REQUEST_HILOGD("CheckUploadFileSpec path");
    file.uri = path;
    if (!GetFdUpload(path, config, error)) {
        return false;
    }
    if (!JsTask::SetPathPermission(file.uri)) {
        error.code = E_FILE_IO;
        error.errInfo = "set path permission fail";
        return false;
    }
    StandardizeFileSpec(file);
    return true;
}

void JsInitialize::StandardizePathApi9(std::string &path)
{
    std::vector<std::string> pathVec;
    if (!JsInitialize::WholeToNormal(path, pathVec) || pathVec.empty()) {
        REQUEST_HILOGE("WholeToNormal Err api9");
    };
}

bool JsInitialize::CheckDownloadFile(
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config, ExceptionError &error)
{
    if (IsUserFile(config.saveas)) {
        if (config.version == Version::API9) {
            error.code = E_PARAMETER_CHECK;
            error.errInfo = "Parameter verification failed, user file can only for request.agent.";
            return false;
        }
        if (!config.overwrite) {
            error.code = E_PARAMETER_CHECK;
            error.errInfo = "Parameter verification failed, download to user file must support overrite.";
            return false;
        }
        FileSpec file = { .uri = config.saveas, .isUserFile = true };
        if (!CheckUserFileSpec(context, config, file, error, false)) {
            return false;
        }
        config.files.push_back(file);
        return true;
    }
    if (config.version == Version::API9) {
        std::string path = config.saveas;
        if (config.saveas.find('/') == 0) {
        } else if (!GetInternalPath(context, config, path, error.errInfo)) {
            error.code = E_PARAMETER_CHECK;
            return false;
        }
        StandardizePathApi9(path);
        config.saveas = path;
    } else {
        if (!CheckDownloadFilePath(context, config, error.errInfo)) {
            error.code = E_PARAMETER_CHECK;
            return false;
        }
    }
    FileSpec file = { .uri = config.saveas, .isUserFile = false };
    StandardizeFileSpec(file);
    config.files.push_back(file);
    if (!GetFdDownload(file.uri, config, error)) {
        return false;
    }
    if (!JsTask::SetPathPermission(config.saveas)) {
        error.code = E_FILE_IO;
        error.errInfo = "set path permission fail, download";
        return false;
    }
    return true;
}

bool JsInitialize::CheckDownloadFilePath(
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config, std::string &errInfo)
{
    std::string path = config.saveas;
    std::vector<std::string> pathVec;
    if (!GetSandboxPath(context, config, path, pathVec, errInfo)) {
        return false;
    }
    // pop filename.
    pathVec.pop_back();
    if (!JsInitialize::CreateDirs(pathVec)) {
        REQUEST_HILOGE("CreateDirs Err");
        errInfo = "Parameter verification failed, this is fail saveas path";
        return false;
    }
    config.saveas = path;
    return true;
}

bool JsInitialize::CreateDirs(const std::vector<std::string> &pathDirs)
{
    std::string path;
    std::error_code err;
    for (auto elem : pathDirs) {
        path += "/" + elem;
        if (std::filesystem::exists(path, err)) {
            continue;
        }
        err.clear();
        // create_directory noexcept.
        if (!std::filesystem::create_directory(path, err)) {
            REQUEST_HILOGE("Create Dir Err: %{public}d, %{public}s", err.value(), err.message().c_str());
            SysEventLog::SendSysEventLog(FAULT_EVENT, STANDARD_FAULT_05, err.message());
            return false;
        }
    }
    return true;
}

bool JsInitialize::FindDir(const std::string &pathDir)
{
    std::error_code err;
    return std::filesystem::exists(pathDir, err);
}

bool JsInitialize::IsUserFile(const std::string &path)
{
    return path.find("file://docs/") == 0 || path.find("file://media/") == 0;
}

bool JsInitialize::GetSandboxPath(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config,
    std::string &path, std::vector<std::string> &pathVec, std::string &errInfo)
{
    if (!StandardizePath(context, config, path)) {
        REQUEST_HILOGE("StandardizePath Err");
        errInfo = "Parameter verification failed, GetSandboxPath failed, StandardizePath fail";
        return false;
    };
    if (!WholeToNormal(path, pathVec) || pathVec.empty()) {
        REQUEST_HILOGE("WholeToNormal Err");
        errInfo = "Parameter verification failed, GetSandboxPath failed, WholeToNormal path fail";
        return false;
    };
    std::string baseDir;
    if (!CheckBelongAppBaseDir(path, baseDir)) {
        REQUEST_HILOGE("CheckBelongAppBaseDir Err");
        errInfo = "Parameter verification failed, GetSandboxPath failed, path not belong app base dir";
        return false;
    };
    return true;
}

// Must not user file.
bool JsInitialize::StandardizePath(
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config, std::string &path)
{
    std::string WHOLE_PREFIX = "/";
    std::string FILE_PREFIX = "file://";
    std::string INTERNAL_PREFIX = "internal://";
    std::string CURRENT_PREFIX = "./";

    if (path.find(WHOLE_PREFIX) == 0) {
        return true;
    }
    if (path.find(FILE_PREFIX) == 0) {
        path.erase(0, FILE_PREFIX.size());
        return FileToWhole(context, config, path);
    }
    if (path.find(INTERNAL_PREFIX) == 0) {
        path.erase(0, INTERNAL_PREFIX.size());
        return BaseToWhole(context, path);
    }
    if (path.find(CURRENT_PREFIX) == 0) {
        path.erase(0, CURRENT_PREFIX.size());
        return CacheToWhole(context, path);
    }
    return CacheToWhole(context, path);
}

// BaseDir is following context.
bool JsInitialize::BaseToWhole(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, std::string &path)
{
    std::string base = context->GetBaseDir();
    if (base.empty()) {
        REQUEST_HILOGE("GetBaseDir error.");
        SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_06, "GetCacheDir error");
        return false;
    }
    path = base + "/" + path;
    return true;
}

bool JsInitialize::CacheToWhole(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, std::string &path)
{
    std::string cache = context->GetCacheDir();
    if (cache.empty()) {
        REQUEST_HILOGE("GetCacheDir error.");
        SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_05, "GetCacheDir error");
        return false;
    }
    path = cache + "/" + path;
    return true;
}

bool JsInitialize::FileToWhole(
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config, std::string &path)
{
    std::string bundleName = path.substr(0, path.find("/"));
    if (bundleName != config.bundleName) {
        REQUEST_HILOGE("path bundleName error.");
        return false;
    }
    path.erase(0, bundleName.size());
    return true;
}

bool JsInitialize::WholeToNormal(std::string &path, std::vector<std::string> &out)
{
    std::string normalPath;
    std::vector<std::string> elems;
    StringSplit(path, '/', elems);
    if (!PathVecToNormal(elems, out)) {
        return false;
    }
    for (auto elem : out) {
        normalPath += "/" + elem;
    }
    path = normalPath;
    return true;
}

// "/A/B/../C" -> "/A/C"
// ["A", "B", "..", "C"] -> ["A", "C"]
bool JsInitialize::PathVecToNormal(const std::vector<std::string> &in, std::vector<std::string> &out)
{
    for (auto elem : in) {
        if (elem == "..") {
            if (out.size() > 0) {
                out.pop_back();
            } else {
                return false;
            }
        } else if (elem != ".") {
            out.push_back(elem);
        }
    }
    return true;
}

// "/A/B//C" -> ["A", "B", "C"]
void JsInitialize::StringSplit(const std::string &str, const char delim, std::vector<std::string> &elems)
{
    std::stringstream stream(str);
    std::string item;
    while (std::getline(stream, item, delim)) {
        if (!item.empty()) {
            elems.push_back(item);
        }
    }
    return;
}

void JsInitialize::StringTrim(std::string &str)
{
    if (str.empty()) {
        return;
    }
    str.erase(0, str.find_first_not_of(" "));
    str.erase(str.find_last_not_of(" ") + 1);
    return;
}

bool JsInitialize::CheckBelongAppBaseDir(const std::string &filepath, std::string &baseDir)
{
    if (!JsInitialize::GetAppBaseDir(baseDir)) {
        return false;
    }
    if ((filepath.find(AREA1) == 0) || filepath.find(AREA2) == 0 || filepath.find(AREA5) == 0) {
        return true;
    } else {
        REQUEST_HILOGE("File dir not include base dir");
        return false;
    }
}
} // namespace OHOS::Request
