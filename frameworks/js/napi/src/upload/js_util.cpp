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

#include "upload/js_util.h"

#include <securec.h>

#include <regex>
#include <string>

#include "napi_utils.h"

namespace OHOS::Request::Upload {

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

std::string JSUtil::Convert2String(napi_env env, napi_value jsString)
{
    size_t maxLen = JSUtil::MAX_LEN;
    napi_status status = napi_get_value_string_utf8(env, jsString, NULL, 0, &maxLen);
    if (status != napi_ok) {
        GET_AND_THROW_LAST_ERROR((env));
        maxLen = JSUtil::MAX_LEN;
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

napi_value JSUtil::Convert2JSString(napi_env env, const std::string &cString)
{
    napi_value jsValue = nullptr;
    napi_create_string_utf8(env, cString.c_str(), cString.size(), &jsValue);
    return jsValue;
}

napi_value JSUtil::Convert2JSValue(napi_env env, int32_t value)
{
    napi_value jsValue;
    napi_status status = napi_create_int32(env, value, &jsValue);
    if (status != napi_ok) {
        return nullptr;
    }
    return jsValue;
}

napi_value JSUtil::Convert2JSUploadResponse(napi_env env, const Upload::UploadResponse &response)
{
    napi_value jsResponse = nullptr;
    napi_create_object(env, &jsResponse);
    napi_set_named_property(env, jsResponse, "code", Convert2JSValue(env, response.code));
    napi_set_named_property(env, jsResponse, "data", Convert2JSString(env, response.data));
    napi_set_named_property(env, jsResponse, "headers", Convert2JSString(env, response.headers));
    return jsResponse;
}

bool JSUtil::ParseFunction(napi_env env, napi_value &object, const char *name, napi_ref &output)
{
    napi_value value = GetNamedProperty(env, object, name);
    if (value == nullptr) {
        return false;
    }
    napi_valuetype valueType = napi_null;
    auto ret = napi_typeof(env, value, &valueType);
    if ((ret != napi_ok) || (valueType != napi_function)) {
        return false;
    }
    napi_create_reference(env, value, 1, &output);
    return true;
}

std::shared_ptr<UploadConfig> JSUtil::ParseUploadConfig(napi_env env, napi_value jsConfig, const std::string &version)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "ParseUploadConfig in");
    UploadConfig config;
    config.protocolVersion = version;
    bool ret = ToUploadOption(env, jsConfig, config);
    if ((!ret) || (!CheckConfig(config))) {
        return nullptr;
    }
    return std::make_shared<UploadConfig>(config);
}

bool JSUtil::CheckConfig(const UploadConfig &config)
{
    if (!CheckUrl(config.url)) {
        return false;
    }
    if (config.files.empty()) {
        return false;
    }
    return CheckMethod(config.method);
}

bool JSUtil::CheckUrl(const std::string &url)
{
    if (url.empty()) {
        return false;
    }
    return regex_match(url, std::regex("^http(s)?:\\/\\/.+"));
}

bool JSUtil::CheckMethod(const std::string &method)
{
    return (method == POST || method == PUT);
}

napi_value JSUtil::GetNamedProperty(napi_env env, napi_value object, const std::string &propertyName)
{
    napi_value value = nullptr;
    bool hasProperty = false;
    NAPI_CALL(env, napi_has_named_property(env, object, propertyName.c_str(), &hasProperty));
    if (!hasProperty) {
        return value;
    }
    NAPI_CALL(env, napi_get_named_property(env, object, propertyName.c_str(), &value));
    return value;
}

bool JSUtil::HasNamedProperty(napi_env env, napi_value object, const std::string &propertyName)
{
    bool hasProperty = false;
    NAPI_CALL_BASE(env, napi_has_named_property(env, object, propertyName.c_str(), &hasProperty), false);
    return hasProperty;
}

bool JSUtil::SetData(napi_env env, napi_value jsConfig, UploadConfig &config)
{
    if (!HasNamedProperty(env, jsConfig, "data")) {
        return true;
    }
    napi_value data = nullptr;
    napi_get_named_property(env, jsConfig, "data", &data);
    if (data == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "GetNamedProperty SetData failed");
        return false;
    }
    config.data = Convert2RequestDataVector(env, data);
    return true;
}

bool JSUtil::SetFiles(napi_env env, napi_value jsConfig, UploadConfig &config)
{
    napi_value files = GetNamedProperty(env, jsConfig, "files");
    if (files == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "GetNamedProperty SetFiles failed");
        return false;
    }
    config.files = Convert2FileVector(env, files, config.protocolVersion);
    return true;
}

bool JSUtil::ToUploadOption(napi_env env, napi_value jsConfig, UploadConfig &config)
{
    if (!SetMandatoryParam(env, jsConfig, "url", config.url)) {
        return false;
    }
    if (!SetData(env, jsConfig, config)) {
        return false;
    }
    if (!SetFiles(env, jsConfig, config)) {
        return false;
    }
    if (!ParseHeader(env, jsConfig, config.header)) {
        return false;
    }
    if (!SetOptionalParam(env, jsConfig, "method", config.method)) {
        return false;
    }
    return true;
}

