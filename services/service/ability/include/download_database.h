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

#ifndef DOWNLOAD_DATABASE_H
#define DOWNLOAD_DATABASE_H

#include <pthread.h>

#include "data_ability_predicates.h"
#include "rdb_errno.h"
#include "rdb_helper.h"
#include "rdb_open_callback.h"
#include "rdb_predicates.h"
#include "rdb_store.h"
#include "result_set.h"
#include "value_object.h"

namespace OHOS::Request::Download {
constexpr const char *DB_NAME = "download.db";
constexpr const char *TABLE_NAME = "downloadInfo";
constexpr int DATABASE_OPEN_VERSION = 1;
constexpr int DATABASE_NEW_VERSION = 2;

constexpr const char *CREATE_DOWNLOAD =
    "CREATE TABLE IF NOT EXISTS [downloadInfo]("
    "[taskid] INTEGER PRIMARY KEY AUTOINCREMENT, "
    "[url] TEXT, "
    "[description] TEXT, "
    "[title] TEXT, "
    "[filePath] TEXT, "
    "[header] TEXT,"
    "[metered] INTEGER, "
    "[roaming] INTEGER, "
    "[network] INTEGER, "
    "[status] INTEGER, "
    "[reason] INTEGER, "
    "[size] INTEGER, "
    "[mime] TEXT )";

class DownloadDataBase {
public:
    static std::shared_ptr<DownloadDataBase> GetInstance();
    static std::shared_ptr<OHOS::NativeRdb::RdbStore> store_;
    int64_t Insert(OHOS::NativeRdb::ValuesBucket insertValues);
    int Update(OHOS::NativeRdb::ValuesBucket values, OHOS::NativeRdb::RdbPredicates &rdbPredicates);
    int Delete(OHOS::NativeRdb::RdbPredicates &rdbPredicates);
    std::unique_ptr<OHOS::NativeRdb::AbsSharedResultSet> Query(
        OHOS::NativeRdb::RdbPredicates &rdbPredicates, std::vector<std::string> columns);
    int BeginTransaction();
    int Commit();
    int RollBack();

private:
    DownloadDataBase();
    DownloadDataBase(const DownloadDataBase &);
    const DownloadDataBase &operator=(const DownloadDataBase &);

private:
    static std::shared_ptr<DownloadDataBase> instance_;
};

class SqliteOpenHelperDownloadCallback : public OHOS::NativeRdb::RdbOpenCallback {
public:
    int OnCreate(OHOS::NativeRdb::RdbStore &rdbStore) override;
    int OnUpgrade(OHOS::NativeRdb::RdbStore &rdbStore, int oldVersion, int newVersion) override;
    int OnDowngrade(OHOS::NativeRdb::RdbStore &rdbStore, int currentVersion, int targetVersion) override;
};
} // namespace OHOS::Request::Download
#endif // DOWNLOAD_DATABASE_H