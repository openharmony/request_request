/*
 * Copyright (c) 2021 Huawei Device Co., Ltd.
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

#include "file_utils.h"
#include "constant.h"
#include "log.h"

namespace OHOS::Request::Download {
FileUtils::FileUtils(void)
{
}

FileUtils::~FileUtils()
{
}

int FileUtils::IsFolderExist(std::string path)
{
    DIR *dp;
    if ((dp = opendir(path.c_str())) == NULL) {
        DOWNLOAD_HILOGE("FileUtils file NULL");
        return OPERATION_ERROR;
    }
    closedir(dp);
    return OPERATION_OK;
}

void FileUtils::Mkdir(std::string path)
{
    if (IsFolderExist(path) == OPERATION_ERROR) {
        int isCreate = ::mkdir(path.c_str(), S_IRUSR | S_IWUSR | S_IXUSR | S_IRWXG | S_IRWXO);
        DOWNLOAD_HILOGI("FileUtils : mkdir = %{public}d", isCreate);
    }
}

void FileUtils::WriteStringToFileAppend(std::string dirStr, const std::string str)
{
    time_t backupTime = time(nullptr);
    if (backupTime == 0) {
        DOWNLOAD_HILOGE("FileUtils : WriteStringToFileAppend time_t null");
        return;
    }
    dirStr.append("/").append(std::to_string(backupTime)).append(".log");
    std::ofstream OsWrite(dirStr, std::ofstream::app);
    OsWrite << str;
    OsWrite << std::endl;
    OsWrite.close();
}
} // namespace OHOS::Request::Download