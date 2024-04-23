/*
 * Copyright (c) 2024 Huawei Device Co., Ltd.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "cj_request_common.h"

#include <cstdlib>
#include <sstream>
#include <fstream>
#include "ffrt.h"
#include "cj_request_log.h"
#include "securec.h"
#include "openssl/sha.h"

namespace OHOS::CJSystemapi::Request {
using OHOS::Request::ExceptionErrorCode;

void ReadBytesFromFile(const std::string &filePath, std::vector<uint8_t> &fileData)
{
    // Ensure filePath validity.
    std::ifstream inputFile(filePath.c_str(), std::ios::binary);
    if (inputFile.is_open()) {
        inputFile.seekg(0, std::ios::end);
        fileData.resize(inputFile.tellg());
        inputFile.seekg(0);
        inputFile.read(reinterpret_cast<char *>(fileData.data()), fileData.size());
        inputFile.close();
    } else {
        REQUEST_HILOGW("Read bytes from file, invalid file path!");
    }
    return;
}

char* MallocCString(const std::string& origin)
{
    if (origin.empty()) {
        return nullptr;
    }
    auto len = origin.length() + 1;
    char* res = (char*)malloc(sizeof(char) * len);
    if (res == nullptr) {
        return nullptr;
    }
    return std::char_traits<char>::copy(res, origin.c_str(), len);
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

std::string SHA256(const char *str, size_t len)
{
    unsigned char hash[SHA256_DIGEST_LENGTH];
    SHA256_CTX sha256;
    SHA256_Init(&sha256);
    SHA256_Update(&sha256, str, len);
    SHA256_Final(hash, &sha256);
    std::stringstream ss;
    for (int i = 0; i < SHA256_DIGEST_LENGTH; i++) {
        // 2 means setting hte width of the output.
        ss << std::hex << std::setw(2) << std::setfill('0') << static_cast<int>(hash[i]);
    }
    return ss.str();
}

ExceptionError ConvertError(int32_t errorCode)
{
    ExceptionError err{};
    auto generateError = [&err](ExceptionErrorCode errorCode, const std::string &info) {
        err.code = errorCode;
        err.errInfo = info;
        REQUEST_HILOGE("errorCode: %{public}d, errInfo: %{public}s", err.code, err.errInfo.c_str());
    };

    switch (errorCode) {
        case ExceptionErrorCode::E_UNLOADING_SA:
            generateError(ExceptionErrorCode::E_SERVICE_ERROR, "Service ability is quitting.");
            break;
        case ExceptionErrorCode::E_IPC_SIZE_TOO_LARGE:
            generateError(ExceptionErrorCode::E_SERVICE_ERROR, "Ipc error.");
            break;
        case ExceptionErrorCode::E_MIMETYPE_NOT_FOUND:
            generateError(ExceptionErrorCode::E_OTHER, "Mimetype not found.");
            break;
        case ExceptionErrorCode::E_TASK_INDEX_TOO_LARGE:
            generateError(ExceptionErrorCode::E_TASK_NOT_FOUND, "Task index out of range.");
            break;
        default:
            break;
    }

    return err;
}


CProgress Convert2CProgress(const Progress &in)
{
    CProgress out = { 0 };
    out.state = static_cast<int32_t>(in.state);
    out.index = in.index;
    out.processed = in.processed;
    out.sizeArrLen = static_cast<int64_t>(in.sizes.size());
    if (out.sizeArrLen > 0) {
        out.sizeArr = static_cast<int64_t *>(malloc(sizeof(int64_t) * in.sizes.size()));
        if (out.sizeArr == nullptr) {
            return out;
        }
        for (std::vector<long>::size_type i = 0; i < in.sizes.size(); ++i) {
            out.sizeArr[i] = in.sizes[i];
        }
    }

    out.extras.size = static_cast<int64_t>(in.extras.size());
    if (out.extras.size <= 0) {
        return out;
    }

    out.extras.headers = static_cast<CHashStrPair *>(malloc(sizeof(CHashStrPair) * out.extras.size));
    if (out.extras.headers == nullptr) {
        return out;
    }

    int index = 0;
    for (auto iter = in.extras.begin(); iter != in.extras.end(); ++iter) {
        CHashStrPair *elem = &out.extras.headers[index++];
        elem->key = MallocCString(iter->first);
        elem->value = MallocCString(iter->second);
    }
    return out;
}

void RemoveFile(const std::string &filePath)
{
    auto removeFile = [filePath]() -> void {
        std::remove(filePath.c_str());
        return;
    };
    ffrt::submit(removeFile, {}, {}, ffrt::task_attr().name("Os_Request_Rm").qos(ffrt::qos_default));
}

} //namespace OHOS::CJSystemapi::Request
