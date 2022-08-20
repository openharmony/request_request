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

#include "mock_obtain_file.h"
#include <gtest/gtest.h>

using namespace OHOS::AppExecFwk;
namespace OHOS::Request::Upload {
uint32_t MockObtainFile::GetFile(FILE **file, std::string &fileUri,
    unsigned int& fileSize, std::shared_ptr<OHOS::AbilityRuntime::Context> &context)
{
    return GetDataAbilityFile(file, fileUri, fileSize, context);
}

uint32_t MockObtainFile::GetDataAbilityFile(FILE **file, std::string &fileUri,
    uint32_t& fileSize, std::shared_ptr<OHOS::AbilityRuntime::Context> &context)
{
    uint32_t ret = UPLOAD_OK;
    FILE *filePtr = nullptr;
    int32_t fileLength = 0;

    std::shared_ptr<Uri> uri = std::make_shared<Uri>(fileUri);
    std::shared_ptr<DataAbilityHelper> dataAbilityHelper = DataAbilityHelper::Creator(context, uri);

    do {
        // dataAbilityHelper->OpenFile unavailble, dummyStart
        int fd = -1;
        FILE *fp = fopen("/data/Dataability/file.txt", "r");
        if (fp == nullptr) {
            return -1;
        }
        fd = fileno(fp);
        if (fd == -1) {
            break;
        }
        // dummyEnd

        filePtr = fdopen(fd, "r");
        if (filePtr == nullptr) {
            break;
        }

        int fseekResult = fseek(filePtr, 0, SEEK_END);
        EXPECT_EQ(fseekResult, 0);
        fileLength = ftell(filePtr);
        fseekResult = fseek(filePtr, 0, SEEK_SET);
        EXPECT_EQ(fseekResult, 0);
    } while (0);

    *file = filePtr;
    fileSize = fileLength;
    return ret;
}
} // end of OHOS::Request::Upload