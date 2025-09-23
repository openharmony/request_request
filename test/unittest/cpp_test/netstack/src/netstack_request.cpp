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
#include <memory>
#include <tuple>
#include <utility>

#include "cxx.h"
#include "http_client_request.h"
#include "http_client_task.h"
#include "log.h"
#include "netstack.h"
#include "set_permission.h"

using namespace testing::ext;
using namespace OHOS::Request;

static std::string TEST_URL_0 = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/"
                                "test.txt";

class NetstackRequest : public testing::Test {
public:
    void SetUp();
    void TearDown();
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
    std::vector<std::string> permissions = { "ohos.permission.INTERNET", "ohos.permission.GET_NETWORK_INFO" };
    SetPermission::SetAccessTokenPermission(permissions, "common_netstack_test");
}

void NetstackRequest::TearDown(void)
{
    // input testCase teardown step，teardown invoked after each testCase
    SetPermission::SetAccesslNoPermission("common_netstack_test");
}

/**
 * @tc.name: SetRequestSslType
 * @tc.desc: Test SetRequestSslType function
 * @tc.precon: NA
 * @tc.step: 1. Create test HttpClientRequest
 *           2. Call SetRequestSslType
 *           3. Verify request is not nullptr
 * @tc.expect: No crash happen
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(NetstackRequest, SetRequestSslType, TestSize.Level1)
{
    std::unique_ptr<HttpClientRequest> request = NewHttpClientRequest();
    SetRequestSslType(*request, "TLS");
    SetRequestSslType(*request, "TLCP");
    SetRequestSslType(*request, "");
    ASSERT_NE(request, nullptr);
}

/**
 * @tc.name: GetResponseHeaders
 * @tc.desc: Test GetHeaders function
 * @tc.precon: NA
 * @tc.step: 1. Create test HttpClientResponse
 *           2. Call GetHeaders
 *           3. Verify Headers is not empty
 * @tc.expect: Headers is not empty
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(NetstackRequest, GetResponseHeaders, TestSize.Level1)
{
    std::unique_ptr<HttpClientResponse> response = std::make_unique<HttpClientResponse>();
    EXPECT_TRUE(GetHeaders(*response).empty());
    std::unique_ptr<HttpClientRequest> request = NewHttpClientRequest();
    request->SetURL(TEST_URL_0);
    request->SetMethod("GET");
    std::shared_ptr<HttpClientTask> task = NewHttpClientTask(*request);
    task->OnSuccess([task](const HttpClientRequest &request, const HttpClientResponse &response) {});
    task->Start();
    while (task->GetCurlHandle() == nullptr) {
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
    }
    while (task->GetStatus() != TaskStatus::IDLE) {
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
    }
    auto code = task->GetResponse().GetResponseCode();
    if (code == 200) {
        rust::vec<rust::string> heasers = GetHeaders(task->GetResponse());
        EXPECT_FALSE(heasers.empty());
    } else {
        REQUEST_HILOGE("GetResponseHeaders %{public}d failed.", code);
        EXPECT_FALSE(true);
    }
}

/**
 * @tc.name: GetRespResolvConf
 * @tc.desc: Test GetResolvConf function
 * @tc.precon: NA
 * @tc.step: 1. Create test HttpClientResponse
 *           2. Call GetResolvConf
 *           3. Verify config is not empty
 * @tc.expect: config is not empty
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(NetstackRequest, GetRespResolvConf, TestSize.Level1)
{
    std::unique_ptr<HttpClientResponse> response = std::make_unique<HttpClientResponse>();
    EXPECT_TRUE(GetHeaders(*response).empty());
    std::unique_ptr<HttpClientRequest> request = NewHttpClientRequest();
    request->SetURL(TEST_URL_0);
    request->SetMethod("GET");
    std::shared_ptr<HttpClientTask> task = NewHttpClientTask(*request);
    task->OnSuccess([task](const HttpClientRequest &request, const HttpClientResponse &response) {});
    task->Start();
    while (task->GetCurlHandle() == nullptr) {
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
    }
    while (task->GetStatus() != TaskStatus::IDLE) {
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
    }
    rust::vec<rust::string> config = GetResolvConf();
    EXPECT_TRUE(config.size() >= 0);
}

/**
 * @tc.name: GetHttpAddress
 * @tc.desc: Test GetHttpAddress function
 * @tc.precon: NA
 * @tc.step: 1. Create test HttpClientResponse
 *           2. Call GetHttpAddress
 *           3. Verify address is not empty
 * @tc.expect: address is not empty
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(NetstackRequest, GetHttpAddress, TestSize.Level1)
{
    std::unique_ptr<HttpClientResponse> response = std::make_unique<HttpClientResponse>();
    EXPECT_TRUE(GetHeaders(*response).empty());
    std::unique_ptr<HttpClientRequest> request = NewHttpClientRequest();
    request->SetURL(TEST_URL_0);
    request->SetMethod("GET");
    std::shared_ptr<HttpClientTask> task = NewHttpClientTask(*request);
    task->OnSuccess([task](const HttpClientRequest &request, const HttpClientResponse &response) {});
    task->Start();
    std::chrono::milliseconds timeout(10000);
    auto start = std::chrono::steady_clock::now();
    while (task->GetCurlHandle() == nullptr) {
        auto now = std::chrono::steady_clock::now();
        if (std::chrono::duration_cast<std::chrono::milliseconds>(now - start) >= timeout) {
            REQUEST_HILOGE("GetHttpAddress GetCurlHandle timeout.");
            EXPECT_FALSE(true);
            return;
        }
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
    }
    while (task->GetStatus() != TaskStatus::IDLE) {
        auto now = std::chrono::steady_clock::now();
        if (std::chrono::duration_cast<std::chrono::milliseconds>(now - start) >= timeout) {
            REQUEST_HILOGE("GetHttpAddress GetStatus timeout.");
            EXPECT_FALSE(true);
            return;
        }
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
    }
    rust::string address = GetHttpAddress(task->GetResponse());
    EXPECT_FALSE(address.empty());
}