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
 * @tc.desc: Test checkAction with ANY action returns false
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to ANY
 *           3. Call checkAction
 * @tc.expect: Returns false for ANY action
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkAction with DOWNLOAD action returns true
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to DOWNLOAD
 *           3. Call checkAction
 * @tc.expect: Returns true for DOWNLOAD action
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkAction with UPLOAD action returns true
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to UPLOAD
 *           3. Call checkAction
 * @tc.expect: Returns true for UPLOAD action
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkUrl with URL exceeding 8KB length returns false
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Generate URL longer than 8KB
 *           3. Set the long URL
 *           4. Call checkUrl
 * @tc.expect: Returns false for URL exceeding length limit
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkUrl with invalid URL format returns false
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set invalid URL "example.com"
 *           3. Call checkUrl
 * @tc.expect: Returns false for invalid URL format
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkUrl with valid HTTPS URL returns true
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set valid HTTPS URL
 *           3. Call checkUrl
 * @tc.expect: Returns true for valid HTTPS URL
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkCertsPath with various URL formats
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Test with HTTP URL
 *           3. Test with malformed HTTPS URLs
 *           4. Test with valid HTTPS URL
 * @tc.expect: Certificate pins remain empty for all cases
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkCertsPath001, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setUrl("http://example.com");
    builder.checkCertsPath();
    builder.setUrl("https:");
    builder.checkCertsPath();
    builder.setUrl("https");
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
 * @tc.desc: Test checkData with string data for UPLOAD action returns false
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to UPLOAD
 *           3. Set string data
 *           4. Call checkData
 * @tc.expect: Returns false for string data in UPLOAD action
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkData with empty FileSpec vector returns false
 * @tc.precon: NA
 * @tc.step: 1. Create empty FileSpec vector
 *           2. Create TaskBuilder instance
 *           3. Set action to UPLOAD
 *           4. Set empty FileSpec vector as data
 *           5. Call checkData
 * @tc.expect: Returns false for empty FileSpec vector
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkData with valid FileSpec returns true
 * @tc.precon: NA
 * @tc.step: 1. Create FileSpec vector with valid file
 *           2. Create TaskBuilder instance
 *           3. Set action to UPLOAD
 *           4. Set valid FileSpec as data
 *           5. Call checkData
 * @tc.expect: Returns true for valid FileSpec
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkData with valid FileSpec and index returns true
 * @tc.precon: NA
 * @tc.step: 1. Create FileSpec vector with valid file
 *           2. Create TaskBuilder instance
 *           3. Set action to UPLOAD
 *           4. Set valid FileSpec as data
 *           5. Set index to 0
 *           6. Call checkData
 * @tc.expect: Returns true for valid FileSpec with index
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkData with FormItem vector for DOWNLOAD returns true
 * @tc.precon: NA
 * @tc.step: 1. Create FormItem vector with valid item
 *           2. Create TaskBuilder instance
 *           3. Set action to DOWNLOAD
 *           4. Set FormItem vector as data
 *           5. Set index to 0
 *           6. Call checkData
 * @tc.expect: Returns true for valid FormItem in DOWNLOAD action
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkIndex with DOWNLOAD action and invalid index
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to DOWNLOAD
 *           3. Set invalid index 2
 *           4. Call checkIndex
 * @tc.expect: Returns true and resets index to 0
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkIndex with UPLOAD action and out-of-range index
 * @tc.precon: NA
 * @tc.step: 1. Create FileSpec vector with one file
 *           2. Create TaskBuilder instance
 *           3. Set action to UPLOAD
 *           4. Set index 2 (out of range)
 *           5. Set FileSpec data
 *           6. Call checkIndex
 * @tc.expect: Returns false for out-of-range index
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkProxy with empty proxy returns true
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set empty proxy string
 *           3. Call checkProxy
 * @tc.expect: Returns true for empty proxy
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkProxy with proxy URL exceeding length limit returns false
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Generate proxy URL longer than 512 characters
 *           3. Set the long proxy URL
 *           4. Call checkProxy
 * @tc.expect: Returns false for proxy URL exceeding length limit
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkProxy with HTTPS proxy returns false
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set HTTPS proxy URL
 *           3. Call checkProxy
 * @tc.expect: Returns false for HTTPS proxy
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkProxy with HTTP proxy without port returns false
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set HTTP proxy URL without port
 *           3. Call checkProxy
 * @tc.expect: Returns false for HTTP proxy without port
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkProxy with valid HTTP proxy returns true
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set valid HTTP proxy with port
 *           3. Call checkProxy
 * @tc.expect: Returns true for valid HTTP proxy with port
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkTitle with title exceeding 255 chars returns false
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Generate title longer than 255 characters
 *           3. Set the long title
 *           4. Call checkTitle
 * @tc.expect: Returns false for title exceeding length limit
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkTitle with empty title for UPLOAD sets default
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set empty title
 *           3. Set action to UPLOAD
 *           4. Call checkTitle
 * @tc.expect: Returns true and sets default title "upload"
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkTitle with empty title for DOWNLOAD sets default
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set empty title
 *           3. Set action to DOWNLOAD
 *           4. Call checkTitle
 * @tc.expect: Returns true and sets default title "download"
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkToken with default token returns true
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Call checkToken without setting token
 * @tc.expect: Returns true for default token
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkToken001, TestSize.Level1)
{
    TaskBuilder builder;
    bool ret = builder.checkToken();
    EXPECT_EQ(ret, true);
}

