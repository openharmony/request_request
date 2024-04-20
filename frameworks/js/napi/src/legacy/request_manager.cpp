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

#include "legacy/request_manager.h"

#include <cerrno>
#include <climits>
#include <cstdlib>

#include "ability.h"
#include "legacy/download_task.h"
#include "log.h"
#include "napi_base_context.h"
#include "napi_utils.h"
#include "uv.h"

namespace OHOS::Request::Legacy {
std::map<std::string, RequestManager::DownloadDescriptor> RequestManager::downloadDescriptors_;
std::mutex RequestManager::lock_;
std::atomic<uint32_t> RequestManager::taskId_;

bool RequestManager::IsLegacy(napi_env env, napi_callback_info info)
{
    size_t argc = DOWNLOAD_ARGC;
    napi_value argv[DOWNLOAD_ARGC]{};
    NAPI_CALL_BASE(env, napi_get_cb_info(env, info, &argc, argv, nullptr, nullptr), false);
    auto successCb = NapiUtils::GetNamedProperty(env, argv[0], "success");
    auto failCb = NapiUtils::GetNamedProperty(env, argv[0], "fail");
    auto completeCb = NapiUtils::GetNamedProperty(env, argv[0], "complete");
    return successCb || failCb || completeCb;
}

std::string RequestManager::GetTaskToken()
{
    uint32_t id = taskId_++;
    return "Download-Task-" + std::to_string(id);
}

void RequestManager::CallFunctionAsync(napi_env env, napi_ref func, const ArgsGenerator &generator)
{
    uv_loop_s *loop = nullptr;
    napi_get_uv_event_loop(env, &loop);
    if (loop == nullptr) {
        REQUEST_HILOGE("Failed to get uv event loop");
        return;
    }
    auto *work = new (std::nothrow) uv_work_t;
    if (work == nullptr) {
        REQUEST_HILOGE("Failed to create uv work");
        return;
    }
    auto *data = new (std::nothrow) CallFunctionData;
    if (data == nullptr) {
        REQUEST_HILOGE("Failed to create CallFunctionData");
        delete work;
        return;
    }
    data->env_ = env;
    data->func_ = func;
    data->generator_ = generator;
    work->data = data;

    uv_queue_work(
        loop, work, [](uv_work_t *work) {},
        [](uv_work_t *work, int st) {
            int argc{};
            auto data = static_cast<CallFunctionData *>(work->data);
            napi_handle_scope scope = nullptr;
            napi_open_handle_scope(data->env_, &scope);
            napi_value argv[MAX_CB_ARGS]{};
            napi_ref recv{};
            data->generator_(data->env_, &recv, argc, argv);
            napi_value callback{};
            napi_get_reference_value(data->env_, data->func_, &callback);
            napi_value thiz{};
            napi_get_reference_value(data->env_, recv, &thiz);
            napi_value result{};
            napi_call_function(data->env_, thiz, callback, argc, argv, &result);
            napi_delete_reference(data->env_, data->func_);
            napi_delete_reference(data->env_, recv);
            napi_close_handle_scope(data->env_, scope);
            delete work;
            delete data;
        });
}

void RequestManager::OnTaskDone(const std::string &token, bool successful, const std::string &errMsg)
{
    DownloadDescriptor descriptor{};
    {
        std::lock_guard<std::mutex> lockGuard(lock_);
        auto it = downloadDescriptors_.find(token);
        if (it == downloadDescriptors_.end()) {
            return;
        }
        descriptor = it->second;
        downloadDescriptors_.erase(it);
    }

    if (successful && descriptor.successCb_) {
        CallFunctionAsync(descriptor.env_, descriptor.successCb_,
            [descriptor](napi_env env, napi_ref *recv, int &argc, napi_value *argv) {
                *recv = descriptor.this_;
                argc = SUCCESS_CB_ARGC;
                argv[0] = NapiUtils::CreateObject(descriptor.env_);
                NapiUtils::SetStringPropertyUtf8(descriptor.env_, argv[0], "uri", URI_PREFIX + descriptor.filename_);
            });
    }
    if (!successful && descriptor.failCb_) {
        CallFunctionAsync(descriptor.env_, descriptor.failCb_,
            [descriptor, errMsg](napi_env env, napi_ref *recv, int &argc, napi_value *argv) {
                *recv = descriptor.this_;
                argc = FAIL_CB_ARGC;
                argv[0] = NapiUtils::Convert2JSValue(descriptor.env_, errMsg);
                argv[1] = NapiUtils::Convert2JSValue(descriptor.env_, FAIL_CB_DOWNLOAD_ERROR);
            });
    }
    delete descriptor.task_;
}

std::string RequestManager::GetFilenameFromUrl(std::string &url)
{
    auto pos = url.rfind('/');
    if (pos != std::string::npos) {
        return url.substr(pos + 1);
    }
    return url;
}

std::string RequestManager::GetCacheDir(napi_env env)
{
    auto ability = AbilityRuntime::GetCurrentAbility(env);
    if (ability == nullptr) {
        REQUEST_HILOGE("GetCurrentAbility failed.");
        return {};
    }
    auto abilityContext = ability->GetAbilityContext();
    if (abilityContext == nullptr) {
        REQUEST_HILOGE("GetAbilityContext failed.");
        return {};
    }
    return abilityContext->GetCacheDir();
}

std::vector<std::string> RequestManager::ParseHeader(napi_env env, napi_value option)
{
    if (!NapiUtils::HasNamedProperty(env, option, "header")) {
        REQUEST_HILOGD("no header present");
        return {};
    }
    napi_value header = NapiUtils::GetNamedProperty(env, option, "header");
    if (NapiUtils::GetValueType(env, header) != napi_object) {
        REQUEST_HILOGE("header type is not object");
        return {};
    }
    auto names = NapiUtils::GetPropertyNames(env, header);
    REQUEST_HILOGD("names size=%{public}d", static_cast<int32_t>(names.size()));
    std::vector<std::string> headerVector;
    for (const auto &name : names) {
        auto value = NapiUtils::Convert2String(env, header, name);
        headerVector.push_back(name + ":" + value);
    }
    return headerVector;
}

DownloadTask::DownloadOption RequestManager::ParseOption(napi_env env, napi_value option)
{
    DownloadTask::DownloadOption downloadOption;
    downloadOption.url_ = NapiUtils::Convert2String(env, option, "url");
    downloadOption.fileDir_ = GetCacheDir(env);

    downloadOption.filename_ = NapiUtils::Convert2String(env, option, "filename");
    if (downloadOption.filename_.empty()) {
        downloadOption.filename_ = GetFilenameFromUrl(downloadOption.url_);
        int i = 0;
        auto filename = downloadOption.filename_;
        while (access((downloadOption.fileDir_ + '/' + filename).c_str(), F_OK) == 0) {
            i++;
            filename = downloadOption.filename_ + std::to_string(i);
        }
        downloadOption.filename_ = filename;
    }

    downloadOption.header_ = ParseHeader(env, option);

    return downloadOption;
}

bool RequestManager::IsPathValid(const std::string &dir, const std::string &filename)
{
    auto filepath = dir + '/' + filename;
    auto fileDirectory = filepath.substr(0, filepath.rfind('/'));
    char resolvedPath[PATH_MAX] = { 0 };
    if (realpath(fileDirectory.c_str(), resolvedPath) && !strncmp(resolvedPath, dir.c_str(), dir.length())) {
        return true;
    }
    REQUEST_HILOGE("file path is invalid, errno=%{public}d", errno);
    return false;
}

bool RequestManager::HasSameFilename(const std::string &filename)
{
    std::lock_guard<std::mutex> lockGuard(lock_);
    for (const auto &element : downloadDescriptors_) {
        if (element.second.filename_ == filename) {
            return true;
        }
    }
    return false;
}

void RequestManager::CallFailCallback(napi_env env, napi_value object, const std::string &msg)
{
    auto callback = NapiUtils::GetNamedProperty(env, object, "fail");
    if (callback != nullptr) {
        REQUEST_HILOGI("call fail of download");
        napi_value result[FAIL_CB_ARGC]{};
        result[0] = NapiUtils::Convert2JSValue(env, msg);
        result[1] = NapiUtils::Convert2JSValue(env, FAIL_CB_DOWNLOAD_ERROR);
        NapiUtils::CallFunction(env, object, callback, FAIL_CB_ARGC, result);
    }
}

void RequestManager::CallSuccessCallback(napi_env env, napi_value object, const std::string &token)
{
    auto successCb = NapiUtils::GetNamedProperty(env, object, "success");
    if (successCb != nullptr) {
        REQUEST_HILOGI("call success of download");
        auto responseObject = NapiUtils::CreateObject(env);
        NapiUtils::SetStringPropertyUtf8(env, responseObject, "token", token);
        NapiUtils::CallFunction(env, object, successCb, 1, &responseObject);
    }
}

napi_value RequestManager::Download(napi_env env, napi_callback_info info)
{
    size_t argc = DOWNLOAD_ARGC;
    napi_value argv[DOWNLOAD_ARGC]{};
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, argv, nullptr, nullptr));
    napi_value res = NapiUtils::GetUndefined(env);

    auto option = ParseOption(env, argv[0]);
    if (!IsPathValid(option.fileDir_, option.filename_)) {
        CallFailCallback(env, argv[0], "invalid file name");
        return res;
    }
    if (HasSameFilename(option.filename_)) {
        CallFailCallback(env, argv[0], "filename conflict");
        return res;
    }

    auto token = GetTaskToken();
    auto *task = new (std::nothrow) DownloadTask(token, option, OnTaskDone);
    if (task == nullptr) {
        return res;
    }
    DownloadDescriptor descriptor{ task, option.filename_, env };
    {
        std::lock_guard<std::mutex> lockGuard(lock_);
        downloadDescriptors_[token] = descriptor;
    }
    CallSuccessCallback(env, argv[0], token);
    task->Start();
    return res;
}

