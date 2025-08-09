/*
 * Copyright (c) 2025 Huawei Device Co., Ltd.
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

#include <atomic>
#include <chrono>
#include <cstddef>
#include <cstdint>
#include <iostream>
#include <memory>
#include <optional>
#include <thread>
#include <tuple>
#include <unordered_map>
#include <vector>

#include "accesstoken_kit.h"
#include "common.h"
#include "gmock/gmock.h"
#include "log.h"
#include "nativetoken_kit.h"
#include "request_preload.h"
#include "token_setproc.h"

using namespace testing::ext;
using namespace OHOS::Request;

constexpr size_t SLEEP_INTERVAL = 100;
static std::string TEST_URL_0 = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt";
constexpr uint64_t TEST_SIZE_0 = 1042003;
constexpr uint64_t INFO_SIZE_0 = 2;

void SetAccessTokenPermission()
{
    auto permissions = std::vector<std::string>();
    permissions.push_back("ohos.permission.INTERNET");
    permissions.push_back("ohos.permission.GET_NETWORK_INFO");

    auto processName = std::string("preload_info");
    auto perms = std::make_unique<const char *[]>(permissions.size());
    for (size_t i = 0; i < permissions.size(); i++) {
        perms[i] = permissions[i].c_str();
    }

    NativeTokenInfoParams infoInstance = {
        .dcapsNum = 0,
        .permsNum = permissions.size(),
        .aclsNum = 0,
        .dcaps = nullptr,
        .perms = perms.get(),
        .acls = nullptr,
        .processName = processName.c_str(),
        .aplStr = "system_core",
    };
    auto tokenId = GetAccessTokenId(&infoInstance);
    if (tokenId == 0) {
        REQUEST_HILOGI("GetAccessTokenId failed.");
        return;
    }
    int ret = SetSelfTokenID(tokenId);
    if (ret != 0) {
        REQUEST_HILOGI("SetSelfTokenID failed, code is %{public}d.", ret);
        return;
    }
    ret = OHOS::Security::AccessToken::AccessTokenKit::ReloadNativeTokenInfo();
    if (ret < 0) {
        REQUEST_HILOGI("ReloadNativeTokenInfo failed, code is %{public}d.", ret);
        return;
    }
}

void SetAccesslNoPermission()
{
    const char **perms = new const char *[0];
    NativeTokenInfoParams infoInstance = {
        .dcapsNum = 0,
        .permsNum = 0,
        .aclsNum = 0,
        .dcaps = nullptr,
        .perms = perms,
        .acls = nullptr,
        .processName = "preload_info",
        .aplStr = "system_core",
    };
    uint64_t tokenId = GetAccessTokenId(&infoInstance);
    SetSelfTokenID(tokenId);
    OHOS::Security::AccessToken::AccessTokenKit::ReloadNativeTokenInfo();
    delete[] perms;
}

class PreloadGetInfo : public testing::Test {
public:
    void SetUp();
    void TearDown();
};

void PreloadGetInfo::SetUp(void)
{
    // input testcase setup step，setup invoked before each testcases
    testing::UnitTest *test = testing::UnitTest::GetInstance();
    ASSERT_NE(test, nullptr);
    const testing::TestInfo *testInfo = test->current_test_info();
    ASSERT_NE(testInfo, nullptr);
    string testCaseName = string(testInfo->name());
    REQUEST_HILOGI("[SetUp] %{public}s start", testCaseName.c_str());
    GTEST_LOG_(INFO) << testCaseName.append(" start");
    SetAccessTokenPermission();
}

void PreloadGetInfo::TearDown(void)
{
    // input testCase teardown step，teardown invoked after each testCase
    SetAccesslNoPermission();
}

void PreDownloadInfo(std::string url, uint64_t size)
{
    Preload::GetInstance()->Remove(url);
    EXPECT_FALSE(Preload::GetInstance()->Contains(url));

    TestCallback test(size);
    auto &[flagS, flagF, flagC, flagP, callback] = test;

    Preload::GetInstance()->SetDownloadInfoListSize(INFO_SIZE_0);
    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback));
    EXPECT_FALSE(handle->IsFinish());
    EXPECT_EQ(handle->GetState(), PreloadState::RUNNING);

    while (!handle->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }
    return;
}

std::optional<CppDownloadInfo> TestGetInfo(std::string url)
{
    return Preload::GetInstance()->GetDownloadInfo(url);
}

/**
 * @tc.name: GetInfoTest
 * @tc.desc: Test GetInfoTest interface base function - GetDownloadInfo
 * @tc.precon: NA
 * @tc.step: 1. Remove test URL from preload manager
 *           2. Create test callback and load valid URL
 *           3. Verify handle is running
 *           4. Wait for download completion
 *           5. Verify download info retrieved successfully
 * @tc.expect: Download info contains valid URL, size, and SUCCESS state
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadGetInfo, GetInfoTest, TestSize.Level1)
{
    PreDownloadInfo(TEST_URL_0, TEST_SIZE_0);
    std::optional<CppDownloadInfo> info = TestGetInfo(TEST_URL_0);
    EXPECT_TRUE(info.has_value());
    const auto &value = info.value();
    EXPECT_TRUE(value.dns_time() >= 0);
    EXPECT_TRUE(value.connect_time() >= 0);
    EXPECT_TRUE(value.total_time() >= 0);
    EXPECT_TRUE(value.tls_time() >= 0);
    EXPECT_TRUE(value.first_send_time() >= 0);
    EXPECT_TRUE(value.first_recv_time() >= 0);
    EXPECT_TRUE(value.redirect_time() >= 0);
    EXPECT_TRUE(value.resource_size() >= 0);
    EXPECT_TRUE(value.network_ip().empty());
    Preload::GetInstance()->Remove(TEST_URL_0);
}

/**
 * @tc.name: CppInfoMove
 * @tc.desc: Test CppInfoMove interface base function - GetDownloadInfo
 * @tc.precon: NA
 * @tc.step: 1. Remove test URL from preload manager
 *           2. Create test callback and load valid URL
 *           3. Verify handle is running
 *           4. Wait for download completion
 *           5. Verify download info retrieved and moved successfully
 * @tc.expect: Download info contains valid URL, size, and SUCCESS state after move
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadGetInfo, CppInfoMove, TestSize.Level1)
{
    PreDownloadInfo(TEST_URL_0, TEST_SIZE_0);
    std::optional<CppDownloadInfo> result = TestGetInfo(TEST_URL_0);
    CppDownloadInfo info1(std::move(result.value()));
    double dnsTime = info1.dns_time();

    std::optional<CppDownloadInfo> result2 = TestGetInfo(TEST_URL_0);
    CppDownloadInfo info2(std::move(result2.value()));
    info2 = std::move(info1);
    EXPECT_EQ(info2.dns_time(), dnsTime);
}
