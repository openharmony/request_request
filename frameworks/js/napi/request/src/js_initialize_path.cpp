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

#include "js_initialize.h"

#include <cstring>
#include <sstream>
#include <string>

#include "log.h"
#include "napi_utils.h"
#include "path_utils.h"
#include "request_common.h"
#include "sys_event.h"

namespace OHOS::Request {

bool JsInitialize::GetInternalPath(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config,
    std::string &path, std::string &errInfo)
{
    std::string fileName;
    std::string pattern = "internal://cache/";
    size_t pos = path.find(pattern);
    if (pos != 0) {
        fileName = path;
    } else {
        fileName = path.substr(pattern.size(), path.size());
    }
    if (fileName.empty()) {
        errInfo = "Parameter verification failed, GetInternalPath failed, fileName is empty";
        return false;
    }
    path = context->GetCacheDir();
    if (path.empty()) {
        REQUEST_HILOGE("internal to cache error");
        errInfo = "Parameter verification failed, GetInternalPath failed, cache path is empty";
        return false;
    }
    path += "/" + fileName;
    if (!NapiUtils::IsPathValid(path)) {
        REQUEST_HILOGE("IsPathValid error");
        errInfo = "Parameter verification failed, GetInternalPath failed, filePath is not valid";
        return false;
    }
    return true;
}

bool JsInitialize::GetSandboxPath(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config,
    std::string &path, std::vector<std::string> &pathVec, std::string &errInfo)
{
    if (!StandardizePath(context, config, path)) {
        REQUEST_HILOGE("StandardizePath Err");
        errInfo = "Parameter verification failed, GetSandboxPath failed, StandardizePath fail";
        return false;
    };
    if (!WholeToNormal(path, pathVec) || pathVec.empty()) {
        REQUEST_HILOGE("WholeToNormal Err");
        errInfo = "Parameter verification failed, GetSandboxPath failed, WholeToNormal path fail";
        return false;
    };
    std::string baseDir;
    if (!CheckBelongAppBaseDir(path, baseDir)) {
        REQUEST_HILOGE("CheckBelongAppBaseDir Err");
        errInfo = "Parameter verification failed, GetSandboxPath failed, path not belong app base dir";
        return false;
    };
    return true;
}

// Must not user file.
bool JsInitialize::StandardizePath(
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config, std::string &path)
{
    std::string WHOLE_PREFIX = "/";
    std::string FILE_PREFIX = "file://";
    std::string INTERNAL_PREFIX = "internal://";
    std::string CURRENT_PREFIX = "./";

    if (path.find(WHOLE_PREFIX) == 0) {
        return true;
    }
    if (path.find(FILE_PREFIX) == 0) {
        path.erase(0, FILE_PREFIX.size());
        return FileToWhole(context, config, path);
    }
    if (path.find(INTERNAL_PREFIX) == 0) {
        path.erase(0, INTERNAL_PREFIX.size());
        return BaseToWhole(context, path);
    }
    if (path.find(CURRENT_PREFIX) == 0) {
        path.erase(0, CURRENT_PREFIX.size());
        return CacheToWhole(context, path);
    }
    return CacheToWhole(context, path);
}

// BaseDir is following context.
bool JsInitialize::BaseToWhole(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, std::string &path)
{
    std::string base = context->GetBaseDir();
    if (base.empty()) {
        REQUEST_HILOGE("GetBaseDir error.");
        SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_06, "GetCacheDir error");
        return false;
    }
    path = base + "/" + path;
    return true;
}

bool JsInitialize::CacheToWhole(const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, std::string &path)
{
    std::string cache = context->GetCacheDir();
    if (cache.empty()) {
        REQUEST_HILOGE("GetCacheDir error.");
        SysEventLog::SendSysEventLog(FAULT_EVENT, ABMS_FAULT_05, "GetCacheDir error");
        return false;
    }
    path = cache + "/" + path;
    return true;
}

bool JsInitialize::FileToWhole(
    const std::shared_ptr<OHOS::AbilityRuntime::Context> &context, const Config &config, std::string &path)
{
    std::string bundleName = path.substr(0, path.find("/"));
    if (bundleName != config.bundleName) {
        REQUEST_HILOGE("path bundleName error.");
        return false;
    }
    path.erase(0, bundleName.size());
    return true;
}

bool JsInitialize::WholeToNormal(std::string &path, std::vector<std::string> &out)
{
    std::string normalPath;
    std::vector<std::string> elems;
    StringSplit(path, '/', elems);
    if (!PathVecToNormal(elems, out)) {
        return false;
    }
    for (auto elem : out) {
        normalPath += "/" + elem;
    }
    path = normalPath;
    return true;
}

// "/A/B/../C" -> "/A/C"
// ["A", "B", "..", "C"] -> ["A", "C"]
bool JsInitialize::PathVecToNormal(const std::vector<std::string> &in, std::vector<std::string> &out)
{
    for (auto elem : in) {
        if (elem == "..") {
            if (out.size() > 0) {
                out.pop_back();
            } else {
                return false;
            }
        } else if (elem != ".") {
            out.push_back(elem);
        }
    }
    return true;
}

// "/A/B//C" -> ["A", "B", "C"]
void JsInitialize::StringSplit(const std::string &str, const char delim, std::vector<std::string> &elems)
{
    std::stringstream stream(str);
    std::string item;
    while (std::getline(stream, item, delim)) {
        if (!item.empty()) {
            elems.push_back(item);
        }
    }
    return;
}

void JsInitialize::StringTrim(std::string &str)
{
    if (str.empty()) {
        return;
    }
    str.erase(0, str.find_first_not_of(" "));
    str.erase(str.find_last_not_of(" ") + 1);
    return;
}

bool JsInitialize::CheckBelongAppBaseDir(const std::string &filepath, std::string &baseDir)
{
    if (!JsInitialize::GetAppBaseDir(baseDir)) {
        return false;
    }
    if ((filepath.find(AREA1) == 0) || filepath.find(AREA2) == 0 || filepath.find(AREA5) == 0) {
        return true;
    } else {
        REQUEST_HILOGE("File dir not include base dir");
        return false;
    }
}

} // namespace OHOS::Request
