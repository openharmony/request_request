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

#include "request_manager.h"

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

class RequestManagerTest : public testing::Test {
public:
    static void SetUpTestCase(void);
    static void TearDownTestCase(void);
    void SetUp();
    void TearDown();
};

void RequestManagerTest::SetUpTestCase(void)
{
    // input testSuit setup step，setup invoked before all testCases
}

void RequestManagerTest::TearDownTestCase(void)
{
    // input testSuit teardown step，teardown invoked after all testCases
}

void RequestManagerTest::SetUp(void)
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

void RequestManagerTest::TearDown(void)
{
    // input testCase teardown step，teardown invoked after each testCase
}

/**
 * @tc.name: GetInstance001
 * @tc.desc: Test GetInstance001 interface base function - GetInstance
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, GetInstance001, TestSize.Level1)
{
    RequestManager::GetInstance();
}

/**
 * @tc.name: CreateTest001
 * @tc.desc: Test CreateTest001 interface base function - Create
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, CreateTest001, TestSize.Level1)
{
    Config config;
    int32_t seq = 1;
    std::string tid = "1";
    RequestManager::GetInstance()->Create(config, seq, tid);
}

/**
 * @tc.name: GetTaskTest001
 * @tc.desc: Test GetTaskTest001 interface base function - GetTask
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, GetTaskTest001, TestSize.Level1)
{
    std::string tidStr = "tid";
    std::string token = "token";
    Config config;
    int32_t seq = 1;
    std::string tid = "1";
    RequestManager::GetInstance()->Create(config, seq, tid);
    RequestManager::GetInstance()->RequestManager::GetInstance()->GetTask(tidStr, token, config);
}

/**
 * @tc.name: StartTest001
 * @tc.desc: Test StartTest001 interface base function - Start
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, StartTest001, TestSize.Level1)
{
    std::string tidStr = "tid";
    RequestManager::GetInstance()->Start(tidStr);
}

/**
 * @tc.name: StopTest001
 * @tc.desc: Test StopTest001 interface base function - Stop
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, StopTest001, TestSize.Level1)
{
    std::string tid = "tid";
    RequestManager::GetInstance()->Stop(tid);
}

/**
 * @tc.name: QueryTest001
 * @tc.desc: Test QueryTest001 interface base function - Query
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, QueryTest001, TestSize.Level1)
{
    std::string tid = "tid";
    TaskInfo info;
    RequestManager::GetInstance()->Query(tid, info);
}

/**
 * @tc.name: TouchTest001
 * @tc.desc: Test TouchTest001 interface base function - Touch
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, Touch001, TestSize.Level1)
{
    std::string tid = "tid";
    std::string token = "token";
    TaskInfo info;
    RequestManager::GetInstance()->Touch(tid, token, info);
}

/**
 * @tc.name: SearchTest001
 * @tc.desc: Test SearchTest001 interface base function - Search
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, SearchTest001, TestSize.Level1)
{
    Filter filter;
    std::vector<std::string> tids;
    RequestManager::GetInstance()->Search(filter, tids);
}

/**
 * @tc.name: ShowTest001
 * @tc.desc: Test ShowTest001 interface base function - Show
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, ShowTest001, TestSize.Level1)
{
    std::string tid = "tid";
    TaskInfo info;
    RequestManager::GetInstance()->Show(tid, info);
}

/**
 * @tc.name: PauseTest001
 * @tc.desc: Test PauseTest001 interface base function - Pause
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, PauseTest001, TestSize.Level1)
{
    std::string tid = "tid";
    RequestManager::GetInstance()->Pause(tid, Version::API9);
    RequestManager::GetInstance()->Pause(tid, Version::API10);
}

/**
 * @tc.name: QueryMimeTypeTest001
 * @tc.desc: Test QueryMimeTypeTest001 interface base function - QueryMimeType
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, QueryMimeTypeTest001, TestSize.Level1)
{
    std::string tid = "tid";
    std::string mimeType = "mimeType";
    RequestManager::GetInstance()->QueryMimeType(tid, mimeType);
}

/**
 * @tc.name: RemoveTest001
 * @tc.desc: Test RemoveTest001 interface base function - Remove
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, RemoveTest001, TestSize.Level1)
{
    std::string tid = "tid";
    RequestManager::GetInstance()->Remove(tid, Version::API9);
    RequestManager::GetInstance()->Remove(tid, Version::API10);
}

/**
 * @tc.name: ResumeTest001
 * @tc.desc: Test ResumeTest001 interface base function - Resume
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, ResumeTest001, TestSize.Level1)
{
    std::string tid = "tid";
    RequestManager::GetInstance()->Resume(tid);
    RequestManager::GetInstance()->Resume(tid);
}

/**
 * @tc.name: SubscribeTest001
 * @tc.desc: Test SubscribeTest001 interface base function - Subscribe
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, SubscribeTest001, TestSize.Level1)
{
    std::string taskId = "taskId";
    RequestManager::GetInstance()->Subscribe(taskId);
}

/**
 * @tc.name: UnsubscribeTest001
 * @tc.desc: Test UnsubscribeTest001 interface base function - Unsubscribe
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, Unsubscribe001, TestSize.Level1)
{
    std::string taskId = "taskId";
    RequestManager::GetInstance()->Unsubscribe(taskId);
}

class RMTResponseListenerImpl : public IResponseListener {
public:
    ~RMTResponseListenerImpl(){};
    void OnResponseReceive(const std::shared_ptr<Response> &response) override
    {
        (void)response;
        return;
    }
};

class RMTNotifyDataListenerImpl : public INotifyDataListener {
public:
    ~RMTNotifyDataListenerImpl(){};
    void OnNotifyDataReceive(const std::shared_ptr<NotifyData> &notifyData) override
    {
        (void)notifyData;
        return;
    }
};

/**
 * @tc.name: AddAndRemoveListenerTest001
 * @tc.desc: Test AddAndRemoveListenerTest001 interface base function - AddListener/RemoveListener
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, AddAndRemoveListenerTest001, TestSize.Level1)
{
    string taskId = "taskId";
    SubscribeType type = SubscribeType::RESPONSE;
    std::shared_ptr<RMTResponseListenerImpl> listener = std::make_shared<RMTResponseListenerImpl>();
    RequestManager::GetInstance()->AddListener(taskId, type, listener);
    RequestManager::GetInstance()->RemoveListener(taskId, type, listener);
    type = SubscribeType::COMPLETED;
    std::shared_ptr<RMTNotifyDataListenerImpl> listener2 = std::make_shared<RMTNotifyDataListenerImpl>();
    RequestManager::GetInstance()->AddListener(taskId, type, listener2);
    RequestManager::GetInstance()->RemoveListener(taskId, type, listener2);
}

/**
 * @tc.name: RemoveAllListenersTest001
 * @tc.desc: Test RemoveAllListenersTest001 interface base function - RemoveAllListeners
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, RemoveAllListenersTest001, TestSize.Level1)
{
    string taskId = "taskId";
    SubscribeType type = SubscribeType::RESPONSE;
    std::shared_ptr<RMTResponseListenerImpl> listener = std::make_shared<RMTResponseListenerImpl>();
    RequestManager::GetInstance()->AddListener(taskId, type, listener);
    type = SubscribeType::COMPLETED;
    std::shared_ptr<RMTNotifyDataListenerImpl> listener2 = std::make_shared<RMTNotifyDataListenerImpl>();
    RequestManager::GetInstance()->AddListener(taskId, type, listener2);
    RequestManager::GetInstance()->RemoveAllListeners(taskId);
    RequestManager::GetInstance()->RestoreListener(nullptr);
}

/**
 * @tc.name: LoadRequestServerTest001
 * @tc.desc: Test LoadRequestServerTest001 interface base function - LoadRequestServer
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, LoadRequestServerTest001, TestSize.Level1)
{
    RequestManager::GetInstance()->LoadRequestServer();
}

/**
 * @tc.name: IsSaReadyTest001
 * @tc.desc: Test IsSaReadyTest001 interface base function - IsSaReady
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, IsSaReadyTest001, TestSize.Level1)
{
    RequestManager::GetInstance()->IsSaReady();
}

/**
 * @tc.name: ReopenChannelTest001
 * @tc.desc: Test ReopenChannelTest001 interface base function - ReopenChannel
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, ReopenChannelTest001, TestSize.Level1)
{
    RequestManager::GetInstance()->ReopenChannel();
}

/**
 * @tc.name: SubscribeSATest001
 * @tc.desc: Test SubscribeSATest001 interface base function - SubscribeSA
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, SubscribeSATest001, TestSize.Level1)
{
    RequestManager::GetInstance()->SubscribeSA();
}

/**
 * @tc.name: UnsubscribeSATest001
 * @tc.desc: Test UnsubscribeSATest001 interface base function - UnsubscribeSA
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, UnsubscribeSATest001, TestSize.Level1)
{
    RequestManager::GetInstance()->UnsubscribeSA();
}

/**
 * @tc.name: GetNextSeqTest001
 * @tc.desc: Test GetNextSeqTest001 interface base function - GetNextSeq
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestManagerTest, GetNextSeqTest001, TestSize.Level1)
{
    RequestManager::GetInstance()->GetNextSeq();
}