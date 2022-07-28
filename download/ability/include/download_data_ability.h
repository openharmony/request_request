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

#ifndef DOWNLOAD_DATA_ABILITY_H
#define DOWNLOAD_DATA_ABILITY_H

#include "ability.h"
#include "ability_loader.h"
#include "want.h"

#include "download_database.h"

namespace OHOS::AppExecFwk {
class DownloadDataAbility : public Ability {
public:
    DownloadDataAbility();
    ~DownloadDataAbility();
    virtual int Insert(const Uri &uri, const NativeRdb::ValuesBucket &value) override;
    virtual int BatchInsert(const Uri &uri, const std::vector<NativeRdb::ValuesBucket> &values) override;
    virtual void OnStart(const Want &want) override;
    virtual int Update(const Uri &uri, const NativeRdb::ValuesBucket &value,
        const NativeRdb::DataAbilityPredicates &predicates) override;
    virtual int Delete(const Uri &uri, const NativeRdb::DataAbilityPredicates &predicates) override;
    virtual std::shared_ptr<NativeRdb::AbsSharedResultSet> Query(const Uri &uri,
        const std::vector<std::string> &columns, const NativeRdb::DataAbilityPredicates &predicates) override;
    virtual void Dump(const std::string &extra) override;

private:
    int UriParse(Uri &uri);
    int InsertExecute(const Uri &uri, const NativeRdb::ValuesBucket &value);
    void DataBaseNotifyChange(int code, Uri uri);
    bool IsBeginTransactionOK(int code, std::mutex &mutex);
    bool IsCommitOk(int code, std::mutex &mutex);

private:
    static std::shared_ptr<OHOS::Request::Download::DownloadDataBase> database_;
    static std::map<std::string, int> uriValueMap_;
};
} // namespace OHOS::AppExecFwk

#endif // DOWNLOAD_DATA_ABILITY_H