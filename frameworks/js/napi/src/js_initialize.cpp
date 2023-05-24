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


#include <regex>
#include "ability.h"
#include "request_manager.h"
#include "log.h"
#include "napi_base_context.h"
#include "js_common.h"
#include "napi_utils.h"
#include "js_initialize.h"

static constexpr const char *PARAM_KEY_DESCRIPTION = "description";
static constexpr const char *PARAM_KEY_NETWORKTYPE = "networkType";
static constexpr const char *PARAM_KEY_FILE_PATH = "filePath";
static constexpr const char *PARAM_KEY_BACKGROUND = "background";
static constexpr uint32_t FILE_PERMISSION = 0644;
static constexpr uint32_t TOKEN_MAX_BYTES = 2048;
static constexpr uint32_t TOKEN_MIN_BYTES = 8;
namespace OHOS::Request {
napi_value JsInitialize::Initialize(napi_env env, napi_callback_info info, Version version)
{
    REQUEST_HILOGD("constructor request task!");
    bool withErrCode = version != Version::API8;
    napi_value self = nullptr;
    size_t argc = NapiUtils::MAX_ARGC;
    napi_value argv[NapiUtils::MAX_ARGC] = { nullptr };
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, argv, &self, nullptr));
    int32_t number = version == Version::API8 ? NapiUtils::ONE_ARG : NapiUtils::TWO_ARG;
    if (argc < number) {
        NapiUtils::ThrowError(env, E_PARAMETER_CHECK, "invalid parameter count", withErrCode);
        return nullptr;
    }

    Config config;
    config.version = version;
    config.withErrCode = withErrCode;
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
    auto finalize = [](napi_env env, void *data, void *hint) {
        REQUEST_HILOGD("destructed task");
        JsTask *task = reinterpret_cast<JsTask *>(data);
        task->ClearListener();
        JsTask::ClearTaskMap(task->GetTid());
        delete task;
    };
    if (napi_wrap(env, self, task, finalize, nullptr, nullptr) != napi_ok) {
        finalize(env, task, nullptr);
        return nullptr;
    }
    return self;
}

ExceptionError JsInitialize::InitParam(napi_env env, napi_value* argv,
    std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config)
{
    REQUEST_HILOGD("InitParam in");
    ExceptionError err = {.code = E_OK};
    int parametersPosition = config.version == Version::API8 ? CONFIG_PARAM_AT_FIRST : CONFIG_PARAM_AT_SECOND;

    napi_status getStatus = GetContext(env, argv[0], context);
    if (getStatus != napi_ok) {
        REQUEST_HILOGE("Get context fail");
        return {.code = E_OTHER, .errInfo = "Get context fail"};
    }

    if (context->GetApplicationInfo() == nullptr) {
        return {.code = E_OTHER, .errInfo = "ApplicationInfo is null"};
    }
    if (!ParseConfig(env, argv[parametersPosition], config, err.errInfo)) {
        err.code = E_PARAMETER_CHECK;
        return err;
    }
    config.bundleName = context->GetBundleName();
    REQUEST_HILOGD("config.bundleName is %{public}s", config.bundleName.c_str());
    return CheckFilePath(context, config);
}

napi_status JsInitialize::GetContext(napi_env env, napi_value value,
    std::shared_ptr<OHOS::AbilityRuntime::Context> &context)
{
    if (!IsStageMode(env, value)) {
        auto ability = OHOS::AbilityRuntime::GetCurrentAbility(env);
        if (ability == nullptr) {
            REQUEST_HILOGE("Get current ability fail");
            return napi_generic_failure;
        }
        context = ability->GetAbilityContext();
    } else {
        context = OHOS::AbilityRuntime::GetStageModeContext(env, value);
    }
    if (context == nullptr) {
        REQUEST_HILOGE("Get Context failed, context is nullptr.");
        return napi_generic_failure;
    }
    return napi_ok;
}

ExceptionError JsInitialize::CheckFilePath(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context,
    Config &config)
{
    ExceptionError err = {.code = E_OK};
    if (config.action == Action::DOWNLOAD) {
        FileSpec file = {.uri = config.saveas};
        config.files.push_back(file);
    }

    for (auto &file : config.files) {
        std::string path;
        if (!GetInternalPath(file.uri, context, config, path)) {
            return {.code = E_FILE_PATH, .errInfo = "this is fail path"};
        }
        file.uri = path;
        if (file.filename.empty()) {
            InterceptData("/", file.uri, file.filename);
        }
        if (file.type.empty()) {
            InterceptData(".", file.filename, file.type);
        }
        if (file.name.empty()) {
            file.name = "file";
        }
        err = GetFD(path, config, file.fd);
        if (err.code != E_OK) {
            return err;
        }
    }
    return err;
}

