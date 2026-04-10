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

#include "request_cert_mgr_adapter.h"

#include <gtest/gtest.h>

#include <cstring>

#include "gmock/gmock.h"
#include "log.h"
#include "syspara/parameter.h"
using namespace testing::ext;

class CertMgrAdapterTest : public testing::Test {
public:
    static void SetUpTestCase(void);
    static void TearDownTestCase(void);
    void SetUp();
    void TearDown();
};

void CertMgrAdapterTest::SetUpTestCase(void)
{
}

void CertMgrAdapterTest::TearDownTestCase(void)
{
}

void CertMgrAdapterTest::SetUp(void)
{
    testing::UnitTest *test = testing::UnitTest::GetInstance();
    ASSERT_NE(test, nullptr);
    const testing::TestInfo *testInfo = test->current_test_info();
    ASSERT_NE(testInfo, nullptr);
    string testCaseName = string(testInfo->name());
    REQUEST_HILOGI("[SetUp] %{public}s start", testCaseName.c_str());
    GTEST_LOG_(INFO) << testCaseName.append(" start");
}

void CertMgrAdapterTest::TearDown(void)
{
}

/**
 * @tc.name: IsDevelopermodeEnabledTest_001
 * @tc.desc: Test IsDevelopermodeEnabled interface
 * @tc.precon: NA
 * @tc.step: 1. Call IsDevelopermodeEnabled function
 *           2. Verify function returns a boolean value
 * @tc.expect: Function successfully returns developer mode status
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(CertMgrAdapterTest, IsDevelopermodeEnabledTest_001, TestSize.Level1)
{
    const char *key = "const.security.developermode.state";
    const char *defaultValue = "false";
    char value[16] = { 0 };
    int ret = GetParameter(key, defaultValue, value, sizeof(value) - 1);
    bool isEnabled = IsDevelopermodeEnabled();
    if (ret < 0) {
        EXPECT_FALSE(isEnabled);
    } else {
        value[sizeof(value) - 1] = '\0';
        if (strcmp(value, "true") == 0) {
            EXPECT_TRUE(isEnabled);
        } else {
            EXPECT_FALSE(isEnabled);
        }
    }
}
