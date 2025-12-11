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

#include "request_utils_dataability.h"
#include "data_ability_helper.h"

namespace OHOS::Request {

int32_t DataAbilityOpenFile(std::shared_ptr<Context> const &context, const std::string &path)
{
    std::shared_ptr<Uri> uri = std::make_shared<Uri>(path);
    std::shared_ptr<AppExecFwk::DataAbilityHelper> dataAbilityHelper =
        AppExecFwk::DataAbilityHelper::Creator(context, uri);
    if (dataAbilityHelper == nullptr) {
        return -1;
    }
    return dataAbilityHelper->OpenFile(*uri, "r");
}

} // namespace OHOS::Request