/*
* Copyright (c) 2024 Huawei Device Co., Ltd.
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

#ifndef REQUEST_BUNDLE_H
#define REQUEST_BUNDLE_H

#include <cstdint>
#include <string>

#include "bundle_mgr_interface.h"
#include "cxx.h"
#include "if_system_ability_manager.h"
#include "iremote_broker.h"
#include "iservice_registry.h"
#include "log.h"
#include "system_ability_definition.h"
#include "task/bundle.rs.h"

namespace OHOS::Request {
AppInfo GetNameAndIndex(int32_t uid);
} // namespace OHOS::Request
#endif // REQUEST_BUNDLE_H
