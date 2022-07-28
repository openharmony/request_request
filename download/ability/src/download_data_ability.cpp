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

#include "download_data_ability.h"

#include <mutex>

#include "ability_loader.h"
#include "common_event.h"
#include "constant.h"
#include "data_ability_predicates.h"
#include "db_path.h"
#include "file_utils.h"
#include "log.h"
#include "predicates_convert.h"
#include "rdb_predicates.h"
#include "sql_analyzer.h"
#include "uri_utils.h"


namespace OHOS::AppExecFwk {
REGISTER_AA(DownloadDataAbility);
namespace {
std::mutex g_mutex;
}
using namespace OHOS::Request::Download;

std::shared_ptr<DownloadDataBase> DownloadDataAbility::database_ = nullptr;
std::map<std::string, int> DownloadDataAbility::uriValueMap_ = {
    {"/com.ohos.download/download/downloadInfo", DOWNLOAD_INFO}};

DownloadDataAbility::DownloadDataAbility()
{
}

DownloadDataAbility::~DownloadDataAbility()
{
}

void DownloadDataAbility::Dump(const std::string &extra)
{
    DOWNLOAD_HILOGE("DownloadDataAbility ====>Dump:%{public}s", extra.c_str());
    FileUtils fileUtils;
    std::string dirStr = DBPath::DUMP_PATH;
    fileUtils.WriteStringToFileAppend(dirStr, extra);
}

void DownloadDataAbility::OnStart(const Want &want)
{
    DOWNLOAD_HILOGE("DownloadDataAbility OnStart");
    std::string basePath = GetDatabaseDir();
    DBPath::RDB_PATH = basePath + "/";
    DBPath::RDB_BACKUP_PATH = basePath + "/backup/";
    DBPath::DUMP_PATH = GetFilesDir() + "/";
}

int DownloadDataAbility::UriParse(Uri &uri)
{
    UriUtils uriUtils;
    int parseCode = uriUtils.UriParse(uri, uriValueMap_);
    return parseCode;
}

/**
 * @brief DownloadDataAbility BeginTransaction emptiness problems
 *
 * @param code the return number of BeginTransaction
 * @param mutex transmission parameter : lock
 *
 * @return BeginTransaction emptiness true or false
 */
bool DownloadDataAbility::IsBeginTransactionOK(int code, std::mutex &mutex)
{
    mutex.try_lock();
    if (code != 0) {
        DOWNLOAD_HILOGE("IsBeginTransactionOK fail");
        mutex.unlock();
        return false;
    }
    return true;
}

/**
 * @brief DownloadDataAbility Commit emptiness problems
 *
 * @param code the return number of Commit
 * @param mutex transmission parameter : lock
 *
 * @return Commit emptiness true or false
 */
bool DownloadDataAbility::IsCommitOk(int code, std::mutex &mutex)
{
    mutex.try_lock();
    if (code != 0) {
        DOWNLOAD_HILOGE("IsCommitOk fail");
        mutex.unlock();
        return false;
    }
    return true;
}

/**
 * @brief DownloadDataAbility Insert database
 *
 * @param uri Determine the data table name based on the URI
 * @param value Insert the data value of the database
 *
 * @return Insert database results code
 */
int DownloadDataAbility::Insert(const Uri &uri, const NativeRdb::ValuesBucket &value)
{
    DOWNLOAD_HILOGE("DownloadDataAbility Insert OnStart");
    SqlAnalyzer sqlAnalyzer;
    bool isOk = sqlAnalyzer.CheckValuesBucket(value);
    if (!isOk) {
        DOWNLOAD_HILOGE("DownloadDataAbility CheckValuesBucket is error");
        return RDB_EXECUTE_FAIL;
    }
    g_mutex.lock();
    database_ = DownloadDataBase::GetInstance();
    int ret = database_->BeginTransaction();
    if (!IsBeginTransactionOK(ret, g_mutex)) {
        g_mutex.unlock();
        return RDB_EXECUTE_FAIL;
    }
    int resultId = InsertExecute(uri, value);
    DOWNLOAD_HILOGE("DownloadDataAbility Insert id %{public}d", resultId);
    if (resultId == OPERATION_ERROR) {
        DOWNLOAD_HILOGE("DownloadDataAbility Insert error");
        database_->RollBack();
        g_mutex.unlock();
        return OPERATION_ERROR;
    }
    ret = database_->Commit();
    if (!IsCommitOk(ret, g_mutex)) {
        DOWNLOAD_HILOGE("DownloadDataAbility Insert error Commit");
        database_->RollBack();
        g_mutex.unlock();
        return RDB_EXECUTE_FAIL;
    }
    g_mutex.unlock();
    DataBaseNotifyChange(DOWNLOAD_INSERT, uri);
    return resultId;
}

int DownloadDataAbility::InsertExecute(const Uri &uri, const NativeRdb::ValuesBucket &value)
{
    int rowId = RDB_EXECUTE_FAIL;
    OHOS::Uri uriTemp = uri;
    int parseCode = UriParse(uriTemp);
    switch (parseCode) {
        case DOWNLOAD_INFO:
            rowId = database_->Insert(value);
            break;
        default:
            DOWNLOAD_HILOGE("DownloadDataAbility ====>no match uri action");
            break;
    }
    DOWNLOAD_HILOGI("DownloadDataAbility InsertExecute id = %{public}d", rowId);
    return rowId;
}

/**
 * @brief DownloadDataAbility BatchInsert database
 *
 * @param uri Determine the data table name based on the URI
 * @param value Insert the data values of the database
 *
 * @return Insert database results code
 */
int DownloadDataAbility::BatchInsert(const Uri &uri, const std::vector<NativeRdb::ValuesBucket> &values)
{
    int rowRet = RDB_EXECUTE_FAIL;
    int size = values.size();
    if (size <= 0) {
        DOWNLOAD_HILOGE("BatchInsert value is error");
        return rowRet;
    }
    g_mutex.lock();
    database_ = DownloadDataBase::GetInstance();
    int ret = database_->BeginTransaction();
    if (!IsBeginTransactionOK(ret, g_mutex)) {
        g_mutex.unlock();
        return RDB_EXECUTE_FAIL;
    }
    int count = 0;
    for (int i = 0; i < size; i++) {
        ++count;
        OHOS::NativeRdb::ValuesBucket rawValues = values[i];
        int code = InsertExecute(uri, rawValues);
        if (code == RDB_EXECUTE_FAIL) {
            database_->RollBack();
            g_mutex.unlock();
            return code;
        }
        if (count % TRANSACTION_COUNT == 0) {
            int markRet = database_->Commit();
            int beginRet = database_->BeginTransaction();
            if (!IsCommitOk(markRet, g_mutex) || !IsBeginTransactionOK(beginRet, g_mutex)) {
                database_->RollBack();
                g_mutex.unlock();
                return RDB_EXECUTE_FAIL;
            }
        }
    }
    int markRet = database_->Commit();
    if (!IsCommitOk(markRet, g_mutex)) {
        database_->RollBack();
        g_mutex.unlock();
        return RDB_EXECUTE_FAIL;
    }
    g_mutex.unlock();
    DataBaseNotifyChange(DOWNLOAD_INSERT, uri);
    return RDB_EXECUTE_OK;
}

/**
 * @brief DownloadDataAbility Update database
 *
 * @param uri Determine the data table name based on the URI
 * @param predicates Update the data value of the condition
 *
 * @return Update database results code
 */
int DownloadDataAbility::Update(
    const Uri &uri, const NativeRdb::ValuesBucket &value, const NativeRdb::DataAbilityPredicates &predicates)
{
    SqlAnalyzer sqlAnalyzer;
    bool isOk = sqlAnalyzer.CheckValuesBucket(value);
    if (!isOk) {
        DOWNLOAD_HILOGE("DownloadDataAbility CheckValuesBucket is error");
        return RDB_EXECUTE_FAIL;
    }
    g_mutex.lock();
    database_ = DownloadDataBase::GetInstance();
    PredicatesConvert predicatesConvert;
    int ret = RDB_EXECUTE_FAIL;
    OHOS::Uri uriTemp = uri;
    int parseCode = UriParse(uriTemp);
    OHOS::NativeRdb::DataAbilityPredicates dataAbilityPredicates = predicates;
    OHOS::NativeRdb::RdbPredicates rdbPredicates("");
    switch (parseCode) {
        case DOWNLOAD_INFO:
            rdbPredicates = predicatesConvert.ConvertPredicates(TABLE_NAME, dataAbilityPredicates);
            ret = database_->Update(value, rdbPredicates);
            break;
        default:
            DOWNLOAD_HILOGE("DownloadDataAbility ====>no match uri action");
            break;
    }
    g_mutex.unlock();
    DataBaseNotifyChange(DOWNLOAD_UPDATE, uri);
    return ret;
}

/**
 * @brief DownloadDataAbility Delete database
 *
 * @param uri Determine the data table name based on the URI
 * @param predicates Delete the data values of the condition
 *
 * @return Delete database results code
 */
int DownloadDataAbility::Delete(const Uri &uri, const NativeRdb::DataAbilityPredicates &predicates)
{
    g_mutex.lock();
    database_ = DownloadDataBase::GetInstance();
    PredicatesConvert predicatesConvert;
    int ret = RDB_EXECUTE_FAIL;
    OHOS::Uri uriTemp = uri;
    int parseCode = UriParse(uriTemp);
    OHOS::NativeRdb::DataAbilityPredicates dataAbilityPredicates = predicates;
    OHOS::NativeRdb::RdbPredicates rdbPredicates("");
    switch (parseCode) {
        case DOWNLOAD_INFO:
            rdbPredicates = predicatesConvert.ConvertPredicates(TABLE_NAME, dataAbilityPredicates);
            ret = database_->Delete(rdbPredicates);
            break;
        default:
            DOWNLOAD_HILOGE("DownloadDataAbility ====>no match uri action");
            break;
    }
    g_mutex.unlock();
    DataBaseNotifyChange(DOWNLOAD_DELETE, uri);
    return ret;
}

/**
 * @brief DownloadDataAbility Query database
 *
 * @param uri Determine the data table name based on the URI
 * @param columns Columns returned by query
 * @param predicates Query the data values of the condition
 *
 * @return Query database results
 */
std::shared_ptr<NativeRdb::AbsSharedResultSet> DownloadDataAbility::Query(
    const Uri &uri, const std::vector<std::string> &columns, const NativeRdb::DataAbilityPredicates &predicates)
{
    DOWNLOAD_HILOGI("DownloadDataAbility ====>Query start");
    database_ = DownloadDataBase::GetInstance();
    PredicatesConvert predicatesConvert;
    std::shared_ptr<NativeRdb::AbsSharedResultSet> result;
    OHOS::Uri uriTemp = uri;
    UriUtils uriUtils;
    int parseCode = uriUtils.UriParse(uriTemp, uriValueMap_);
    OHOS::NativeRdb::DataAbilityPredicates dataAbilityPredicates = predicates;
    OHOS::NativeRdb::RdbPredicates rdbPredicates("");
    std::vector<std::string> columnsTemp = columns;
    switch (parseCode) {
        case DOWNLOAD_INFO:
            rdbPredicates = predicatesConvert.ConvertPredicates(TABLE_NAME, dataAbilityPredicates);
            result = database_->Query(rdbPredicates, columnsTemp);
            break;
        default:
            DOWNLOAD_HILOGE("DownloadDataAbility ====>no match uri action");
            break;
    }
    std::shared_ptr<OHOS::NativeRdb::AbsSharedResultSet> sharedPtrResult = std::move(result);
    DOWNLOAD_HILOGI("DownloadDataAbility ====>Query end");
    return sharedPtrResult;
}

void DownloadDataAbility::DataBaseNotifyChange(int code, Uri uri)
{
    CommonEvent::SendChange(code);
}
} // namespace OHOS::AppExecFwk