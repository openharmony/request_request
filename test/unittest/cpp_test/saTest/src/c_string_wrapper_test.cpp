/*
 * Copyright (c) 2023 Huawei Device Co., Ltd.
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

#include "c_string_wrapper.h"

#include <gtest/gtest.h>

#include "gmock/gmock.h"
#include "log.h"
using namespace testing::ext;

class StringWrapperTest : public testing::Test {
public:
    static void SetUpTestCase(void);
    static void TearDownTestCase(void);
    void SetUp();
    void TearDown();
};

void StringWrapperTest::SetUpTestCase(void)
{
    // input testsuit setup step，setup invoked before all testcases
}

void StringWrapperTest::TearDownTestCase(void)
{
    // input testsuit teardown step，teardown invoked after all testcases
}

void StringWrapperTest::SetUp(void)
{
    // input testcase setup step，setup invoked before each testcases
    testing::UnitTest *test = testing::UnitTest::GetInstance();
    ASSERT_NE(test, nullptr);
    const testing::TestInfo *testInfo = test->current_test_info();
    ASSERT_NE(testInfo, nullptr);
    string testCaseName = string(testInfo->name());
    REQUEST_HILOGI("[SetUp] %{public}s start", testCaseName.c_str());
    GTEST_LOG_(INFO) << testCaseName.append(" start");
}

void StringWrapperTest::TearDown(void)
{
    // input testcase teardown step，teardown invoked after each testcases
}

/**
 * @tc.name: WrapperCStringTest_001
 * @tc.desc: Test WrapperCString interface base function
 * @tc.precon: NA
 * @tc.step: 1. Create test string with content "c_string_wrapper_for_test"
 *           2. Call WrapperCString with test string
 *           3. Verify returned CStringWrapper length matches string length
 *           4. Verify CStringWrapper content matches original string
 *           5. Clean up allocated memory
 * @tc.expect: CStringWrapper correctly wraps string with proper length and content
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(StringWrapperTest, WrapperCStringTest_001, TestSize.Level1)
{
    std::string str("c_string_wrapper_for_test");
    CStringWrapper ret = WrapperCString(str);
    EXPECT_EQ(ret.len, str.length());
    EXPECT_EQ(strncmp(ret.cStr, str.c_str(), ret.len), 0);

    char *str1 = new char[10];
    DeleteChar(str1);
}