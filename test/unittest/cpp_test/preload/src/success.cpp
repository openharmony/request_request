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
constexpr size_t FETCH_INTERVAL = 5;
static std::string TEST_URL_0 = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/"
                                "test.txt";
static std::string TEST_URL_1 = "https://www.w3cschool.cn/statics/demosource/movie.mp4";
static std::string TEST_URL_2 = "https://www.baidu.com";
static std::string TEST_URL_3 = "https://vd4.bdstatic.com/mda-pm7bte3t6fs50rsh/sc/cae_h264/"
                                "1702057792414494257/"
                                "mda-pm7bte3t6fs50rsh.mp4?v_from_s=bdapp-author-nanjing";

constexpr uint64_t TEST_SIZE_0 = 1042003;
constexpr uint64_t TEST_SIZE_1 = 318465;

class PreloadSuccess : public testing::Test {
public:
    void SetUp();
};

void PreloadSuccess::SetUp(void)
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
    EXPECT_FALSE(Preload::GetInstance()->Contains(url));

    TestCallback test(size);
    auto &[flagS, flagF, flagC, flagP, callback] = test;

    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback));
    EXPECT_FALSE(handle->IsFinish());
    EXPECT_EQ(handle->GetState(), PreloadState::RUNNING);

    while (!handle->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }
    EXPECT_FALSE(flagF->load());
    EXPECT_FALSE(flagC->load());
    EXPECT_TRUE(flagP->load());
    EXPECT_TRUE(flagS->load());
    EXPECT_TRUE(Preload::GetInstance()->Contains(url));
    EXPECT_EQ(handle->GetState(), PreloadState::SUCCESS);
    EXPECT_NE(Preload::GetInstance()->fetch(url), std::nullopt);
    Preload::GetInstance()->Remove(url);
}

/**
 * @tc.name: OnSuccessTest
 * @tc.desc: Test OnSuccessTest interface base function - OnSuccess
 * @tc.precon: NA
 * @tc.step: 1. Remove test URL from preload manager
 *           2. Create test callback and load valid URL
 *           3. Verify handle is running
 *           4. Wait for download completion
 *           5. Verify success callback triggered and data cached
 * @tc.expect: Download succeeds with SUCCESS state and OnSuccess callback triggered
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadSuccess, OnSuccessTest, TestSize.Level1)
{
    // chunk
    DownloadSuccessTest(TEST_URL_0, TEST_SIZE_0);
    // content-length
    DownloadSuccessTest(TEST_URL_1, TEST_SIZE_1);
}

/**
 * @tc.name: OnSuccessAddCallback
 * @tc.desc: Test Add callback for same url after success - post-success loading
 * @tc.precon: NA
 * @tc.step: 1. Remove test URL from preload manager
 *           2. Create first callback and load URL
 *           3. Wait for download completion
 *           4. Create second callback and load same URL
 *           5. Verify second callback uses cached data
 * @tc.expect: Second callback completes immediately using cached data
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */

HWTEST_F(PreloadSuccess, OnSuccessAddCallback, TestSize.Level1)
{
    auto url = TEST_URL_2;
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

    EXPECT_FALSE(flagF->load());
    EXPECT_FALSE(flagF_1->load());
    EXPECT_FALSE(flagC->load());
    EXPECT_FALSE(flagC_1->load());

    EXPECT_TRUE(flagP->load());
    EXPECT_TRUE(flagP_1->load());
    EXPECT_TRUE(flagS->load());
    EXPECT_TRUE(flagS_1->load());
    EXPECT_NE(Preload::GetInstance()->fetch(url), std::nullopt);
    Preload::GetInstance()->Remove(url);
}

/**
 * @tc.name: OnSuccessCache
 * @tc.desc: Test cached data
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */

HWTEST_F(PreloadSuccess, OnSuccessCache, TestSize.Level1)
{
    auto url = TEST_URL_3;

    Preload::GetInstance()->Remove(url);
    TestCallback test;
    auto &[flagS, flagF, flagC, flagP, callback] = test;

    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback));

    while (!handle->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }

    TestCallback test1;
    auto &[flagS_1, flagF_1, flagC_1, flagP_1, callback_1] = test1;

    auto handle_1 = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback_1));
    std::this_thread::sleep_for(std::chrono::milliseconds(FETCH_INTERVAL));

    EXPECT_FALSE(flagF->load());
    EXPECT_FALSE(flagF_1->load());
    EXPECT_FALSE(flagC->load());
    EXPECT_FALSE(flagC_1->load());
    EXPECT_FALSE(flagP_1->load());

    EXPECT_TRUE(flagP->load());
    EXPECT_TRUE(flagS->load());
    EXPECT_TRUE(flagS_1->load());
    EXPECT_NE(Preload::GetInstance()->fetch(url), std::nullopt);
    Preload::GetInstance()->Remove(url);
}