/**
 * @tc.name: checkToken002
 * @tc.desc: Test checkToken with token shorter than 8 chars returns false
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set token shorter than 8 characters
 *           3. Call checkToken
 * @tc.expect: Returns false for short token
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkToken002, TestSize.Level1)
{
    TaskBuilder builder;
    bool ret = builder.setToken("less8").checkToken();
    EXPECT_EQ(ret, false);
}

/**
 * @tc.name: checkToken003
 * @tc.desc: Test checkToken with token exceeding 2048 chars returns false
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Generate token longer than 2048 characters
 *           3. Set the long token
 *           4. Call checkToken
 * @tc.expect: Returns false for token exceeding length limit
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkToken with valid token length returns true
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set valid token with sufficient length
 *           3. Call checkToken
 * @tc.expect: Returns true for valid token length
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkToken004, TestSize.Level1)
{
    TaskBuilder builder;
    bool ret = builder.setToken("Token more than 8").checkToken();
    EXPECT_EQ(ret, true);
}

/**
 * @tc.name: checkDescription001
 * @tc.desc: Test checkDescription with description exceeding 2048 chars returns false
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Generate description longer than 2048 characters
 *           3. Set the long description
 *           4. Call checkDescription
 * @tc.expect: Returns false for description exceeding length limit
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkDescription with valid description returns true
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set valid description
 *           3. Call checkDescription
 * @tc.expect: Returns true for valid description
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkDescription002, TestSize.Level1)
{
    TaskBuilder builder;
    bool ret = builder.setDescription("TaskBuilder description").checkDescription();
    EXPECT_EQ(ret, true);
}

/**
 * @tc.name: checkSaveas001
 * @tc.desc: Test checkSaveas with UPLOAD action ignores save path
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to UPLOAD
 *           3. Set any save path
 *           4. Call checkSaveas
 * @tc.expect: Returns true and saveas remains empty for UPLOAD
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkSaveas with valid save path for DOWNLOAD returns true
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to DOWNLOAD
 *           3. Set valid save path
 *           4. Call checkSaveas
 * @tc.expect: Returns true for valid save path
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkSaveas002, TestSize.Level1)
{
    TaskBuilder builder;
    bool ret = builder.setAction(Action::DOWNLOAD).setSaveAs("./saveAs.txt").checkSaveas();
    EXPECT_EQ(ret, true);
}

/**
 * @tc.name: checkSaveas003
 * @tc.desc: Test checkSaveas with filename extraction from URL
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to DOWNLOAD
 *           3. Set URL with filename
 *           4. Set save path as directory
 *           5. Call checkSaveas
 * @tc.expect: Returns true and extracts filename from URL
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Test checkSaveas with invalid directory save path returns false
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to DOWNLOAD
 *           3. Set URL without filename
 *           4. Set save path as directory
 *           5. Call checkSaveas
 * @tc.expect: Returns false for invalid directory path
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkSaveas004, TestSize.Level1)
{
    TaskBuilder builder;
    bool ret = builder.setAction(Action::DOWNLOAD).setUrl("https://example.com/").setSaveAs("./").checkSaveas();
    EXPECT_EQ(ret, false);
}

/**
 * @tc.name: checkSaveas005
 * @tc.desc: Test checkSaveas with non-existent directory returns false
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to DOWNLOAD
 *           3. Set URL without filename
 *           4. Set save path to non-existent directory
 *           5. Call checkSaveas
 * @tc.expect: Returns false for non-existent directory
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkSaveas005, TestSize.Level1)
{
    TaskBuilder builder;
    bool ret = builder.setAction(Action::DOWNLOAD).setUrl("https://example.com/").setSaveAs("./data/").checkSaveas();
    EXPECT_EQ(ret, false);
}

/**
 * @tc.name: checkSaveas006
 * @tc.desc: Test checkSaveas with current directory save path returns true
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to DOWNLOAD
 *           3. Set URL with filename
 *           4. Set save path to current directory
 *           5. Call checkSaveas
 * @tc.expect: Returns true for current directory path
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkSaveas006, TestSize.Level1)
{
    TaskBuilder builder;
    bool ret = builder.setAction(Action::DOWNLOAD).setUrl("https://example.com/1").setSaveAs(".").checkSaveas();
    EXPECT_EQ(ret, true);
}

/**
 * @tc.name: checkCertificatePins001
 * @tc.desc: Test checkCertificatePins with empty URL returns empty pins
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set empty URL
 *           3. Call checkCertificatePins
 * @tc.expect: Returns empty certificate pins
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkCertificatePins001, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setUrl("").checkCertificatePins();
    EXPECT_TRUE(builder.config.certificatePins.empty());
}

/**
 * @tc.name: checkCertificatePins002
 * @tc.desc: Test checkCertificatePins with HTTPS URL returns empty pins
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set HTTPS URL
 *           3. Call checkCertificatePins
 * @tc.expect: Returns empty certificate pins
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkCertificatePins002, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setUrl("https://checkCertificate.test:80/data").checkCertificatePins();
    EXPECT_TRUE(builder.config.certificatePins.empty());
}

/**
 * @tc.name: checkCertificatePins003
 * @tc.desc: Test checkCertificatePins with invalid URL format returns empty pins
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set invalid URL format
 *           3. Call checkCertificatePins
 * @tc.expect: Returns empty certificate pins
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkCertificatePins003, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setUrl("checkCertificateTest").checkCertificatePins();
    EXPECT_TRUE(builder.config.certificatePins.empty());
}

/**
 * @tc.name: checkCertificatePins004
 * @tc.desc: Test checkCertificatePins with path-style URL returns empty pins
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set path-style URL
 *           3. Call checkCertificatePins
 * @tc.expect: Returns empty certificate pins
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkCertificatePins004, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setUrl("checkCertificate/Test").checkCertificatePins();
    EXPECT_TRUE(builder.config.certificatePins.empty());
}

/**
 * @tc.name: checkMethod001
 * @tc.desc: Verify default HTTP method for UPLOAD action
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to UPLOAD
 *           3. Call checkMethod()
 * @tc.expect: Default method should be "PUT"
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkMethod001, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setAction(Action::UPLOAD).checkMethod();
    EXPECT_EQ(builder.config.method, "PUT");
}

/**
 * @tc.name: checkMethod002
 * @tc.desc: Verify default HTTP method for DOWNLOAD action
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to DOWNLOAD
 *           3. Call checkMethod()
 * @tc.expect: Default method should be "GET"
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkMethod002, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setAction(Action::DOWNLOAD).checkMethod();
    EXPECT_EQ(builder.config.method, "GET");
}

/**
 * @tc.name: checkMethod003
 * @tc.desc: Verify custom HTTP method override for UPLOAD action
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to UPLOAD
 *           3. Set custom method to "POST"
 *           4. Call checkMethod()
 * @tc.expect: Method should be "POST"
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkMethod003, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setAction(Action::UPLOAD).setMethod("POST").checkMethod();
    EXPECT_EQ(builder.config.method, "POST");
}

/**
 * @tc.name: checkMethod004
 * @tc.desc: Verify explicit GET method for DOWNLOAD action
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to DOWNLOAD
 *           3. Set method to "GET"
 *           4. Call checkMethod()
 * @tc.expect: Method should be "GET"
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkMethod004, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setAction(Action::DOWNLOAD).setMethod("GET").checkMethod();
    EXPECT_EQ(builder.config.method, "GET");
}

/**
 * @tc.name: checkMethod005
 * @tc.desc: Verify POST method override for DOWNLOAD action
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to DOWNLOAD
 *           3. Set method to "POST"
 *           4. Call checkMethod()
 * @tc.expect: Method should be "POST"
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkMethod005, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setAction(Action::DOWNLOAD).setMethod("POST").checkMethod();
    EXPECT_EQ(builder.config.method, "POST");
}

/**
 * @tc.name: checkMethod006
 * @tc.desc: Verify PUT method override for UPLOAD action
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to UPLOAD
 *           3. Set method to "PUT"
 *           4. Call checkMethod()
 * @tc.expect: Method should be "PUT"
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkMethod006, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setAction(Action::UPLOAD).setMethod("PUT").checkMethod();
    EXPECT_EQ(builder.config.method, "PUT");
}

/**
 * @tc.name: checkMethod007
 * @tc.desc: Verify invalid method fallback to default for UPLOAD action
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to UPLOAD
 *           3. Set invalid method "aa"
 *           4. Call checkMethod()
 * @tc.expect: Method should fallback to "PUT"
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkMethod007, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setAction(Action::UPLOAD).setMethod("aa").checkMethod();
    EXPECT_EQ(builder.config.method, "PUT");
}

/**
 * @tc.name: checkMethod008
 * @tc.desc: Verify invalid method fallback to default for DOWNLOAD action
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to DOWNLOAD
 *           3. Set invalid method "aa"
 *           4. Call checkMethod()
 * @tc.expect: Method should fallback to "GET"
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkMethod008, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setAction(Action::DOWNLOAD).setMethod("aa").checkMethod();
    EXPECT_EQ(builder.config.method, "GET");
}

/**
 * @tc.name: checkMethod009
 * @tc.desc: Verify empty method fallback to default for DOWNLOAD action
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to DOWNLOAD
 *           3. Set empty method ""
 *           4. Call checkMethod()
 * @tc.expect: Method should fallback to "GET"
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkMethod009, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setAction(Action::DOWNLOAD).setMethod("").checkMethod();
    EXPECT_EQ(builder.config.method, "GET");
}

/**
 * @tc.name: checkOtherConfig001
 * @tc.desc: Verify negative begins value validation and reset to 0
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set begins to -1
 *           3. Call checkOtherConfig()
 * @tc.expect: begins should be reset to 0
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkOtherConfig001, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setBegins(-1).checkOtherConfig();
    EXPECT_EQ(builder.config.begins, 0);
}

/**
 * @tc.name: checkOtherConfig002
 * @tc.desc: Verify mode configuration affects background flag correctly
 * @tc.precon: NA
 * @tc.step: 1. Create first TaskBuilder instance
 *           2. Set mode to BACKGROUND
 *           3. Call checkOtherConfig()
 *           4. Create second TaskBuilder instance
 *           5. Set mode to FOREGROUND
 *           6. Call checkOtherConfig()
 * @tc.expect: BACKGROUND mode sets background=true, FOREGROUND sets background=false
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, checkOtherConfig002, TestSize.Level1)
{
    TaskBuilder builder;
    builder.setMode(Mode::BACKGROUND).checkOtherConfig();
    EXPECT_TRUE(builder.config.background);
    TaskBuilder builder1;
    builder1.setMode(Mode::FOREGROUND).checkOtherConfig();
    EXPECT_FALSE(builder1.config.background);
}

/**
 * @tc.name: build001
 * @tc.desc: Verify comprehensive parameter validation fails with invalid ends value
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set all parameters including invalid ends=-1
 *           3. Call build()
 * @tc.expect: Should return E_PARAMETER_CHECK error
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Verify invalid action ANY causes parameter validation failure
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to Action::ANY
 *           3. Call build()
 * @tc.expect: Should return E_PARAMETER_CHECK error
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, build002, TestSize.Level1)
{
    TaskBuilder builder;
    auto res = builder.setAction(Action::ANY).build();
    EXPECT_EQ(res.second, ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: build003
 * @tc.desc: Verify invalid URL format causes parameter validation failure
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to DOWNLOAD
 *           3. Set invalid URL "123"
 *           4. Call build()
 * @tc.expect: Should return E_PARAMETER_CHECK error
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, build003, TestSize.Level1)
{
    TaskBuilder builder;
    auto res = builder.setAction(Action::DOWNLOAD).setUrl("123").build();
    EXPECT_EQ(res.second, ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: build004
 * @tc.desc: Verify empty data vector causes UPLOAD validation failure
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to UPLOAD
 *           3. Set valid URL
 *           4. Set empty data vector
 *           5. Call build()
 * @tc.expect: Should return E_PARAMETER_CHECK error
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Verify invalid index out of range causes parameter validation failure
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to UPLOAD
 *           3. Set valid URL
 *           4. Create single file in data vector
 *           5. Set invalid index 100 (out of range)
 *           6. Call build()
 * @tc.expect: Should return E_PARAMETER_CHECK error
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, build005, TestSize.Level1)
{
    TaskBuilder builder;
    std::vector<FileSpec> files;
    FileSpec checkedFile;
    checkedFile.uri = "./checkData.txt";
    files.push_back(checkedFile);
    auto res =
        builder.setAction(Action::UPLOAD).setUrl("https://127.0.0.1/data.txt").setData(files).setIndex(100).build();
    EXPECT_EQ(res.second, ExceptionErrorCode::E_PARAMETER_CHECK);
}

/**
 * @tc.name: build006
 * @tc.desc: Verify invalid proxy format causes parameter validation failure
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to DOWNLOAD
 *           3. Set valid URL
 *           4. Set invalid proxy "http://example.com"
 *           5. Call build()
 * @tc.expect: Should return E_PARAMETER_CHECK error
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Verify oversized title causes parameter validation failure
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to DOWNLOAD
 *           3. Set valid URL
 *           4. Set title with 257 characters (exceeds limit)
 *           5. Call build()
 * @tc.expect: Should return E_PARAMETER_CHECK error
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Verify oversized description causes parameter validation failure
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to DOWNLOAD
 *           3. Set valid URL
 *           4. Set description with 1025 characters (exceeds limit)
 *           5. Call build()
 * @tc.expect: Should return E_PARAMETER_CHECK error
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
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
 * @tc.desc: Verify directory path in saveAs causes parameter validation failure
 * @tc.precon: NA
 * @tc.step: 1. Create TaskBuilder instance
 *           2. Set action to DOWNLOAD
 *           3. Set valid URL
 *           4. Set saveAs to directory path "./data/"
 *           5. Call build()
 * @tc.expect: Should return E_PARAMETER_CHECK error
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(TaskBuilderTest, build009, TestSize.Level1)
{
    TaskBuilder builder;
    auto res = builder.setAction(Action::DOWNLOAD).setUrl("https://example.com/").setSaveAs("./data/").build();
    EXPECT_EQ(res.second, ExceptionErrorCode::E_PARAMETER_CHECK);
}