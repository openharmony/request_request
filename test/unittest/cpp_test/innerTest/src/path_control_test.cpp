/*
 * Copyright (c) 2025 Huawei Device Co., Ltd.
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

#include <cstdint>
#include <cstring>
#include <sstream>
#define private public
#define protected public

#include <gtest/gtest.h>

#include "log.h"
#include "path_control.h"
#include "request_common.h"

using namespace testing::ext;
using namespace OHOS::Request;

#undef private
#undef protected

class PathControlTest : public testing::Test {
public:
    static void SetUpTestCase(void);
    static void TearDownTestCase(void);
    void SetUp();
    void TearDown();
};

void PathControlTest::SetUpTestCase(void)
{
    // input testSuit setup step，setup invoked before all testCases
}

void PathControlTest::TearDownTestCase(void)
{
    // input testSuit teardown step，teardown invoked after all testCases
}

void PathControlTest::SetUp(void)
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

void PathControlTest::TearDown(void)
{
    // input testCase teardown step，teardown invoked after each testCase
}

/**
 * @tc.name: PathControlTest001
 * @tc.desc: Verify AddPathsToMap returns false for non-existent path
 * @tc.precon: NA
 * @tc.step: 1. Create filepath string "/A/B/C/test1"
 *           2. Call AddPathsToMap with filepath
 * @tc.expect: Should return false indicating failure
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PathControlTest, PathControlTest001, TestSize.Level1)
{
    std::string filepath = "/A/B/C/test1";
    bool res = PathControl::AddPathsToMap(filepath);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: PathControlTest002
 * @tc.desc: Verify SubPathsToMap returns false for non-existent path
 * @tc.precon: NA
 * @tc.step: 1. Create filepath string "/A/B/C/test1"
 *           2. Call SubPathsToMap with filepath
 * @tc.expect: Should return false indicating failure
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PathControlTest, PathControlTest002, TestSize.Level1)
{
    std::string filepath = "/A/B/C/test1";
    bool res = PathControl::SubPathsToMap(filepath);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: PathControlTest003
 * @tc.desc: Verify CheckBelongAppBaseDir returns false for non-app path
 * @tc.precon: NA
 * @tc.step: 1. Create filepath string "/A/B/C/test1"
 *           2. Call CheckBelongAppBaseDir with filepath
 * @tc.expect: Should return false indicating path is not in app base directory
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PathControlTest, PathControlTest003, TestSize.Level1)
{
    std::string filepath = "/A/B/C/test1";
    bool res = PathControl::CheckBelongAppBaseDir(filepath);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: PathControlTest004
 * @tc.desc: Verify ShieldPath correctly masks sensitive path segments
 * @tc.precon: NA
 * @tc.step: 1. Create path string "/ab/abcde"
 *           2. Call ShieldPath with path
 * @tc.expect: Should return masked path
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PathControlTest, PathControlTest004, TestSize.Level1)
{
    std::string path = "/ab/abcde";
    std::string res = PathControl::ShieldPath(path);
    EXPECT_EQ(res, "/*b/**cde");
}
