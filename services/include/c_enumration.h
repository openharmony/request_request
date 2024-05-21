/*
* Copyright (c) 2023 Huawei Device Co., Ltd.
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

#ifndef REQUEST_C_ENUMRATION_H
#define REQUEST_C_ENUMRATION_H
#include <cstdint>

enum class Network : uint8_t {
    ANY,
    WIFI,
    CELLULAR,
};

/* used only in sa, do not mix with enum Network. */
enum class NetworkInner : uint8_t {
    ANY,      /* Maintain consistency with Network::ANY */
    WIFI,     /* Maintain consistency with Network::WIFI */
    CELLULAR, /* Maintain consistency with Network::CECCULAR */
    NET_LOST,
};

enum class Action : uint8_t {
    DOWNLOAD,
    UPLOAD,
    ANY,
};

enum class Mode : uint8_t {
    BACKGROUND,
    FOREGROUND,
    ANY,
};

enum class State : uint8_t {
    INITIALIZED = 0x00,
    WAITING = 0x10,
    RUNNING = 0x20,
    RETRYING = 0x21,
    PAUSED = 0x30,
    STOPPED = 0x31,
    COMPLETED = 0x40,
    FAILED = 0x41,
    REMOVED = 0x50,
    CREATED = 0x60,
    ANY = 0x61,
};

enum class Reason : uint8_t {
    REASON_OK = 0,
    TASK_SURVIVAL_ONE_MONTH,
    WAITTING_NETWORK_ONE_DAY,
    STOPPED_NEW_FRONT_TASK,
    RUNNING_TASK_MEET_LIMITS,
    USER_OPERATION,
    APP_BACKGROUND_OR_TERMINATE,
    NETWORK_OFFLINE,
    UNSUPPORTED_NETWORK_TYPE,
    BUILD_CLIENT_FAILED,
    BUILD_REQUEST_FAILED,
    GET_FILESIZE_FAILED,
    CONTINUOUS_TASK_TIMEOUT,
    CONNECT_ERROR,
    REQUEST_ERROR,
    UPLOAD_FILE_ERROR,
    REDIRECT_ERROR,
    PROTOCOL_ERROR,
    IO_ERROR,
    UNSUPPORT_RANGE_REQUEST,
    OTHERS_ERROR,
};
#endif // REQUEST_C_ENUMRATION_H
