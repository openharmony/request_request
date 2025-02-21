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
    std::string tid = "tid";
    std::vector<std::string> tids = { tid };
    std::unordered_map<std::string, ExceptionErrorCode> rets;
    ExceptionErrorCode res = RequestAction::GetInstance()->DisableTaskNotification(tids, rets);
    EXPECT_EQ(res, ExceptionErrorCode::E_OK);
    EXPECT_EQ(rets[tid], ExceptionErrorCode::E_TASK_NOT_FOUND);
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

class ContextTest : public ApplicationContext {
public:
    ~ContextTest(){};
    std::string GetBaseDir(void) const override
    {
        return "/data/app/base";
    }
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
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTest>();
    std::string path;
    auto res = RequestAction::BaseToWhole(context, path);
    EXPECT_EQ(res, true);
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
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTest>();
    Config config;
    std::string path;
    auto res = RequestAction::CacheToWhole(context, path);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest0041
 * @tc.desc: Test CreateInnerTest004 interface base function - StandardizePath
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest005, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTest>();
    Config config;
    std::string path;
    auto res = RequestAction::StandardizePath(context, config, path);
    EXPECT_EQ(res, false);
}

/**
 * @tc.name: CreateInnerTest0041
 * @tc.desc: Test CreateInnerTest004 interface base function - StandardizePath
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest0051, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTest>();
    Config config;
    std::string path = "/";
    auto res = RequestAction::StandardizePath(context, config, path);
    EXPECT_EQ(res, true);
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
 * @tc.name: CreateInnerTest011
 * @tc.desc: Test CreateInnerTest011 interface base function - GetSandboxPath
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest011, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTest>();
    Config config;
    std::string path;
    std::vector<std::string> pathVec;
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
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTest>();
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
    std::string filepath;
    std::string baseDir;
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
 * @tc.name: CreateInnerTest018
 * @tc.desc: Test CreateInnerTest018 interface base function - GetInternalPath
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest018, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTest>();
    Config config;
    std::string path;
    auto res = RequestAction::GetInternalPath(context, config, path);
    EXPECT_EQ(res, false);
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
 * @tc.name: CreateInnerTest021
 * @tc.desc: Test CreateInnerTest021 interface base function - CheckDownloadFile
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest021, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTest>();
    Config config;
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
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTest>();
    Config config;
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
    std::string path;
    auto res = RequestAction::CheckPathIsFile(path);
    EXPECT_EQ(res, false);
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
    Config config = { .version = Version::API10 };
    auto res = RequestAction::GetFdUpload(path, config);
    EXPECT_EQ(res, ExceptionErrorCode::E_FILE_IO);
}

/**
 * @tc.name: CreateInnerTest026
 * @tc.desc: Test CreateInnerTest026 interface base function - CheckUploadFileSpec
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest026, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTest>();
    Config config;
    FileSpec file;
    auto res = RequestAction::CheckUploadFileSpec(context, config, file);
    EXPECT_EQ(res, ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: CreateInnerTest027
 * @tc.desc: Test CreateInnerTest027 interface base function - CheckUploadFiles
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateInnerTest027, TestSize.Level1)
{
    std::shared_ptr<ApplicationContext> context = std::make_shared<ContextTest>();
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
    std::string filepath = "data/storage/testRemove";
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
    std::vector<std::string> dirs;
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