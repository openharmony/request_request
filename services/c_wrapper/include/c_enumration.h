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
    DEFAULT = 0x60,
    ANY = 0x61,
};
#endif // REQUEST_C_ENUMRATION_H