ExceptionError JsInitialize::GetFD(const std::string &path, const Config &config, int32_t &fd)
{
    ExceptionError error = {.code = E_OK};
    fd = config.action == Action::UPLOAD ? open(path.c_str(), O_RDONLY) : open(path.c_str(), O_RDWR);
    if (fd >= 0) {
        REQUEST_HILOGD("File already exists");
        if (config.action == Action::UPLOAD) {
            return error;
        }
        if (config.version == Version::API10 && config.overwrite == true) {
            return error;
        }
        return {.code = E_FILE_PATH, .errInfo = "Download File already exists"};
    } else {
        if (config.action == Action::UPLOAD) {
            return {.code = E_FILE_IO, .errInfo = "Failed to open file errno " + std::to_string(errno)};
        }
        fd = open(path.c_str(), O_CREAT | O_RDWR, FILE_PERMISSION);
        if (fd < 0) {
            return {.code = E_FILE_IO, .errInfo = "Failed to open file errno " + std::to_string(errno)};
        }
    }
    return error;
}

bool JsInitialize::GetInternalPath(const std::string &fileUri,
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config, std::string &filePath)
{
    if (config.action == Action::DOWNLOAD &&config.version != Version::API10) {
        filePath = fileUri;
        return true;
    }
    std::string fileName;
    std::string pattern = config.version == Version::API10 ? "./" : "internal://cache/";
    size_t pos = fileUri.find(pattern);
    if (pos != 0) {
        if (config.version == Version::API9) {
            return false;
        }
        fileName = fileUri;
    } else {
        fileName = fileUri.substr(pattern.size(), fileUri.size());
    }
    if (fileName.empty()) {
        return false;
    }
    filePath = context->GetCacheDir();
    if (filePath.empty()) {
        REQUEST_HILOGE("internal to cache error");
        return false;
    }
    filePath += "/" + fileName;
    if (!NapiUtils::IsPathValid(filePath)) {
        REQUEST_HILOGE("IsPathValid error %{public}s", filePath.c_str());
        return false;
    }
    return true;
}

bool JsInitialize::ParseConfig(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo)
{
    if (config.version != Version::API10) {
        return ParseConfigV9(env, jsConfig, config, errInfo);
    }

    if (!ParseAction(env, jsConfig, config.action)) {
        errInfo = "parse action error";
        return false;
    }
    if (!ParseUrl(env, jsConfig, config.url)) {
        errInfo = "parse url error";
        return false;
    }
    if (!ParseData(env, jsConfig, config)) {
        errInfo = "parse data error";
        return false;
    }
    if (!ParseIndex(env, jsConfig, config)) {
        errInfo = "Index exceeds file list";
        return false;
    }
    ParseTitle(env, jsConfig, config);
    ParseMethod(env, jsConfig, config);
    ParseSaveas(env, jsConfig, config);
    ParseRedirect(env, jsConfig, config.redirect);
    ParseToken(env, jsConfig, config.token);
    ParseNetwork(env, jsConfig, config.network);
    ParseRetry(env, jsConfig, config.retry);

    config.overwrite = NapiUtils::Convert2Boolean(env, jsConfig, "overwrite");
    config.metered = NapiUtils::Convert2Boolean(env, jsConfig, "metered");
    config.roaming = NapiUtils::Convert2Boolean(env, jsConfig, "roaming");
    config.gauge = NapiUtils::Convert2Boolean(env, jsConfig, "gauge");
    config.precise = NapiUtils::Convert2Boolean(env, jsConfig, "precise");
    config.begins = ParseBegins(env, jsConfig);
    config.ends = ParseEnds(env, jsConfig);
    config.mode = static_cast<Mode>(NapiUtils::Convert2Uint32(env, jsConfig, "mode"));
    config.description = NapiUtils::Convert2String(env, jsConfig, "description");
    config.headers = ParseMap(env, jsConfig, "headers");
    config.extras = ParseMap(env, jsConfig, "extras");
    return true;
}

void JsInitialize::ParseNetwork(napi_env env, napi_value jsConfig, Network &network)
{
    network = static_cast<Network>(NapiUtils::Convert2Uint32(env, jsConfig, "network"));
    if (network != Network::ANY && network != Network::WIFI && network != Network::CELLULAR) {
        network = Network::ANY;
    }
}

void JsInitialize::ParseToken(napi_env env, napi_value jsConfig, std::string &token)
{
    token = NapiUtils::Convert2String(env, jsConfig, "token");
    if (token.size() < TOKEN_MIN_BYTES || token.size() > TOKEN_MAX_BYTES) {
        token = "";
    }
}