bool JSUtil::ParseHeader(napi_env env, napi_value configValue, std::map<std::string, std::string> &header)
{
    if (!NapiUtils::HasNamedProperty(env, configValue, "header")) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "No header present, Reassign value");
        header[tlsVersion] = TLS_VERSION;
        header[cipherList] = TLS_CIPHER;
        return true;
    }
    napi_value jsHeader = NapiUtils::GetNamedProperty(env, configValue, "header");
    if (NapiUtils::GetValueType(env, jsHeader) != napi_object) {
        return false;
    }
    auto names = NapiUtils::GetPropertyNames(env, jsHeader);
    auto iter = find(names.begin(), names.end(), cipherList);
    if (iter == names.end()) {
        header[cipherList] = TLS_CIPHER;
    }
    for (iter = names.begin(); iter != names.end(); ++iter) {
        auto value = NapiUtils::Convert2String(env, jsHeader, *iter);
        if (!value.empty()) {
            header[*iter] = value;
        }
    }
    return true;
}

bool JSUtil::SetMandatoryParam(napi_env env, napi_value jsValue, const std::string &str, std::string &out)
{
    napi_value value = GetNamedProperty(env, jsValue, str);
    if (value == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "SetMandatoryParam failed");
        return false;
    }
    out = Convert2String(env, value);
    return true;
}

bool JSUtil::SetOptionalParam(napi_env env, napi_value jsValue, const std::string &str, std::string &out)
{
    if (!HasNamedProperty(env, jsValue, str)) {
        out = (str == "method" ? "POST" : "");
        return true;
    }
    napi_value value = nullptr;
    napi_get_named_property(env, jsValue, str.c_str(), &value);
    if (value == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "SetOptionalParam failed");
        return false;
    }
    out = Convert2String(env, value);
    return true;
}

bool JSUtil::Convert2FileL5(napi_env env, napi_value jsFile, Upload::File &file)
{
    if (!SetOptionalParam(env, jsFile, "filename", file.filename)) {
        return false;
    }
    if (!SetOptionalParam(env, jsFile, "name", file.name)) {
        return false;
    }
    if (!SetMandatoryParam(env, jsFile, "uri", file.uri)) {
        return false;
    }
    if (!SetOptionalParam(env, jsFile, "type", file.type)) {
        return false;
    }
    return true;
}

std::vector<Upload::File> JSUtil::Convert2FileVector(napi_env env, napi_value jsFiles, const std::string &version)
{
    bool isArray = false;
    napi_is_array(env, jsFiles, &isArray);
    NAPI_ASSERT_BASE(env, isArray, "not array", {});
    uint32_t length = 0;
    napi_get_array_length(env, jsFiles, &length);
    std::vector<Upload::File> files;
    for (uint32_t i = 0; i < length; ++i) {
        napi_value jsFile = nullptr;
        napi_handle_scope scope = nullptr;
        napi_open_handle_scope(env, &scope);
        napi_get_element(env, jsFiles, i, &jsFile);
        if (jsFile == nullptr) {
            continue;
        }

        Upload::File file;
        bool ret = Convert2FileL5(env, jsFile, file);
        if (!ret) {
            continue;
        }
        files.push_back(file);
        napi_close_handle_scope(env, scope);
    }
    return files;
}

Upload::RequestData JSUtil::Convert2RequestData(napi_env env, napi_value jsRequestData)
{
    Upload::RequestData requestData;
    napi_value value = nullptr;
    napi_get_named_property(env, jsRequestData, "name", &value);
    if (value != nullptr) {
        requestData.name = Convert2String(env, value);
    }
    value = nullptr;
    napi_get_named_property(env, jsRequestData, "value", &value);
    if (value != nullptr) {
        requestData.value = Convert2String(env, value);
    }
    return requestData;
}

std::vector<Upload::RequestData> JSUtil::Convert2RequestDataVector(napi_env env, napi_value jsRequestDatas)
{
    bool isArray = false;
    napi_is_array(env, jsRequestDatas, &isArray);
    NAPI_ASSERT_BASE(env, isArray, "not array", {});
    uint32_t length = 0;
    napi_get_array_length(env, jsRequestDatas, &length);
    std::vector<Upload::RequestData> requestDatas;
    for (uint32_t i = 0; i < length; ++i) {
        napi_value requestData = nullptr;
        napi_get_element(env, jsRequestDatas, i, &requestData);
        if (requestData == nullptr) {
            continue;
        }
        requestDatas.push_back(Convert2RequestData(env, requestData));
    }
    return requestDatas;
}

napi_value JSUtil::CreateBusinessError(
    napi_env env, const ExceptionErrorCode &errorCode, const std::string &errorMessage)
{
    napi_value error = nullptr;
    napi_value code = nullptr;
    napi_value msg = nullptr;
    auto iter = ErrorCodeToMsg.find(errorCode);
    std::string strMsg = (iter != ErrorCodeToMsg.end() ? iter->second : "") + "   " + errorMessage;
    NAPI_CALL(env, napi_create_string_utf8(env, strMsg.c_str(), strMsg.length(), &msg));
    NAPI_CALL(env, napi_create_uint32(env, errorCode, &code));
    NAPI_CALL(env, napi_create_error(env, nullptr, msg, &error));
    napi_set_named_property(env, error, "code", code);
    return error;
}

} // namespace OHOS::Request::Upload