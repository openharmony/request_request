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

#include "background_notification.h"

#include <gtest/gtest.h>

#include <cstdint>

#include "c_string_wrapper.h"
#include "js_common.h"

using namespace testing::ext;
using namespace OHOS::Request;

class BackgroundNotificationTest : public testing::Test {
public:
    static void SetUpTestCase(void);
    static void TearDownTestCase(void);
    void SetUp();
    void TearDown();
};

void BackgroundNotificationTest::SetUpTestCase(void)
{
    // input testsuit setup step，setup invoked before all testcases
}

void BackgroundNotificationTest::TearDownTestCase(void)
{
    // input testsuit teardown step，teardown invoked after all testcases
}

void BackgroundNotificationTest::SetUp(void)
{
    // input testcase setup step，setup invoked before each testcases
}

void BackgroundNotificationTest::TearDown(void)
{
    // input testcase teardown step，teardown invoked after each testcases
}

/**
 * @tc.name: BackGroundNotifyTest_001
 * @tc.desc: Test RequestBackgroundNotify interface base function - download
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(BackgroundNotificationTest, BackGroundNotifyTest_001, TestSize.Level1)
{
    RequestTaskMsg msg = RequestTaskMsg{
        123,                                   // uint32_t taskId
        123456,                                // pid_t uid
        static_cast<uint8_t>(Action::DOWNLOAD) // uint8_t action
    };

    CStringWrapper wrappedPath = WrapperCString("../BUILD.gn");
    CStringWrapper wrappedFileName = WrapperCString("BUILD.gn");
    uint32_t percent = 50;
    RequestBackgroundNotify(msg, wrappedPath, wrappedFileName, percent);
}

/**
 * @tc.name: BackGroundNotifyTest_002
 * @tc.desc: Test RequestBackgroundNotify interface base function - upload
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(BackgroundNotificationTest, BackGroundNotifyTest_002, TestSize.Level1)
{
    RequestTaskMsg msg = RequestTaskMsg{
        123,                                 // uint32_t taskId
        123456,                              // pid_t uid
        static_cast<uint8_t>(Action::UPLOAD) // uint8_t action
    };

    CStringWrapper wrappedPath = WrapperCString("../BUILD.gn");
    CStringWrapper wrappedFileName = WrapperCString("BUILD.gn");
    uint32_t percent = 50;
    RequestBackgroundNotify(msg, wrappedPath, wrappedFileName, percent);
}