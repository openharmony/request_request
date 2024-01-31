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

#include "c_string_wrapper.h"

#include <securec.h>

#include <cstdint>

#include "log.h"

void DeleteChar(char *ptr)
{
    delete[] ptr;
}

CStringWrapper WrapperCString(const std::string &str)
{
    CStringWrapper cStringWrapper;
    cStringWrapper.len = str.length();
    cStringWrapper.cStr = new char[cStringWrapper.len];
    memcpy_s(cStringWrapper.cStr, cStringWrapper.len, str.c_str(), cStringWrapper.len);
    return cStringWrapper;
}
