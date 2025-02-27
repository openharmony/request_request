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

#include <cstdint>
#include <cstring>

#define private public
#define protected public
#include <gtest/gtest.h>

#include "gmock/gmock.h"
#include "log.h"
#include "parcel_helper.h"
#include "request_common.h"
#include "request_common_utils.h"

using namespace testing::ext;
using namespace OHOS::Request;

#undef private
#undef protected

class ParcelHelperTest : public testing::Test {
public:
    static void SetUpTestCase(void);
    static void TearDownTestCase(void);
    void SetUp();
    void TearDown();
};

void ParcelHelperTest::SetUpTestCase(void)
{
    // input testSuit setup step，setup invoked before all testCases
}

void ParcelHelperTest::TearDownTestCase(void)
{
    // input testSuit teardown step，teardown invoked after all testCases
}

void ParcelHelperTest::SetUp(void)
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

void ParcelHelperTest::TearDown(void)
{
    // input testCase teardown step，teardown invoked after each testCase
}

/**
 * @tc.name: UnMarshalFormItem001
 * @tc.desc: Test UnMarshalFormItem001 interface base function - UnMarshalFormItem
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshalFormItem001, TestSize.Level1)
{
    OHOS::MessageParcel data;
    TaskInfo info;
    uint32_t size = 0;
    data.WriteUint32(size);
    EXPECT_TRUE(ParcelHelper::UnMarshalFormItem(data, info));
    size = 1;
    data.WriteUint32(size);
    EXPECT_FALSE(ParcelHelper::UnMarshalFormItem(data, info));
    data.WriteUint32(size);
    data.WriteString("name");
    data.WriteString("value");
    EXPECT_TRUE(ParcelHelper::UnMarshalFormItem(data, info));
    EXPECT_EQ(info.forms[0].name, "name");
    EXPECT_EQ(info.forms[0].value, "value");
}

/**
 * @tc.name: UnMarshalFileSpec001
 * @tc.desc: Test UnMarshalFileSpec001 interface base function - UnMarshalFileSpec
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshalFileSpec001, TestSize.Level1)
{
    OHOS::MessageParcel data;
    TaskInfo info;
    uint32_t size = 0;
    data.WriteUint32(size);
    EXPECT_TRUE(ParcelHelper::UnMarshalFileSpec(data, info));
    size = 1;
    data.WriteUint32(size);
    EXPECT_FALSE(ParcelHelper::UnMarshalFileSpec(data, info));
    data.WriteUint32(size);
    data.WriteString("name");
    data.WriteString("uri");
    data.WriteString("filename");
    data.WriteString("type");
    EXPECT_TRUE(ParcelHelper::UnMarshalFileSpec(data, info));
    EXPECT_EQ(info.files[0].name, "name");
    EXPECT_EQ(info.files[0].uri, "uri");
    EXPECT_EQ(info.files[0].filename, "filename");
    EXPECT_EQ(info.files[0].type, "type");
}

/**
 * @tc.name: UnMarshalMapProgressExtras001
 * @tc.desc: Test UnMarshalMapProgressExtras001 interface base function - UnMarshalMapProgressExtras
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshalMapProgressExtras001, TestSize.Level1)
{
    OHOS::MessageParcel data;
    TaskInfo info;
    uint32_t size = 0;
    data.WriteUint32(size);
    EXPECT_TRUE(ParcelHelper::UnMarshalMapProgressExtras(data, info.progress));
    size = 1;
    data.WriteUint32(size);
    EXPECT_FALSE(ParcelHelper::UnMarshalMapProgressExtras(data, info.progress));
    data.WriteUint32(size);
    data.WriteString("key");
    data.WriteString("value");
    EXPECT_TRUE(ParcelHelper::UnMarshalMapProgressExtras(data, info.progress));
    EXPECT_EQ(info.progress.extras["key"], "value");
}

/**
 * @tc.name: UnMarshalMapExtras001
 * @tc.desc: Test UnMarshalMapExtras001 interface base function - UnMarshalMapExtras
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshalMapExtras001, TestSize.Level1)
{
    OHOS::MessageParcel data;
    TaskInfo info;
    uint32_t size = 0;
    data.WriteUint32(size);
    EXPECT_TRUE(ParcelHelper::UnMarshalMapExtras(data, info));
    size = 1;
    data.WriteUint32(size);
    EXPECT_FALSE(ParcelHelper::UnMarshalMapExtras(data, info));
    data.WriteUint32(size);
    data.WriteString("key");
    data.WriteString("value");
    EXPECT_TRUE(ParcelHelper::UnMarshalMapExtras(data, info));
    EXPECT_EQ(info.extras["key"], "value");
}

/**
 * @tc.name: UnMarshalTaskState001
 * @tc.desc: Test UnMarshalTaskState001 interface base function - UnMarshalTaskState
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshalTaskState001, TestSize.Level1)
{
    OHOS::MessageParcel data;
    TaskInfo info;
    uint32_t size = 0;
    data.WriteUint32(size);
    EXPECT_TRUE(ParcelHelper::UnMarshalTaskState(data, info));
    size = 1;
    uint32_t responseCode = 0;
    data.WriteUint32(size);
    EXPECT_FALSE(ParcelHelper::UnMarshalTaskState(data, info));
    data.WriteUint32(size);
    data.WriteString("path");
    data.WriteUint32(responseCode);
    data.WriteString("message");
    EXPECT_TRUE(ParcelHelper::UnMarshalTaskState(data, info));
    EXPECT_EQ(info.taskStates[0].path, "path");
    EXPECT_EQ(info.taskStates[0].responseCode, responseCode);
    EXPECT_EQ(info.taskStates[0].message, "message");
}

/**
 * @tc.name: UnMarshalConfigHeaders001
 * @tc.desc: Test UnMarshalConfigHeaders001 interface base function - UnMarshalConfigHeaders
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshalConfigHeaders001, TestSize.Level1)
{
    OHOS::MessageParcel data;
    Config config;
    uint32_t size = 0;
    data.WriteUint32(size);
    EXPECT_TRUE(ParcelHelper::UnMarshalConfigHeaders(data, config));
    size = 1;
    data.WriteUint32(size);
    EXPECT_FALSE(ParcelHelper::UnMarshalConfigHeaders(data, config));
    data.WriteUint32(size);
    data.WriteString("key");
    data.WriteString("value");
    EXPECT_TRUE(ParcelHelper::UnMarshalConfigHeaders(data, config));
    EXPECT_EQ(config.headers["key"], "value");
}

/**
 * @tc.name: UnMarshalConfigExtras001
 * @tc.desc: Test UnMarshalConfigExtras001 interface base function - UnMarshalConfigExtras
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshalConfigExtras001, TestSize.Level1)
{
    OHOS::MessageParcel data;
    Config config;
    uint32_t size = 0;
    data.WriteUint32(size);
    EXPECT_TRUE(ParcelHelper::UnMarshalConfigExtras(data, config));
    size = 1;
    data.WriteUint32(size);
    EXPECT_FALSE(ParcelHelper::UnMarshalConfigExtras(data, config));
    data.WriteUint32(size);
    data.WriteString("key");
    data.WriteString("value");
    EXPECT_TRUE(ParcelHelper::UnMarshalConfigExtras(data, config));
    EXPECT_EQ(config.extras["key"], "value");
}

/**
 * @tc.name: UnMarshalConfigFormItem001
 * @tc.desc: Test UnMarshalConfigFormItem001 interface base function - UnMarshalConfigFormItem
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshalConfigFormItem001, TestSize.Level1)
{
    OHOS::MessageParcel data;
    Config config;
    uint32_t size = 0;
    data.WriteUint32(size);
    EXPECT_TRUE(ParcelHelper::UnMarshalConfigFormItem(data, config));
    size = 1;
    data.WriteUint32(size);
    EXPECT_FALSE(ParcelHelper::UnMarshalConfigFormItem(data, config));
    data.WriteUint32(size);
    data.WriteString("name");
    data.WriteString("value");
    EXPECT_TRUE(ParcelHelper::UnMarshalConfigFormItem(data, config));
    EXPECT_EQ(config.forms[0].name, "name");
    EXPECT_EQ(config.forms[0].value, "value");
}

/**
 * @tc.name: UnMarshalConfigFileSpec001
 * @tc.desc: Test UnMarshalConfigFileSpec001 interface base function - UnMarshalConfigFileSpec
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshalConfigFileSpec001, TestSize.Level1)
{
    OHOS::MessageParcel data;
    Config config;
    uint32_t size = 0;
    data.WriteUint32(size);
    EXPECT_TRUE(ParcelHelper::UnMarshalConfigFileSpec(data, config));
    size = 1;
    data.WriteUint32(size);
    EXPECT_FALSE(ParcelHelper::UnMarshalConfigFileSpec(data, config));
    data.WriteUint32(size);
    data.WriteString("name");
    data.WriteString("uri");
    data.WriteString("filename");
    data.WriteString("type");
    EXPECT_TRUE(ParcelHelper::UnMarshalConfigFileSpec(data, config));
    EXPECT_EQ(config.files[0].name, "name");
    EXPECT_EQ(config.files[0].uri, "uri");
    EXPECT_EQ(config.files[0].filename, "filename");
    EXPECT_EQ(config.files[0].type, "type");
}

/**
 * @tc.name: UnMarshalConfigBodyFileName001
 * @tc.desc: Test UnMarshalConfigBodyFileName001 interface base function - UnMarshalConfigBodyFileName
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshalConfigBodyFileName001, TestSize.Level1)
{
    OHOS::MessageParcel data;
    Config config;
    uint32_t size = 0;
    data.WriteUint32(size);
    EXPECT_TRUE(ParcelHelper::UnMarshalConfigBodyFileName(data, config));
    size = 1;
    data.WriteUint32(size);
    EXPECT_FALSE(ParcelHelper::UnMarshalConfigBodyFileName(data, config));
    data.WriteUint32(size);
    data.WriteString("name");
    EXPECT_TRUE(ParcelHelper::UnMarshalConfigBodyFileName(data, config));
    EXPECT_EQ(config.bodyFileNames[0], "name");
}

void MarshalBase(OHOS::MessageParcel &data)
{
    TaskInfo info;
    data.WriteBool(info.gauge);
    data.WriteBool(info.retry);
    data.WriteUint32(static_cast<uint32_t>(info.action));
    data.WriteUint32(static_cast<uint32_t>(info.mode));
    data.WriteUint32(info.code);
    data.WriteUint32(info.tries);
    data.WriteString("uid");
    data.WriteString("bundle");
    data.WriteString(info.url);
    data.WriteString("tid");
    data.WriteString(info.title);
    data.WriteString("mimeType");
    data.WriteUint64(info.ctime);
    data.WriteUint64(info.mtime);
    data.WriteString(info.data);
    data.WriteString(info.description);
    data.WriteUint32(info.priority);
}

void MarshalProgress(OHOS::MessageParcel &data)
{
    State state = State::DEFAULT;
    uint32_t index = 0;
    uint64_t progress = 0;
    uint64_t totalProgress = 0;
    std::vector<int64_t> val;
    val.push_back(1);
    data.WriteUint32(static_cast<uint32_t>(state));
    data.WriteUint32(index);
    data.WriteUint64(progress);
    data.WriteUint64(totalProgress);
    data.WriteInt64Vector(val);
}

/**
 * @tc.name: UnMarshal001
 * @tc.desc: Test UnMarshal001 interface base function - UnMarshal
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshal001, TestSize.Level1)
{
    OHOS::MessageParcel data;
    TaskInfo info;
    Version version = Version::API10;

    MarshalBase(data);
    uint32_t formItemSize = 1;
    data.WriteUint32(formItemSize);
    ParcelHelper::UnMarshal(data, info);

    MarshalBase(data);
    formItemSize = 0;
    uint32_t fileSpecSize = 1;
    data.WriteUint32(formItemSize);
    data.WriteUint32(fileSpecSize);
    ParcelHelper::UnMarshal(data, info);

    MarshalBase(data);
    fileSpecSize = 0;
    data.WriteUint32(formItemSize);
    data.WriteUint32(fileSpecSize);
    MarshalProgress(data);
    uint32_t progressExtrasSize = 1;
    data.WriteUint32(progressExtrasSize);
    ParcelHelper::UnMarshal(data, info);

    MarshalBase(data);
    data.WriteUint32(formItemSize);
    data.WriteUint32(fileSpecSize);
    MarshalProgress(data);
    progressExtrasSize = 0;
    uint32_t mapExtrasSize = 1;
    data.WriteUint32(progressExtrasSize);
    data.WriteUint32(mapExtrasSize);
    ParcelHelper::UnMarshal(data, info);

    MarshalBase(data);
    data.WriteUint32(formItemSize);
    data.WriteUint32(fileSpecSize);
    MarshalProgress(data);
    mapExtrasSize = 0;
    uint32_t taskStateSize = 1;
    data.WriteUint32(progressExtrasSize);
    data.WriteUint32(mapExtrasSize);
    data.WriteUint32(static_cast<uint32_t>(version));
    data.WriteUint32(taskStateSize);
    ParcelHelper::UnMarshal(data, info);

    MarshalBase(data);
    data.WriteUint32(formItemSize);
    data.WriteUint32(fileSpecSize);
    MarshalProgress(data);
    taskStateSize = 0;
    data.WriteUint32(progressExtrasSize);
    data.WriteUint32(mapExtrasSize);
    data.WriteUint32(static_cast<uint32_t>(version));
    data.WriteUint32(taskStateSize);
    ParcelHelper::UnMarshal(data, info);

    EXPECT_EQ(info.version, Version::API10);
    EXPECT_EQ(info.uid, "uid");
    EXPECT_EQ(info.bundle, "bundle");
    EXPECT_EQ(info.tid, "tid");
    EXPECT_EQ(info.mimeType, "mimeType");
    EXPECT_EQ(info.progress.sizes.size(), 1);
    EXPECT_EQ(info.progress.sizes[0], 1);
}

/**
 * @tc.name: UnMarshalTaskProgress001
 * @tc.desc: Test UnMarshalTaskProgress001 interface base function - UnMarshalTaskProgress001
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshalTaskProgress001, TestSize.Level1)
{
    OHOS::MessageParcel data;
    data.WriteString("tid");
    MarshalProgress(data);
    uint32_t progressExtrasSize = 0;
    data.WriteUint32(progressExtrasSize);
    data.WriteUint32(static_cast<uint32_t>(Reason::REASON_OK));
    data.WriteUint32(200);
    TaskProgress taskProgress;
    ParcelHelper::UnMarshalTaskProgress(data, taskProgress);
    EXPECT_EQ(taskProgress.tid, "tid");
    EXPECT_EQ(taskProgress.progress.sizes.size(), 1);
    EXPECT_EQ(taskProgress.code, Reason::REASON_OK);
    EXPECT_EQ(taskProgress.statusCode, 200);
    EXPECT_EQ(static_cast<uint32_t>(taskProgress.faults), 0);
    EXPECT_EQ(taskProgress.reason, "");

    data.WriteString("tid");
    MarshalProgress(data);
    data.WriteUint32(progressExtrasSize);
    data.WriteUint32(static_cast<uint32_t>(Reason::IO_ERROR));
    data.WriteUint32(200);
    ParcelHelper::UnMarshalTaskProgress(data, taskProgress);
    EXPECT_EQ(taskProgress.tid, "tid");
    EXPECT_EQ(taskProgress.progress.sizes.size(), 1);
    EXPECT_EQ(taskProgress.code, Reason::IO_ERROR);
    EXPECT_EQ(taskProgress.statusCode, 200);
    EXPECT_EQ(taskProgress.faults, Faults::FSIO);
    EXPECT_EQ(taskProgress.reason, "Io Error");
}

void MarshalConfigBase(OHOS::MessageParcel &data)
{
    Config config;
    data.WriteUint32(static_cast<uint32_t>(config.action));
    data.WriteUint32(static_cast<uint32_t>(config.mode));
    data.WriteUint32(config.bundleType);
    data.WriteBool(config.overwrite);
    data.WriteUint32(static_cast<uint32_t>(config.network));
    config.metered = data.WriteBool(config.metered);
    data.WriteBool(config.roaming);
    data.WriteBool(config.retry);
    data.WriteBool(config.redirect);
    data.WriteUint32(config.index);
    data.WriteInt64(config.begins);
    data.WriteInt64(config.ends);
    data.WriteBool(config.gauge);
    data.WriteBool(config.precise);
    data.WriteUint32(config.priority);
    data.WriteBool(config.background);
    data.WriteBool(config.multipart);
    data.WriteString("bundleName");
    data.WriteString("url");
    data.WriteString("title");
    data.WriteString("description");
    data.WriteString("method");
}

/**
 * @tc.name: UnMarshalConfig001
 * @tc.desc: Test UnMarshalConfig001 interface base function - UnMarshalConfig
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, UnMarshalConfig001, TestSize.Level1)
{
    Config config;
    OHOS::MessageParcel data;
    Version version = Version::API10;

    MarshalConfigBase(data);
    uint32_t configHeadersSize = 1;
    data.WriteUint32(configHeadersSize);
    ParcelHelper::UnMarshalConfig(data, config);

    MarshalConfigBase(data);
    configHeadersSize = 0;
    data.WriteUint32(configHeadersSize);
    data.WriteString("data");
    data.WriteString("token");
    uint32_t configExtrasSize = 1;
    data.WriteUint32(configExtrasSize);
    ParcelHelper::UnMarshalConfig(data, config);

    MarshalConfigBase(data);
    configExtrasSize = 0;
    data.WriteUint32(configHeadersSize);
    data.WriteString("data");
    data.WriteString("token");
    data.WriteUint32(configExtrasSize);
    data.WriteUint32(static_cast<uint32_t>(version));
    uint32_t configFormItemSize = 1;
    data.WriteUint32(configFormItemSize);
    ParcelHelper::UnMarshalConfig(data, config);

    MarshalConfigBase(data);
    data.WriteUint32(configHeadersSize);
    data.WriteString("data");
    data.WriteString("token");
    data.WriteUint32(configExtrasSize);
    data.WriteUint32(static_cast<uint32_t>(version));
    configFormItemSize = 0;
    data.WriteUint32(configFormItemSize);
    uint32_t configFileSpecSize = 1;
    data.WriteUint32(configFileSpecSize);
    ParcelHelper::UnMarshalConfig(data, config);

    MarshalConfigBase(data);
    data.WriteUint32(configHeadersSize);
    data.WriteString("data");
    data.WriteString("token");
    data.WriteUint32(configExtrasSize);
    data.WriteUint32(static_cast<uint32_t>(version));
    data.WriteUint32(configFormItemSize);
    configFileSpecSize = 0;
    data.WriteUint32(configFileSpecSize);
    uint32_t configBodyFileNameSize = 1;
    data.WriteUint32(configBodyFileNameSize);
    ParcelHelper::UnMarshalConfig(data, config);

    MarshalConfigBase(data);
    data.WriteUint32(configHeadersSize);
    data.WriteString("data");
    data.WriteString("token");
    data.WriteUint32(configExtrasSize);
    data.WriteUint32(static_cast<uint32_t>(version));
    data.WriteUint32(configFormItemSize);
    data.WriteUint32(configFileSpecSize);
    configBodyFileNameSize = 0;
    data.WriteUint32(configBodyFileNameSize);
    ParcelHelper::UnMarshalConfig(data, config);

    EXPECT_EQ(config.version, Version::API10);
    EXPECT_EQ(config.bundleName, "bundleName");
    EXPECT_EQ(config.url, "url");
    EXPECT_EQ(config.title, "title");
    EXPECT_EQ(config.description, "description");
    EXPECT_EQ(config.method, "method");
    EXPECT_EQ(config.data, "data");
    EXPECT_EQ(config.token, "token");
}

/**
 * @tc.name: GetFaultByReason001
 * @tc.desc: Test GetFaultByReason001 interface base function - GetFaultByReason
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, CommonUtilsGetFaultByReason001, TestSize.Level1)
{
    EXPECT_EQ(CommonUtils::GetFaultByReason(REASON_OK), Faults::OTHERS);
    EXPECT_EQ(CommonUtils::GetFaultByReason(TASK_SURVIVAL_ONE_MONTH), Faults::OTHERS);
    EXPECT_EQ(CommonUtils::GetFaultByReason(WAITTING_NETWORK_ONE_DAY), Faults::OTHERS);
    EXPECT_EQ(CommonUtils::GetFaultByReason(STOPPED_NEW_FRONT_TASK), Faults::OTHERS);
    EXPECT_EQ(CommonUtils::GetFaultByReason(RUNNING_TASK_MEET_LIMITS), Faults::OTHERS);
    EXPECT_EQ(CommonUtils::GetFaultByReason(USER_OPERATION), Faults::OTHERS);
    EXPECT_EQ(CommonUtils::GetFaultByReason(APP_BACKGROUND_OR_TERMINATE), Faults::OTHERS);
    EXPECT_EQ(CommonUtils::GetFaultByReason(NETWORK_OFFLINE), Faults::DISCONNECTED);
    EXPECT_EQ(CommonUtils::GetFaultByReason(UNSUPPORTED_NETWORK_TYPE), Faults::OTHERS);
    EXPECT_EQ(CommonUtils::GetFaultByReason(BUILD_CLIENT_FAILED), Faults::PARAM);
    EXPECT_EQ(CommonUtils::GetFaultByReason(BUILD_REQUEST_FAILED), Faults::PARAM);
    EXPECT_EQ(CommonUtils::GetFaultByReason(GET_FILESIZE_FAILED), Faults::FSIO);
    EXPECT_EQ(CommonUtils::GetFaultByReason(CONTINUOUS_TASK_TIMEOUT), Faults::OTHERS);
    EXPECT_EQ(CommonUtils::GetFaultByReason(CONNECT_ERROR), Faults::TCP);
    EXPECT_EQ(CommonUtils::GetFaultByReason(REQUEST_ERROR), Faults::PROTOCOL);
    EXPECT_EQ(CommonUtils::GetFaultByReason(UPLOAD_FILE_ERROR), Faults::OTHERS);
    EXPECT_EQ(CommonUtils::GetFaultByReason(REDIRECT_ERROR), Faults::REDIRECT);
    EXPECT_EQ(CommonUtils::GetFaultByReason(PROTOCOL_ERROR), Faults::PROTOCOL);
    EXPECT_EQ(CommonUtils::GetFaultByReason(IO_ERROR), Faults::FSIO);
    EXPECT_EQ(CommonUtils::GetFaultByReason(UNSUPPORT_RANGE_REQUEST), Faults::PROTOCOL);
    EXPECT_EQ(CommonUtils::GetFaultByReason(OTHERS_ERROR), Faults::OTHERS);
    EXPECT_EQ(CommonUtils::GetFaultByReason(ACCOUNT_STOPPED), Faults::OTHERS);
    EXPECT_EQ(CommonUtils::GetFaultByReason(NETWORK_CHANGED), Faults::OTHERS);
    EXPECT_EQ(CommonUtils::GetFaultByReason(DNS), Faults::DNS);
    EXPECT_EQ(CommonUtils::GetFaultByReason(TCP), Faults::TCP);
    EXPECT_EQ(CommonUtils::GetFaultByReason(SSL), Faults::SSL);
    EXPECT_EQ(CommonUtils::GetFaultByReason(INSUFFICIENT_SPACE), Faults::OTHERS);
    EXPECT_EQ(CommonUtils::GetFaultByReason(NETWORK_APP), Faults::DISCONNECTED);
    EXPECT_EQ(CommonUtils::GetFaultByReason(NETWORK_ACCOUNT), Faults::DISCONNECTED);
    EXPECT_EQ(CommonUtils::GetFaultByReason(APP_ACCOUNT), Faults::OTHERS);
    EXPECT_EQ(CommonUtils::GetFaultByReason(NETWORK_APP_ACCOUNT), Faults::DISCONNECTED);
    Reason code = static_cast<Reason>(1000);
    EXPECT_EQ(CommonUtils::GetFaultByReason(code), Faults::OTHERS);
}

/**
 * @tc.name: GetMsgByReason001
 * @tc.desc: Test GetMsgByReason001 interface base function - GetMsgByReason
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(ParcelHelperTest, CommonUtilsGetMsgByReason001, TestSize.Level1)
{
    EXPECT_EQ(CommonUtils::GetMsgByReason(REASON_OK), CommonUtils::REASON_OK_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(REASON_OK), CommonUtils::REASON_OK_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(TASK_SURVIVAL_ONE_MONTH), CommonUtils::TASK_SURVIVAL_ONE_MONTH_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(WAITTING_NETWORK_ONE_DAY), CommonUtils::WAITTING_NETWORK_ONE_DAY_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(STOPPED_NEW_FRONT_TASK), CommonUtils::STOPPED_NEW_FRONT_TASK_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(RUNNING_TASK_MEET_LIMITS), CommonUtils::RUNNING_TASK_MEET_LIMITS_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(USER_OPERATION), CommonUtils::USER_OPERATION_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(APP_BACKGROUND_OR_TERMINATE), CommonUtils::APP_BACKGROUND_OR_TERMINATE_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(NETWORK_OFFLINE), CommonUtils::NETWORK_OFFLINE_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(UNSUPPORTED_NETWORK_TYPE), CommonUtils::UNSUPPORTED_NETWORK_TYPE_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(BUILD_CLIENT_FAILED), CommonUtils::BUILD_CLIENT_FAILED_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(BUILD_REQUEST_FAILED), CommonUtils::BUILD_REQUEST_FAILED_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(GET_FILESIZE_FAILED), CommonUtils::GET_FILESIZE_FAILED_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(CONTINUOUS_TASK_TIMEOUT), CommonUtils::CONTINUOUS_TASK_TIMEOUT_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(CONNECT_ERROR), CommonUtils::CONNECT_ERROR_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(REQUEST_ERROR), CommonUtils::REQUEST_ERROR_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(UPLOAD_FILE_ERROR), CommonUtils::UPLOAD_FILE_ERROR_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(REDIRECT_ERROR), CommonUtils::REDIRECT_ERROR_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(PROTOCOL_ERROR), CommonUtils::PROTOCOL_ERROR_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(IO_ERROR), CommonUtils::IO_ERROR_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(UNSUPPORT_RANGE_REQUEST), CommonUtils::UNSUPPORT_RANGE_REQUEST_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(OTHERS_ERROR), CommonUtils::OTHERS_ERROR_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(ACCOUNT_STOPPED), CommonUtils::ACCOUNT_STOPPED_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(NETWORK_CHANGED), CommonUtils::NETWORK_CHANGED_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(DNS), CommonUtils::DNS_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(TCP), CommonUtils::TCP_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(SSL), CommonUtils::SSL_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(INSUFFICIENT_SPACE), CommonUtils::INSUFFICIENT_SPACE_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(NETWORK_APP), CommonUtils::NETWORK_APP_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(NETWORK_ACCOUNT), CommonUtils::NETWORK_ACCOUNT_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(APP_ACCOUNT), CommonUtils::APP_ACCOUNT_INFO);
    EXPECT_EQ(CommonUtils::GetMsgByReason(NETWORK_APP_ACCOUNT), CommonUtils::NETWORK_ACCOUNT_APP_INFO);
    Reason code = static_cast<Reason>(1000);
    EXPECT_EQ(CommonUtils::GetMsgByReason(code), "unknown");
}