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

#define private public
#define protected public
#include "curl_adp.h"

#include <fcntl.h>
#include <gtest/gtest.h>
#include <string>
#include <fcntl.h>
#include "upload_hilog_wrapper.h"
#include "upload_config.h"
#include "curl_adp.h"
#include "upload_task.h"
#include "upload_config.h"
#include "upload_hilog_wrapper.h"
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
 * @tc.name: UploadTest.PostUploadNetworkOff001
 * @tc.desc: Use post to upload files when the network is off.
 * @tc.type: FUNC
 */
HWTEST_F(UploadTest, PostUploadNetworkOff001, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********PostUploadNetworkOff001**in**********");
    FileData fileData;
    fileData.name = "upload_UT_test_1.xml";
    std::vector<FileData> fileDatas;
    fileDatas.push_back(fileData);

    std::shared_ptr<UploadConfig> uploadConfig = std::make_shared<UploadConfig>();
    uploadConfig->url = "http://192.168.1.180/uploadservice/";
    uploadConfig->method = "POST";
    std::vector<std::string> header;
    std::string str = "Content-Type:multipart/form-data";
    header.push_back(str);
    uploadConfig->header = header;

    auto curl = std::make_shared<CUrlAdp>(fileDatas, uploadConfig);
    uint32_t ret = curl->DoUpload(nullptr);
    EXPECT_EQ(ret, UPLOAD_ERRORCODE_UPLOAD_FAIL);
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********PostUploadNetworkOff001***out**********");
}

/**
 * @tc.name: UploadTest.PostUploadNetworkOff002
 * @tc.desc: Upload after removing the task.
 * @tc.type: FUNC
 */
HWTEST_F(UploadTest, PostUploadNetworkOff002, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********PostUploadNetworkOff002**in**********");
    FileData fileData;
    fileData.type = "xml";
    std::vector<FileData> fileDatas;
    fileDatas.push_back(fileData);

    std::shared_ptr<UploadConfig> uploadConfig = std::make_shared<UploadConfig>();
    uploadConfig->url = "http://192.168.1.180/uploadservice/";
    uploadConfig->method = "POST";
    std::vector<RequestData> data;
    RequestData requestData;
    requestData.name = "upload";
    requestData.value = "value";
    data.push_back(requestData);
    uploadConfig->data = data;

    auto curl = std::make_shared<CUrlAdp>(fileDatas, uploadConfig);
    uint32_t ret = curl->DoUpload(nullptr);
    EXPECT_EQ(ret, UPLOAD_ERRORCODE_UPLOAD_FAIL);
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********PostUploadNetworkOff002***out**********");
}

/**
 * @tc.name: UploadTest.PutUploadNetworkOff001
 * @tc.desc: Use put to upload files when the network is off.
 * @tc.type: FUNC
 */
HWTEST_F(UploadTest, PutUploadNetworkOff001, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********PutUploadNetworkOff001**in**********");
    FileData fileData;
    std::vector<FileData> fileDatas;
    fileDatas.push_back(fileData);

    std::shared_ptr<UploadConfig> uploadConfig = std::make_shared<UploadConfig>();
    uploadConfig->url = "http://192.168.1.180/uploadservice/";
    uploadConfig->method = "PUT";
    uploadConfig->protocolVersion = "API5";
    std::vector<std::string> header;
    std::string str = "multipart/form-data";
    header.push_back(str);
    uploadConfig->header = header;

    auto curl = std::make_shared<CUrlAdp>(fileDatas, uploadConfig);
    uint32_t ret = curl->DoUpload(nullptr);
    EXPECT_EQ(ret, UPLOAD_ERRORCODE_UPLOAD_FAIL);
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********PutUploadNetworkOff001***out**********");
}

/**
 * @tc.name: UploadTest.PutUploadNetworkOff002
 * @tc.desc: Use put to upload files when the network is off.
 * @tc.type: FUNC
 */
