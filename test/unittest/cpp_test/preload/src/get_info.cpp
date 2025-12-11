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

#include "common.h"
#include "gmock/gmock.h"
#include "log.h"
#include "request_preload.h"
#include "set_permission.h"
#include "utf8_utils.h"

using namespace testing::ext;
using namespace OHOS::Request;

constexpr size_t SLEEP_INTERVAL = 100;
static std::string TEST_URL_0 = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt";
constexpr uint64_t TEST_SIZE_0 = 1042003;
constexpr uint64_t INFO_SIZE_0 = 2;

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
    std::vector<std::string> permissions = { "ohos.permission.INTERNET", "ohos.permission.GET_NETWORK_INFO" };
    SetPermission::SetAccessTokenPermission(permissions, "preload_test");
}

void PreloadGetInfo::TearDown(void)
{
    // input testCase teardown step，teardown invoked after each testCase
    SetPermission::SetAccesslNoPermission("preload_test");
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
    EXPECT_FALSE(value.server_addr().empty());
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

/**
 * @tc.name: InfoIsInvalidUtf8
 * @tc.desc: Test GetDownloadInfo interface behavior with invalid UTF-8 URL input
 * @tc.precon: Preload module should validate UTF-8 encoding before processing URLs
 * @tc.step: 1. Verify UTF-8 validity
 *           2. Create invalid UTF-8 URL byte sequence
 *           3. Verify UTF-8 validation correctly identifies invalid input
 *           4. Call GetDownloadInfo with invalid UTF-8 URL
 *           5. Check return value for expected behavior
 * @tc.expect: 1. UTF-8 validation should return false for invalid input
 *             2. GetDownloadInfo should return std::nullopt for invalid UTF-8 URLs
 *             3. No crash or undefined behavior should occur
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadGetInfo, InfoIsInvalidUtf8, TestSize.Level1)
{
    std::string invalidUtf8Url = "Test String Invalid \xFF\xFE";
    EXPECT_FALSE(Utf8Utils::RunUtf8Validation(std::vector<uint8_t>(invalidUtf8Url.begin(), invalidUtf8Url.end())));

    auto result = Preload::GetInstance()->GetDownloadInfo(invalidUtf8Url);

    EXPECT_FALSE(result.has_value());

    EXPECT_FALSE(Preload::GetInstance()->Contains(invalidUtf8Url));
}

/**
 * @tc.name: InvalidUtf8_1
 * @tc.desc: Test RunUtf8Validation interface behavior with invalid/valid UTF-8 input
 * @tc.precon: NA
 * @tc.step: 1. Verify UTF-8 validity
 * @tc.expect: 1. Verify the validity is correct.
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadGetInfo, InvalidUtf8_1, TestSize.Level1)
{
    std::vector<uint8_t> test_ee_valid = { 0xEE, 0x80, 0x80 };
    EXPECT_TRUE(Utf8Utils::RunUtf8Validation(test_ee_valid));
    std::vector<uint8_t> test_ef_valid = { 0xEF, 0xBF, 0xBD };
    EXPECT_TRUE(Utf8Utils::RunUtf8Validation(test_ef_valid));
    std::vector<uint8_t> test_ee_max = { 0xEE, 0xBF, 0xBF };
    EXPECT_TRUE(Utf8Utils::RunUtf8Validation(test_ee_max));
    std::vector<uint8_t> test_ef_min = { 0xEF, 0x80, 0x80 };
    EXPECT_TRUE(Utf8Utils::RunUtf8Validation(test_ef_min));

    std::vector<uint8_t> test_f1_valid = { 0xF1, 0x80, 0x80, 0x80 };
    EXPECT_TRUE(Utf8Utils::RunUtf8Validation(test_f1_valid));
    std::vector<uint8_t> test_f2_valid = { 0xF2, 0x80, 0x80, 0x80 };
    EXPECT_TRUE(Utf8Utils::RunUtf8Validation(test_f2_valid));
    std::vector<uint8_t> test_f3_valid = { 0xF3, 0x80, 0x80, 0x80 };
    EXPECT_TRUE(Utf8Utils::RunUtf8Validation(test_f3_valid));
    std::vector<uint8_t> test_f1_max = { 0xF1, 0xBF, 0xBF, 0xBF };
    EXPECT_TRUE(Utf8Utils::RunUtf8Validation(test_f1_max));
    std::vector<uint8_t> test_f3_max = { 0xF3, 0xBF, 0xBF, 0xBF };
    EXPECT_TRUE(Utf8Utils::RunUtf8Validation(test_f3_max));

    std::vector<uint8_t> valid2 = { 0xC3, 0x87 };
    EXPECT_TRUE(Utf8Utils::RunUtf8Validation(valid2));
    std::vector<uint8_t> valid3 = { 0xE0, 0xA4, 0x85 };
    EXPECT_TRUE(Utf8Utils::RunUtf8Validation(valid3));
    std::vector<uint8_t> valid4 = { 0xF0, 0x90, 0x8C, 0x82 };
    EXPECT_TRUE(Utf8Utils::RunUtf8Validation(valid4));
    std::vector<uint8_t> valid5 = { 0xF4, 0x80, 0x80, 0x80 };
    EXPECT_TRUE(Utf8Utils::RunUtf8Validation(valid5));
    std::vector<uint8_t> valid6 = { 0xF4, 0x8F, 0xBF, 0xBF };
    EXPECT_TRUE(Utf8Utils::RunUtf8Validation(valid6));
    std::vector<uint8_t> mixed = { 'H', 'e', 'l', 'l', 'o', 0xC3, 0xA4, ' ', 0xE2, 0x82, 0xAC, '!' };
    EXPECT_TRUE(Utf8Utils::RunUtf8Validation(mixed));
}

/**
 * @tc.name: InvalidUtf8_2
 * @tc.desc: Test RunUtf8Validation interface behavior with invalid/valid UTF-8 input
 * @tc.precon: NA
 * @tc.step: 1. Verify UTF-8 validity
 * @tc.expect: 1. Verify the validity is correct.
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadGetInfo, InvalidUtf8_2, TestSize.Level1)
{
    std::vector<uint8_t> invalid = { 0xC2 };
    EXPECT_FALSE(Utf8Utils::RunUtf8Validation(invalid));
    std::vector<uint8_t> invalid_1 = { 0xE0, 0x9F, 0x80 };
    EXPECT_FALSE(Utf8Utils::RunUtf8Validation(invalid_1));
    std::vector<uint8_t> invalid_2 = { 0xED, 0xA0, 0x80 };
    EXPECT_FALSE(Utf8Utils::RunUtf8Validation(invalid_2));
    std::vector<uint8_t> invalid_3 = { 0xF0, 0x8F, 0x80, 0x80 };
    EXPECT_FALSE(Utf8Utils::RunUtf8Validation(invalid_3));
    std::vector<uint8_t> invalid_4 = { 0xF4, 0x90, 0x80, 0x80 };
    EXPECT_FALSE(Utf8Utils::RunUtf8Validation(invalid_4));
    std::vector<uint8_t> invalid_5 = { 0xE0, 0xA0 };
    EXPECT_FALSE(Utf8Utils::RunUtf8Validation(invalid_5));
    std::vector<uint8_t> invalid_6 = { 0x80 };
    EXPECT_FALSE(Utf8Utils::RunUtf8Validation(invalid_6));
    std::vector<uint8_t> invalid_7 = { 0xFF };
    EXPECT_FALSE(Utf8Utils::RunUtf8Validation(invalid_7));
    std::vector<uint8_t> invalid_8 = { 0xC0 };
    EXPECT_FALSE(Utf8Utils::RunUtf8Validation(invalid_8));
    std::vector<uint8_t> invalid_9 = { 0xF0, 0x90, 0x80 };
    EXPECT_FALSE(Utf8Utils::RunUtf8Validation(invalid_9));

    std::vector<uint8_t> v2_invalid = { 0xC2, 0x7F };
    EXPECT_FALSE(Utf8Utils::RunUtf8Validation(v2_invalid));
    std::vector<uint8_t> v3_invalid = { 0xE2, 0x82, 0x7F };
    EXPECT_FALSE(Utf8Utils::RunUtf8Validation(v3_invalid));
    std::vector<uint8_t> v3_invalid2 = { 0xE0, 0xA0, 0x7F };
    EXPECT_FALSE(Utf8Utils::RunUtf8Validation(v3_invalid2));
    std::vector<uint8_t> v4_invalid = { 0xF0, 0x9F, 0x98, 0x7F };
    EXPECT_FALSE(Utf8Utils::RunUtf8Validation(v4_invalid));
    std::vector<uint8_t> v4_invalid2 = { 0xF0, 0x90, 0x7F, 0x80 };
    EXPECT_FALSE(Utf8Utils::RunUtf8Validation(v4_invalid2));
    std::vector<uint8_t> v4_invalid3 = { 0xF4, 0x7F, 0x80, 0x80 };
    EXPECT_FALSE(Utf8Utils::RunUtf8Validation(v4_invalid3));
    std::vector<uint8_t> v4_invalid4 = { 0xF4, 0x90, 0x80, 0x80 };
    EXPECT_FALSE(Utf8Utils::RunUtf8Validation(v4_invalid4));
}

/**
 * @tc.name: ServerAddrInfo
 * @tc.desc: Test GetDownloadInfo interface behavior for server address retrieval
 * @tc.precon: NA
 * @tc.step: 1. Remove test URL from preload manager
 *           2. Create test callback and load valid URL
 *           3. Verify handle is running
 *           4. Wait for download completion
 *           5. Call GetDownloadInfo and verify server address info
 * @tc.expect: Download info contains non-empty server address list
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadGetInfo, ServerAddrInfo, TestSize.Level1)
{
    PreDownloadInfo(TEST_URL_0, TEST_SIZE_0);
    std::optional<CppDownloadInfo> result = TestGetInfo(TEST_URL_0);

    EXPECT_TRUE(result.has_value());
    EXPECT_TRUE(result.value().server_addr().size() >= 0);
}

/**
 * @tc.name: HttpFailCallbackInfo
 * @tc.desc: Test get the download information in the download failure callback.
 * @tc.precon: NA
 * @tc.step: 1. Remove test URL from preload manager
 *           2. Create test callback and load invalid path URL
 *           3. Verify handle is running
 *           4. Wait for download completion
 *           5. Verify download info contains non-empty server address list in failure callback
 * @tc.expect: Download info contains non-empty server address list
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadGetInfo, HttpFailCallbackInfo, TestSize.Level1)
{
    std::string invalidUtf8Url = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/"
                                 "notExistResource.txt";
    Preload::GetInstance()->Remove(invalidUtf8Url);
    EXPECT_FALSE(Preload::GetInstance()->Contains(invalidUtf8Url));

    TestCallback test(0);
    auto &[flagS, flagFDeprecated, flagC, flagP, callback] = test;

    auto flagF = std::make_shared<std::atomic_bool>(false);
    callback.OnFail = [flagF](const PreloadError &error, const std::string &taskId) {
        flagF->store(true);
        auto info = error.GetDownloadInfo();
        EXPECT_FALSE(info->server_addr().empty());
    };

    auto handle = Preload::GetInstance()->load(invalidUtf8Url, std::make_unique<PreloadCallback>(callback));
    EXPECT_FALSE(handle->IsFinish());
    EXPECT_EQ(handle->GetState(), PreloadState::RUNNING);

    size_t counter = 100;

    while ((!handle->IsFinish() || !(flagC->load() || flagF->load() || flagS->load())) && counter-- > 0) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }
    EXPECT_TRUE(flagF->load());
    EXPECT_FALSE(flagC->load());
    EXPECT_FALSE(flagS->load());
}

/**
 * @tc.name: DnsFailCallbackInfo
 * @tc.desc: Test get the download information in the download failure callback.
 * @tc.precon: NA
 * @tc.step: 1. Remove test URL from preload manager
 *           2. Create test callback and load invalid authority URL
 *           3. Verify handle is running
 *           4. Wait for download completion
 *           5. Verify download info contains non-empty server address list in failure callback
 * @tc.expect: Download info contains non-empty server address list
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadGetInfo, DnsFailCallbackInfo, TestSize.Level1)
{
    std::string invalidUtf8Url = "https://PreloadGetInfo.DnsFailCallbackInfo.InvalidAuthority/releases/download/v1.01/"
                                 "notExistResource.txt";
    Preload::GetInstance()->Remove(invalidUtf8Url);
    EXPECT_FALSE(Preload::GetInstance()->Contains(invalidUtf8Url));

    TestCallback test(0);
    auto &[flagS, flagFDeprecated, flagC, flagP, callback] = test;

    auto flagF = std::make_shared<std::atomic_bool>(false);
    callback.OnFail = [flagF](const PreloadError &error, const std::string &taskId) {
        flagF->store(true);
        auto info = error.GetDownloadInfo();
        EXPECT_TRUE(info->server_addr().empty());
    };

    auto handle = Preload::GetInstance()->load(invalidUtf8Url, std::make_unique<PreloadCallback>(callback));
    EXPECT_FALSE(handle->IsFinish());
    EXPECT_EQ(handle->GetState(), PreloadState::RUNNING);

    size_t counter = 90;
    size_t oneSecond = 1000;
    while ((!handle->IsFinish() || !(flagC->load() || flagF->load() || flagS->load())) && counter-- > 0) {
        std::this_thread::sleep_for(std::chrono::milliseconds(oneSecond));
    }
    EXPECT_TRUE(flagF->load());
    EXPECT_FALSE(flagC->load());
    EXPECT_FALSE(flagS->load());
}