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

#include <securec.h>
#include <regex>
#include <string>
#include "js_util.h"

using namespace OHOS::Request::Upload;
namespace OHOS::Request::UploadNapi {

static const std::map<Download::ExceptionErrorCode, std::string> ErrorCodeToMsg {
    {Download::EXCEPTION_OK, Download::EXCEPTION_OK_INFO },
    {Download::EXCEPTION_PERMISSION, Download::EXCEPTION_PERMISSION_INFO },
    {Download::EXCEPTION_PARAMETER_CHECK, Download::EXCEPTION_PARAMETER_CHECK_INFO },
    {Download::EXCEPTION_UNSUPPORTED, Download::EXCEPTION_UNSUPPORTED_INFO },
    {Download::EXCEPTION_FILE_IO, Download::EXCEPTION_FILE_IO_INFO },
    {Download::EXCEPTION_FILE_PATH, Download::EXCEPTION_FILE_PATH_INFO },
    {Download::EXCEPTION_SERVICE_ERROR, Download::EXCEPTION_SERVICE_ERROR_INFO },
    {Download::EXCEPTION_OTHER, Download::EXCEPTION_OTHER_INFO },
};

void JSUtil::ThrowError(napi_env env, Download::ExceptionErrorCode code, const std::string &msg)
{
    napi_value error = CreateBusinessError(env, code, msg);
    napi_throw(env, error);
}

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

std::vector<std::string> JSUtil::Convert2StrVector(napi_env env, napi_value value)
{
    uint32_t arrLen = 0;
    napi_get_array_length(env, value, &arrLen);
    if (arrLen == 0) {
        return {};
    }
    std::vector<std::string> result;
    for (size_t i = 0; i < arrLen; ++i) {
        napi_value element;
        napi_get_element(env, value, i, &element);
        result.push_back(Convert2String(env, element));
    }
    return result;
}

std::vector<std::string> JSUtil::Convert2Header(napi_env env, napi_value value)
{
    std::vector<std::string> result;
    napi_value keyArr = nullptr;
    napi_status status = napi_get_property_names(env, value, &keyArr);
    if (status != napi_ok) {
        return result;
    }

    uint32_t len = 0;
    status = napi_get_array_length(env, keyArr, &len);
    if (status != napi_ok) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "Convert2Header. napi_get_array_length statue Error");
        return result;
    }
    for (uint32_t i = 0; i < len; i++) {
        napi_value keyNapiValue = nullptr;
        napi_get_element(env, keyArr, i, &keyNapiValue);

        napi_valuetype valueType;
        napi_typeof(env, keyNapiValue, &valueType);
        if (valueType != napi_valuetype::napi_string) {
            continue;
        }

        char key[JSUtil::MAX_LEN] = { 0 };
        size_t cValueLength = 0;
        napi_get_value_string_utf8(env, keyNapiValue, key, JSUtil::MAX_LEN - 1, &cValueLength);

        napi_value jsvalue = nullptr;
        napi_get_named_property(env, value, key, &jsvalue);
        if (jsvalue != nullptr) {
            std::string item(key);
            item.append(SEPARATOR);
            item.append(Convert2String(env, jsvalue));
            result.push_back(item);
        }
    }
    return result;
}

napi_value JSUtil::Convert2JSStringVector(napi_env env, const std::vector<std::string> &cStrings)
{
    napi_value jsStrings = nullptr;
    napi_create_array_with_length(env, cStrings.size(), &jsStrings);
    int index = 0;
    for (const auto &cString : cStrings) {
        napi_value jsString = Convert2JSString(env, cString);
        napi_set_element(env, jsStrings, index++, jsString);
    }
    return jsStrings;
}

