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

#ifndef REQUEST_UTILS_WRAPPER_H
#define REQUEST_UTILS_WRAPPER_H

#include <memory>

#include "ani.h"
#include "application_context.h"
#include "application_info.h"
#include "context.h"
#include "cxx.h"

namespace OHOS::Request {
using namespace OHOS::AbilityRuntime;

struct AniEnv;
struct AniObject;

rust::string GetCacheDir();

rust::string SHA256(rust::str input);

bool IsStageContext(AniEnv *env, AniObject *obj);

std::shared_ptr<Context> GetStageModeContext(AniEnv **env, AniObject *obj);

inline rust::string GetBundleName(std::shared_ptr<Context> const &context)
{
    return context->GetBundleName();
}

inline rust::string ContextGetCacheDir(std::shared_ptr<Context> const &context)
{
    return context->GetCacheDir();
}

inline rust::string ContextGetBaseDir(std::shared_ptr<Context> const &context)
{
    return context->GetBaseDir();
}

inline AppExecFwk::BundleType BundleType(std::shared_ptr<AppExecFwk::ApplicationInfo> const &info)
{
    return info->bundleType;
}

} // namespace OHOS::Request
#endif