HWTEST_F(UploadTest, PutUploadNetworkOff002, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********PutUploadNetworkOff002**in**********");
    std::vector<FileData> fileDatas;
    std::shared_ptr<UploadConfig> uploadConfig = std::make_shared<UploadConfig>();
    auto curl = std::make_shared<CUrlAdp>(fileDatas, uploadConfig);
    uint32_t ret = curl->DoUpload(nullptr);
    EXPECT_EQ(ret, UPLOAD_ERRORCODE_UPLOAD_FAIL);
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********PutUploadNetworkOff002***out**********");
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
    auto curl = std::make_shared<CUrlAdp>(fileDatas, uploadConfig);
    bool res = curl->Remove();
    EXPECT_EQ(res, true);

    uint32_t ret = curl->DoUpload(nullptr);
    EXPECT_EQ(ret, UPLOAD_ERRORCODE_UPLOAD_FAIL);
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********UploadAfterRemoveTask***out**********");
}

/**
 * @tc.name: UploadTest.ProgressCallback001
 * @tc.desc: Upload progress callback test.
 * @tc.type: FUNC
 */
HWTEST_F(UploadTest, ProgressCallback001, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ProgressCallback001**in**********");
    std::vector<FileData> fileDatas;
    std::shared_ptr<UploadConfig> uploadConfig = std::make_shared<UploadConfig>();
    auto curl = std::make_shared<CUrlAdp>(fileDatas, uploadConfig);
    curl->DoUpload(nullptr);

    FileData *fData = new FileData();
    fData->adp = curl;
    int ret = CUrlAdp::ProgressCallback(static_cast<void *> (fData), 10, 3, 10, 3);
    EXPECT_EQ(ret, 0);
    delete fData;
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ProgressCallback001***out**********");
}

/**
 * @tc.name: UploadTest.ProgressCallback002
 * @tc.desc: Upload progress callback test.
 * @tc.type: FUNC
 */
HWTEST_F(UploadTest, ProgressCallback002, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ProgressCallback002**in**********");
    std::vector<FileData> fileDatas;
    std::shared_ptr<UploadConfig> uploadConfig = std::make_shared<UploadConfig>();
    auto curl = std::make_shared<CUrlAdp>(fileDatas, uploadConfig);
    curl->DoUpload(nullptr);

    FileData *fData = new FileData();
    fData->adp = curl;
    int ret = CUrlAdp::ProgressCallback(static_cast<void *> (fData), 9, 3, 9, 0);
    EXPECT_EQ(ret, 0);
    delete fData;
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ProgressCallback002***out**********");
}

/**
 * @tc.name: UploadTest.ProgressCallback003
 * @tc.desc: Upload progress callback test.
 * @tc.type: FUNC
 */
HWTEST_F(UploadTest, ProgressCallback003, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ProgressCallback003**in**********");
    std::vector<FileData> fileDatas;
    std::shared_ptr<UploadConfig> uploadConfig = std::make_shared<UploadConfig>();
    auto curl = std::make_shared<CUrlAdp>(fileDatas, uploadConfig);
    curl->Remove();

    FileData *fData = new FileData();
    fData->adp = curl;
    int ret = CUrlAdp::ProgressCallback(static_cast<void *> (fData), 10, 3, 10, 0);
    EXPECT_EQ(ret, UPLOAD_ERRORCODE_UPLOAD_FAIL);
    delete fData;
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ProgressCallback003***out**********");
}

/**
 * @tc.name: UploadTest.ProgressCallback004
 * @tc.desc: Upload progress callback test.
 * @tc.type: FUNC
 */
HWTEST_F(UploadTest, ProgressCallback004, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ProgressCallback004**in**********");
    FileData *fData = new FileData();
    fData->adp = nullptr;
    int ret = CUrlAdp::ProgressCallback(static_cast<void *> (fData), 0, 0, 0, 0);
    EXPECT_EQ(ret, UPLOAD_ERRORCODE_UPLOAD_FAIL);
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ProgressCallback004***out**********");
}

/**
 * @tc.name: UploadTest.HeaderCallback001
 * @tc.desc: Uoload header callback test.
 * @tc.type: FUNC
 */
