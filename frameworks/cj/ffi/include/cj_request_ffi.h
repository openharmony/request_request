/*
 * Copyright (c) 2024 Huawei Device Co., Ltd.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef OH_CJ_REQUEST_FFI_H
#define OH_CJ_REQUEST_FFI_H

#include <cstdint>

#include "cj_common_ffi.h"

#ifndef FFI_EXPORT
#ifndef WINDOWS_PLATFORM
#define FFI_EXPORT __attribute__((visibility("default")))
#else
#define FFI_EXPORT __declspec(dllexport)
#endif
#endif

extern "C" {
typedef struct {
    char *key;
    char *value;
} CHashStrPair;

typedef struct {
    CHashStrPair *headers;
    int64_t size;
} CHashStrArr;

typedef struct {
    char *path;
    char *mimeType;
    char *filename;
    CHashStrArr extras;
} CFileSpec;

typedef struct {
    CFileSpec *head;
    int64_t size;
} CFileSpecArr;

enum CFormItemValueType {
    CFORM_ITEM_VALUE_TYPE_STRING = 0U,
    CFORM_ITEM_VALUE_TYPE_FILE,
    CFORM_ITEM_VALUE_TYPE_FILES,
};

typedef struct {
    char *str;
    CFileSpec file;
    CFileSpecArr files;
    uint32_t type;
} CFormItemValueTypeUion;

typedef struct {
    char *name;
    CFormItemValueTypeUion value;
} CFormItem;

typedef struct {
    CFormItem *head;
    int64_t size;
} CFormItemArr;

typedef struct {
    char *str;
    CFormItemArr formItems;
} CConfigDataTypeUion;

typedef struct {
    uint32_t action;
    char *url;
    char *title;
    char *description;
    uint32_t mode;
    bool overwrite;
    char *method;
    CHashStrArr headers;
    CConfigDataTypeUion data;
    char *saveas;
    uint32_t network;
    bool metered;
    bool roaming;
    bool retry;
    bool redirect;
    uint32_t index;
    int64_t begins;
    int64_t ends;
    bool gauge;
    bool precise;
    char *token;
    uint32_t priority;
    CHashStrArr extras;
} CConfig;

typedef struct {
    uint32_t state;
    uint32_t index;
    int64_t processed;
    int64_t *sizeArr;
    int64_t sizeArrLen;
    CHashStrArr extras;
} CProgress;

typedef struct {
    char *key;
    CArrString value;
} CHttpHeaderHashPair;

typedef struct {
    CHttpHeaderHashPair *hashHead;
    int64_t size;
} CHttpHeader;

typedef struct {
    char *version;
    int32_t statusCode;
    char *reason;
    CHttpHeader headers;
} CResponse;

typedef struct {
    char *uid;
    char *bundle;
    char *saveas;
    char *url;
    CConfigDataTypeUion data;
    char *tid;
    char *title;
    char *description;
    uint32_t action;
    uint32_t mode;
    uint32_t priority;
    char *mimeType;
    CProgress progress;
    bool gauge;
    uint64_t ctime;
    uint64_t mtime;
    bool retry;
    uint32_t tries;
    uint32_t faults;
    char *reason;
    CHashStrArr extras;
} CTaskInfo;

typedef struct {
    bool hasValue;
    const char *value;
} RequestNativeOptionCString;

typedef struct {
    bool hasValue;
    int64_t value;
} RequestNativeOptionInt64;

typedef struct {
    bool hasValue;
    uint32_t value;
} RequestNativeOptionUInt32;

typedef struct {
    RequestNativeOptionCString bundle;
    RequestNativeOptionInt64 before;
    RequestNativeOptionInt64 after;
    RequestNativeOptionUInt32 state;
    RequestNativeOptionUInt32 action;
    RequestNativeOptionUInt32 mode;
} CFilter;

typedef struct {
    char **head;
    int64_t size;
} RequestCArrString;

typedef struct {
    int32_t errCode;
    char *errMsg;
} RetError;

typedef struct {
    int64_t instanceId;
    const char *taskId;
    RetError err;
} RetReqData;

typedef struct {
    RetError err;
    CTaskInfo task;
} RetTaskInfo;

typedef struct {
    RetError err;
    RequestCArrString tasks;
} RetTaskArr;

typedef struct {
    const char *taskId;
    CConfig config;
} CTask;

typedef struct {
    RetError err;
    CTask tid;
} RetTask;

FFI_EXPORT void FfiOHOSRequestFreeTask(const char *taskId);
FFI_EXPORT RetError FfiOHOSRequestTaskProgressOn(char *event, const char *taskId, void *callback);
FFI_EXPORT RetError FfiOHOSRequestTaskProgressOff(char *event, const char *taskId, void *callback);
FFI_EXPORT RetError FfiOHOSRequestTaskStart(const char *taskId);
FFI_EXPORT RetError FfiOHOSRequestTaskPause(const char *taskId);
FFI_EXPORT RetError FfiOHOSRequestTaskResume(const char *taskId);
FFI_EXPORT RetError FfiOHOSRequestTaskStop(const char *taskId);
FFI_EXPORT RetReqData FfiOHOSRequestCreateTask(void *context, CConfig config);
FFI_EXPORT RetTask FfiOHOSRequestGetTask(void *context, const char *taskId, RequestNativeOptionCString token);
FFI_EXPORT RetError FfiOHOSRequestRemoveTask(const char *taskId);
FFI_EXPORT RetTaskInfo FfiOHOSRequestShowTask(const char *taskId);
FFI_EXPORT RetTaskInfo FfiOHOSRequestTouchTask(const char *taskId, char *token);
FFI_EXPORT RetTaskArr FfiOHOSRequestSearchTask(CFilter filter);
}
#endif