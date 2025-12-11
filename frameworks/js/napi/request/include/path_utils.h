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

#ifndef REQUEST_PATH_UTILS_H
#define REQUEST_PATH_UTILS_H

#include <sys/stat.h>

#include <cstdint>
#include <map>
#include <mutex>
#include <vector>

#include "request_common.h"

namespace OHOS::Request {
class PathUtils {
public:
    static bool AddPathsToMap(const std::string &path, const Action action);
    static bool SubPathsToMap(const std::string &path);
    static bool CheckBelongAppBaseDir(const std::string &filepath);
    static std::string ShieldPath(const std::string &path);
};
} // namespace OHOS::Request
#endif // PATH_UTILS