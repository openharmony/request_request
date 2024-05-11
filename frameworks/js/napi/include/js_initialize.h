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

#ifndef JS_INITIALIZE_H
#define JS_INITIALIZE_H

#include "ability.h"
#include "data_ability_helper.h"
#include "directory_ex.h"
#include "js_task.h"
#include "napi_base_context.h"

namespace OHOS::Request {
static constexpr uint32_t TOKEN_MAX_BYTES = 2048;
static constexpr uint32_t TOKEN_MIN_BYTES = 8;
static constexpr int ACL_SUCC = 0;
static const std::string SA_PERMISSION_RWX = "g:3815:rwx";
static const std::string SA_PERMISSION_X = "g:3815:x";
static const std::string SA_PERMISSION_CLEAN = "g:3815:---";
static const std::string AREA1 = "el1";
static const std::string AREA2 = "el2";
class JsInitialize {
public:
    JsInitialize() = default;
    ~JsInitialize() = default;

    static napi_value Initialize(napi_env env, napi_callback_info info, Version version, bool firstInit = true);
    static void CreatProperties(napi_env env, napi_value &self, napi_value config, JsTask *task);
    static napi_status GetContext(
        napi_env env, napi_value value, std::shared_ptr<OHOS::AbilityRuntime::Context> &context);
    static bool GetAppBaseDir(std::string &baseDir);
    static bool CheckBelongAppBaseDir(const std::string &filepath, std::string &baseDir);
    static void StringSplit(const std::string &str, const char delim, std::vector<std::string> &elems);
    static void StringTrim(std::string &str);
    static bool CreateDirs(const std::vector<std::string> &pathDirs);
    static bool FindDir(const std::string &pathDir);

private:
    static ExceptionError InitParam(
        napi_env env, napi_value *argv, std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config);
    static bool ParseConfig(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo);
    static bool ParseConfigV9(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo);
    static void SetParseConfig(napi_env env, napi_value jsConfig, Config &config);
    static bool ParseUploadConfig(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo);
    static bool ParseDownloadConfig(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo);
    static bool ParseAction(napi_env env, napi_value jsConfig, Action &action, std::string &errInfo);
    static bool ParseUrl(napi_env env, napi_value jsConfig, std::string &url, std::string &errInfo);
    static bool ParseProxy(napi_env env, napi_value jsConfig, std::string &proxy, std::string &errInfo);
    static bool ParseCertsPath(
        napi_env env, napi_value jsConfig, std::vector<std::string> &certsPath, std::string &errInfo);
    static bool ParseData(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo);
    static bool ParseIndex(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo);
    static bool ParseName(napi_env env, napi_value jsVal, std::string &name);
    static bool ParseTitle(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo);
    static void ParseNetwork(napi_env env, napi_value jsConfig, Network &network);
    static void ParseCertificatePins(napi_env env, std::string &url, std::string &certificatePins);
    static void ParseMethod(napi_env env, napi_value jsConfig, Config &config);
    static void ParseRedirect(napi_env env, napi_value jsConfig, bool &redirect);
    static void ParseRoaming(napi_env env, napi_value jsConfig, Config &config);
    static void ParseRetry(napi_env env, napi_value jsConfig, bool &retry);
    static bool ParseSaveas(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo);
    static bool ParseToken(napi_env env, napi_value jsConfig, Config &config, std::string &errInfo);
    static bool ParseDescription(napi_env env, napi_value jsConfig, std::string &description, std::string &errInfo);
    static int64_t ParseEnds(napi_env env, napi_value jsConfig);
    static int64_t ParseBegins(napi_env env, napi_value jsConfig);
    static uint32_t ParsePriority(napi_env env, napi_value jsConfig);
    static std::map<std::string, std::string> ParseMap(
        napi_env env, napi_value jsConfig, const std::string &propertyName);

    static bool GetFormItems(
        napi_env env, napi_value jsVal, std::vector<FormItem> &forms, std::vector<FileSpec> &files);
    static bool Convert2FormItems(napi_env env, napi_value jsValue, std::vector<FormItem> &forms,
        std::vector<FileSpec> &files, std::string &errInfo);
    static bool Convert2FileSpecs(
        napi_env env, napi_value jsValue, const std::string &name, std::vector<FileSpec> &files);
    static bool Convert2FileSpec(napi_env env, napi_value jsValue, const std::string &name, FileSpec &file);
    static bool GetInternalPath(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config,
        std::string &path, std::string &errInfo);

    static bool CheckUploadBodyFiles(const std::string &filePath, Config &config, ExceptionError &error);
    static bool GetFD(const std::string &path, const Config &config, int32_t &fd, ExceptionError &error);
    static bool InterceptData(const std::string &str, const std::string &in, std::string &out);
    static bool IsStageMode(napi_env env, napi_value value);
    static bool CheckDownloadFilePath(
        const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config, std::string &errInfo);
    static bool StandardizePath(
        const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config, std::string &path);
    static bool BaseToWhole(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, std::string &path);
    static bool CacheToWhole(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, std::string &path);
    static bool FileToWhole(
        const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config, std::string &path);
    static bool WholeToNormal(std::string &path, std::vector<std::string> &out);
    static bool PathVecToNormal(const std::vector<std::string> &in, std::vector<std::string> &out);
    static bool IsUserFile(const std::string &filePath);
    static void StandardizeFileSpec(FileSpec &file);
    static bool GetSandboxPath(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config,
        std::string &path, std::vector<std::string> &pathVec, std::string &errInfo);
    static bool CheckUserFileSpec(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config,
        FileSpec &file, ExceptionError &error);
    static bool CheckUploadFileSpec(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config,
        FileSpec &file, ExceptionError &error);
    static bool CheckDownloadFile(
        const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config, ExceptionError &error);
    static bool CheckUploadFiles(
        const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config, ExceptionError &error);
    static bool CheckFilePath(
        const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config, ExceptionError &error);
};
} // namespace OHOS::Request
#endif // JS_INITIALIZE_H
