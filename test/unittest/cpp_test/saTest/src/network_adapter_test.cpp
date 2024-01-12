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

#include <gtest/gtest.h>

#define private public
#define protected public
#include "network_adapter.h"

using namespace testing::ext;
using namespace OHOS::Request;

#undef private
#undef protected

class NetworkAdapterTest : public testing::Test {
public:
    static void SetUpTestCase(void);
    static void TearDownTestCase(void);
    void SetUp();
    void TearDown();
};

void NetworkAdapterTest::SetUpTestCase(void)
{
    // input testsuit setup step，setup invoked before all testcases
}

void NetworkAdapterTest::TearDownTestCase(void)
{
    // input testsuit teardown step，teardown invoked after all testcases
}

void NetworkAdapterTest::SetUp(void)
{
    // input testcase setup step，setup invoked before each testcases
}

void NetworkAdapterTest::TearDown(void)
{
    // input testcase teardown step，teardown invoked after each testcases
}

// function for testing RegisterNetworkCallback
void ParamFun()
{
    return;
}

/**
 * @tc.name: RegisterNetworkCallbackTest_001
 * @tc.desc: Test RegisterNetworkCallback interface base function
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(NetworkAdapterTest, RegisterNetworkCallbackTest_001, TestSize.Level1)
{
    RegisterNetworkCallback(ParamFun);
}

/**
 * @tc.name: GetNetworkInfoTest_001
 * @tc.desc: Test GetNetworkInfo interface base function
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(NetworkAdapterTest, GetNetworkInfoTest_001, TestSize.Level1)
{
    NetworkInfo* netWorkInfo = GetNetworkInfo();
    EXPECT_EQ(netWorkInfo->networkType, Network::ANY);
    EXPECT_FALSE(netWorkInfo->isMetered);
    EXPECT_FALSE(netWorkInfo->isRoaming);
}

/**
 * @tc.name: NetworkAdapterCoverTest_001
 * @tc.desc: Cover some functions return void
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(NetworkAdapterTest, NetworkAdapterCoverTest_001, TestSize.Level1)
{
    NetworkAdapter::GetInstance().UpdateNetworkInfo();
    NetworkAdapter::GetInstance().UpdateRoaming();
}