napi_value JSUtil::Convert2JSValue(napi_env env, const std::vector<int32_t> &cInts)
{
    napi_value jsInts = nullptr;
    napi_create_array_with_length(env, cInts.size(), &jsInts);
    int index = 0;
    for (const auto &cInt : cInts) {
        napi_value jsInt = Convert2JSValue(env, cInt);
        napi_set_element(env, jsInts, index++, jsInt);
    }
    return jsInts;
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

napi_value JSUtil::Convert2JSValue(napi_env env, const std::vector<Upload::TaskState> &taskStates)
{
    napi_value jsTaskStates = nullptr;
    napi_create_array_with_length(env, taskStates.size(), &jsTaskStates);
    int index = 0;
    for (const auto &taskState : taskStates) {
        napi_value jsTaskState = nullptr;
        napi_create_object(env, &jsTaskState);
        napi_set_named_property(env, jsTaskState, "path", Convert2JSString(env, taskState.path));
        napi_set_named_property(env, jsTaskState, "responseCode", Convert2JSValue(env, taskState.responseCode));
        napi_set_named_property(env, jsTaskState, "message", Convert2JSString(env, taskState.message));
        napi_set_element(env, jsTaskStates, index++, jsTaskState);
    }
    return jsTaskStates;
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

napi_value JSUtil::Convert2JSString(napi_env env, const std::string &cString)
{
    napi_value jsValue = nullptr;
    napi_create_string_utf8(env, cString.c_str(), cString.size(), &jsValue);
    return jsValue;
}

std::shared_ptr<UploadConfig> JSUtil::ParseUploadConfig(napi_env env, napi_value jsConfig,
    const std::string &version)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "ParseUploadConfig in");
    UploadConfig config;
    config.protocolVersion = version;
    auto func = (config.protocolVersion == API5) ? ToUploadOption : ToUploadConfig;
    bool ret = func(env, jsConfig, config);
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

bool JSUtil::SetHeader(napi_env env, napi_value jsConfig, UploadConfig &config)
{
    if (!HasNamedProperty(env, jsConfig, "header")) {
        return true;
    }
    napi_value header = nullptr;
    napi_get_named_property(env, jsConfig, "header", &header);
    if (header == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "GetNamedProperty SetHeader failed");
        return false;
    }
    config.header = Convert2Header(env, header);
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
    if (!SetHeader(env, jsConfig, config)) {
        return false;
    }
    if (!SetOptionalParam(env, jsConfig, "method", config.method)) {
        return false;
    }
    return true;
}

bool JSUtil::ToUploadConfig(napi_env env, napi_value jsConfig, UploadConfig &config)
{
    napi_value url = GetNamedProperty(env, jsConfig, "url");
    if (url == nullptr) {
        return false;
    }
    config.url = Convert2String(env, url);

    napi_value header = GetNamedProperty(env, jsConfig, "header");
    if (header == nullptr) {
        return false;
    }
    config.header = Convert2Header(env, header);

    napi_value method = GetNamedProperty(env, jsConfig, "method");
    if (method == nullptr) {
        return false;
    }
    config.method = Convert2String(env, method);
    transform(config.method.begin(), config.method.end(), config.method.begin(), ::toupper);

    napi_value files = GetNamedProperty(env, jsConfig, "files");
    if (files == nullptr) {
        return false;
    }
    config.files = Convert2FileVector(env, files, config.protocolVersion);

    napi_value data = GetNamedProperty(env, jsConfig, "data");
    if (data == nullptr) {
        return false;
    }
    config.data = Convert2RequestDataVector(env, data);
    return true;
}

napi_value JSUtil::Convert2JSUploadConfig(napi_env env, const UploadConfig &config)
{
    napi_value jsConfig = nullptr;
    napi_create_object(env, &jsConfig);
    napi_set_named_property(env, jsConfig, "url", Convert2JSString(env, config.url));
    napi_set_named_property(env, jsConfig, "header", Convert2JSStringVector(env, config.header));
    napi_set_named_property(env, jsConfig, "method", Convert2JSString(env, config.method));
    napi_set_named_property(env, jsConfig, "files", Convert2JSFileVector(env, config.files));
    napi_set_named_property(env, jsConfig, "data", Convert2JSRequestDataVector(env, config.data));
    return jsConfig;
}

bool JSUtil::Convert2File(napi_env env, napi_value jsFile, Upload::File &file)
{
    napi_value filename = GetNamedProperty(env, jsFile, "filename");
    if (filename == nullptr) {
        return false;
    }
    file.filename = Convert2String(env, filename);

    napi_value name = GetNamedProperty(env, jsFile, "name");
    if (name == nullptr) {
        return false;
    }
    file.name = Convert2String(env, name);

    napi_value uri = GetNamedProperty(env, jsFile, "uri");
    if (uri == nullptr) {
        return false;
    }
    file.uri = Convert2String(env, uri);

    napi_value type = GetNamedProperty(env, jsFile, "type");
    if (type == nullptr) {
        return false;
    }
    file.type = Convert2String(env, type);
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

napi_value JSUtil::Convert2JSFile(napi_env env, const Upload::File &file)
{
    napi_value jsFile = nullptr;
    napi_create_object(env, &jsFile);
    napi_set_named_property(env, jsFile, "filename", Convert2JSString(env, file.filename));
    napi_set_named_property(env, jsFile, "name", Convert2JSString(env, file.name));
    napi_set_named_property(env, jsFile, "uri", Convert2JSString(env, file.uri));
    napi_set_named_property(env, jsFile, "type", Convert2JSString(env, file.type));
    return jsFile;
}

std::vector<Upload::File> JSUtil::Convert2FileVector(napi_env env, napi_value jsFiles, const std::string &version)
{
    bool isArray = false;
    napi_is_array(env, jsFiles, &isArray);
    NAPI_ASSERT_BASE(env, isArray, "not array", { });
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
        auto func = (version == API5) ? Convert2FileL5 : Convert2File;
        bool ret = func(env, jsFile, file);
        if (!ret) {
            continue;
        }
        files.push_back(file);
        napi_close_handle_scope(env, scope);
    }
    return files;
}

