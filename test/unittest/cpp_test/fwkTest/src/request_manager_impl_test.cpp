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

#include <optional>

#include "constant.h"
#include "refbase.h"
#include "runcount_notify_stub.h"
#include "running_task_count.h"
#define private public
#define protected public

#include <gtest/gtest.h>
#include <sys/socket.h>

#include <cstdint>
#include <memory>
#include <vector>

#include "gmock/gmock.h"
#include "log.h"
#include "request_common.h"
#include "request_manager_impl.h"
#include "request_running_task_count.h"
#include "request_service_proxy.h"
#include "system_ability_definition.h"

using namespace testing::ext;
using namespace OHOS::Request;

#undef private
#undef protected

class MockRequestServiceInterface : public RequestServiceInterface {
public:
    MOCK_METHOD(ExceptionErrorCode, StartTasks,
        (const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets), (override));
    MOCK_METHOD(ExceptionErrorCode, StopTasks,
        (const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets), (override));
    MOCK_METHOD(ExceptionErrorCode, ResumeTasks,
        (const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets), (override));
    MOCK_METHOD(ExceptionErrorCode, RemoveTasks,
        (const std::vector<std::string> &tids, const Version version, std::vector<ExceptionErrorCode> &rets),
        (override));
    MOCK_METHOD(ExceptionErrorCode, PauseTasks,
        (const std::vector<std::string> &tids, const Version version, std::vector<ExceptionErrorCode> &rets),
        (override));
    MOCK_METHOD(ExceptionErrorCode, QueryTasks,
        (const std::vector<std::string> &tids, (std::vector<TaskInfoRet> & rets)), (override));
    MOCK_METHOD(ExceptionErrorCode, ShowTasks,
        (const std::vector<std::string> &tids, (std::vector<TaskInfoRet> & rets)), (override));
    MOCK_METHOD(ExceptionErrorCode, TouchTasks,
        ((const std::vector<TaskIdAndToken> &tidTokens), (std::vector<TaskInfoRet> & rets)), (override));

    MOCK_METHOD(ExceptionErrorCode, SetMode, (const std::string &tid, const Mode mode), (override));
    MOCK_METHOD(int32_t, Create, (const Config &config, std::string &taskId), (override));
    MOCK_METHOD(int32_t, GetTask, (const std::string &tid, const std::string &token, Config &config), (override));
    MOCK_METHOD(int32_t, Start, (const std::string &tid), (override));
    MOCK_METHOD(int32_t, Pause, (const std::string &tid, const Version version), (override));
    MOCK_METHOD(int32_t, QueryMimeType, (const std::string &tid, std::string &mimeType), (override));
    MOCK_METHOD(int32_t, Remove, (const std::string &tid, const Version version), (override));
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
    MOCK_METHOD(int32_t, CreateGroup, (std::string & gid, const bool gauge, Notification &info), (override));
    MOCK_METHOD(int32_t, AttachGroup, (const std::string &gid, const std::vector<std::string> &tid), (override));
    MOCK_METHOD(int32_t, DeleteGroup, (const std::string &gid), (override));
    MOCK_METHOD(int32_t, SetMaxSpeed, (const std::string &tid, const int64_t maxSpeed), (override));
    MOCK_METHOD(ExceptionErrorCode, SetMaxSpeeds,
        (const std::vector<SpeedConfig> &speedConfig, std::vector<ExceptionErrorCode> &rets), (override));
    MOCK_METHOD(ExceptionErrorCode, DisableTaskNotification,
        (const std::vector<std::string> &tids, std::vector<ExceptionErrorCode> &rets));
    MOCK_METHOD(ExceptionErrorCode, CreateTasks, (const std::vector<Config> &configs, std::vector<TaskRet> &rets));
};

class RequestManagerImplTest : public testing::Test {
public:
    static void SetUpTestCase(void);
    static void TearDownTestCase(void);
    void SetUp();
    void TearDown();
    OHOS::sptr<RequestServiceInterface> testProxy;
    OHOS::sptr<MockRequestServiceInterface> exceptProxy;
};

void RequestManagerImplTest::SetUpTestCase(void)
{
    // input testSuit setup step，setup invoked before all testCases
}

