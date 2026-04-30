/*
 * Copyright (c) 2025 Huawei Device Co., Ltd.
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

#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>
#include <thread>

#include "application_context.h"
#include "context.h"
#include "log.h"
#include "preload_callback_test.h"
#include "request_preload.h"

using namespace testing::ext;
using namespace OHOS::Request;
using namespace OHOS::AbilityRuntime;
using namespace std;

constexpr size_t SLEEP_INTERVAL = 100;
static std::string g_testUrlNotExist = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/"
                                       "test_not_exist.txt";
class PreloadNapiTest : public testing::Test {
public:
    void SetUp();
};

void PreloadNapiTest::SetUp(void)
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
 * @tc.name: BuildDownloadInfoNullTest
 * @tc.desc: Test BuildDownloadInfo function
 * @tc.precon: NA
 * @tc.step: 1. Create callback.
 *           2. Dwonload test url with 404 response.
 *           3. Verify BuildDownloadInfo return nullptr.
 * @tc.expect: BuildDownloadInfo return nullptr.
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadNapiTest, BuildDownloadInfoNullTest, TestSize.Level1)
{
    std::string url = g_testUrlNotExist;
    Preload::GetInstance()->Remove(url);
    TestCallback test;
    auto &[flagS, flagF, flagInfo, flagC, flagP, callback] = test;
    std::unique_ptr<PreloadOptions> options = std::make_unique<PreloadOptions>();
    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback), std::move(options));

    size_t counter = 100;
    while ((!handle->IsFinish() || !(flagC->load() || flagF->load() || flagS->load())) && counter-- > 0) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }
    EXPECT_TRUE(flagF->load());
    EXPECT_TRUE(flagInfo->load());
    Preload::GetInstance()->Remove(url);
}

/**
 * @tc.name: SetGlobalRetryOptionsTest001
 * @tc.desc: Test SetGlobalRetryOptions with valid value
 * @tc.precon: NA
 * @tc.step: 1. Create RetryOptions with maxRetryCount = 5
 *           2. Call SetGlobalRetryOptions
 *           3. Restore to default value
 * @tc.expect: Function executes without error
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadNapiTest, SetGlobalRetryOptionsTest001, TestSize.Level1)
{
    RetryOptions retryOptions;
    retryOptions.maxRetryCount = 5;
    Preload::GetInstance()->SetGlobalRetryOptions(retryOptions);
    EXPECT_TRUE(true);
    // Restore to default
    retryOptions.maxRetryCount = 1;
    Preload::GetInstance()->SetGlobalRetryOptions(retryOptions);
}

/**
 * @tc.name: SetGlobalRetryOptionsTest002
 * @tc.desc: Test SetGlobalRetryOptions with minimum value
 * @tc.precon: NA
 * @tc.step: 1. Create RetryOptions with maxRetryCount = 0
 *           2. Call SetGlobalRetryOptions
 *           3. Restore to default value
 * @tc.expect: Function executes without error
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadNapiTest, SetGlobalRetryOptionsTest002, TestSize.Level1)
{
    RetryOptions retryOptions;
    retryOptions.maxRetryCount = 0;
    Preload::GetInstance()->SetGlobalRetryOptions(retryOptions);
    EXPECT_TRUE(true);
    // Restore to default
    retryOptions.maxRetryCount = 1;
    Preload::GetInstance()->SetGlobalRetryOptions(retryOptions);
}

/**
 * @tc.name: SetGlobalRetryOptionsTest003
 * @tc.desc: Test SetGlobalRetryOptions with maximum value
 * @tc.precon: NA
 * @tc.step: 1. Create RetryOptions with maxRetryCount = 10
 *           2. Call SetGlobalRetryOptions
 *           3. Restore to default value
 * @tc.expect: Function executes without error
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadNapiTest, SetGlobalRetryOptionsTest003, TestSize.Level1)
{
    RetryOptions retryOptions;
    retryOptions.maxRetryCount = 10;
    Preload::GetInstance()->SetGlobalRetryOptions(retryOptions);
    EXPECT_TRUE(true);
    // Restore to default
    retryOptions.maxRetryCount = 1;
    Preload::GetInstance()->SetGlobalRetryOptions(retryOptions);
}

/**
 * @tc.name: SetGlobalRetryOptionsTest004
 * @tc.desc: Test SetGlobalRetryOptions with default value
 * @tc.precon: NA
 * @tc.step: 1. Create RetryOptions with default maxRetryCount = 1
 *           2. Call SetGlobalRetryOptions
 * @tc.expect: Function executes without error
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadNapiTest, SetGlobalRetryOptionsTest004, TestSize.Level1)
{
    RetryOptions retryOptions;
    retryOptions.maxRetryCount = 1;
    Preload::GetInstance()->SetGlobalRetryOptions(retryOptions);
    EXPECT_TRUE(true);
}

/**
 * @tc.name: SetGlobalTimeoutOptionsTest001
 * @tc.desc: Test SetGlobalTimeoutOptions with valid values
 * @tc.precon: NA
 * @tc.step: 1. Create TimeoutOptions with networkCheckTimeout = 10, httpTotalTimeout = 30
 *           2. Call SetGlobalTimeoutOptions
 *           3. Restore to default values
 * @tc.expect: Function executes without error
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadNapiTest, SetGlobalTimeoutOptionsTest001, TestSize.Level1)
{
    TimeoutOptions timeoutOptions;
    timeoutOptions.networkCheckTimeout = 10;
    timeoutOptions.httpTotalTimeout = 30;
    Preload::GetInstance()->SetGlobalTimeoutOptions(timeoutOptions);
    EXPECT_TRUE(true);
    // Restore to default
    timeoutOptions.networkCheckTimeout = 20;
    timeoutOptions.httpTotalTimeout = 60;
    Preload::GetInstance()->SetGlobalTimeoutOptions(timeoutOptions);
}

/**
 * @tc.name: SetGlobalTimeoutOptionsTest002
 * @tc.desc: Test SetGlobalTimeoutOptions with minimum networkCheckTimeout
 * @tc.precon: NA
 * @tc.step: 1. Create TimeoutOptions with networkCheckTimeout = 0, httpTotalTimeout = 60
 *           2. Call SetGlobalTimeoutOptions
 *           3. Restore to default values
 * @tc.expect: Function executes without error
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadNapiTest, SetGlobalTimeoutOptionsTest002, TestSize.Level1)
{
    TimeoutOptions timeoutOptions;
    timeoutOptions.networkCheckTimeout = 0;
    timeoutOptions.httpTotalTimeout = 60;
    Preload::GetInstance()->SetGlobalTimeoutOptions(timeoutOptions);
    EXPECT_TRUE(true);
    // Restore to default
    timeoutOptions.networkCheckTimeout = 20;
    timeoutOptions.httpTotalTimeout = 60;
    Preload::GetInstance()->SetGlobalTimeoutOptions(timeoutOptions);
}

/**
 * @tc.name: SetGlobalTimeoutOptionsTest003
 * @tc.desc: Test SetGlobalTimeoutOptions with maximum networkCheckTimeout
 * @tc.precon: NA
 * @tc.step: 1. Create TimeoutOptions with networkCheckTimeout = 20, httpTotalTimeout = 60
 *           2. Call SetGlobalTimeoutOptions
 * @tc.expect: Function executes without error
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadNapiTest, SetGlobalTimeoutOptionsTest003, TestSize.Level1)
{
    TimeoutOptions timeoutOptions;
    timeoutOptions.networkCheckTimeout = 20;
    timeoutOptions.httpTotalTimeout = 60;
    Preload::GetInstance()->SetGlobalTimeoutOptions(timeoutOptions);
    EXPECT_TRUE(true);
}

/**
 * @tc.name: SetGlobalTimeoutOptionsTest004
 * @tc.desc: Test SetGlobalTimeoutOptions with minimum httpTotalTimeout
 * @tc.precon: NA
 * @tc.step: 1. Create TimeoutOptions with networkCheckTimeout = 20, httpTotalTimeout = 1
 *           2. Call SetGlobalTimeoutOptions
 *           3. Restore to default values
 * @tc.expect: Function executes without error
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadNapiTest, SetGlobalTimeoutOptionsTest004, TestSize.Level1)
{
    TimeoutOptions timeoutOptions;
    timeoutOptions.networkCheckTimeout = 20;
    timeoutOptions.httpTotalTimeout = 1;
    Preload::GetInstance()->SetGlobalTimeoutOptions(timeoutOptions);
    EXPECT_TRUE(true);
    // Restore to default
    timeoutOptions.httpTotalTimeout = 60;
    Preload::GetInstance()->SetGlobalTimeoutOptions(timeoutOptions);
}

/**
 * @tc.name: SetGlobalTimeoutOptionsTest005
 * @tc.desc: Test SetGlobalTimeoutOptions with default values
 * @tc.precon: NA
 * @tc.step: 1. Create TimeoutOptions with default values (20, 60)
 *           2. Call SetGlobalTimeoutOptions
 * @tc.expect: Function executes without error
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadNapiTest, SetGlobalTimeoutOptionsTest005, TestSize.Level1)
{
    TimeoutOptions timeoutOptions;
    timeoutOptions.networkCheckTimeout = 20;
    timeoutOptions.httpTotalTimeout = 60;
    Preload::GetInstance()->SetGlobalTimeoutOptions(timeoutOptions);
    EXPECT_TRUE(true);
}

/**
 * @tc.name: PreloadOptionsRetryTest001
 * @tc.desc: Test PreloadOptions with task-level retry configuration
 * @tc.precon: NA
 * @tc.step: 1. Create PreloadOptions with retry.maxRetryCount = 5
 *           2. Verify the retry configuration is set correctly
 * @tc.expect: retry.maxRetryCount equals 5
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadNapiTest, PreloadOptionsRetryTest001, TestSize.Level1)
{
    std::unique_ptr<PreloadOptions> options = std::make_unique<PreloadOptions>();
    options->retry.maxRetryCount = 5;
    EXPECT_EQ(options->retry.maxRetryCount, 5);
}

/**
 * @tc.name: PreloadOptionsTimeoutTest001
 * @tc.desc: Test PreloadOptions with task-level timeout configuration
 * @tc.precon: NA
 * @tc.step: 1. Create PreloadOptions with timeout settings
 *           2. Verify the timeout configuration is set correctly
 * @tc.expect: timeout fields equal expected values
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(PreloadNapiTest, PreloadOptionsTimeoutTest001, TestSize.Level1)
{
    std::unique_ptr<PreloadOptions> options = std::make_unique<PreloadOptions>();
    options->timeout.networkCheckTimeout = 15;
    options->timeout.httpTotalTimeout = 90;
    EXPECT_EQ(options->timeout.networkCheckTimeout, 15);
    EXPECT_EQ(options->timeout.httpTotalTimeout, 90);
}