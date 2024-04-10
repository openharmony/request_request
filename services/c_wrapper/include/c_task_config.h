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

#ifndef C_TASK_CONFIG_H
#define C_TASK_CONFIG_H

#include <stdint.h>

#include <cstdint>
#include <map>
#include <string>
#include <vector>

#include "c_form_item.h"
#include "c_string_wrapper.h"

struct CommonTaskConfig {
    uint32_t taskId;
    uint64_t uid;
    uint64_t tokenId;
    uint8_t action;
    uint8_t mode;
    bool cover;
    uint8_t network;
    bool metered;
    bool roaming;
    bool retry;
    bool redirect;
    uint32_t index;
    uint64_t begins;
    int64_t ends;
    bool gauge;
    bool precise;
    uint32_t priority;
    bool background;
};

struct CStringMap {
    CStringWrapper key;
    CStringWrapper value;
};

struct CTaskConfig {
    CStringWrapper bundle;
    CStringWrapper url;
    CStringWrapper title;
    CStringWrapper description;
    CStringWrapper method;
    CStringWrapper headers;
    CStringWrapper data;
    CStringWrapper token;
    CStringWrapper proxy;
    CStringWrapper certificatePins;
    CStringWrapper extras;
    uint8_t version;
    CFormItem *formItemsPtr;
    uint32_t formItemsLen;
    CFileSpec *fileSpecsPtr;
    uint32_t fileSpecsLen;
    CStringWrapper *bodyFileNamesPtr;
    uint32_t bodyFileNamesLen;
    CStringWrapper *certsPathPtr;
    uint32_t certsPathLen;
    CommonTaskConfig commonData;
};

struct TaskConfig {
    std::string bundle;
    std::string url;
    std::string title;
    std::string description;
    std::string method;
    std::string headers;
    std::string data;
    std::string token;
    std::string proxy;
    std::string certificatePins;
    std::string extras;
    uint8_t version;
    std::vector<FormItem> formItems;
    std::vector<FileSpec> fileSpecs;
    std::vector<std::string> bodyFileNames;
    std::vector<std::string> certsPath;
    CommonTaskConfig commonData;
};

#ifdef __cplusplus
extern "C" {
#endif

void DeleteCTaskConfig(CTaskConfig *ptr);
void DeleteCTaskConfigs(CTaskConfig **ptr);

#ifdef __cplusplus
}
#endif
#endif // C_TASK_CONFIG_H