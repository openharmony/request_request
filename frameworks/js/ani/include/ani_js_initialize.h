/*
 * Copyright (C) 2025 Huawei Device Co., Ltd.
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

#ifndef ANI_JS_INITIALIZE_H
#define ANI_JS_INITIALIZE_H

#include <ani.h>
#include "ability.h"
#include "data_ability_helper.h"
#include "directory_ex.h"
#include "request_common.h"

namespace OHOS::Request {
static const std::map<ExceptionErrorCode, std::string> ErrorCodeToMsg{
    { E_OK, E_OK_INFO },
    { E_PERMISSION, E_PERMISSION_INFO },
    { E_PARAMETER_CHECK, E_PARAMETER_CHECK_INFO },
    { E_UNSUPPORTED, E_UNSUPPORTED_INFO },
    { E_FILE_IO, E_FILE_IO_INFO },
    { E_FILE_PATH, E_FILE_PATH_INFO },
    { E_SERVICE_ERROR, E_SERVICE_ERROR_INFO },
    { E_OTHER, E_OTHER_INFO },
};

class JsInitialize {
public:
    JsInitialize() = default;
    ~JsInitialize() = default;

    static bool GetAppBaseDir(std::string &baseDir);
    static bool CheckBelongAppBaseDir(const std::string &filepath, std::string &baseDir);
    static void StringSplit(const std::string &str, const char delim, std::vector<std::string> &elems);
    static void StringTrim(std::string &str);
    static bool CreateDirs(const std::vector<std::string> &pathDirs);
    static bool FindDir(const std::string &pathDir);

    static std::shared_ptr<OHOS::AbilityRuntime::Context> GetContext(ani_env *env, ani_object object);

    static bool GetInternalPath(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config,
        std::string &path, std::string &errInfo);
    
    static bool CheckUploadBodyFiles(const std::string &filePath, Config &config, ExceptionError &error);
    static bool CheckPathIsFile(const std::string &path, ExceptionError &error);
    static bool CheckPathOverWrite(const std::string &path, const Config &config, ExceptionError &error);
    static bool GetFdUpload(const std::string &path, const Config &config, ExceptionError &error);
    static bool GetFdDownload(const std::string &path, const Config &config, ExceptionError &error);
    static bool InterceptData(const std::string &str, const std::string &in, std::string &out);
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

    static bool Convert2FileSpecs(
        ani_env *env, ani_object aniValue, const std::string &name, std::vector<FileSpec> &files);
    static bool Convert2FileSpec(ani_env *env, ani_object aniValue, const std::string &name, FileSpec &file);
};
}
#endif // ANI_JS_INITIALIZE_H