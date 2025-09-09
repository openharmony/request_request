/*
* Copyright (C) 2024 Huawei Device Co., Ltd.
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

#ifndef REQUEST_UTILS_H
#define REQUEST_UTILS_H

#include "cxx.h"

namespace OHOS::Request {

int GetForegroundAbilities(rust::vec<int> &uid);
rust::string GetCallingBundle(rust::u64 tokenId);
bool IsSystemAPI(uint64_t tokenId);
bool CheckPermission(uint64_t tokenId, rust::str permission);
bool PublishStateChangeEvent(rust::str bundleName, uint32_t taskId, int32_t state, int32_t uid);
int32_t UpdatePolicy(bool result);
bool IsCalledByHAP(uint32_t tokenId);

} // namespace OHOS::Request

#endif