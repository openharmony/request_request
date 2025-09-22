/*
 * Copyright (c) 2024 Huawei Device Co., Ltd.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef OH_CJ_REQUEST_TASK_H
#define OH_CJ_REQUEST_TASK_H

#include <cstdint>
#include <map>
#include <mutex>
#include <vector>

#include "ability_context.h"
#include "cj_notify_data_listener.h"
#include "cj_request_ffi.h"
#include "cj_response_listener.h"
#include "constant.h"
#include "request_common.h"

namespace OHOS::CJSystemapi::Request {
using OHOS::Request::Config;
using OHOS::Request::ExceptionError;
using OHOS::Request::Filter;
using OHOS::Request::SubscribeType;
using OHOS::Request::TaskInfo;

class CJRequestTask {
public:
    CJRequestTask();
    ~CJRequestTask();

    static ExceptionError Remove(const std::string &tid);
    static ExceptionError Touch(const std::string &tid, TaskInfo &task, const std::string &token = "null");
    static ExceptionError Search(const Filter &filter, std::vector<std::string> &tids);

    std::recursive_mutex listenerMutex_;
    std::map<SubscribeType, std::shared_ptr<CJNotifyDataListener>> notifyDataListenerMap_;
    std::shared_ptr<CJResponseListener> responseListener_;

    Config config_;
    std::string taskId_{};

    static std::mutex taskMutex_;
    static std::map<std::string, CJRequestTask *> taskMap_;
    static void AddTaskMap(const std::string &key, CJRequestTask *task);
    static CJRequestTask *FindTaskById(std::string &taskId);
    static ExceptionError GetTask(OHOS::AbilityRuntime::Context *context, std::string &taskId, std::string &token,
                                  Config &config);
    static CJRequestTask *ClearTaskMap(const std::string &key);
    static void ClearTaskTemp(const std::string &tid, bool isRmFiles, bool isRmAcls, bool isRmCertsAcls);

    static std::mutex pathMutex_;
    static std::map<std::string, int32_t> pathMap_;
    static void AddPathMap(const std::string &filepath, const std::string &baseDir);
    static void RemovePathMap(const std::string &filepath);
    static void ResetDirAccess(const std::string &filepath);
    static void RemoveDirsPermission(const std::vector<std::string> &dirs);

    static bool register_;
    static void RegisterForegroundResume();

    static bool SetPathPermission(const std::string &filepath);
    static bool SetDirsPermission(std::vector<std::string> &dirs);

    std::string GetTidStr() const;
    void SetTid();

    ExceptionError Create(OHOS::AbilityRuntime::Context *context, Config &config);
    ExceptionError On(std::string type, std::string &taskId, void *callback);
    ExceptionError Off(std::string event, CFunc callback);

    static void ReloadListener();

private:
    std::string tid_;
};

} // namespace OHOS::CJSystemapi::Request
#endif