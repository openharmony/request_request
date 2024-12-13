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

static std::string TEST_URL_0 = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/"
                                "test.txt";
static std::string TEST_URL_1 = "https://www.w3cschool.cn/statics/demosource/movie.mp4";
static std::string TEST_URL_2 = "https://www.baidu.com";
static std::string TEST_URL_3 = "https://vd4.bdstatic.com/mda-pm7bte3t6fs50rsh/sc/cae_h264/"
                                "1702057792414494257/"
                                "mda-pm7bte3t6fs50rsh.mp4?v_from_s=bdapp-author-nanjing";

constexpr size_t SLEEP_INTERVAL = 100;

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
    while (!handle->IsFinish()) {
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
 * @tc.desc: Test PreloadSuccessCache interface base function - OnSuccess
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */

HWTEST_F(PreloadProgress, OnProgressTest, TestSize.Level1)
{
    DownloadProgressTest(TEST_URL_0);
    DownloadProgressTest(TEST_URL_1);
    DownloadProgressTest(TEST_URL_2);
    DownloadProgressTest(TEST_URL_3);
}
