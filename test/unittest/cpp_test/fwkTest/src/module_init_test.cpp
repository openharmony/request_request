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

#include "openssl/crypto.h"
#define private public
#define protected public

#include <gtest/gtest.h>

#include "module_init.h"

using namespace testing::ext;
using namespace OHOS::Request;

#undef private
#undef protected

class ModuleInitTest : public testing::Test {
public:
    static void SetUpTestCase(void);
    static void TearDownTestCase(void);
    void SetUp();
    void TearDown();
};

void ModuleInitTest::SetUpTestCase(void)
{
    // input testsuit setup step，setup invoked before all testcases
}

void ModuleInitTest::TearDownTestCase(void)
{
    // input testsuit teardown step，teardown invoked after all testcases
}

void ModuleInitTest::SetUp(void)
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

void ModuleInitTest::TearDown(void)
{
    // input testcase teardown step，teardown invoked after each testcases
}

/**
 * @tc.name: ThreadIdCallback_001
 * @tc.desc: Test ThreadIdCallback_001 interface base function - ThreadIdCallback
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ModuleInitTest, ThreadIdCallback_001, TestSize.Level1)
{
    ModuleInit::LockCallback(CRYPTO_LOCK, 0, nullptr, 0);
    ModuleInit::LockCallback(CRYPTO_UNLOCK, 0, nullptr, 0);
    unsigned long result = ModuleInit::ThreadIdCallback();
    ASSERT_EQ(result, static_cast<unsigned long>(pthread_self()));
}