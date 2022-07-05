/*
 * Copyright (C) 2022 Huawei Device Co., Ltd.
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

#include <gtest/gtest.h>
#include <string>
#include <fcntl.h>
#include "upload_hilog_wrapper.h"
#include "upload_config.h"
#include "curl_adp.h"
#include "upload_test.h"

using namespace testing::ext;
namespace OHOS::Request::Upload {
class UploadTest : public testing::Test {
public:
    static void SetUpTestCase(void);

    static void TearDownTestCase(void);

    void SetUp();

    void TearDown();
};

void UploadTest::SetUpTestCase(void)
{
}

void UploadTest::TearDownTestCase(void)
{
}

void UploadTest::SetUp()
{
}

void UploadTest::TearDown()
{
}

/**
 * @tc.name: UploadTest.UploadTestTest_001
 * @tc.type: FUNC
 */
HWTEST_F(UploadTest, UploadTest_001, TestSize.Level1)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********UploadUtTest_001**in**********");
    FILE *fd1;
    FILE *fd2;
    FileData fileInfo1;
    FileData fileInfo2;
    std::vector<FileData> fileArray;
    std::shared_ptr<UploadConfig> uploadConfig = std::make_shared<UploadConfig>();

    fd1 = fopen("upload_UT_test_1.xml", "rb");
    fd2 = fopen("upload_UT_test_2.xml", "rb");
    fileInfo1.fp = fd1;
    fileInfo1.name = "upload_UT_test_1.xml";
    fileInfo2.fp = fd2;
    fileInfo2.name = "upload_UT_test_2.xml";
    if (fd1 && fd2) {
        // url needs to be configured according to the server URL
        uploadConfig->url = "http://192.168.1.180/uploadservice/";
        fileArray.push_back(fileInfo1);
        fileArray.push_back(fileInfo2);
        auto curl = std::make_shared<CUrlAdp>(fileArray, uploadConfig);
        TaskResult taskResult = {0};
        curl->DoUpload(nullptr, taskResult);
    } else {
        UPLOAD_HILOGE(UPLOAD_MODULE_TEST, "open file failed");
    }
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********UploadUtTest_001***out**********");
    SUCCEED();
}
} // end of OHOS::Request::Upload