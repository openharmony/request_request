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
 * @tc.name: SubscribeRunningTaskCountTest_001
 * @tc.desc: Test SubscribeRunningTaskCountTest_001 interface base function - subscribe failed
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RunningTaskCountTest, SubscribeRunningTaskCountTest_001, TestSize.Level1)
{
    GTEST_LOG_(INFO) << "RunningTaskCountTest, SubscribeRunningTaskCountTest_001, TestSize.Level1";
    REQUEST_HILOGI("[RunningTaskCountTest] SubscribeRunningTaskCountTest_001 begin");
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy();
    if (proxy == nullptr) {
        std::shared_ptr<IRunningTaskObserver> ob = std::make_shared<FwkTestOberver>();
        int32_t ret = SubscribeRunningTaskCount(ob);
        EXPECT_EQ(ret, E_OK);
        UnsubscribeRunningTaskCount(ob);
    }
    REQUEST_HILOGI("[RunningTaskCountTest] SubscribeRunningTaskCountTest_001 end");
}

/**
 * @tc.name: SubscribeRunningTaskCountTest_002
 * @tc.desc: Test SubscribeRunningTaskCountTest_002 interface base function - subscribe success
 * @tc.type: FUNC
 * @tc.require: Issue Number
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
 * @tc.name: UnubscribeRunningTaskCountTest_001
 * @tc.desc: Test UnubscribeRunningTaskCountTest_001 interface base function - unsubscribe
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RunningTaskCountTest, UnubscribeRunningTaskCountTest_001, TestSize.Level1)
{
    GTEST_LOG_(INFO) << "RunningTaskCountTest, UnubscribeRunningTaskCountTest_001, TestSize.Level1";
    REQUEST_HILOGI("[RunningTaskCountTest] UnubscribeRunningTaskCountTest_001 begin");
    std::shared_ptr<IRunningTaskObserver> ob1 = std::make_shared<FwkTestOberver>();
    FwkRunningTaskCountManager::GetInstance()->AttachObserver(ob1);

    std::shared_ptr<IRunningTaskObserver> ob2 = std::make_shared<FwkTestOberver>();
    UnsubscribeRunningTaskCount(ob2);
    UnsubscribeRunningTaskCount(ob1);
    REQUEST_HILOGI("[RunningTaskCountTest] UnubscribeRunningTaskCountTest_001 end");
}

/**
 * @tc.name: GetAndSetCount001
 * @tc.desc: Test GetAndSetCount001 interface base function - GetCount/SetCount
 * @tc.type: FUNC
 * @tc.require: Issue Number
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
 * @tc.name: UpdateRunningTaskCountTest001
 * @tc.desc: Test UpdateRunningTaskCountTest001 interface base function - UpdateRunningTaskCount
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RunningTaskCountTest, UpdateRunningTaskCountTest001, TestSize.Level1)
{
    std::shared_ptr<IRunningTaskObserver> ob = std::make_shared<FwkTestOberver>();
    FwkIRunningTaskObserver runningOb = FwkIRunningTaskObserver(ob);
    runningOb.UpdateRunningTaskCount();
}

/**
 * @tc.name: NotifyAllObserversTest001
 * @tc.desc: Test NotifyAllObserversTest001 interface base function - NotifyAllObservers
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RunningTaskCountTest, NotifyAllObserversTest001, TestSize.Level1)
{
    std::shared_ptr<IRunningTaskObserver> ob1 = std::make_shared<FwkTestOberver>();
    FwkRunningTaskCountManager::GetInstance()->AttachObserver(ob1);
    FwkRunningTaskCountManager::GetInstance()->NotifyAllObservers();
    FwkRunningTaskCountManager::GetInstance()->DetachObserver(ob1);
}