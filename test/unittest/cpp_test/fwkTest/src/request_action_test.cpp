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

#include <string>
#define private public
#define protected public

#include <gtest/gtest.h>

#include "log.h"
#include "request_action.h"
#include "request_common.h"
#include "request_manager_impl.h"

using namespace testing::ext;
using namespace OHOS::Request;

#undef private
#undef protected

class RequestActionTest : public testing::Test {
public:
    static void SetUpTestCase(void);
    static void TearDownTestCase(void);
    void SetUp();
    void TearDown();
};

void RequestActionTest::SetUpTestCase(void)
{
    // input testSuit setup step，setup invoked before all testCases
}

void RequestActionTest::TearDownTestCase(void)
{
    // input testSuit teardown step，teardown invoked after all testCases
}

void RequestActionTest::SetUp(void)
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

void RequestActionTest::TearDown(void)
{
    // input testCase teardown step，teardown invoked after each testCase
}

/**
 * @tc.name: GetTaskTest001
 * @tc.desc: Test CreateTest001 interface base function - GetTask
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, GetTaskTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string token = "token";
    Config config;
    std::string tid = "1";
    RequestAction::GetInstance()->RequestAction::GetInstance()->GetTask(tid, token, config);
}

/**
 * @tc.name: StartTest001
 * @tc.desc: Test StartTest001 interface base function - Start
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, StartTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tidStr = "tid";
    RequestAction::GetInstance()->Start(tidStr);
}

/**
 * @tc.name: StopTest001
 * @tc.desc: Test StopTest001 interface base function - Stop
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, StopTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = "tid";
    RequestAction::GetInstance()->Stop(tid);
}

/**
 * @tc.name: TouchTest001
 * @tc.desc: Test TouchTest001 interface base function - Touch
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, TouchTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = "tid";
    std::string token = "token";
    TaskInfo info;
    RequestAction::GetInstance()->Touch(token, tid, info);
}

/**
 * @tc.name: ShowTest001
 * @tc.desc: Test ShowTest001 interface base function - Show
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, ShowTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = "tid";
    TaskInfo info;
    RequestAction::GetInstance()->Show(tid, info);
}

/**
 * @tc.name: PauseTest001
 * @tc.desc: Test PauseTest001 interface base function - Pause
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, PauseTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = "tid";
    RequestAction::GetInstance()->Pause(tid);
}

/**
 * @tc.name: RemoveTest001
 * @tc.desc: Test RemoveTest001 interface base function - Remove
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, RemoveTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = "tid";
    RequestAction::GetInstance()->Remove(tid);
}

/**
 * @tc.name: ResumeTest001
 * @tc.desc: Test ResumeTest001 interface base function - Resume
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, ResumeTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = "tid";
    RequestAction::GetInstance()->Resume(tid);
}
