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

static std::string TEST_URL_0 = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt";

class PreloadAbnormalTest : public testing::Test {
public:
    void SetUp();
};

void PreloadAbnormalTest::SetUp(void)
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
 * @tc.name: PreloadAbnormalTest
 * @tc.desc: Test PreloadAbnormalTest interface base function - nullCallback
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(PreloadAbnormalTest, PreloadAbnormalTest, TestSize.Level1)
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