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

#include <gtest/gtest.h>

#include "get_calling_bundle.h"
#include "get_top_bundle.h"
using namespace testing::ext;

class GetBundleTest : public testing::Test {
public:
    static void SetUpTestCase(void);
    static void TearDownTestCase(void);
    void SetUp();
    void TearDown();
};

void GetBundleTest::SetUpTestCase(void)
{
    // input testsuit setup step，setup invoked before all testcases
}

void GetBundleTest::TearDownTestCase(void)
{
    // input testsuit teardown step，teardown invoked after all testcases
}

void GetBundleTest::SetUp(void)
{
    // input testcase setup step，setup invoked before each testcases
}

void GetBundleTest::TearDown(void)
{
    // input testcase teardown step，teardown invoked after each testcases
}

/**
 * @tc.name: GetBundleTest_001
 * @tc.desc: Test GetCallingBundle interface base function
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(GetBundleTest, GetBundleTest_001, TestSize.Level1)
{
    uint64_t tokenId = 123456;
    CStringWrapper ret = GetCallingBundle(tokenId);
    EXPECT_EQ(ret.len, 0);
}

/**
 * @tc.name: GetBundleTest_002
 * @tc.desc: Test GetTopBundleName interface base function
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(GetBundleTest, GetBundleTest_002, TestSize.Level1)
{
    GetTopBundleName();
}