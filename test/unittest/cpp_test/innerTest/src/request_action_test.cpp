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
#include <string>

#include "request_manager.h"
#include "request_service_proxy.h"
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

#undef private
#undef protected

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

void GrantDownInterPermission()
{
    const char **perms = new const char *[2];
    perms[0] = "ohos.permission.INTERNET";
    perms[1] = "ohos.permission.DOWNLOAD_SESSION_MANAGER";
    NativeTokenInfoParams infoInstance = {
        .dcapsNum = 0,
        .permsNum = 2,
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

std::string g_tidUser = "550015967"; //test correct tid which will be replaced after create used

/**
 * @tc.name: CreateTest001
 * @tc.desc: Test CreateTest001 interface base function - Start
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(RequestActionTest, CreateTest001, TestSize.Level1)
{
    EXPECT_NE(RequestAction::GetInstance(), nullptr);
    Config config;
    std::string tid;
    auto res = RequestAction::GetInstance()->Create(config, 1, tid);
    EXPECT_NE(res, 0);
}

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
    GrantDownSessionPermission();
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
    EXPECT_EQ(res, 21900006);
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
    EXPECT_EQ(res, 21900006);
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
    GrantDownInterPermission();
    auto res = RequestAction::GetInstance()->Start(tid);
    EXPECT_NE(res, 201);
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
    EXPECT_NE(res, 201);
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
    EXPECT_NE(res, 201);
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
    EXPECT_NE(res, 201);
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
    EXPECT_NE(res, 201);
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
    GrantDownInterPermission();
    auto res = RequestAction::GetInstance()->Resume(tid);
    EXPECT_NE(res, 201);
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
    GrantInternetPermission();
    auto res = RequestAction::GetInstance()->Start(tid);
    EXPECT_EQ(res, 21900006);
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
    GrantInternetPermission();
    auto res = RequestAction::GetInstance()->Resume(tid);
    EXPECT_EQ(res, 21900006);
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