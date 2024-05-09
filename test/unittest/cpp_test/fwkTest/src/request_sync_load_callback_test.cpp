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

#include "request_sync_load_callback.h"

#include <gtest/gtest.h>

#include <cstdint>
#include <memory>
#include <vector>

#include "gmock/gmock.h"
#include "js_common.h"
#include "log.h"
#include "refbase.h"
#include "system_ability_definition.h"

using namespace testing::ext;
using namespace OHOS::Request;

#undef private
#undef protected

class RequestSyncLoadCallbackTest : public testing::Test {
public:
    static void SetUpTestCase(void);
    static void TearDownTestCase(void);
    void SetUp();
    void TearDown();
};

void RequestSyncLoadCallbackTest::SetUpTestCase(void)
{
    // input testSuit setup step，setup invoked before all testCases
}

void RequestSyncLoadCallbackTest::TearDownTestCase(void)
{
    // input testSuit teardown step，teardown invoked after all testCases
}

void RequestSyncLoadCallbackTest::SetUp(void)
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

void RequestSyncLoadCallbackTest::TearDown(void)
{
    // input testCase teardown step，teardown invoked after each testCase
}

class RSLCTRemoteObjectImpl : public OHOS::IRemoteObject {};

/**
 * @tc.name: OnLoadSystemAbilityTest001
 * @tc.desc: Test OnLoadSystemAbilityTest001 interface base function - OnLoadSystemAbilityFail/Success
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestSyncLoadCallbackTest, OnLoadSystemAbility001, TestSize.Level1)
{
    OHOS::sptr<RSLCTRemoteObjectImpl> remote;
    RequestSyncLoadCallback requestSyncLoadCallback = RequestSyncLoadCallback();
    requestSyncLoadCallback.OnLoadSystemAbilityFail(OHOS::PRINT_SERVICE_ID);
    requestSyncLoadCallback.OnLoadSystemAbilityFail(OHOS::DOWNLOAD_SERVICE_ID);
    requestSyncLoadCallback.OnLoadSystemAbilitySuccess(OHOS::PRINT_SERVICE_ID, remote);
    requestSyncLoadCallback.OnLoadSystemAbilitySuccess(OHOS::DOWNLOAD_SERVICE_ID, remote);
}