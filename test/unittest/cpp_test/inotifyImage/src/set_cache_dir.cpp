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

#include <gtest/gtest.h>

#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>
#include <thread>

#include "application_context.h"
#include "common.h"
#include "context.h"
#include "dir_operations.h"
#include "log.h"
#include "request_preload.h"

using namespace testing::ext;
using namespace OHOS::Request;
using namespace OHOS::AbilityRuntime;
using namespace std;

const std::string SLASH = "/";
const std::string IMAGE_FILE_CACHE_DIR = "image_file_cache";
const std::string PRELOAD_CACHE = "preload_caches";
const std::string DEFAULT_CACHE_PATH = "/data/storage/el2/base/cache";
constexpr size_t SLEEP_INTERVAL = 100;
static std::string TEST_URL_0 = "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt";
const std::string URL_FILE_NAME = "d506dca3cf0894bdbbd0e9310a51b9b7bf7845431e4077a96adb70662ff9749f_F";
class FileCachePath : public testing::Test {
public:
    void SetUp();
};

void FileCachePath::SetUp(void)
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

std::string GetCacheDir()
{
    auto context = Context::GetApplicationContext();
    std::string filePath;
    if (context == nullptr) {
        REQUEST_HILOGI("Get context nullptr, use default cache dir.");
        filePath = DEFAULT_CACHE_PATH + SLASH + PRELOAD_CACHE;
    } else {
        std::string contextPath = context->GetCacheDir();
        if (contextPath.empty()) {
            REQUEST_HILOGI("Get context cache dir fail, use default cache dir.");
            filePath = DEFAULT_CACHE_PATH + SLASH + PRELOAD_CACHE;
        } else {
            filePath = contextPath + SLASH + PRELOAD_CACHE;
        }
    }
    return filePath;
}

/**
 * @tc.name: SetFileCachePathTest
 * @tc.desc: Test SetFileCachePath function
 * @tc.precon: NA
 * @tc.step: 1. Create inotify dir if not exist
 *           2. Dwonload test url
 *           3. Remove inotify dir
 *           4. Verify preload_cache dir is recreate
 * @tc.expect: preload_cache is recreate.
 * @tc.type: FUNC
 * @tc.require: issueNumber
 * @tc.level: Level 1
 */
HWTEST_F(FileCachePath, SetFileCachePathTest, TestSize.Level1)
{
    std::string emptyPath;
    std::string iCachePath = DEFAULT_CACHE_PATH;

    std::string cacheDir = iCachePath + SLASH + IMAGE_FILE_CACHE_DIR;
    bool dirCreated = false;
    if (!DirOperation::IsDirExist(cacheDir)) {
        dirCreated = DirOperation::CreateDir(cacheDir);
        EXPECT_TRUE(dirCreated);
    }
    Preload::SetFileCachePath(emptyPath);
    Preload::SetFileCachePath(cacheDir);

    std::string url = TEST_URL_0;
    Preload::GetInstance()->Remove(url);
    TestCallback test;
    auto &[flagS, flagF, flagC, flagP, callback] = test;
    std::unique_ptr<PreloadOptions> options = std::make_unique<PreloadOptions>();
    auto handle = Preload::GetInstance()->load(url, std::make_unique<PreloadCallback>(callback), std::move(options));

    while (!handle->IsFinish()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
    }

    EXPECT_FALSE(flagF->load());
    EXPECT_FALSE(flagC->load());

    EXPECT_TRUE(flagP->load());
    EXPECT_TRUE(flagS->load());
    std::string filePath = GetCacheDir() + SLASH + URL_FILE_NAME;
    bool existBefore = DirOperation::IsFileExist(filePath);
    ASSERT_TRUE(existBefore);
    bool dirRemoved = DirOperation::RemoveDir(cacheDir);
    if (dirRemoved) {
        std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_INTERVAL));
        bool existAfter = DirOperation::IsFileExist(filePath);
        ASSERT_FALSE(existAfter);
    }
    Preload::GetInstance()->Remove(url);
}