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
#include <tuple>
#include <unordered_map>
#include <vector>

#include "common.h"
#include "gmock/gmock.h"
#include "log.h"
#include "request_preload.h"

using namespace testing::ext;
using namespace OHOS::Request;

class PreloadFail : public testing::Test {
public:
    void SetUp();
};

void PreloadFail::SetUp(void)
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

static std::string TEST_URL_0 = "https://127.3.1.123";
static std::string TEST_URL_1 = "https://www.gitee.com/fqwert/aaaaa";
constexpr size_t SLEEP_INTERVAL = 100;

void DownloadFailTest(std::string url)
{
    Preload::GetInstance()->Remove(url);
    EXPECT_FALSE(Preload::GetInstance()->Contains(url));
    TestCallback test;
    auto &[flagS, flagF, flagC, flagP, callback] = test;

    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback));
    EXPECT_FALSE(handle->IsFinish());
    EXPECT_EQ(handle->GetState(), PreloadState::RUNNING);

    while (!handle->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }
    EXPECT_TRUE(flagF->load());
    EXPECT_FALSE(flagS->load());
    EXPECT_FALSE(flagC->load());
    EXPECT_FALSE(Preload::GetInstance()->Contains(url));
    EXPECT_EQ(Preload::GetInstance()->fetch(url), std::nullopt);
    EXPECT_EQ(flagP->load(), 0);
    EXPECT_EQ(handle->GetState(), PreloadState::FAIL);
    Preload::GetInstance()->Remove(url);
}

/**
 * @tc.name: OnFailTest
 * @tc.desc: Test PreloadFail interface base function - OnFail
 * @tc.precon: NA
 * @tc.step: 1. Remove test URL from preload manager
 *           2. Create test callback and load invalid URL
 *           3. Verify handle is running
 *           4. Wait for download failure
 *           5. Verify fail callback triggered and URL removed
 * @tc.expect: Download fails with FAIL state and OnFail callback triggered
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadFail, OnFailTest, TestSize.Level1)
{
    DownloadFailTest(TEST_URL_0);
}

/**
 * @tc.name: PreloadFailTest
 * @tc.desc: Test Add callback for same url on fail
 * @tc.precon: NA
 * @tc.step: 1. Remove test URL from preload manager
 *           2. Create first test callback and load invalid URL
 *           3. Create second test callback and load same invalid URL
 *           4. Wait for both downloads to fail
 *           5. Verify both fail callbacks triggered correctly
 * @tc.expect: Both handles fail with FAIL state and OnFail callbacks triggered
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadFail, OnFailAddCallback, TestSize.Level1)
{
    auto url = TEST_URL_1;
    Preload::GetInstance()->Remove(url);

    TestCallback test;
    auto &[flagS, flagF, flagC, flagP, callback] = test;

    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback));

    TestCallback test1;
    auto &[flagS_1, flagF_1, flagC_1, flagP_1, callback_1] = test1;

    auto handle_1 = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback_1));

    while (!handle->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }

    EXPECT_TRUE(flagF->load());
    EXPECT_TRUE(flagF_1->load());
    EXPECT_FALSE(flagC->load());
    EXPECT_FALSE(flagC_1->load());

    EXPECT_TRUE(flagP->load());
    EXPECT_TRUE(flagP_1->load());
    EXPECT_FALSE(flagS->load());
    EXPECT_FALSE(flagS_1->load());
    Preload::GetInstance()->Remove(url);
}