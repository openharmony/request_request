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
#ifndef REQUEST_JS_UTIL_H
#define REQUEST_JS_UTIL_H

#include <cstdint>
#include <map>
#include <vector>

#include "constant.h"
#include "context.h"
#include "napi/native_api.h"
#include "napi/native_common.h"
#include "napi/native_node_api.h"
#include "upload/upload_common.h"
#include "upload/upload_hilog_wrapper.h"
#include "upload_config.h"

#define DECLARE_NAPI_METHOD(name, func)         \
    {                                           \
        name, 0, func, 0, 0, 0, napi_default, 0 \
    }
namespace OHOS::Request::Upload {
class JSUtil {
public:
    static constexpr int32_t MAX_ARGC = 6;
    static constexpr int32_t MAX_NUMBER_BYTES = 8;
    static constexpr int32_t MAX_LEN = 4096;
    static constexpr const char *SEPARATOR = ": ";

    static std::string Convert2String(napi_env env, napi_value jsString);
    static napi_value Convert2JSString(napi_env env, const std::string &cString);
    static napi_value Convert2JSValue(napi_env env, int32_t value);
    static napi_value Convert2JSUploadResponse(napi_env env, const Upload::UploadResponse &response);
    static bool ParseFunction(napi_env env, napi_value &object, const char *name, napi_ref &output);
    static std::shared_ptr<Upload::UploadConfig> ParseUploadConfig(
        napi_env env, napi_value jsConfig, const std::string &version);

    static std::vector<Upload::File> Convert2FileVector(napi_env env, napi_value jsFiles, const std::string &version);

    static Upload::RequestData Convert2RequestData(napi_env env, napi_value jsRequestData);
    static std::vector<Upload::RequestData> Convert2RequestDataVector(napi_env env, napi_value jsRequestDatas);

    static bool CheckConfig(const Upload::UploadConfig &config);
    static bool CheckMethod(const std::string &method);
    static bool CheckUrl(const std::string &url);
    static napi_value GetNamedProperty(napi_env env, napi_value object, const std::string &propertyName);
    static bool HasNamedProperty(napi_env env, napi_value object, const std::string &propertyName);
    static bool ToUploadOption(napi_env env, napi_value jsConfig, Upload::UploadConfig &config);
    static bool SetData(napi_env env, napi_value jsConfig, Upload::UploadConfig &config);
    static bool SetFiles(napi_env env, napi_value jsConfig, Upload::UploadConfig &config);
    static bool Convert2FileL5(napi_env env, napi_value jsFile, Upload::File &file);
    static bool SetMandatoryParam(napi_env env, napi_value jsValue, const std::string &str, std::string &out);
    static bool SetOptionalParam(napi_env env, napi_value jsValue, const std::string &str, std::string &out);
    static bool ParseHeader(napi_env env, napi_value configValue, std::map<std::string, std::string> &header);
    static napi_value CreateBusinessError(
        napi_env env, const ExceptionErrorCode &errorCode, const std::string &errorMessage);
};
} // namespace OHOS::Request::Upload
#endif // REQUEST_JS_UTIL_H
