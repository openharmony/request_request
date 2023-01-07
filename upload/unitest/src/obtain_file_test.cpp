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
#include <gtest/gtest.h>
#include "mock_obfile.h"
#include "ability.h"
#include "mock_file_adapter.h"
#include "obtain_file.h"
#include "file_adapter.h"
#include "obtain_file_test.h"


using namespace OHOS::AppExecFwk;
using namespace OHOS::AbilityRuntime;
using namespace testing::ext;
namespace OHOS::Request::Upload {
class ObtainFileTest : public testing::Test {
public:
    static void SetUpTestCase(void);

    static void TearDownTestCase(void);

    void SetUp();

    void TearDown();

    std::shared_ptr<MockFileAdapter> adapter;
    std::shared_ptr<ObtainFile> obfile;
};

void ObtainFileTest::SetUpTestCase(void)
{
}

void ObtainFileTest::TearDownTestCase(void)
{
}

void ObtainFileTest::SetUp()
{
    this->obfile = std::make_shared<ObtainFile>();
    this->obfile->fileAdapter_.reset();
    this->adapter = std::make_shared<MockFileAdapter>();
    this->obfile->fileAdapter_ = this->adapter;
}

void ObtainFileTest::TearDown()
{
    this->obfile->fileAdapter_.reset();
    this->obfile.reset();
    this->adapter.reset();
}

/**
 * @tc.name: ObtainFileUtTest001
 * @tc.desc: GetFile with DataAbilityUri succsee
 * @tc.type: FUNC
 */
HWTEST_F(ObtainFileTest, ObtainFileUtTest001, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ObtainFileUtTest001**in**********");
    FILE *file = nullptr;
    unsigned int fileSize = 0;
    unsigned int result = UPLOAD_OK;
    std::string uri = "dataability:///com.domainname.dataability.persondata/person/10";
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;

    EXPECT_CALL(*(this->adapter.get()), DataAbilityOpenFile(testing::_, testing::_))
        .Times(1)
        .WillOnce(testing::Return(-1));
    result = this->obfile->GetFile(&file, uri, fileSize, context);
    EXPECT_EQ(result, UPLOAD_ERRORCODE_GET_FILE_ERROR);
    EXPECT_EQ(file, nullptr);
    EXPECT_EQ(fileSize, 0);
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ObtainFileUtTest001***out**********");
}

/**
 * @tc.name: ObtainFileUtTest002
 * @tc.desc: GetFile with DataAbilityUri success
 * @tc.type: FUNC
 */
HWTEST_F(ObtainFileTest, ObtainFileUtTest002, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ObtainFileUtTest002**in**********");
    std::string testFileInfo = "test date for dataability file.";
    std::string createCachePathCommend = "mkdir -p /data/Dataability/";
    std::string createCacheFileCommend = "touch /data/Dataability/file.txt";
    std::string writFileCommend = "echo '" + testFileInfo + "' >/data/Dataability/file.txt";
    std::string deleteCacheFileCommend = "rm -rf /data/Dataability/";
    system(createCachePathCommend.c_str());
    system(createCacheFileCommend.c_str());
    system(writFileCommend.c_str());

    FILE* file = nullptr;
    unsigned int fileSize = 0;
    unsigned int result = UPLOAD_ERRORCODE_GET_FILE_ERROR;
    std::string uri = "dataability:///com.domainname.dataability.persondata/person/10";
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;
    uint32_t fd = fileno(fopen("/data/Dataability/file.txt", "r"));

    EXPECT_CALL(*(this->adapter.get()), DataAbilityOpenFile(testing::_, testing::_))
        .Times(1)
        .WillOnce(testing::Return(fd));
    
    result = this->obfile->GetFile(&file, uri, fileSize, context);

    EXPECT_EQ(result, UPLOAD_OK);
    EXPECT_NE(file, nullptr);
    EXPECT_EQ(fileSize, testFileInfo.size() + 1);
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ObtainFileUtTest002***out**********");
    fclose(file);
    system(deleteCacheFileCommend.c_str());
}

/**
 * @tc.name: ObtainFileUtTest003
 * @tc.desc: GetFile with DataAbilityUri success
 * @tc.type: FUNC
 */
HWTEST_F(ObtainFileTest, ObtainFileUtTest003, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ObtainFileUtTest003**in**********");
    FILE *file = nullptr;
    unsigned int fileSize = 0;
    unsigned int result = UPLOAD_ERRORCODE_GET_FILE_ERROR;
    std::string uri = "internal:--//cache/path/to/file.txt";
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;

    result = this->obfile->GetFile(&file, uri, fileSize, context);

    EXPECT_EQ(result, UPLOAD_ERRORCODE_UNSUPPORT_URI);
    EXPECT_EQ(file, nullptr);
    EXPECT_EQ(fileSize, 0);
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ObtainFileUtTest003***out**********");
}
/**
 * @tc.name: ObtainFileUtTest004
 * @tc.desc: GetFile with DataAbilityUri success
 * @tc.type: FUNC
 */
HWTEST_F(ObtainFileTest, ObtainFileUtTest004, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ObtainFileUtTest004**in**********");
    FILE *file = nullptr;
    unsigned int fileSize = 0;
    unsigned int result = UPLOAD_ERRORCODE_GET_FILE_ERROR;
    std::string uri = "internal:/ccc/cache/path/to/file.txt";
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;

    result = this->obfile->GetFile(&file, uri, fileSize, context);

    EXPECT_EQ(result, UPLOAD_ERRORCODE_UNSUPPORT_URI);
    EXPECT_EQ(file, nullptr);
    EXPECT_EQ(fileSize, 0);
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ObtainFileUtTest004***out**********");
}
/**
 * @tc.name: ObtainFileUtTest005
 * @tc.desc: GetFile with DataAbilityUri success
 * @tc.type: FUNC
 */
HWTEST_F(ObtainFileTest, ObtainFileUtTest005, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ObtainFileUtTest005**in**********");
    FILE *file = nullptr;
    unsigned int fileSize = 0;
    unsigned int result = UPLOAD_ERRORCODE_GET_FILE_ERROR;
    std::string uri = "internal://cache---/path/to/file.txt";
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;

    result = this->obfile->GetFile(&file, uri, fileSize, context);

    EXPECT_EQ(result, UPLOAD_ERRORCODE_UNSUPPORT_URI);
    EXPECT_EQ(file, nullptr);
    EXPECT_EQ(fileSize, 0);
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ObtainFileUtTest005***out**********");
}

/**
 * @tc.name: ObtainFileUtTest006
 * @tc.desc: GetFile with DataAbilityUri success
 * @tc.type: FUNC
 */
HWTEST_F(ObtainFileTest, ObtainFileUtTest006, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ObtainFileUtTest006**in**********");
    FILE *file = nullptr;
    unsigned int fileSize = 0;
    unsigned int result = UPLOAD_ERRORCODE_GET_FILE_ERROR;
    std::string uri = "internal://cache/path/to/file.txt";
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;
    std::string path = "";

    EXPECT_CALL(*(this->adapter.get()), InternalGetFilePath(testing::_))
        .Times(1)
        .WillOnce(testing::Return(path));

    result = this->obfile->GetFile(&file, uri, fileSize, context);

    EXPECT_EQ(result, UPLOAD_ERRORCODE_GET_FILE_ERROR);
    EXPECT_EQ(file, nullptr);
    EXPECT_EQ(fileSize, 0);
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ObtainFileUtTest006***out**********");
}
/**
 * @tc.name: ObtainFileUtTest007
 * @tc.desc: GetFile with DataAbilityUri success
 * @tc.type: FUNC
 */
HWTEST_F(ObtainFileTest, ObtainFileUtTest007, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ObtainFileUtTest007**in**********");
    FILE *file = nullptr;
    unsigned int fileSize = 0;
    unsigned int result = UPLOAD_ERRORCODE_GET_FILE_ERROR;
    std::string uri = "internal://cache/path/to/file.txt";
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;
    std::string path = "XXXXX";

    EXPECT_CALL(*(this->adapter.get()), InternalGetFilePath(testing::_))
        .Times(1)
        .WillOnce(testing::Return(path));

    result = this->obfile->GetFile(&file, uri, fileSize, context);

    EXPECT_EQ(result, UPLOAD_ERRORCODE_GET_FILE_ERROR);
    EXPECT_EQ(file, nullptr);
    EXPECT_EQ(fileSize, 0);
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ObtainFileUtTest007***out**********");
}
/**
 * @tc.name: ObtainFileUtTest008
 * @tc.desc: GetFile with DataAbilityUri success
 * @tc.type: FUNC
 */
HWTEST_F(ObtainFileTest, ObtainFileUtTest008, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ObtainFileUtTest008**in**********");
    std::string testFileInfo = "test date for internal file.";
    std::string createCachePathCommend = "mkdir -p /data/testApp/CacheDir/path/to/";
    std::string createCacheFileCommend = "touch /data/testApp/CacheDir/path/to/file.txt";
    std::string writFileCommend = "echo '" + testFileInfo + "' >/data/testApp/CacheDir/path/to/file.txt";
    std::string deleteCacheFileCommend = "rm -rf /data/testApp/CacheDir/";
    system(createCachePathCommend.c_str());
    system(createCacheFileCommend.c_str());
    system(writFileCommend.c_str());

    FILE* file = nullptr;
    unsigned int fileSize = 0;
    unsigned int result = UPLOAD_ERRORCODE_GET_FILE_ERROR;
    std::string uri = "internal://cache/path/to/file.txt";
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;
    std::string path = "/data/testApp/CacheDir";

    EXPECT_CALL(*(this->adapter.get()), InternalGetFilePath(testing::_))
        .Times(1)
        .WillOnce(testing::Return(path));

    result = this->obfile->GetFile(&file, uri, fileSize, context);

    EXPECT_EQ(result, UPLOAD_OK);
    EXPECT_NE(file, nullptr);
    EXPECT_EQ(fileSize, testFileInfo.size() + 1);
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ObtainFileUtTest008***out**********");

    fclose(file);
    system(deleteCacheFileCommend.c_str());
}
/**
 * @tc.name: ObtainFileUtTest009
 * @tc.desc: GetFile with DataAbilityUri success
 * @tc.type: FUNC
 */
HWTEST_F(ObtainFileTest, ObtainFileUtTest009, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ObtainFileUtTest009**in**********");
    FILE *file = nullptr;
    unsigned int fileSize = 0;
    unsigned int result = UPLOAD_ERRORCODE_GET_FILE_ERROR;
    std::string uri = "XXXXXXXXXXXXXX://cache/path/to/file.txt";
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;

    result = this->obfile->GetFile(&file, uri, fileSize, context);

    EXPECT_EQ(result, UPLOAD_ERRORCODE_UNSUPPORT_URI);
    EXPECT_EQ(file, nullptr);
    EXPECT_EQ(fileSize, 0);
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ObtainFileUtTest009***out**********");
}

/**
 * @tc.name: ObtainFileUtTest010
 * @tc.desc: GetFile with DataAbilityUri succsee
 * @tc.type: FUNC
 */
HWTEST_F(ObtainFileTest, ObtainFileUtTest010, TestSize.Level0)
{
    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ObtainFileUtTest010**in**********");

    std::shared_ptr<IFileAdapter> fileAdapter = std::make_shared<FileAdapter>();
    std::string uri = "dataability:///com.domainname.dataability.persondata/person/10";
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;
    int32_t fd = fileAdapter->DataAbilityOpenFile(uri, context);
    EXPECT_EQ(fd, -1);

    UPLOAD_HILOGD(UPLOAD_MODULE_TEST, "**********ObtainFileUtTest010**in**********");
}
}  // namespace OHOS::Request::Upload
