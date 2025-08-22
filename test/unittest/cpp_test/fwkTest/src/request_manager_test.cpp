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
#include "log.h"
#include "request_common.h"
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
 * @tc.name: CreateTest001
 * @tc.desc: Test the basic functionality of the Create interface to create a new task
 * @tc.precon: RequestManager instance is available and initialized
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Prepare valid Config parameters
 *           3. Set sequence number and task ID
 *           4. Call Create method with parameters
 * @tc.expect: Create method executes successfully without throwing exceptions
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, CreateTest001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    Config config;
    int32_t seq = 1;
    std::string tid = "1";
    RequestManager::GetInstance()->Create(config, seq, tid);
}

/**
 * @tc.name: GetTaskTest001
 * @tc.desc: Test the GetTask interface to retrieve task information by task ID and token
 * @tc.precon: RequestManager instance is available and a task has been created
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Create a task with valid parameters
 *           3. Prepare task ID and token strings
 *           4. Call GetTask method with task ID and token
 * @tc.expect: GetTask method returns successfully and populates the Config object
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, GetTaskTest001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
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
 * @tc.desc: Test the Start interface to begin task execution
 * @tc.precon: RequestManager instance is available and a task exists
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Prepare a valid task ID string
 *           3. Call Start method with the task ID
 * @tc.expect: Start method executes successfully without throwing exceptions
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, StartTest001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    std::string tidStr = "tid";
    RequestManager::GetInstance()->Start(tidStr);
}

/**
 * @tc.name: StopTest001
 * @tc.desc: Test the Stop interface to halt task execution
 * @tc.precon: RequestManager instance is available and a task is running
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Prepare a valid task ID string
 *           3. Call Stop method with the task ID
 * @tc.expect: Stop method executes successfully without throwing exceptions
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, StopTest001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    std::string tid = "tid";
    RequestManager::GetInstance()->Stop(tid);
}

/**
 * @tc.name: QueryTest001
 * @tc.desc: Test the Query interface to retrieve task information
 * @tc.precon: RequestManager instance is available and a task exists
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Prepare a valid task ID string
 *           3. Create a TaskInfo object
 *           4. Call Query method with task ID and TaskInfo reference
 * @tc.expect: Query method populates the TaskInfo object successfully
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, QueryTest001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    std::string tid = "tid";
    TaskInfo info;
    RequestManager::GetInstance()->Query(tid, info);
}

/**
 * @tc.name: TouchTest001
 * @tc.desc: Test the Touch interface to update task access timestamp
 * @tc.precon: RequestManager instance is available and a task exists
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Prepare valid task ID and token strings
 *           3. Create a TaskInfo object
 *           4. Call Touch method with task ID, token, and TaskInfo reference
 * @tc.expect: Touch method updates task timestamp and populates TaskInfo successfully
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, Touch001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    std::string tid = "tid";
    std::string token = "token";
    TaskInfo info;
    RequestManager::GetInstance()->Touch(tid, token, info);
}

/**
 * @tc.name: SearchTest001
 * @tc.desc: Test the Search interface to find tasks matching filter criteria
 * @tc.precon: RequestManager instance is available and tasks may exist
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Create a Filter object with search criteria
 *           3. Create a vector to store matching task IDs
 *           4. Call Search method with filter and task ID vector
 * @tc.expect: Search method populates the task ID vector with matching results
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, SearchTest001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    Filter filter;
    std::vector<std::string> tids;
    RequestManager::GetInstance()->Search(filter, tids);
}

/**
 * @tc.name: ShowTest001
 * @tc.desc: Test the Show interface to display task details
 * @tc.precon: RequestManager instance is available and a task exists
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Prepare a valid task ID string
 *           3. Create a TaskInfo object
 *           4. Call Show method with task ID and TaskInfo reference
 * @tc.expect: Show method populates TaskInfo with detailed task information
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, ShowTest001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    std::string tid = "tid";
    TaskInfo info;
    RequestManager::GetInstance()->Show(tid, info);
}

/**
 * @tc.name: PauseTest001
 * @tc.desc: Test the Pause interface to temporarily halt task execution for both API versions
 * @tc.precon: RequestManager instance is available and a running task exists
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Prepare a valid task ID string
 *           3. Call Pause method with task ID and Version::API9
 *           4. Call Pause method with task ID and Version::API10
 * @tc.expect: Both Pause calls execute successfully without throwing exceptions
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, PauseTest001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    std::string tid = "tid";
    RequestManager::GetInstance()->Pause(tid, Version::API9);
    RequestManager::GetInstance()->Pause(tid, Version::API10);
}

/**
 * @tc.name: QueryMimeTypeTest001
 * @tc.desc: Test the QueryMimeType interface to retrieve MIME type of task content
 * @tc.precon: RequestManager instance is available and a task with content exists
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Prepare a valid task ID string
 *           3. Prepare a string to store MIME type
 *           4. Call QueryMimeType method with task ID and MIME type reference
 * @tc.expect: QueryMimeType method populates MIME type string successfully
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, QueryMimeTypeTest001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    std::string tid = "tid";
    std::string mimeType = "mimeType";
    RequestManager::GetInstance()->QueryMimeType(tid, mimeType);
}

/**
 * @tc.name: RemoveTest001
 * @tc.desc: Test the Remove interface to delete tasks for both API versions
 * @tc.precon: RequestManager instance is available and a task exists
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Prepare a valid task ID string
 *           3. Call Remove method with task ID and Version::API9
 *           4. Call Remove method with task ID and Version::API10
 * @tc.expect: Both Remove calls execute successfully without throwing exceptions
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, RemoveTest001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    std::string tid = "tid";
    RequestManager::GetInstance()->Remove(tid, Version::API9);
    RequestManager::GetInstance()->Remove(tid, Version::API10);
}

/**
 * @tc.name: ResumeTest001
 * @tc.desc: Test the Resume interface to continue paused task execution
 * @tc.precon: RequestManager instance is available and a paused task exists
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Prepare a valid task ID string
 *           3. Call Resume method with the task ID
 * @tc.expect: Resume method executes successfully without throwing exceptions
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, ResumeTest001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    std::string tid = "tid";
    RequestManager::GetInstance()->Resume(tid);
}

/**
 * @tc.name: SubscribeTest001
 * @tc.desc: Test the Subscribe interface to register for task notifications
 * @tc.precon: RequestManager instance is available and a task exists
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Prepare a valid task ID string
 *           3. Call Subscribe method with the task ID
 * @tc.expect: Subscribe method executes successfully without throwing exceptions
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, SubscribeTest001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    std::string taskId = "taskId";
    RequestManager::GetInstance()->Subscribe(taskId);
}

/**
 * @tc.name: UnsubscribeTest001
 * @tc.desc: Test the Unsubscribe interface to unregister from task notifications
 * @tc.precon: RequestManager instance is available and a task is subscribed
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Prepare a valid task ID string
 *           3. Call Unsubscribe method with the task ID
 * @tc.expect: Unsubscribe method executes successfully without throwing exceptions
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, Unsubscribe001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
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
    void OnFaultsReceive(const std::shared_ptr<int32_t> &tid, const std::shared_ptr<SubscribeType> &type,
        const std::shared_ptr<Reason> &reason) override
    {
    }
    void OnWaitReceive(std::int32_t taskId, WaitingReason reason) override
    {
    }
};

/**
 * @tc.name: AddAndRemoveListenerTest001
 * @tc.desc: Test the AddListener and RemoveListener interfaces for both response and notification listeners
 * @tc.precon: RequestManager instance is available and listener implementations exist
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Create response listener implementation
 *           3. Add response listener for RESPONSE type
 *           4. Remove response listener for RESPONSE type
 *           5. Create notification listener implementation
 *           6. Add notification listener for COMPLETED type
 *           7. Remove notification listener for COMPLETED type
 * @tc.expect: All AddListener and RemoveListener calls execute successfully
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, AddAndRemoveListenerTest001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
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

void TestRestoreCallback()
{
}

/**
 * @tc.name: RemoveAllListenersTest001
 * @tc.desc: Test the RemoveAllListeners and RestoreListener interfaces
 * @tc.precon: RequestManager instance is available and listeners have been added
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Create response listener implementation
 *           3. Add response listener for RESPONSE type
 *           4. Create notification listener implementation
 *           5. Add notification listener for COMPLETED type
 *           6. Call RemoveAllListeners to remove all listeners
 *           7. Call RestoreListener with callback function
 *           8. Verify callback is set correctly
 *           9. Call RestoreListener with nullptr
 * @tc.expect: All operations execute successfully and callback is properly set
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
    RequestManager::GetInstance()->RestoreListener(TestRestoreCallback);
    EXPECT_EQ(RequestManagerImpl::GetInstance()->callback_, TestRestoreCallback);
    RequestManager::GetInstance()->RestoreListener(nullptr);
}

/**
 * @tc.name: LoadRequestServerTest001
 * @tc.desc: Test the LoadRequestServer interface to initialize the request server
 * @tc.precon: RequestManager instance is available and server is not loaded
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Call LoadRequestServer method
 * @tc.expect: LoadRequestServer method executes successfully without throwing exceptions
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, LoadRequestServerTest001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    RequestManager::GetInstance()->LoadRequestServer();
}

/**
 * @tc.name: IsSaReadyTest001
 * @tc.desc: Test the IsSaReady interface to check system ability readiness
 * @tc.precon: RequestManager instance is available and initialized
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Call IsSaReady method
 * @tc.expect: IsSaReady method returns a valid boolean value indicating readiness
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, IsSaReadyTest001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    RequestManager::GetInstance()->IsSaReady();
}

/**
 * @tc.name: ReopenChannelTest001
 * @tc.desc: Test the ReopenChannel interface to re-establish communication channel
 * @tc.precon: RequestManager instance is available and channel may need reopening
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Call ReopenChannel method
 * @tc.expect: ReopenChannel method executes successfully without throwing exceptions
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, ReopenChannelTest001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    RequestManager::GetInstance()->ReopenChannel();
}

/**
 * @tc.name: SubscribeSATest001
 * @tc.desc: Test the SubscribeSA interface to register for system ability notifications
 * @tc.precon: RequestManager instance is available and SA subscription is needed
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Call SubscribeSA method
 * @tc.expect: SubscribeSA method executes successfully without throwing exceptions
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, SubscribeSATest001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    RequestManager::GetInstance()->SubscribeSA();
}

/**
 * @tc.name: UnsubscribeSATest001
 * @tc.desc: Test the UnsubscribeSA interface to unregister from system ability notifications
 * @tc.precon: RequestManager instance is available and SA subscription exists
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Call UnsubscribeSA method
 * @tc.expect: UnsubscribeSA method executes successfully without throwing exceptions
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, UnsubscribeSATest001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    RequestManager::GetInstance()->UnsubscribeSA();
}

/**
 * @tc.name: GetNextSeqTest001
 * @tc.desc: Test the GetNextSeq interface to generate sequential task identifiers
 * @tc.precon: RequestManager instance is available and initialized
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Call GetNextSeq method to get current sequence
 *           3. Call GetNextSeq method again
 *           4. Verify the second call returns incremented value
 * @tc.expect: Second GetNextSeq call returns value one greater than first call
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, GetNextSeqTest001, TestSize.Level1)
{
    int32_t ret = RequestManager::GetInstance()->GetNextSeq();
    EXPECT_EQ(RequestManager::GetInstance()->GetNextSeq(), ret + 1);
}

/**
 * @tc.name: CreateGroupTest001
 * @tc.desc: Test the CreateGroup interface to create a new task group with notification settings
 * @tc.precon: RequestManager instance is available and group ID is unique
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Prepare a unique group ID string
 *           3. Set gauge parameter to true
 *           4. Create Notification object with title, text, and disable=false
 *           5. Call CreateGroup method with parameters
 * @tc.expect: CreateGroup returns 0 indicating successful group creation
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, CreateGroupTest001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    std::string gid = "gid";
    bool gauge = true;
    Notification info{
        .text = "text",
        .title = "title",
        .disable = false,
    };

    EXPECT_EQ(RequestManager::GetInstance()->CreateGroup(gid, gauge, info), 0);
}

/**
 * @tc.name: CreateGroupTest002
 * @tc.desc: Test the CreateGroup interface to create a new task group with notification settings includes visibility
 * @tc.precon: RequestManager instance is available and group ID is unique
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Prepare a unique group ID string
 *           3. Set gauge parameter to true
 *           4. Create Notification object with title, text, disable=false and visibility=VISIBILITY_COMPLETION
 *           5. Call CreateGroup method with parameters
 * @tc.expect: CreateGroup returns 0 indicating successful group creation
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, CreateGroupTest002, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    std::string gid = "gid";
    bool gauge = true;
    Notification info{
        .text = "text",
        .title = "title",
        .disable = false,
        .visibility = VISIBILITY_COMPLETION,
    };

    EXPECT_EQ(RequestManager::GetInstance()->CreateGroup(gid, gauge, info), 0);
}

/**
 * @tc.name: AttachGroupTest001
 * @tc.desc: Test the AttachGroup interface to associate tasks with an existing group
 * @tc.precon: RequestManager instance is available and group exists
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Prepare a valid group ID string
 *           3. Create empty vector of task IDs
 *           4. Call AttachGroup method with group ID and task IDs
 * @tc.expect: AttachGroup returns 21900008 indicating no tasks to attach
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, AttachGroupTest001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    std::string gid = "gid";
    std::vector<std::string> tids;
    EXPECT_EQ(RequestManager::GetInstance()->AttachGroup(gid, tids), 21900008);
}

/**
 * @tc.name: DeleteGroupTest001
 * @tc.desc: Test the DeleteGroup interface to remove an existing task group
 * @tc.precon: RequestManager instance is available and group exists
 * @tc.step: 1. Get RequestManager singleton instance
 *           2. Prepare a valid group ID string
 *           3. Call DeleteGroup method with the group ID
 * @tc.expect: DeleteGroup returns 21900008 indicating group deletion initiated
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, DeleteGroupTest001, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    std::string gid = "gid";
    EXPECT_EQ(RequestManager::GetInstance()->DeleteGroup(gid), 21900008);
}

/**
 * @tc.name: VisibilityValuesTest001
 * @tc.desc: Test the visibility related values to ensure they match the expected binary values
 * @tc.precon: RequestCommon header file is included
 * @tc.step: 1. Verify Visibility enum values
 *           2. Verify static visibility constants
 *           3. Verify Notification struct default visibility value
 * @tc.expect: All visibility values match their expected binary representations
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(RequestManagerTest, VisibilityValuesTest001, TestSize.Level1)
{
    EXPECT_EQ(static_cast<uint32_t>(Visibility::NONE), 0b00);
    EXPECT_EQ(static_cast<uint32_t>(Visibility::COMPLETION), 0b01);
    EXPECT_EQ(static_cast<uint32_t>(Visibility::PROGRESS), 0b10);
    EXPECT_EQ(static_cast<uint32_t>(Visibility::ANY), 0b11);
    
    EXPECT_EQ(VISIBILITY_COMPLETION, 0b00000001);
    EXPECT_EQ(VISIBILITY_PROGRESS, 0b00000010);
    
    Notification defaultNotification;
    EXPECT_EQ(defaultNotification.visibility, 0b01);
}