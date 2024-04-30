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

#define private public
#define protected public

#include "runcount_notify_stub.h"

#include <gtest/gtest.h>

#include <cstdint>
#include <memory>

#include "gmock/gmock.h"
#include "js_common.h"
#include "log.h"
#include "request_running_task_count.h"

using namespace testing::ext;
using namespace OHOS::Request;

#undef private
#undef protected

class RuncountNotifyStubTest : public testing::Test {
public:
    static void SetUpTestCase(void);
    static void TearDownTestCase(void);
    void SetUp();
    void TearDown();
};

void RuncountNotifyStubTest::SetUpTestCase(void)
{
    // input testSuit setup step，setup invoked before all testCases
}

void RuncountNotifyStubTest::TearDownTestCase(void)
{
    // input testSuit teardown step，teardown invoked after all testCases
}

void RuncountNotifyStubTest::SetUp(void)
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

void RuncountNotifyStubTest::TearDown(void)
{
    // input testCase teardown step，teardown invoked after each testCase
}

/**
 * @tc.name: GetInstanceTest001
 * @tc.desc: Test GetInstanceTest001 interface base function - GetInstance
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RuncountNotifyStubTest, GetInstanceTest001, TestSize.Level1)
{
    RunCountNotifyStub::GetInstance();
}

/**
 * @tc.name: CallBackTest001
 * @tc.desc: Test CallBackTest001 interface base function - CallBack
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RuncountNotifyStubTest, CallBackTest001, TestSize.Level1)
{
    Notify notify;
    RunCountNotifyStub::GetInstance()->CallBack(notify);
}

/**
 * @tc.name: DoneTest001
 * @tc.desc: Test DoneTest001 interface base function - Done
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RuncountNotifyStubTest, DoneTest001, TestSize.Level1)
{
    TaskInfo taskInfo;
    RunCountNotifyStub::GetInstance()->Done(taskInfo);
}

/**
 * @tc.name: OnCallBackTest001
 * @tc.desc: Test OnCallBackTest001 interface base function - OnCallBack
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RuncountNotifyStubTest, OnCallBackTest001, TestSize.Level1)
{
    int64_t except = 10; // 10 is except value
    int old = FwkRunningTaskCountManager::GetInstance()->GetCount();
    OHOS::MessageParcel data;
    data.WriteInt64(except);
    RunCountNotifyStub::GetInstance()->OnCallBack(data);
    int count = FwkRunningTaskCountManager::GetInstance()->GetCount();
    EXPECT_EQ(count, except);
    FwkRunningTaskCountManager::GetInstance()->SetCount(old);
    count = FwkRunningTaskCountManager::GetInstance()->GetCount();
    EXPECT_EQ(count, old);
}