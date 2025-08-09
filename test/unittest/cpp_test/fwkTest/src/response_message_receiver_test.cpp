/*
 * Copyright (c) 2024 Huawei Device Co., Ltd.
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

#include <securec.h>
#include <sys/socket.h>

#include <cstring>

#define private public
#define protected public
#include <gtest/gtest.h>

#include <cstdint>
#include <memory>

#include "gmock/gmock.h"
#include "i_response_listener.h"
#include "log.h"
#include "request_common.h"
#include "response_message_receiver.h"

using namespace testing::ext;
using namespace OHOS::Request;

#undef private
#undef protected

class ResponseMessageReceiverTest : public testing::Test {
public:
    static void SetUpTestCase(void);
    static void TearDownTestCase(void);
    void SetUp();
    void TearDown();
};

void ResponseMessageReceiverTest::SetUpTestCase(void)
{
    // input testSuit setup step，setup invoked before all testCases
}

void ResponseMessageReceiverTest::TearDownTestCase(void)
{
    // input testSuit teardown step，teardown invoked after all testCases
}

void ResponseMessageReceiverTest::SetUp(void)
{
    // input testCase setup step，setup invoked before each testCase
    testing::UnitTest *test = testing::UnitTest::GetInstance();
    ASSERT_NE(test, nullptr);
    const testing::TestInfo *testInfo = test->current_test_info();
    ASSERT_NE(testInfo, nullptr);
    string testCaseName = string(testInfo->name());
    REQUEST_HILOGI("[SetUp] %{public}s start", testCaseName.c_str());
    GTEST_LOG_(INFO) << testCaseName.append(" start");
}

void ResponseMessageReceiverTest::TearDown(void)
{
    // input testCase teardown step，teardown invoked after each testCase
}

static constexpr int32_t ARRAY_LEN = 256; // 128 is array length
static constexpr int32_t INT64_SIZE = 8;  // 8 is int64 and uint64 num length
static constexpr int32_t INT32_SIZE = 4;  // 4 is int32 and uint32 num length
static constexpr int32_t INT16_SIZE = 2;  // 2 is int16 and uint16 num length

/**
 * @tc.name: Int64FromParcel001
 * @tc.desc: Test Int64FromParcel interface base function - Int64FromParcel
 * @tc.precon: NA
 * @tc.step: 1. Prepare test data with expected int64 value 123456
 *           2. Test with insufficient buffer size (INT32_SIZE)
 *           3. Test with correct buffer size (INT64_SIZE)
 *           4. Verify parsed value matches expected
 *           5. Verify size parameter updated correctly
 * @tc.expect: Returns -1 for insufficient size, 0 for correct size with expected value parsed
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(ResponseMessageReceiverTest, Int64FromParcel001, TestSize.Level1)
{
    int64_t except = 123456; // 123456 is except num
    char *parcel = reinterpret_cast<char *>(&except);
    int64_t num;
    int size = INT32_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::Int64FromParcel(num, parcel, size), -1);
    size = INT64_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::Int64FromParcel(num, parcel, size), 0);
    EXPECT_EQ(num, except);
    EXPECT_EQ(size, 0);
}

/**
 * @tc.name: Uint64FromParcel001
 * @tc.desc: Test Uint64FromParcel interface base function - Uint64FromParcel
 * @tc.precon: NA
 * @tc.step: 1. Prepare test data with expected uint64 value 123456
 *           2. Test with insufficient buffer size (INT32_SIZE)
 *           3. Test with correct buffer size (INT64_SIZE)
 *           4. Verify parsed value matches expected
 *           5. Verify size parameter updated correctly
 * @tc.expect: Returns -1 for insufficient size, 0 for correct size with expected value parsed
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(ResponseMessageReceiverTest, Uint64FromParcel001, TestSize.Level1)
{
    uint64_t except = 123456; // 123456 is except num
    char *parcel = reinterpret_cast<char *>(&except);
    uint64_t num;
    int size = INT32_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::Uint64FromParcel(num, parcel, size), -1);
    size = INT64_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::Uint64FromParcel(num, parcel, size), 0);
    EXPECT_EQ(num, except);
    EXPECT_EQ(size, 0);
}

/**
 * @tc.name: Int32FromParcel001
 * @tc.desc: Test Int32FromParcel interface base function - Int32FromParcel
 * @tc.precon: NA
 * @tc.step: 1. Prepare test data with expected int32 value 123456
 *           2. Test with insufficient buffer size (INT16_SIZE)
 *           3. Test with correct buffer size (INT32_SIZE)
 *           4. Verify parsed value matches expected
 *           5. Verify size parameter updated correctly
 * @tc.expect: Returns -1 for insufficient size, 0 for correct size with expected value parsed
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(ResponseMessageReceiverTest, Int32FromParcel001, TestSize.Level1)
{
    int32_t except = 123456; // 123456 is except num
    char *parcel = reinterpret_cast<char *>(&except);
    int32_t num;
    int size = INT16_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::Int32FromParcel(num, parcel, size), -1);
    size = INT32_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::Int32FromParcel(num, parcel, size), 0);
    EXPECT_EQ(num, except);
    EXPECT_EQ(size, 0);
}

/**
 * @tc.name: Uint32FromParcel001
 * @tc.desc: Test Uint32FromParcel interface base function - Uint32FromParcel
 * @tc.precon: NA
 * @tc.step: 1. Prepare test data with expected uint32 value 123456
 *           2. Test with insufficient buffer size (INT16_SIZE)
 *           3. Test with correct buffer size (INT32_SIZE)
 *           4. Verify parsed value matches expected
 *           5. Verify size parameter updated correctly
 * @tc.expect: Returns -1 for insufficient size, 0 for correct size with expected value parsed
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(ResponseMessageReceiverTest, Uint32FromParcel001, TestSize.Level1)
{
    uint32_t except = 123456; // 123456 is except num
    char *parcel = reinterpret_cast<char *>(&except);
    uint32_t num;
    int size = INT16_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::Uint32FromParcel(num, parcel, size), -1);
    size = INT32_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::Uint32FromParcel(num, parcel, size), 0);
    EXPECT_EQ(num, except);
    EXPECT_EQ(size, 0);
}

/**
 * @tc.name: Int16FromParcel001
 * @tc.desc: Test Int16FromParcel interface base function - Int16FromParcel
 * @tc.precon: NA
 * @tc.step: 1. Prepare test data with expected int16 value 123
 *           2. Test with insufficient buffer size (0)
 *           3. Test with correct buffer size (INT16_SIZE)
 *           4. Verify parsed value matches expected
 *           5. Verify size parameter updated correctly
 * @tc.expect: Returns -1 for insufficient size, 0 for correct size with expected value parsed
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(ResponseMessageReceiverTest, Int16FromParcel001, TestSize.Level1)
{
    int16_t except = 123; // 123 is except num
    char *parcel = reinterpret_cast<char *>(&except);
    int16_t num;
    int size = 0;
    EXPECT_EQ(ResponseMessageReceiver::Int16FromParcel(num, parcel, size), -1);
    size = INT16_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::Int16FromParcel(num, parcel, size), 0);
    EXPECT_EQ(num, except);
    EXPECT_EQ(size, 0);
}

/**
 * @tc.name: StateFromParcel001
 * @tc.desc: Test StateFromParcel interface base function - StateFromParcel
 * @tc.precon: NA
 * @tc.step: 1. Prepare test data with invalid state value (ANY+1)
 *           2. Test with invalid state value
 *           3. Prepare test data with valid state value (ANY)
 *           4. Test with valid state value
 *           5. Verify parsed state matches expected
 * @tc.expect: Returns -1 for invalid state, 0 for valid state with correct value parsed
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(ResponseMessageReceiverTest, StateFromParcel001, TestSize.Level1)
{
    State state;
    uint32_t except = static_cast<uint32_t>(State::ANY) + 1;
    char *parcel = reinterpret_cast<char *>(&except);
    int size = INT32_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::StateFromParcel(state, parcel, size), -1);
    except = static_cast<uint32_t>(State::ANY);
    parcel = reinterpret_cast<char *>(&except);
    size = INT32_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::StateFromParcel(state, parcel, size), 0);
    EXPECT_EQ(state, State::ANY);
    EXPECT_EQ(size, 0);
}

/**
 * @tc.name: ActionFromParcel001
 * @tc.desc: Test ActionFromParcel interface base function - ActionFromParcel
 * @tc.precon: NA
 * @tc.step: 1. Prepare test data with invalid action value (ANY+1)
 *           2. Test with invalid action value
 *           3. Prepare test data with valid action value (ANY)
 *           4. Test with valid action value
 *           5. Verify parsed action matches expected
 * @tc.expect: Returns -1 for invalid action, 0 for valid action with correct value parsed
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(ResponseMessageReceiverTest, ActionFromParcel001, TestSize.Level1)
{
    Action action;
    uint32_t except = static_cast<uint32_t>(Action::ANY) + 1;
    char *parcel = reinterpret_cast<char *>(&except);
    int size = INT32_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::ActionFromParcel(action, parcel, size), -1);
    except = static_cast<uint32_t>(Action::ANY);
    parcel = reinterpret_cast<char *>(&except);
    size = INT32_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::ActionFromParcel(action, parcel, size), 0);
    EXPECT_EQ(action, Action::ANY);
    EXPECT_EQ(size, 0);
}

/**
 * @tc.name: VersionFromParcel001
 * @tc.desc: Test VersionFromParcel interface base function - VersionFromParcel
 * @tc.precon: NA
 * @tc.step: 1. Prepare test data with invalid version value (API10+1)
 *           2. Test with invalid version value
 *           3. Prepare test data with valid version value (API10)
 *           4. Test with valid version value
 *           5. Verify parsed version matches expected
 * @tc.expect: Returns -1 for invalid version, 0 for valid version with correct value parsed
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(ResponseMessageReceiverTest, VersionFromParcel001, TestSize.Level1)
{
    Version version;
    uint32_t except = static_cast<uint32_t>(Version::API10) + 1;
    char *parcel = reinterpret_cast<char *>(&except);
    int size = INT32_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::VersionFromParcel(version, parcel, size), -1);
    except = static_cast<uint32_t>(Version::API10);
    parcel = reinterpret_cast<char *>(&except);
    size = INT32_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::VersionFromParcel(version, parcel, size), 0);
    EXPECT_EQ(version, Version::API10);
    EXPECT_EQ(size, 0);
}

/**
 * @tc.name: SubscribeTypeFromParcel001
 * @tc.desc: Test SubscribeTypeFromParcel interface base function - SubscribeTypeFromParcel
 * @tc.precon: NA
 * @tc.step: 1. Prepare test data with invalid subscribe type value (BUTT+1)
 *           2. Test with invalid subscribe type value
 *           3. Prepare test data with valid subscribe type value (BUTT)
 *           4. Test with valid subscribe type value
 *           5. Verify parsed subscribe type matches expected
 * @tc.expect: Returns -1 for invalid type, 0 for valid type with correct value parsed
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(ResponseMessageReceiverTest, SubscribeTypeFromParcel001, TestSize.Level1)
{
    SubscribeType type;
    uint32_t except = static_cast<uint32_t>(SubscribeType::BUTT) + 1;
    char *parcel = reinterpret_cast<char *>(&except);
    int size = INT32_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::SubscribeTypeFromParcel(type, parcel, size), -1);
    except = static_cast<uint32_t>(SubscribeType::BUTT);
    parcel = reinterpret_cast<char *>(&except);
    size = INT32_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::SubscribeTypeFromParcel(type, parcel, size), 0);
    EXPECT_EQ(type, SubscribeType::BUTT);
    EXPECT_EQ(size, 0);
}

/**
 * @tc.name: StringFromParcel001
 * @tc.desc: Test StringFromParcel interface base function - StringFromParcel
 * @tc.precon: NA
 * @tc.step: 1. Prepare test string "except string"
 *           2. Test with insufficient buffer size (size-1)
 *           3. Test with correct buffer size (size+1)
 *           4. Verify parsed string matches expected
 *           5. Verify size parameter updated correctly
 * @tc.expect: Returns -1 for insufficient size, 0 for correct size with expected string parsed
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(ResponseMessageReceiverTest, StringFromParcel001, TestSize.Level1)
{
    string str;
    string except = "except string";
    char *parcel = const_cast<char *>(except.c_str());
    int size = except.size() - 1;
    EXPECT_EQ(ResponseMessageReceiver::StringFromParcel(str, parcel, size), -1);
    size = except.size() + 1;
    EXPECT_EQ(ResponseMessageReceiver::StringFromParcel(str, parcel, size), 0);
    EXPECT_EQ(str, except);
    EXPECT_EQ(size, 0);
}

/**
 * @tc.name: ResponseHeaderFromParcel001
 * @tc.desc: Test ResponseHeaderFromParcel interface base function - ResponseHeaderFromParcel
 * @tc.precon: NA
 * @tc.step: 1. Prepare test header string "header:aaa,bbb,ccc\n"
 *           2. Create empty headers map
 *           3. Parse header string into map
 *           4. Verify parsing returns success (0)
 *           5. Verify header values are correctly parsed
 * @tc.expect: Header parsed successfully with values ["aaa", "bbb", "ccc"] for key "header"
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(ResponseMessageReceiverTest, ResponseHeaderFromParcel001, TestSize.Level1)
{
    std::map<std::string, std::vector<std::string>> headers;
    string except = "header:aaa,bbb,ccc\n";
    std::vector<std::string> header;
    char *parcel = const_cast<char *>(except.c_str());
    int size = except.size();
    EXPECT_EQ(ResponseMessageReceiver::ResponseHeaderFromParcel(headers, parcel, size), 0);
    header = headers["header"];
    EXPECT_EQ(header[0], "aaa");
    EXPECT_EQ(header[1], "bbb");
    EXPECT_EQ(header[2], "ccc");
}

/**
 * @tc.name: ProgressExtrasFromParcel001
 * @tc.desc: Test ProgressExtrasFromParcel interface base function - ProgressExtrasFromParcel
 * @tc.precon: NA
 * @tc.step: 1. Prepare test data with length and key-value pairs
 *           2. Test with insufficient buffer sizes (INT16_SIZE, INT32_SIZE+1, INT32_SIZE+6)
 *           3. Test with correct buffer size (full array)
 *           4. Verify parsing returns success (0)
 *           5. Verify key-value pairs are correctly parsed
 * @tc.expect: Returns -1 for insufficient sizes, 0 for correct size with key-value pairs parsed
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(ResponseMessageReceiverTest, ProgressExtrasFromParcel001, TestSize.Level1)
{
    int arraySize = 64; // 64 is char array length
    char except[arraySize];
    uint32_t length = 1;
    EXPECT_EQ(memcpy_s(except, static_cast<size_t>(arraySize), reinterpret_cast<void *>(&length), sizeof(length)), 0);
    char keyValue[] = "key\0value\0";
    EXPECT_EQ(memcpy_s(except + sizeof(length), static_cast<size_t>(arraySize - sizeof(length)), keyValue, 9),
        0); // 9 is keyValue length
    std::map<std::string, std::string> extras;
    char *parcel = except;
    int size = INT16_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::ProgressExtrasFromParcel(extras, parcel, size), -1);
    parcel = except;
    size = sizeof(length) + 1;
    EXPECT_EQ(ResponseMessageReceiver::ProgressExtrasFromParcel(extras, parcel, size), -1);
    parcel = except;
    size = sizeof(length) + 6; // 6 make except size between the keyValue
    EXPECT_EQ(ResponseMessageReceiver::ProgressExtrasFromParcel(extras, parcel, size), -1);
    parcel = except;
    size = arraySize;
    EXPECT_EQ(ResponseMessageReceiver::ProgressExtrasFromParcel(extras, parcel, size), 0);
    EXPECT_EQ(extras["key"], "value");
}

/**
 * @tc.name: VecInt64FromParcel001
 * @tc.desc: Test VecInt64FromParcel interface base function - VecInt64FromParcel
 * @tc.precon: NA
 * @tc.step: 1. Prepare test data with length and int64 vector values
 *           2. Test with insufficient buffer sizes (INT16_SIZE, INT64_SIZE)
 *           3. Test with correct buffer size (full array)
 *           4. Verify parsing returns success (0)
 *           5. Verify vector values are correctly parsed
 * @tc.expect: Returns -1 for insufficient sizes, 0 for correct size with vector values parsed
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(ResponseMessageReceiverTest, VecInt64FromParcel001, TestSize.Level1)
{
    int arraySize = INT32_SIZE + INT64_SIZE;
    char except[arraySize];
    uint32_t length = 1;
    EXPECT_EQ(memcpy_s(except, static_cast<size_t>(arraySize), reinterpret_cast<void *>(&length), sizeof(length)), 0);
    int64_t value = 123456; // 123456 is except num
    EXPECT_EQ(memcpy_s(except + sizeof(length), static_cast<size_t>(arraySize - sizeof(length)),
                  reinterpret_cast<void *>(&value), sizeof(value)),
        0);
    std::vector<int64_t> vec;
    char *parcel = except;
    int size = INT16_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::VecInt64FromParcel(vec, parcel, size), -1);
    parcel = except;
    size = INT64_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::VecInt64FromParcel(vec, parcel, size), -1);
    parcel = except;
    size = arraySize; // 6 make except size between the keyValue
    EXPECT_EQ(ResponseMessageReceiver::VecInt64FromParcel(vec, parcel, size), 0);
    EXPECT_EQ(vec[0], value);
    EXPECT_EQ(vec.size(), length);
}

class RMRestResponseListener : public IResponseMessageHandler {
public:
    RMRestResponseListener(){};
    ~RMRestResponseListener(){};
    void OnResponseReceive(const std::shared_ptr<Response> &response) override
    {
    }
    void OnNotifyDataReceive(const std::shared_ptr<NotifyData> &notifyData) override
    {
    }
    void OnFaultsReceive(const std::shared_ptr<int32_t> &tid, const std::shared_ptr<SubscribeType> &type,
        const std::shared_ptr<Reason> &reason) override
    {
    }
    void OnChannelBroken() override
    {
    }
    void OnWaitReceive(std::int32_t taskId, WaitingReason reason) override
    {
    }
};

/**
 * @tc.name: ResponseMessageReceiver001
 * @tc.desc: Test ResponseMessageReceiver constructor with valid parameters
 * @tc.precon: NA
 * @tc.step: 1. Create test response listener handler
 *           2. Set socket file descriptor to -1
 *           3. Create ResponseMessageReceiver instance with handler and socket
 *           4. Verify handler pointer is correctly stored
 * @tc.expect: ResponseMessageReceiver created successfully with handler pointer stored correctly
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(ResponseMessageReceiverTest, ResponseMessageReceiver001, TestSize.Level1)
{
    RMRestResponseListener handler = RMRestResponseListener();
    int32_t sockFd = -1;
    ResponseMessageReceiver receiver = ResponseMessageReceiver(&handler, sockFd);
    EXPECT_EQ(&handler, receiver.handler_);
}

/**
 * @tc.name: MsgHeaderParcel001
 * @tc.desc: Test MsgHeaderParcel interface base function - MsgHeaderParcel
 * @tc.precon: NA
 * @tc.step: 1. Prepare test data with magic number, message ID, type, and body size
 *           2. Test with insufficient buffer sizes (INT16_SIZE, INT32_SIZE, INT64_SIZE)
 *           3. Test with correct buffer size (full array)
 *           4. Verify parsing returns success (0)
 *           5. Verify all header fields are correctly parsed
 * @tc.expect: Returns -1 for insufficient sizes, 0 for correct size with all fields parsed correctly
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(ResponseMessageReceiverTest, MsgHeaderParcel001, TestSize.Level1)
{
    uint32_t magicNum = ResponseMessageReceiver::RESPONSE_MAGIC_NUM - 1;
    int pos = 0;
    int arraySize = INT32_SIZE + INT64_SIZE;
    char except[arraySize];
    EXPECT_EQ(
        memcpy_s(except, static_cast<size_t>(arraySize), reinterpret_cast<void *>(&magicNum), sizeof(magicNum)), 0);
    pos += sizeof(magicNum);
    int32_t msgId = 123456; // 123456 is except num
    EXPECT_EQ(
        memcpy_s(except + pos, static_cast<size_t>(arraySize - pos), reinterpret_cast<void *>(&msgId), sizeof(msgId)),
        0);
    pos += sizeof(msgId);
    int16_t msgType = 123; // 123 is except num
    EXPECT_EQ(memcpy_s(except + pos, static_cast<size_t>(arraySize - pos), reinterpret_cast<void *>(&msgType),
                  sizeof(msgType)),
        0);
    pos += sizeof(msgType);
    int16_t bodySize = 456; // 456 is except num
    EXPECT_EQ(memcpy_s(except + pos, static_cast<size_t>(arraySize - pos), reinterpret_cast<void *>(&bodySize),
                  sizeof(bodySize)),
        0);
    pos += sizeof(bodySize);
    msgId = 0;
    msgType = 0;
    bodySize = 0;
    char *parcel = except;
    int size = INT16_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::MsgHeaderParcel(msgId, msgType, bodySize, parcel, size), -1);
    parcel = except;
    size = INT32_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::MsgHeaderParcel(msgId, msgType, bodySize, parcel, size), -1);
    parcel = except;
    size = INT32_SIZE;
    magicNum = ResponseMessageReceiver::RESPONSE_MAGIC_NUM;
    EXPECT_EQ(
        memcpy_s(except, static_cast<size_t>(arraySize), reinterpret_cast<void *>(&magicNum), sizeof(magicNum)), 0);
    EXPECT_EQ(ResponseMessageReceiver::MsgHeaderParcel(msgId, msgType, bodySize, parcel, size), -1);
    parcel = except;
    size = INT32_SIZE + INT16_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::MsgHeaderParcel(msgId, msgType, bodySize, parcel, size), -1);
    parcel = except;
    size = INT64_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::MsgHeaderParcel(msgId, msgType, bodySize, parcel, size), -1);
    EXPECT_EQ(msgId, 123456); // 123456 is except num
    parcel = except;
    size = INT64_SIZE + INT16_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::MsgHeaderParcel(msgId, msgType, bodySize, parcel, size), -1);
    EXPECT_EQ(msgType, 123); // 123 is except num
    parcel = except;
    size = arraySize;
    EXPECT_EQ(ResponseMessageReceiver::MsgHeaderParcel(msgId, msgType, bodySize, parcel, size), 0);
    EXPECT_EQ(bodySize, 456); // 456 is except num
}

/**
 * @tc.name: ResponseFromParcel001
 * @tc.desc: Test ResponseFromParcel interface base function - ResponseFromParcel
 * @tc.precon: NA
 * @tc.step: 1. Prepare test response data with task ID, version, status code, reason, and headers
 *           2. Test with insufficient buffer sizes (INT16_SIZE, INT32_SIZE, INT64_SIZE)
 *           3. Test with correct buffer size (full array)
 *           4. Verify parsing returns success (0)
 *           5. Verify all response fields are correctly parsed
 * @tc.expect: Returns -1 for insufficient sizes, 0 for correct size with all response fields parsed correctly
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(ResponseMessageReceiverTest, ResponseFromParcel001, TestSize.Level1)
{
    std::shared_ptr<Response> response = std::make_shared<Response>();
    int pos = 0;
    int32_t tid = 123; // 123 is except tid
    string version = "version";
    int32_t statusCode = 456; // 456 is except statusCode
    string reason = "reason";
    string headers = "header:aaa,bbb,ccc\n";
    char except[ARRAY_LEN];
    EXPECT_EQ(memcpy_s(except, static_cast<size_t>(ARRAY_LEN), reinterpret_cast<void *>(&tid), sizeof(tid)), 0);
    pos += sizeof(tid);
    EXPECT_EQ(memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), version.c_str(), version.size() + 1), 0);
    pos += (version.size() + 1);
    EXPECT_EQ(memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&statusCode),
                  sizeof(statusCode)),
        0);
    pos += sizeof(statusCode);
    EXPECT_EQ(memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), reason.c_str(), reason.size() + 1), 0);
    pos += (reason.size() + 1);
    EXPECT_EQ(memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), headers.c_str(), headers.size() + 1), 0);
    pos += (headers.size() + 1);
    char *parcel = except;
    int size = INT16_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::ResponseFromParcel(response, parcel, size), -1);
    parcel = except;
    size = INT32_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::ResponseFromParcel(response, parcel, size), -1);
    EXPECT_EQ(response->taskId, "123");
    parcel = except;
    size = INT32_SIZE + version.size() + 1;
    EXPECT_EQ(ResponseMessageReceiver::ResponseFromParcel(response, parcel, size), -1);
    EXPECT_EQ(response->version, version);
    parcel = except;
    size = INT64_SIZE + version.size() + 1;
    EXPECT_EQ(ResponseMessageReceiver::ResponseFromParcel(response, parcel, size), -1);
    EXPECT_EQ(response->statusCode, statusCode);
    parcel = except;
    size = ARRAY_LEN;
    EXPECT_EQ(ResponseMessageReceiver::ResponseFromParcel(response, parcel, size), 0);
    EXPECT_EQ(response->reason, reason);
    auto header = response->headers["header"];
    EXPECT_EQ(header[0], "aaa");
    EXPECT_EQ(header[1], "bbb");
    EXPECT_EQ(header[2], "ccc");
}

/**
 * @tc.name: TaskStatesFromParcel001
 * @tc.desc: Test TaskStatesFromParcel interface base function - TaskStatesFromParcel
 * @tc.precon: NA
 * @tc.step: 1. Prepare test task states data with length, path, response code, and message
 *           2. Test with insufficient buffer sizes (INT16_SIZE, INT32_SIZE, INT64_SIZE)
 *           3. Test with correct buffer size (full array)
 *           4. Verify parsing returns success (0)
 *           5. Verify all task state fields are correctly parsed
 * @tc.expect: Returns -1 for insufficient sizes, 0 for correct size with all task state fields parsed correctly
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(ResponseMessageReceiverTest, TaskStatesFromParcel001, TestSize.Level1)
{
    std::vector<TaskState> taskStates;
    int pos = 0;
    int32_t length = 1;
    string path = "path";
    int32_t responseCode = NETWORK_OFFLINE;
    string message = "message";
    char except[ARRAY_LEN];
    EXPECT_EQ(memcpy_s(except, static_cast<size_t>(ARRAY_LEN), reinterpret_cast<void *>(&length), sizeof(length)), 0);
    pos += sizeof(length);
    EXPECT_EQ(memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), path.c_str(), path.size() + 1), 0);
    pos += (path.size() + 1);
    EXPECT_EQ(memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&responseCode),
                  sizeof(responseCode)),
        0);
    pos += sizeof(responseCode);
    EXPECT_EQ(memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), message.c_str(), message.size() + 1), 0);
    pos += (message.size() + 1);
    char *parcel = except;
    int size = INT16_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::TaskStatesFromParcel(taskStates, parcel, size), -1);
    parcel = except;
    size = INT32_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::TaskStatesFromParcel(taskStates, parcel, size), -1);
    parcel = except;
    size = INT32_SIZE + path.size() + 1;
    EXPECT_EQ(ResponseMessageReceiver::TaskStatesFromParcel(taskStates, parcel, size), -1);
    parcel = except;
    size = INT64_SIZE + path.size() + 1;
    EXPECT_EQ(ResponseMessageReceiver::TaskStatesFromParcel(taskStates, parcel, size), -1);
    parcel = except;
    size = ARRAY_LEN;
    EXPECT_EQ(ResponseMessageReceiver::TaskStatesFromParcel(taskStates, parcel, size), 0);
    EXPECT_EQ(taskStates[0].message, message);
    EXPECT_EQ(taskStates[0].path, path);
    EXPECT_EQ(taskStates[0].responseCode, responseCode);
}

/**
 * @tc.name: NotifyDataFromParcel001
 * @tc.desc: Test NotifyDataFromParcel interface base function - NotifyDataFromParcel
 * @tc.precon: NA
 * @tc.step: 1. Prepare comprehensive notify data with all fields including progress, extras, and task states
 *           2. Test with various insufficient buffer sizes incrementally
 *           3. Test with correct buffer size (full array)
 *           4. Verify parsing returns success (0)
 *           5. Verify all notify data fields including nested structures are correctly parsed
 * @tc.expect: Returns -1 for insufficient sizes, 0 for correct size with all notify data fields parsed correctly
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(ResponseMessageReceiverTest, NotifyDataFromParcel001, TestSize.Level1)
{
    std::shared_ptr<NotifyData> notifyData = std::make_shared<NotifyData>();
    int pos = 0;
    int32_t length = 1;
    SubscribeType type = SubscribeType::BUTT;
    uint32_t taskId = 123; // 123 is except tid
    State state = State::ANY;
    uint32_t index = 456;             // 456 is except index
    uint64_t processed = 123456;      // 123456 is except processed
    uint64_t totalProcessed = 111222; // 111222 is except totalProcessed
    int64_t value = 333444;           // 333444 is except num
    int ketValueLen = 10;             //9 is keyValue length
    char keyValue[] = "key\0value\0";
    Action action = Action::UPLOAD;
    Version version = Version::API10;
    string path = "path";
    int32_t responseCode = NETWORK_OFFLINE;
    string message = "message";
    char except[ARRAY_LEN];
    EXPECT_EQ(memcpy_s(except, static_cast<size_t>(ARRAY_LEN), reinterpret_cast<void *>(&type), sizeof(type)), 0);
    pos += sizeof(type);
    EXPECT_EQ(
        memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&taskId), sizeof(taskId)),
        0);
    pos += sizeof(taskId);
    EXPECT_EQ(
        memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&state), sizeof(state)),
        0);
    pos += sizeof(state);
    EXPECT_EQ(
        memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&index), sizeof(index)),
        0);
    pos += sizeof(index);
    EXPECT_EQ(memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&processed),
                  sizeof(processed)),
        0);
    pos += sizeof(processed);
    EXPECT_EQ(memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&totalProcessed),
                  sizeof(totalProcessed)),
        0);
    pos += sizeof(totalProcessed);
    EXPECT_EQ(
        memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&length), sizeof(length)),
        0);
    pos += sizeof(length);
    EXPECT_EQ(
        memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&value), sizeof(value)),
        0);
    pos += sizeof(value);
    EXPECT_EQ(
        memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&length), sizeof(length)),
        0);
    pos += sizeof(length);
    EXPECT_EQ(memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), keyValue, ketValueLen), 0);
    pos += ketValueLen;
    EXPECT_EQ(
        memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&action), sizeof(action)),
        0);
    pos += sizeof(action);
    EXPECT_EQ(memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&version),
                  sizeof(version)),
        0);
    pos += sizeof(version);
    EXPECT_EQ(
        memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&length), sizeof(length)),
        0);
    pos += sizeof(length);
    EXPECT_EQ(memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), path.c_str(), path.size() + 1), 0);
    pos += (path.size() + 1);
    EXPECT_EQ(memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), reinterpret_cast<void *>(&responseCode),
                  sizeof(responseCode)),
        0);
    pos += sizeof(responseCode);
    EXPECT_EQ(memcpy_s(except + pos, static_cast<size_t>(ARRAY_LEN - pos), message.c_str(), message.size() + 1), 0);
    pos += (message.size() + 1);
    char *parcel = except;
    int size = INT16_SIZE;
    EXPECT_EQ(ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, size), -1);
    parcel = except;
    size = INT32_SIZE;
    int maxLen = size;
    EXPECT_EQ(ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, size), -1);
    parcel = except;
    maxLen += INT32_SIZE;
    size = maxLen;
    EXPECT_EQ(ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, size), -1);
    parcel = except;
    maxLen += INT32_SIZE;
    size = maxLen;
    EXPECT_EQ(ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, size), -1);
    parcel = except;
    maxLen += INT32_SIZE;
    size = maxLen;
    EXPECT_EQ(ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, size), -1);
    parcel = except;
    maxLen += INT64_SIZE;
    size = maxLen;
    EXPECT_EQ(ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, size), -1);
    parcel = except;
    maxLen += INT64_SIZE;
    size = maxLen;
    EXPECT_EQ(ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, size), -1);
    parcel = except;
    maxLen += (sizeof(length) + sizeof(value));
    size = maxLen;
    EXPECT_EQ(ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, size), -1);
    parcel = except;
    maxLen += (sizeof(length) + ketValueLen);
    size = maxLen;
    EXPECT_EQ(ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, size), -1);
    parcel = except;
    maxLen += INT32_SIZE;
    size = maxLen;
    EXPECT_EQ(ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, size), -1);
    parcel = except;
    maxLen += INT32_SIZE;
    size = maxLen;
    EXPECT_EQ(ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, size), -1);
    parcel = except;
    size = ARRAY_LEN;
    EXPECT_EQ(ResponseMessageReceiver::NotifyDataFromParcel(notifyData, parcel, size), 0);
    EXPECT_EQ(notifyData->type, type);
    EXPECT_EQ(notifyData->taskId, taskId);
    EXPECT_EQ(notifyData->progress.state, state);
    EXPECT_EQ(notifyData->progress.processed, processed);
    EXPECT_EQ(notifyData->progress.totalProcessed, totalProcessed);
    EXPECT_EQ(notifyData->progress.sizes[0], value);
    EXPECT_EQ(notifyData->progress.extras["key"], "value");
    EXPECT_EQ(notifyData->action, action);
    EXPECT_EQ(notifyData->version, version);
    EXPECT_EQ(notifyData->taskStates[0].message, message);
    EXPECT_EQ(notifyData->taskStates[0].path, path);
    EXPECT_EQ(notifyData->taskStates[0].responseCode, responseCode);
}

/**
 * @tc.name: OnReadable001
 * @tc.desc: Test OnReadable interface base function - OnReadable
 * @tc.precon: NA
 * @tc.step: 1. Create test response listener handler
 *           2. Set socket file descriptor to -1
 *           3. Create ResponseMessageReceiver instance
 *           4. Call OnReadable method
 * @tc.expect: OnReadable method executes without crashing
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(ResponseMessageReceiverTest, OnReadable001, TestSize.Level1)
{
    int32_t fd[2]; // 2 socket for socketpair
    fd[0] = -1;
    RMRestResponseListener handler = RMRestResponseListener();
    int32_t sockFd = -1;
    ResponseMessageReceiver receiver = ResponseMessageReceiver(&handler, sockFd);
    receiver.OnReadable(fd[0]);
    uint32_t magicNum = ResponseMessageReceiver::RESPONSE_MAGIC_NUM - 1;
    int pos = 0;
    int arraySize = INT32_SIZE + INT64_SIZE;
    char except[arraySize];
    EXPECT_EQ(
        memcpy_s(except, static_cast<size_t>(arraySize), reinterpret_cast<void *>(&magicNum), sizeof(magicNum)), 0);
    pos += sizeof(magicNum);
    int32_t msgId = 123456; // 123456 is except num
    EXPECT_EQ(
        memcpy_s(except + pos, static_cast<size_t>(arraySize - pos), reinterpret_cast<void *>(&msgId), sizeof(msgId)),
        0);
    pos += sizeof(msgId);
    int16_t msgType = 123; // 123 is except num
    EXPECT_EQ(memcpy_s(except + pos, static_cast<size_t>(arraySize - pos), reinterpret_cast<void *>(&msgType),
                  sizeof(msgType)),
        0);
    int msgTypePos = pos;
    pos += sizeof(msgType);
    int16_t bodySize = 456; // 456 is except num
    EXPECT_EQ(memcpy_s(except + pos, static_cast<size_t>(arraySize - pos), reinterpret_cast<void *>(&bodySize),
                  sizeof(bodySize)),
        0);
    pos += sizeof(bodySize);
    EXPECT_TRUE(socketpair(AF_UNIX, SOCK_STREAM, 0, fd) >= 0);
    EXPECT_TRUE(write(fd[1], except, pos + 1) > 0);
    receiver.OnReadable(fd[0]);
    msgType = MessageType::HTTP_RESPONSE;
    EXPECT_EQ(memcpy_s(except + msgTypePos, static_cast<size_t>(arraySize - msgTypePos),
                  reinterpret_cast<void *>(&msgType), sizeof(msgType)),
        0);
    EXPECT_TRUE(write(fd[1], except, pos + 1) > 0);
    receiver.OnReadable(fd[0]);
    msgType = MessageType::NOTIFY_DATA;
    EXPECT_EQ(memcpy_s(except + msgTypePos, static_cast<size_t>(arraySize - msgTypePos),
                  reinterpret_cast<void *>(&msgType), sizeof(msgType)),
        0);
    EXPECT_TRUE(write(fd[1], except, pos + 1) > 0);
    receiver.messageId_ = msgId;
    receiver.OnReadable(fd[0]);
    EXPECT_EQ(close(fd[0]), 0);
    EXPECT_EQ(close(fd[1]), 0);
}