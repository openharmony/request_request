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

#include "c_event_handler.h"

#include <gtest/gtest.h>
using namespace testing::ext;

#define PARAM_FUNC_RET 123

class EventHandlerTest : public testing::Test {
public:
    static void SetUpTestCase(void);
    static void TearDownTestCase(void);
    void SetUp();
    void TearDown();
};

void EventHandlerTest::SetUpTestCase(void)
{
    // input testsuit setup step，setup invoked before all testcases
}

void EventHandlerTest::TearDownTestCase(void)
{
    // input testsuit teardown step，teardown invoked after all testcases
}

void EventHandlerTest::SetUp(void)
{
    // input testcase setup step，setup invoked before each testcases
}

void EventHandlerTest::TearDown(void)
{
    // input testcase teardown step，teardown invoked after each testcases
}

// function for testing RequestPostTask
static int32_t ParamFunc()
{
    return PARAM_FUNC_RET;
}

/**
 * @tc.name: InitServiceHandlerTest_001
 * @tc.desc: Test RequestInitServiceHandler interface base function
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(EventHandlerTest, InitServiceHandlerTest_001, TestSize.Level1)
{
    RequestPostTask(ParamFunc);
    RequestInitServiceHandler();
    RequestInitServiceHandler();
    RequestPostTask(ParamFunc);
}