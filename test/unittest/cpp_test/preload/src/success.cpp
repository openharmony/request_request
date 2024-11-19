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

constexpr size_t SLEEP_INTERVAL = 100;
constexpr size_t FETCH_INTERVAL = 5;
static std::string TEST_URL_0 = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt";
static std::string TEST_URL_1 = "https://www.w3cschool.cn/statics/demosource/movie.mp4";
static std::string TEST_URL_2 = "https://www.baidu.com";
static std::string TEST_URL_3 = "https://vd4.bdstatic.com/mda-pm7bte3t6fs50rsh/sc/cae_h264/1702057792414494257/"
                                "mda-pm7bte3t6fs50rsh.mp4?v_from_s=bdapp-author-nanjing";

constexpr uint64_t TEST_SIZE_0 = 1042003;
constexpr uint64_t TEST_SIZE_1 = 318465;

class PreloadSuccessTest : public testing::Test {
public:
    void SetUp();
};

void PreloadSuccessTest::SetUp(void)
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

void DownloadSuccessTest(std::string url, uint64_t size)
{
    Preload::GetInstance()->Remove(url);
    auto flagS = std::make_shared<std::atomic_uint64_t>(0);
    auto flagF = std::make_shared<std::atomic_bool>(false);
    auto flagC = std::make_shared<std::atomic_bool>(false);
    auto flagP = std::make_shared<std::atomic_int64_t>(0);

    auto callback = PreloadCallback{
        .OnSuccess = [flagS](const std::shared_ptr<Data> &&data,
                         const std::string &taskId) { flagS->store(data->bytes().size()); },
        .OnCancel = [flagC]() { flagC->store(true); },
        .OnFail = [flagF](const PreloadError &error, const std::string &taskId) { flagF->store(true); },
        .OnProgress = [flagP](uint64_t current, uint64_t total) { flagP->fetch_add(1); },
    };
    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback));
    EXPECT_FALSE(handle->IsFinish());

    while (!handle->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }
    EXPECT_FALSE(flagF->load());
    EXPECT_FALSE(flagC->load());
    EXPECT_TRUE(flagP->load() > 0);
    EXPECT_EQ(flagS->load(), size);
    Preload::GetInstance()->Remove(url);
}

/**
 * @tc.name: PreloadSuccessTest_001
 * @tc.desc: Test PreloadSuccessTest_001 interface base function - OnSuccess
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(PreloadSuccessTest, PreloadSuccessCallback, TestSize.Level1)
{
    // chunk
    DownloadSuccessTest(TEST_URL_0, TEST_SIZE_0);
    // content-length
    DownloadSuccessTest(TEST_URL_1, TEST_SIZE_1);
}

/**
 * @tc.name: PreloadSuccessAddCallback
 * @tc.desc: Test PreloadSuccessAddCallback interface base function - OnSuccess
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */

HWTEST_F(PreloadSuccessTest, PreloadSuccessAddCallback, TestSize.Level1)
{
    auto url = TEST_URL_2;
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

    auto flagS_1 = std::make_shared<std::atomic_bool>(false);
    auto flagF_1 = std::make_shared<std::atomic_bool>(false);
    auto flagC_1 = std::make_shared<std::atomic_bool>(false);
    auto flagP_1 = std::make_shared<std::atomic_int64_t>(0);

    auto callback_1 = PreloadCallback{
        .OnSuccess = [flagS_1](const std::shared_ptr<Data> &&data, const std::string &taskId) { flagS_1->store(true); },
        .OnCancel = [flagC_1]() { flagC_1->store(true); },
        .OnFail = [flagF_1](const PreloadError &error, const std::string &taskId) { flagF_1->store(true); },
        .OnProgress = [flagP_1](uint64_t current, uint64_t total) { flagP_1->fetch_add(1); },
    };

    auto handle_1 = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback_1));

    while (!handle->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }

    EXPECT_FALSE(flagF->load());
    EXPECT_FALSE(flagF_1->load());
    EXPECT_FALSE(flagC->load());
    EXPECT_FALSE(flagC_1->load());
    EXPECT_TRUE(flagP->load() > 0);
    EXPECT_TRUE(flagP_1->load() > 0);
    EXPECT_TRUE(flagS->load());
    EXPECT_TRUE(flagS_1->load());
    Preload::GetInstance()->Remove(url);
}

/**
 * @tc.name: PreloadSuccessCache
 * @tc.desc: Test PreloadSuccessCache interface base function - OnSuccess
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */

HWTEST_F(PreloadSuccessTest, PreloadSuccessCache, TestSize.Level1)
{
    auto url = TEST_URL_3;

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

    while (!handle->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }

    auto flagS_1 = std::make_shared<std::atomic_bool>(false);
    auto flagF_1 = std::make_shared<std::atomic_bool>(false);
    auto flagC_1 = std::make_shared<std::atomic_bool>(false);
    auto flagP_1 = std::make_shared<std::atomic_int64_t>(0);

    auto callback_1 = PreloadCallback{
        .OnSuccess = [flagS_1](const std::shared_ptr<Data> &&data, const std::string &taskId) { flagS_1->store(true); },
        .OnCancel = [flagC_1]() { flagC_1->store(true); },
        .OnFail = [flagF_1](const PreloadError &error, const std::string &taskId) { flagF_1->store(true); },
        .OnProgress = [flagP_1](uint64_t current, uint64_t total) { flagP_1->fetch_add(1); },
    };

    auto handle_1 = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback_1));
    std::this_thread::sleep_for(std::chrono::milliseconds(FETCH_INTERVAL));

    EXPECT_FALSE(flagF->load());
    EXPECT_FALSE(flagF_1->load());
    EXPECT_FALSE(flagC->load());
    EXPECT_FALSE(flagC_1->load());
    EXPECT_TRUE(flagP->load() > 0);
    EXPECT_EQ(flagP_1->load(), 0);
    EXPECT_TRUE(flagS->load());
    EXPECT_TRUE(flagS_1->load());
    Preload::GetInstance()->Remove(url);
}
