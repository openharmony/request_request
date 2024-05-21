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

#include "constant.h"
#define private public
#define protected public

#include <gtest/gtest.h>

#include <cstdint>
#include <memory>
#include <vector>

#include "gmock/gmock.h"
#include "js_common.h"
#include "log.h"
#include "request_manager_impl.h"

using namespace testing::ext;
using namespace OHOS::Request;

#undef private
#undef protected

class MockRequestServiceInterface : public RequestServiceInterface {
public:
    MOCK_METHOD(int32_t, Create, (const Config &config, std::string &taskId), (override));
    MOCK_METHOD(int32_t, GetTask, (const std::string &tid, const std::string &token, Config &config), (override));
    MOCK_METHOD(int32_t, Start, (const std::string &tid), (override));
    MOCK_METHOD(int32_t, Pause, (const std::string &tid, Version version), (override));
    MOCK_METHOD(int32_t, QueryMimeType, (const std::string &tid, std::string &mimeType), (override));
    MOCK_METHOD(int32_t, Remove, (const std::string &tid, Version version), (override));
    MOCK_METHOD(int32_t, Resume, (const std::string &tid), (override));
    MOCK_METHOD(int32_t, Stop, (const std::string &tid), (override));
    MOCK_METHOD(int32_t, Query, (const std::string &tid, TaskInfo &info), (override));
    MOCK_METHOD(int32_t, Touch, (const std::string &tid, const std::string &token, TaskInfo &info), (override));
    MOCK_METHOD(int32_t, Search, (const Filter &filter, std::vector<std::string> &tids), (override));
    MOCK_METHOD(int32_t, Show, (const std::string &tid, TaskInfo &info), (override));

    MOCK_METHOD(int32_t, OpenChannel, (int32_t & sockFd), (override));
    MOCK_METHOD(int32_t, Subscribe, (const std::string &taskId), (override));
    MOCK_METHOD(int32_t, Unsubscribe, (const std::string &taskId), (override));
    MOCK_METHOD(int32_t, SubRunCount, (const OHOS::sptr<NotifyInterface> &listener), (override));
    MOCK_METHOD(int32_t, UnsubRunCount, (), (override));
    MOCK_METHOD(OHOS::sptr<OHOS::IRemoteObject>, AsObject, (), (override));
};

OHOS::sptr<RequestServiceInterface> g_testProxy(nullptr);
OHOS::sptr<MockRequestServiceInterface> g_exceptProxy(nullptr);

class RequestManagerImplTest : public testing::Test {
public:
    static void SetUpTestCase(void);
    static void TearDownTestCase(void);
    void SetUp();
    void TearDown();
};

void RequestManagerImplTest::SetUpTestCase(void)
{
    // input testSuit setup step，setup invoked before all testCases
    g_exceptProxy = OHOS::sptr<MockRequestServiceInterface>(new MockRequestServiceInterface());
}

void RequestManagerImplTest::TearDownTestCase(void)
{
    // input testSuit teardown step，teardown invoked after all testCases
}

void RequestManagerImplTest::SetUp(void)
{
    g_testProxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy();
    RequestManagerImpl::GetInstance()->SetRequestServiceProxy(g_exceptProxy);
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy();
    EXPECT_TRUE(proxy == (static_cast<OHOS::sptr<RequestServiceInterface>>(g_exceptProxy)));
    // input testCase setup step，setup invoked before each testCase
    testing::UnitTest *test = testing::UnitTest::GetInstance();
    ASSERT_NE(test, nullptr);
    const testing::TestInfo *testInfo = test->current_test_info();
    ASSERT_NE(testInfo, nullptr);
    string testCaseName = string(testInfo->name());
    REQUEST_HILOGI("[SetUp] %{public}s start", testCaseName.c_str());
    GTEST_LOG_(INFO) << testCaseName.append(" start");
}

void RequestManagerImplTest::TearDown(void)
{
    // input testCase teardown step，teardown invoked after each testCase
    RequestManagerImpl::GetInstance()->SetRequestServiceProxy(g_testProxy);
    g_testProxy = nullptr;
}

