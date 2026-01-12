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

class PreloadProgress : public testing::Test {
public:
    void SetUp();
};

void PreloadProgress::SetUp(void)
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

static std::string g_testUrl0 = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/"
                                "test.txt";
static std::string g_testUrl1 = "https://www.baidu.com";

constexpr size_t SLEEP_INTERVAL = 1000;

void DownloadProgressTest(std::string url)
{
    Preload::GetInstance()->Remove(url);
    auto flagS = std::make_shared<std::atomic_bool>(false);
    auto flagF = std::make_shared<std::atomic_bool>(false);
    auto flagC = std::make_shared<std::atomic_bool>(false);
    auto flagTot = std::make_shared<std::atomic_uint64_t>(0);
    auto flagCur = std::make_shared<std::atomic_uint64_t>(0);
    auto flagP = std::make_shared<std::atomic_bool>(true);
    auto callback = PreloadCallback{
        .OnSuccess = [flagS](const std::shared_ptr<Data> &&data, const std::string &taskId) { flagS->store(true); },
        .OnCancel = [flagC]() { flagC->store(true); },
        .OnFail = [flagF](const PreloadError &error, const std::string &taskId) { flagF->store(true); },
        .OnProgress =
            [flagCur, flagTot, flagP](uint64_t current, uint64_t total) {
                if (flagCur->load() > current) {
                    flagP->store(false);
                }
                if (flagTot->load() > total) {
                    flagP->store(false);
                }
                flagCur->store(current);
                flagTot->store(total);
            },
    };
    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback));
    size_t counter = 10;
    while ((!handle->IsFinish() || !(flagC->load() || flagF->load() || flagS->load())) && counter-- > 0) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }
    EXPECT_FALSE(flagF->load());
    EXPECT_FALSE(flagC->load());

    EXPECT_TRUE(flagP->load());
    EXPECT_TRUE(flagS->load());

    EXPECT_EQ(flagCur->load(), flagTot->load());
    Preload::GetInstance()->Remove(url);
}

/**
 * @tc.name: OnProgressTest
 * @tc.desc: Test progress callback for multiple downloads.
 * @tc.precon: NA
 * @tc.step: 1. Remove test URL from preload manager
 *           2. Create first callback and load URL
 *           3. Create second callback and load another URL
 *           4. Wait for download completion with progress tracking

 * @tc.expect: Both callbacks complete successfully with progress tracking
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */

HWTEST_F(PreloadProgress, OnProgressTest, TestSize.Level1)
{
    DownloadProgressTest(g_testUrl0);
    DownloadProgressTest(g_testUrl1);
}