bool JsInitialize::ParseIndex(napi_env env, napi_value jsConfig, Config &config)
{
    config.index = NapiUtils::Convert2Uint32(env, jsConfig, "index");
    if (config.action == Action::DOWNLOAD) {
        config.index = 0;
        return true;
    }
    if (config.files.size() <= config.index) {
        REQUEST_HILOGE("Index exceeds file list");
        return false;
    }
}

bool JsInitialize::ParseAction(napi_env env, napi_value jsConfig, Action &action)
{
    if (!NapiUtils::HasNamedProperty(env, jsConfig, "action")) {
        REQUEST_HILOGE("ParseAction err");
        return false;
    }
    napi_value value = NapiUtils::GetNamedProperty(env, jsConfig, "action");
    if (NapiUtils::GetValueType(env, value) != napi_number) {
        REQUEST_HILOGE("GetNamedProperty err");
        return false;
    }
    action = static_cast<Action>(NapiUtils::Convert2Uint32(env, value));
    if (action != Action::DOWNLOAD && action != Action::UPLOAD) {
        REQUEST_HILOGE("Must be UPLOAD or DOWNLOAD");
        return false;
    }
    return true;
}

void JsInitialize::ParseSaveas(napi_env env, napi_value jsConfig, Config &config)
{
    config.saveas = NapiUtils::Convert2String(env, jsConfig, "saveas");
    if (config.saveas.empty() || config.saveas == "./") {
        InterceptData("/", config.url, config.saveas);
    }
}

int64_t JsInitialize::ParseBegins(napi_env env, napi_value jsConfig)
{
    if (!NapiUtils::HasNamedProperty(env, jsConfig, "begins")) {
        return 0;
    }
    napi_value value = NapiUtils::GetNamedProperty(env, jsConfig, "begins");
    int64_t size = NapiUtils::Convert2Int64(env, value);
    return size >= 0 ? size : 0;
}

int64_t JsInitialize::ParseEnds(napi_env env, napi_value jsConfig)
{
    if (!NapiUtils::HasNamedProperty(env, jsConfig, "ends")) {
        return -1;
    }
    napi_value value = NapiUtils::GetNamedProperty(env, jsConfig, "ends");
    return NapiUtils::Convert2Int64(env, value);
}

std::map<std::string, std::string> JsInitialize::ParseMap(napi_env env, napi_value jsConfig,
    const std::string &propertyName)
{
    std::map<std::string, std::string> result;
    napi_value jsValue = NapiUtils::GetNamedProperty(env, jsConfig, propertyName);
    if (jsValue == nullptr) {
        return result;
    }
    auto names = NapiUtils::GetPropertyNames(env, jsValue);
    for (auto iter = names.begin(); iter != names.end(); ++iter) {
        auto value = NapiUtils::Convert2String(env, jsValue, *iter);
        if (!value.empty()) {
            result[*iter] = value;
        }
    }
    return result;
}

bool JsInitialize::ParseUrl(napi_env env, napi_value jsConfig, std::string &url)
{
    url = NapiUtils::Convert2String(env, jsConfig, "url");
    if (!regex_match(url, std::regex("^http(s)?:\\/\\/.+"))) {
        REQUEST_HILOGE("ParseUrl error");
        return false;
    }
    return true;
}

void JsInitialize::ParseTitle(napi_env env, napi_value jsConfig, Config &config)
{
    config.title = NapiUtils::Convert2String(env, jsConfig, "title");
    if (config.title.empty()) {
        config.title = config.action == Action::UPLOAD ? "upload" : "download";
    }
}

void JsInitialize::ParseMethod(napi_env env, napi_value jsConfig, Config &config)
{
    config.method = NapiUtils::Convert2String(env, jsConfig, "method");
    if (config.method.empty()) {
        if (config.version == Version::API10) {
            config.method = config.action == Action::UPLOAD ? "PUT" : "GET";
        } else {
            config.method = "POST";
        }
    } else {
        transform(config.method.begin(), config.method.end(), config.method.begin(), ::toupper);
    }
}

bool JsInitialize::ParseData(napi_env env, napi_value jsConfig, Config &config)
{
    napi_value value = NapiUtils::GetNamedProperty(env, jsConfig, "data");
    if (value == nullptr) {
        REQUEST_HILOGE("ParseData err");
        return true;
    }

    if (config.action == Action::UPLOAD) {
        return Convert2FormItems(env, value, config.forms, config.files);
    }
    config.data = NapiUtils::Convert2String(env, value);
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

bool JsInitialize::GetFormItems(napi_env env, napi_value jsVal, std::vector<FormItem> &forms,
    std::vector<FileSpec> &files)
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
    if (NapiUtils::GetValueType(env, value) == napi_string) {
        FormItem form;
        form.name = name;
        form.value = NapiUtils::Convert2String(env, value);
        forms.push_back(form);
    } else if (!isArray) {
        FileSpec file;
        if (!Convert2FileSpec(env, value, name, file)) {
            return false;
        }
        files.push_back(file);
    } else {
        if (!Convert2FileSpecs(env, value, name, files)) {
            return false;
        }
    }
}

