/*
 * Copyright (c) 2021 Huawei Device Co., Ltd.
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

#ifndef PREDICATES_CONVERT_H
#define PREDICATES_CONVERT_H

#include "data_ability_predicates.h"
#include "predicates_utils.h"
#include "rdb_predicates.h"

namespace OHOS::Request::Download {
class PredicatesConvert {
public:
    PredicatesConvert();
    ~PredicatesConvert();
    OHOS::NativeRdb::RdbPredicates ConvertPredicates(
        std::string tableName, OHOS::NativeRdb::DataAbilityPredicates &dataAbilityPredicates);
    OHOS::NativeRdb::RdbPredicates CopyPredicates(
        std::string tableName, OHOS::NativeRdb::RdbPredicates &oldRdbPredicates);
};
} // namespace OHOS::Request::Download
#endif // PREDICATES_CONVERT_H