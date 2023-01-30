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

#ifndef OHOS_REQUEST_UPLOAD_I_FILE_ADAPTER
#define OHOS_REQUEST_UPLOAD_I_FILE_ADAPTER

#include <stdio.h>
#include "context.h"
#include "ability_context.h"
#include "data_ability_helper.h"

namespace OHOS::Request::Upload {
class IFileAdapter {
public:
    virtual ~IFileAdapter() {};
    virtual int32_t DataAbilityOpenFile(std::string &fileUri,
                                         std::shared_ptr<OHOS::AbilityRuntime::Context> &context) = 0;
    virtual std::string InternalGetFilePath(std::shared_ptr<OHOS::AbilityRuntime::Context> &context) = 0;
};
} // end of OHOS::Request::Upload
#endif