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
#include "mock_obfile.h"
#include "ability.h"
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
HWTEST_F(ObtainFileTest, ObtainFileUtTest000, TestSize.Level0)
{
    GTEST_LOG_(INFO) << "ObtainFileUtTest000 start";
    FILE* file;
    unsigned int fileSize = 0;
    unsigned int result = UPLOAD_OK;
    std::string uri = "dataability:///com.domainname.dataability.persondata/person/10";
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;

    std::shared_ptr<MockObfile> mockObfile = std::make_shared<MockObfile>();
    EXPECT_CALL(*mockObfile, GetFile(testing::_, testing::_, testing::_, testing::_))
        .Times(1)
        .WillOnce(testing::Return(0));
    result = mockObfile->GetFile(&file, uri, fileSize, context);
    EXPECT_EQ(result, UPLOAD_OK);
    GTEST_LOG_(INFO) << "ObtainFileUtTest000 end";
}

/**
 * @tc.name: ObtainFileUtTest001
 * @tc.desc: GetFile with DataAbilityUri succsee
 * @tc.type: FUNC
 */
HWTEST_F(ObtainFileTest, ObtainFileUtTest001, TestSize.Level0)
{
    GTEST_LOG_(INFO) << "ObtainFileUtTest001 start";
    FILE* file;
    unsigned int fileSize = 0;
    unsigned int result = UPLOAD_ERRORCODE_GET_FILE_ERROR;
    std::string uri = "dataability:///com.domainname.dataability.persondata/person/10";
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;

    std::shared_ptr<MockObfile> mockObfile = std::make_shared<MockObfile>();
    EXPECT_CALL(*mockObfile, GetFile(testing::_, testing::_, testing::_, testing::_))
        .Times(1)
        .WillOnce(testing::Return(2));
    result = mockObfile->GetFile(&file, uri, fileSize, context);
    EXPECT_EQ(result, UPLOAD_ERRORCODE_GET_FILE_ERROR);
    GTEST_LOG_(INFO) << "ObtainFileUtTest001 end";
}

/**
 * @tc.name: ObtainFileUtTest001
 * @tc.desc: GetFile with DataAbilityUri succsee
 * @tc.type: FUNC
 */
HWTEST_F(ObtainFileTest, ObtainFileUtTest002, TestSize.Level0)
{
    GTEST_LOG_(INFO) << "ObtainFileUtTest002 start";
    FILE* file;
    unsigned int fileSize = 0;
    unsigned int result = UPLOAD_OK;
    std::string uri = "dataability:///com.domainname.dataability.persondata/person/10";
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;

    std::shared_ptr<MockObfile> mockObfile = std::make_shared<MockObfile>();
    EXPECT_CALL(*mockObfile, GetDataAbilityFile(testing::_, testing::_, testing::_, testing::_))
        .Times(1)
        .WillOnce(testing::Return(0));
    result = mockObfile->GetDataAbilityFile(&file, uri, fileSize, context);
    EXPECT_EQ(result, UPLOAD_OK);
    GTEST_LOG_(INFO) << "ObtainFileUtTest002 end";
}

/**
 * @tc.name: ObtainFileUtTest001
 * @tc.desc: GetFile with DataAbilityUri succsee
 * @tc.type: FUNC
 */
HWTEST_F(ObtainFileTest, ObtainFileUtTest003, TestSize.Level0)
{
    GTEST_LOG_(INFO) << "ObtainFileUtTest003 start";
    FILE* file;
    unsigned int fileSize = 0;
    unsigned int result = UPLOAD_ERRORCODE_GET_FILE_ERROR;
    std::string uri = "dataability:///com.domainname.dataability.persondata/person/10";
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;

    std::shared_ptr<MockObfile> mockObfile = std::make_shared<MockObfile>();
    EXPECT_CALL(*mockObfile, GetDataAbilityFile(testing::_, testing::_, testing::_, testing::_))
        .Times(1)
        .WillOnce(testing::Return(2));
    result = mockObfile->GetDataAbilityFile(&file, uri, fileSize, context);
    EXPECT_EQ(result, UPLOAD_ERRORCODE_GET_FILE_ERROR);
    GTEST_LOG_(INFO) << "ObtainFileUtTest003 end";
}

/**
 * @tc.name: ObtainFileUtTest001
 * @tc.desc: GetFile with DataAbilityUri succsee
 * @tc.type: FUNC
 */
HWTEST_F(ObtainFileTest, ObtainFileUtTest004, TestSize.Level0)
{
    GTEST_LOG_(INFO) << "ObtainFileUtTest004 start";
    FILE* file;
    unsigned int fileSize = 0;
    unsigned int result = UPLOAD_OK;
    std::string uri = "dataability:///com.domainname.dataability.persondata/person/10";
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;

    std::shared_ptr<MockObfile> mockObfile = std::make_shared<MockObfile>();
    EXPECT_CALL(*mockObfile, GetInternalFile(testing::_, testing::_, testing::_, testing::_))
        .Times(1)
        .WillOnce(testing::Return(0));
    result = mockObfile->GetInternalFile(&file, uri, fileSize, context);
    EXPECT_EQ(result, UPLOAD_OK);
    GTEST_LOG_(INFO) << "ObtainFileUtTest004 end";
}

/**
 * @tc.name: ObtainFileUtTest001
 * @tc.desc: GetFile with DataAbilityUri succsee
 * @tc.type: FUNC
 */
HWTEST_F(ObtainFileTest, ObtainFileUtTest005, TestSize.Level0)
{
    GTEST_LOG_(INFO) << "ObtainFileUtTest005 start";
    FILE* file;
    unsigned int fileSize = 0;
    unsigned int result = UPLOAD_ERRORCODE_GET_FILE_ERROR;
    std::string uri = "dataability:///com.domainname.dataability.persondata/person/10";
    std::shared_ptr<OHOS::AbilityRuntime::Context> context = nullptr;

    std::shared_ptr<MockObfile> mockObfile = std::make_shared<MockObfile>();
    EXPECT_CALL(*mockObfile, GetInternalFile(testing::_, testing::_, testing::_, testing::_))
        .Times(1)
        .WillOnce(testing::Return(2));
    result = mockObfile->GetInternalFile(&file, uri, fileSize, context);
    EXPECT_EQ(result, UPLOAD_ERRORCODE_GET_FILE_ERROR);
    GTEST_LOG_(INFO) << "ObtainFileUtTest005 end";
}
}  // namespace OHOS::Request::Upload
