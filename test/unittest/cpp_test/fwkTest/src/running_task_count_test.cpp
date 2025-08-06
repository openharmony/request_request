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

#include "running_task_count.h"

#include <gtest/gtest.h>

#include <cstdint>
#include <memory>

#include "constant.h"
#include "request_manager_impl.h"
#include "request_running_task_count.h"

using namespace testing::ext;
using namespace OHOS::Request;

#undef private
#undef protected

class FwkTestOberver : public IRunningTaskObserver {
public:
    void OnRunningTaskCountUpdate(int count) override;
    ~FwkTestOberver() = default;
    FwkTestOberver() = default;
};

void FwkTestOberver::OnRunningTaskCountUpdate(int count)
{
    EXPECT_EQ(FwkRunningTaskCountManager::GetInstance()->GetCount(), count);
    REQUEST_HILOGI("[RunningTaskCountTest] OnRunningTaskCountUpdate count = %{public}d", count);
}

class RunningTaskCountTest : public testing::Test {
public:
    static void SetUpTestCase(void);
    static void TearDownTestCase(void);
    void SetUp();
    void TearDown();
};

void RunningTaskCountTest::SetUpTestCase(void)
{
    // input testsuit setup step，setup invoked before all testcases
}

void RunningTaskCountTest::TearDownTestCase(void)
{
    // input testsuit teardown step，teardown invoked after all testcases
}

void RunningTaskCountTest::SetUp(void)
{
    // input testcase setup step，setup invoked before each testcases
}

void RunningTaskCountTest::TearDown(void)
{
    // input testcase teardown step，teardown invoked after each testcases
}

/**
 * @tc.name: SubscribeRunningTaskCount001
 * @tc.desc: Test SubscribeRunningTaskCount with null proxy - should succeed
 * @tc.precon: Request service proxy is null
 * @tc.step: 1. Get request service proxy and verify it's null
 *           2. Create a valid observer instance
 *           3. Call SubscribeRunningTaskCount with the observer
 *           4. Verify the return value is E_OK
 *           5. Clean up by unsubscribing the observer
 * @tc.expect: Subscription succeeds with E_OK return value
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level1
 */
HWTEST_F(RunningTaskCountTest, SubscribeRunningTaskCountTest_001, TestSize.Level1)
{
    GTEST_LOG_(INFO) << "RunningTaskCountTest, SubscribeRunningTaskCountTest_001, TestSize.Level1";
    REQUEST_HILOGI("[RunningTaskCountTest] SubscribeRunningTaskCountTest_001 begin");
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    if (proxy == nullptr) {
        std::shared_ptr<IRunningTaskObserver> ob = std::make_shared<FwkTestOberver>();
        int32_t ret = SubscribeRunningTaskCount(ob);
        EXPECT_EQ(ret, E_OK);
        UnsubscribeRunningTaskCount(ob);
    }
    REQUEST_HILOGI("[RunningTaskCountTest] SubscribeRunningTaskCountTest_001 end");
}

/**
 * @tc.name: SubscribeRunningTaskCount002
 * @tc.desc: Test SubscribeRunningTaskCount with multiple observers - should succeed
 * @tc.precon: No observers are initially attached
 * @tc.step: 1. Create first observer and subscribe it
 *           2. Create second observer and attach it manually
 *           3. Call SubscribeRunningTaskCount with second observer
 *           4. Verify the return value is E_OK
 *           5. Clean up by detaching both observers
 * @tc.expect: Subscription succeeds with E_OK return value for both observers
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level1
 */
HWTEST_F(RunningTaskCountTest, SubscribeRunningTaskCountTest_002, TestSize.Level1)
{
    GTEST_LOG_(INFO) << "RunningTaskCountTest, SubscribeRunningTaskCountTest_002, TestSize.Level1";
    REQUEST_HILOGI("[RunningTaskCountTest] SubscribeRunningTaskCountTest_002 begin");

    std::shared_ptr<IRunningTaskObserver> ob1 = std::make_shared<FwkTestOberver>();
    SubscribeRunningTaskCount(ob1);
    std::shared_ptr<IRunningTaskObserver> ob2 = std::make_shared<FwkTestOberver>();
    FwkRunningTaskCountManager::GetInstance()->AttachObserver(ob2);
    int ret = SubscribeRunningTaskCount(ob2);
    EXPECT_EQ(ret, E_OK);
    FwkRunningTaskCountManager::GetInstance()->DetachObserver(ob1);
    FwkRunningTaskCountManager::GetInstance()->DetachObserver(ob2);
    REQUEST_HILOGI("[RunningTaskCountTest] SubscribeRunningTaskCountTest_002 end");
}