napi_value RequestManager::OnDownloadComplete(napi_env env, napi_callback_info info)
{
    size_t argc = DOWNLOAD_ARGC;
    napi_value argv[DOWNLOAD_ARGC]{};
    NAPI_CALL(env, napi_get_cb_info(env, info, &argc, argv, nullptr, nullptr));
    napi_value res = NapiUtils::GetUndefined(env);

    auto token = NapiUtils::Convert2String(env, argv[0], "token");
    {
        std::lock_guard<std::mutex> lockGuard(lock_);
        auto it = downloadDescriptors_.find(token);
        if (it != downloadDescriptors_.end()) {
            it->second.env_ = env;
            napi_create_reference(env, argv[0], 1, &it->second.this_);
            auto callback = NapiUtils::GetNamedProperty(env, argv[0], "success");
            napi_create_reference(env, callback, 1, &it->second.successCb_);
            callback = NapiUtils::GetNamedProperty(env, argv[0], "fail");
            napi_create_reference(env, callback, 1, &it->second.failCb_);
            return res;
        }
    }
    auto callback = NapiUtils::GetNamedProperty(env, argv[0], "fail");
    if (callback != nullptr) {
        napi_value result[FAIL_CB_ARGC]{};
        std::string message = "Download task doesn't exist!";
        result[0] = NapiUtils::Convert2JSValue(env, message);
        result[1] = NapiUtils::Convert2JSValue(env, FAIL_CB_TASK_NOT_EXIST);
        NapiUtils::CallFunction(env, argv[0], callback, FAIL_CB_ARGC, result);
    }
    return res;
}
} // namespace OHOS::Request::Legacy