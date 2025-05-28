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

#ifndef REQUEST_ANI_WRAPPER_H
#define REQUEST_ANI_WRAPPER_H

#include "application_context.h"
#include "context.h"
#include "cxx.h"
#include "storage_acl.h"

namespace OHOS::RequestAni {

int AclSetAccess(const rust::Str target, const rust::Str entry);
rust::String GetAppBaseDir();

} // namespace OHOS::RequestAni

#endif // REQUEST_ANI_WRAPPER_H
