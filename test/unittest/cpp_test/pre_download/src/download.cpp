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
#include "request_pre_download.h"
using namespace testing::ext;
using namespace OHOS::Request;

class PreloadTest : public testing::Test {
public:
    static void SetUpTestCase(void);
    static void TearDownTestCase(void);
    void SetUp();
    void TearDown();
};

static std::string TEST_URL_0 = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt";
static std::string TEST_URL_4 = "https://www.w3cschool.cn/statics/demosource/movie.mp4";
static std::string TEST_URL_1 = "https://www.baidu.com";
static std::string TEST_URL_2 = "https://127.3.1.123";

constexpr size_t INTERVAL = 500;
constexpr uint64_t TEST_SIZE = 1042003;
constexpr uint64_t TEST_SIZE_4 = 318465;

void PreloadTest::SetUpTestCase(void)
{
    // input testsuit setup step，setup invoked before all testcases
}

void PreloadTest::TearDownTestCase(void)
{
    // input testsuit teardown step，teardown invoked after all testcases
}

void PreloadTest::SetUp(void)
{
    // input testcase setup step，setup invoked before each testcases
}

void PreloadTest::TearDown(void)
{
    // input testcase teardown step，teardown invoked after each testcases
}

void DownloadSuccessTest(std::string url, uint64_t size)
{
    auto flagS = std::make_shared<std::atomic_uint64_t>(0);
    PreloadOptions options = { .headers = std::vector<std::tuple<std::string, std::string>>() };
    options.headers.push_back(std::tuple<std::string, std::string>("Accept", "text/html"));

    auto flagP = std::make_shared<std::atomic_int64_t>(0);
    auto callback = PreloadCallback{
        .OnSuccess = [flagS](const std::shared_ptr<Data> &&data,
                         const std::string &taskId) { flagS->store(data->bytes().size()); },
        .OnCancel = []() {},
        .OnFail = [](const PreloadError &error, const std::string &taskId) {},
        .OnProgress = [flagP](uint64_t current, uint64_t total) { flagP->fetch_add(1); },
    };
    auto handle = Preload::GetInstance()->load(TEST_URL_0, std::make_unique<PreloadCallback>(callback));
    EXPECT_FALSE(handle->IsFinish());
    EXPECT_EQ(handle->GetState(), PreloadState::RUNNING);

    while (!handle->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(INTERVAL));
    }
    EXPECT_TRUE(flagP->load() > 0);
    EXPECT_EQ(flagS->load(), TEST_SIZE);
    EXPECT_EQ(handle->GetState(), PreloadState::SUCCESS);
    Preload::GetInstance()->Remove(TEST_URL_0);
}

// test success and progress callback
HWTEST_F(PreloadTest, PreloadTest_001, TestSize.Level1)
{
    DownloadSuccessTest(TEST_URL_0, TEST_SIZE);
    DownloadSuccessTest(TEST_URL_4, TEST_SIZE_4);
}

// test cancel callback
HWTEST_F(PreloadTest, PreloadTest_002, TestSize.Level1)
{
    auto flag = std::make_shared<std::atomic_uint64_t>(0);
    auto callback = PreloadCallback{
        .OnSuccess = [](const std::shared_ptr<Data> &&data, const std::string &taskId) {},
        .OnCancel = [flag]() { flag->fetch_add(1); },
        .OnFail = [](const PreloadError &error, const std::string &taskId) {},
        .OnProgress = [](uint64_t current, uint64_t total) {},
    };

    auto handle = Preload::GetInstance()->load(TEST_URL_1, std::make_unique<PreloadCallback>(callback));
    handle->Cancel();
    std::this_thread::sleep_for(std::chrono::seconds(1));
    EXPECT_EQ(flag->load(), 1);
    EXPECT_TRUE(handle->IsFinish());
    EXPECT_EQ(handle->GetState(), PreloadState::CANCEL);
    Preload::GetInstance()->Remove(TEST_URL_1);
}

// test fail callback
HWTEST_F(PreloadTest, PreloadTest_003, TestSize.Level1)
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

// test nullptr callback
HWTEST_F(PreloadTest, PreloadTest_004, TestSize.Level1)
{
    auto callback = PreloadCallback{
        .OnSuccess = nullptr,
        .OnCancel = nullptr,
        .OnFail = nullptr,
        .OnProgress = nullptr,
    };

    auto handle = Preload::GetInstance()->load(TEST_URL_1, std::make_unique<PreloadCallback>(callback));
}