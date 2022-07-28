/*
 * Copyright (c) 2021 Huawei Device Co., Ltd.
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
#include "sql_analyzer.h"
#include "log.h"

namespace OHOS::Request::Download {
constexpr int POS_ADD_TWO = 2;

SqlAnalyzer::SqlAnalyzer()
{
}

SqlAnalyzer::~SqlAnalyzer()
{
}

bool SqlAnalyzer::CheckValuesBucket(const NativeRdb::ValuesBucket &value)
{
    std::map<std::string, NativeRdb::ValueObject> valuesMap;
    value.GetAll(valuesMap);
    for (auto it = valuesMap.begin(); it != valuesMap.end(); ++it) {
        std::string key = it->first;
        bool isKey = FindIllegalWords(key);
        if (isKey) {
            DOWNLOAD_HILOGE("SqlAnalyzer CheckValuesBucket key is %{public}s error", key.c_str());
            return false;
        }
        NativeRdb::ValueObject value = it->second;
        if (value.GetType() == NativeRdb::ValueObjectType::TYPE_STRING) {
            std::string str;
            value.GetString(str);
            bool isValue = FindIllegalWords(str);
            if (isValue) {
                DOWNLOAD_HILOGE("SqlAnalyzer CheckValuesBucket value is %{public}s error", str.c_str());
                return false;
            }
        }
    }
    return true;
}

bool SqlAnalyzer::CharCheck(char &ch, std::string sql, std::size_t &pos)
{
    if (ch == '[') {
        pos++;
        std::size_t found = sql.find(']', pos);
        if (found == std::string::npos) {
            return true;
        }
        pos++;
    }
    if (ch == '-' && PickChar(sql, pos + 1) == '-') {
        pos += POS_ADD_TWO;
        std::size_t found = sql.find('\n', pos);
        if (found == std::string::npos) {
            return true;
        }
        pos++;
    }
    if (ch == '/' && PickChar(sql, pos + 1) == '*') {
        pos += POS_ADD_TWO;
        std::size_t found = sql.find("*/", pos);
        if (found == std::string::npos) {
            return true;
        }
        pos += POS_ADD_TWO;
    }
    if (ch == ';') {
        return true;
    }
    pos++;
    return false;
}

bool SqlAnalyzer::StrCheck(char &ch, std::size_t strlen, std::string sql, std::size_t &pos)
{
    if (IsInStr(ch, "'\"`") == 0) {
        pos++;
        while (pos < strlen) {
            std::size_t found = sql.find(ch, pos);
            if (found == std::string::npos) {
                return true;
            }
            if (PickChar(sql, pos + 1) != ch) {
                break;
            }
            pos += POS_ADD_TWO;
        }
    }
    return false;
}

bool SqlAnalyzer::FindIllegalWords(std::string sql)
{
    if (sql.empty()) {
        return false;
    }
    std::size_t pos = 0;
    std::size_t strlen = sql.length();
    while (pos < strlen) {
        char ch = PickChar(sql, pos);
        if (IsLetter(ch)) {
            std::size_t start = pos;
            pos++;
            while (IsLetterNumber(PickChar(sql, pos))) {
                pos++;
            }
            std::size_t count = pos - start + 1;
            sql.substr(start, count);
        }
        if (IsInStr(ch, "'\"`") == 0) {
            if (StrCheck(ch, strlen, sql, pos)) {
                return true;
            } else {
                continue;
            }
        }
        if (CharCheck(ch, sql, pos)) {
            return true;
        } else {
            continue;
        }
    }
    return false;
}
} // namespace OHOS::Request::Download