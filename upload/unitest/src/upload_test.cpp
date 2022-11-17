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
 * @tc.name: UploadTest.PostUploadNetworkOff
 * @tc.desc: Use post to upload files when the network is off.
 * @tc.type: FUNC
 */
HWTEST_F(UploadTest, PostUploadNetworkOff, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********PostUploadNetworkOff**in**********");
    FileData fileData;
    fileData.name = "upload_UT_test_1.xml";
    std::vector<FileData> fileDatas;
    fileDatas.push_back(fileData);

    std::shared_ptr<UploadConfig> uploadConfig = std::make_shared<UploadConfig>();
    uploadConfig->url = "http://192.168.1.180/uploadservice/";
    uploadConfig->method = "POST";

    auto curl = std::make_shared<CUrlAdp>(fileDatas, uploadConfig);
    uint32_t ret = curl->DoUpload(nullptr);
    EXPECT_EQ(ret, UPLOAD_ERRORCODE_UPLOAD_FAIL);
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********PostUploadNetworkOff***out**********");
}

/**
 * @tc.name: UploadTest.PutUploadNetworkOff
 * @tc.desc: Use put to upload files when the network is off.
 * @tc.type: FUNC
 */
HWTEST_F(UploadTest, PutUploadNetworkOff, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********PutUploadNetworkOff**in**********");
    FileData fileData;
    fileData.name = "upload_UT_test_1.xml";
    std::vector<FileData> fileDatas;
    fileDatas.push_back(fileData);

    std::shared_ptr<UploadConfig> uploadConfig = std::make_shared<UploadConfig>();
    uploadConfig->url = "http://192.168.1.180/uploadservice/";
    uploadConfig->method = "PUT";
    uploadConfig->protocolVersion = "API5";

    auto curl = std::make_shared<CUrlAdp>(fileDatas, uploadConfig);
    uint32_t ret = curl->DoUpload(nullptr);
    EXPECT_EQ(ret, UPLOAD_ERRORCODE_UPLOAD_FAIL);
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********PutUploadNetworkOff***out**********");
}

/**
 * @tc.name: UploadTest.UploadAfterRemoveTask
 * @tc.desc: Upload after removing the task.
 * @tc.type: FUNC
 */
HWTEST_F(UploadTest, UploadAfterRemoveTask, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********UploadAfterRemoveTask**in**********");
    FileData fileData;
    fileData.name = "upload_UT_test_1.xml";
    std::vector<FileData> fileDatas;
    fileDatas.push_back(fileData);

    std::shared_ptr<UploadConfig> uploadConfig = std::make_shared<UploadConfig>();
    uploadConfig->url = "http://192.168.1.180/uploadservice/";
    uploadConfig->method = "POST";

    auto curl = std::make_shared<CUrlAdp>(fileDatas, uploadConfig);
    bool res = curl->Remove();
    EXPECT_EQ(res, true);
    uint32_t ret = curl->DoUpload(nullptr);
    EXPECT_EQ(ret, UPLOAD_ERRORCODE_UPLOAD_FAIL);
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********UploadAfterRemoveTask***out**********");
}
} // end of OHOS::Request::Upload