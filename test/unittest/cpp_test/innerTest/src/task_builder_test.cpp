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

#include <cstdint>
#include <cstring>
#include <sstream>
#define private public
#define protected public

#include <gtest/gtest.h>

#include "log.h"
#include "request_common.h"
#include "task_builder.h"

using namespace testing::ext;
using namespace OHOS::Request;

#undef private
#undef protected

class TaskBuilderTest : public testing::Test {
public:
    static void SetUpTestCase(void);
    static void TearDownTestCase(void);
    void SetUp();
    void TearDown();
};

void TaskBuilderTest::SetUpTestCase(void)
{
    // input testSuit setup step，setup invoked before all testCases
}

void TaskBuilderTest::TearDownTestCase(void)
{
    // input testSuit teardown step，teardown invoked after all testCases
}

void TaskBuilderTest::SetUp(void)
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

void TaskBuilderTest::TearDown(void)
{
    // input testCase teardown step，teardown invoked after each testCase
}

/**
 * @tc.name: CheckAction001
 * @tc.desc: Test TaskBuilder interface base function - CheckAction001
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, CheckAction001, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setAction(Action::ANY);
    bool ret = builder.checkAction();
    EXPECT_EQ(ret, false);
}

/**
 * @tc.name: CheckAction002
 * @tc.desc: Test TaskBuilder interface base function - CheckAction002
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, CheckAction002, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setAction(Action::DOWNLOAD);
    bool ret = builder.checkAction();
    EXPECT_EQ(ret, true);
}

/**
 * @tc.name: CheckAction003
 * @tc.desc: Test TaskBuilder interface base function - CheckAction003
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, CheckAction003, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setAction(Action::UPLOAD);
    bool ret = builder.checkAction();
    EXPECT_EQ(ret, true);
}

/**
 * @tc.name: CheckUrl001
 * @tc.desc: Test TaskBuilder interface base function - CheckUrl001
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, CheckUrl001, TestSize.Level1)
{
    TaskBuilder builder;
    std::ostringstream oss;
    oss << "http://example.com/";
    for (size_t i = 0; i < 8192; i++) {
        oss << "A";
    }
    std::string longUrl = oss.str();
    builder.setUrl(longUrl);
    bool ret = builder.checkUrl();
    EXPECT_EQ(ret, false);
}

/**
 * @tc.name: CheckUrl002
 * @tc.desc: Test TaskBuilder interface base function - CheckUrl002
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, CheckUrl002, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setUrl("example.com");
    bool ret = builder.checkUrl();
    EXPECT_EQ(ret, false);
}

/**
 * @tc.name: CheckUrl003
 * @tc.desc: Test TaskBuilder interface base function - CheckUrl003
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, CheckUrl003, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setUrl("https://example.com");
    bool ret = builder.checkUrl();
    EXPECT_EQ(ret, true);
}

/**
 * @tc.name: checkCertsPath001
 * @tc.desc: Test TaskBuilder interface base function - checkCertsPath001
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkCertsPath001, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setUrl("http://example.com");
    builder.checkCertsPath();
    builder.setUrl("https:");
    builder.checkCertsPath();
    builder.setUrl("https:example");
    builder.checkCertsPath();
    builder.setUrl("https://example.com/files?query=1");
    builder.checkCertsPath();
    bool ret = builder.checkUrl();
    EXPECT_EQ(ret, true);
}

/**
 * @tc.name: checkData001
 * @tc.desc: Test TaskBuilder interface base function - checkData001
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkData001, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setAction(Action::UPLOAD).setData("string data");
    bool ret = builder.checkData();
    EXPECT_EQ(ret, false);
}

/**
 * @tc.name: checkData002
 * @tc.desc: Test TaskBuilder interface base function - checkData002
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkData002, TestSize.Level1)
{
    std::vector<FileSpec> files;
    FileSpec checkedFile;
    files.push_back(checkedFile);
    TaskBuilder builder;
    builder.setAction(Action::UPLOAD).setData(files);
    bool ret = builder.checkData();
    EXPECT_EQ(ret, false);
}

/**
 * @tc.name: checkData003
 * @tc.desc: Test TaskBuilder interface base function - checkData003
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkData003, TestSize.Level1)
{
    std::vector<FileSpec> files;
    FileSpec checkedFile;
    checkedFile.uri = "./checkData.txt";
    files.push_back(checkedFile);
    TaskBuilder builder;
    builder.setAction(Action::UPLOAD).setData(files);
    bool ret = builder.checkData();
    EXPECT_EQ(ret, true);
}

/**
 * @tc.name: checkData004
 * @tc.desc: Test TaskBuilder interface base function - checkData004
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkData004, TestSize.Level1)
{
    std::vector<FileSpec> files;
    FileSpec checkedFile;
    checkedFile.uri = "./checkData.txt";
    files.push_back(checkedFile);
    TaskBuilder builder;
    builder.setAction(Action::UPLOAD).setData(files).setIndex(0);
    bool ret = builder.checkData();
    EXPECT_EQ(ret, true);
}

/**
 * @tc.name: checkData005
 * @tc.desc: Test TaskBuilder interface base function - checkData005
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkData005, TestSize.Level1)
{
    std::vector<FormItem> formItems;
    FormItem item;
    item.name = "key";
    item.value = "value";
    formItems.push_back(item);
    TaskBuilder builder;
    builder.setAction(Action::DOWNLOAD).setData(formItems).setIndex(0);
    bool ret = builder.checkData();
    EXPECT_EQ(ret, true);
}

/**
 * @tc.name: checkIndex001
 * @tc.desc: Test TaskBuilder interface base function - checkIndex001
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkIndex001, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setAction(Action::DOWNLOAD).setIndex(2);
    bool ret = builder.checkIndex();
    EXPECT_EQ(ret, true);
    EXPECT_EQ(builder.config.index, 0);
}

/**
 * @tc.name: checkIndex002
 * @tc.desc: Test TaskBuilder interface base function - checkIndex002
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkIndex002, TestSize.Level1)
{
    std::vector<FileSpec> files;
    FileSpec checkedFile;
    checkedFile.uri = "./checkData.txt";
    files.push_back(checkedFile);
    TaskBuilder builder;
    builder.setAction(Action::UPLOAD).setIndex(2).setData(files);
    bool ret = builder.checkIndex();
    EXPECT_EQ(ret, false);
}

/**
 * @tc.name: checkProxy001
 * @tc.desc: Test TaskBuilder interface base function - checkProxy001
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkProxy001, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setProxy("");
    bool ret = builder.checkProxy();
    EXPECT_EQ(ret, true);
}

/**
 * @tc.name: checkProxy002
 * @tc.desc: Test TaskBuilder interface base function - checkProxy002
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkProxy002, TestSize.Level1)
{
    std::ostringstream oss;
    oss << "http://example.com/";
    for (size_t i = 0; i < 513; i++) {
        oss << "A";
    }
    std::string proxyUrl = oss.str();
    TaskBuilder builder;
    builder.setProxy(proxyUrl);
    bool ret = builder.checkProxy();
    EXPECT_EQ(ret, false);
}

/**
 * @tc.name: checkProxy003
 * @tc.desc: Test TaskBuilder interface base function - checkProxy003
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkProxy003, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setProxy("https://example.com");
    bool ret = builder.checkProxy();
    EXPECT_EQ(ret, false);
}

/**
 * @tc.name: checkProxy004
 * @tc.desc: Test TaskBuilder interface base function - checkProxy004
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkProxy004, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setProxy("http://example.com");
    bool ret = builder.checkProxy();
    EXPECT_EQ(ret, false);
}

/**
 * @tc.name: checkProxy005
 * @tc.desc: Test TaskBuilder interface base function - checkProxy005
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkProxy005, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setProxy("http://example.com:80");
    bool ret = builder.checkProxy();
    EXPECT_EQ(ret, true);
}

/**
 * @tc.name: checkTitle001
 * @tc.desc: Test TaskBuilder interface base function - checkTitle001
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkTitle001, TestSize.Level1)
{
    std::ostringstream oss;
    oss << "TaskBuilder Title";
    for (size_t i = 0; i < 256; i++) {
        oss << "A";
    }
    std::string title = oss.str();
    TaskBuilder builder;
    builder.setTitle(title);
    bool ret = builder.checkTitle();
    EXPECT_EQ(ret, false);
}

/**
 * @tc.name: checkTitle002
 * @tc.desc: Test TaskBuilder interface base function - checkTitle002
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkTitle002, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setTitle("");
    bool ret = builder.setAction(Action::UPLOAD).checkTitle();
    EXPECT_EQ(ret, true);
    EXPECT_EQ(builder.config.title, "upload");
}

/**
 * @tc.name: checkTitle003
 * @tc.desc: Test TaskBuilder interface base function - checkTitle003
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkTitle003, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setTitle("");
    bool ret = builder.setAction(Action::DOWNLOAD).checkTitle();
    EXPECT_EQ(ret, true);
    EXPECT_EQ(builder.config.title, "download");
}

/**
 * @tc.name: checkToken001
 * @tc.desc: Test TaskBuilder interface base function - checkToken001
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkToken001, TestSize.Level1)
{
    TaskBuilder builder;
    bool ret = builder.checkToken();
    EXPECT_EQ(ret, true);
}

/**
 * @tc.name: checkToken002
 * @tc.desc: Test TaskBuilder interface base function - checkToken002
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkToken002, TestSize.Level1)
{
    TaskBuilder builder;
    bool ret = builder.setToken("less8").checkToken();
    EXPECT_EQ(ret, false);
}

/**
 * @tc.name: checkToken003
 * @tc.desc: Test TaskBuilder interface base function - checkToken003
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkToken003, TestSize.Level1)
{
    std::ostringstream oss;
    oss << "TaskBuilder Token";
    for (size_t i = 0; i < 2049; i++) {
        oss << "A";
    }
    std::string tokenStr = oss.str();
    TaskBuilder builder;
    bool ret = builder.setToken(tokenStr).checkToken();
    EXPECT_EQ(ret, false);
}

/**
 * @tc.name: checkToken004
 * @tc.desc: Test TaskBuilder interface base function - checkToken004
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkToken004, TestSize.Level1)
{
    TaskBuilder builder;
    bool ret = builder.setToken("Token more than 8").checkToken();
    EXPECT_EQ(ret, true);
}

/**
 * @tc.name: checkDescription001
 * @tc.desc: Test TaskBuilder interface base function - checkDescription001
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkDescription001, TestSize.Level1)
{
    std::ostringstream oss;
    oss << "TaskBuilder Description";
    for (size_t i = 0; i < 2029; i++) {
        oss << "A";
    }
    std::string descriptionStr = oss.str();
    TaskBuilder builder;
    bool ret = builder.setDescription(descriptionStr).checkDescription();
    EXPECT_EQ(ret, false);
}

/**
 * @tc.name: checkDescription002
 * @tc.desc: Test TaskBuilder interface base function - checkDescription002
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkDescription002, TestSize.Level1)
{
    TaskBuilder builder;
    bool ret = builder.setDescription("TaskBuilder description").checkDescription();
    EXPECT_EQ(ret, true);
}

/**
 * @tc.name: checkSaveas001
 * @tc.desc: Test TaskBuilder interface base function - checkSaveas001
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkSaveas001, TestSize.Level1)
{
    TaskBuilder builder;
    bool ret = builder.setAction(Action::UPLOAD).setSaveAs("any").checkSaveas();
    EXPECT_EQ(ret, true);
    EXPECT_EQ(builder.config.saveas, "");
}

/**
 * @tc.name: checkSaveas002
 * @tc.desc: Test TaskBuilder interface base function - checkSaveas002
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkSaveas002, TestSize.Level1)
{
    TaskBuilder builder;
    bool ret = builder.setAction(Action::DOWNLOAD).setSaveAs("./saveAs.txt").checkSaveas();
    EXPECT_EQ(ret, true);
}

/**
 * @tc.name: checkSaveas003
 * @tc.desc: Test TaskBuilder interface base function - checkSaveas003
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkSaveas003, TestSize.Level1)
{
    TaskBuilder builder;
    bool ret =
        builder.setAction(Action::DOWNLOAD).setUrl("https://example.com/saveAs.txt").setSaveAs("./").checkSaveas();
    EXPECT_EQ(ret, true);
}

/**
 * @tc.name: checkSaveas004
 * @tc.desc: Test TaskBuilder interface base function - checkSaveas004
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkSaveas004, TestSize.Level1)
{
    TaskBuilder builder;
    bool ret = builder.setAction(Action::DOWNLOAD).setUrl("https://example.com/").setSaveAs("./").checkSaveas();
    EXPECT_EQ(ret, false);
}

/**
 * @tc.name: checkSaveas005
 * @tc.desc: Test TaskBuilder interface base function - checkSaveas005
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkSaveas005, TestSize.Level1)
{
    TaskBuilder builder;
    bool ret = builder.setAction(Action::DOWNLOAD).setUrl("https://example.com/").setSaveAs("./data/").checkSaveas();
    EXPECT_EQ(ret, false);
}

/**
 * @tc.name: checkCertificatePins001
 * @tc.desc: Test TaskBuilder interface base function - checkCertificatePins001
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkCertificatePins001, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setUrl("").checkCertificatePins();
    EXPECT_TRUE(builder.config.certificatePins.empty());
}

/**
 * @tc.name: checkCertificatePins002
 * @tc.desc: Test TaskBuilder interface base function - checkCertificatePins002
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkCertificatePins002, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setUrl("https://checkCertificate.test:80/data").checkCertificatePins();
    EXPECT_TRUE(builder.config.certificatePins.empty());
}

/**
 * @tc.name: checkMethod001
 * @tc.desc: Test TaskBuilder interface base function - checkMethod001
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkMethod001, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setAction(Action::UPLOAD).checkMethod();
    EXPECT_EQ(builder.config.method, "PUT");
}

/**
 * @tc.name: checkMethod002
 * @tc.desc: Test TaskBuilder interface base function - checkMethod002
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkMethod002, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setAction(Action::DOWNLOAD).checkMethod();
    EXPECT_EQ(builder.config.method, "GET");
}

/**
 * @tc.name: checkMethod003
 * @tc.desc: Test TaskBuilder interface base function - checkMethod003
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkMethod003, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setAction(Action::UPLOAD).setMethod("POST").checkMethod();
    EXPECT_EQ(builder.config.method, "POST");
}

/**
 * @tc.name: checkMethod004
 * @tc.desc: Test TaskBuilder interface base function - checkMethod004
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkMethod004, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setAction(Action::DOWNLOAD).setMethod("GET").checkMethod();
    EXPECT_EQ(builder.config.method, "GET");
}

/**
 * @tc.name: checkOtherConfig001
 * @tc.desc: Test TaskBuilder interface base function - checkOtherConfig001
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkOtherConfig001, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setBegins(-1).checkOtherConfig();
    EXPECT_EQ(builder.config.begins, 0);
}

/**
 * @tc.name: checkOtherConfig002
 * @tc.desc: Test TaskBuilder interface base function - checkOtherConfig002
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, checkOtherConfig002, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setMode(Mode::BACKGROUND).checkOtherConfig();
    EXPECT_TRUE(builder.config.background);
}

/**
 * @tc.name: build001
 * @tc.desc: Test TaskBuilder interface base function - build001
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, build001, TestSize.Level1)
{
    TaskBuilder builder;
    auto res = builder.setUrl("https://127.0.0.1/data.txt")
                   .setDescription("test for TaskBuilder")
                   .setMode(Mode::BACKGROUND)
                   .setOverwrite(true)
                   .setMethod("GET")
                   .setAction(Action::DOWNLOAD)
                   .setSaveAs("./task_builder_test.txt")
                   .setNetwork(Network::WIFI)
                   .setMetered(true)
                   .setRoaming(false)
                   .setRetry(true)
                   .setRedirect(true)
                   .setIndex(0)
                   .setBegins(0)
                   .setEnds(-1)
                   .setGauge(true)
                   .setToken("null")
                   .build();
    EXPECT_EQ(res.second, ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: build002
 * @tc.desc: Test TaskBuilder interface base function - checkAction
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, build002, TestSize.Level1)
{
    TaskBuilder builder;
    auto res = builder.setAction(Action::ANY).build();
    EXPECT_EQ(res.second, ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: build003
 * @tc.desc: Test TaskBuilder interface base function - checkUrl
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, build003, TestSize.Level1)
{
    TaskBuilder builder;
    auto res = builder.setAction(Action::DOWNLOAD).setUrl("123").build();
    EXPECT_EQ(res.second, ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: build004
 * @tc.desc: Test TaskBuilder interface base function - checkData
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, build004, TestSize.Level1)
{
    TaskBuilder builder;
    std::vector<FileSpec> data;
    auto res = builder.setAction(Action::UPLOAD).setUrl("https://127.0.0.1/data.txt").setData(data).build();
    EXPECT_EQ(res.second, ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: build005
 * @tc.desc: Test TaskBuilder interface base function - checkIndex
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, build005, TestSize.Level1)
{
    TaskBuilder builder;
    auto res = builder.setAction(Action::UPLOAD).setUrl("https://127.0.0.1/data.txt").setIndex(100).build();
    EXPECT_EQ(res.second, ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: build006
 * @tc.desc: Test TaskBuilder interface base function - checkProxy
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, build006, TestSize.Level1)
{
    TaskBuilder builder;
    auto res =
        builder.setAction(Action::DOWNLOAD).setUrl("https://127.0.0.1/data.txt").setProxy("http://example.com").build();
    EXPECT_EQ(res.second, ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: build007
 * @tc.desc: Test TaskBuilder interface base function - checkTitle
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, build007, TestSize.Level1)
{
    TaskBuilder builder;
    std::string title(257, 'a');
    auto res = builder.setAction(Action::DOWNLOAD).setUrl("https://127.0.0.1/data.txt").setTitle(title).build();
    EXPECT_EQ(res.second, ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: build008
 * @tc.desc: Test TaskBuilder interface base function - checkDescription
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, build008, TestSize.Level1)
{
    TaskBuilder builder;
    std::string description(1025, 'a');
    auto res =
        builder.setAction(Action::DOWNLOAD).setUrl("https://127.0.0.1/data.txt").setDescription(description).build();
    EXPECT_EQ(res.second, ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: build009
 * @tc.desc: Test TaskBuilder interface base function - checkSaveas
 * @tc.type: FUNC
 * @tc.require: Issue Number
 */
HWTEST_F(TaskBuilderTest, build009, TestSize.Level1)
{
    TaskBuilder builder;
    std::string description(1025, 'a');
    auto res = builder.setAction(Action::DOWNLOAD).setUrl("https://example.com/").setSaveAs("./data/").build();
    EXPECT_EQ(res.second, ExceptionErrorCode::E_PARAMETER_CHECK);
}