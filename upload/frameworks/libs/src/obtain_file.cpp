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
    uint32_t ret = UPLOAD_OK;
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

    UPLOAD_HILOGD(UPLOAD_MODULE_FRAMEWORK, "get file ret : %{public}d, size : %{public}u", ret, fileSize);
    return ret;
}

uint32_t ObtainFile::GetDataAbilityFile(FILE **file, std::string &fileUri,
    uint32_t& fileSize, std::shared_ptr<OHOS::AbilityRuntime::Context> &context)
{
    uint32_t ret = UPLOAD_OK;
    FILE *filePtr = nullptr;
    int32_t fileLength = 0;

    do {
        int32_t fd = fileAdapter_->DataAbilityOpenFile(fileUri, context);
        if (fd < 0) {
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
    fileSize = static_cast<uint32_t>(fileLength);
    return ret;
}

bool ObtainFile::IsValidPath(const std::string &filePath)
{
    char resolvedPath[PATH_MAX + 1] = { 0 };
    if (filePath.length() > PATH_MAX || realpath(filePath.c_str(), resolvedPath) == nullptr ||
        strncmp(resolvedPath, filePath.c_str(), filePath.length()) != 0) {
        UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "filePath error");
        return false;
    }
    return true;
}

bool ObtainFile::SplitPath(const std::string &fileUri, std::string &fileName)
{
    std::string pattern = "internal://cache/";
    size_t pos = fileUri.find(pattern);
    if (pos != 0) {
        UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "internal path is invalid");
        return false;
    }
    fileName = fileUri.substr(pattern.size(), fileUri.size());
    return true;
}

uint32_t ObtainFile::GetInternalFile(FILE **file, const std::string &fileUri, uint32_t &fileSize,
    std::shared_ptr<OHOS::AbilityRuntime::Context> &context)
{
    std::string fileName;
    if (!SplitPath(fileUri, fileName)) {
        return UPLOAD_ERRORCODE_UNSUPPORT_URI;
    }
    std::string filePath = fileAdapter_->InternalGetFilePath(context);
    if (filePath.empty()) {
        UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "ObtainFile::GetInternalFile, internal to cache error");
        return UPLOAD_ERRORCODE_GET_FILE_ERROR;
    }
    filePath += "/" + fileName;
    if (!IsValidPath(filePath)) {
        return UPLOAD_ERRORCODE_GET_FILE_ERROR;
    }
    FILE *filePtr = fopen(filePath.c_str(), "r");
    if (filePtr == nullptr) {
        UPLOAD_HILOGE(UPLOAD_MODULE_FRAMEWORK, "open file error, error info : %{public}d.", errno);
        return UPLOAD_ERRORCODE_GET_FILE_ERROR;
    }
    (void)fseek(filePtr, 0, SEEK_END);
    int32_t fileLength = ftell(filePtr);
    (void)fseek(filePtr, 0, SEEK_SET);

    *file = filePtr;
    fileSize = fileLength;
    return UPLOAD_OK;
}
} // end of OHOS::Request::Upload