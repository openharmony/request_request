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

#ifndef C_TASK_INFO_H
#define C_TASK_INFO_H

#include <stdint.h>

#include <cstdint>
#include <string>
#include <vector>

#include "c_form_item.h"
#include "c_progress.h"
#include "c_string_wrapper.h"

struct CommonTaskInfo {
    uint32_t taskId;
    uint64_t uid;
    uint8_t action;
    uint8_t mode;
    uint64_t ctime;
    uint64_t mtime;
    uint8_t reason;
    bool gauge;
    bool retry;
    uint32_t tries;
    uint8_t version;
    uint32_t priority;
};

struct CEachFileStatus {
    CStringWrapper path;
    uint8_t reason;
    CStringWrapper message;
};

struct EachFileStatus {
    std::string path;
    uint8_t reason;
    std::string message;
};

struct CTaskInfo {
    CStringWrapper bundle;
    CStringWrapper url;
    CStringWrapper data;
    CStringWrapper token;
    CFormItem *formItemsPtr;
    uint32_t formItemsLen;
    CFileSpec *fileSpecsPtr;
    uint32_t fileSpecsLen;
    CStringWrapper title;
    CStringWrapper description;
    CStringWrapper mimeType;
    CProgress progress;
    CEachFileStatus *eachFileStatusPtr;
    uint32_t eachFileStatusLen;
    CommonTaskInfo commonData;
};

struct TaskInfo {
    std::string bundle;
    std::string url;
    std::string data;
    std::string token;
    std::vector<FormItem> formItems;
    std::vector<FileSpec> fileSpecs;
    std::string title;
    std::string description;
    std::string mimeType;
    Progress progress;
    std::vector<EachFileStatus> eachFileStatus;
    CommonTaskInfo commonData;
};

struct CUpdateInfo {
    uint64_t mtime;
    uint8_t reason;
    uint32_t tries;
    CStringWrapper mimeType;
    CProgress progress;
    CEachFileStatus *eachFileStatusPtr;
    uint32_t eachFileStatusLen;
};

struct TaskQosInfo {
    uint32_t taskId;
    uint8_t action;
    uint8_t mode;
    uint8_t state;
    uint32_t priority;
};

struct AppInfo {
    uint64_t uid;
    CStringWrapper bundle;
};

#ifdef __cplusplus
extern "C" {
#endif

void DeleteCFormItem(CFormItem *ptr);
void DeleteCFileSpec(CFileSpec *ptr);
void DeleteCStringPtr(CStringWrapper *ptr);
void DeleteCEachFileStatus(CEachFileStatus *ptr);
void DeleteCTaskInfo(CTaskInfo *ptr);
void DeleteTaskQosInfo(TaskQosInfo *ptr);
void DeleteAppInfo(AppInfo *ptr);

#ifdef __cplusplus
}
#endif
#endif // C_TASK_INFO_H