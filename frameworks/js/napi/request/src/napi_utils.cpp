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

#include <fcntl.h>

#include <cstdint>
#include <cstring>
#include <fstream>
#include <initializer_list>
#include <memory>
#include <regex>

#include "constant.h"
#include "log.h"
#include "request_common.h"
#include "request_common_utils.h"
#include "request_manager.h"
#include "securec.h"

namespace OHOS::Request::NapiUtils {
static constexpr int64_t JS_NUMBER_MAX_VALUE = (1LL << 53) - 1;
static constexpr const char *NOT_SYSTEM_APP = "permission verification failed, application which is not a system "
                                              "application uses system API";

static const std::map<ExceptionErrorCode, std::string> ErrorCodeToMsg{ { E_OK, E_OK_INFO },
    { E_PERMISSION, E_PERMISSION_INFO }, { E_PARAMETER_CHECK, E_PARAMETER_CHECK_INFO },
    { E_UNSUPPORTED, E_UNSUPPORTED_INFO }, { E_FILE_IO, E_FILE_IO_INFO }, { E_FILE_PATH, E_FILE_PATH_INFO },
    { E_SERVICE_ERROR, E_SERVICE_ERROR_INFO }, { E_TASK_QUEUE, E_TASK_QUEUE_INFO }, { E_TASK_MODE, E_TASK_MODE_INFO },
    { E_TASK_NOT_FOUND, E_TASK_NOT_FOUND_INFO }, { E_TASK_STATE, E_TASK_STATE_INFO }, { E_OTHER, E_OTHER_INFO },
    { E_NOT_SYSTEM_APP, NOT_SYSTEM_APP }, { E_GROUP_NOT_FOUND, E_GROUP_NOT_FOUND_INFO } };

napi_status Convert2JSValue(napi_env env, const DownloadInfo &in, napi_value &out)
{
    napi_create_object(env, &out);
    SetStringPropertyUtf8(env, out, "description", in.description);
    SetUint32Property(env, out, "downloadedBytes", in.downloadedBytes);
    SetUint32Property(env, out, "downloadId", in.downloadId);
    SetUint32Property(env, out, "failedReason", in.failedReason);
    SetStringPropertyUtf8(env, out, "fileName", in.fileName);
    SetStringPropertyUtf8(env, out, "filePath", in.filePath);
    SetUint32Property(env, out, "pausedReason", in.pausedReason);
    SetUint32Property(env, out, "status", in.status);
    SetStringPropertyUtf8(env, out, "targetURI", in.url);
    SetStringPropertyUtf8(env, out, "downloadTitle", in.downloadTitle);
    SetInt64Property(env, out, "downloadTotalBytes", in.downloadTotalBytes);
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

napi_value Convert2JSValue(napi_env env, bool code)
{
    napi_value value = nullptr;
    if (napi_get_boolean(env, code, &value) != napi_ok) {
        return nullptr;
    }
    return value;
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
    if (code > JS_NUMBER_MAX_VALUE) {
        return nullptr;
    }
    napi_value value = nullptr;
    if (napi_create_int64(env, static_cast<int64_t>(code), &value) != napi_ok) {
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

napi_value Convert2JSValue(napi_env env, const std::vector<int32_t> &code)
{
    napi_value value = nullptr;
    napi_create_array_with_length(env, code.size(), &value);
    int index = 0;
    for (const auto &cInt : code) {
        napi_set_element(env, value, index++, Convert2JSValue(env, cInt));
    }
    return value;
}

napi_value Convert2JSValue(napi_env env, const std::vector<std::string> &ids)
{
    napi_value value = nullptr;
    napi_create_array_with_length(env, ids.size(), &value);
    int index = 0;
    for (const auto &id : ids) {
        napi_set_element(env, value, index++, Convert2JSValue(env, id));
    }
    return value;
}

napi_value Convert2JSHeadersAndBody(napi_env env, const std::map<std::string, std::string> &header,
    const std::vector<uint8_t> &bodyBytes, bool isSeparate)
{
    napi_value headers = nullptr;
    napi_create_object(env, &headers);
    for (const auto &cInt : header) {
        napi_set_named_property(env, headers, cInt.first.c_str(), Convert2JSValue(env, cInt.second));
    }
    napi_value body = nullptr;
    if (Utf8Utils::RunUtf8Validation(bodyBytes)) {
        napi_create_string_utf8(env, reinterpret_cast<const char *>(bodyBytes.data()), bodyBytes.size(), &body);
    } else {
        uint8_t *data = nullptr;
        napi_create_arraybuffer(env, bodyBytes.size(), reinterpret_cast<void **>(&data), &body);
        if (memcpy_s(data, bodyBytes.size(), bodyBytes.data(), bodyBytes.size()) != EOK) {
            if (bodyBytes.size() > 0) {
                REQUEST_HILOGW("Body data memcpy_s error");
            }
        }
    }

    if (isSeparate) {
        napi_value object = nullptr;
        napi_create_object(env, &object);
        napi_set_named_property(env, object, "headers", headers);
        napi_set_named_property(env, object, "body", body);
        return object;
    } else {
        napi_set_named_property(env, headers, "body", body);
        return headers;
    }
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
    napi_set_named_property(
        env, value, "extras", Convert2JSHeadersAndBody(env, progress.extras, progress.bodyBytes, false));
    return value;
}

napi_value Convert2JSValue(napi_env env, const std::vector<FileSpec> &files, const std::vector<FormItem> &forms)
{
    napi_value data = nullptr;
    size_t filesLen = files.size();
    size_t formsLen = forms.size();
    napi_create_array_with_length(env, filesLen + formsLen, &data);
    size_t i = 0;
    for (; i < formsLen; i++) {
        napi_value object = nullptr;
        napi_create_object(env, &object);
        napi_set_named_property(env, object, "name", Convert2JSValue(env, forms[i].name));
        napi_set_named_property(env, object, "value", Convert2JSValue(env, forms[i].value));
        napi_set_element(env, data, i, object);
    }
    for (size_t j = 0; j < filesLen; j++) {
        napi_value fileSpec = nullptr;
        napi_create_object(env, &fileSpec);
        napi_set_named_property(env, fileSpec, "path", Convert2JSValue(env, files[j].uri));
        napi_set_named_property(env, fileSpec, "mimeType", Convert2JSValue(env, files[j].type));
        napi_set_named_property(env, fileSpec, "filename", Convert2JSValue(env, files[j].filename));
        napi_value object = nullptr;
        napi_create_object(env, &object);
        napi_set_named_property(env, object, "name", Convert2JSValue(env, files[j].name));
        napi_set_named_property(env, object, "value", fileSpec);
        napi_set_element(env, data, i, object);
        i++;
    }
    return data;
}

napi_value Convert2JSValue(napi_env env, TaskInfo &taskInfo)
{
    napi_value value = nullptr;
    napi_create_object(env, &value);
    if (taskInfo.withSystem) {
        napi_set_named_property(env, value, "uid", Convert2JSValue(env, taskInfo.uid));
        napi_set_named_property(env, value, "bundle", Convert2JSValue(env, taskInfo.bundle));
        taskInfo.url = "";
        taskInfo.data = "";
        if (taskInfo.action == Action::UPLOAD) {
            taskInfo.files.clear();
            taskInfo.forms.clear();
        }
    }
    napi_set_named_property(env, value, "url", Convert2JSValue(env, taskInfo.url));
    napi_set_named_property(env, value, "saveas", Convert2JSValue(env, GetSaveas(taskInfo.files, taskInfo.action)));
    if (taskInfo.action == Action::DOWNLOAD) {
        napi_set_named_property(env, value, "data", Convert2JSValue(env, taskInfo.data));
    } else {
        napi_set_named_property(env, value, "data", Convert2JSValue(env, taskInfo.files, taskInfo.forms));
    }
    napi_set_named_property(env, value, "tid", Convert2JSValue(env, taskInfo.tid));
    napi_set_named_property(env, value, "title", Convert2JSValue(env, taskInfo.title));
    napi_set_named_property(env, value, "description", Convert2JSValue(env, taskInfo.description));
    napi_set_named_property(env, value, "action", Convert2JSValue(env, static_cast<uint32_t>(taskInfo.action)));
    napi_set_named_property(env, value, "mode", Convert2JSValue(env, static_cast<uint32_t>(taskInfo.mode)));
    napi_set_named_property(env, value, "mimeType", Convert2JSValue(env, taskInfo.mimeType));
    napi_set_named_property(env, value, "progress", Convert2JSValue(env, taskInfo.progress));
    napi_set_named_property(env, value, "gauge", Convert2JSValue(env, taskInfo.gauge));
    napi_set_named_property(env, value, "priority", Convert2JSValue(env, taskInfo.priority));
    napi_set_named_property(env, value, "ctime", Convert2JSValue(env, taskInfo.ctime));
    napi_set_named_property(env, value, "mtime", Convert2JSValue(env, taskInfo.mtime));
    napi_set_named_property(env, value, "retry", Convert2JSValue(env, taskInfo.retry));
    napi_set_named_property(env, value, "tries", Convert2JSValue(env, taskInfo.tries));
    if (taskInfo.code == Reason::REASON_OK) {
        napi_value value1 = nullptr;
        napi_get_null(env, &value1);
        napi_set_named_property(env, value, "faults", value1);
    } else {
        Faults fault = CommonUtils::GetFaultByReason(taskInfo.code);
        napi_set_named_property(env, value, "faults", Convert2JSValue(env, static_cast<uint32_t>(fault)));
    }
    napi_set_named_property(env, value, "reason", Convert2JSValue(env, CommonUtils::GetMsgByReason(taskInfo.code)));
    napi_set_named_property(env, value, "extras", Convert2JSValue(env, taskInfo.extras));
    return value;
}

napi_value Convert2JSValueConfig(napi_env env, Config &config)
{
    napi_value value = nullptr;
    napi_create_object(env, &value);
    napi_set_named_property(env, value, "action", Convert2JSValue(env, static_cast<uint32_t>(config.action)));
    napi_set_named_property(env, value, "url", Convert2JSValue(env, config.url));
    napi_set_named_property(env, value, "title", Convert2JSValue(env, config.title));
    napi_set_named_property(env, value, "description", Convert2JSValue(env, config.description));
    napi_set_named_property(env, value, "mode", Convert2JSValue(env, static_cast<uint32_t>(config.mode)));
    napi_set_named_property(env, value, "overwrite", Convert2JSValue(env, config.overwrite));
    napi_set_named_property(env, value, "method", Convert2JSValue(env, config.method));
    napi_set_named_property(env, value, "headers", Convert2JSValue(env, config.headers));
    if (config.action == Action::DOWNLOAD) {
        napi_set_named_property(env, value, "data", Convert2JSValue(env, config.data));
    } else {
        napi_set_named_property(env, value, "data", Convert2JSValue(env, config.files, config.forms));
    }
    napi_set_named_property(env, value, "saveas", Convert2JSValue(env, config.saveas));
    napi_set_named_property(env, value, "network", Convert2JSValue(env, static_cast<uint32_t>(config.network)));
    napi_set_named_property(env, value, "metered", Convert2JSValue(env, config.metered));
    napi_set_named_property(env, value, "roaming", Convert2JSValue(env, config.roaming));
    napi_set_named_property(env, value, "retry", Convert2JSValue(env, config.retry));
    napi_set_named_property(env, value, "redirect", Convert2JSValue(env, config.redirect));
    napi_set_named_property(env, value, "index", Convert2JSValue(env, config.index));
    napi_set_named_property(env, value, "begins", Convert2JSValue(env, config.begins));
    napi_set_named_property(env, value, "ends", Convert2JSValue(env, config.ends));
    napi_set_named_property(env, value, "priority", Convert2JSValue(env, config.priority));
    napi_set_named_property(env, value, "gauge", Convert2JSValue(env, config.gauge));
    napi_set_named_property(env, value, "precise", Convert2JSValue(env, config.precise));
    if (config.token != "null") {
        napi_set_named_property(env, value, "token", Convert2JSValue(env, config.token));
    }
    napi_set_named_property(env, value, "extras", Convert2JSValue(env, config.extras));
    napi_set_named_property(env, value, "multipart", Convert2JSValue(env, config.multipart));
    return value;
}

napi_value Convert2JSValue(napi_env env, const std::shared_ptr<Response> &response)
{
    napi_value value = nullptr;
    napi_create_object(env, &value);
    napi_set_named_property(env, value, "version", Convert2JSValue(env, response->version));
    napi_set_named_property(env, value, "statusCode", Convert2JSValue(env, response->statusCode));
    napi_set_named_property(env, value, "reason", Convert2JSValue(env, response->reason));
    napi_set_named_property(env, value, "headers", Convert2JSHeaders(env, response->headers));
    return value;
}

napi_value Convert2JSValue(napi_env env, const Reason reason)
{
    napi_value value = nullptr;
    napi_create_object(env, &value);

    Faults fault = CommonUtils::GetFaultByReason(reason);
    if (napi_create_uint32(env, static_cast<uint32_t>(fault), &value) != napi_ok) {
        return nullptr;
    }

    return value;
}

napi_value Convert2JSValue(napi_env env, WaitingReason reason)
{
    return Convert2JSValue(env, static_cast<uint32_t>(reason));
}

napi_value Convert2JSHeaders(napi_env env, const std::map<std::string, std::vector<std::string>> &headers)
{
    napi_value value = nullptr;
    napi_value value2 = nullptr;
    napi_value global = nullptr;
    napi_value mapConstructor = nullptr;
    napi_value mapSet = nullptr;
    const uint32_t paramNumber = 2;
    napi_value args[paramNumber] = { 0 };

    napi_status status = napi_get_global(env, &global);
    if (status != napi_ok) {
        REQUEST_HILOGE("response napi_get_global failed");
        return nullptr;
    }

    status = napi_get_named_property(env, global, "Map", &mapConstructor);
    if (status != napi_ok) {
        REQUEST_HILOGE("response map failed");
        return nullptr;
    }

    status = napi_new_instance(env, mapConstructor, 0, nullptr, &value);
    if (status != napi_ok) {
        REQUEST_HILOGE("response napi_new_instance failed");
        return nullptr;
    }

    status = napi_get_named_property(env, value, "set", &mapSet);
    if (status != napi_ok) {
        REQUEST_HILOGE("response set failed");
        return nullptr;
    }

    for (const auto &it : headers) {
        args[0] = Convert2JSValue(env, it.first);
        args[1] = Convert2JSValue(env, it.second);
        status = napi_call_function(env, value, mapSet, paramNumber, args, &value2);
        if (status != napi_ok) {
            REQUEST_HILOGE("response napi_call_function failed, %{public}d", status);
            return nullptr;
        }
    }
    return value;
}

std::string GetSaveas(const std::vector<FileSpec> &files, Action action)
{
    if (action == Action::UPLOAD) {
        return "";
    }
    if (files.empty()) {
        return "";
    }
    return files[0].uri;
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

int32_t Convert2Int32(napi_env env, napi_value value)
{
    int32_t ret = 0;
    NAPI_CALL_BASE(env, napi_get_value_int32(env, value, &ret), 0);
    return ret;
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

napi_value CreateBusinessError(
    napi_env env, ExceptionErrorCode errorCode, const std::string &errorMessage, bool withErrCode)
{
    napi_value error = nullptr;
    napi_value msg = nullptr;
    auto iter = ErrorCodeToMsg.find(errorCode);
    std::string strMsg = (iter != ErrorCodeToMsg.end() ? iter->second : "") + "   " + errorMessage;
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

void SetInt64Property(napi_env env, napi_value object, const std::string &name, int64_t value)
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

Action GetRequestAction(napi_env env, napi_value configValue)
{
    if (HasNamedProperty(env, configValue, PARAM_KEY_METHOD) || HasNamedProperty(env, configValue, PARAM_KEY_FILES)
        || HasNamedProperty(env, configValue, PARAM_KEY_DATA)) {
        return Action::UPLOAD;
    }
    return Action::DOWNLOAD;
}

std::vector<FileSpec> Convert2FileVector(napi_env env, napi_value jsFiles, const std::string &version)
{
    bool isArray = false;
    napi_is_array(env, jsFiles, &isArray);
    NAPI_ASSERT_BASE(env, isArray, "not array", {});
    uint32_t length = 0;
    napi_get_array_length(env, jsFiles, &length);
    std::vector<FileSpec> files;
    for (uint32_t i = 0; i < length; ++i) {
        napi_value jsFile = nullptr;
        napi_handle_scope scope = nullptr;
        napi_open_handle_scope(env, &scope);
        if (scope == nullptr) {
            continue;
        }
        napi_get_element(env, jsFiles, i, &jsFile);
        if (jsFile == nullptr) {
            napi_close_handle_scope(env, scope);
            continue;
        }
        FileSpec file;
        bool ret = Convert2File(env, jsFile, file);
        if (!ret) {
            napi_close_handle_scope(env, scope);
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
    std::string mimeType = Convert2String(env, type);
    // If it is empty, it need to be reset.
    if (!mimeType.empty()) {
        file.hasContentType = true;
        file.type = mimeType;
    }
    return true;
}

std::vector<FormItem> Convert2RequestDataVector(napi_env env, napi_value jsRequestDatas)
{
    bool isArray = false;
    napi_is_array(env, jsRequestDatas, &isArray);
    NAPI_ASSERT_BASE(env, isArray, "not array", {});
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
    if (path.length() > PATH_MAX || realpath(path.c_str(), resolvedPath) == nullptr
        || strncmp(resolvedPath, path.c_str(), path.length()) != 0) {
        REQUEST_HILOGE("invalid file path!");
        return false;
    }
    return true;
}

std::string SHA256(const char *str, size_t len)
{
    unsigned char hash[SHA256_DIGEST_LENGTH];
    SHA256_CTX sha256;
    SHA256_Init(&sha256);
    SHA256_Update(&sha256, str, len);
    SHA256_Final(hash, &sha256);
    std::stringstream ss;
    for (int i = 0; i < SHA256_DIGEST_LENGTH; i++) {
        // 2 means setting the width of the output.
        ss << std::hex << std::setw(2) << std::setfill('0') << static_cast<int>(hash[i]);
    }
    return ss.str();
}

void ReadBytesFromFile(const std::string &filePath, std::vector<uint8_t> &fileData)
{
    // Ensure filePath validity.
    std::ifstream inputFile(filePath.c_str(), std::ios::binary);
    if (inputFile.is_open()) {
        inputFile.seekg(0, std::ios::end);
        if (!inputFile) {
            inputFile.close();
            return;
        }
        fileData.resize(inputFile.tellg());
        inputFile.seekg(0);
        inputFile.read(reinterpret_cast<char *>(fileData.data()), fileData.size());
        inputFile.close();
    } else {
        REQUEST_HILOGW("Read bytes from file, invalid file path!");
    }
    return;
}

void RemoveFile(const std::string &filePath)
{
    auto removeFile = [filePath]() -> void {
        std::remove(filePath.c_str());
        return;
    };
    ffrt::submit(removeFile, {}, {}, ffrt::task_attr().name("Os_Request_Rm").qos(ffrt::qos_default));
}
} // namespace OHOS::Request::NapiUtils