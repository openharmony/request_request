/*
 * Copyright (C) 2023 Huawei Device Co., Ltd.
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

#ifndef OHOS_REQUEST_ACTION_H
#define OHOS_REQUEST_ACTION_H

#include "access_token.h"
#include "accesstoken_kit.h"
#include "request_manager.h"

namespace OHOS::Request {

static const std::string DOWNLOAD_PERMISSION = "ohos.permission.DOWNLOAD_SESSION_MANAGER";
static const std::string UPLOAD_PERMISSION = "ohos.permission.UPLOAD_SESSION_MANAGER";

class RequestAction {
public:
    static const std::unique_ptr<RequestAction> &GetInstance();
    int32_t GetTask(const std::string &tid, const std::string &token, Config &config);
    int32_t Start(const std::string &tid);
    int32_t Stop(const std::string &tid);
    int32_t Touch(const std::string &tid, const std::string &token, TaskInfo &info);
    int32_t Show(const std::string &tid, TaskInfo &info);
    int32_t Pause(const std::string &tid);
    int32_t Remove(const std::string &tid);
    int32_t Resume(const std::string &tid);

    int32_t StartTasks(const std::vector<std::string> &tids, std::vector<int32_t> &rets);
    int32_t StopTasks(const std::vector<std::string> &tids, std::vector<int32_t> &rets);
    int32_t ResumeTasks(const std::vector<std::string> &tids, std::vector<int32_t> &rets);
    int32_t RemoveTasks(const std::vector<std::string> &tids, std::vector<int32_t> &rets);
    int32_t PauseTasks(const std::vector<std::string> &tids, std::vector<int32_t> &rets);
    int32_t ShowTasks(const std::vector<std::string> &tids, std::vector<std::pair<int32_t, TaskInfo>> &rets);
    int32_t TouchTasks(const std::vector<std::pair<std::string, std::string>> &tids,
        std::vector<std::pair<int32_t, TaskInfo>> &rets);
};

} // namespace OHOS::Request
#endif // OHOS_REQUEST_ACTION_H
