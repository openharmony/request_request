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

#include "js_util.h"
#include <securec.h>

using namespace OHOS::Request::Upload;
namespace OHOS::Request::UploadNapi {
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

std::shared_ptr<Upload::UploadConfig> JSUtil::Convert2UploadConfig(napi_env env, napi_value jsConfig)
{
    Upload::UploadConfig config;
    napi_value value = nullptr;
    napi_get_named_property(env, jsConfig, "url", &value);
    if (value != nullptr) {
        config.url = Convert2String(env, value);
    }
    value = nullptr;
    napi_get_named_property(env, jsConfig, "header", &value);
    if (value != nullptr) {
        config.header = Convert2String(env, value);
    }
    value = nullptr;
    napi_get_named_property(env, jsConfig, "method", &value);
    if (value != nullptr) {
        config.method = Convert2String(env, value);
    }
    value = nullptr;
    napi_get_named_property(env, jsConfig, "files", &value);
    if (value != nullptr) {
        config.files = Convert2FileVector(env, value);
    }
    value = nullptr;
    napi_get_named_property(env, jsConfig, "data", &value);
    if (value != nullptr) {
        config.data = Convert2RequestDataVector(env, value);
    }

    std::shared_ptr<Upload::UploadConfig> tmpConfig = std::make_shared<Upload::UploadConfig>(config);
    return tmpConfig;
}

napi_value JSUtil::Convert2JSUploadConfig(napi_env env, const Upload::UploadConfig &config)
{
    napi_value jsConfig = nullptr;
    napi_create_object(env, &jsConfig);
    napi_set_named_property(env, jsConfig, "url", Convert2JSString(env, config.url));
    napi_set_named_property(env, jsConfig, "header", Convert2JSString(env, config.header));
    napi_set_named_property(env, jsConfig, "method", Convert2JSString(env, config.method));
    napi_set_named_property(env, jsConfig, "files", Convert2JSFileVector(env, config.files));
    napi_set_named_property(env, jsConfig, "data", Convert2JSRequestDataVector(env, config.data));
    return jsConfig;
}

Upload::File JSUtil::Convert2File(napi_env env, napi_value jsFile)
{
    Upload::File file;
    napi_value value = nullptr;
    napi_get_named_property(env, jsFile, "filename", &value);
    if (value != nullptr) {
        file.filename = Convert2String(env, value);
    }
    value = nullptr;
    napi_get_named_property(env, jsFile, "name", &value);
    if (value != nullptr) {
        file.name = Convert2String(env, value);
    }
    value = nullptr;
    napi_get_named_property(env, jsFile, "uri", &value);
    if (value != nullptr) {
        file.uri = Convert2String(env, value);
    }
    value = nullptr;
    napi_get_named_property(env, jsFile, "type", &value);
    if (value != nullptr) {
        file.type = Convert2String(env, value);
    }
    return file;
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

std::vector<Upload::File> JSUtil::Convert2FileVector(napi_env env, napi_value jsFiles)
{
    bool isArray = false;
    napi_is_array(env, jsFiles, &isArray);
    NAPI_ASSERT_BASE(env, isArray, "not array", { });
    uint32_t length = 0;
    napi_get_array_length(env, jsFiles, &length);
    std::vector<Upload::File> files;
    for (uint32_t i = 0; i < length; ++i) {
        napi_value file = nullptr;
        napi_get_element(env, jsFiles, i, &file);
        if (file == nullptr) {
            continue;
        }
        files.push_back(Convert2File(env, file));
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
} // namespace OHOS::Request::UploadNapi