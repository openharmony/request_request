/*
 * Copyright (c) 2024 Huawei Device Co., Ltd.
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

#include <cstring>
#define private public
#define protected public

#include <gtest/gtest.h>

#include "gmock/gmock.h"
#include "js_common.h"
#include "log.h"
#include "parcel_helper.h"

using namespace testing::ext;
using namespace OHOS::Request;

#undef private
#undef protected

class ParcelHelperTest : public testing::Test {
public:
    static void SetUpTestCase(void);
    static void TearDownTestCase(void);
    void SetUp();
    void TearDown();
};

void ParcelHelperTest::SetUpTestCase(void)
{
    // input testSuit setup step，setup invoked before all testCases
}

void ParcelHelperTest::TearDownTestCase(void)
{
    // input testSuit teardown step，teardown invoked after all testCases
}

void ParcelHelperTest::SetUp(void)
{
    // input testCase setup step，setup invoked before each testCase
    testing::UnitTest *test = testing::UnitTest::GetInstance();
    ASSERT_NE(test, nullptr);
    const testing::TestInfo *testInfo = test->current_test_info();
    ASSERT_NE(testInfo, nullptr);
    string testCaseName = string(testInfo->name());
    REQUEST_HILOGI("[SetUp] %{public}s start", testCaseName.c_str());
    GTEST_LOG_(INFO) << testCaseName.append(" start");
}

void ParcelHelperTest::TearDown(void)
{
    // input testCase teardown step，teardown invoked after each testCase
}

/**
 * @tc.name: UnMarshalFormItem001
 * @tc.desc: Test UnMarshalFormItem001 interface base function - UnMarshalFormItem
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshalFormItem001, TestSize.Level1)
{
    OHOS::MessageParcel data;
    TaskInfo info;
    uint32_t size = 0;
    data.WriteUint32(size);
    EXPECT_TRUE(ParcelHelper::UnMarshalFormItem(data, info));
    size = 1;
    data.WriteUint32(size);
    EXPECT_FALSE(ParcelHelper::UnMarshalFormItem(data, info));
    data.WriteUint32(size);
    data.WriteString("name");
    data.WriteString("value");
    EXPECT_TRUE(ParcelHelper::UnMarshalFormItem(data, info));
    EXPECT_EQ(info.forms[0].name, "name");
    EXPECT_EQ(info.forms[0].value, "value");
}

/**
 * @tc.name: UnMarshalFileSpec001
 * @tc.desc: Test UnMarshalFileSpec001 interface base function - UnMarshalFileSpec
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshalFileSpec001, TestSize.Level1)
{
    OHOS::MessageParcel data;
    TaskInfo info;
    uint32_t size = 0;
    data.WriteUint32(size);
    EXPECT_TRUE(ParcelHelper::UnMarshalFileSpec(data, info));
    size = 1;
    data.WriteUint32(size);
    EXPECT_FALSE(ParcelHelper::UnMarshalFileSpec(data, info));
    data.WriteUint32(size);
    data.WriteString("name");
    data.WriteString("uri");
    data.WriteString("filename");
    data.WriteString("type");
    EXPECT_TRUE(ParcelHelper::UnMarshalFileSpec(data, info));
    EXPECT_EQ(info.files[0].name, "name");
    EXPECT_EQ(info.files[0].uri, "uri");
    EXPECT_EQ(info.files[0].filename, "filename");
    EXPECT_EQ(info.files[0].type, "type");
}

/**
 * @tc.name: UnMarshalMapProgressExtras001
 * @tc.desc: Test UnMarshalMapProgressExtras001 interface base function - UnMarshalMapProgressExtras
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshalMapProgressExtras001, TestSize.Level1)
{
    OHOS::MessageParcel data;
    TaskInfo info;
    uint32_t size = 0;
    data.WriteUint32(size);
    EXPECT_TRUE(ParcelHelper::UnMarshalMapProgressExtras(data, info));
    size = 1;
    data.WriteUint32(size);
    EXPECT_FALSE(ParcelHelper::UnMarshalMapProgressExtras(data, info));
    data.WriteUint32(size);
    data.WriteString("key");
    data.WriteString("value");
    EXPECT_TRUE(ParcelHelper::UnMarshalMapProgressExtras(data, info));
    EXPECT_EQ(info.progress.extras["key"], "value");
}

/**
 * @tc.name: UnMarshalMapExtras001
 * @tc.desc: Test UnMarshalMapExtras001 interface base function - UnMarshalMapExtras
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshalMapExtras001, TestSize.Level1)
{
    OHOS::MessageParcel data;
    TaskInfo info;
    uint32_t size = 0;
    data.WriteUint32(size);
    EXPECT_TRUE(ParcelHelper::UnMarshalMapExtras(data, info));
    size = 1;
    data.WriteUint32(size);
    EXPECT_FALSE(ParcelHelper::UnMarshalMapExtras(data, info));
    data.WriteUint32(size);
    data.WriteString("key");
    data.WriteString("value");
    EXPECT_TRUE(ParcelHelper::UnMarshalMapExtras(data, info));
    EXPECT_EQ(info.extras["key"], "value");
}

/**
 * @tc.name: UnMarshalTaskState001
 * @tc.desc: Test UnMarshalTaskState001 interface base function - UnMarshalTaskState
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshalTaskState001, TestSize.Level1)
{
    OHOS::MessageParcel data;
    TaskInfo info;
    uint32_t size = 0;
    data.WriteUint32(size);
    EXPECT_TRUE(ParcelHelper::UnMarshalTaskState(data, info));
    size = 1;
    uint32_t responseCode = 0;
    data.WriteUint32(size);
    EXPECT_FALSE(ParcelHelper::UnMarshalTaskState(data, info));
    data.WriteUint32(size);
    data.WriteString("path");
    data.WriteUint32(responseCode);
    data.WriteString("message");
    EXPECT_TRUE(ParcelHelper::UnMarshalTaskState(data, info));
    EXPECT_EQ(info.taskStates[0].path, "path");
    EXPECT_EQ(info.taskStates[0].responseCode, responseCode);
    EXPECT_EQ(info.taskStates[0].message, "message");
}

/**
 * @tc.name: UnMarshalConfigHeaders001
 * @tc.desc: Test UnMarshalConfigHeaders001 interface base function - UnMarshalConfigHeaders
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshalConfigHeaders001, TestSize.Level1)
{
    OHOS::MessageParcel data;
    Config config;
    uint32_t size = 0;
    data.WriteUint32(size);
    EXPECT_TRUE(ParcelHelper::UnMarshalConfigHeaders(data, config));
    size = 1;
    data.WriteUint32(size);
    EXPECT_FALSE(ParcelHelper::UnMarshalConfigHeaders(data, config));
    data.WriteUint32(size);
    data.WriteString("key");
    data.WriteString("value");
    EXPECT_TRUE(ParcelHelper::UnMarshalConfigHeaders(data, config));
    EXPECT_EQ(config.headers["key"], "value");
}

/**
 * @tc.name: UnMarshalConfigExtras001
 * @tc.desc: Test UnMarshalConfigExtras001 interface base function - UnMarshalConfigExtras
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshalConfigExtras001, TestSize.Level1)
{
    OHOS::MessageParcel data;
    Config config;
    uint32_t size = 0;
    data.WriteUint32(size);
    EXPECT_TRUE(ParcelHelper::UnMarshalConfigExtras(data, config));
    size = 1;
    data.WriteUint32(size);
    EXPECT_FALSE(ParcelHelper::UnMarshalConfigExtras(data, config));
    data.WriteUint32(size);
    data.WriteString("key");
    data.WriteString("value");
    EXPECT_TRUE(ParcelHelper::UnMarshalConfigExtras(data, config));
    EXPECT_EQ(config.extras["key"], "value");
}

/**
 * @tc.name: UnMarshalConfigFormItem001
 * @tc.desc: Test UnMarshalConfigFormItem001 interface base function - UnMarshalConfigFormItem
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshalConfigFormItem001, TestSize.Level1)
{
    OHOS::MessageParcel data;
    Config config;
    uint32_t size = 0;
    data.WriteUint32(size);
    EXPECT_TRUE(ParcelHelper::UnMarshalConfigFormItem(data, config));
    size = 1;
    data.WriteUint32(size);
    EXPECT_FALSE(ParcelHelper::UnMarshalConfigFormItem(data, config));
    data.WriteUint32(size);
    data.WriteString("name");
    data.WriteString("value");
    EXPECT_TRUE(ParcelHelper::UnMarshalConfigFormItem(data, config));
    EXPECT_EQ(config.forms[0].name, "name");
    EXPECT_EQ(config.forms[0].value, "value");
}

/**
 * @tc.name: UnMarshalConfigFileSpec001
 * @tc.desc: Test UnMarshalConfigFileSpec001 interface base function - UnMarshalConfigFileSpec
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshalConfigFileSpec001, TestSize.Level1)
{
    OHOS::MessageParcel data;
    Config config;
    uint32_t size = 0;
    data.WriteUint32(size);
    EXPECT_TRUE(ParcelHelper::UnMarshalConfigFileSpec(data, config));
    size = 1;
    data.WriteUint32(size);
    EXPECT_FALSE(ParcelHelper::UnMarshalConfigFileSpec(data, config));
    data.WriteUint32(size);
    data.WriteString("name");
    data.WriteString("uri");
    data.WriteString("filename");
    data.WriteString("type");
    EXPECT_TRUE(ParcelHelper::UnMarshalConfigFileSpec(data, config));
    EXPECT_EQ(config.files[0].name, "name");
    EXPECT_EQ(config.files[0].uri, "uri");
    EXPECT_EQ(config.files[0].filename, "filename");
    EXPECT_EQ(config.files[0].type, "type");
}

/**
 * @tc.name: UnMarshalConfigBodyFileName001
 * @tc.desc: Test UnMarshalConfigBodyFileName001 interface base function - UnMarshalConfigBodyFileName
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshalConfigBodyFileName001, TestSize.Level1)
{
    OHOS::MessageParcel data;
    Config config;
    uint32_t size = 0;
    data.WriteUint32(size);
    EXPECT_TRUE(ParcelHelper::UnMarshalConfigBodyFileName(data, config));
    size = 1;
    data.WriteUint32(size);
    EXPECT_FALSE(ParcelHelper::UnMarshalConfigBodyFileName(data, config));
    data.WriteUint32(size);
    data.WriteString("name");
    EXPECT_TRUE(ParcelHelper::UnMarshalConfigBodyFileName(data, config));
    EXPECT_EQ(config.bodyFds[0], 0);
    EXPECT_EQ(config.bodyFileNames[0], "name");
}