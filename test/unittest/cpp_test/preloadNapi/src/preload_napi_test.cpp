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