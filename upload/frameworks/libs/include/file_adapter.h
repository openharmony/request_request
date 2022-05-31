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

#ifndef FILE_ADAPTER
#define FILE_ADAPTER

#include "i_file_adapter.h"

namespace OHOS::Request::Upload {
class FileAdapter : public IFileAdapter {
public:
    uint32_t DataAbilityOpenFile(std::string &fileUri,
        std::shared_ptr<OHOS::AbilityRuntime::Context> &context);
    std::string InternalGetFilePath(std::shared_ptr<OHOS::AbilityRuntime::Context> &context);
};
} // end of OHOS::Request::Upload
#endif