napi_value JSUtil::Convert2JSFileVector(napi_env env, const std::vector<Upload::File> &files)
{
    napi_value jsFiles = nullptr;
    napi_create_array_with_length(env, files.size(), &jsFiles);
    int index = 0;
    for (const auto &file : files) {
        napi_value jsFile = Convert2JSFile(env, file);
        napi_set_element(env, jsFiles, index++, jsFile);
    }
    return jsFiles;
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

napi_value JSUtil::Convert2JSRequestData(napi_env env, const Upload::RequestData &requestData)
{
    napi_value jsRequestData = nullptr;
    napi_create_object(env, &jsRequestData);
    napi_set_named_property(env, jsRequestData, "name", Convert2JSString(env, requestData.name));
    napi_set_named_property(env, jsRequestData, "value", Convert2JSString(env, requestData.value));
    return jsRequestData;
}

std::vector<Upload::RequestData> JSUtil::Convert2RequestDataVector(napi_env env, napi_value jsRequestDatas)
{
    bool isArray = false;
    napi_is_array(env, jsRequestDatas, &isArray);
    NAPI_ASSERT_BASE(env, isArray, "not array", { });
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

napi_value JSUtil::Convert2JSRequestDataVector(napi_env env, const std::vector<Upload::RequestData> &requestDatas)
{
    napi_value jsRequestDatas = nullptr;
    napi_create_array_with_length(env, requestDatas.size(), &jsRequestDatas);
    int index = 0;
    for (const auto &requestData : requestDatas) {
        napi_value jsRequestData = Convert2JSRequestData(env, requestData);
        napi_set_element(env, jsRequestData, index++, jsRequestDatas);
    }
    return jsRequestDatas;
}

bool JSUtil::Equals(napi_env env, napi_value value, napi_ref copy)
{
    if (copy == nullptr) {
        return (value == nullptr);
    }
    napi_value copyValue = nullptr;
    napi_get_reference_value(env, copy, &copyValue);

    bool isEquals = false;
    napi_strict_equals(env, value, copyValue, &isEquals);
    return isEquals;
}

bool JSUtil::CheckParamNumber(size_t argc, bool IsRequiredParam)
{
    if (IsRequiredParam) {
        return argc == TWO_ARG;
    }
    return (argc == ONE_ARG || argc == TWO_ARG);
}

bool JSUtil::CheckParamType(napi_env env, napi_value jsType, napi_valuetype type)
{
    napi_valuetype valueType = napi_undefined;
    napi_status status = napi_typeof(env, jsType, &valueType);
    if (status != napi_ok || valueType != type) {
        return false;
    }
    return true;
}

napi_value JSUtil::CreateBusinessError(napi_env env, const
    Download::ExceptionErrorCode &errorCode, const std::string &errorMessage)
{
    napi_value error = nullptr;
    napi_value code = nullptr;
    napi_value msg = nullptr;
    auto iter = ErrorCodeToMsg.find(errorCode);
    std::string strMsg = (iter != ErrorCodeToMsg.end() ? iter->second : "") + "   "+ errorMessage;
    NAPI_CALL(env, napi_create_string_utf8(env, strMsg.c_str(), strMsg.length(), &msg));
    NAPI_CALL(env, napi_create_uint32(env, errorCode, &code));
    NAPI_CALL(env, napi_create_error(env, nullptr, msg, &error));
    napi_set_named_property(env, error, "code", code);
    return error;
}

void JSUtil::GetMessage(const std::vector<Upload::TaskState> &taskStates, std::string &msg)
{
    for (auto &vmem : taskStates) {
        std::string strMsg;
        auto iter = ErrorCodeToMsg.find(static_cast<Download::ExceptionErrorCode>(vmem.responseCode));
        if (iter != ErrorCodeToMsg.end()) {
            strMsg = " --{" + vmem.path + " " + std::to_string(vmem.responseCode) + " " + iter->second + "}-- ";
        }
        msg += strMsg;
    }
}
} // namespace OHOS::Request::UploadNapi