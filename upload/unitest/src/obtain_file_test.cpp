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

#include "obtain_file_test.h"
#include <gtest/gtest.h>
#include "ability.h"

using namespace OHOS::AppExecFwk;
using namespace testing::ext;
namespace OHOS::Request::Upload {
class ObtainFileTest : public testing::Test {
public:
    static void SetUpTestCase(void);

    static void TearDownTestCase(void);

    void SetUp();

    void TearDown();
};

void ObtainFileTest::SetUpTestCase(void)
{
}

void ObtainFileTest::TearDownTestCase(void)
{
}

void ObtainFileTest::SetUp()
{
}

void ObtainFileTest::TearDown()
{
}

/**
 * @tc.name: ObtainFileUtTest001
 * @tc.desc: GetFile with DataAbilityUri succsee
 * @tc.type: FUNC
 */
HWTEST_F(ObtainFileTest, ObtainFileUtTest001, TestSize.Level0)
{
    unsigned int fileSize = 0;
    unsigned int result = UPLOAD_ERRORCODE_NO_ERROR;
    FILE* file;

    std::string testFileInfo = "test date for dataability file.";
    std::string createCachePathCommend = "mkdir -p /data/Dataability/";
    std::string createCacheFileCommend = "touch /data/Dataability/file.txt";
    std::string writFileCommend = "echo '" + testFileInfo + "' >/data/Dataability/file.txt";
    std::string deleteCacheFileCommend = "rm -rf /data/Dataability/";
    system(createCachePathCommend.c_str());
    system(createCacheFileCommend.c_str());
    system(writFileCommend.c_str());

    std::string uri = "dataability:///com.domainname.dataability.persondata/person/10";
    MockObtainFile obtainFile;
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;

    result = obtainFile.GetFile(&file, uri, fileSize, context);
    EXPECT_EQ(result, UPLOAD_ERRORCODE_NO_ERROR) << "GetFile fun ret failed.";
    EXPECT_NE(file, nullptr) << "GetFile filePtr is NULL";
    EXPECT_EQ(fileSize, testFileInfo.size()+1) << "GetFile size failed.";

    if (file != nullptr) {
        fclose(file);
    }
    system(deleteCacheFileCommend.c_str());
}

/**
 * @tc.name: ObtainFileUtTest002
 * @tc.desc: GetFile with InternalUri succsee
 * @tc.type: FUNC
 */
HWTEST_F(ObtainFileTest, ObtainFileUtTest002, TestSize.Level0)
{
    unsigned int fileSize = 0;
    unsigned int result = UPLOAD_ERRORCODE_NO_ERROR;
    FILE* file;

    std::string uri = "internal://cache/path/to/file.txt";
    std::string testFileInfo = "test date for internal file.";
    std::string createCachePathCommend = "mkdir -p /data/testApp/CacheDir/path/to/";
    std::string createCacheFileCommend = "touch /data/testApp/CacheDir/path/to/file.txt";
    std::string writFileCommend = "echo '" + testFileInfo + "' >/data/testApp/CacheDir/path/to/file.txt";
    std::string deleteCacheFileCommend = "rm -rf /data/CacheDir/";
    system(createCachePathCommend.c_str());
    system(createCacheFileCommend.c_str());
    system(writFileCommend.c_str());

    std::string dir = "/data/testApp/CacheDir";
    ObtainFile obtainFile;

    std::shared_ptr<ApplicationInfo> info = std::make_shared<ApplicationInfo>();
    std::shared_ptr<ContextDeal> deal = std::make_shared<ContextDeal>();
    std::shared_ptr<AbilityContext> abilityContext = std::make_shared<AbilityContext>();
    info->cacheDir = dir;
    deal->SetApplicationInfo(info);
    abilityContext->AttachBaseContext(deal);
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;

    result = obtainFile.GetFile(&file, uri, fileSize, context);
    EXPECT_EQ(result, UPLOAD_ERRORCODE_NO_ERROR) << "GetFile fun ret failed.";
    EXPECT_NE(file, nullptr) << "GetFile filePtr is NULL";
    EXPECT_EQ(fileSize, testFileInfo.size()+1) << "GetFile size failed.";

    if (file != nullptr) {
        fclose(file);
    }
    system(deleteCacheFileCommend.c_str());
}

/**
 * @tc.name: ObtainFileUtTest003
 * @tc.desc: GetFile with DataAbilityUri fail (DataAbilityHelper->OpenFile error)
 * @tc.type: FUNC
 */
HWTEST_F(ObtainFileTest, ObtainFileUtTest003, TestSize.Level0)
{
    unsigned int fileSize = 0;
    unsigned int result = UPLOAD_ERRORCODE_NO_ERROR;
    FILE* file;

    std::string uri = "dataability:///com.domainname.dataability.persondata/person/10";
    ObtainFile obtainFile;
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;

    result = obtainFile.GetFile(&file, uri, fileSize, context);
    EXPECT_EQ(result, UPLOAD_ERRORCODE_GET_FILE_ERROR) << "GetFile fun ret failed.";
    EXPECT_EQ(file, nullptr) << "GetFile filePtr is NULL";
    EXPECT_EQ(fileSize, 0) << "GetFile size failed.";
}

/**
 * @tc.name: ObtainFileUtTest004
 * @tc.desc: GetFile with InternalUri fail (Context->GetCache error)
 * @tc.type: FUNC
 */
HWTEST_F(ObtainFileTest, ObtainFileUtTest004, TestSize.Level0)
{
    unsigned int fileSize = 0;
    unsigned int result = UPLOAD_ERRORCODE_NO_ERROR;
    FILE* file;

    std::string uri = "internal://cache/path/to/file.txt";
    ObtainFile obtainFile;
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;

    result = obtainFile.GetFile(&file, uri, fileSize, context);
    EXPECT_EQ(result, UPLOAD_ERRORCODE_GET_FILE_ERROR) << "GetFile fun ret failed.";
    EXPECT_EQ(file, nullptr) << "GetFile filePtr is NULL";
    EXPECT_EQ(fileSize, 0) << "GetFile size failed.";
}

/**
 * @tc.name: ObtainFileUtTest005
 * @tc.desc: GetFile with Wrong URI (Local Path)
 * @tc.type: FUNC
 */
HWTEST_F(ObtainFileTest, ObtainFileUtTest005, TestSize.Level0)
{
    unsigned int fileSize = 0;
    unsigned int result = UPLOAD_ERRORCODE_NO_ERROR;
    FILE* file;

    std::string uri = "/data/upload_obtain_file_UT_test";
    ObtainFile obtainFile;
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;
    result = obtainFile.GetFile(&file, uri, fileSize, context);
    EXPECT_EQ(result, UPLOAD_ERRORCODE_UNSUPPORT_URI) << "GetFile fun ret failed.";
    EXPECT_EQ(file, nullptr) << "GetFile filePtr is NULL";
    EXPECT_EQ(fileSize, 0) << "GetFile size failed.";
}
}
