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

#include "download_database.h"

#include "constant.h"
#include "db_path.h"
#include "log.h"

namespace OHOS::Request::Download {
std::shared_ptr<DownloadDataBase> DownloadDataBase::instance_ = nullptr;
std::shared_ptr<OHOS::NativeRdb::RdbStore> DownloadDataBase::store_ = nullptr;
static std::string g_databaseName;

DownloadDataBase::DownloadDataBase()
{
    g_databaseName = DBPath::RDB_PATH + DB_NAME;
    DOWNLOAD_HILOGI("DownloadDataBase g_databaseName :%{public}s", g_databaseName.c_str());
    int errCode = OHOS::NativeRdb::E_OK;
    OHOS::NativeRdb::RdbStoreConfig config(g_databaseName);
    SqliteOpenHelperDownloadCallback sqliteOpenHelperCallback;
    store_ = OHOS::NativeRdb::RdbHelper::GetRdbStore(config, DATABASE_OPEN_VERSION, sqliteOpenHelperCallback, errCode);
    if (errCode != OHOS::NativeRdb::E_OK) {
        DOWNLOAD_HILOGE("DownloadDataBase errCode :%{public}d", errCode);
    } else {
        DOWNLOAD_HILOGI("DownloadDataBase errCode :%{public}d", errCode);
    }
}

std::shared_ptr<DownloadDataBase> DownloadDataBase::GetInstance()
{
    if (instance_ == nullptr) {
        instance_.reset(new DownloadDataBase());
        return instance_;
    }
    return instance_;
}

int DownloadDataBase::BeginTransaction()
{
    if (store_ == nullptr) {
        DOWNLOAD_HILOGE("DownloadDataBase BeginTransaction store_ is nullptr");
        return RDB_OBJECT_EMPTY;
    }
    int ret = store_->BeginTransaction();
    if (ret != OHOS::NativeRdb::E_OK) {
        DOWNLOAD_HILOGE("DownloadDataBase BeginTransaction fail :%{public}d", ret);
    }
    return ret;
}

int DownloadDataBase::Commit()
{
    if (store_ == nullptr) {
        DOWNLOAD_HILOGE(" DownloadDataBase Commit store_ is nullptr");
        return RDB_OBJECT_EMPTY;
    }
    int ret = store_->Commit();
    if (ret != OHOS::NativeRdb::E_OK) {
        DOWNLOAD_HILOGE(" DownloadDataBase Commit fail :%{public}d", ret);
    }
    return ret;
}

int DownloadDataBase::RollBack()
{
    if (store_ == nullptr) {
        DOWNLOAD_HILOGE(" DownloadDataBase RollBack store_ is nullptr");
        return RDB_OBJECT_EMPTY;
    }
    int ret = store_->RollBack();
    if (ret != OHOS::NativeRdb::E_OK) {
        DOWNLOAD_HILOGE(" DownloadDataBase RollBack fail :%{public}d", ret);
    }
    return ret;
}

/**
 * @brief Insert operation
 *
 * @param insertValues Conditions for update operation
 *
 * @return Insert operation results
 */
int64_t DownloadDataBase::Insert(OHOS::NativeRdb::ValuesBucket insertValues)
{
    int64_t outRowId = RDB_EXECUTE_FAIL;
    if (store_ == nullptr) {
        DOWNLOAD_HILOGE("DownloadDataBase Insert store_ is  nullptr");
        return RDB_OBJECT_EMPTY;
    }

    int ret = store_->Insert(outRowId, TABLE_NAME, insertValues);
    DOWNLOAD_HILOGI("DownloadDataBase Insert id = %{public}lld ", (long long)outRowId);
    if (ret != OHOS::NativeRdb::E_OK) {
        DOWNLOAD_HILOGE("DownloadDataBase Insert ret :%{public}d", ret);
        return RDB_EXECUTE_FAIL;
    }
    return outRowId;
}

/**
 * @brief Update operation
 *
 * @param values Conditions for update operation
 * @param predicates Conditions for update operation
 *
 * @return Update operation results
 */
int DownloadDataBase::Update(OHOS::NativeRdb::ValuesBucket values, OHOS::NativeRdb::RdbPredicates &predicates)
{
    if (store_ == nullptr) {
        DOWNLOAD_HILOGE("DownloadDataBase Update store_ is nullptr");
        return RDB_OBJECT_EMPTY;
    }

    int changeRow;
    int ret = store_->Update(changeRow, values, predicates);
    if (ret != OHOS::NativeRdb::E_OK) {
        DOWNLOAD_HILOGE("DownloadDataBase Update ret :%{public}d", ret);
        return RDB_EXECUTE_FAIL;
    }
    return ret;
}

/**
 * @brief Delete operation
 *
 * @param predicates Conditions for delete operation
 *
 * @return Delete operation results
 */
int DownloadDataBase::Delete(OHOS::NativeRdb::RdbPredicates &predicates)
{
    if (store_ == nullptr) {
        DOWNLOAD_HILOGE("DownloadDataBase Delete store_ is  nullptr");
        return RDB_OBJECT_EMPTY;
    }
    int deleteRow;
    int ret = store_->Delete(deleteRow, predicates);
    if (ret != OHOS::NativeRdb::E_OK) {
        DOWNLOAD_HILOGE("DownloadDataBase Delete ret :%{public}d", ret);
        return RDB_EXECUTE_FAIL;
    }
    return ret;
}

/**
 * @brief Query operation
 *
 * @param predicates Conditions for query operation
 * @param columns Conditions for query operation
 *
 * @return Query database results
 */
std::unique_ptr<OHOS::NativeRdb::AbsSharedResultSet> DownloadDataBase::Query(
    OHOS::NativeRdb::RdbPredicates &predicates, std::vector<std::string> columns)
{
    if (store_ == nullptr) {
        DOWNLOAD_HILOGE("DownloadDataBase Query store_ is  nullptr");
        return nullptr;
    }
    std::unique_ptr<OHOS::NativeRdb::AbsSharedResultSet> result = store_->Query(predicates, columns);
    return result;
}

int SqliteOpenHelperDownloadCallback::OnCreate(OHOS::NativeRdb::RdbStore &store)
{
    DOWNLOAD_HILOGD("Download DB OnCreat Enter");
    if (store.ExecuteSql(CREATE_DOWNLOAD) != OHOS::NativeRdb::E_OK) {
        DOWNLOAD_HILOGE("SqliteOpenHelperDownloadCallback create table error");
    }
    return OHOS::NativeRdb::E_OK;
}

int SqliteOpenHelperDownloadCallback::OnUpgrade(OHOS::NativeRdb::RdbStore &store, int oldVersion, int newVersion)
{
    return OHOS::NativeRdb::E_OK;
}

int SqliteOpenHelperDownloadCallback::OnDowngrade(OHOS::NativeRdb::RdbStore &store, int oldVersion, int newVersion)
{
    return OHOS::NativeRdb::E_OK;
}
} // namespace OHOS::Request::Download