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

#ifndef CONSTANT_H
#define CONSTANT_H
#include <cstdint>
#include <string>

namespace OHOS::Request {

enum PausedReason {
    PAUSED_QUEUED_FOR_WIFI,
    PAUSED_WAITING_FOR_NETWORK,
    PAUSED_WAITING_TO_RETRY,
    PAUSED_BY_USER,
    PAUSED_UNKNOWN,
};

enum ExceptionErrorCode : int32_t {
    E_OK = 0,
    E_UNLOADING_SA,
    E_IPC_SIZE_TOO_LARGE,
    E_MIMETYPE_NOT_FOUND,
    E_TASK_INDEX_TOO_LARGE,
    E_CHANNEL_NOT_OPEN = 5,
    E_PERMISSION = 201,
    E_NOT_SYSTEM_APP = 202,
    E_PARAMETER_CHECK = 401,
    E_UNSUPPORTED = 801,
    E_FILE_IO = 13400001,
    E_FILE_PATH = 13400002,
    E_SERVICE_ERROR = 13400003,
    E_OTHER = 13499999,
    E_TASK_QUEUE = 21900004,
    E_TASK_MODE = 21900005,
    E_TASK_NOT_FOUND = 21900006,
    E_TASK_STATE = 21900007,
};

struct ExceptionError {
    ExceptionErrorCode code;
    std::string errInfo;
};

static constexpr const char *E_OK_INFO = "check succeeded";
static constexpr const char *E_PERMISSION_INFO = "the permissions check fails";
static constexpr const char *E_PARAMETER_CHECK_INFO = "the parameters check fails";
static constexpr const char *E_UNSUPPORTED_INFO = "call unsupported api";
static constexpr const char *E_FILE_IO_INFO = " file operation error";
static constexpr const char *E_FILE_PATH_INFO = "bad file path";
static constexpr const char *E_SERVICE_ERROR_INFO = "task service ability error";
static constexpr const char *E_OTHER_INFO = "other error";
static constexpr const char *E_TASK_QUEUE_INFO = "application task queue full error";
static constexpr const char *E_TASK_MODE_INFO = "task mode error";
static constexpr const char *E_TASK_NOT_FOUND_INFO = "task not found error";
static constexpr const char *E_TASK_STATE_INFO = "task state error";

static constexpr const char *FUNCTION_PAUSE = "pause";
static constexpr const char *FUNCTION_QUERY = "query";
static constexpr const char *FUNCTION_QUERY_MIME_TYPE = "queryMimeType";
static constexpr const char *FUNCTION_REMOVE = "remove";
static constexpr const char *FUNCTION_RESUME = "resume";
static constexpr const char *FUNCTION_ON = "on";
static constexpr const char *FUNCTION_OFF = "off";
static constexpr const char *FUNCTION_START = "start";
static constexpr const char *FUNCTION_STOP = "stop";
static constexpr const char *FUNCTION_SUSPEND = "suspend";
static constexpr const char *FUNCTION_GET_TASK_INFO = "getTaskInfo";
static constexpr const char *FUNCTION_GET_TASK_MIME_TYPE = "getTaskMimeType";
static constexpr const char *FUNCTION_DELETE = "delete";
static constexpr const char *FUNCTION_RESTORE = "restore";

constexpr const std::uint32_t CONFIG_PARAM_AT_FIRST = 0;
constexpr const std::uint32_t CONFIG_PARAM_AT_SECOND = 1;

static constexpr const char *PARAM_KEY_METHOD = "method";
static constexpr const char *PARAM_KEY_FILES = "files";
static constexpr const char *PARAM_KEY_DATA = "data";

static constexpr uint32_t NETWORK_MOBILE = 0x00000001;
static constexpr uint32_t NETWORK_WIFI = 0x00010000;

static const std::string tlsVersion = "X-TLS-Version";
static const std::string cipherList = "X-Cipher-List";
static const std::string TLS_VERSION = "CURL_SSLVERSION_TLSv1_2";
static const std::string TLS_CIPHER = "TLS_DHE_RSA_WITH_AES_128_GCM_SHA256,TLS_DHE_RSA_WITH_AES_256_GCM_SHA384,"
                                      "TLS_DHE_DSS_WITH_AES_128_GCM_SHA256,TLS_DSS_RSA_WITH_AES_256_GCM_SHA384,"
                                      "TLS_PSK_WITH_AES_256_GCM_SHA384,TLS_DHE_PSK_WITH_AES_128_GCM_SHA256,"
                                      "TLS_DHE_PSK_WITH_AES_256_GCM_SHA384,"
                                      "TLS_DHE_PSK_WITH_CHACHA20_POLY1305_SHA256,"
                                      "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256,"
                                      "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384,"
                                      "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256,TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384,"
                                      "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256,"
                                      "TLS_ECDHE_PSK_WITH_CHACHA20_POLY1305_SHA256,"
                                      "TLS_ECDHE_PSK_WITH_AES_128_GCM_SHA256,TLS_ECDHE_PSK_WITH_AES_256_GCM_SHA384,"
                                      "TLS_ECDHE_PSK_WITH_AES_128_GCM_SHA256,"
                                      "TLS_DHE_RSA_WITH_AES_128_CCM,TLS_DHE_RSA_WITH_AES_256_CCM,"
                                      "TLS_DHE_RSA_WITH_CHACHA20_POLY1305_SHA256,TLS_PSK_WITH_AES_256_CCM,"
                                      "TLS_DHE_PSK_WITH_AES_128_CCM,TLS_DHE_PSK_WITH_AES_256_CCM,"
                                      "TLS_ECDHE_ECDSA_WITH_AES_128_CCM,TLS_ECDHE_ECDSA_WITH_AES_256_CCM,"
                                      "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256,TLS_AES_128_GCM_SHA256,"
                                      "TLS_AES_256_GCM_SHA384,TLS_CHACHA20_POLY1305_SHA256,TLS_AES_128_CCM_SHA256,"
                                      "TLS_SM4_GCM_SM3,TLS_SM4_CCM_SM3";

} // namespace OHOS::Request

#endif // CONSTANT_H
