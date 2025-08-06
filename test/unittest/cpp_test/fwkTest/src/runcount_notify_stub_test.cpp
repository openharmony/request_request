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

#include <cstdint>
#include <memory>

#include "download_server_ipc_interface_code.h"
#include "gmock/gmock.h"
#include "log.h"
#include "parcel_helper.h"
#include "request_common.h"
#include "request_running_task_count.h"
#include "runcount_notify_stub.h"

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
 * @tc.name: OnCallBackTest001
 * @tc.desc: Test the OnCallBack function of RunCountNotifyStub
 * @tc.precon: NA
 * @tc.step: 1. Create a Notify object
 *           2. Call CallBack method with the Notify object
 *           3. Create a TaskInfo object
 *           4. Call Done method with the TaskInfo object
 *           5. Write an int64_t value to MessageParcel
 *           6. Call OnCallBack method with the MessageParcel
 *           7. Check if the count is updated correctly
 * @tc.expect: The count should be updated to the expected value
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RuncountNotifyStubTest, OnCallBackTest001, TestSize.Level1)
{
    Notify notify;
    RunCountNotifyStub::GetInstance()->CallBack(notify);
    TaskInfo taskInfo;
    RunCountNotifyStub::GetInstance()->Done(taskInfo);
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

/**
 * @tc.name: OnRemoteRequestTest001
 * @tc.desc: Test the OnRemoteRequest function of RunCountNotifyStub
 * @tc.precon: NA
 * @tc.step: 1. Prepare MessageParcel data with interface token
 *           2. Call OnRemoteRequest with REQUEST_NOTIFY_RUNCOUNT code
 *           3. Prepare another MessageParcel with correct descriptor
 *           4. Call OnRemoteRequest with REQUEST_DONE_NOTIFY code
 *           5. Verify the return value is not zero
 * @tc.expect: OnRemoteRequest should handle different request codes correctly
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RuncountNotifyStubTest, OnRemoteRequestTest001, TestSize.Level1)
{
    uint32_t code = static_cast<uint32_t>(RequestNotifyInterfaceCode::REQUEST_NOTIFY_RUNCOUNT);
    OHOS::MessageParcel data;
    std::u16string token = u"token";
    data.WriteInterfaceToken(token);
    data.WriteInt64(0);
    OHOS::MessageParcel reply;
    OHOS::MessageOption option;
    RunCountNotifyStub runCount = RunCountNotifyStub();
    runCount.OnRemoteRequest(code, data, reply, option);
    OHOS::MessageParcel data1;
    token = runCount.GetDescriptor();
    data1.WriteInterfaceToken(token);
    data1.WriteInt64(0);
    code = static_cast<uint32_t>(RequestNotifyInterfaceCode::REQUEST_DONE_NOTIFY);
    runCount.OnRemoteRequest(code, data1, reply, option);
    EXPECT_NE(runCount.OnRemoteRequest(code, data1, reply, option), 0);
    code = static_cast<uint32_t>(RequestNotifyInterfaceCode::REQUEST_NOTIFY_RUNCOUNT);
}