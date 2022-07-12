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

#include <thread>
#include <cstdio>
#include "upload_task.h"
#include "file_adapter.h"

using namespace OHOS::AppExecFwk;
namespace OHOS::Request::Upload {
ObtainFile::ObtainFile()
{
    fileAdapter_ = std::make_shared<FileAdapter>();
}
ObtainFile::~ObtainFile()
{
}

uint32_t ObtainFile::GetFile(FILE **file, std::string &fileUri,
    unsigned int& fileSize, std::shared_ptr<OHOS::AbilityRuntime::Context> &context)
{
    uint32_t ret = UPLOAD_ERRORCODE_NO_ERROR;
    std::string dataAbilityHead("dataability");
    std::string internalHead("internal");

    // file type check
    if (fileUri.compare(0, dataAbilityHead.size(), dataAbilityHead) == 0) {
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "GetDataAbilityFile");
        ret = GetDataAbilityFile(file, fileUri, fileSize, context);
    } else if (fileUri.compare(0, internalHead.size(), internalHead) == 0) {
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "GetInternalFile");
        ret = GetInternalFile(file, fileUri, fileSize, context);
    } else {
        UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "wrong path");
        ret = UPLOAD_ERRORCODE_UNSUPPORT_URI;
        *file = nullptr;
        fileSize = 0;
    }

    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK,
        "ObtainFile::GetFile, ret : %{public}d, size : %{public}d, pf : %{public}p", ret, fileSize, *file);
    return ret;
}

uint32_t ObtainFile::GetDataAbilityFile(FILE **file, std::string &fileUri,
    uint32_t& fileSize, std::shared_ptr<OHOS::AbilityRuntime::Context> &context)
{
    uint32_t ret = UPLOAD_ERRORCODE_NO_ERROR;
    FILE *filePtr = nullptr;
    int32_t fileLength = 0;

    do {
        int32_t fd = fileAdapter_->DataAbilityOpenFile(fileUri, context);
        if (fd == -1) {
            UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "ObtainFile::GetDataAbilityFile, open file error.");
            ret = UPLOAD_ERRORCODE_GET_FILE_ERROR;
            break;
        }

        filePtr = fdopen(fd, "r");
        if (filePtr == nullptr) {
            UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "ObtainFile::GetDataAbilityFile, fdopen error.");
            ret = UPLOAD_ERRORCODE_GET_FILE_ERROR;
            break;
        }

        (void)fseek(filePtr, 0, SEEK_END);
        fileLength = ftell(filePtr);
        if (fileLength == -1) {
            UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "ObtainFile::GetDataAbilityFile, ftell error.");
            ret = UPLOAD_ERRORCODE_GET_FILE_ERROR;
            break;
        }
        (void)fseek(filePtr, 0, SEEK_SET);
    } while (0);

    *file = filePtr;
    fileSize = static_cast<uint32_t>fileLength;
    return ret;
}

uint32_t ObtainFile::GetInternalFile(FILE **file, std::string &fileUri,
    uint32_t& fileSize, std::shared_ptr<OHOS::AbilityRuntime::Context> &context)
{
    uint32_t ret = UPLOAD_ERRORCODE_NO_ERROR;
    std::string filePath;
    FILE *filePtr = nullptr;
    int32_t fileLength = 0;

    do {
        std::vector<std::string> uriSplit;
        std::string pattern = "/";
        std::string pathTmp = fileUri + pattern;
        size_t pos = pathTmp.find(pattern);
        while (pos != pathTmp.npos) {
            std::string temp = pathTmp.substr(0, pos);
            uriSplit.push_back(temp);
            pathTmp = pathTmp.substr(pos + 1, pathTmp.size());
            pos = pathTmp.find(pattern);
        }
        if (uriSplit[SPLIT_ZERO] != "internal:" || uriSplit[SPLIT_ONE] != "" ||
            uriSplit[SPLIT_TWO] != "cache" || uriSplit.size() <= SPLIT_THREE) {
            UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "ObtainFile::GetInternalFile, internal path woring");
            ret = UPLOAD_ERRORCODE_UNSUPPORT_URI;
            break;
        }
        filePath = fileAdapter_->InternalGetFilePath(context);
        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "ObtainFile::GetInternalFile, cache dir = [%{public}s].",
            filePath.c_str());
        if (filePath.size() == 0) {
            UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "ObtainFile::GetInternalFile, internal to cache error");
            ret = UPLOAD_ERRORCODE_GET_FILE_ERROR;
            break;
        }
        for (size_t i = SPLIT_THREE; i < uriSplit.size(); ++i) {
            filePath = filePath + "/" + uriSplit[i];
        }

        UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "ObtainFile::GetInternalFile, internal file path = [%{public}s].",
            filePath.c_str());
        filePtr = fopen(filePath.c_str(), "r");
        if (filePtr == nullptr) {
            UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "ObtainFile::GetInternalFile, open file error");
            UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "ObtainFile::GetInternalFile, error info : %{public}d.", errno);
            ret = UPLOAD_ERRORCODE_GET_FILE_ERROR;
            break;
        }
        (void)fseek(filePtr, 0, SEEK_END);
        fileLength = ftell(filePtr);
        (void)fseek(filePtr, 0, SEEK_SET);
    } while (0);

    *file = filePtr;
    fileSize = fileLength;
    return ret;
}
} // end of OHOS::Request::Upload