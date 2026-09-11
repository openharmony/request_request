/*
 * Copyright (C) 2026 Huawei Device Co., Ltd.
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

#include "multi_instance_runcount_manager.h"
#include "runcount_notify_stub.h"

#include <gtest/gtest.h>

#include <cstdint>
#include <memory>

#undef private
#undef protected

#include "request_running_task_count.h"
#include "message_parcel.h"
#include "refbase.h"

using namespace testing::ext;
using namespace OHOS::Request;

namespace {
// Access to the private per-pid map and the re-entrancy guard, reachable
// because the header was included with `#define private public`.
void ResetManagerState()
{
    std::lock_guard<std::mutex> lock(MultiInstanceRunCountManager::GetInstance().lock_);
    MultiInstanceRunCountManager::GetInstance().pidCountMap_.clear();
    MultiInstanceRunCountManager::GetInstance().rebuilding_.store(false);
}
} // namespace

class MultiInstanceTest : public testing::Test {
public:
    static void SetUpTestCase(void) {}
    static void TearDownTestCase(void) {}
    void SetUp() override { ResetManagerState(); }
    void TearDown() override { ResetManagerState(); }
};

/**
 * @tc.name: MultiInstanceSetCountByPid001
 * @tc.desc: SetCountByPid records one instance; GetTotalCount returns it
 * @tc.type: FUNC
 * @tc.level: Level 0
 */
HWTEST_F(MultiInstanceTest, SetCountByPid001, TestSize.Level0)
{
    auto &mgr = MultiInstanceRunCountManager::GetInstance();
    mgr.SetCountByPid(100, 3);
    EXPECT_EQ(mgr.GetTotalCount(), 3);
}

/**
 * @tc.name: MultiInstanceSetCountByPid002
 * @tc.desc: Multiple pids aggregate by summation
 * @tc.type: FUNC
 * @tc.level: Level 0
 */
HWTEST_F(MultiInstanceTest, SetCountByPid002, TestSize.Level0)
{
    auto &mgr = MultiInstanceRunCountManager::GetInstance();
    mgr.SetCountByPid(100, 3);
    mgr.SetCountByPid(101, 5);
    mgr.SetCountByPid(102, 7);
    EXPECT_EQ(mgr.GetTotalCount(), 15);
}

/**
 * @tc.name: MultiInstanceSetCountByPid003
 * @tc.desc: Zero-count instance is dropped from the map
 * @tc.type: FUNC
 * @tc.level: Level 1
 */
HWTEST_F(MultiInstanceTest, SetCountByPid003, TestSize.Level1)
{
    auto &mgr = MultiInstanceRunCountManager::GetInstance();
    mgr.SetCountByPid(100, 0);
    mgr.SetCountByPid(101, 4);
    EXPECT_EQ(mgr.GetTotalCount(), 4);
    // Zero-count pid is dropped, only the live one remains.
    EXPECT_EQ(mgr.pidCountMap_.size(), 1);
    EXPECT_EQ(mgr.pidCountMap_.count(100), 0);
    EXPECT_EQ(mgr.pidCountMap_.at(101), 4);
}

/**
 * @tc.name: MultiInstanceSetCountByPid004
 * @tc.desc: Re-push from the same pid overwrites, not adds
 * @tc.type: FUNC
 * @tc.level: Level 1
 */
HWTEST_F(MultiInstanceTest, SetCountByPid004, TestSize.Level1)
{
    auto &mgr = MultiInstanceRunCountManager::GetInstance();
    mgr.SetCountByPid(100, 3);
    mgr.SetCountByPid(100, 8);
    EXPECT_EQ(mgr.GetTotalCount(), 8);
    EXPECT_EQ(mgr.pidCountMap_.size(), 1);
}

/**
 * @tc.name: MultiInstanceGetTotalCountEmpty
 * @tc.desc: No instances reported => total is zero; explicit map clear keeps
 *           it zero and leaves no stale pids
 * @tc.type: FUNC
 * @tc.level: Level 1
 */
HWTEST_F(MultiInstanceTest, GetTotalCountEmpty, TestSize.Level1)
{
    auto &mgr = MultiInstanceRunCountManager::GetInstance();
    // Fresh manager state (SetUp clears the map): total must be zero.
    EXPECT_EQ(mgr.GetTotalCount(), 0);
    // Re-clearing an already empty map is a no-op, still zero.
    mgr.pidCountMap_.clear();
    EXPECT_EQ(mgr.GetTotalCount(), 0);
    // No stale pids linger after an empty aggregate.
    EXPECT_EQ(mgr.pidCountMap_.size(), 0);
}

