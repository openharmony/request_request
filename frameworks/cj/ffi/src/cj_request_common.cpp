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
#include <fstream>
#include <sstream>

#include "cj_request_ffi.h"
#include "ffrt.h"
#include "log.h"
#include "openssl/sha.h"
#include "parameter.h"
#include "request_common.h"
#include "securec.h"

namespace OHOS::CJSystemapi::Request {
using OHOS::Request::Action;
using OHOS::Request::ExceptionErrorCode;
using OHOS::Request::Faults;
using OHOS::Request::FileSpec;
using OHOS::Request::FormItem;
using OHOS::Request::Reason;

static constexpr const char *REASON_OK_INFO = "Task successful";
static constexpr const char *TASK_SURVIVAL_ONE_MONTH_INFO = "The task has not been completed for a month yet";
static constexpr const char *WAITTING_NETWORK_ONE_DAY_INFO = "The task waiting for network recovery has not been "
                                                             "completed for a day yet";
static constexpr const char *STOPPED_NEW_FRONT_TASK_INFO = "Stopped by a new front task";
static constexpr const char *RUNNING_TASK_MEET_LIMITS_INFO = "Too many task in running state";
static constexpr const char *USER_OPERATION_INFO = "User operation";
static constexpr const char *APP_BACKGROUND_OR_TERMINATE_INFO = "The app is background or terminate";
static constexpr const char *NETWORK_OFFLINE_INFO = "NetWork is offline";
static constexpr const char *UNSUPPORTED_NETWORK_TYPE_INFO = "NetWork type not meet the task config";
static constexpr const char *BUILD_CLIENT_FAILED_INFO = "Build client error";
static constexpr const char *BUILD_REQUEST_FAILED_INFO = "Build request error";
static constexpr const char *GET_FILESIZE_FAILED_INFO = "Failed because cannot get the file size from the server and "
                                                        "the precise is setted true by user";
static constexpr const char *CONTINUOUS_TASK_TIMEOUT_INFO = "Continuous processing task time out";
static constexpr const char *CONNECT_ERROR_INFO = "Connect error";
static constexpr const char *REQUEST_ERROR_INFO = "Request error";
static constexpr const char *UPLOAD_FILE_ERROR_INFO = "There are some files upload failed";
static constexpr const char *REDIRECT_ERROR_INFO = "Redirect error";
static constexpr const char *PROTOCOL_ERROR_INFO = "Http protocol error";
static constexpr const char *IO_ERROR_INFO = "Io Error";
static constexpr const char *UNSUPPORT_RANGE_REQUEST_INFO = "Range request not supported";
static constexpr const char *OTHERS_ERROR_INFO = "Some other error occured";
static constexpr const char *ACCOUNT_STOPPED_INFO = "Account stopped";
static constexpr const char *NETWORK_CHANGED_INFO = "Network changed";
static constexpr const char *DNS_INFO = "DNS error";
static constexpr const char *TCP_INFO = "TCP error";
static constexpr const char *SSL_INFO = "TSL/SSL error";
static constexpr const char *INSUFFICIENT_SPACE_INFO = "Insufficient space error";
static constexpr int32_t API_VERSION_19 = 19;

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

char *MallocCString(const std::string &origin)
{
    if (origin.empty()) {
        return nullptr;
    }
    auto len = origin.length() + 1;
    char *res = static_cast<char *>(malloc(sizeof(char) * len));
    if (res == nullptr) {
        return nullptr;
    }
    return std::char_traits<char>::copy(res, origin.c_str(), len);
}

bool IsPathValid(const std::string &filePath)
{
    auto path = filePath.substr(0, filePath.rfind('/'));
    char resolvedPath[PATH_MAX + 1] = {0};
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
    ExceptionError err;
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
            generateError(static_cast<ExceptionErrorCode>(errorCode), "");
            break;
    }

    return err;
}

CProgress Convert2CProgress(const Progress &in)
{
    CProgress out = {0};
    out.state = static_cast<uint32_t>(in.state);
    out.index = in.index;
    out.processed = in.processed;

    if (in.sizes.size() > 0) {
        out.sizeArr = static_cast<int64_t *>(malloc(sizeof(int64_t) * in.sizes.size()));
        if (out.sizeArr == nullptr) {
            return out;
        }
        for (std::vector<long>::size_type i = 0; i < in.sizes.size(); ++i) {
            out.sizeArr[i] = in.sizes[i];
        }
        out.sizeArrLen = static_cast<int64_t>(in.sizes.size());
    }

    if (in.extras.size() <= 0) {
        return out;
    }

    out.extras.headers = static_cast<CHashStrPair *>(malloc(sizeof(CHashStrPair) * in.extras.size()));
    if (out.extras.headers == nullptr) {
        return out;
    }

    int index = 0;
    for (auto iter = in.extras.begin(); iter != in.extras.end(); ++iter) {
        CHashStrPair *elem = &out.extras.headers[index++];
        elem->key = MallocCString(iter->first);
        elem->value = MallocCString(iter->second);
    }
    out.extras.size = static_cast<int64_t>(in.extras.size());
    return out;
}

