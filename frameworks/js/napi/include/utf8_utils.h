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

#ifndef UTF8_UTILS_H
#define UTF8_UTILS_H

#include <cstddef>
#include <cstdint>
#include <vector>

namespace OHOS::Request::Utf8Utils {
size_t Utf8CharWidth(uint8_t b);
bool GetNextByte(const std::vector<uint8_t> &v, size_t &index, uint8_t &next);
bool Check2Bytes(const std::vector<uint8_t> &v, size_t &index);
bool Check3Bytes(const std::vector<uint8_t> &v, const size_t &first, size_t &index);
bool Check4Bytes(const std::vector<uint8_t> &v, const size_t &first, size_t &index);
bool RunUtf8Validation(const std::vector<uint8_t> &v);
} // namespace OHOS::Request::Utf8Utils
#endif // UV_QUEUE_H
