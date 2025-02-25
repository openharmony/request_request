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

#include <__chrono/duration.h>
#include <sys/stat.h>

#include <cstdint>
#include <filesystem>
#include <fstream>
#include <memory>
#include <string>
#include <vector>

#include "application_context.h"
#include "constant.h"
#include "context.h"
#include "context_impl.h"
#include "request_manager.h"
#include "request_service_proxy.h"
#include "task_builder.h"
#define private public
#define protected public

#include <gtest/gtest.h>

#include "accesstoken_kit.h"
#include "log.h"
#include "nativetoken_kit.h"
#include "request_action.h"
#include "request_common.h"
#include "request_manager_impl.h"
#include "token_setproc.h"

using namespace testing::ext;
using namespace OHOS::Request;
using namespace OHOS::AbilityRuntime;

#undef private
#undef protected

void GrantInternetPermission()
{
    const char **perms = new const char *[1];
    perms[0] = "ohos.permission.INTERNET";
    NativeTokenInfoParams infoInstance = {
        .dcapsNum = 0,
        .permsNum = 1,
        .aclsNum = 0,
        .dcaps = nullptr,
        .perms = perms,
        .acls = nullptr,
        .processName = "request_service",
        .aplStr = "system_core",
    };
    uint64_t tokenId = GetAccessTokenId(&infoInstance);
    SetSelfTokenID(tokenId);
    OHOS::Security::AccessToken::AccessTokenKit::ReloadNativeTokenInfo();
    delete[] perms;
}

void GrantDownSessionPermission()
{
    const char **perms = new const char *[1];
    perms[0] = "ohos.permission.DOWNLOAD_SESSION_MANAGER";
    NativeTokenInfoParams infoInstance = {
        .dcapsNum = 0,
        .permsNum = 1,
        .aclsNum = 0,
        .dcaps = nullptr,
        .perms = perms,
        .acls = nullptr,
        .processName = "request_service",
        .aplStr = "system_core",
    };
    uint64_t tokenId = GetAccessTokenId(&infoInstance);
    SetSelfTokenID(tokenId);
    OHOS::Security::AccessToken::AccessTokenKit::ReloadNativeTokenInfo();
    delete[] perms;
}

void GrantNoPermission()
{
    const char **perms = new const char *[0];
    NativeTokenInfoParams infoInstance = {
        .dcapsNum = 0,
        .permsNum = 0,
        .aclsNum = 0,
        .dcaps = nullptr,
        .perms = perms,
        .acls = nullptr,
        .processName = "request_service",
        .aplStr = "system_core",
    };
    uint64_t tokenId = GetAccessTokenId(&infoInstance);
    SetSelfTokenID(tokenId);
    OHOS::Security::AccessToken::AccessTokenKit::ReloadNativeTokenInfo();
    delete[] perms;
}

class RequestActionTest : public testing::Test {
public:
    static void SetUpTestCase(void);
    static void TearDownTestCase(void);
    void SetUp();
    void TearDown();
};

void RequestActionTest::SetUpTestCase(void)
{
    // input testSuit setup step，setup invoked before all testCases
}

void RequestActionTest::TearDownTestCase(void)
{
    // input testSuit teardown step，teardown invoked after all testCases
    GrantNoPermission();
}

void RequestActionTest::SetUp(void)
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

void RequestActionTest::TearDown(void)
{
    // input testCase teardown step，teardown invoked after each testCase
}

std::string g_tidUser = "550015967"; //test correct tid which will be replaced after create used