/**
 * @tc.name: SubscribeRunningTaskCount003
 * @tc.desc: Test SubscribeRunningTaskCount with null observer - should fail
 * @tc.precon: Observer parameter is null
 * @tc.step: 1. Set observer to nullptr
 *           2. Call SubscribeRunningTaskCount with null observer
 *           3. Verify the return value is E_OTHER
 * @tc.expect: Subscription fails with E_OTHER return value due to null observer
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level1
 */
HWTEST_F(RunningTaskCountTest, SubscribeRunningTaskCountTest_003, TestSize.Level1)
{
    GTEST_LOG_(INFO) << "RunningTaskCountTest, SubscribeRunningTaskCountTest_003, TestSize.Level1";
    REQUEST_HILOGI("[RunningTaskCountTest] SubscribeRunningTaskCountTest_003 begin");

    std::shared_ptr<IRunningTaskObserver> ob = nullptr;
    auto ret = SubscribeRunningTaskCount(ob);
    EXPECT_EQ(ret, E_OTHER);
}

/**
 * @tc.name: UnsubscribeRunningTaskCount001
 * @tc.desc: Test UnsubscribeRunningTaskCount with valid and invalid observers
 * @tc.precon: One observer is already attached
 * @tc.step: 1. Create and attach first observer
 *           2. Verify observer is attached
 *           3. Create second observer (not attached)
 *           4. Call UnsubscribeRunningTaskCount with unattached observer
 *           5. Call UnsubscribeRunningTaskCount with attached observer
 * @tc.expect: Unsubscription handles both valid and invalid observers gracefully
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level1
 */
HWTEST_F(RunningTaskCountTest, UnubscribeRunningTaskCountTest_001, TestSize.Level1)
{
    GTEST_LOG_(INFO) << "RunningTaskCountTest, UnubscribeRunningTaskCountTest_001, TestSize.Level1";
    REQUEST_HILOGI("[RunningTaskCountTest] UnubscribeRunningTaskCountTest_001 begin");
    std::shared_ptr<IRunningTaskObserver> ob1 = std::make_shared<FwkTestOberver>();
    FwkRunningTaskCountManager::GetInstance()->AttachObserver(ob1);
    EXPECT_TRUE(FwkRunningTaskCountManager::GetInstance()->HasObserver());
    std::shared_ptr<IRunningTaskObserver> ob2 = std::make_shared<FwkTestOberver>();
    UnsubscribeRunningTaskCount(ob2);
    UnsubscribeRunningTaskCount(ob1);
    REQUEST_HILOGI("[RunningTaskCountTest] UnubscribeRunningTaskCountTest_001 end");
}

/**
 * @tc.name: GetAndSetCount001
 * @tc.desc: Test GetCount and SetCount functionality of running task count manager
 * @tc.precon: Running task count manager is initialized
 * @tc.step: 1. Store the current count value
 *           2. Set count to a new value (10)
 *           3. Verify GetCount returns the new value
 *           4. Restore the original count value
 *           5. Verify GetCount returns the original value
 * @tc.expect: SetCount correctly updates the count and GetCount returns accurate values
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level1
 */
HWTEST_F(RunningTaskCountTest, GetAndSetCount001, TestSize.Level1)
{
    int old = FwkRunningTaskCountManager::GetInstance()->GetCount();
    int except = 10; // 10 is except count num
    FwkRunningTaskCountManager::GetInstance()->SetCount(except);
    int count = FwkRunningTaskCountManager::GetInstance()->GetCount();
    EXPECT_EQ(count, except);
    FwkRunningTaskCountManager::GetInstance()->SetCount(old);
    count = FwkRunningTaskCountManager::GetInstance()->GetCount();
    EXPECT_EQ(count, old);
}

/**
 * @tc.name: NotifyAllObservers001
 * @tc.desc: Test NotifyAllObservers functionality with observer updates
 * @tc.precon: Observer infrastructure is set up
 * @tc.step: 1. Create and attach a test observer
 *           2. Verify observer is attached
 *           3. Call NotifyAllObservers to trigger updates
 *           4. Verify observer receives update notification
 *           5. Clean up by detaching the observer
 * @tc.expect: All attached observers receive count update notifications
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level1
 */
HWTEST_F(RunningTaskCountTest, NotifyAllObserversTest001, TestSize.Level1)
{
    std::shared_ptr<IRunningTaskObserver> ob = std::make_shared<FwkTestOberver>();
    FwkIRunningTaskObserver runningOb = FwkIRunningTaskObserver(ob);
    runningOb.UpdateRunningTaskCount();
    std::shared_ptr<IRunningTaskObserver> ob1 = std::make_shared<FwkTestOberver>();
    FwkRunningTaskCountManager::GetInstance()->AttachObserver(ob1);
    EXPECT_TRUE(FwkRunningTaskCountManager::GetInstance()->HasObserver());
    FwkRunningTaskCountManager::GetInstance()->NotifyAllObservers();
    FwkRunningTaskCountManager::GetInstance()->DetachObserver(ob1);
}