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

#ifndef OBTAIN_FILE_
#define OBTAIN_FILE_

#include <stdio.h>
#include "context.h"
#include "ability_context.h"
#include "data_ability_helper.h"
#include "i_file_adapter.h"

#define SPLIT_ZERO 0
#define SPLIT_ONE 1
#define SPLIT_TWO 2
#define SPLIT_THREE 3

namespace OHOS::Request::Upload {
class ObtainFile {
public:
    ObtainFile();
    ~ObtainFile();
    uint32_t GetFile(FILE **file, std::string& fileUri,
        uint32_t& fileSize, std::shared_ptr<OHOS::AbilityRuntime::Context> &context);
protected:
    uint32_t GetDataAbilityFile(FILE **file, std::string& fileUri,
        uint32_t& fileSize, std::shared_ptr<OHOS::AbilityRuntime::Context> &context);
    uint32_t GetInternalFile(FILE **file, const std::string& fileUri,
        uint32_t& fileSize, std::shared_ptr<OHOS::AbilityRuntime::Context> &context);
    bool SplitPath(const std::string &fileUri, std::string &fileName);
    bool IsValidPath(const std::string &filePath);
private:
    std::shared_ptr<IFileAdapter> fileAdapter_;
};
} // end of OHOS::Request::Upload
#endif