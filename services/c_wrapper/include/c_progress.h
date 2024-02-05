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

#ifndef C_PROGRESS_H
#define C_PROGRESS_H

#include <stdint.h>

#include <cstdint>
#include <string>

#include "c_string_wrapper.h"

struct CommonProgress {
    uint8_t state;
    uintptr_t index;
    uintptr_t totalProcessed;
};

struct CProgress {
    CommonProgress commonData;
    CStringWrapper sizes;
    CStringWrapper processed;
    CStringWrapper extras;
};

struct Progress {
    CommonProgress commonData;
    std::string sizes;
    std::string processed;
    std::string extras;
};

#endif // C_PROGRESS_H