void RequestManagerImplTest::TearDownTestCase(void)
{
    // input testSuit teardown step，teardown invoked after all testCases
}

void RequestManagerImplTest::SetUp(void)
{
    exceptProxy = OHOS::sptr<MockRequestServiceInterface>(new MockRequestServiceInterface());
    testProxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    RequestManagerImpl::GetInstance()->requestServiceProxy_ = exceptProxy;
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    EXPECT_TRUE(proxy == (static_cast<OHOS::sptr<RequestServiceInterface>>(exceptProxy)));
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
    RequestManagerImpl::GetInstance()->requestServiceProxy_ = testProxy;
    testProxy = nullptr;
    exceptProxy = nullptr;
}

/**
 * @tc.name: CreateTest001
 * @tc.desc: Test Create interface with channel recovery and task creation scenarios
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Simulate channel broken state
 *           2. Expect OpenChannel to return E_TASK_STATE first, then E_OK
 *           3. Create config with API9 version
 *           4. Expect Create to return E_CHANNEL_NOT_OPEN first, then E_OK
 *           5. Expect Subscribe to return E_OK
 *           6. Expect Start to return E_OK first, then E_CHANNEL_NOT_OPEN
 * @tc.expect: First Create call succeeds (E_OK), second returns E_CHANNEL_NOT_OPEN
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, CreateTest001, TestSize.Level1)
{
    RequestManagerImpl::GetInstance()->OnChannelBroken();
    EXPECT_CALL(*exceptProxy, OpenChannel(testing::_))
        .WillOnce(testing::Return(E_TASK_STATE))
        .WillOnce(testing::Return(E_OK));
    Config config;
    config.version = Version::API9;
    int32_t seq = 1;
    std::string tid = "1";
    EXPECT_CALL(*exceptProxy, Create(testing::_, tid))
        .WillOnce(testing::Return(E_CHANNEL_NOT_OPEN))
        .WillOnce(testing::Return(E_OK));
    EXPECT_CALL(*exceptProxy, Subscribe(testing::_)).WillOnce(testing::Return(E_OK));
    EXPECT_CALL(*exceptProxy, Start(testing::_))
        .WillOnce(testing::Return(E_OK))
        .WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Create(config, seq, tid), E_OK);
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Create(config, seq, tid), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: GetTaskTest001
 * @tc.desc: Test GetTask interface with channel broken and error handling
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Create config, token and tid
 *           2. Simulate channel broken state
 *           3. Expect OpenChannel to return E_CHANNEL_NOT_OPEN repeatedly
 *           4. Expect Subscribe to return E_OK
 *           5. Expect GetTask to return E_CHANNEL_NOT_OPEN first, then E_OTHER
 * @tc.expect: First GetTask call returns E_OK (channel recovery), second returns E_OTHER
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, GetTaskTest001, TestSize.Level1)
{
    Config config;
    string token = "token";
    string tid = "tid";
    RequestManagerImpl::GetInstance()->OnChannelBroken();
    EXPECT_CALL(*exceptProxy, OpenChannel(testing::_)).WillRepeatedly(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_CALL(*exceptProxy, Subscribe(testing::_)).WillOnce(testing::Return(E_OK));
    EXPECT_CALL(*exceptProxy, GetTask(tid, token, testing::_))
        .WillOnce(testing::Return(E_CHANNEL_NOT_OPEN))
        .WillOnce(testing::Return(E_OTHER));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->GetTask(tid, token, config), E_OK);
    EXPECT_EQ(RequestManagerImpl::GetInstance()->GetTask(tid, token, config), E_OTHER);
}

/**
 * @tc.name: StartTest001
 * @tc.desc: Test Start interface with channel not open scenario
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Create test task ID
 *           2. Expect Start to return E_CHANNEL_NOT_OPEN
 *           3. Call Start with the test task ID
 * @tc.expect: Start returns E_CHANNEL_NOT_OPEN as expected
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, StartTest001, TestSize.Level1)
{
    string tid = "tid";
    EXPECT_CALL(*exceptProxy, Start(tid)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Start(tid), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: StopTest001
 * @tc.desc: Test Stop interface with channel not open scenario
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Create test task ID
 *           2. Expect Stop to return E_CHANNEL_NOT_OPEN
 *           3. Call Stop with the test task ID
 * @tc.expect: Stop returns E_CHANNEL_NOT_OPEN as expected
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, StopTest001, TestSize.Level1)
{
    string tid = "tid";
    EXPECT_CALL(*exceptProxy, Stop(tid)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Stop(tid), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: QueryTest001
 * @tc.desc: Test Query interface with channel not open scenario
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Create test task info and task ID
 *           2. Expect Query to return E_CHANNEL_NOT_OPEN
 *           3. Call Query with the test task ID and info
 * @tc.expect: Query returns E_CHANNEL_NOT_OPEN as expected
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, QueryTest001, TestSize.Level1)
{
    TaskInfo info;
    string tid = "tid";
    EXPECT_CALL(*exceptProxy, Query(tid, testing::_)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Query(tid, info), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: TouchTest001
 * @tc.desc: Test Touch interface with channel not open scenario
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Create test task info, task ID, and token
 *           2. Expect Touch to return E_CHANNEL_NOT_OPEN
 *           3. Call Touch with the test parameters
 * @tc.expect: Touch returns E_CHANNEL_NOT_OPEN as expected
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, TouchTest001, TestSize.Level1)
{
    TaskInfo info;
    string tid = "tid";
    string token = "token";
    EXPECT_CALL(*exceptProxy, Touch(tid, token, testing::_)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Touch(tid, token, info), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: SearchTest001
 * @tc.desc: Test Search interface with channel not open scenario
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Create test filter and task ID vector
 *           2. Expect Search to return E_CHANNEL_NOT_OPEN
 *           3. Call Search with the test parameters
 * @tc.expect: Search returns E_CHANNEL_NOT_OPEN as expected
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, SearchTest001, TestSize.Level1)
{
    Filter filter;
    std::vector<std::string> tids;
    EXPECT_CALL(*exceptProxy, Search(testing::_, tids)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Search(filter, tids), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: ShowTest001
 * @tc.desc: Test Show interface with channel not open scenario
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Create test task info and task ID
 *           2. Expect Show to return E_CHANNEL_NOT_OPEN
 *           3. Call Show with the test parameters
 * @tc.expect: Show returns E_CHANNEL_NOT_OPEN as expected
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, ShowTest001, TestSize.Level1)
{
    TaskInfo info;
    string tid = "tid";
    EXPECT_CALL(*exceptProxy, Show(tid, testing::_)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Show(tid, info), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: PauseTest001
 * @tc.desc: Test Pause interface with channel not open scenario
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Create test task ID
 *           2. Expect Pause with API10 version to return E_CHANNEL_NOT_OPEN
 *           3. Call Pause with the test task ID and version
 * @tc.expect: Pause returns E_CHANNEL_NOT_OPEN as expected
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, PauseTest001, TestSize.Level1)
{
    string tid = "tid";
    EXPECT_CALL(*exceptProxy, Pause(tid, Version::API10)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Pause(tid, Version::API10), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: QueryMimeTypeTest001
 * @tc.desc: Test QueryMimeType interface with channel not open scenario
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Create test task ID and mime type
 *           2. Expect QueryMimeType to return E_CHANNEL_NOT_OPEN
 *           3. Call QueryMimeType with the test parameters
 * @tc.expect: QueryMimeType returns E_CHANNEL_NOT_OPEN as expected
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, QueryMimeTypeTest001, TestSize.Level1)
{
    string tid = "tid";
    std::string mimeType = "mimeType";
    EXPECT_CALL(*exceptProxy, QueryMimeType(tid, mimeType)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->QueryMimeType(tid, mimeType), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: RemoveTest001
 * @tc.desc: Test Remove interface with task not found scenario
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Create test task ID
 *           2. Call Remove with the test task ID and API10 version
 * @tc.expect: Remove returns E_TASK_NOT_FOUND as expected for non-existent task
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, RemoveTest001, TestSize.Level1)
{
    string tid = "tid";
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Remove(tid, Version::API10), E_TASK_NOT_FOUND);
}

/**
 * @tc.name: ResumeTest001
 * @tc.desc: Test Resume interface with channel not open scenario
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Create test task ID
 *           2. Expect Resume to return E_CHANNEL_NOT_OPEN
 *           3. Call Resume with the test task ID
 * @tc.expect: Resume returns E_CHANNEL_NOT_OPEN as expected
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, ResumeTest001, TestSize.Level1)
{
    string tid = "tid";
    EXPECT_CALL(*exceptProxy, Resume(tid)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Resume(tid), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: SubscribeTest001
 * @tc.desc: Test Subscribe interface with channel broken and recovery scenarios
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Create test task ID
 *           2. Simulate channel broken state
 *           3. Expect OpenChannel to return E_CHANNEL_NOT_OPEN
 *           4. Expect Subscribe to return E_CHANNEL_NOT_OPEN first, then E_OK
 *           5. Call Subscribe with the test task ID
 * @tc.expect: Subscribe returns E_OK after channel recovery
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, SubscribeTest001, TestSize.Level1)
{
    string taskId = "taskId";
    RequestManagerImpl::GetInstance()->OnChannelBroken();
    EXPECT_CALL(*exceptProxy, OpenChannel(testing::_)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_CALL(*exceptProxy, Subscribe(taskId))
        .WillOnce(testing::Return(E_CHANNEL_NOT_OPEN))
        .WillOnce(testing::Return(E_OK));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Subscribe(taskId), E_OK);
}

/**
 * @tc.name: UnsubscribeTest001
 * @tc.desc: Test Unsubscribe interface with channel not open scenario
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Create test task ID
 *           2. Expect Unsubscribe to return E_CHANNEL_NOT_OPEN
 *           3. Call Unsubscribe with the test task ID
 * @tc.expect: Unsubscribe returns E_CHANNEL_NOT_OPEN as expected
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, UnsubscribeTest001, TestSize.Level1)
{
    string taskId = "taskId";
    EXPECT_CALL(*exceptProxy, Unsubscribe(taskId)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Unsubscribe(taskId), E_CHANNEL_NOT_OPEN);
}

/**
 * @tc.name: SubRunCountTest001
 * @tc.desc: Test SubRunCount and UnsubRunCount interfaces for run count subscription
 * @tc.precon: RequestManagerImpl instance is initialized, RunCountNotifyStub is available
 * @tc.step: 1. Get RunCountNotifyStub instance
 *           2. Call SubRunCount with the listener
 *           3. Verify SubRunCount returns E_OK
 *           4. Call UnsubRunCount
 *           5. Verify UnsubRunCount returns E_OK
 * @tc.expect: Both SubRunCount and UnsubRunCount return E_OK successfully
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, SubRunCountTest001, TestSize.Level1)
{
    auto listener = RunCountNotifyStub::GetInstance();
    EXPECT_EQ(RequestManagerImpl::GetInstance()->SubRunCount(listener), E_OK);
    EXPECT_EQ(RequestManagerImpl::GetInstance()->UnsubRunCount(), E_OK);
}

/**
 * @tc.name: EnsureChannelOpenTest001
 * @tc.desc: Test EnsureChannelOpen interface with channel broken and recovery scenarios
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Simulate channel broken state
 *           2. Expect OpenChannel to return E_CHANNEL_NOT_OPEN first, then E_OK
 *           3. Call EnsureChannelOpen twice
 * @tc.expect: First call returns E_CHANNEL_NOT_OPEN, second returns E_OK after recovery
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, EnsureChannelOpenTest001, TestSize.Level1)
{
    RequestManagerImpl::GetInstance()->OnChannelBroken();
    EXPECT_CALL(*exceptProxy, OpenChannel(testing::_))
        .WillOnce(testing::Return(E_CHANNEL_NOT_OPEN))
        .WillOnce(testing::Return(E_OK));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->EnsureChannelOpen(), E_CHANNEL_NOT_OPEN);
    EXPECT_EQ(RequestManagerImpl::GetInstance()->EnsureChannelOpen(), E_OK);
}

/**
 * @tc.name: OnResponseReceiveTest001
 * @tc.desc: Test OnResponseReceive interface with valid response handling
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Verify mock proxy is not null
 *           2. Create a Response instance
 *           3. Call OnResponseReceive with the response
 * @tc.expect: OnResponseReceive processes the response without errors
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, OnResponseReceiveTest001, TestSize.Level1)
{
    EXPECT_NE(exceptProxy, nullptr);
    std::shared_ptr<Response> response = std::make_shared<Response>();
    RequestManagerImpl::GetInstance()->OnResponseReceive(response);
}

/**
 * @tc.name: OnNotifyDataReceiveTest001
 * @tc.desc: Test OnNotifyDataReceive interface with valid notify data
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Verify mock proxy is not null
 *           2. Create a NotifyData instance
 *           3. Call OnNotifyDataReceive with the notify data
 * @tc.expect: OnNotifyDataReceive processes the data without errors
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, OnNotifyDataReceiveTest001, TestSize.Level1)
{
    EXPECT_NE(exceptProxy, nullptr);
    std::shared_ptr<NotifyData> notifyData = std::make_shared<NotifyData>();
    RequestManagerImpl::GetInstance()->OnNotifyDataReceive(notifyData);
}

/**
 * @tc.name: UnsubscribeSA001
 * @tc.desc: Test UnsubscribeSA interface for system ability unsubscription
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Verify mock proxy is not null
 *           2. Call UnsubscribeSA
 * @tc.expect: UnsubscribeSA executes without errors
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, UnsubscribeSA001, TestSize.Level1)
{
    EXPECT_NE(exceptProxy, nullptr);
    RequestManagerImpl::GetInstance()->UnsubscribeSA();
}

void RMITestCallback()
{
}

/**
 * @tc.name: OnAddSystemAbility001
 * @tc.desc: Test OnAddSystemAbility interface with system ability addition handling
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Create test device ID
 *           2. Create SystemAbilityStatusChangeListener
 *           3. Set up test callback
 *           4. Test with DOWNLOAD_SERVICE_ID
 *           5. Restore null listener
 *           6. Test with PRINT_SERVICE_ID
 * @tc.expect: Listener handles both service IDs appropriately
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, OnAddSystemAbility001, TestSize.Level1)
{
    EXPECT_NE(exceptProxy, nullptr);
    string deviceId = "deviceId";
    RequestManagerImpl::SystemAbilityStatusChangeListener listener =
        RequestManagerImpl::SystemAbilityStatusChangeListener();
    RequestManagerImpl::GetInstance()->RestoreListener(RMITestCallback);
    listener.OnAddSystemAbility(OHOS::DOWNLOAD_SERVICE_ID, deviceId);
    RequestManagerImpl::GetInstance()->RestoreListener(nullptr);
    listener.OnAddSystemAbility(OHOS::PRINT_SERVICE_ID, deviceId);
}

/**
 * @tc.name: OnRemoveSystemAbility001
 * @tc.desc: Test OnRemoveSystemAbility interface with system ability removal handling
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Create test device ID
 *           2. Create SystemAbilityStatusChangeListener
 *           3. Test with DOWNLOAD_SERVICE_ID
 *           4. Test with PRINT_SERVICE_ID
 * @tc.expect: Listener handles both service ID removals appropriately
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, OnRemoveSystemAbility001, TestSize.Level1)
{
    EXPECT_NE(exceptProxy, nullptr);
    string deviceId = "deviceId";
    RequestManagerImpl::SystemAbilityStatusChangeListener listener =
        RequestManagerImpl::SystemAbilityStatusChangeListener();
    listener.OnRemoveSystemAbility(OHOS::DOWNLOAD_SERVICE_ID, deviceId);
    listener.OnRemoveSystemAbility(OHOS::PRINT_SERVICE_ID, deviceId);
}

/**
 * @tc.name: CreateTest002
 * @tc.desc: Test Create interface with multiple error scenarios and channel states
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Simulate channel broken state
 *           2. Expect OpenChannel to return E_TASK_STATE repeatedly
 *           3. Create config with API10 version
 *           4. Expect Create to return E_FILE_PATH, then E_OK twice
 *           5. Verify proxy equality after operations
 * @tc.expect: Create returns appropriate error codes, proxy remains consistent
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, CreateTest002, TestSize.Level1)
{
    RequestManagerImpl::GetInstance()->OnChannelBroken();
    EXPECT_CALL(*exceptProxy, OpenChannel(testing::_)).WillRepeatedly(testing::Return(E_TASK_STATE));
    Config config;
    config.version = Version::API10;
    int32_t seq = 1;
    std::string tid = "1";
    EXPECT_CALL(*exceptProxy, Create(testing::_, tid))
        .WillOnce(testing::Return(E_FILE_PATH))
        .WillOnce(testing::Return(E_OK))
        .WillOnce(testing::Return(E_OK));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Create(config, seq, tid), E_FILE_PATH);
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Create(config, seq, tid), E_OK);
    RequestManagerImpl::GetInstance()->Create(config, seq, tid);
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy(true);
    EXPECT_TRUE(proxy == (static_cast<OHOS::sptr<RequestServiceInterface>>(exceptProxy)));
}

/**
 * @tc.name: SubscribeTest002
 * @tc.desc: Test Subscribe interface with channel recovery scenario
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Create test task ID
 *           2. Simulate channel broken state
 *           3. Expect OpenChannel to return E_CHANNEL_NOT_OPEN
 *           4. Expect Subscribe to return E_OK
 *           5. Call Subscribe with the test task ID
 * @tc.expect: Subscribe succeeds with E_OK despite initial channel issues
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, SubscribeTest002, TestSize.Level1)
{
    string taskId = "taskId";
    RequestManagerImpl::GetInstance()->OnChannelBroken();
    EXPECT_CALL(*exceptProxy, OpenChannel(testing::_)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_CALL(*exceptProxy, Subscribe(taskId)).WillOnce(testing::Return(E_OK));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Subscribe(taskId), E_OK);
}

/**
 * @tc.name: EnsureChannelOpenTest002
 * @tc.desc: Test EnsureChannelOpen interface with existing message receiver
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Create ResponseMessageReceiver instance
 *           2. Set msgReceiver_ to the new instance
 *           3. Call EnsureChannelOpen
 *           4. Simulate channel broken state
 * @tc.expect: EnsureChannelOpen returns E_OK with existing receiver
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, EnsureChannelOpenTest002, TestSize.Level1)
{
    RequestManagerImpl::GetInstance()->msgReceiver_ =
        std::make_shared<ResponseMessageReceiver>(RequestManagerImpl::GetInstance().get(), -1);
    EXPECT_EQ(RequestManagerImpl::GetInstance()->EnsureChannelOpen(), E_OK);
    RequestManagerImpl::GetInstance()->OnChannelBroken();
}

/**
 * @tc.name: GetTaskTest002
 * @tc.desc: Test GetTask interface with task cache management
 * @tc.precon: RequestManagerImpl instance is initialized, task map is accessible
 * @tc.step: 1. Create test task ID
 *           2. Access tasks map and clear the test task ID
 *           3. Call GetTask with the test task ID
 *           4. Verify task is not null
 *           5. Call GetTask again
 *           6. Clear task from map again
 * @tc.expect: GetTask returns non-null task instances consistently
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, GetTaskTest002, TestSize.Level1)
{
    string taskId = "taskId";
    std::map<std::string, std::shared_ptr<Request>> tasks = RequestManagerImpl::GetInstance()->tasks_;
    tasks.erase(taskId);
    std::shared_ptr<Request> task = RequestManagerImpl::GetInstance()->GetTask(taskId);
    EXPECT_NE(task.get(), nullptr);
    task = RequestManagerImpl::GetInstance()->GetTask(taskId);
    EXPECT_NE(task.get(), nullptr);
    tasks.erase(taskId);
}

/**
 * @tc.name: SubscribeSATest001
 * @tc.desc: Test SubscribeSA interface with listener initialization
 * @tc.precon: RequestManagerImpl instance is initialized, saChangeListener_ is initially null
 * @tc.step: 1. Verify saChangeListener_ is initially null
 *           2. Call SubscribeSA
 *           3. Verify saChangeListener_ is no longer null
 *           4. Call SubscribeSA again
 * @tc.expect: SubscribeSA creates listener on first call, handles duplicate calls gracefully
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, SubscribeSATest001, TestSize.Level1)
{
    RequestManagerImpl::GetInstance()->saChangeListener_ = nullptr;
    RequestManagerImpl::GetInstance()->SubscribeSA();
    EXPECT_NE(RequestManagerImpl::GetInstance()->saChangeListener_, nullptr);
    RequestManagerImpl::GetInstance()->SubscribeSA();
}

/**
 * @tc.name: RestoreSubRunCountTest001
 * @tc.desc: Test RestoreSubRunCount interface with null listener handling
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Verify mock proxy is not null
 *           2. Create null NotifyInterface listener
 *           3. Set requestServiceProxy_ to mock
 *           4. Call RestoreSubRunCount
 * @tc.expect: RestoreSubRunCount handles null listener gracefully
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, RestoreSubRunCountTest001, TestSize.Level1)
{
    EXPECT_NE(exceptProxy, nullptr);
    OHOS::sptr<NotifyInterface> listener(nullptr);
    RequestManagerImpl::GetInstance()->requestServiceProxy_ = exceptProxy;
    RequestManagerImpl::GetInstance()->RestoreSubRunCount();
}

class TestRunCountDemo : public IRunningTaskObserver {
public:
    ~TestRunCountDemo() = default;
    void OnRunningTaskCountUpdate(int count) override
    {
        return;
    }
};

/**
 * @tc.name: OnAddSystemAbility002
 * @tc.desc: Test OnAddSystemAbility interface with observer management
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Create test device ID
 *           2. Restore null listener
 *           3. Verify no observers initially
 *           4. Create test observer
 *           5. Add observer to manager
 *           6. Verify observer exists
 *           7. Test with PRINT_SERVICE_ID
 *           8. Clear observers
 * @tc.expect: Listener manages observers correctly for non-download services
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, OnAddSystemAbility002, TestSize.Level1)
{
    EXPECT_NE(exceptProxy, nullptr);
    string deviceId = "deviceId";
    RequestManagerImpl::GetInstance()->RestoreListener(nullptr);
    EXPECT_FALSE(FwkRunningTaskCountManager::GetInstance()->HasObserver());
    std::shared_ptr<IRunningTaskObserver> ob = std::make_shared<TestRunCountDemo>();
    auto pNewFwkOb = std::make_shared<FwkIRunningTaskObserver>(ob);
    FwkRunningTaskCountManager::GetInstance()->observers_.push_back(pNewFwkOb);
    EXPECT_TRUE(FwkRunningTaskCountManager::GetInstance()->HasObserver());
    RequestManagerImpl::SystemAbilityStatusChangeListener listener =
        RequestManagerImpl::SystemAbilityStatusChangeListener();
    listener.OnAddSystemAbility(OHOS::PRINT_SERVICE_ID, deviceId);
    FwkRunningTaskCountManager::GetInstance()->observers_.clear();
}

/**
 * @tc.name: ReopenChannel001
 * @tc.desc: Test ReopenChannel interface with channel reopening scenario
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Verify mock proxy is not null
 *           2. Create ResponseMessageReceiver instance
 *           3. Set msgReceiver_ to new instance
 *           4. Expect OpenChannel to return E_CHANNEL_NOT_OPEN
 *           5. Call ReopenChannel
 * @tc.expect: ReopenChannel handles channel reopening appropriately
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, ReopenChannel001, TestSize.Level1)
{
    EXPECT_NE(exceptProxy, nullptr);
    RequestManagerImpl::GetInstance()->msgReceiver_ =
        std::make_shared<ResponseMessageReceiver>(RequestManagerImpl::GetInstance().get(), -1);
    EXPECT_CALL(*exceptProxy, OpenChannel(testing::_)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    RequestManagerImpl::GetInstance()->ReopenChannel();
}

/**
 * @tc.name: CreateGroup001
 * @tc.desc: Test CreateGroup interface with successful group creation
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Verify mock proxy is not null
 *           2. Create test group ID
 *           3. Set gauge to true
 *           4. Create notification info with text and title
 *           5. Expect CreateGroup to return E_OK
 *           6. Call CreateGroup with test parameters
 * @tc.expect: CreateGroup returns E_OK successfully
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, CreateGroup001, TestSize.Level1)
{
    EXPECT_NE(exceptProxy, nullptr);
    std::string gid = "gid";
    bool gauge = true;
    Notification info{
        .text = "text",
        .title = "title",
        .disable = false,

    };
    EXPECT_CALL(*exceptProxy, CreateGroup(gid, testing::_, testing::_)).WillOnce(testing::Return(E_OK));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->CreateGroup(gid, gauge, info), E_OK);
}

/**
 * @tc.name: AttachGroup001
 * @tc.desc: Test AttachGroup interface with task attachment to group
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Create test group ID
 *           2. Create test task ID vector with multiple tasks
 *           3. Expect AttachGroup to return E_OK
 *           4. Call AttachGroup with test parameters
 *           5. Verify result equals E_OK
 * @tc.expect: AttachGroup returns E_OK successfully
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, AttachGroup001, TestSize.Level1)
{
    std::string gid = "gid";
    std::vector<std::string> tids = { "tid", "1231" };
    EXPECT_CALL(*exceptProxy, AttachGroup(gid, testing::_)).WillOnce(testing::Return(E_OK));
    auto res = RequestManagerImpl::GetInstance()->AttachGroup(gid, tids);
    EXPECT_EQ(res, E_OK);
}

/**
 * @tc.name: DeleteGroup001
 * @tc.desc: Test DeleteGroup interface with successful group deletion
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Create test group ID
 *           2. Expect DeleteGroup to return E_OK
 *           3. Call DeleteGroup with test group ID
 *           4. Verify result equals E_OK
 * @tc.expect: DeleteGroup returns E_OK successfully
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, DeleteGroup001, TestSize.Level1)
{
    std::string gid = "gid";
    EXPECT_CALL(*exceptProxy, DeleteGroup(gid)).WillOnce(testing::Return(E_OK));
    auto res = RequestManagerImpl::GetInstance()->DeleteGroup(gid);
    EXPECT_EQ(res, E_OK);
}

/**
 * @tc.name: QueryTasks001
 * @tc.desc: Test QueryTasks interface with multiple task query
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Verify mock proxy is not null
 *           2. Create test task ID vector with multiple tasks
 *           3. Create result vector for TaskInfoRet
 *           4. Expect QueryTasks to return E_OK
 *           5. Call QueryTasks with test parameters
 * @tc.expect: QueryTasks returns E_OK successfully
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, QueryTasks001, TestSize.Level1)
{
    EXPECT_NE(exceptProxy, nullptr);
    std::vector<std::string> tids = { "tid", "123" };
    std::vector<TaskInfoRet> rets;
    EXPECT_CALL(*exceptProxy, QueryTasks(tids, testing::_)).WillOnce(testing::Return(E_OK));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->QueryTasks(tids, rets), E_OK);
}

/**
 * @tc.name: CreateWithNotificationTest001
 * @tc.desc: Test Create interface with notification configuration
 * @tc.precon: RequestManagerImpl instance is initialized, mock proxy is set up
 * @tc.step: 1. Create config with API9 version and notification settings
 *           2. Set notification properties: text, title, disable, visibility
 *           3. Expect Create to return E_OK
 *           4. Call Create method with the configured parameters
 * @tc.expect: Create returns E_OK successfully
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerImplTest, CreateWithNotificationTest001, TestSize.Level1)
{
    RequestManagerImpl::GetInstance()->OnChannelBroken();
    EXPECT_CALL(*exceptProxy, OpenChannel(testing::_))
        .WillOnce(testing::Return(E_TASK_STATE))
        .WillOnce(testing::Return(E_OK));
    Config config;
    config.version = Version::API9;

    config.notification.text = "text";
    config.notification.title = "title";
    config.notification.disable = false;
    config.notification.visibility = VISIBILITY_COMPLETION;

    int32_t seq = 1;
    std::string tid = "1";
    EXPECT_CALL(*exceptProxy, Create(testing::_, tid))
        .WillOnce(testing::Return(E_CHANNEL_NOT_OPEN))
        .WillOnce(testing::Return(E_OK));
    EXPECT_CALL(*exceptProxy, Subscribe(testing::_)).WillOnce(testing::Return(E_OK));
    EXPECT_CALL(*exceptProxy, Start(testing::_))
        .WillOnce(testing::Return(E_OK))
        .WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Create(config, seq, tid), E_OK);
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Create(config, seq, tid), E_CHANNEL_NOT_OPEN);
}
