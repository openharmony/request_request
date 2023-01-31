/*
 * Copyright (c) 2022 Huawei Device Co., Ltd.
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

#ifndef LEGACY_DOWNLOAD_MANAGER_H
#define LEGACY_DOWNLOAD_MANAGER_H

#include <atomic>
#include <functional>
#include <map>
#include <mutex>
#include "visibility.h"
#include "napi/native_api.h"
#include "napi/native_common.h"
#include "legacy/download_task.h"

namespace OHOS::Request::Download::Legacy {
class DownloadManager {
public:
    static bool IsLegacy(napi_env env, napi_callback_info info);

    static napi_value Download(napi_env env, napi_callback_info info);

    DOWNLOAD_API static napi_value OnDownloadComplete(napi_env env, napi_callback_info info);

    static void OnTaskDone(const std::string &token, bool successful, const std::string &errMsg);

private:
    using ArgsGenerator = std::function<void(napi_env env, napi_ref *recv, int &argc, napi_value *argv)>;

    struct DownloadDescriptor {
        DownloadTask *task_ {};
        std::string filename_;
        napi_env env_ {};
        napi_ref this_ {};
        napi_ref successCb_ {};
        napi_ref failCb_ {};
    };

    struct CallFunctionData {
        napi_env env_ {};
        napi_ref func_ {};
        ArgsGenerator generator_;
    };

    static std::string GetTaskToken();

    static std::string GetCacheDir(napi_env env);

    static std::string GetFilenameFromUrl(std::string &url);

    static bool IsPathValid(const std::string& dir, const std::string& filename);

    static bool HasSameFilename(const std::string& filename);

    static std::vector<std::string> ParseHeader(napi_env env, napi_value option);

    static DownloadTask::DownloadOption ParseOption(napi_env env, napi_value option);

    static void CallFailCallback(napi_env env, napi_value object, const std::string& msg);

    static void CallSuccessCallback(napi_env env, napi_value object, const std::string& token);

    static void CallFunctionAsync(napi_env env, napi_ref func, const ArgsGenerator &generator);

    static std::atomic<uint32_t> taskId_;
    static std::mutex lock_;
    static std::map<std::string, DownloadManager::DownloadDescriptor> downloadDescriptors_;

    static constexpr int DOWNLOAD_ARGC = 1;
    static constexpr int SUCCESS_CB_ARGC = 1;
    static constexpr int FAIL_CB_ARGC = 2;
    static constexpr int FAIL_CB_DOWNLOAD_ERROR = 400;
    static constexpr int FAIL_CB_TASK_NOT_EXIST = 401;
    static constexpr int MAX_CB_ARGS = 2;

    static inline const std::string URI_PREFIX = "internal://cache/";
};
}
#endif // LEGACY_DOWNLOAD_MANAGER_H