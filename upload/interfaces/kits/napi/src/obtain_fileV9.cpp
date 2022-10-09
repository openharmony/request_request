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
#include "constant.h"
#include "upload_common.h"
#include "file_adapterV9.h"
#include "upload_hilog_wrapper.h"
#include "obtain_fileV9.h"

using namespace OHOS::AppExecFwk;
namespace OHOS::Request::Upload {
ObtainFileV9::ObtainFileV9()
{
    fileAdapter_ = std::make_shared<FileAdapterV9>();
}
ObtainFileV9::~ObtainFileV9()
{
}

uint32_t ObtainFileV9::GetFile(FILE **file, std::string &fileUri,
    unsigned int& fileSize, std::shared_ptr<OHOS::AbilityRuntime::Context> &context)
{
    uint32_t ret = UPLOAD_OK;
    std::string dataAbilityHead("dataability");
    std::string internalHead("internal");

    // file type check
    if (fileUri.compare(0, dataAbilityHead.size(), dataAbilityHead) == 0) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "GetDataAbilityFile");
        ret = GetDataAbilityFile(file, fileUri, fileSize, context);
    } else if (fileUri.compare(0, internalHead.size(), internalHead) == 0) {
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "GetInternalFile");
        ret = GetInternalFile(file, fileUri, fileSize, context);
    } else {
        UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "wrong path");
        ret = Download::EXCEPTION_FILE_PATH;
        *file = nullptr;
        fileSize = 0;
    }

    UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI,
        "ObtainFileV9::GetFile, ret : %{public}d, size : %{public}d, pf : %{public}p", ret, fileSize, *file);
    return ret;
}

uint32_t ObtainFileV9::GetDataAbilityFile(FILE **file, std::string &fileUri,
    uint32_t& fileSize, std::shared_ptr<OHOS::AbilityRuntime::Context> &context)
{
    uint32_t ret = UPLOAD_OK;
    FILE *filePtr = nullptr;
    int32_t fileLength = 0;

    do {
        int32_t fd = fileAdapter_->DataAbilityOpenFile(fileUri, context);
        if (fd < 0) {
            UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "ObtainFileV9::GetDataAbilityFile, open file error.");
            ret = Download::EXCEPTION_FILE_IO;
            break;
        }

        filePtr = fdopen(fd, "r");
        if (filePtr == nullptr) {
            UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "ObtainFileV9::GetDataAbilityFile, fdopen error.");
            ret = Download::EXCEPTION_FILE_IO;
            break;
        }

        (void)fseek(filePtr, 0, SEEK_END);
        fileLength = ftell(filePtr);
        if (fileLength == -1) {
            UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "ObtainFileV9::GetDataAbilityFile, ftell error.");
            ret = Download::EXCEPTION_FILE_SIZE;
            break;
        }
        (void)fseek(filePtr, 0, SEEK_SET);
    } while (0);

    *file = filePtr;
    fileSize = static_cast<uint32_t>(fileLength);
    return ret;
}

uint32_t ObtainFileV9::GetInternalFile(FILE **file, std::string &fileUri,
    uint32_t& fileSize, std::shared_ptr<OHOS::AbilityRuntime::Context> &context)
{
    uint32_t ret = UPLOAD_OK;
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
            UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "ObtainFileV9::GetInternalFile, internal path woring");
            ret = Download::EXCEPTION_FILE_PATH;
            break;
        }
        filePath = fileAdapter_->InternalGetFilePath(context);
        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "ObtainFileV9::GetInternalFile, cache dir = [%{public}s].",
            filePath.c_str());
        if (filePath.size() == 0) {
            UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "ObtainFileV9::GetInternalFile, internal to cache error");
            ret = Download::EXCEPTION_FILE_SIZE;
            break;
        }
        for (size_t i = SPLIT_THREE; i < uriSplit.size(); ++i) {
            filePath = filePath + "/" + uriSplit[i];
        }

        UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "ObtainFileV9::GetInternalFile, internal file path = [%{public}s].",
            filePath.c_str());
        filePtr = fopen(filePath.c_str(), "r");
        if (filePtr == nullptr) {
            UPLOAD_HILOGE(UPLOAD_MODULE_JS_NAPI, "ObtainFileV9::GetInternalFile, open file error");
            UPLOAD_HILOGD(UPLOAD_MODULE_JS_NAPI, "ObtainFileV9::GetInternalFile, error info : %{public}d.", errno);
            ret = Download::EXCEPTION_FILE_IO;
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