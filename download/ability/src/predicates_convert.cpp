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

#include "predicates_convert.h"

namespace OHOS::Request::Download {
PredicatesConvert::PredicatesConvert(void)
{
}

PredicatesConvert::~PredicatesConvert()
{
}

OHOS::NativeRdb::RdbPredicates PredicatesConvert::ConvertPredicates(
    std::string tableName, OHOS::NativeRdb::DataAbilityPredicates &dataPredicates)
{
    OHOS::NativeRdb::RdbPredicates predicates(tableName);
    OHOS::NativeRdb::PredicatesUtils::SetWhereClauseAndArgs(
        &predicates, dataPredicates.GetWhereClause(), dataPredicates.GetWhereArgs());
    OHOS::NativeRdb::PredicatesUtils::SetAttributes(&predicates, dataPredicates.IsDistinct(), dataPredicates.GetIndex(),
        dataPredicates.GetGroup(), dataPredicates.GetOrder(), dataPredicates.GetLimit(), dataPredicates.GetOffset());
    return predicates;
}

OHOS::NativeRdb::RdbPredicates PredicatesConvert::CopyPredicates(
    std::string tableName, OHOS::NativeRdb::RdbPredicates &oldRdbPredicates)
{
    OHOS::NativeRdb::RdbPredicates predicates(tableName);
    OHOS::NativeRdb::PredicatesUtils::SetWhereClauseAndArgs(
        &predicates, oldRdbPredicates.GetWhereClause(), oldRdbPredicates.GetWhereArgs());
    OHOS::NativeRdb::PredicatesUtils::SetAttributes(&predicates, oldRdbPredicates.IsDistinct(),
        oldRdbPredicates.GetIndex(), oldRdbPredicates.GetGroup(), oldRdbPredicates.GetOrder(),
        oldRdbPredicates.GetLimit(), oldRdbPredicates.GetOffset());
    return predicates;
}
} // namespace OHOS::Request::Download