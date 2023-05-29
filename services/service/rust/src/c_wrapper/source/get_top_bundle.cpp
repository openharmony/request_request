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

#include "get_top_bundle.h"

#include "ability_manager_client.h"
#include "log.h"

char *GetTopBundleName(void)
{
    REQUEST_HILOGE("GetTopBundleName start");
    OHOS::AppExecFwk::ElementName elementName = OHOS::AAFwk::AbilityManagerClient::GetInstance()->GetTopAbility();
    static std::string bundleName = elementName.GetBundleName();
    return const_cast<char *>(bundleName.c_str());
}