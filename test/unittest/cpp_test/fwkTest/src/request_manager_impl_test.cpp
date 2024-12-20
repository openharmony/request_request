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
#include "refbase.h"
#define private public
#define protected public

#include <gtest/gtest.h>
#include <sys/socket.h>

#include <cstdint>
#include <memory>
#include <vector>

#include "gmock/gmock.h"
#include "js_common.h"
#include "log.h"
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
    testProxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy();
    RequestManagerImpl::GetInstance()->SetRequestServiceProxy(exceptProxy);
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy();
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
    RequestManagerImpl::GetInstance()->SetRequestServiceProxy(testProxy);
    testProxy = nullptr;
    exceptProxy = nullptr;
}

/**
 * @tc.name: CreateTest001
 * @tc.desc: Test CreateTest001 interface base function - Create
 * @tc.type: FUNC
 * @tc.require: Issue Number
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
 * @tc.desc: Test GetTaskTest001 interface base function - GetTask
 * @tc.type: FUNC
 * @tc.require: Issue Number
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
        .WillOnce(testing::Return(E_UNLOADING_SA))
        .WillOnce(testing::Return(E_CHANNEL_NOT_OPEN))
        .WillOnce(testing::Return(E_OTHER));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->GetTask(tid, token, config), E_UNLOADING_SA);
    EXPECT_EQ(RequestManagerImpl::GetInstance()->GetTask(tid, token, config), E_OK);
    EXPECT_EQ(RequestManagerImpl::GetInstance()->GetTask(tid, token, config), E_OTHER);
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
    EXPECT_CALL(*exceptProxy, Start(tid)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
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
    EXPECT_CALL(*exceptProxy, Stop(tid)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
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
    EXPECT_CALL(*exceptProxy, Query(tid, testing::_)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
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
    EXPECT_CALL(*exceptProxy, Touch(tid, token, testing::_)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
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
    EXPECT_CALL(*exceptProxy, Search(testing::_, tids)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
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
    EXPECT_CALL(*exceptProxy, Show(tid, testing::_)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
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
    EXPECT_CALL(*exceptProxy, Pause(tid, Version::API10)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
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
    EXPECT_CALL(*exceptProxy, QueryMimeType(tid, mimeType)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
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
    EXPECT_CALL(*exceptProxy, Remove(tid, Version::API10)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
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
    EXPECT_CALL(*exceptProxy, Resume(tid)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
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
    RequestManagerImpl::GetInstance()->OnChannelBroken();
    EXPECT_CALL(*exceptProxy, OpenChannel(testing::_)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    EXPECT_CALL(*exceptProxy, Subscribe(taskId))
        .WillOnce(testing::Return(E_CHANNEL_NOT_OPEN))
        .WillOnce(testing::Return(E_OK));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Subscribe(taskId), E_OK);
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
    EXPECT_CALL(*exceptProxy, Unsubscribe(taskId)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
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
    EXPECT_CALL(*exceptProxy, SubRunCount(testing::_)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
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
    EXPECT_CALL(*exceptProxy, UnsubRunCount()).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
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
    RequestManagerImpl::GetInstance()->OnChannelBroken();
    EXPECT_CALL(*exceptProxy, OpenChannel(testing::_))
        .WillOnce(testing::Return(E_CHANNEL_NOT_OPEN))
        .WillOnce(testing::Return(E_OK));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->EnsureChannelOpen(), E_CHANNEL_NOT_OPEN);
    EXPECT_EQ(RequestManagerImpl::GetInstance()->EnsureChannelOpen(), E_OK);
}

/**
 * @tc.name: OnResponseReceiveTest001
 * @tc.desc: Test OnResponseReceiveTest001 interface base function - OnResponseReceive
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, OnResponseReceiveTest001, TestSize.Level1)
{
    EXPECT_NE(exceptProxy, nullptr);
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
    EXPECT_NE(exceptProxy, nullptr);
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
    EXPECT_NE(exceptProxy, nullptr);
    OHOS::sptr<NotifyInterface> listener(nullptr);
    EXPECT_CALL(*exceptProxy, SubRunCount(testing::_)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    RequestManagerImpl::GetInstance()->SetRequestServiceProxy(exceptProxy);
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
    EXPECT_NE(exceptProxy, nullptr);
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
    EXPECT_NE(exceptProxy, nullptr);
    OHOS::wptr<OHOS::IRemoteObject> remote;
    RequestSaDeathRecipient recipient = RequestSaDeathRecipient();
    recipient.OnRemoteDied(remote);
}

/**
 * @tc.name: RetryTest001
 * @tc.desc: Test RetryTest001 interface base function - Retry
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, RetryTest001, TestSize.Level1)
{
    EXPECT_NE(exceptProxy, nullptr);
    std::string taskId;
    Config config;
    int32_t errorCode = E_TASK_STATE;
    FileSpec file;
    file.uri = "uri";
    RequestManagerImpl::GetInstance()->Retry(taskId, config, errorCode);
    config.files.push_back(file);
    RequestManagerImpl::GetInstance()->Retry(taskId, config, errorCode);
}

/**
 * @tc.name: UnsubscribeSA001
 * @tc.desc: Test UnsubscribeSA001 interface base function - UnsubscribeSA
 * @tc.type: FUNC
 * @tc.require: Issue Number
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
 * @tc.desc: Test OnAddSystemAbility001 interface base function - OnAddSystemAbility
 * @tc.type: FUNC
 * @tc.require: Issue Number
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
 * @tc.desc: Test OnRemoveSystemAbility001 interface base function - OnRemoveSystemAbility
 * @tc.type: FUNC
 * @tc.require: Issue Number
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
 * @tc.name: GetVectorData001
 * @tc.desc: Test GetVectorData001 interface base function - GetVectorData
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, GetVectorData001, TestSize.Level1)
{
    int32_t fd[2] = { -1, -1 }; // 4 socket for socketpair
    Config config;
    OHOS::MessageParcel data;
    EXPECT_TRUE(socketpair(AF_UNIX, SOCK_STREAM, 0, fd) >= 0);
    RequestServiceProxy::GetVectorData(config, data);
    EXPECT_EQ(data.ReadUint32(), 0);
    EXPECT_EQ(data.ReadUint32(), 0);
    EXPECT_EQ(data.ReadUint32(), 0);
    EXPECT_EQ(data.ReadUint32(), 0);
    EXPECT_EQ(data.ReadUint32(), 0);
    EXPECT_EQ(data.ReadUint32(), 0);

    config.certsPath.push_back("certsPath");
    FormItem form = { .name = "name", .value = "value" };
    config.forms.push_back(form);
    FileSpec file0 = {
        .name = "file0",
        .uri = "uri0",
        .filename = "filename0",
        .type = "type0",
        .isUserFile = false,
    };
    FileSpec file1 = {
        .name = "file1",
        .uri = "uri1",
        .filename = "filename1",
        .type = "type1",
        .isUserFile = true,
        .fd = fd[0],
    };
    config.files.push_back(file0);
    config.files.push_back(file1);
    config.bodyFds.push_back(-1);
    config.bodyFds.push_back(fd[1]);

    config.bodyFileNames.push_back("bodyFileName");

    config.headers.emplace("first0", "second0");

    config.extras.emplace("first1", "second1");

    RequestServiceProxy::GetVectorData(config, data);

    EXPECT_EQ(data.ReadUint32(), 1);
    EXPECT_EQ(data.ReadString(), "certsPath");

    EXPECT_EQ(data.ReadUint32(), 1);
    EXPECT_EQ(data.ReadString(), "name");
    EXPECT_EQ(data.ReadString(), "value");

    EXPECT_EQ(data.ReadUint32(), 2);
    EXPECT_EQ(data.ReadString(), "file0");
    EXPECT_EQ(data.ReadString(), "uri0");
    EXPECT_EQ(data.ReadString(), "filename0");
    EXPECT_EQ(data.ReadString(), "type0");
    EXPECT_EQ(data.ReadBool(), false);
    EXPECT_EQ(data.ReadString(), "file1");
    EXPECT_EQ(data.ReadString(), "uri1");
    EXPECT_EQ(data.ReadString(), "filename1");
    EXPECT_EQ(data.ReadString(), "type1");
    EXPECT_EQ(data.ReadBool(), true);
    EXPECT_EQ(data.ReadFileDescriptor(), fd[0]);

    EXPECT_EQ(data.ReadUint32(), 2);
    EXPECT_EQ(data.ReadString(), "bodyFileName");

    EXPECT_EQ(data.ReadUint32(), 1);
    EXPECT_EQ(data.ReadString(), "first0");
    EXPECT_EQ(data.ReadString(), "second0");
    EXPECT_EQ(data.ReadUint32(), 1);
    EXPECT_EQ(data.ReadString(), "first1");
    EXPECT_EQ(data.ReadString(), "second1");
}

/**
 * @tc.name: CreateTest002
 * @tc.desc: Test CreateTest002 interface base function - Create
 * @tc.type: FUNC
 * @tc.require: Issue Number
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
        .WillOnce(testing::Return(E_UNLOADING_SA));
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Create(config, seq, tid), E_FILE_PATH);
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Create(config, seq, tid), E_OK);
    RequestManagerImpl::GetInstance()->Create(config, seq, tid);
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy();
    EXPECT_TRUE(proxy != (static_cast<OHOS::sptr<RequestServiceInterface>>(exceptProxy)));
}

/**
 * @tc.name: RetryTest002
 * @tc.desc: Test RetryTest002 interface base function - Retry
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, RetryTest002, TestSize.Level1)
{
    Config config;
    config.action = Action::DOWNLOAD;
    int32_t errCode = E_FILE_PATH;
    std::string tid = "1";
    RequestManagerImpl::GetInstance()->Retry(tid, config, errCode);
    FileSpec file0 = {
        .name = "file0",
        .uri = "uri0",
        .filename = "filename0",
        .type = "type0",
        .isUserFile = false,
    };
    config.files.push_back(file0);
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Retry(tid, config, errCode), errCode);
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy();
    EXPECT_TRUE(proxy == (static_cast<OHOS::sptr<RequestServiceInterface>>(exceptProxy)));
    errCode = E_UNLOADING_SA;
    RequestManagerImpl::GetInstance()->Retry(tid, config, errCode);
    proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy();
    EXPECT_TRUE(proxy != (static_cast<OHOS::sptr<RequestServiceInterface>>(exceptProxy)));
}

/**
 * @tc.name: RetryTest003
 * @tc.desc: Test RetryTest003 interface base function - Retry
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, RetryTest003, TestSize.Level1)
{
    Config config;
    config.action = Action::UPLOAD;
    int32_t errCode = E_OK;
    std::string tid = "1";
    FileSpec file0 = {
        .name = "file0",
        .uri = "uri0",
        .filename = "filename0",
        .type = "type0",
        .isUserFile = false,
    };
    config.files.push_back(file0);
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Retry(tid, config, errCode), errCode);
    errCode = E_FILE_PATH;
    EXPECT_EQ(RequestManagerImpl::GetInstance()->Retry(tid, config, errCode), errCode);
    auto proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy();
    EXPECT_TRUE(proxy == (static_cast<OHOS::sptr<RequestServiceInterface>>(exceptProxy)));
    errCode = E_UNLOADING_SA;
    RequestManagerImpl::GetInstance()->Retry(tid, config, errCode);
    proxy = RequestManagerImpl::GetInstance()->GetRequestServiceProxy();
    EXPECT_TRUE(proxy != (static_cast<OHOS::sptr<RequestServiceInterface>>(exceptProxy)));
}

/**
 * @tc.name: SubscribeTest002
 * @tc.desc: Test SubscribeTest002 interface base function - Subscribe
 * @tc.type: FUNC
 * @tc.require: Issue Number
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
 * @tc.desc: Test EnsureChannelOpenTest002 interface base function - EnsureChannelOpen
 * @tc.type: FUNC
 * @tc.require: Issue Number
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
 * @tc.desc: Test GetTaskTest002 interface base function - GetTask
 * @tc.type: FUNC
 * @tc.require: Issue Number
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
 * @tc.desc: Test SubscribeSATest001 interface base function - SubscribeSA
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, SubscribeSATest001, TestSize.Level1)
{
    RequestManagerImpl::GetInstance()->saChangeListener_ = nullptr;
    RequestManagerImpl::GetInstance()->SubscribeSA();
    EXPECT_NE(RequestManagerImpl::GetInstance()->saChangeListener_, nullptr);
    RequestManagerImpl::GetInstance()->SubscribeSA();
}

/**
 * @tc.name: RestoreSubRunCountTest002
 * @tc.desc: Test RestoreSubRunCountTest002 interface base function - RestoreSubRunCount
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, RestoreSubRunCountTest002, TestSize.Level1)
{
    EXPECT_NE(exceptProxy, nullptr);
    OHOS::sptr<NotifyInterface> listener(nullptr);
    EXPECT_CALL(*exceptProxy, SubRunCount(testing::_)).WillOnce(testing::Return(E_OK));
    RequestManagerImpl::GetInstance()->SetRequestServiceProxy(exceptProxy);
    RequestManagerImpl::GetInstance()->RestoreSubRunCount();
}

/**
 * @tc.name: OnAddSystemAbility002
 * @tc.desc: Test OnAddSystemAbility002 interface base function - OnAddSystemAbility
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, OnAddSystemAbility002, TestSize.Level1)
{
    EXPECT_NE(exceptProxy, nullptr);
    string deviceId = "deviceId";
    RequestManagerImpl::GetInstance()->RestoreListener(nullptr);
    EXPECT_FALSE(FwkRunningTaskCountManager::GetInstance()->HasObserver());
    auto pNewFwkOb = std::make_shared<FwkIRunningTaskObserver>(nullptr);
    FwkRunningTaskCountManager::GetInstance()->observers_.push_back(pNewFwkOb);
    EXPECT_TRUE(FwkRunningTaskCountManager::GetInstance()->HasObserver());
    EXPECT_CALL(*exceptProxy, SubRunCount(testing::_)).WillOnce(testing::Return(E_OK));
    RequestManagerImpl::SystemAbilityStatusChangeListener listener =
        RequestManagerImpl::SystemAbilityStatusChangeListener();
    listener.OnAddSystemAbility(OHOS::PRINT_SERVICE_ID, deviceId);
    FwkRunningTaskCountManager::GetInstance()->observers_.clear();
}

/**
 * @tc.name: LoadRequestServer001
 * @tc.desc: Test LoadRequestServer001 interface base function - LoadRequestServer
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, LoadRequestServer001, TestSize.Level1)
{
    RequestManagerImpl::GetInstance()->ready_ = true;
    EXPECT_EQ(RequestManagerImpl::GetInstance()->LoadRequestServer(), true);
    RequestManagerImpl::GetInstance()->ready_ = false;
    RequestManagerImpl::GetInstance()->LoadRequestServer();
}

/**
 * @tc.name: ReopenChannel001
 * @tc.desc: Test ReopenChannel001 interface base function - ReopenChannel
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerImplTest, ReopenChannel001, TestSize.Level1)
{
    EXPECT_NE(exceptProxy, nullptr);
    RequestManagerImpl::GetInstance()->msgReceiver_ =
        std::make_shared<ResponseMessageReceiver>(RequestManagerImpl::GetInstance().get(), -1);
    EXPECT_CALL(*exceptProxy, OpenChannel(testing::_)).WillOnce(testing::Return(E_CHANNEL_NOT_OPEN));
    RequestManagerImpl::GetInstance()->ReopenChannel();
}