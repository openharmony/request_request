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

namespace OHOS::Request::Upload {
enum Type {
    TYPE_PROGRESS_CALLBACK,
    TYPE_HEADER_RECEIVE_CALLBACK,
    TYPE_FAIL_CALLBACK,
};

struct TaskResult {
    int32_t successCount {};
    int32_t failCount {};
    int32_t errorCode {};
};

#ifndef UPLOAD_API
#define UPLOAD_API __attribute__ ((visibility ("default")))
#endif
} // end of OHOS::Request::Upload
#endif