/**
 * @tc.name: PreloadSuccessCache
 * @tc.desc: Test cached data
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */

HWTEST_F(PreloadSuccess, PreloadSuccessCache, TestSize.Level1)
{
    std::string url = TEST_URL_0;
    auto preload = Preload::GetInstance();
    preload->SetRamCacheSize(1000);
    preload->SetFileCacheSize(1000);
    TestCallback test;
    auto &[flagS, flagF, flagC, flagP, callback] = test;
    auto handle = preload->load(url, std::make_unique<PreloadCallback>(callback));
    auto id = handle->GetTaskId();
    EXPECT_FALSE(id.empty());
    preload->Cancel(url);
    handle->Cancel();
}

/**
 * @tc.name: PreloadSslTypeTls
 * @tc.desc: Test SslType::TLS for download
 * @tc.precon: NA
 * @tc.step: 1. Remove test URL from preload manager
 *           2. Create test callback and load valid URL with SslType::TLS
 *           3. Verify handle is running
 *           4. Wait for download completion
 *           5. Verify success callback triggered
 * @tc.expect: Download succeeds with SUCCESS state and OnSuccess callback triggered
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadSuccess, PreloadSslTypeTls, TestSize.Level1)
{
    std::string url = TEST_URL_0;

    Preload::GetInstance()->Remove(url);
    TestCallback test;
    auto &[flagS, flagF, flagC, flagP, callback] = test;
    std::unique_ptr<PreloadOptions> options = std::make_unique<PreloadOptions>();
    options->sslType = SslType::TLS;

    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback), std::move(options));

    while (!handle->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }
    EXPECT_FALSE(flagF->load());
    EXPECT_FALSE(flagC->load());

    EXPECT_TRUE(flagP->load());
    EXPECT_TRUE(flagS->load());
    Preload::GetInstance()->Remove(url);
}

/**
 * @tc.name: PreloadSuccessTls
 * @tc.desc: Test SslType::TLS for download
 * @tc.precon: NA
 * @tc.step: 1. Remove test URL from preload manager
 *           2. Create test callback and load valid URL with SslType::DEFAULT
 *           3. Verify handle is running
 *           4. Wait for download completion
 *           5. Verify success callback triggered
 * @tc.expect: Download succeeds with SUCCESS state and OnSuccess callback triggered
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadSuccess, PreloadSslTypeDefault, TestSize.Level1)
{
    std::string url = TEST_URL_0;

    Preload::GetInstance()->Remove(url);
    TestCallback test;
    auto &[flagS, flagF, flagC, flagP, callback] = test;
    std::unique_ptr<PreloadOptions> options = std::make_unique<PreloadOptions>();
    options->sslType = SslType::DEFAULT;

    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback), std::move(options));

    while (!handle->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }
    EXPECT_FALSE(flagF->load());
    EXPECT_FALSE(flagC->load());

    EXPECT_TRUE(flagP->load());
    EXPECT_TRUE(flagS->load());
    Preload::GetInstance()->Remove(url);
}

/**
 * @tc.name: PreloadSuccessTls
 * @tc.desc: Test SslType::TLS for download.
 * @tc.precon: NA
 * @tc.step: 1. Remove test URL from preload manager
 *           2. Create test callback and load valid URL with SslType::TLS.
 *           3. Verify handle is running
 *           4. Wait for download completion
 *           5. Verify success callback triggered
 * @tc.expect: Download succeeds with SUCCESS state and OnSuccess callback triggered
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadSuccess, PreloadSslTypeBadCa, TestSize.Level1)
{
    std::string url = TEST_URL_0;

    Preload::GetInstance()->Remove(url);
    TestCallback test;
    auto &[flagS, flagF, flagC, flagP, callback] = test;
    std::unique_ptr<PreloadOptions> options = std::make_unique<PreloadOptions>();
    options->sslType = SslType::TLS;
    options->caPath = "/data/notExist.cert";

    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback), std::move(options));

    while (!handle->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }
    EXPECT_FALSE(flagF->load());
    EXPECT_FALSE(flagC->load());

    EXPECT_TRUE(flagP->load());
    EXPECT_TRUE(flagS->load());
    Preload::GetInstance()->Remove(url);
}
