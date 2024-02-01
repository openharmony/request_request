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

#include "utf8_utils.h"

namespace OHOS::Request::Utf8Utils {

static constexpr size_t TWO_OCTET = 2;
static constexpr size_t THREE_OCTET = 3;
static constexpr size_t FOUR_OCTET = 4;

// Given a first byte, determines how many bytes are in this UTF-8 character.
size_t Utf8CharWidth(uint8_t b)
{
    // https://tools.ietf.org/html/rfc3629
    static const size_t UTF8_CHAR_WIDTH[256] = {
        // 1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 1
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 2
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 3
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 4
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 5
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 6
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // A
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // B
        0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // C
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // D
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // E
        4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // F
    };
    return UTF8_CHAR_WIDTH[b];
}

bool GetNextByte(const std::vector<uint8_t> &v, size_t &index, uint8_t &next)
{
    index += 1;
    if (index >= v.size()) {
        return false;
    }
    next = v[index];
    return true;
}

// https://tools.ietf.org/html/rfc3629
// UTF8-1      = %x00-7F
// UTF8-2      = %xC2-DF UTF8-tail
bool Check2Bytes(const std::vector<uint8_t> &v, size_t &index)
{
    uint8_t next = 0;
    return GetNextByte(v, index, next) && (next >= 0x80 && next <= 0xBF);
}

// https://tools.ietf.org/html/rfc3629
// UTF8-3      = %xE0 %xA0-BF UTF8-tail / %xE1-EC 2( UTF8-tail ) /
//               %xED %x80-9F UTF8-tail / %xEE-EF 2( UTF8-tail )
bool Check3Bytes(const std::vector<uint8_t> &v, const size_t &first, size_t &index)
{
    uint8_t next = 0;
    if (!GetNextByte(v, index, next)) {
        return false;
    };

    if (first == 0xE0 && next >= 0xA0 && next <= 0xBF) {
    } else if (first >= 0xE1 && first <= 0xEC && next >= 0x80 && next <= 0xBF) {
    } else if (first == 0xED && next >= 0x80 && next <= 0x9F) {
    } else if (first >= 0xEE && first <= 0xEF && next >= 0x80 && next <= 0xBF) {
    } else {
        return false;
    };

    return Check2Bytes(v, index);
}

// https://tools.ietf.org/html/rfc3629
// UTF8-4      = %xF0 %x90-BF 2( UTF8-tail ) / %xF1-F3 3( UTF8-tail ) /
//               %xF4 %x80-8F 2( UTF8-tail )
bool Check4Bytes(const std::vector<uint8_t> &v, const size_t &first, size_t &index)
{
    uint8_t next = 0;
    if (!GetNextByte(v, index, next)) {
        return false;
    };

    if (first == 0xF0 && next >= 0x90 && next <= 0xBF) {
    } else if (first >= 0xF1 && first <= 0xF3 && next >= 0x80 && next <= 0xBF) {
    } else if (first == 0xF4 && next >= 0x80 && next <= 0x8F) {
    } else {
        return false;
    }

    return Check2Bytes(v, index) && Check2Bytes(v, index);
}

bool RunUtf8Validation(const std::vector<uint8_t> &v)
{
    size_t index = 0;
    size_t len = v.size();

    while (index < len) {
        uint8_t first = v[index];

        // <= 0x7F means single byte.
        if (first <= 0x7F) {
            index += 1;
            continue;
        }

        size_t w = Utf8CharWidth(first);
        if (w == TWO_OCTET) {
            if (!Check2Bytes(v, index)) {
                return false;
            }
        } else if (w == THREE_OCTET) {
            if (!Check3Bytes(v, first, index)) {
                return false;
            }
        } else if (w == FOUR_OCTET) {
            if (!Check4Bytes(v, first, index)) {
                return false;
            }
        } else {
            return false;
        };
        index += 1;
    }
    return true;
}
} // namespace OHOS::Request::Utf8Utils