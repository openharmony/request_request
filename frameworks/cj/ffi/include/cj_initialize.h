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
#include "cj_request_ffi.h"
#include "constant.h"
#include "directory_ex.h"
#include "napi_base_context.h"
#include "request_common.h"

namespace OHOS::CJSystemapi::Request {
using OHOS::AbilityRuntime::Context;
using OHOS::Request::Config;
using OHOS::Request::ExceptionError;
using OHOS::Request::FileSpec;
using OHOS::Request::FormItem;
using OHOS::Request::Mode;
using OHOS::Request::Network;

static constexpr uint32_t TOKEN_MAX_BYTES = 2048;
static constexpr uint32_t TOKEN_MIN_BYTES = 8;

class CJInitialize {
public:
    CJInitialize() = default;
    ~CJInitialize() = default;

    static bool GetAppBaseDir(std::string &baseDir);
    static bool CheckBelongAppBaseDir(const std::string &filepath, std::string &baseDir);
    static void StringSplit(const std::string &str, const char delim, std::vector<std::string> &elems);
    static void StringTrim(std::string &str);
    static bool GetBaseDir(std::string &baseDir);

    static ExceptionError ParseConfig(OHOS::AbilityRuntime::Context *context, const CConfig *ffiConfig, Config &config);
    static ExceptionError ParseBundleName(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context,
                                          std::string &config);
    static bool ParseUrl(std::string &url, std::string &errInfo);
    static bool ParseCertsPath(std::string &url, std::vector<std::string> &certsPath, std::string &errInfo);
    static bool ParseFormItems(const CFormItemArr *cForms, std::vector<FormItem> &forms, std::vector<FileSpec> &files,
                               std::string &errInfo);
    static bool ParseData(const CConfig *config, Config &out, std::string &errInfo);
    static bool Convert2FileSpec(const CFileSpec *cFile, const char *name, FileSpec &file);
    static bool Convert2FileSpecs(const CFileSpecArr *cFiles, const char *name, std::vector<FileSpec> &files);
    static bool ParseIndex(Config &config, std::string &errInfo);
    static int64_t ParseBegins(int64_t &begins);
    static bool ParseTitle(Config &config, std::string &errInfo);
    static bool ParseToken(Config &config, std::string &errInfo);
    static bool ParseDescription(std::string &description, std::string &errInfo);
    static void ParseGauge(Config &config);
    static bool ParseSaveas(Config &config, std::string &errInfo);
    static void ParseCertificatePins(std::string &url, std::string &certificatePins);
    static void ParseMethod(Config &config);
    static void ParseNetwork(Network &network);
    static void ParseBackGround(Mode mode, bool &background);

    static ExceptionError CheckFileSpec(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config);
    static ExceptionError CheckFilePath(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config);
    static ExceptionError CheckFilePathV2(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context,
                                          Config &config);
    static bool CheckPathBaseDir(const std::string &filepath, std::string &baseDir);
    static bool CreateDirs(const std::vector<std::string> &pathDirs);
    static bool InterceptData(const std::string &str, const std::string &in, std::string &out);
    static bool GetInternalPath(const std::string &fileUri,
                                const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config,
                                std::string &filePath);
    static ExceptionError GetFD(const std::string &path, const Config &config, int32_t &fd);
    static bool FindDir(const std::string &pathDir);

private:
    static bool CheckDownloadFilePath(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config,
                                      std::string &errInfo);
    static bool StandardizePath(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config,
                                std::string &path);
    static bool CacheToWhole(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, std::string &path);
    static bool FileToWhole(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config,
                            std::string &path);
    static bool PathVecToNormal(const std::vector<std::string> &in, std::vector<std::string> &out);
    static bool WholeToNormal(std::string &path, std::vector<std::string> &out);
    static bool IsUserFile(const std::string &filePath);
    static ExceptionError CheckUploadBodyFiles(Config &config, const std::string &filePath);
    static ExceptionError UploadBodyFileProcV2(std::string &fileName, Config &config);
    static ExceptionError UploadBodyFileProc(std::string &fileName, Config &config);
    static bool GetSandboxPath(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config,
        std::string &path, std::vector<std::string> &pathVec, std::string &errInfo);
    static bool CheckUserFileSpec(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config,
        FileSpec &file, ExceptionError &error, bool isUpload);
    static bool CheckUploadFileSpec(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config,
        FileSpec &file, ExceptionError &error);
    static bool CheckDownloadFile(
        const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config, ExceptionError &error);
    static bool CheckUploadFiles(
        const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, Config &config, ExceptionError &error);
    static bool GetFdUpload(const std::string &path, const Config &config, ExceptionError &error);
    static bool GetFdDownload(const std::string &path, const Config &config, ExceptionError &error);
    static bool CheckPathIsFile(const std::string &path, ExceptionError &error);
    static void StandardizeFileSpec(FileSpec &file);
};
} // namespace OHOS::CJSystemapi::Request
#endif // CJ_INITIALIZE_H
