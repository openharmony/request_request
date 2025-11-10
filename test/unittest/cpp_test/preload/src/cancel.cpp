/*
 * Copyright (c) 2023 Huawei Device Co., Ltd.
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
#include <memory>
#include <thread>
#include <vector>

#include "common.h"
#include "gmock/gmock.h"
#include "log.h"
#include "request_preload.h"
#include "utf8_utils.h"
using namespace testing::ext;
using namespace OHOS::Request;
using namespace std;
class PreloadCancel : public testing::Test {
public:
    void SetUp();
};

void PreloadCancel::SetUp(void)
{
    // input testcase setup step，setup invoked before each testcases
    testing::UnitTest *test = testing::UnitTest::GetInstance();
    ASSERT_NE(test, nullptr);
    const testing::TestInfo *testInfo = test->current_test_info();
    ASSERT_NE(testInfo, nullptr);
    string testCaseName = string(testInfo->name());
    REQUEST_HILOGI("[SetUp] %{public}s start", testCaseName.c_str());
    GTEST_LOG_(INFO) << testCaseName.append(" start");
}

static std::string TEST_URL_0 = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/"
                                "test.txt";
static std::string TEST_URL_1 = "https://www.w3cschool.cn/statics/demosource/movie.mp4";
static std::string TEST_URL_2 = "https://www.baidu.com";
static std::string TEST_URL_3 = "https://vd4.bdstatic.com/mda-pm7bte3t6fs50rsh/sc/cae_h264/"
                                "1702057792414494257/"
                                "mda-pm7bte3t6fs50rsh.mp4?v_from_s=bdapp-author-nanjing";

constexpr size_t SLEEP_INTERVAL = 1000;

void DownloadCancelTest(std::string url)
{
    Preload::GetInstance()->Remove(url);
    EXPECT_FALSE(Preload::GetInstance()->Contains(url));

    TestCallback test;
    auto &[flagS, flagF, flagC, flagP, callback] = test;

    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback));
    EXPECT_FALSE(handle->IsFinish());
    EXPECT_EQ(handle->GetState(), PreloadState::RUNNING);

    handle->Cancel();
    size_t counter = 10;
    while ((!handle->IsFinish() || !(flagC->load() || flagF->load() || flagS->load())) && counter-- > 0) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }
    EXPECT_FALSE(flagF->load());
    EXPECT_FALSE(flagS->load());
    EXPECT_TRUE(flagC->load());
    EXPECT_EQ(flagP->load(), 0);
    EXPECT_FALSE(Preload::GetInstance()->Contains(url));
    Preload::GetInstance()->Remove(url);
}

/**
 * @tc.name: OnCancelTest
 * @tc.desc: Test PreloadCancel interface base function - OnCancel
 * @tc.precon: NA
 * @tc.step: 1. Remove test URL from preload manager
 *           2. Create test callback and load URL
 *           3. Verify handle is running
 *           4. Cancel the download and wait for completion
 *           5. Verify cancel callback triggered and URL removed
 * @tc.expect: Cancel operation succeeds and callbacks triggered correctly
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadCancel, OnCancelTest, TestSize.Level1)
{
    // chunk
    DownloadCancelTest(TEST_URL_0);
    // content-length
    DownloadCancelTest(TEST_URL_1);
    DownloadCancelTest(TEST_URL_2);
    DownloadCancelTest(TEST_URL_3);
}

/**
 * @tc.name: OnCancelAddCallback_0
 * @tc.desc: Test Add callback for same url on cancel
 * @tc.precon: NA
 * @tc.step: 1. Remove test URL from preload manager
 *           2. Create first test callback and load URL
 *           3. Create second test callback and load same URL
 *           4. Cancel first handle and wait for completion
 *           5. Verify callbacks triggered correctly for both handles
 * @tc.expect: Second handle continues normally despite first handle cancellation
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadCancel, OnCancelAddCallback_0, TestSize.Level1)
{
    auto url = TEST_URL_0;
    Preload::GetInstance()->Remove(url);

    TestCallback test;
    auto &[flagS, flagF, flagC, flagP, callback] = test;

    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback));

    TestCallback test1;
    auto &[flagS_1, flagF_1, flagC_1, flagP_1, callback_1] = test1;

    auto handle_1 = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback_1));
    handle->Cancel();
    while (!handle->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }

    EXPECT_FALSE(flagF->load());
    EXPECT_FALSE(flagF_1->load());
    EXPECT_FALSE(flagC->load());
    EXPECT_FALSE(flagC_1->load());

    EXPECT_TRUE(flagP->load());
    EXPECT_TRUE(flagP_1->load());
    EXPECT_TRUE(flagS->load());
    EXPECT_TRUE(flagS_1->load());
    Preload::GetInstance()->Remove(url);
}

/**
 * @tc.name: OnCancelAddCallback_1
 * @tc.desc: Test Add callback for same url on cancel - dual cancellation
 * @tc.precon: NA
 * @tc.step: 1. Remove test URL from preload manager
 *           2. Create first test callback and load URL
 *           3. Create second test callback and load same URL
 *           4. Cancel both handles and wait for completion
 *           5. Verify both handles trigger cancel callbacks
 * @tc.expect: Both handles complete with CANCEL state
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadCancel, OnCancelAddCallback_1, TestSize.Level1)
{
    auto url = TEST_URL_0;
    Preload::GetInstance()->Remove(url);

    TestCallback test;
    auto &[flagS, flagF, flagC, flagP, callback] = test;

    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback));

    TestCallback test1;
    auto &[flagS_1, flagF_1, flagC_1, flagP_1, callback_1] = test1;

    auto handle_1 = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback_1));
    handle->Cancel();
    handle_1->Cancel();

    while (!handle->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }
    std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));

    EXPECT_FALSE(flagF->load());
    EXPECT_FALSE(flagF_1->load());
    EXPECT_FALSE(flagP->load());
    EXPECT_FALSE(flagP_1->load());
    EXPECT_FALSE(flagS->load());
    EXPECT_FALSE(flagS_1->load());

    EXPECT_TRUE(flagC->load());
    EXPECT_TRUE(flagC_1->load());
    Preload::GetInstance()->Remove(url);
}

/**
 * @tc.name: OnCancelAddCallback_2
 * @tc.desc: Test Add callback for same url after cancellation
 * @tc.precon: NA
 * @tc.step: 1. Remove test URL from preload manager
 *           2. Create first test callback and load URL
 *           3. Cancel first handle immediately
 *           4. Create second test callback and load same URL after cancellation
 *           5. Wait for completion and verify callbacks triggered correctly
 * @tc.expect: Second handle runs normally after first handle cancellation
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadCancel, OnCancelAddCallback_2, TestSize.Level1)
{
    auto url = TEST_URL_0;
    Preload::GetInstance()->Remove(url);

    TestCallback test;
    auto &[flagS, flagF, flagC, flagP, callback] = test;

    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback));

    handle->Cancel();

    TestCallback test1;
    auto &[flagS_1, flagF_1, flagC_1, flagP_1, callback_1] = test1;

    auto handle_1 = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback_1));

    while (!handle_1->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }

    EXPECT_FALSE(flagF->load());
    EXPECT_FALSE(flagF_1->load());

    EXPECT_FALSE(flagP->load());
    EXPECT_TRUE(flagP_1->load());

    EXPECT_FALSE(flagS->load());
    EXPECT_TRUE(flagS_1->load());

    EXPECT_TRUE(flagC->load());
    EXPECT_FALSE(flagC_1->load());
    Preload::GetInstance()->Remove(url);
}

/**
 * @tc.name: Cancel_WhenUrlIsInvalidUtf8
 * @tc.desc: Test Add callback for same url after cancellation
 * @tc.precon: NA
 * @tc.step: 1. Remove test URL from preload manager
 *           2. Create first test callback and load URL
 *           3. Cancel first handle immediately
 *           4. Wait for completion and verify callbacks triggered correctly
 * @tc.expect: Handle runs normally after handle cancellation
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadCancel, Cancel_WhenUrlIsInvalidUtf8, TestSize.Level1)
{
    auto url = TEST_URL_0;
    Preload::GetInstance()->Remove(url);

    TestCallback test;
    auto &[flagS, flagF, flagC, flagP, callback] = test;

    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback));

    std::string invalidUtf8Url = "Test String Invalid \xFF\xFE";
    EXPECT_FALSE(Utf8Utils::RunUtf8Validation(std::vector<uint8_t>(invalidUtf8Url.begin(), invalidUtf8Url.end())));

    Preload::GetInstance()->Cancel(invalidUtf8Url);

    while (!handle->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }

    EXPECT_FALSE(flagF->load());
    EXPECT_TRUE(flagP->load());
    EXPECT_TRUE(flagS->load());
    EXPECT_FALSE(flagC->load());
    Preload::GetInstance()->Remove(url);
}

/**
 * @tc.name: Remove_WhenUrlIsInvalidUtf8
 * @tc.desc: Test Add callback for same url after cancellation
 * @tc.precon: NA
 * @tc.step: 1. Remove test URL from preload manager
 *           2. Create first test callback and load URL
 *           3. Remove first handle immediately
 *           4. Wait for completion and verify callbacks triggered correctly
 * @tc.expect: Handle runs normally after handle remove
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadCancel, Remove_WhenUrlIsInvalidUtf8, TestSize.Level1)
{
    auto url = TEST_URL_0;
    Preload::GetInstance()->Remove(url);

    TestCallback test;
    auto &[flagS, flagF, flagC, flagP, callback] = test;

    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback));

    std::string invalidUtf8Url = "Test String Invalid \xFF\xFE";
    EXPECT_FALSE(Utf8Utils::RunUtf8Validation(std::vector<uint8_t>(invalidUtf8Url.begin(), invalidUtf8Url.end())));

    Preload::GetInstance()->Remove(invalidUtf8Url);

    while (!handle->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }

    EXPECT_FALSE(flagF->load());
    EXPECT_TRUE(flagP->load());
    EXPECT_TRUE(flagS->load());
    EXPECT_FALSE(flagC->load());
    Preload::GetInstance()->Remove(url);
}