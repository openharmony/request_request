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
#include <optional>
#include <thread>
#include <tuple>
#include <unordered_map>
#include <utility>
#include <vector>

#include "common.h"
#include "gmock/gmock.h"
#include "log.h"
#include "request_preload.h"
using namespace testing::ext;
using namespace OHOS::Request;

constexpr size_t SLEEP_INTERVAL = 100;
static std::string TEST_URL_0 = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/"
                                "test.txt";

class PreloadClearCache : public testing::Test {
public:
    void SetUp();
};

void PreloadClearCache::SetUp(void)
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
 * @tc.name: ClearFileCacheTest
 * @tc.desc: Test ClearFileCache function
 * @tc.precon: NA
 * @tc.step: 1. Remove test URL from preload manager
 *           2. Create first callback and load URL
 *           3. Wait for download completion
 *           4. Verify cache exists and clear file cache
 *           5. Verify cache exists after clear file cache
 * @tc.expect: Second callback completes immediately using cached data
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadClearCache, ClearFileCacheTest, TestSize.Level1)
{
    auto url = TEST_URL_0;
    Preload::GetInstance()->Remove(url);

    TestCallback test;
    auto &[flagS, flagF, flagC, flagP, callback] = test;
    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback));

    size_t times = 100;
    while (!handle->IsFinish() && times > 0) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
        times--;
    }
    EXPECT_TRUE(flagS->load());
    EXPECT_NE(Preload::GetInstance()->fetch(url), std::nullopt);
    Preload::GetInstance()->ClearFileCache();
    EXPECT_NE(Preload::GetInstance()->fetch(url), std::nullopt);
    Preload::GetInstance()->Remove(url);
}

/**
 * @tc.name: ClearMemoryCacheTest
 * @tc.desc: Test ClearMemoryCache function
 * @tc.precon: NA
 * @tc.step: 1. Remove test URL from preload manager
 *           2. Create first callback and load URL
 *           3. Wait for download completion
 *           4. Verify cache exists and clear memory cache
 *           5. Verify cache exists after clear memory cache
 *           6. Clear file cache and memory cache
 *           7. Verify cache does not exist after clear file cache and memory cache
 * @tc.expect: Second callback completes immediately using cached data
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadClearCache, ClearMemoryCacheTest, TestSize.Level1)
{
    auto url = TEST_URL_0;
    Preload::GetInstance()->Remove(url);

    TestCallback test;
    auto &[flagS, flagF, flagC, flagP, callback] = test;
    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback));
    size_t times = 100;
    while (!handle->IsFinish() && times > 0) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
        times--;
    }
    EXPECT_TRUE(flagS->load());
    EXPECT_NE(Preload::GetInstance()->fetch(url), std::nullopt);
    Preload::GetInstance()->ClearMemoryCache();
    EXPECT_NE(Preload::GetInstance()->fetch(url), std::nullopt);
    Preload::GetInstance()->ClearFileCache();
    Preload::GetInstance()->ClearMemoryCache();
    EXPECT_EQ(Preload::GetInstance()->fetch(url), std::nullopt);
    Preload::GetInstance()->Remove(url);
}
