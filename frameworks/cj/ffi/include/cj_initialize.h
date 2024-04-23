/*
 * Copyright (C) 2024 Huawei Device Co., Ltd.
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

#ifndef OH_CJ_INITIALIZE_H
#define OH_CJ_INITIALIZE_H

#include <vector>
#include "ability.h"
#include "directory_ex.h"
#include "constant.h"
#include "js_common.h"
#include "napi_base_context.h"
#include "cj_request_ffi.h"

namespace OHOS::CJSystemapi::Request {
using OHOS::Request::ExceptionError;
using OHOS::Request::Config;
using OHOS::Request::FormItem;
using OHOS::Request::FileSpec;
using OHOS::Request::Action;
using OHOS::Request::Network;
using OHOS::Request::Mode;
using OHOS::AbilityRuntime::Context;
class CJInitialize {
public:
    CJInitialize() = default;
    ~CJInitialize() = default;

    static void StringSplit(const std::string &str, const char delim, std::vector<std::string> &elems);
    static bool GetBaseDir(std::string &baseDir);
    
    static ExceptionError ParseConfig(OHOS::AbilityRuntime::Context *context, const CConfig *ffiConfig, Config &config);
    static ExceptionError ParseBundleName(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context,
        std::string &config);
    static bool ParseUrl(std::string &url);
    static bool ParseCertsPath(std::string &url, std::vector<std::string> &certsPath);
    static bool ParseFormItems(const CFormItemArr *cForms, std::vector<FormItem> &forms, std::vector<FileSpec> &files);
    static bool ParseData(const CConfig *config, Config &out);
    static bool Convert2FileSpec(const CFileSpec *cFile, const char *name, FileSpec &file);
    static bool Convert2FileSpecs(const CFileSpecArr *cFiles, const char *name, std::vector<FileSpec> &files);
    static bool ParseIndex(Config &config);
    static int64_t ParseBegins(int64_t &begins);
    static bool ParseTitle(Config &config);
    static bool ParseToken(Config &config);
    static bool ParseDescription(std::string &description);
    static bool ParseSaveas(Config &config);
    static void ParseMethod(Config &config);
    static void ParseNetwork(Network &network);
    static void ParseBackGround(Mode mode, bool &background);

    static ExceptionError CheckFilePath(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config);
    static bool CheckPathBaseDir(const std::string &filepath, std::string &baseDir);
    static bool CreateDirs(const std::vector<std::string> &pathDirs);
    static bool InterceptData(const std::string &str, const std::string &in, std::string &out);
    static bool GetInternalPath(const std::string &fileUri,
        const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config, std::string &filePath);
    static ExceptionError GetFD(const std::string &path, const Config &config, int32_t &fd);
    static bool FindDir(const std::string &pathDir);
private:
    static bool CheckDownloadFilePath(
        const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config, std::string &errInfo);
    static bool StandardizePath(
        const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config, std::string &path);
    static bool CacheToWhole(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, std::string &path);
    static bool FileToWhole(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config,
        std::string &path);
    static bool PathVecToNormal(const std::vector<std::string> &in, std::vector<std::string> &out);
    static bool WholeToNormal(const std::string &wholePath, std::string &normalPath, std::vector<std::string> &out);
    static ExceptionError CheckUploadBodyFiles(Config &config, const std::string &filePath);
    static ExceptionError UploadBodyFileProc(std::string &fileName, Config &config);
};
} // OHOS::CJSystemapi::Request
#endif // CJ_INITIALIZE_H