HWTEST_F(UploadTest, HeaderCallback001, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********HeaderCallback001**in**********");
    std::vector<FileData> fileDatas;
    std::shared_ptr<UploadConfig> uploadConfig = std::make_shared<UploadConfig>();
    auto curl = std::make_shared<CUrlAdp>(fileDatas, uploadConfig);
    curl->DoUpload(nullptr);

    FileData *fData = new FileData();
    fData->adp = curl;
    char str[] = "/1.1 200";
    size_t ret = CUrlAdp::HeaderCallback(str, 1, 8, static_cast<void *> (fData));
    EXPECT_EQ(ret, 8);
    delete fData;
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********HeaderCallback001***out**********");
}

/**
 * @tc.name: UploadTest.HeaderCallback002
 * @tc.desc: Uoload header callback test.
 * @tc.type: FUNC
 */
HWTEST_F(UploadTest, HeaderCallback002, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********HeaderCallback002**in**********");
    std::vector<FileData> fileDatas;
    std::shared_ptr<UploadConfig> uploadConfig = std::make_shared<UploadConfig>();
    auto curl = std::make_shared<CUrlAdp>(fileDatas, uploadConfig);

    FileData *fData = new FileData();
    fData->adp = curl;
    char str[] = "HTTP/1.1 200 OK\r\n";
    size_t ret = CUrlAdp::HeaderCallback(str, 1, 19, static_cast<void *> (fData));
    EXPECT_EQ(ret, 19);
    delete fData;
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********HeaderCallback002***out**********");
}

/**
 * @tc.name: UploadTest.HeaderCallback003
 * @tc.desc: Uoload header callback test.
 * @tc.type: FUNC
 */
HWTEST_F(UploadTest, HeaderCallback003, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********HeaderCallback003**in**********");
    std::vector<FileData> fileDatas;
    std::shared_ptr<UploadConfig> uploadConfig = std::make_shared<UploadConfig>();
    auto curl = std::make_shared<CUrlAdp>(fileDatas, uploadConfig);

    FileData *fData = new FileData();
    fData->adp = curl;
    char str[] = "\r\n";
    size_t ret = CUrlAdp::HeaderCallback(str, 1, 4, static_cast<void *> (fData));
    EXPECT_EQ(ret, 4);
    delete fData;
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********HeaderCallback003***out**********");
}

/**
 * @tc.name: UploadTest.HeaderCallback004
 * @tc.desc: Uoload header callback test.
 * @tc.type: FUNC
 */
HWTEST_F(UploadTest, HeaderCallback004, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********HeaderCallback004**in**********");
    FileData *fData = new FileData();
    fData->adp = nullptr;
    size_t ret = CUrlAdp::HeaderCallback(nullptr, 0, 0, static_cast<void *> (fData));
    EXPECT_EQ(ret, CURLE_WRITE_ERROR);
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********HeaderCallback004***out**********");
}

/**
 * @tc.name: UploadTest.ReadCallback001
 * @tc.desc: Uoload read callback test.
 * @tc.type: FUNC
 */
HWTEST_F(UploadTest, ReadCallback001, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ReadCallback001**in**********");
    FileData *fData = new FileData();
    fData->adp = nullptr;
    size_t ret = CUrlAdp::ReadCallback(nullptr, 0, 0, static_cast<void *> (fData));
    EXPECT_EQ(ret, CURL_READFUNC_ABORT);
    delete fData;
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ReadCallback001***out**********");
}

/**
 * @tc.name: UploadTest.ReadCallback002
 * @tc.desc: Uoload read callback test.
 * @tc.type: FUNC
 */
HWTEST_F(UploadTest, ReadCallback002, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ReadCallback002**in**********");
    std::vector<FileData> fileDatas;
    std::shared_ptr<UploadConfig> uploadConfig = std::make_shared<UploadConfig>();
    auto curl = std::make_shared<CUrlAdp>(fileDatas, uploadConfig);

    FileData *fData = new FileData();
    fData->adp = curl;
    FILE *fp = fopen("file.txt", "w");
    fData->fp = fp;
    size_t ret = CUrlAdp::ReadCallback(nullptr, 0, 0, static_cast<void *> (fData));
    EXPECT_EQ(ret, 0);
    fclose(fp);
    delete fData;
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ReadCallback002***out**********");
}
} // end of OHOS::Request::Upload