/**
 * @tc.name: MultiInstanceRestoreSubRunCountReentrancy
 * @tc.desc: Second RestoreSubRunCount while a rebuild is in progress returns
 *           immediately and does not clear the per-pid map.
 * @tc.type: FUNC
 * @tc.level: Level 2
 */
HWTEST_F(MultiInstanceTest, RestoreSubRunCountReentrancy, TestSize.Level2)
{
    auto &mgr = MultiInstanceRunCountManager::GetInstance();
    mgr.SetCountByPid(100, 5);
    // Simulate an in-progress rebuild: a second call must bail out untouched.
    mgr.rebuilding_.store(true);
    mgr.RestoreSubRunCount();
    EXPECT_TRUE(mgr.rebuilding_.load());
    EXPECT_EQ(mgr.GetTotalCount(), 5);
}

/**
 * @tc.name: MultiInstanceRestoreSubRunCountGuardReleased
 * @tc.desc: Once the guard is free, RestoreSubRunCount attempts a rebuild and
 *           releases the guard on the SAM-missing path without throwing.
 * @tc.type: FUNC
 * @tc.level: Level 2
 */
HWTEST_F(MultiInstanceTest, RestoreSubRunCountGuardReleased, TestSize.Level2)
{
    auto &mgr = MultiInstanceRunCountManager::GetInstance();
    // Guard is free; the call runs (SAM may be unavailable in UT) and must
    // release the guard on every exit path without throwing.
    mgr.RestoreSubRunCount();
    EXPECT_FALSE(mgr.rebuilding_.load());
}

/**
 * @tc.name: MultiInstanceRebuildFromSaListClearsMap
 * @tc.desc: RebuildFromSaList with an empty list clears stale counts and
 *           returns E_OK (no instances to subscribe)
 * @tc.type: FUNC
 * @tc.level: Level 1
 */
HWTEST_F(MultiInstanceTest, RebuildFromSaListClearsMap, TestSize.Level1)
{
    auto &mgr = MultiInstanceRunCountManager::GetInstance();
    mgr.SetCountByPid(100, 3);
    mgr.SetCountByPid(101, 5);
    EXPECT_EQ(mgr.GetTotalCount(), 8);

    // No running instances: rebuild clears the stale per-pid map.
    std::vector<OHOS::sptr<OHOS::IRemoteObject>> emptyList;
    EXPECT_EQ(mgr.RebuildFromSaList(emptyList), E_OK);
    EXPECT_EQ(mgr.GetTotalCount(), 0);
    EXPECT_EQ(mgr.pidCountMap_.size(), 0);
}

/**
 * @tc.name: MultiInstanceOnCallBackAggregatesByPid
 * @tc.desc: OnCallBack (the SubRunCount push-back entry) records the caller's
 *           pid into the per-pid map; in UT the caller pid is the test process.
 * @tc.type: FUNC
 * @tc.level: Level 1
 */
HWTEST_F(MultiInstanceTest, OnCallBackAggregatesByPid, TestSize.Level1)
{
    auto &mgr = MultiInstanceRunCountManager::GetInstance();
    OHOS::MessageParcel parcel;
    parcel.WriteInt64(6);
    RunCountNotifyStub::GetInstance()->OnCallBack(parcel);
    EXPECT_EQ(mgr.GetTotalCount(), 6);
    // OnCallBack records under IPCSkeleton::GetCallingPid() (test process pid).
    EXPECT_EQ(mgr.pidCountMap_.size(), 1);
    EXPECT_EQ(mgr.pidCountMap_.at(static_cast<int32_t>(getpid())), 6);
}

/**
 * @tc.name: MultiInstanceFwkGetCountAggregates
 * @tc.desc: FwkRunningTaskCountManager::GetCount() delegates to the per-pid
 *           aggregation under SUPPORT_MULTI_INSTANCE.
 * @tc.type: FUNC
 * @tc.level: Level 1
 */
HWTEST_F(MultiInstanceTest, FwkGetCountAggregates, TestSize.Level1)
{
    auto &mgr = MultiInstanceRunCountManager::GetInstance();
    mgr.SetCountByPid(100, 3);
    mgr.SetCountByPid(101, 5);
    EXPECT_EQ(FwkRunningTaskCountManager::GetInstance()->GetCount(), 8);
}
