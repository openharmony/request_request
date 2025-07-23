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

#include "path_control.h"

#include <string>
#include <tuple>
#include <utility>

#include "log.h"
#include "storage_acl.h"

namespace OHOS::Request {

static constexpr int ACL_SUCC = 0;
static const std::string SA_PERMISSION_U_RW = "u:3815:rw";
static const std::string SA_PERMISSION_G_X = "g:3815:x";
static const std::string SA_PERMISSION_U_CLEAN = "u:3815:---";
static const std::string SA_PERMISSION_G_CLEAN = "g:3815:---";
static const std::string AREA1 = "/data/storage/el1/base";
static const std::string AREA2 = "/data/storage/el2/base";
static const std::string AREA5 = "/data/storage/el5/base";

static std::mutex pathMutex_;
static std::map<std::string, std::tuple<bool, uint32_t>> pathMap_;

static constexpr size_t TWO_OCTET = 2;
static constexpr size_t THREE_OCTET = 3;
static constexpr size_t FOUR_OCTET = 4;

bool PathControl::CheckBelongAppBaseDir(const std::string &filepath)
{
    return (filepath.find(AREA1) == 0) || filepath.find(AREA2) == 0 || filepath.find(AREA5) == 0;
}

// "/A/B/C" -> ["/A", "/A/B", "/A/B/C"]
std::vector<std::string> SplitPath(const std::string &path)
{
    std::vector<std::string> result;
    if (path.empty() || path[0] != '/') {
        return result;
    }

    result.reserve(std::count(path.begin(), path.end(), '/') + 1);
    std::string currentPath = "";
    size_t pos = 1;
    while (pos < path.size()) {
        size_t nextPos = path.find('/', pos);
        if (nextPos == std::string::npos) {
            nextPos = path.size();
        }
        if (nextPos > pos) {
            currentPath += ("/" + path.substr(pos, nextPos - pos));
            result.emplace_back(currentPath);
        }
        pos = nextPos + 1;
    }
    return result;
}

// ["/A", "/A/B", "/A/B/C"] -> [("/A", false), ("/A/B", false), ("/A/B/C", true)]
std::vector<std::pair<std::string, bool>> SelectPath(const std::vector<std::string> &paths)
{
    std::vector<std::pair<std::string, bool>> result;
    if (paths.empty())
        return result;

    for (const auto &elem : paths) {
        if (!PathControl::CheckBelongAppBaseDir(elem)) {
            continue;
        }
        result.emplace_back(elem, false);
    }

    if (!result.empty()) {
        result.back().second = true;
    }
    return result;
}

bool AddAcl(const std::string &path, const bool isFile)
{
    std::string entry;
    if (isFile) {
        entry = SA_PERMISSION_U_RW;
    } else {
        entry = SA_PERMISSION_G_X;
    }
    if (StorageDaemon::AclSetAccess(path, entry) != ACL_SUCC) {
        REQUEST_HILOGE("Add Acl Failed");
        return false;
    };
    return true;
}

bool SubAcl(const std::string &path, const bool isFile)
{
    std::string entry;
    if (isFile) {
        entry = SA_PERMISSION_U_CLEAN;
    } else {
        entry = SA_PERMISSION_G_CLEAN;
    }
    if (StorageDaemon::AclSetAccess(path, entry) != ACL_SUCC) {
        REQUEST_HILOGE("Sub Acl Failed");
        return false;
    };
    return true;
}

bool AddOnePathToMap(const std::string &path, const bool isFile)
{
    std::lock_guard<std::mutex> lockGuard(pathMutex_);
    auto it = pathMap_.find(path);
    if (it == pathMap_.end()) {
        if (!AddAcl(path, isFile)) {
            return false;
        }
        pathMap_.emplace(path, std::tuple(isFile, 1));
    } else {
        auto &[iFile, count] = it->second;
        iFile = isFile;
        count++;
    }
    return true;
}

bool SubOnePathToMap(const std::string &path, const bool isFile)
{
    std::lock_guard<std::mutex> lockGuard(pathMutex_);
    auto it = pathMap_.find(path);
    if (it == pathMap_.end()) {
        REQUEST_HILOGE("SubOnePathToMap no path");
        return false;
    }
    auto &[iFile, count] = it->second;
    if (count > 1) {
        count--;
        return true;
    }

    if (iFile != isFile) {
        REQUEST_HILOGE("SubOnePathToMap path changed");
    }
    if (!SubAcl(path, isFile)) {
        return false;
    }
    return true;
}

bool SubPathsVec(const std::vector<std::pair<std::string, bool>> &paths)
{
    for (auto &elem : paths) {
        if (!SubOnePathToMap(elem.first, elem.second)) {
            return false;
        }
    }
    return true;
}

bool PathControl::AddPathsToMap(const std::string &path)
{
    std::vector<std::pair<std::string, bool>> paths = SelectPath(SplitPath(path));
    if (paths.empty()) {
        return false;
    }
    std::vector<std::pair<std::string, bool>> completePaths;
    completePaths.reserve(paths.size());
    for (auto &elem : paths) {
        if (!AddOnePathToMap(elem.first, elem.second)) {
            SubPathsVec(completePaths);
            return false;
        }
        completePaths.emplace_back(elem);
    }
    return true;
}

bool PathControl::SubPathsToMap(const std::string &path)
{
    std::vector<std::pair<std::string, bool>> paths = SelectPath(SplitPath(path));
    if (paths.empty()) {
        return false;
    }
    return SubPathsVec(paths);
}
} // namespace OHOS::Request
