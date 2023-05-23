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

#include "napi_utils.h"

#include <cstring>
#include <initializer_list>
#include <memory>
#include <regex>
#include <fcntl.h>

#include "request_manager.h"
#include "log.h"
#include "securec.h"

namespace OHOS::Request::NapiUtils {
static constexpr const int MAX_STRING_LENGTH = 65536;
static const std::map<ExceptionErrorCode, std::string> ErrorCodeToMsg {
    {E_OK, E_OK_INFO },
    {E_PERMISSION, E_PERMISSION_INFO },
    {E_PARAMETER_CHECK, E_PARAMETER_CHECK_INFO },
    {E_UNSUPPORTED, E_UNSUPPORTED_INFO },
    {E_FILE_IO, E_FILE_IO_INFO },
    {E_FILE_PATH, E_FILE_PATH_INFO },
    {E_SERVICE_ERROR, E_SERVICE_ERROR_INFO },
    {E_TASK_QUEUE, E_TASK_QUEUE_INFO },
    {E_TASK_MODE, E_TASK_MODE_INFO },
    {E_TASK_NOT_FOUND, E_TASK_NOT_FOUND_INFO },
    {E_TASK_STATE, E_TASK_STATE_INFO },
    {E_OTHER, E_OTHER_INFO },
};

napi_status Convert2JSValue(napi_env env, DownloadInfo &in, napi_value &out)
{
    napi_create_object(env, &out);
    SetStringPropertyUtf8(env, out, "description", in.description.c_str());
    SetUint32Property(env, out, "downloadedBytes", in.downloadedBytes);
    SetUint32Property(env, out, "downloadId", in.downloadId);
    SetUint32Property(env, out, "failedReason", in.failedReason);
    SetStringPropertyUtf8(env, out, "fileName", in.fileName.c_str());
    SetStringPropertyUtf8(env, out, "filePath", in.filePath.c_str());
    SetUint32Property(env, out, "pausedReason", in.pausedReason);
    SetUint32Property(env, out, "status", in.status);
    SetStringPropertyUtf8(env, out, "targetURI", in.url.c_str());
    SetStringPropertyUtf8(env, out, "downloadTitle", in.downloadTitle.c_str());
    SetUint32Property(env, out, "downloadTotalBytes", in.downloadTotalBytes);
    return napi_ok;
}

napi_status Convert2JSValue(napi_env env, std::string &in, napi_value &out)
{
    return napi_create_string_utf8(env, in.c_str(), strlen(in.c_str()), &out);
}

napi_status Convert2JSValue(napi_env env, bool in, napi_value &out)
{
    return napi_get_boolean(env, in, &out);
}

napi_value Convert2JSValue(napi_env env, int32_t code)
{
    napi_value value = nullptr;
    if (napi_create_int32(env, code, &value) != napi_ok) {
        return nullptr;
    }
    return value;
}

napi_value Convert2JSValue(napi_env env, uint32_t code)
{
    napi_value value = nullptr;
    if (napi_create_uint32(env, code, &value) != napi_ok) {
        return nullptr;
    }
    return value;
}

napi_value Convert2JSValue(napi_env env, int64_t code)
{
    napi_value value = nullptr;
    if (napi_create_int64(env, code, &value) != napi_ok) {
        return nullptr;
    }
    return value;
}

napi_value Convert2JSValue(napi_env env, uint64_t code)
{
    napi_value value = nullptr;
    if (napi_create_bigint_uint64(env, code, &value) != napi_ok) {
        return nullptr;
    }
    return value;
}


napi_value Convert2JSValue(napi_env env, const std::vector<int64_t> &code)
{
    napi_value value = nullptr;
    napi_create_array_with_length(env, code.size(), &value);
    int index = 0;
    for (const auto &cInt : code) {
        napi_value jsInt = Convert2JSValue(env, cInt);
        napi_set_element(env, value, index++, jsInt);
    }
    return value;
}

napi_value Convert2JSHeaders(napi_env env, const std::map<std::string, std::string> &header)
{
    napi_value headers = nullptr;
    napi_create_object(env, &headers);
    napi_value body = nullptr;
    for (const auto &cInt : header) {
        if (cInt.first == "body") {
            body = Convert2JSValue(env, cInt.second);
        } else {
            napi_set_named_property(env, headers, cInt.first.c_str(), Convert2JSValue(env, cInt.second));
        }
    }
    napi_value object = nullptr;
    napi_create_object(env, &object);
    napi_set_named_property(env, object, "headers", headers);
    napi_set_named_property(env, object, "body", body);
    return object;
}

napi_value Convert2JSValue(napi_env env, const std::map<std::string, std::string> &code)
{
    napi_value object = nullptr;
    napi_create_object(env, &object);
    for (const auto &cInt : code) {
        napi_set_named_property(env, object, cInt.first.c_str(), Convert2JSValue(env, cInt.second));
    }
    return object;
}

napi_value Convert2JSValue(napi_env env, const std::string &str)
{
    napi_value value = nullptr;
    if (napi_create_string_utf8(env, str.c_str(), strlen(str.c_str()), &value) != napi_ok) {
        return nullptr;
    }
    return value;
}

napi_value Convert2JSValue(napi_env env, const std::vector<TaskState> &taskStates)
{
    napi_value value = nullptr;
    napi_create_array_with_length(env, taskStates.size(), &value);
    int index = 0;
    for (const auto &taskState : taskStates) {
        napi_value jsTaskState = nullptr;
        napi_create_object(env, &jsTaskState);
        napi_set_named_property(env, jsTaskState, "path", Convert2JSValue(env, taskState.path));
        napi_set_named_property(env, jsTaskState, "responseCode", Convert2JSValue(env, taskState.responseCode));
        napi_set_named_property(env, jsTaskState, "message", Convert2JSValue(env, taskState.message));
        napi_set_element(env, value, index++, jsTaskState);
    }
    return value;
}

napi_value Convert2JSValue(napi_env env, const Progress &progress)
{
    napi_value value = nullptr;
    napi_create_object(env, &value);
    napi_set_named_property(env, value, "state", Convert2JSValue(env, static_cast<uint32_t>(progress.state)));
    napi_set_named_property(env, value, "index", Convert2JSValue(env, progress.index));
    napi_set_named_property(env, value, "processed", Convert2JSValue(env, progress.processed));
    napi_set_named_property(env, value, "sizes", Convert2JSValue(env, progress.sizes));
    napi_set_named_property(env, value, "extras", Convert2JSValue(env, progress.extras));
    return value;
}

bool Convert2Boolean(napi_env env, napi_value object, const std::string &propertyName)
{
    if (!HasNamedProperty(env, object, propertyName)) {
        return false;
    }
    napi_value value = GetNamedProperty(env, object, propertyName);
    if (GetValueType(env, value) != napi_boolean) {
        return false;
    }
    bool ret = false;
    NAPI_CALL_BASE(env, napi_get_value_bool(env, value, &ret), false);
    return ret;
}

uint32_t Convert2Uint32(napi_env env, napi_value value)
{
    uint32_t ret = 0;
    NAPI_CALL_BASE(env, napi_get_value_uint32(env, value, &ret), 0);
    return ret;
}

uint32_t Convert2Uint32(napi_env env, napi_value object, const std::string &propertyName)
{
    if (!HasNamedProperty(env, object, propertyName)) {
        return 0;
    }
    napi_value value = GetNamedProperty(env, object, propertyName);
    if (GetValueType(env, value) != napi_number) {
        return 0;
    }
    return Convert2Uint32(env, value);
}

int64_t Convert2Int64(napi_env env, napi_value value)
{
    int64_t ret = 0;
    NAPI_CALL_BASE(env, napi_get_value_int64(env, value, &ret), 0);
    return ret;
}

int64_t Convert2Int64(napi_env env, napi_value object, const std::string &propertyName)
{
    if (!HasNamedProperty(env, object, propertyName)) {
        return 0;
    }
    napi_value value = GetNamedProperty(env, object, propertyName);
    if (GetValueType(env, value) != napi_number) {
        return 0;
    }
    return Convert2Int64(env, value);
}

std::string Convert2String(napi_env env, napi_value value)
{
    std::string result;
    std::vector<char> str(MAX_STRING_LENGTH + 1, '\0');
    size_t length = 0;
    NAPI_CALL_BASE(env, napi_get_value_string_utf8(env, value, &str[0], MAX_STRING_LENGTH, &length), result);
    if (length > 0) {
        return result.append(&str[0], length);
    }
    return result;
}

std::string Convert2String(napi_env env, napi_value object, const std::string &propertyName)
{
    if (!HasNamedProperty(env, object, propertyName)) {
        return "";
    }
    napi_value value = GetNamedProperty(env, object, propertyName);
    if (GetValueType(env, value) != napi_string) {
        return "";
    }
    return Convert2String(env, value);
}

void ThrowError(napi_env env, ExceptionErrorCode code, const std::string &msg, bool withErrCode)
{
    if (code == E_UNLOADING_SA) {
        code = E_SERVICE_ERROR;
    }
    napi_value error = CreateBusinessError(env, code, msg, withErrCode);
    napi_throw(env, error);
}

void ConvertError(int32_t errorCode, ExceptionError &err)
{
    auto generateError = [&err](ExceptionErrorCode errorCode, const std::string &info) {
        err.code = errorCode;
        err.errInfo = info;
        REQUEST_HILOGE("errorCode: %{public}d, errInfo: %{public}s", err.code, err.errInfo.c_str());
    };

    switch (errorCode) {
        case E_UNLOADING_SA:
            generateError(E_SERVICE_ERROR, "Service ability is quitting.");
            break;
        case E_IPC_SIZE_TOO_LARGE:
            generateError(E_SERVICE_ERROR, "Ipc error.");
            break;
        case E_MIMETYPE_NOT_FOUND:
            generateError(E_OTHER, "Mimetype not found.");
            break;
        case E_TASK_INDEX_TOO_LARGE:
            generateError(E_TASK_NOT_FOUND, "Task index out of range.");
            break;
        default:
            generateError(static_cast<ExceptionErrorCode>(errorCode), "");
            break;
    }
}

napi_value CreateBusinessError(napi_env env, ExceptionErrorCode errorCode,
    const std::string &errorMessage, bool withErrCode)
{
    napi_value error = nullptr;
    napi_value msg = nullptr;
    auto iter = ErrorCodeToMsg.find(errorCode);
    std::string strMsg = (iter != ErrorCodeToMsg.end() ? iter->second : "") + "   "+ errorMessage;
    NAPI_CALL(env, napi_create_string_utf8(env, strMsg.c_str(), strMsg.length(), &msg));
    NAPI_CALL(env, napi_create_error(env, nullptr, msg, &error));
    if (!withErrCode) {
        return error;
    }
    napi_value code = nullptr;
    NAPI_CALL(env, napi_create_uint32(env, static_cast<uint32_t>(errorCode), &code));
    napi_set_named_property(env, error, "code", code);
    return error;
}

napi_valuetype GetValueType(napi_env env, napi_value value)
{
    if (value == nullptr) {
        return napi_undefined;
    }

    napi_valuetype valueType = napi_undefined;
    NAPI_CALL_BASE(env, napi_typeof(env, value, &valueType), napi_undefined);
    return valueType;
}

bool HasNamedProperty(napi_env env, napi_value object, const std::string &propertyName)
{
    bool hasProperty = false;
    NAPI_CALL_BASE(env, napi_has_named_property(env, object, propertyName.c_str(), &hasProperty), false);
    return hasProperty;
}

napi_value GetNamedProperty(napi_env env, napi_value object, const std::string &propertyName)
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

std::vector<std::string> GetPropertyNames(napi_env env, napi_value object)
{
    std::vector<std::string> ret;
    napi_value names = nullptr;
    NAPI_CALL_BASE(env, napi_get_property_names(env, object, &names), ret);
    uint32_t length = 0;
    NAPI_CALL_BASE(env, napi_get_array_length(env, names, &length), ret);
    for (uint32_t index = 0; index < length; ++index) {
        napi_value name = nullptr;
        if (napi_get_element(env, names, index, &name) != napi_ok) {
            continue;
        }
        if (GetValueType(env, name) != napi_string) {
            continue;
        }
        ret.emplace_back(Convert2String(env, name));
    }
    return ret;
}


void SetUint32Property(napi_env env, napi_value object, const std::string &name, uint32_t value)
{
    napi_value jsValue = Convert2JSValue(env, value);
    if (GetValueType(env, jsValue) != napi_number) {
        return;
    }

    napi_set_named_property(env, object, name.c_str(), jsValue);
}

void SetStringPropertyUtf8(napi_env env, napi_value object, const std::string &name, const std::string &value)
{
    napi_value jsValue = Convert2JSValue(env, value);
    if (GetValueType(env, jsValue) != napi_string) {
        return;
    }
    napi_set_named_property(env, object, name.c_str(), jsValue);
}

napi_value CreateObject(napi_env env)
{
    napi_value object = nullptr;
    NAPI_CALL(env, napi_create_object(env, &object));
    return object;
}

napi_value GetUndefined(napi_env env)
{
    napi_value undefined = nullptr;
    NAPI_CALL(env, napi_get_undefined(env, &undefined));
    return undefined;
}

napi_value CallFunction(napi_env env, napi_value recv, napi_value func, size_t argc, const napi_value *argv)
{
    napi_value res = nullptr;
    NAPI_CALL(env, napi_call_function(env, recv, func, argc, argv, &res));
    return res;
}


std::string ToLower(const std::string &s)
{
    std::string res = s;
    std::transform(res.begin(), res.end(), res.begin(), tolower);
    return res;
}

int32_t GetParameterNumber(napi_env env, napi_callback_info info, napi_value *argv, napi_value *this_arg)
{
    size_t argc = MAX_ARGC;
    void *data = nullptr;
    napi_status status = napi_get_cb_info(env, info, &argc, argv, this_arg, &data);
    if (status != napi_ok) {
        return -1;
    }
    return static_cast<int32_t>(argc);
}


bool CheckParameterCorrect(napi_env env, napi_callback_info info, const std::string &type, ExceptionError &err)
{
    napi_value argv[MAX_ARGC] = { nullptr };
    int32_t num = GetParameterNumber(env, info, argv, nullptr);
    if (num < 0) {
        err = {.code = E_PARAMETER_CHECK, .errInfo = "function ${" + type + "} Wrong number of arguments"};
        return false;
    }
    if (num == ONE_ARG && GetValueType(env, argv[FIRST_ARGV]) != napi_function) {
        err = {.code = E_PARAMETER_CHECK, .errInfo = "function ${" + type + "} the first parameter must be function"};
        return false;
    }
    return true;
}

Action GetRequestAction(napi_env env, napi_value configValue)
{
    if (HasNamedProperty(env, configValue, PARAM_KEY_METHOD) || HasNamedProperty(env, configValue, PARAM_KEY_FILES) ||
        HasNamedProperty(env, configValue, PARAM_KEY_DATA)) {
        return Action::UPLOAD;
    }
    return Action::DOWNLOAD;
}


std::vector<FileSpec> Convert2FileVector(napi_env env, napi_value jsFiles, const std::string &version)
{
    bool isArray = false;
    napi_is_array(env, jsFiles, &isArray);
    NAPI_ASSERT_BASE(env, isArray, "not array", { });
    uint32_t length = 0;
    napi_get_array_length(env, jsFiles, &length);
    std::vector<FileSpec> files;
    for (uint32_t i = 0; i < length; ++i) {
        napi_value jsFile = nullptr;
        napi_handle_scope scope = nullptr;
        napi_open_handle_scope(env, &scope);
        napi_get_element(env, jsFiles, i, &jsFile);
        if (jsFile == nullptr) {
            continue;
        }
        FileSpec file;
        bool ret = Convert2File(env, jsFile, file);
        if (!ret) {
            continue;
        }
        files.push_back(file);
        napi_close_handle_scope(env, scope);
    }
    return files;
}

bool Convert2File(napi_env env, napi_value jsFile, FileSpec &file)
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

std::vector<FormItem> Convert2RequestDataVector(napi_env env, napi_value jsRequestDatas)
{
    bool isArray = false;
    napi_is_array(env, jsRequestDatas, &isArray);
    NAPI_ASSERT_BASE(env, isArray, "not array", { });
    uint32_t length = 0;
    napi_get_array_length(env, jsRequestDatas, &length);
    std::vector<FormItem> requestDatas;
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

FormItem Convert2RequestData(napi_env env, napi_value jsRequestData)
{
    FormItem requestData;
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

bool IsPathValid(const std::string &filePath)
{
    auto path = filePath.substr(0, filePath.rfind('/'));
    char resolvedPath[PATH_MAX + 1] = { 0 };
    if (path.length() > PATH_MAX || realpath(path.c_str(), resolvedPath) == nullptr ||
        strncmp(resolvedPath, path.c_str(), path.length()) != 0) {
        REQUEST_HILOGE("invalid file path!");
        return false;
    }
    return true;
}
} // namespace OHOS::Request::NapiUtils