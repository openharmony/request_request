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

/**
 * @tc.name: WrapperCStringTest_001
 * @tc.desc: Test WrapperCString interface base function
 * @tc.type: FUNC
 * @tc.require: Issue Number
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

static std::string TEST_URL_0 = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/"
                                "test.txt";
static std::string TEST_URL_1 = "https://www.gitee.com/fqwert/aaaaaa";

constexpr size_t SLEEP_INTERVAL = 100;
constexpr size_t ABNORMAL_INTERVAL = 24;

class PreloadAbnormal : public testing::Test {
public:
    void SetUp();
};

void PreloadAbnormal::SetUp(void)
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

/**
 * @tc.name: NullptrTest
 * @tc.desc: Test nullptr callback
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(PreloadAbnormal, NullptrTest, TestSize.Level1)
{
    auto callback = PreloadCallback{
        .OnSuccess = nullptr,
        .OnCancel = nullptr,
        .OnFail = nullptr,
        .OnProgress = nullptr,
    };
    auto handle = Preload::GetInstance()->load(TEST_URL_0, std::make_unique<PreloadCallback>(callback));
    EXPECT_NE(handle, nullptr);
}

/**
 * @tc.name: SuccessBlockCallbackTest
 * @tc.desc: Test block callback not affect other callback
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(PreloadAbnormal, SuccessBlockCallbackTest, TestSize.Level1)
{
    auto url = TEST_URL_0;
    Preload::GetInstance()->Remove(url);
    auto abnormal_callback = PreloadCallback{
        .OnSuccess =
            [](const std::shared_ptr<Data> &&data, const std::string &taskId) {
                std::this_thread::sleep_for(std::chrono::hours(ABNORMAL_INTERVAL));
            },
    };
    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(abnormal_callback));

    TestCallback test;
    auto &[flagS, flagF, flagC, flagP, callback] = test;
    Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback));

    while (!handle->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }

    EXPECT_FALSE(flagF->load());
    EXPECT_FALSE(flagC->load());
    EXPECT_TRUE(flagP->load());
    EXPECT_TRUE(flagS->load());
    EXPECT_EQ(handle->GetState(), PreloadState::SUCCESS);
    Preload::GetInstance()->Remove(url);
}

/**
 * @tc.name: FailBlockCallbackTest
 * @tc.desc: Test block callback not affect other callback
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(PreloadAbnormal, FailBlockCallbackTest, TestSize.Level1)
{
    auto url = TEST_URL_1;
    Preload::GetInstance()->Remove(url);
    auto abnormal_callback = PreloadCallback{
        .OnFail =
            [](const PreloadError &error, const std::string &taskId) {
                std::this_thread::sleep_for(std::chrono::hours(ABNORMAL_INTERVAL));
            },
    };
    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(abnormal_callback));

    TestCallback test;
    auto &[flagS, flagF, flagC, flagP, callback] = test;
    Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback));

    while (!handle->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }

    EXPECT_TRUE(flagF->load());
    EXPECT_TRUE(flagP->load());
    EXPECT_FALSE(flagC->load());
    EXPECT_FALSE(flagS->load());
    EXPECT_EQ(handle->GetState(), PreloadState::FAIL);
    Preload::GetInstance()->Remove(url);
}

/**
 * @tc.name: CancelBlockCallbackTest
 * @tc.desc: Test block callback not affect other callback
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(PreloadAbnormal, CancelBlockCallbackTest, TestSize.Level1)
{
    auto url = TEST_URL_1;
    Preload::GetInstance()->Remove(url);
    auto abnormal_callback = PreloadCallback{
        .OnCancel = []() { std::this_thread::sleep_for(std::chrono::hours(ABNORMAL_INTERVAL)); },
    };
    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(abnormal_callback));

    TestCallback test;
    auto &[flagS, flagF, flagC, flagP, callback] = test;
    auto handle_1 = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback));
    handle->Cancel();
    handle_1->Cancel();

    while (!handle->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }
    std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));

    EXPECT_FALSE(flagF->load());
    EXPECT_TRUE(flagC->load());
    EXPECT_FALSE(flagP->load());
    EXPECT_FALSE(flagS->load());
    EXPECT_EQ(handle->GetState(), PreloadState::CANCEL);
    Preload::GetInstance()->Remove(url);
}

/**
 * @tc.name: ProgressBlockCallbackTest
 * @tc.desc: Test block callback not affect other callback
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(PreloadAbnormal, ProgressBlockCallbackTest, TestSize.Level1)
{
    auto url = TEST_URL_0;
    Preload::GetInstance()->Remove(url);
    auto abnormal_callback = PreloadCallback{
        .OnProgress = [](uint64_t current,
                          uint64_t total) { std::this_thread::sleep_for(std::chrono::hours(ABNORMAL_INTERVAL)); },
    };
    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(abnormal_callback));

    TestCallback test;
    auto &[flagS, flagF, flagC, flagP, callback] = test;
    auto handle_1 = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback));

    while (!handle->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }
    std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));

    EXPECT_FALSE(flagF->load());
    EXPECT_FALSE(flagC->load());
    EXPECT_TRUE(flagP->load());
    EXPECT_TRUE(flagS->load());
    EXPECT_EQ(handle->GetState(), PreloadState::SUCCESS);
    Preload::GetInstance()->Remove(url);
}