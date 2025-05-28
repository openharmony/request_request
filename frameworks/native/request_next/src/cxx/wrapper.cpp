/*
 * Copyright (C) 2025 Huawei Device Co., Ltd.
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

#include "wrapper.h"

#include "base/request/request/common/include/log.h"
#include "cxx.h"
#include "wrapper.rs.h"

namespace OHOS::RequestAni {

int AclSetAccess(const rust::Str target, const rust::Str entry)
{
    std::string targetFile(target);
    std::string entryTxt(entry);
    return StorageDaemon::AclSetAccess(targetFile, entryTxt);
}

rust::String GetAppBaseDir()
{
    auto context = AbilityRuntime::Context::GetApplicationContext();
    if (context == nullptr) {
        return "";
    } else {
        return context->GetBaseDir();
    }
}
} // namespace OHOS::RequestAni