CArrString Convert2CArrString(const std::vector<std::string> &v)
{
    CArrString out = {};
    if (v.empty()) {
        return out;
    }

    out.head = static_cast<char **>(malloc(sizeof(char *) * v.size()));
    if (out.head == nullptr) {
        return out;
    }

    int64_t index = 0;
    for (auto iter : v) {
        out.head[index] = MallocCString(iter);
        index++;
    }
    out.size = index;
    return out;
}

CResponse Convert2CResponse(const std::shared_ptr<Response> &in)
{
    CResponse out = {0};
    out.version = MallocCString(in->version);
    out.statusCode = in->statusCode;
    out.reason = MallocCString(in->reason);

    if (in->headers.size() <= 0) {
        return out;
    }
    CHttpHeaderHashPair *hashHead =
        static_cast<CHttpHeaderHashPair *>(malloc(sizeof(CHttpHeaderHashPair) * in->headers.size()));
    if (hashHead == nullptr) {
        return out;
    }

    int64_t index = 0;
    for (auto iter : in->headers) {
        hashHead[index].key = MallocCString(iter.first);
        hashHead[index].value = Convert2CArrString(iter.second);
        index++;
    }
    out.headers.hashHead = hashHead;
    out.headers.size = index;
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

uint32_t Convert2Broken(Reason code)
{
    static std::map<Reason, Faults> InnerCodeToBroken = {
        {Reason::REASON_OK, Faults::OTHERS},
        {Reason::TASK_SURVIVAL_ONE_MONTH, Faults::OTHERS},
        {Reason::WAITTING_NETWORK_ONE_DAY, Faults::OTHERS},
        {Reason::STOPPED_NEW_FRONT_TASK, Faults::OTHERS},
        {Reason::RUNNING_TASK_MEET_LIMITS, Faults::OTHERS},
        {Reason::USER_OPERATION, Faults::OTHERS},
        {Reason::APP_BACKGROUND_OR_TERMINATE, Faults::OTHERS},
        {Reason::NETWORK_OFFLINE, Faults::DISCONNECTED},
        {Reason::UNSUPPORTED_NETWORK_TYPE, Faults::OTHERS},
        {Reason::BUILD_CLIENT_FAILED, Faults::PARAM},
        {Reason::BUILD_REQUEST_FAILED, Faults::PARAM},
        {Reason::GET_FILESIZE_FAILED, Faults::FSIO},
        {Reason::CONTINUOUS_TASK_TIMEOUT, Faults::OTHERS},
        {Reason::CONNECT_ERROR, Faults::TCP},
        {Reason::REQUEST_ERROR, Faults::PROTOCOL},
        {Reason::UPLOAD_FILE_ERROR, Faults::OTHERS},
        {Reason::REDIRECT_ERROR, Faults::REDIRECT},
        {Reason::PROTOCOL_ERROR, Faults::PROTOCOL},
        {Reason::IO_ERROR, Faults::FSIO},
        {Reason::UNSUPPORT_RANGE_REQUEST, Faults::PROTOCOL},
        {Reason::OTHERS_ERROR, Faults::OTHERS},
        {Reason::ACCOUNT_STOPPED, Faults::OTHERS},
        {Reason::NETWORK_CHANGED, Faults::OTHERS},
        {Reason::DNS, Faults::DNS},
        {Reason::TCP, Faults::TCP},
        {Reason::SSL, Faults::SSL},
        {Reason::INSUFFICIENT_SPACE, Faults::OTHERS},
        {Reason::NETWORK_APP, Faults::DISCONNECTED},
        {Reason::NETWORK_ACCOUNT, Faults::DISCONNECTED},
        {Reason::APP_ACCOUNT, Faults::OTHERS},
        {Reason::NETWORK_APP_ACCOUNT, Faults::DISCONNECTED},
    };
    constexpr const int32_t detailVersion = 12;
    auto iter = InnerCodeToBroken.find(code);
    if (iter != InnerCodeToBroken.end()) {
        int32_t sdkVersion = GetSdkApiVersion();
        REQUEST_HILOGD("GetSdkApiVersion %{public}d", sdkVersion);
        if (sdkVersion < detailVersion &&
            (iter->second == Faults::PARAM || iter->second == Faults::DNS || iter->second == Faults::TCP ||
             iter->second == Faults::SSL || iter->second == Faults::REDIRECT)) {
            return static_cast<uint32_t>(Faults::OTHERS);
        }
        return static_cast<uint32_t>(iter->second);
    }
    return 0;
}

std::string Convert2ReasonMsg(Reason code)
{
    static std::map<Reason, std::string> ReasonMsg = {
        {Reason::REASON_OK, REASON_OK_INFO},
        {Reason::TASK_SURVIVAL_ONE_MONTH, TASK_SURVIVAL_ONE_MONTH_INFO},
        {Reason::WAITTING_NETWORK_ONE_DAY, WAITTING_NETWORK_ONE_DAY_INFO},
        {Reason::STOPPED_NEW_FRONT_TASK, STOPPED_NEW_FRONT_TASK_INFO},
        {Reason::RUNNING_TASK_MEET_LIMITS, RUNNING_TASK_MEET_LIMITS_INFO},
        {Reason::USER_OPERATION, USER_OPERATION_INFO},
        {Reason::APP_BACKGROUND_OR_TERMINATE, APP_BACKGROUND_OR_TERMINATE_INFO},
        {Reason::NETWORK_OFFLINE, NETWORK_OFFLINE_INFO},
        {Reason::UNSUPPORTED_NETWORK_TYPE, UNSUPPORTED_NETWORK_TYPE_INFO},
        {Reason::BUILD_CLIENT_FAILED, BUILD_CLIENT_FAILED_INFO},
        {Reason::BUILD_REQUEST_FAILED, BUILD_REQUEST_FAILED_INFO},
        {Reason::GET_FILESIZE_FAILED, GET_FILESIZE_FAILED_INFO},
        {Reason::CONTINUOUS_TASK_TIMEOUT, CONTINUOUS_TASK_TIMEOUT_INFO},
        {Reason::CONNECT_ERROR, CONNECT_ERROR_INFO},
        {Reason::REQUEST_ERROR, REQUEST_ERROR_INFO},
        {Reason::UPLOAD_FILE_ERROR, UPLOAD_FILE_ERROR_INFO},
        {Reason::REDIRECT_ERROR, REDIRECT_ERROR_INFO},
        {Reason::PROTOCOL_ERROR, PROTOCOL_ERROR_INFO},
        {Reason::IO_ERROR, IO_ERROR_INFO},
        {Reason::UNSUPPORT_RANGE_REQUEST, UNSUPPORT_RANGE_REQUEST_INFO},
        {Reason::OTHERS_ERROR, OTHERS_ERROR_INFO},
        {Reason::ACCOUNT_STOPPED, ACCOUNT_STOPPED_INFO},
        {Reason::NETWORK_CHANGED, NETWORK_CHANGED_INFO},
        {Reason::DNS, DNS_INFO},
        {Reason::TCP, TCP_INFO},
        {Reason::SSL, SSL_INFO},
        {Reason::INSUFFICIENT_SPACE, INSUFFICIENT_SPACE_INFO},
    };
    auto iter = ReasonMsg.find(code);
    if (iter != ReasonMsg.end()) {
        return iter->second;
    }
    return "unknown";
}

CHashStrArr Convert2CHashStrArr(const std::map<std::string, std::string> &extras)
{
    CHashStrArr out = {NULL};
    size_t size = extras.size();
    if (size == 0 || size > std::numeric_limits<size_t>::max() / sizeof(CHashStrPair)) {
        return out;
    }

    out.headers = static_cast<CHashStrPair *>(malloc(sizeof(CHashStrPair) * size));
    if (out.headers == nullptr) {
        return out;
    }

    size_t i = 0;
    for (const auto &it : extras) {
        out.headers[i].key = MallocCString(it.first);
        out.headers[i].value = MallocCString(it.second);
        ++i;
    }
    out.size = static_cast<int64_t>(i);
    return out;
}

CFormItemArr Convert2CFormItemArr(const std::vector<FileSpec> &files, const std::vector<FormItem> &forms)
{
    CFormItemArr out = {NULL};
    size_t filesLen = files.size();
    size_t formsLen = forms.size();
    size_t len = filesLen + formsLen;
    if (len == 0) {
        return out;
    }

    out.head = static_cast<CFormItem *>(malloc(sizeof(CFormItem) * len));
    if (out.head == NULL) {
        return out;
    }
    memset_s(out.head, sizeof(CFormItem) * len, 0, sizeof(CFormItem) * len);
    size_t i = 0;
    for (; i < formsLen; ++i) {
        out.head[i].name = MallocCString(forms[i].name);
        out.head[i].value.str = MallocCString(forms[i].value);
        out.head[i].value.type = CFORM_ITEM_VALUE_TYPE_STRING;
    }

    for (size_t j = 0; j < filesLen; ++j) {
        out.head[i].name = MallocCString(files[j].name);
        out.head[i].value.file.path = MallocCString(files[j].uri);
        out.head[i].value.file.mimeType = MallocCString(files[j].type);
        out.head[i].value.file.filename = MallocCString(files[j].filename);
        out.head[i].value.type = CFORM_ITEM_VALUE_TYPE_FILE;
        ++i;
    }

    out.size = static_cast<int64_t>(i);
    return out;
}

bool CheckApiVersionAfter19()
{
    return GetSdkApiVersion() > API_VERSION_19;
}
} // namespace OHOS::CJSystemapi::Request
