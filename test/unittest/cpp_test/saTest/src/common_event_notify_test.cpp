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

#include "common_event_notify.h"

#include <gtest/gtest.h>
using namespace testing::ext;

class CommonEventNotifyTest : public testing::Test {
public:
    static void SetUpTestCase(void);
    static void TearDownTestCase(void);
    void SetUp();
    void TearDown();
};

void CommonEventNotifyTest::SetUpTestCase(void)
{
    // input testsuit setup step，setup invoked before all testcases
}

void CommonEventNotifyTest::TearDownTestCase(void)
{
    // input testsuit teardown step，teardown invoked after all testcases
}

void CommonEventNotifyTest::SetUp(void)
{
    // input testcase setup step，setup invoked before each testcases
}

void CommonEventNotifyTest::TearDown(void)
{
    // input testcase teardown step，teardown invoked after each testcases
}

/**
 * @tc.name: PublishStateChangeEventsTest_001
 * @tc.desc: Test PublishStateChangeEvents interface base function
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(CommonEventNotifyTest, PublishStateChangeEventsTest_001, TestSize.Level1)
{
    const char *bundleName = "com.example.myapplication";
    uint32_t len = strlen(bundleName);
    uint32_t taskId = 123456;
    int32_t state = 0x40; // State::Completed
    PublishStateChangeEvents(bundleName, len, taskId, state);
}