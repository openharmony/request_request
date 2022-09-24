/*
 * Copyright (c) 2021 Huawei Device Co., Ltd.
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

#include "uri_utils.h"
#include "constant.h"
#include "log.h"

namespace OHOS::Request::Download {
UriUtils::UriUtils(void)
{
}

UriUtils::~UriUtils()
{
}

std::map<std::string, std::string> UriUtils::getQueryParameter(OHOS::Uri &uri)
{
    std::map<std::string, std::string> mapQuery;
    std::string query = uri.GetQuery();
    if (query.empty()) {
        return mapQuery;
    }
    std::vector<std::string> tempVector = split(query, "&");
    size_t tempSize = tempVector.size();
    for (size_t i = 0; i < tempSize; i++) {
        std::vector<std::string> childTempVector = split(tempVector[i], "=");
        size_t size = childTempVector.size();
        if (size != REQUEST_PARAMS_NUM) {
            DOWNLOAD_HILOGE("UriUtils uriParse getQueryParameter query parameter error");
            break;
        }
        std::string key = childTempVector[0];
        std::string value = childTempVector[1];
        mapQuery.insert(std::make_pair(key, value));
    }
    return mapQuery;
}

std::vector<std::string> UriUtils::split(const std::string &str, const std::string &split)
{
    char *saveChar = nullptr;
    char *token = strtok_r(const_cast<char *>(str.c_str()), split.c_str(), &saveChar);
    std::vector<std::string> result;
    while (token != nullptr) {
        result.emplace_back(token);
        token = strtok_r(nullptr, split.c_str(), &saveChar);
    }
    return result;
}

int UriUtils::UriParse(OHOS::Uri &uri, std::map<std::string, int> &keyMap)
{
    std::string path = uri.GetPath();
    if (path.empty()) {
        return OPERATION_ERROR;
    }
    std::map<std::string, int>::iterator iterator = keyMap.find(path);
    if (iterator != keyMap.end()) {
        return iterator->second;
    }
    return OPERATION_ERROR;
}
} // namespace OHOS::Request::Download