bool JsInitialize::Convert2FormItems(napi_env env, napi_value jsValue, std::vector<FormItem> &forms,
    std::vector<FileSpec> &files)
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
        napi_get_element(env, jsValue, i, &jsVal);
        if (jsVal == nullptr) {
            REQUEST_HILOGE("Get element jsVal failed");
            return false;
        }
        GetFormItems(env, jsVal, forms, files);
        napi_close_handle_scope(env, scope);
    }
    if (files.empty()) {
        return false;
    }
    return true;
}

bool JsInitialize::Convert2FileSpecs(napi_env env, napi_value jsValue, const std::string &name,
    std::vector<FileSpec> &files)
{
    uint32_t length = 0;
    napi_get_array_length(env, jsValue, &length);
    for (uint32_t i = 0; i < length; ++i) {
        napi_value jsVal = nullptr;
        napi_handle_scope scope = nullptr;
        napi_open_handle_scope(env, &scope);
        napi_get_element(env, jsValue, i, &jsVal);
        if (jsVal == nullptr) {
            return false;
        }
        FileSpec file;
        bool ret = Convert2FileSpec(env, jsVal, name, file);
        if (!ret) {
            return false;
        }
        files.push_back(file);
        napi_close_handle_scope(env, scope);
    }
    return true;
}

void JsInitialize::InterceptData(const std::string &str, const std::string &in, std::string &out)
{
    std::size_t position = in.find_last_of(str.c_str());
    if (position == std::string::npos) {
        return;
    }
    out = std::string(in, position + 1);
    out.erase(out.find_last_not_of(" ") + 1);
}

bool JsInitialize::Convert2FileSpec(napi_env env, napi_value jsValue, const std::string &name, FileSpec &file)
{
    file.name = name;
    file.uri = NapiUtils::Convert2String(env, jsValue, "path");
    if (file.uri.empty()) {
        return false;
    }
    file.filename = NapiUtils::Convert2String(env, jsValue, "filename");
    file.type = NapiUtils::Convert2String(env, jsValue, "mimetype");
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
    return stageMode;
}

bool JsInitialize::ParseConfigV9(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo)
{
    REQUEST_HILOGD("ParseConfigV9 in");
    config.action = NapiUtils::GetRequestAction(env, jsConfig);
    config.headers = ParseMap(env, jsConfig, "headers");
    if (!ParseUrl(env, jsConfig, config.url)) {
        errInfo = "Parse url error";
        return false;
    }
    auto func = config.action == Action::UPLOAD ? ParseUploadConfig : ParseDownloadConfig;
    if (!func(env, jsConfig, config, errInfo)) {
        return false;
    }
    ParseTitle(env, jsConfig, config);
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
        errInfo = "Parse config files error";
        return false;
    }

    napi_value jsData = NapiUtils::GetNamedProperty(env, jsConfig, PARAM_KEY_DATA);
    if (jsData == nullptr) {
        errInfo = "Parse config data error";
        return false;
    }
    config.forms = NapiUtils::Convert2RequestDataVector(env, jsData);
    return true;
}

bool JsInitialize::ParseDownloadConfig(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo)
{
    REQUEST_HILOGD("ParseDownloadConfig in");
    config.metered = NapiUtils::Convert2Boolean(env, jsConfig, "enableMetered");
    config.roaming = NapiUtils::Convert2Boolean(env, jsConfig, "enableRoaming");
    config.description = NapiUtils::Convert2String(env, jsConfig, PARAM_KEY_DESCRIPTION);
    uint32_t type = NapiUtils::Convert2Uint32(env, jsConfig, PARAM_KEY_NETWORKTYPE);
    if (type == 0) {
        config.network = Network::WIFI;
    } else if (type == 1) {
        config.network = Network::CELLULAR;
    } else {
        config.network = Network::ANY;
    }
    config.saveas = NapiUtils::Convert2String(env, jsConfig, PARAM_KEY_FILE_PATH);
    config.background = NapiUtils::Convert2Boolean(env, jsConfig, PARAM_KEY_BACKGROUND);
    config.method = "GET";
    return true;
}

void JsInitialize::CreatProperties(napi_env env, napi_value &self, napi_value config, JsTask *task)
{
    if (task->config_.version == Version::API10) {
        NapiUtils::SetStringPropertyUtf8(env, self, "tid", task->GetTid());
        napi_set_named_property(env, self, "conf", config);
    }
}
} // namespace OHOS::Request