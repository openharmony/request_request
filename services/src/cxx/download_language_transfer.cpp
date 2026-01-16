/*
* Copyright (C) 2026 Huawei Device Co., Ltd.
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

#include <string>
#include "locale_config.h"

namespace OHOS::Request {
using namespace Global;

extern "C" {
void GetSystemLanguageByIntl(char *buffer, size_t bufferSize)
{
    if (buffer == nullptr || bufferSize == 0) {
        return;
    }
    std::string language = I18n::LocaleConfig::GetSystemLanguage();
    if (language.empty()) {
        language = "zh-Hans";
    }
    size_t copyLen = std::min(language.size(), bufferSize - 1);
    std::copy_n(language.begin(), copyLen, buffer);
    buffer[copyLen] = '\0';
}
}
} // namespace OHOS::Request