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

#include "net_all_capabilities.h"
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
static void ParamFun()
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
    NetworkInfo *netWorkInfo = GetNetworkInfo();
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

/**
 * @tc.name: NetworkAdapterIsOnlineTest_001
 * @tc.desc: Cover some functions return void
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(NetworkAdapterTest, NetworkAdapterIsOnlineTest_001, TestSize.Level1)
{
    IsOnline();
    NetworkAdapter::GetInstance().IsOnline();

    OHOS::NetManagerStandard::NetAllCapabilities capabilities;
    NetworkAdapter::GetInstance().UpdateNetworkInfoInner(capabilities);
}

/**
 * @tc.name: NetAvailableTest_001
 * @tc.desc: Cover some functions return void
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(NetworkAdapterTest, NetAvailableTest_001, TestSize.Level1)
{
    NetworkAdapter network = NetworkAdapter();
    OHOS::sptr<OHOS::NetManagerStandard::NetHandle> netHandle;
    OHOS::Request::NetworkAdapter::NetConnCallbackObserver ob =
        OHOS::Request::NetworkAdapter::NetConnCallbackObserver(network);
    ob.NetAvailable(netHandle);
}

/**
 * @tc.name: NetConnectionPropertiesChangeTest_001
 * @tc.desc: Cover some functions return void
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(NetworkAdapterTest, NetConnectionPropertiesChangeTest_001, TestSize.Level1)
{
    NetworkAdapter network = NetworkAdapter();
    OHOS::sptr<OHOS::NetManagerStandard::NetHandle> netHandle;
    OHOS::sptr<OHOS::NetManagerStandard::NetLinkInfo> info;
    OHOS::Request::NetworkAdapter::NetConnCallbackObserver ob =
        OHOS::Request::NetworkAdapter::NetConnCallbackObserver(network);
    ob.NetConnectionPropertiesChange(netHandle, info);
}

/**
 * @tc.name: NetUnavailableTest_001
 * @tc.desc: Cover some functions return void
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(NetworkAdapterTest, NetUnavailableTest_001, TestSize.Level1)
{
    NetworkAdapter network = NetworkAdapter();
    OHOS::Request::NetworkAdapter::NetConnCallbackObserver ob =
        OHOS::Request::NetworkAdapter::NetConnCallbackObserver(network);
    ob.NetUnavailable();
}

/**
 * @tc.name: NetBlockStatusChangeTest_001
 * @tc.desc: Cover some functions return void
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(NetworkAdapterTest, NetBlockStatusChangeTest_001, TestSize.Level1)
{
    NetworkAdapter network = NetworkAdapter();
    OHOS::sptr<OHOS::NetManagerStandard::NetHandle> netHandle;
    OHOS::sptr<OHOS::NetManagerStandard::NetLinkInfo> info;
    OHOS::Request::NetworkAdapter::NetConnCallbackObserver ob =
        OHOS::Request::NetworkAdapter::NetConnCallbackObserver(network);
    ob.NetBlockStatusChange(netHandle, info);
}

void RegCallBackTest()
{
}

/**
 * @tc.name: NetLostTest_001
 * @tc.desc: Cover some functions return void
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(NetworkAdapterTest, NetLostTest_001, TestSize.Level1)
{
    NetworkAdapter network = NetworkAdapter();
    network.RegOnNetworkChange(RegCallBackTest);
    OHOS::sptr<OHOS::NetManagerStandard::NetHandle> netHandle;
    OHOS::Request::NetworkAdapter::NetConnCallbackObserver ob =
        OHOS::Request::NetworkAdapter::NetConnCallbackObserver(network);
    ob.NetLost(netHandle);
    auto networkInfo = network.GetNetworkInfo();
    EXPECT_EQ(networkInfo->networkType, NetworkInner::NET_LOST);
    EXPECT_FALSE(networkInfo->isMetered);
    EXPECT_FALSE(network.IsOnline());
}

/**
 * @tc.name: NetCapabilitiesChangeTest_001
 * @tc.desc: Cover some functions return void
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(NetworkAdapterTest, NetCapabilitiesChangeTest_001, TestSize.Level1)
{
    OHOS::sptr<OHOS::NetManagerStandard::NetAllCapabilities> capabilities(
        new OHOS::NetManagerStandard::NetAllCapabilities());
    NetworkAdapter network = NetworkAdapter();
    network.UpdateNetworkInfoInner(*capabilities);
    network.RegOnNetworkChange(RegCallBackTest);
    OHOS::sptr<OHOS::NetManagerStandard::NetHandle> netHandle;
    OHOS::Request::NetworkAdapter::NetConnCallbackObserver ob =
        OHOS::Request::NetworkAdapter::NetConnCallbackObserver(network);
    ob.NetCapabilitiesChange(netHandle, capabilities);
}