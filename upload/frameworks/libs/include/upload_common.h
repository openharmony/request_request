/*
 * Copyright (c) 2022 Huawei Device Co., Ltd.
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

#ifndef UPLOAD_COMMON_
#define UPLOAD_COMMON_

#include <string>
namespace OHOS::Request::Upload {
enum Type {
    TYPE_PROGRESS_CALLBACK,
    TYPE_HEADER_RECEIVE_CALLBACK,
    TYPE_FAIL_CALLBACK,
    TYPE_COMPLETE_CALLBACK,
};

enum UploadErrorCode {
    UPLOAD_OK = 0,
    UPLOAD_ERRORCODE_UNSUPPORT_URI,
    UPLOAD_ERRORCODE_GET_FILE_ERROR,
    UPLOAD_ERRORCODE_CONFIG_ERROR,
    UPLOAD_ERRORCODE_UPLOAD_LIB_ERROR,
    UPLOAD_ERRORCODE_UPLOAD_FAIL,
    UPLOAD_ERRORCODE_UPLOAD_OUTTIME,
};

struct TaskResult {
    uint32_t successCount {0};
    uint32_t failCount {0};
    int32_t errorCode {UPLOAD_OK};
};

struct TaskState {
    std::string path;
    int32_t responseCode {UPLOAD_OK};
    std::string message;
};

#ifndef UPLOAD_API
#define UPLOAD_API __attribute__ ((visibility ("default")))
#endif
} // end of OHOS::Request::Upload
#endif