/**
 * @tc.name: StartTest001
 * @tc.desc: Test StartTest001 interface base function - Start
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, StartTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tidStr = "tid";
    GrantInternetPermission();
    auto res = RequestAction::GetInstance()->Start(tidStr);
    REQUEST_HILOGI("===> StartTest001 res %{public}d", res);
}

/**
 * @tc.name: StopTest001
 * @tc.desc: Test StopTest001 interface base function - Stop
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, StopTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = "tid";
    RequestAction::GetInstance()->Stop(tid);
}

/**
 * @tc.name: TouchTest001
 * @tc.desc: Test TouchTest001 interface base function - Touch
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, TouchTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = "tid";
    std::string token = "11111111";
    TaskInfo info;
    RequestAction::GetInstance()->Touch(tid, token, info);
}

/**
 * @tc.name: ShowTest001
 * @tc.desc: Test ShowTest001 interface base function - Show
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, ShowTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = "tid";
    TaskInfo info;
    RequestAction::GetInstance()->Show(tid, info);
}

/**
 * @tc.name: PauseTest001
 * @tc.desc: Test PauseTest001 interface base function - Pause
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, PauseTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = "tid";
    RequestAction::GetInstance()->Pause(tid);
}

/**
 * @tc.name: ResumeTest001
 * @tc.desc: Test ResumeTest001 interface base function - Resume
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, ResumeTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = "tid";
    GrantInternetPermission();
    auto res = RequestAction::GetInstance()->Resume(tid);
    REQUEST_HILOGI("===> ResumeTest001 res %{public}d", res);
}

/**
 * @tc.name: RemoveTest001
 * @tc.desc: Test RemoveTest001 interface base function - Remove
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, RemoveTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = "tid";
    auto res = RequestAction::GetInstance()->Remove(tid);
    REQUEST_HILOGI("===>except 0= %{public}d", res);
}

/**
 * @tc.name: StartTest002
 * @tc.desc: Test StartTest002 interface base function - Start
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, StartTest002, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tidStr = "tid";
    GrantInternetPermission();
    auto res = RequestAction::GetInstance()->Start(tidStr);
    EXPECT_NE(res, 0);
}

/**
 * @tc.name: StopTest002
 * @tc.desc: Test StopTest002 interface base function - Stop
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, StopTest002, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = "tid";
    auto res = RequestAction::GetInstance()->Stop(tid);
    EXPECT_EQ(res, 21900006);
}

/**
 * @tc.name: TouchTest002
 * @tc.desc: Test TouchTest002 interface base function - Touch
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, TouchTest002, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = "tid";
    std::string token = "11111111";
    TaskInfo info;
    auto res = RequestAction::GetInstance()->Touch(tid, token, info);
    EXPECT_EQ(res, 21900006);
}

/**
 * @tc.name: ShowTest002
 * @tc.desc: Test ShowTest002 interface base function - Show
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, ShowTest002, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = "tid";
    TaskInfo info;
    auto res = RequestAction::GetInstance()->Show(tid, info);
    EXPECT_EQ(res, 21900006);
}

/**
 * @tc.name: PauseTest002
 * @tc.desc: Test PauseTest002 interface base function - Pause
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, PauseTest002, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = "tid";
    auto res = RequestAction::GetInstance()->Pause(tid);
    EXPECT_EQ(res, 21900006);
}

/**
 * @tc.name: ResumeTest002
 * @tc.desc: Test ResumeTest002 interface base function - Resume
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, ResumeTest002, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = "tid";
    GrantInternetPermission();
    auto res = RequestAction::GetInstance()->Resume(tid);
    EXPECT_NE(res, 0);
}

/**
 * @tc.name: RemoveTest002
 * @tc.desc: Test RemoveTest002 interface base function - Remove
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, RemoveTest002, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = "tid";
    auto res = RequestAction::GetInstance()->Remove(tid);
    EXPECT_EQ(res, 21900006);
}

/**
 * @tc.name: StartTest003
 * @tc.desc: Test StartTest003 interface base function - Start
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, StartTest003, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = g_tidUser;
    GrantDownSessionPermission();
    auto res = RequestAction::GetInstance()->Start(tid);
    EXPECT_NE(res, 13499999);
    REQUEST_HILOGI("===> StartTest003 res 0=%{public}d", res);
}

/**
 * @tc.name: StopTest003
 * @tc.desc: Test StopTest003 interface base function - Stop
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, StopTest003, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = g_tidUser;
    GrantDownSessionPermission();
    auto res = RequestAction::GetInstance()->Stop(tid);
    EXPECT_EQ(res, 21900006);
    REQUEST_HILOGI("===> StopTest003 res 0=%{public}d", res);
}

/**
 * @tc.name: TouchTest003
 * @tc.desc: Test TouchTest003 interface base function - Touch
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, TouchTest003, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = g_tidUser;
    std::string token = "11111111";
    TaskInfo info;
    GrantDownSessionPermission();
    auto res = RequestAction::GetInstance()->Touch(tid, token, info);
    EXPECT_EQ(res, 21900006);
    REQUEST_HILOGI("===> TouchTest003 res 0=%{public}d", res);
}

/**
 * @tc.name: ShowTest003
 * @tc.desc: Test ShowTest003 interface base function - Show
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, ShowTest003, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = g_tidUser;
    TaskInfo info;
    GrantDownSessionPermission();
    auto res = RequestAction::GetInstance()->Show(tid, info);
    EXPECT_EQ(res, 21900006);
    REQUEST_HILOGI("===> ShowTest003 res 0=%{public}d", res);
}

/**
 * @tc.name: PauseTest003
 * @tc.desc: Test PauseTest003 interface base function - Pause
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, PauseTest003, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = g_tidUser;
    GrantDownSessionPermission();
    auto res = RequestAction::GetInstance()->Pause(tid);
    EXPECT_EQ(res, 21900006);
    REQUEST_HILOGI("===> PauseTest003 res 0=%{public}d", res);
}

/**
 * @tc.name: ResumeTest003
 * @tc.desc: Test ResumeTest003 interface base function - Resume
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, ResumeTest003, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = g_tidUser;
    GrantDownSessionPermission();
    auto res = RequestAction::GetInstance()->Resume(tid);
    EXPECT_NE(res, 13499999);
    REQUEST_HILOGI("===> ResumeTest003 res 0=%{public}d", res);
}

/**
 * @tc.name: RemoveTest003
 * @tc.desc: Test RemoveTest003 interface base function - Remove
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, RemoveTest003, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = g_tidUser;
    GrantDownSessionPermission();
    auto res = RequestAction::GetInstance()->Remove(tid);
    EXPECT_NE(res, 201);
    REQUEST_HILOGI("===>RemoveTest003 res 0= %{public}d", res);
}

/**
 * @tc.name: StartTest004
 * @tc.desc: Test StartTest004 interface base function - Start
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, StartTest004, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = g_tidUser;
    auto res = RequestAction::GetInstance()->Start(tid);
    EXPECT_NE(res, 13499999);
}

/**
 * @tc.name: StopTest004
 * @tc.desc: Test StopTest004 interface base function - Stop
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, StopTest004, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = g_tidUser;
    auto res = RequestAction::GetInstance()->Stop(tid);
    EXPECT_EQ(res, 21900006);
}

/**
 * @tc.name: TouchTest004
 * @tc.desc: Test TouchTest004 interface base function - Touch
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, TouchTest004, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = g_tidUser;
    std::string token = "11111111";
    TaskInfo info;
    auto res = RequestAction::GetInstance()->Touch(tid, token, info);
    EXPECT_EQ(res, 21900006);
}

/**
 * @tc.name: ShowTest004
 * @tc.desc: Test ShowTest004 interface base function - Show
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, ShowTest004, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = g_tidUser;
    TaskInfo info;
    auto res = RequestAction::GetInstance()->Show(tid, info);
    EXPECT_EQ(res, 21900006);
}

/**
 * @tc.name: PauseTest004
 * @tc.desc: Test PauseTest004 interface base function - Pause
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, PauseTest004, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = g_tidUser;
    auto res = RequestAction::GetInstance()->Pause(tid);
    EXPECT_EQ(res, 21900006);
}

/**
 * @tc.name: ResumeTest004
 * @tc.desc: Test ResumeTest004 interface base function - Resume
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, ResumeTest004, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = g_tidUser;
    auto res = RequestAction::GetInstance()->Resume(tid);
    EXPECT_NE(res, 13499999);
}

/**
 * @tc.name: RemoveTest004
 * @tc.desc: Test RemoveTest004 interface base function - Remove
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, RemoveTest004, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::string tid = g_tidUser;
    auto res = RequestAction::GetInstance()->Remove(tid);
    EXPECT_EQ(res, 21900006);
}

/**
 * @tc.name: StartTasksTest001
 * @tc.desc: Test StartTasksTest001 interface base function - StartTasks
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, StartTasksTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    GrantDownSessionPermission();
    std::string tid = "tid";
    std::vector<std::string> tids = { tid };
    std::unordered_map<std::string, ExceptionErrorCode> rets;
    ExceptionErrorCode res = RequestAction::GetInstance()->StartTasks(tids, rets);
    EXPECT_NE(res, ExceptionErrorCode::E_OTHER);
    ExceptionErrorCode res0 = rets[tid];
    EXPECT_NE(res, ExceptionErrorCode::E_OTHER);
    REQUEST_HILOGI("===> StartTasksTest001 res 0=%{public}d", res0);
}

/**
 * @tc.name: StopTasksTest001
 * @tc.desc: Test StopTasksTest001 interface base function - StopTasks
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, StopTasksTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    GrantDownSessionPermission();
    std::string tid = "tid";
    std::vector<std::string> tids = { tid };
    std::unordered_map<std::string, ExceptionErrorCode> rets;
    ExceptionErrorCode res = RequestAction::GetInstance()->StopTasks(tids, rets);
    EXPECT_EQ(res, ExceptionErrorCode::E_OK);
    ExceptionErrorCode res0 = rets[tid];
    EXPECT_EQ(res0, ExceptionErrorCode::E_TASK_NOT_FOUND);
    REQUEST_HILOGI("===> StopTasksTest001 res 0=%{public}d", res0);
}

/**
 * @tc.name: ResumeTasksTest001
 * @tc.desc: Test ResumeTasksTest001 interface base function - ResumeTasks
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, ResumeTasksTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    GrantDownSessionPermission();
    std::string tid = "tid";
    std::vector<std::string> tids = { tid };
    std::unordered_map<std::string, ExceptionErrorCode> rets;
    ExceptionErrorCode res = RequestAction::GetInstance()->ResumeTasks(tids, rets);
    EXPECT_NE(res, ExceptionErrorCode::E_OTHER);
    ExceptionErrorCode res0 = rets[tid];
    EXPECT_NE(res, ExceptionErrorCode::E_OTHER);
    REQUEST_HILOGI("===> ResumeTasksTest001 res 0=%{public}d", res0);
}

/**
 * @tc.name: PauseTasksTest001
 * @tc.desc: Test PauseTasksTest001 interface base function - PauseTasks
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, PauseTasksTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    GrantDownSessionPermission();
    std::string tid = "tid";
    std::vector<std::string> tids = { tid };
    std::unordered_map<std::string, ExceptionErrorCode> rets;
    ExceptionErrorCode res = RequestAction::GetInstance()->PauseTasks(tids, rets);
    EXPECT_EQ(res, ExceptionErrorCode::E_OK);
    ExceptionErrorCode res0 = rets[tid];
    EXPECT_EQ(res0, ExceptionErrorCode::E_TASK_NOT_FOUND);
    REQUEST_HILOGI("===> PauseTasksTest001 res 0=%{public}d", res0);
}

/**
 * @tc.name: ShowTasksTest001
 * @tc.desc: Test ShowTasksTest001 interface base function - ShowTasks
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, ShowTasksTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    GrantDownSessionPermission();
    std::string tid = "tid";
    std::vector<std::string> tids = { tid };
    std::unordered_map<std::string, TaskInfoRet> rets;
    ExceptionErrorCode res = RequestAction::GetInstance()->ShowTasks(tids, rets);
    EXPECT_EQ(res, ExceptionErrorCode::E_OK);
    TaskInfoRet res0 = rets[tid];
    EXPECT_EQ(res0.code, ExceptionErrorCode::E_TASK_NOT_FOUND);
}

/**
 * @tc.name: TouchTasksTest001
 * @tc.desc: Test TouchTasksTest001 interface base function - TouchTasks
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, TouchTasksTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    GrantDownSessionPermission();
    std::string tid = "tid";
    std::string token = "tasktoken";
    TaskIdAndToken tidToken = { tid, token };
    std::vector<TaskIdAndToken> tids = { tidToken };
    std::unordered_map<std::string, TaskInfoRet> rets;
    ExceptionErrorCode res = RequestAction::GetInstance()->TouchTasks(tids, rets);
    EXPECT_EQ(res, ExceptionErrorCode::E_OK);
    TaskInfoRet res0 = rets[tid];
    EXPECT_EQ(res0.code, ExceptionErrorCode::E_TASK_NOT_FOUND);
}

/**
 * @tc.name: SetMaxSpeedTest001
 * @tc.desc: Test SetMaxSpeedTest001 interface base function - SetMaxSpeed
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, SetMaxSpeedTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    GrantNoPermission();
    std::string tid = "tid";
    int64_t maxSpeed = 1000;
    auto res = RequestAction::GetInstance()->SetMaxSpeed(tid, maxSpeed);
    EXPECT_EQ(res, ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: SetMaxSpeedsTest001
 * @tc.desc: Test SetMaxSpeedsTest001 interface base function - SetMaxSpeeds
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, SetMaxSpeedsTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    GrantNoPermission();
    std::string tid = "tid";
    SpeedConfig config = { tid, 1000 };
    std::vector<SpeedConfig> configs = { config };
    std::unordered_map<std::string, ExceptionErrorCode> rets;
    ExceptionErrorCode res = RequestAction::GetInstance()->SetMaxSpeeds(configs, rets);
    EXPECT_EQ(res, ExceptionErrorCode::E_OK);
    ExceptionErrorCode res0 = rets[tid];
    EXPECT_EQ(res0, ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: SetModeTest001
 * @tc.desc: Test SetModeTest001 interface base function - SetMode
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, SetModeTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    GrantNoPermission();
    std::string tid = "tid";
    Mode mode = Mode::BACKGROUND;
    ExceptionErrorCode res = RequestAction::GetInstance()->SetMode(tid, mode);
    EXPECT_EQ(res, ExceptionErrorCode::E_PERMISSION);
}

/**
 * @tc.name: DisableTaskNotificationTest001
 * @tc.desc: Test DisableTaskNotificationTest001 interface base function - DisableTaskNotification
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, DisableTaskNotificationTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    std::vector<std::string> tids = { "tid", "123", "123123" };
    std::unordered_map<std::string, ExceptionErrorCode> rets;
    ExceptionErrorCode res = RequestAction::GetInstance()->DisableTaskNotification(tids, rets);
    EXPECT_EQ(res, ExceptionErrorCode::E_OK);
    EXPECT_EQ(rets["tid"], ExceptionErrorCode::E_TASK_NOT_FOUND);
}

/**
 * @tc.name: CreateTest001
 * @tc.desc: Test CreateTest001 interface base function - Create
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateTest001, TestSize.Level1)
{
    std::string tid;
    TaskBuilder builder;
    std::string url = "https://gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt";
    std::map<std::string, std::string> headers;
    std::map<std::string, std::string> extras;
    auto buildRes = builder.setAction(Action::DOWNLOAD)
                        .setUrl(url)
                        .setTitle("title")
                        .setDescription("description")
                        .setMode(Mode::FOREGROUND)
                        .setOverwrite(true)
                        .setMethod("GET")
                        .setHeaders(headers)
                        .setData("data")
                        .setSaveAs("./test.txt")
                        .setNetwork(Network::ANY)
                        .setMetered(true)
                        .setRoaming(true)
                        .setRetry(true)
                        .setRedirect(true)
                        .setProxy("")
                        .setIndex(0)
                        .setBegins(0)
                        .setEnds(-1)
                        .setGauge(true)
                        .setPrecise(false)
                        .setToken("")
                        .setPriority(0)
                        .setExtras(extras)
                        .build();
    auto res = RequestAction::GetInstance()->Create(builder, tid);
    REQUEST_HILOGI("===> CreateTest001 res 0=%{public}d", res);
    EXPECT_EQ(res, ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: CreateTasksTest001
 * @tc.desc: Test CreateTasksTest001 interface base function - CreateTasks
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateTasksTest001, TestSize.Level1)
{
    std::vector<TaskBuilder> builders;
    std::vector<TaskRet> rets;
    auto res = RequestAction::GetInstance()->CreateTasks(builders, rets);
    EXPECT_EQ(res, ExceptionErrorCode::E_OK);
}

/**
 * @tc.name: RemoveTasksTest001
 * @tc.desc: Test RemoveTasksTest001 interface base function - RemoveTasks
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, RemoveTasksTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    GrantDownSessionPermission();
    std::string tid = "tid";
    std::vector<std::string> tids = { tid };
    std::unordered_map<std::string, ExceptionErrorCode> rets;
    ExceptionErrorCode res = RequestAction::GetInstance()->RemoveTasks(tids, rets);
    EXPECT_EQ(res, ExceptionErrorCode::E_OK);
    ExceptionErrorCode res0 = rets[tid];
    EXPECT_EQ(res0, ExceptionErrorCode::E_TASK_NOT_FOUND);
    REQUEST_HILOGI("===> RemoveTasksTest001 res 0=%{public}d", res0);
}

/**
 * @tc.name: CreateInnerTest001
 * @tc.desc: Test CreateInnerTest001 interface base function - CreateDirs
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest001, TestSize.Level1)
{
    std::vector<std::string> pathDirs;
    auto res = RequestAction::CreateDirs(pathDirs);
    EXPECT_EQ(res, true);
}

/**
 * @tc.name: CreateInnerTest0011
 * @tc.desc: Test CreateInnerTest0011 interface base function - CreateDirs
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0011, TestSize.Level1)
{
    std::vector<std::string> pathDirs = { "sys", "tmp" };
    auto res = RequestAction::CreateDirs(pathDirs);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest0012
 * @tc.desc: Test CreateInnerTest0012 interface base function - CreateDirs
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0012, TestSize.Level1)
{
    std::vector<std::string> pathDirs = { "data", "test", "CreateInTestDir" };
    auto res = RequestAction::CreateDirs(pathDirs);
    EXPECT_EQ(res, true);
}

/**
 * @tc.name: CreateInnerTest002
 * @tc.desc: Test CreateInnerTest002 interface base function - FileToWhole
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest002, TestSize.Level1)
{
    // convert "file://example" to "/data/storage/el?/base/exmaple"
    std::shared_ptr<OHOS::AbilityRuntime::Context> context;
    Config config;
    std::string path;
    auto res = RequestAction::FileToWhole(context, config, path);
    EXPECT_EQ(res, true);
}

/**
 * @tc.name: CreateInnerTest0021
 * @tc.desc: Test CreateInnerTest0021 interface base function - FileToWhole
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0021, TestSize.Level1)
{
    // convert "file://example" to "/data/storage/el?/base/exmaple"
    std::shared_ptr<OHOS::AbilityRuntime::Context> context;
    Config config = { .bundleName = "com.example.aaa" };
    std::string path = "aaa/file";
    auto res = RequestAction::FileToWhole(context, config, path);
    EXPECT_EQ(res, false);
}

class ContextTestMock : public ApplicationContext {
public:
    ~ContextTestMock(){};
    std::string GetBaseDir(void) const override
    {
        return "/data/app/base";
    };
    std::string GetCacheDir(void) override
    {
        return "/data/app/cache";
    };
};

class ContextTestErrMock : public ApplicationContext {
public:
    ~ContextTestErrMock(){};
    std::string GetBaseDir(void) const override
    {
        return "";
    };
    std::string GetCacheDir(void) override
    {
        return "";
    };
};

/**
 * @tc.name: CreateInnerTest003
 * @tc.desc: Test CreateInnerTest003 interface base function - BaseToWhole
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest003, TestSize.Level1)
{
    // convert "internal://cache/exmaple" to "/data/....../cache/example"
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestMock>();
    std::string path;
    auto res = RequestAction::BaseToWhole(context, path);
    EXPECT_EQ(res, true);
}

/**
 * @tc.name: CreateInnerTest0031
 * @tc.desc: Test CreateInnerTest0031 interface base function - BaseToWhole
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0031, TestSize.Level1)
{
    // convert "internal://cache/exmaple" to "/data/....../cache/example"
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestErrMock>();
    std::string path;
    auto res = RequestAction::BaseToWhole(context, path);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest004
 * @tc.desc: Test CreateInnerTest004 interface base function - CacheToWhole
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest004, TestSize.Level1)
{
    // convert "./exmaple" to "/data/....../cache/example"
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestMock>();
    std::string path;
    auto res = RequestAction::CacheToWhole(context, path);
    EXPECT_EQ(res, true);
}

/**
 * @tc.name: CreateInnerTest0041
 * @tc.desc: Test CreateInnerTest0041 interface base function - CacheToWhole
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0041, TestSize.Level1)
{
    // convert "./exmaple" to "/data/....../cache/example"
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestErrMock>();
    std::string path;
    auto res = RequestAction::CacheToWhole(context, path);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest005
 * @tc.desc: Test CreateInnerTest005 interface base function - StandardizePath
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest005, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestMock>();
    Config config;
    std::string path;
    auto res = RequestAction::StandardizePath(context, config, path);
    EXPECT_EQ(res, true);
}

/**
 * @tc.name: CreateInnerTest0051
 * @tc.desc: Test CreateInnerTest0051 interface base function - StandardizePath
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0051, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestMock>();
    Config config;
    std::string path = "/";
    auto res = RequestAction::StandardizePath(context, config, path);
    EXPECT_EQ(res, true);
}

/**
 * @tc.name: CreateInnerTest0052
 * @tc.desc: Test CreateInnerTest0052 interface base function - StandardizePath
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0052, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestErrMock>();
    Config config;
    std::string path = "file://aa";
    auto res = RequestAction::StandardizePath(context, config, path);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest0053
 * @tc.desc: Test CreateInnerTest0053 interface base function - StandardizePath
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0053, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestErrMock>();
    Config config;
    std::string path = "internal://aa";
    auto res = RequestAction::StandardizePath(context, config, path);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest0054
 * @tc.desc: Test CreateInnerTest0054 interface base function - StandardizePath
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0054, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestErrMock>();
    Config config;
    std::string path = "./";
    auto res = RequestAction::StandardizePath(context, config, path);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest006
 * @tc.desc: Test CreateInnerTest006 interface base function - StringSplit
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest006, TestSize.Level1)
{
    const std::string str = "test/test1";
    char delim = '/';
    std::vector<std::string> elems;
    RequestAction::StringSplit(str, delim, elems);
    EXPECT_EQ(elems.size(), 2);
}

/**
 * @tc.name: CreateInnerTest007
 * @tc.desc: Test CreateInnerTest007 interface base function - PathVecToNormal
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest007, TestSize.Level1)
{
    std::vector<std::string> in;
    std::vector<std::string> out;
    auto res = RequestAction::PathVecToNormal(in, out);
    EXPECT_EQ(res, true);
}

/**
 * @tc.name: CreateInnerTest0071
 * @tc.desc: Test CreateInnerTest0071 interface base function - PathVecToNormal
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0071, TestSize.Level1)
{
    std::vector<std::string> in = { "..", "aaaa" };
    std::vector<std::string> out;
    out.resize(10);
    auto res = RequestAction::PathVecToNormal(in, out);
    EXPECT_EQ(res, true);
}

/**
 * @tc.name: CreateInnerTest0072
 * @tc.desc: Test CreateInnerTest0072 interface base function - PathVecToNormal
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0072, TestSize.Level1)
{
    std::vector<std::string> in = { ".." };
    std::vector<std::string> out;
    out.resize(0);
    auto res = RequestAction::PathVecToNormal(in, out);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest008
 * @tc.desc: Test CreateInnerTest008 interface base function - WholeToNormal
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest008, TestSize.Level1)
{
    std::string path;
    std::vector<std::string> out;
    auto res = RequestAction::WholeToNormal(path, out);
    EXPECT_EQ(res, true);
}

/**
 * @tc.name: CreateInnerTest0081
 * @tc.desc: Test CreateInnerTest0081 interface base function - WholeToNormal
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0081, TestSize.Level1)
{
    std::string path = "../aa";
    std::vector<std::string> out;
    out.resize(0);
    auto res = RequestAction::WholeToNormal(path, out);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest0082
 * @tc.desc: Test CreateInnerTest0082 interface base function - WholeToNormal
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0082, TestSize.Level1)
{
    std::string path = "/data/../aa";
    std::vector<std::string> out;
    out.resize(10);
    auto res = RequestAction::WholeToNormal(path, out);
    EXPECT_EQ(res, true);
}

/**
 * @tc.name: CreateInnerTest009
 * @tc.desc: Test CreateInnerTest009 interface base function - GetAppBaseDir
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest009, TestSize.Level1)
{
    std::string baseDir;
    auto res = RequestAction::GetAppBaseDir(baseDir);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest010
 * @tc.desc: Test CreateInnerTest010 interface base function - CheckBelongAppBaseDir
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest010, TestSize.Level1)
{
    std::string filepath;
    std::string baseDir;
    auto res = RequestAction::CheckBelongAppBaseDir(filepath, baseDir);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest0101
 * @tc.desc: Test CreateInnerTest0101 interface base function - FindAREAPath
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0101, TestSize.Level1)
{
    std::string filepath;
    auto res = RequestAction::FindAREAPath(filepath);
    EXPECT_EQ(res, false);
    std::string filepath1 = "/data/storage/el1/base/a";
    auto res1 = RequestAction::FindAREAPath(filepath1);
    EXPECT_EQ(res1, true);
    std::string filepath2 = "/data/storage/el2/base/a";
    auto res2 = RequestAction::FindAREAPath(filepath2);
    EXPECT_EQ(res2, true);
    std::string filepath3 = "/data/storage/el5/base/a";
    auto res3 = RequestAction::FindAREAPath(filepath3);
    EXPECT_EQ(res3, true);
}

/**
 * @tc.name: CreateInnerTest011
 * @tc.desc: Test CreateInnerTest011 interface base function - GetSandboxPath
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest011, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestMock>();
    Config config;
    std::string path;
    std::vector<std::string> pathVec;
    auto res = RequestAction::GetSandboxPath(context, config, path, pathVec);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest0111
 * @tc.desc: Test CreateInnerTest0111 interface base function - GetSandboxPath
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0111, TestSize.Level1)
{
    // StandardizePath is false
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestErrMock>();
    Config config;
    std::string path = "/";
    std::vector<std::string> pathVec;
    auto res = RequestAction::GetSandboxPath(context, config, path, pathVec);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest0112
 * @tc.desc: Test CreateInnerTest0112 interface base function - GetSandboxPath
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0112, TestSize.Level1)
{
    // WholeToNormal is false
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestMock>();
    Config config;
    std::string path = "/";
    std::vector<std::string> pathVec;
    auto res = RequestAction::GetSandboxPath(context, config, path, pathVec);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest0113
 * @tc.desc: Test CreateInnerTest0113 interface base function - GetSandboxPath
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0113, TestSize.Level1)
{
    // pathVec empty
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestMock>();
    Config config;
    std::string path = "../aa";
    std::vector<std::string> pathVec;
    pathVec.resize(0);
    auto res = RequestAction::GetSandboxPath(context, config, path, pathVec);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest0114
 * @tc.desc: Test CreateInnerTest0114 interface base function - GetSandboxPath
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0114, TestSize.Level1)
{
    // CheckBelongAppBaseDir is false
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestMock>();
    Config config;
    std::string path = "/";
    std::vector<std::string> pathVec;
    pathVec.resize(10);
    auto res = RequestAction::GetSandboxPath(context, config, path, pathVec);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest012
 * @tc.desc: Test CreateInnerTest012 interface base function - CheckDownloadFilePath
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest012, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestMock>();
    Config config;
    auto res = RequestAction::CheckDownloadFilePath(context, config);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest013
 * @tc.desc: Test CreateInnerTest013 interface base function - InterceptData
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest013, TestSize.Level1)
{
    std::string str;
    std::string in;
    std::string out;
    auto res = RequestAction::InterceptData(str, in, out);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest0131
 * @tc.desc: Test CreateInnerTest0131 interface base function - InterceptData
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0131, TestSize.Level1)
{
    std::string str = "/";
    std::string in = "a/";
    std::string out;
    auto res = RequestAction::InterceptData(str, in, out);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest014
 * @tc.desc: Test CreateInnerTest014 interface base function - StandardizeFileSpec
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest014, TestSize.Level1)
{
    FileSpec file = { .uri = "/test.txt" };
    RequestAction::StandardizeFileSpec(file);
    EXPECT_EQ(file.name, "file");
    EXPECT_EQ(file.filename, "test.txt");
    EXPECT_EQ(file.type, "txt");
    FileSpec file1 = { .uri = "/test.txt", .filename = "test1", .name = "file1", .type = "text/plain" };
    RequestAction::StandardizeFileSpec(file1);
    EXPECT_EQ(file1.name, "file1");
    EXPECT_FALSE(file1.filename.empty());
    EXPECT_FALSE(file1.type.empty());
}

/**
 * @tc.name: CreateInnerTest015
 * @tc.desc: Test CreateInnerTest015 interface base function - AddPathMap
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest015, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    std::string filepath = "a/entry/file/cache";
    std::string baseDir = "base";
    RequestAction::AddPathMap(filepath, baseDir);
    RequestAction::AddPathMap(filepath, baseDir);
}

/**
 * @tc.name: CreateInnerTest016
 * @tc.desc: Test CreateInnerTest016 interface base function - SetPathPermission
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest016, TestSize.Level1)
{
    std::string filepath;
    auto res = RequestAction::SetPathPermission(filepath);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest017
 * @tc.desc: Test CreateInnerTest017 interface base function - IsPathValid
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest017, TestSize.Level1)
{
    std::string filepath;
    auto res = RequestAction::IsPathValid(filepath);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest0171
 * @tc.desc: Test CreateInnerTest0171 interface base function - IsPathValid
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0171, TestSize.Level1)
{
    std::string filepath = "/data/storage/el1/base/test_createinner_0171/";
    auto res = RequestAction::IsPathValid(filepath);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest0172
 * @tc.desc: Test CreateInnerTest0172 interface base function - IsPathValid
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0172, TestSize.Level1)
{
    std::string filepath = "/data/test/";
    auto res = RequestAction::IsPathValid(filepath);
    EXPECT_EQ(res, true);
}

/**
 * @tc.name: CreateInnerTest018
 * @tc.desc: Test CreateInnerTest018 interface base function - GetInternalPath
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest018, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestMock>();
    Config config;
    std::string path;
    auto res = RequestAction::GetInternalPath(context, config, path);
    EXPECT_EQ(res, false);
    std::string path1 = "internal://cache/test1.txt";
    auto res1 = RequestAction::GetInternalPath(context, config, path1);
    EXPECT_EQ(res1, false);
}

/**
 * @tc.name: CreateInnerTest0181
 * @tc.desc: Test CreateInnerTest0181 interface base function - GetInternalPath
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0181, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestErrMock>();
    Config config;
    std::string path = "internal://cache/test1.txt";
    auto res = RequestAction::GetInternalPath(context, config, path);
    EXPECT_EQ(res, false);
}

class ContextCacheTestMock : public ApplicationContext {
public:
    ~ContextCacheTestMock(){};
    std::string GetCacheDir(void) override
    {
        return "/data";
    };
};

/**
 * @tc.name: CreateInnerTest0182
 * @tc.desc: Test CreateInnerTest0182 interface base function - GetInternalPath
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0182, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextCacheTestMock>();
    Config config;
    std::string path = "test";
    auto res = RequestAction::GetInternalPath(context, config, path);
    EXPECT_EQ(res, true);
}

/**
 * @tc.name: CreateInnerTest019
 * @tc.desc: Test CreateInnerTest019 interface base function - FindDir
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest019, TestSize.Level1)
{
    std::string pathDir;
    auto res = RequestAction::FindDir(pathDir);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest020
 * @tc.desc: Test CreateInnerTest020 interface base function - GetFdDownload
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest020, TestSize.Level1)
{
    std::string path;
    Config config;
    auto res = RequestAction::GetFdDownload(path, config);
    EXPECT_EQ(res, ExceptionErrorCode::E_FILE_IO);
}

/**
 * @tc.name: CreateInnerTest0201
 * @tc.desc: Test CreateInnerTest0201 interface base function - GetFdDownload
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0201, TestSize.Level1)
{
    std::string path = "/data/test";
    Config config = { .version = Version::API10, .firstInit = true, .overwrite = false };
    auto res = RequestAction::GetFdDownload(path, config);
    EXPECT_EQ(res, ExceptionErrorCode::E_FILE_IO);
    Config config1 = { .version = Version::API9, .firstInit = true, .overwrite = false };
    auto res1 = RequestAction::GetFdDownload(path, config1);
    EXPECT_EQ(res1, ExceptionErrorCode::E_FILE_PATH);
    Config config2 = { .version = Version::API10, .firstInit = false, .overwrite = true };
    auto res2 = RequestAction::GetFdDownload(path, config2);
    EXPECT_EQ(res2, ExceptionErrorCode::E_FILE_IO);
    Config config3 = { .version = Version::API10, .firstInit = false, .overwrite = false };
    auto res3 = RequestAction::GetFdDownload(path, config3);
    EXPECT_EQ(res2, ExceptionErrorCode::E_FILE_IO);
    Config config4 = { .version = Version::API10, .firstInit = true, .overwrite = true };
    EXPECT_EQ(RequestAction::GetFdDownload(path, config4), ExceptionErrorCode::E_FILE_IO);
}

/**
 * @tc.name: CreateInnerTest0202
 * @tc.desc: Test CreateInnerTest0202 interface base function - GetFdDownload
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0202, TestSize.Level1)
{
    std::string path = "/data/storage/el1/base/test";
    Config config = { .version = Version::API10, .firstInit = true, .overwrite = true };
    auto res = RequestAction::GetFdDownload(path, config);
    EXPECT_EQ(res, ExceptionErrorCode::E_FILE_IO);
}

/**
 * @tc.name: CreateInnerTest021
 * @tc.desc: Test CreateInnerTest021 interface base function - CheckDownloadFile
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest021, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestMock>();
    Config config;
    auto res = RequestAction::CheckDownloadFile(context, config);
    EXPECT_EQ(res, ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: CreateInnerTest0211
 * @tc.desc: Test CreateInnerTest0211 interface base function - CheckDownloadFile
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0211, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestMock>();
    FileSpec file = { .uri = "/test.txt" };
    Config config = { .version = Version::API9, .files = { file } };
    auto res = RequestAction::CheckDownloadFile(context, config);
    EXPECT_EQ(res, ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: CreateInnerTest0212
 * @tc.desc: Test CreateInnerTest0212 interface base function - CheckDownloadFile
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0212, TestSize.Level1)
{
    // CheckDownloadFile api9/ find("/")-false/ GetInternalPath-true
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextCacheTestMock>();
    FileSpec file = { .uri = "test" };
    Config config = { .version = Version::API9, .files = { file } };
    auto res = RequestAction::CheckDownloadFile(context, config);
    EXPECT_EQ(res, ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: CreateInnerTest022
 * @tc.desc: Test CreateInnerTest022 interface base function - IsUserFile
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest022, TestSize.Level1)
{
    std::string path;
    auto res = RequestAction::IsUserFile(path);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest0221
 * @tc.desc: Test CreateInnerTest0221 interface base function - IsUserFile
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0221, TestSize.Level1)
{
    std::string path = "file://docs/";
    auto res = RequestAction::IsUserFile(path);
    EXPECT_EQ(res, true);
}

/**
 * @tc.name: CreateInnerTest0222
 * @tc.desc: Test CreateInnerTest0222 interface base function - IsUserFile
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0222, TestSize.Level1)
{
    std::string path = "file://media/";
    auto res = RequestAction::IsUserFile(path);
    EXPECT_EQ(res, true);
}

/**
 * @tc.name: CreateInnerTest023
 * @tc.desc: Test CreateInnerTest023 interface base function - CheckUserFileSpec
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest023, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestMock>();
    Config config;
    FileSpec file;
    auto res = RequestAction::CheckUserFileSpec(context, config, file);
    EXPECT_EQ(res, ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: CreateInnerTest0231
 * @tc.desc: Test CreateInnerTest0231 interface base function - CheckUserFileSpec
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0231, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context;
    Config config = { .mode = Mode::BACKGROUND };
    FileSpec file;
    auto res = RequestAction::CheckUserFileSpec(context, config, file);
    EXPECT_EQ(res, ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: CreateInnerTest0232
 * @tc.desc: Test CreateInnerTest0232 interface base function - CheckUserFileSpec
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0232, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context;
    Config config = { .mode = Mode::FOREGROUND };
    FileSpec file;
    auto res = RequestAction::CheckUserFileSpec(context, config, file);
    EXPECT_EQ(res, ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: CreateInnerTest024
 * @tc.desc: Test CreateInnerTest024 interface base function - CheckPathIsFile
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest024, TestSize.Level1)
{
    // exist is false
    std::string path;
    auto res = RequestAction::CheckPathIsFile(path);
    EXPECT_EQ(res, false);
    // is_directory is true
    std::string path1 = "/data/test";
    auto res1 = RequestAction::CheckPathIsFile(path1);
    EXPECT_EQ(res1, false);
    // exist and no_directory
    std::ofstream file("/data/test/CreateInnerFile");
    file.close();
    std::string path2 = "/data/test/CreateInnerFile";
    auto res2 = RequestAction::CheckPathIsFile(path2);
    EXPECT_EQ(res2, true);
}

/**
 * @tc.name: CreateInnerTest025
 * @tc.desc: Test CreateInnerTest025 interface base function - GetFdUpload
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest025, TestSize.Level1)
{
    std::string path;
    // open file error in api10
    Config config = { .version = Version::API10 };
    auto res = RequestAction::GetFdUpload(path, config);
    EXPECT_EQ(res, ExceptionErrorCode::E_FILE_IO);
    // open file error in api9
    Config config1 = { .version = Version::API9 };
    auto res1 = RequestAction::GetFdUpload(path, config1);
    EXPECT_EQ(res1, ExceptionErrorCode::E_FILE_PATH);
}

/**
 * @tc.name: CreateInnerTest0251
 * @tc.desc: Test CreateInnerTest0251 interface base function - GetFdUpload
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0251, TestSize.Level1)
{
    std::ofstream file("/data/test/CreateInnerFile");
    file.close();
    std::string path = "/data/test/CreateInnerFile";
    Config config = { .version = Version::API10 };
    auto res = RequestAction::GetFdUpload(path, config);
    EXPECT_EQ(res, ExceptionErrorCode::E_OK);
}

/**
 * @tc.name: CreateInnerTest0252
 * @tc.desc: Test CreateInnerTest0252 interface base function - GetFdUpload
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0252, TestSize.Level1)
{
    std::string path = " system/etc/init.cfg";
    // open file error in api10
    Config config = { .version = Version::API10 };
    auto res = RequestAction::GetFdUpload(path, config);
    EXPECT_EQ(res, ExceptionErrorCode::E_FILE_IO);
    // open file error in api9
    Config config1 = { .version = Version::API9 };
    auto res1 = RequestAction::GetFdUpload(path, config1);
    EXPECT_EQ(res1, ExceptionErrorCode::E_FILE_PATH);
}

/**
 * @tc.name: CreateInnerTest026
 * @tc.desc: Test CreateInnerTest026 interface base function - CheckUploadFileSpec
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest026, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestMock>();
    Config config;
    FileSpec file;
    auto res = RequestAction::CheckUploadFileSpec(context, config, file);
    EXPECT_EQ(res, ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: CreateInnerTest0261
 * @tc.desc: Test CreateInnerTest0261 interface base function - CheckUploadFileSpec
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0261, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestMock>();
    FileSpec file;
    Config config = { .version = Version::API9 };
    auto res = RequestAction::CheckUploadFileSpec(context, config, file);
    EXPECT_EQ(res, ExceptionErrorCode::E_PARAMETER_CHECK);
    Config config1 = { .version = Version::API10 };
    EXPECT_EQ(RequestAction::CheckUploadFileSpec(context, config1, file), ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: CreateInnerTest027
 * @tc.desc: Test CreateInnerTest027 interface base function - CheckUploadFiles
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest027, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTestMock>();
    Config config = { .version = Version::API10 };
    auto res = RequestAction::CheckUploadFiles(context, config);
    EXPECT_EQ(res, ExceptionErrorCode::E_OK);
}

/**
 * @tc.name: CreateInnerTest028
 * @tc.desc: Test CreateInnerTest028 interface base function - CheckUploadBodyFiles
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest028, TestSize.Level1)
{
    std::string filepath;
    Config config = { .version = Version::API10 };
    auto res = RequestAction::CheckUploadBodyFiles(filepath, config);
    EXPECT_EQ(res, ExceptionErrorCode::E_OK);
    // len !=0 but filePath empty
    FileSpec fileSpec = {
        .filename = "filename", .name = "file", .uri = "/data/test/CheckUploadBodyFilesTest", .type = "text/plain"
    };
    Config config1 = { .multipart = true, .files = { fileSpec } };
    EXPECT_EQ(RequestAction::CheckUploadBodyFiles(filepath, config1), ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: CreateInnerTest0281
 * @tc.desc: Test CreateInnerTest0281 interface base function - CheckUploadBodyFiles
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0281, TestSize.Level1)
{
    // len !=0 and filepath no empty
    std::string filepath = "/data/test";
    FileSpec fileSpec = {
        .filename = "filename", .name = "file", .uri = "/data/test/CheckUploadBodyFilesTest", .type = "text/plain"
    };
    Config config1 = { .multipart = true, .files = { fileSpec } };
    EXPECT_EQ(RequestAction::CheckUploadBodyFiles(filepath, config1), ExceptionErrorCode::E_FILE_IO);
}

/**
 * @tc.name: CreateInnerTest029
 * @tc.desc: Test CreateInnerTest029 interface base function - SetDirsPermission
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest029, TestSize.Level1)
{
    std::vector<std::string> dirs = { "test" };
    auto res = RequestAction::SetDirsPermission(dirs);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest0291
 * @tc.desc: Test CreateInnerTest0291 interface base function - SetDirsPermission
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0291, TestSize.Level1)
{
    std::vector<std::string> dirs;
    auto res = RequestAction::SetDirsPermission(dirs);
    EXPECT_EQ(res, true);
}

/**
 * @tc.name: CreateInnerTest030
 * @tc.desc: Test CreateInnerTest030 interface base function - CheckFilePath
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest030, TestSize.Level1)
{
    Config config;
    auto res = RequestAction::CheckFilePath(config);
    EXPECT_EQ(res, ExceptionErrorCode::E_FILE_IO);
}

/**
 * @tc.name: CreateInnerTest031
 * @tc.desc: Test CreateInnerTest031 interface base function - RemoveFile
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest031, TestSize.Level1)
{
    std::string filepath = "data/test/testRemove";
    RequestAction::RemoveFile(filepath);
    auto res = std::filesystem::exists(filepath);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest032
 * @tc.desc: Test CreateInnerTest032 interface base function - RemovePathMap
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest032, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    std::string filepath;
    RequestAction::RemovePathMap(filepath);
}

/**
 * @tc.name: CreateInnerTest033
 * @tc.desc: Test CreateInnerTest033 interface base function - RemoveDirsPermission
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest033, TestSize.Level1)
{
    EXPECT_NE(RequestManager::GetInstance(), nullptr);
    std::vector<std::string> dirs = { "/data/test" };
    RequestAction::RemoveDirsPermission(dirs);
}

/**
 * @tc.name: CreateInnerTest034
 * @tc.desc: Test CreateInnerTest034 interface base function - ClearTaskTemp
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest034, TestSize.Level1)
{
    std::string tid;
    auto res = RequestAction::ClearTaskTemp(tid);
    EXPECT_EQ(res, false);
}