/**
 * @tc.name: CreateTest001
 * @tc.desc: Test CreateTest001 interface base function - Create
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, CreateTest001, TestSize.Level1)
{
    EXPECT_CALL(*g_exceptProxy, OpenChannel(testing::_)).WillRepeatedly(testing::Return(E_TASK_STATE));
    Config config;
    config.version = Version::API9;
    int32_t seq = 1;
    std::string tid = "1";
    EXPECT_CALL(*g_exceptProxy, Create(testing::_, tid))
        .WillOnce(testing::Return(E_CHANNEL_NOT_OPEN))
        .WillOnce(testing::Return(E_OK));
    EXPECT_CALL(*g_exceptProxy, Start(testing::_)).WillOnce(testing::Return(E_OK));
    EXPECT_CALL(*g_exceptProxy, Start(testing::_))
        .WillOnce(testing::Return(E_OK))
        .WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Create(config, seq, tid), E_OK);
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Create(config, seq, tid), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: GetTaskTest001
 * @tc.desc: Test GetTaskTest001 interface base function - GetTask
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, GetTaskTest001, TestSize.Level1)
{
    Config config;
    config.version = Version::API9;
    string token = "token";
    string tid = "tid";
    EXPECT_CALL(*g_exceptProxy, GetTask(tid, token, testing::_)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->GetTask(tid, token, config), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: StartTest001
 * @tc.desc: Test StartTest001 interface base function - Start
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, StartTest001, TestSize.Level1)
{
    string tid = "tid";
    EXPECT_CALL(*g_exceptProxy, Start(tid)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Start(tid), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: StopTest001
 * @tc.desc: Test StopTest001 interface base function - Stop
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, StopTest001, TestSize.Level1)
{
    string tid = "tid";
    EXPECT_CALL(*g_exceptProxy, Stop(tid)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Stop(tid), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: QueryTest001
 * @tc.desc: Test QueryTest001 interface base function - Query
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, QueryTest001, TestSize.Level1)
{
    TaskInfo info;
    string tid = "tid";
    EXPECT_CALL(*g_exceptProxy, Query(tid, testing::_)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Query(tid, info), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: TouchTest001
 * @tc.desc: Test TouchTest001 interface base function - Touch
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, TouchTest001, TestSize.Level1)
{
    TaskInfo info;
    string tid = "tid";
    string token = "token";
    EXPECT_CALL(*g_exceptProxy, Touch(tid, token, testing::_)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Touch(tid, token, info), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: SearchTest001
 * @tc.desc: Test SearchTest001 interface base function - Search
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, SearchTest001, TestSize.Level1)
{
    Filter filter;
    std::vector<std::string> tids;
    EXPECT_CALL(*g_exceptProxy, Search(testing::_, tids)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Search(filter, tids), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: ShowTest001
 * @tc.desc: Test ShowTest001 interface base function - Show
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, ShowTest001, TestSize.Level1)
{
    TaskInfo info;
    string tid = "tid";
    EXPECT_CALL(*g_exceptProxy, Show(tid, testing::_)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Show(tid, info), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: PauseTest001
 * @tc.desc: Test PauseTest001 interface base function - Pause
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, PauseTest001, TestSize.Level1)
{
    string tid = "tid";
    EXPECT_CALL(*g_exceptProxy, Pause(tid, Version::API10)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Pause(tid, Version::API10), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: QueryMimeTypeTest001
 * @tc.desc: Test QueryMimeTypeTest001 interface base function - QueryMimeType
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, QueryMimeTypeTest001, TestSize.Level1)
{
    string tid = "tid";
    std::string mimeType = "mimeType";
    EXPECT_CALL(*g_exceptProxy, QueryMimeType(tid, mimeType)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->QueryMimeType(tid, mimeType), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: RemoveTest001
 * @tc.desc: Test RemoveTest001 interface base function - Remove
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, RemoveTest001, TestSize.Level1)
{
    string tid = "tid";
    EXPECT_CALL(*g_exceptProxy, Remove(tid, Version::API10)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Remove(tid, Version::API10), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: ResumeTest001
 * @tc.desc: Test ResumeTest001 interface base function - Resume
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, ResumeTest001, TestSize.Level1)
{
    string tid = "tid";
    EXPECT_CALL(*g_exceptProxy, Resume(tid)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Resume(tid), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: SubscribeTest001
 * @tc.desc: Test SubscribeTest001 interface base function - Subscribe
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, SubscribeTest001, TestSize.Level1)
{
    string taskId = "taskId";
    EXPECT_CALL(*g_exceptProxy, Subscribe(taskId))
        .WillOnce(testing::Return(E_CHANNEL_NOT_OPEN))
        .WillOnce(testing::Return(E_TASK_STATE));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Subscribe(taskId), E_TASK_STATE);
}

/**
 * @tc.name: UnsubscribeTest001
 * @tc.desc: Test UnsubscribeTest001 interface base function - Unsubscribe
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, UnsubscribeTest001, TestSize.Level1)
{
    string taskId = "taskId";
    EXPECT_CALL(*g_exceptProxy, Unsubscribe(taskId)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Unsubscribe(taskId), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: SubRunCountTest001
 * @tc.desc: Test SubRunCountTest001 interface base function - SubRunCount
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, SubRunCountTest001, TestSize.Level1)
{
    OHOS::sptr<NotifyInterface> listener(nullptr);
    EXPECT_CALL(*g_exceptProxy, SubRunCount(testing::_)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->SubRunCount(listener), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: UnsubRunCountTest001
 * @tc.desc: Test UnsubRunCountTest001 interface base function - UnsubRunCount
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, UnsubRunCountTest001, TestSize.Level1)
{
    EXPECT_CALL(*g_exceptProxy, UnsubRunCount()).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->UnsubRunCount(), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: EnsureChannelOpenTest001
 * @tc.desc: Test EnsureChannelOpenTest001 interface base function - EnsureChannelOpen
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, EnsureChannelOpenTest001, TestSize.Level1)
{
    EXPECT_CALL(*g_exceptProxy, OpenChannel(testing::_))
        .WillOnce(testing::Return(E_CHANNEL_NOT_OPEN))
        .WillOnce(testing::Return(E_OK));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->EnsureChannelOpen(), E_CHANNEL_NOT_OPEN);
    EXPECT_EQ(RequestManagerImpl::GetInstance()->EnsureChannelOpen(), E_OK);
}

/**
 * @tc.name: OnChannelBrokenTest001
 * @tc.desc: Test OnChannelBrokenTest001 interface base function - OnChannelBroken
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, OnChannelBrokenTest001, TestSize.Level1)
{
    RequestManagerImpl::GetInstance()->OnChannelBroken();
}

/**
 * @tc.name: OnResponseReceiveTest001
 * @tc.desc: Test OnResponseReceiveTest001 interface base function - OnResponseReceive
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, OnResponseReceiveTest001, TestSize.Level1)
{
    std::shared_ptr<Response> response = std::make_shared<Response>();
    RequestManagerImpl::GetInstance()->OnResponseReceive(response);
}

/**
 * @tc.name: OnNotifyDataReceiveTest001
 * @tc.desc: Test OnNotifyDataReceiveTest001 interface base function - OnNotifyDataReceive
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, OnNotifyDataReceiveTest001, TestSize.Level1)
{
    std::shared_ptr<NotifyData> notifyData = std::make_shared<NotifyData>();
    RequestManagerImpl::GetInstance()->OnNotifyDataReceive(notifyData);
}

/**
 * @tc.name: RestoreSubRunCountTest001
 * @tc.desc: Test RestoreSubRunCountTest001 interface base function - RestoreSubRunCount
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, RestoreSubRunCountTest001, TestSize.Level1)
{
    OHOS::sptr<NotifyInterface> listener(nullptr);
    EXPECT_CALL(*g_exceptProxy, SubRunCount(testing::_)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    RequestManagerImpl::GetInstance()->SetRequestServiceProxy(g_exceptProxy);
    RequestManagerImpl::GetInstance()->RestoreSubRunCount();
}

/**
 * @tc.name: OnRemoteSaDiedTest001
 * @tc.desc: Test OnRemoteSaDiedTest001 interface base function - OnRemoteSaDied
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, OnRemoteSaDiedTest001, TestSize.Level1)
{
    OHOS::wptr<OHOS::IRemoteObject> remote;
    RequestManagerImpl::GetInstance()->OnRemoteSaDied(remote);
}

/**
 * @tc.name: OnRemoteDiedTest001
 * @tc.desc: Test OnRemoteDiedTest001 interface base function - OnRemoteDied
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, OnRemoteDiedTest001, TestSize.Level1)
{
    OHOS::wptr<OHOS::IRemoteObject> remote;
    RequestSaDeathRecipient recipient = RequestSaDeathRecipient();
    recipient.OnRemoteDied(remote);
}