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

#include <chrono>
#include <cstddef>
#include <cstdint>
#include <tuple>
#include <utility>

#include "http_client_request.h"
#include "log.h"
#include "netstack.h"

using namespace testing::ext;
using namespace OHOS::Request;

class NetstackRequest : public testing::Test {
public:
    void SetUp();
};

void NetstackRequest::SetUp(void)
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
 * @tc.name: SetRequestSslType
 * @tc.desc: Test SetRequestSslType function
 * @tc.precon: NA
 * @tc.step: 1. Create test HttpClientRequest
 *           2. Call SetRequestSslType
 *           5. Verify request is not nullptr
 * @tc.expect: No crash happen
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(NetstackRequest, SetRequestSslType, TestSize.Level1)
{
    // chunk
    std::unique_ptr<HttpClientRequest> request = NewHttpClientRequest();
    SetRequestSslType(*request, "TLS");
    SetRequestSslType(*request, "TLCP");
    SetRequestSslType(*request, "");
    ASSERT_NE(request, nullptr);
}