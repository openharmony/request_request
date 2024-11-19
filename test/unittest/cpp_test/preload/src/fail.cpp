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

#include "gmock/gmock.h"
#include "log.h"
#include "request_preload.h"
using namespace testing::ext;
using namespace OHOS::Request;

class PreloadFailTest : public testing::Test {
public:
    void SetUp();
};

void PreloadFailTest::SetUp(void)
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

static std::string TEST_URL_2 = "https://127.3.1.123";
constexpr size_t SLEEP_INTERVAL = 100;

void DownloadFailTest(std::string url)
{
    Preload::GetInstance()->Remove(url);
    auto flagS = std::make_shared<std::atomic_bool>(false);
    auto flagF = std::make_shared<std::atomic_bool>(false);
    auto flagC = std::make_shared<std::atomic_bool>(false);
    auto flagP = std::make_shared<std::atomic_int64_t>(0);

    auto callback = PreloadCallback{
        .OnSuccess = [flagS](const std::shared_ptr<Data> &&data, const std::string &taskId) { flagS->store(true); },
        .OnCancel = [flagC]() { flagC->store(true); },
        .OnFail = [flagF](const PreloadError &error, const std::string &taskId) { flagF->store(true); },
        .OnProgress = [flagP](uint64_t current, uint64_t total) { flagP->fetch_add(1); },
    };
    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback));
    EXPECT_FALSE(handle->IsFinish());

    while (!handle->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }
    EXPECT_TRUE(flagF->load());
    EXPECT_FALSE(flagS->load());
    EXPECT_FALSE(flagC->load());
    EXPECT_EQ(flagP->load(), 0);
    Preload::GetInstance()->Remove(url);
}

/**
 * @tc.name: PreloadFailTest
 * @tc.desc: Test PreloadFailTest interface base function - OnFail
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(PreloadFailTest, PreloadFailTest, TestSize.Level1)
{
    auto flag = std::make_shared<std::atomic_uint64_t>(0);
    auto callback = PreloadCallback{
        .OnSuccess = [](const std::shared_ptr<Data> &&data, const std::string &taskId) {},
        .OnCancel = []() {},
        .OnFail = [flag](const PreloadError &error, const std::string &taskId) { flag->fetch_add(1); },
        .OnProgress = [](uint64_t current, uint64_t total) {},
    };

    auto handle = Preload::GetInstance()->load(TEST_URL_2, std::make_unique<PreloadCallback>(callback));
    std::this_thread::sleep_for(std::chrono::seconds(1));
    EXPECT_EQ(flag->load(), 1);
    EXPECT_TRUE(handle->IsFinish());
    EXPECT_EQ(handle->GetState(), PreloadState::FAIL);
}