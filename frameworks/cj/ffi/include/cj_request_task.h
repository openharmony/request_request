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
#include "js_common.h"
#include "cj_notify_data_listener.h"
#include "cj_request_ffi.h"

namespace OHOS::CJSystemapi::Request {
using OHOS::Request::Config;
using OHOS::Request::Network;
using OHOS::Request::TaskInfo;
using OHOS::Request::NotifyData;
using OHOS::Request::Reason;
using OHOS::Request::DownloadErrorCode;
using OHOS::Request::ExceptionError;
using OHOS::Request::SubscribeType;
using OHOS::AbilityRuntime::Context;

class CJTask {
public:
    CJTask();
    ~CJTask();

    static ExceptionError Remove(const std::string &tid);

    std::mutex listenerMutex_;
    std::map<SubscribeType, std::shared_ptr<CJNotifyDataListener>> notifyDataListenerMap_;

    Config config_;
    std::string taskId_{ };

    static std::mutex taskMutex_;
    static std::map<std::string, CJTask *> taskMap_;
    static void AddTaskMap(const std::string &key, CJTask *task);
    static CJTask* FindTaskById(int32_t taskId);
    static CJTask* ClearTaskMap(const std::string &key);
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

    ExceptionError Create(OHOS::AbilityRuntime::Context* context, Config &config);
    ExceptionError On(std::string type, int32_t taskId, void (*callback)(CProgress progress));
    ExceptionError Off(std::string event, CFunc callback);

    static void ReloadListener();

private:
    std::string tid_;
};

}
#endif