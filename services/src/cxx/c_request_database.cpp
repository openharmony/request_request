/*
 * Copyright (c) 2023 Huawei Device Co., Ltd.
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

#include "c_request_database.h"

#include <securec.h>

#include <algorithm>
#include <cstdint>

#include "log.h"
#include "rdb_errno.h"

namespace OHOS::Request {
RequestDataBase::RequestDataBase()
{
    REQUEST_HILOGI("Process Get request database");
    int errCode = OHOS::NativeRdb::E_OK;
    OHOS::NativeRdb::RdbStoreConfig config(DB_NAME);
    config.SetSecurityLevel(NativeRdb::SecurityLevel::S1);
    config.SetEncryptStatus(true);
    RequestDBOpenCallback requestDBOpenCallback;
    // retry 10 times
    for (int index = 0; index < 10; ++index) {
        store_ = OHOS::NativeRdb::RdbHelper::GetRdbStore(config, DATABASE_VERSION, requestDBOpenCallback, errCode);
        if (store_ == nullptr) {
            REQUEST_HILOGE("GetRdbStore failed with reason: %{public}d, try DeleteRdbStore", errCode);
            OHOS::NativeRdb::RdbHelper::DeleteRdbStore(DB_NAME);
        } else {
            REQUEST_HILOGI("End get request database successful");
            return;
        }
    }
}

RequestDataBase &RequestDataBase::GetInstance()
{
    static RequestDataBase requestDataBase;
    return requestDataBase;
}

bool RequestDataBase::Insert(const std::string &table, const OHOS::NativeRdb::ValuesBucket &insertValues)
{
    if (store_ == nullptr) {
        return false;
    }

    int64_t outRowId = 0;
    int ret = store_->Insert(outRowId, table, insertValues);
    REQUEST_HILOGD("Request databases insert values, ret: %{public}d", ret);
    return ret == OHOS::NativeRdb::E_OK;
}

bool RequestDataBase::Update(
    const OHOS::NativeRdb::ValuesBucket values, const OHOS::NativeRdb::AbsRdbPredicates &predicates)
{
    if (store_ == nullptr) {
        return false;
    }

    int changedRows = 0;
    int ret = store_->Update(changedRows, values, predicates);
    REQUEST_HILOGD("Request databases update, changedRows: %{public}d, ret: %{public}d", changedRows, ret);
    return ret == OHOS::NativeRdb::E_OK;
}

std::shared_ptr<OHOS::NativeRdb::ResultSet> RequestDataBase::Query(
    const OHOS::NativeRdb::AbsRdbPredicates &predicates, const std::vector<std::string> &columns)
{
    if (store_ == nullptr) {
        return nullptr;
    }
    return store_->Query(predicates, columns);
}

bool RequestDataBase::Delete(const OHOS::NativeRdb::AbsRdbPredicates &predicates)
{
    if (store_ == nullptr) {
        return false;
    }

    int deletedRows = 0;
    int ret = store_->Delete(deletedRows, predicates);
    REQUEST_HILOGD("Request databases delete rows, rows: %{public}d, ret: %{public}d", ret, deletedRows);
    return ret == OHOS::NativeRdb::E_OK;
}

int RequestDBOpenCallback::OnCreate(OHOS::NativeRdb::RdbStore &store)
{
    return OHOS::NativeRdb::E_OK;
}

int RequestDBInitVersionTable(OHOS::NativeRdb::RdbStore &store)
{
    REQUEST_HILOGD("Inits version_table");
    // Clears `request_version` table first.
    int ret = store.ExecuteSql("DELETE FROM request_version");
    if (ret != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("Clears request_version table failed with reason: %{public}d", ret);
        return ret;
    }

    int64_t outRowId = 0;
    OHOS::NativeRdb::ValuesBucket insertValues;
    insertValues.PutString("version", std::string(REQUEST_DATABASE_VERSION));
    insertValues.PutString("task_table", std::string(REQUEST_TASK_TABLE_NAME));
    ret = store.Insert(outRowId, std::string("request_version"), insertValues);
    if (ret != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("Inits request_version table failed with reason: %{public}d", ret);
        return ret;
    }
    REQUEST_HILOGD("Inits version_table success");
    return ret;
}

int RequestDBDropTable(OHOS::NativeRdb::RdbStore &store, const char *name)
{
    return store.ExecuteSql(std::string("DROP TABLE IF EXISTS ") + name);
}

void RequestDBRemoveOldTables(OHOS::NativeRdb::RdbStore &store)
{
    REQUEST_HILOGD("Begins removing old tables");

    // These two tables followed was defined in 4.0-release.
    if (RequestDBDropTable(store, "request_task_info") != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("Removes request_task_info table failed");
    }

    if (RequestDBDropTable(store, "task_info_attachment") != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("Removes task_info_attachment table failed");
    }

    // These four tables followed was defined in 4.1-beta.
    if (RequestDBDropTable(store, "request_task_config") != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("Removes request_task_config table failed");
    }

    if (RequestDBDropTable(store, "task_config_attachment") != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("Removes task_config_attachment table failed");
    }

    if (RequestDBDropTable(store, "priority_table") != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("Removes priority_table table failed");
    }

    if (RequestDBDropTable(store, "certs_table") != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("Removes certs_table table failed");
    }

    REQUEST_HILOGD("Removes old tables end");
}

int RequestDBCheckVersion(OHOS::NativeRdb::RdbStore &store)
{
    REQUEST_HILOGD("RequestDBCheckVersion in");
    auto resultSet = store.QuerySql(CHECK_REQUEST_VERSION);
    if (resultSet == nullptr) {
        return CHECK_VERSION_FAILED;
    }
    int rowCount = 0;
    int ret = resultSet->GetRowCount(rowCount);
    if (ret != OHOS::NativeRdb::E_OK || rowCount > 1) {
        REQUEST_HILOGE("Gets rowCount failed, GetRowCount ret: %{public}d, rowCount: %{public}d", ret, rowCount);
        return CHECK_VERSION_FAILED;
    }

    if (rowCount == 0) {
        return WITHOUT_VERSION_TABLE;
    }

    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_version");
    resultSet = store.Query(rdbPredicates, { "version", "task_table" });
    if (resultSet == nullptr) {
        return CHECK_VERSION_FAILED;
    }

    ret = resultSet->GetRowCount(rowCount);
    if (ret != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("Gets rowCount failed, GetRowCount ret: %{public}d", ret);
        return CHECK_VERSION_FAILED;
    }

    if (rowCount == 0 || rowCount > 1) {
        return INVALID_VERSION;
    }

    ret = resultSet->GoToRow(0);
    if (ret != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("ResultSet goes to first row failed, GoToRow ret: %{public}d", ret);
        return CHECK_VERSION_FAILED;
    }

    std::string version = "";
    ret = resultSet->GetString(0, version);
    if (ret != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("ResultSet gets version failed, GetString ret: %{public}d", ret);
        return CHECK_VERSION_FAILED;
    }

    REQUEST_HILOGI("request database version: %{public}s", version.c_str());

    if (version == REQUEST_DATABASE_VERSION_4_1_RELEASE) {
        return API11_4_1_RELEASE;
    }
    if (version == REQUEST_DATABASE_VERSION) {
        return API12_5_0_RELEASE;
    }

    return INVALID_VERSION;
}

int RequestDBCreateTables(OHOS::NativeRdb::RdbStore &store)
{
    // Creates request_version table first.
    int ret = store.ExecuteSql(CREATE_REQUEST_VERSION_TABLE);
    if (ret != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("Creates request_version table failed, ret: %{public}d", ret);
        return ret;
    }
    REQUEST_HILOGI("Creates request_version table success");

    // ..then creates request_task table.
    ret = store.ExecuteSql(CREATE_REQUEST_TASK_TABLE);
    if (ret != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("Creates request_task table failed, ret: %{public}d", ret);
        return ret;
    }
    REQUEST_HILOGI("Creates request_task table success");
    return ret;
}

// Keeps this function for possible extensions later
int RequestDBUpgradeFrom41(OHOS::NativeRdb::RdbStore &store)
{
    int ret = store.ExecuteSql(REQUEST_TASK_TABLE_ADD_PROXY);
    if (ret != OHOS::NativeRdb::E_OK && ret != OHOS::NativeRdb::E_SQLITE_ERROR) {
        REQUEST_HILOGE("add column proxy failed, ret: %{public}d", ret);
        return ret;
    }

    ret = store.ExecuteSql(REQUEST_TASK_TABLE_ADD_CERTIFICATE_PINS);
    if (ret != OHOS::NativeRdb::E_OK && ret != OHOS::NativeRdb::E_SQLITE_ERROR) {
        REQUEST_HILOGE("add column certificate_pins failed, ret: %{public}d", ret);
        return ret;
    }
    return OHOS::NativeRdb::E_OK;
}

// This function is used to adapt beta version, remove it later.
void RequestDBUpgradeFrom50(OHOS::NativeRdb::RdbStore &store)
{
    // Ignores these error if these columns already exists.
    store.ExecuteSql(REQUEST_TASK_TABLE_ADD_PROXY);
    store.ExecuteSql(REQUEST_TASK_TABLE_ADD_CERTIFICATE_PINS);
}

int RequestDBUpgrade(OHOS::NativeRdb::RdbStore &store)
{
    REQUEST_HILOGD("Begins upgrading database");

    int res;
    int version = RequestDBCheckVersion(store);
    switch (version) {
        case INVALID_VERSION: {
            REQUEST_HILOGI("Upgrading database from invaliad version");
            RequestDBRemoveOldTables(store);
        }
            [[fallthrough]];
        case WITHOUT_VERSION_TABLE: {
            REQUEST_HILOGI("Upgrading database from 4.0 or earlier");
            res = RequestDBCreateTables(store);
            if (res != OHOS::NativeRdb::E_OK) {
                return res;
            }
        }
            [[fallthrough]];
        case API11_4_1_RELEASE: {
            REQUEST_HILOGI("Upgrading database from 4.1-Release");
            res = RequestDBUpgradeFrom41(store);
            if (res != OHOS::NativeRdb::E_OK) {
                return res;
            }
        }
            [[fallthrough]];
        case API12_5_0_RELEASE: {
            REQUEST_HILOGI("Version is 5.0-release, no need to update database.");
            RequestDBUpgradeFrom50(store);
            break;
        }
        default: {
            REQUEST_HILOGI("Checks version failed, cannot update request database.");
            return OHOS::NativeRdb::E_ERROR;
        }
    }
    if (version != API12_5_0_RELEASE) {
        return RequestDBInitVersionTable(store);
    }
    return 0;
}

void RequestDBUpdateInvalidRecords(OHOS::NativeRdb::RdbStore &store)
{
    REQUEST_HILOGI("Updates all invalid task to failed");

    OHOS::NativeRdb::ValuesBucket values;
    values.PutInt("state", static_cast<uint8_t>(State::FAILED));

    // Tasks in `WAITING` and `PAUSED` states need to be resumed,
    // so they are not processed.
    int changedRows = 0;
    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task");
    rdbPredicates.EqualTo("state", static_cast<uint8_t>(State::RUNNING))
        ->Or()
        ->EqualTo("state", static_cast<uint8_t>(State::RETRYING))
        ->Or()
        ->EqualTo("state", static_cast<uint8_t>(State::CREATED));

    if (store.Update(changedRows, values, rdbPredicates) != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("Updates all invalid task to `FAILED` state failed");
        return;
    }
    REQUEST_HILOGI("Updates all invalid task to `FAILED` state success");
    return;
}

int RequestDBOpenCallback::OnOpen(OHOS::NativeRdb::RdbStore &store)
{
    int ret = RequestDBUpgrade(store);
    if (ret != 0) {
        REQUEST_HILOGE("database upgrade failed with reason: %{public}d", ret);
    }
    RequestDBUpdateInvalidRecords(store);
    return ret;
}

int RequestDBOpenCallback::OnUpgrade(OHOS::NativeRdb::RdbStore &store, int oldVersion, int newVersion)
{
    return OHOS::NativeRdb::E_OK;
}

int RequestDBOpenCallback::OnDowngrade(OHOS::NativeRdb::RdbStore &store, int oldVersion, int newVersion)
{
    return OHOS::NativeRdb::E_OK;
}
} // namespace OHOS::Request

namespace {
std::vector<uint8_t> CFormItemToBlob(const CFormItem *cpointer, uint32_t length)
{
    std::vector<uint8_t> blob;
    for (uint32_t i = 0; i < length; ++i) {
        const CFormItem &obj = cpointer[i];
        const uint8_t *objBytes = reinterpret_cast<const uint8_t *>(&obj);
        blob.insert(blob.end(), objBytes, objBytes + sizeof(CFormItem));
        blob.insert(blob.end(), obj.name.cStr, obj.name.cStr + obj.name.len);
        blob.insert(blob.end(), obj.value.cStr, obj.value.cStr + obj.value.len);
    }
    return blob;
}

std::vector<CFormItem> BlobToCFormItem(const std::vector<uint8_t> &blob)
{
    std::vector<CFormItem> vec;
    size_t position = 0;
    while (position < blob.size()) {
        CFormItem obj;
        memcpy_s(&obj, sizeof(CFormItem), blob.data() + position, sizeof(CFormItem));
        position += sizeof(CFormItem);

        obj.name.cStr = new char[obj.name.len];
        memcpy_s(obj.name.cStr, obj.name.len, blob.data() + position, obj.name.len);
        position += obj.name.len;

        obj.value.cStr = new char[obj.value.len];
        memcpy_s(obj.value.cStr, obj.value.len, blob.data() + position, obj.value.len);
        position += obj.value.len;

        vec.push_back(obj);
    }
    return vec;
}

std::vector<uint8_t> CFileSpecToBlob(const CFileSpec *cpointer, uint32_t length)
{
    std::vector<uint8_t> blob;
    for (uint32_t i = 0; i < length; ++i) {
        const CFileSpec &obj = cpointer[i];
        const uint8_t *objBytes = reinterpret_cast<const uint8_t *>(&obj);
        blob.insert(blob.end(), objBytes, objBytes + sizeof(CFileSpec));
        blob.insert(blob.end(), obj.name.cStr, obj.name.cStr + obj.name.len);
        blob.insert(blob.end(), obj.path.cStr, obj.path.cStr + obj.path.len);
        blob.insert(blob.end(), obj.fileName.cStr, obj.fileName.cStr + obj.fileName.len);
        blob.insert(blob.end(), obj.mimeType.cStr, obj.mimeType.cStr + obj.mimeType.len);
        blob.emplace_back(obj.is_user_file);
    }
    return blob;
}

std::vector<CFileSpec> BlobToCFileSpec(const std::vector<uint8_t> &blob)
{
    std::vector<CFileSpec> vec;
    size_t position = 0;
    while (position < blob.size()) {
        CFileSpec obj;
        memcpy_s(&obj, sizeof(CFileSpec), blob.data() + position, sizeof(CFileSpec));
        position += sizeof(CFileSpec);

        obj.name.cStr = new char[obj.name.len];
        memcpy_s(obj.name.cStr, obj.name.len, blob.data() + position, obj.name.len);
        position += obj.name.len;

        obj.path.cStr = new char[obj.path.len];
        memcpy_s(obj.path.cStr, obj.path.len, blob.data() + position, obj.path.len);
        position += obj.path.len;

        obj.fileName.cStr = new char[obj.fileName.len];
        memcpy_s(obj.fileName.cStr, obj.fileName.len, blob.data() + position, obj.fileName.len);
        position += obj.fileName.len;

        obj.mimeType.cStr = new char[obj.mimeType.len];
        memcpy_s(obj.mimeType.cStr, obj.mimeType.len, blob.data() + position, obj.mimeType.len);
        position += obj.mimeType.len;

        obj.is_user_file = blob[position];
        position += 1;

        vec.push_back(obj);
    }
    return vec;
}

std::vector<uint8_t> CEachFileStatusToBlob(const CEachFileStatus *cpointer, uint32_t length)
{
    std::vector<uint8_t> blob;
    for (uint32_t i = 0; i < length; ++i) {
        const CEachFileStatus &obj = cpointer[i];
        const uint8_t *objBytes = reinterpret_cast<const uint8_t *>(&obj);
        blob.insert(blob.end(), objBytes, objBytes + sizeof(CEachFileStatus));
        blob.insert(blob.end(), obj.path.cStr, obj.path.cStr + obj.path.len);
        blob.insert(blob.end(), &obj.reason, &obj.reason + sizeof(uint8_t));
        blob.insert(blob.end(), obj.message.cStr, obj.message.cStr + obj.message.len);
    }
    return blob;
}

std::vector<CEachFileStatus> BlobToCEachFileStatus(const std::vector<uint8_t> &blob)
{
    std::vector<CEachFileStatus> vec;
    size_t position = 0;
    while (position < blob.size()) {
        CEachFileStatus obj;
        memcpy_s(&obj, sizeof(CEachFileStatus), blob.data() + position, sizeof(CEachFileStatus));
        position += sizeof(CEachFileStatus);

        obj.path.cStr = new char[obj.path.len];
        memcpy_s(obj.path.cStr, obj.path.len, blob.data() + position, obj.path.len);
        position += obj.path.len;

        memcpy_s(&obj.reason, sizeof(uint8_t), blob.data() + position, sizeof(uint8_t));
        position += sizeof(uint8_t);

        obj.message.cStr = new char[obj.message.len];
        memcpy_s(obj.message.cStr, obj.message.len, blob.data() + position, obj.message.len);
        position += obj.message.len;

        vec.push_back(obj);
    }
    return vec;
}

std::vector<uint8_t> CStringToBlob(const CStringWrapper *cpointer, uint32_t length)
{
    std::vector<uint8_t> blob;
    for (uint32_t i = 0; i < length; ++i) {
        const CStringWrapper &obj = cpointer[i];
        blob.push_back(static_cast<uint8_t>(obj.len));
        blob.insert(blob.end(), obj.cStr, obj.cStr + obj.len);
    }
    return blob;
}

std::vector<std::string> BlobToStringVec(const std::vector<uint8_t> &blob)
{
    std::vector<std::string> vec;
    uint32_t position = 0;
    while (position < blob.size()) {
        uint32_t len = static_cast<uint32_t>(blob[position++]);
        std::string str(blob.begin() + position, blob.begin() + position + len);
        position += len;

        vec.push_back(std::move(str));
    }

    return vec;
}

// convert vector<CFormItem> to vector<FormItem>
std::vector<FormItem> VecToFormItem(const std::vector<CFormItem> &cvec)
{
    std::vector<FormItem> vec;
    for (const CFormItem &obj : cvec) {
        FormItem formItem;
        formItem.name = std::string(obj.name.cStr, obj.name.len);
        formItem.value = std::string(obj.value.cStr, obj.value.len);
        vec.push_back(std::move(formItem));
    }
    return vec;
}

// convert vector<CFileSpec> to vector<FileSpec>
std::vector<FileSpec> VecToFileSpec(const std::vector<CFileSpec> &cvec)
{
    std::vector<FileSpec> vec;
    for (const CFileSpec &obj : cvec) {
        FileSpec fileSpec;
        fileSpec.name = std::string(obj.name.cStr, obj.name.len);
        fileSpec.path = std::string(obj.path.cStr, obj.path.len);
        fileSpec.fileName = std::string(obj.fileName.cStr, obj.fileName.len);
        fileSpec.mimeType = std::string(obj.mimeType.cStr, obj.mimeType.len);
        fileSpec.is_user_file = obj.is_user_file;
        vec.push_back(std::move(fileSpec));
    }
    return vec;
}

// convert vector<CEachFileStatus> to vector<EachFileStatus>
std::vector<EachFileStatus> VecToEachFileStatus(const std::vector<CEachFileStatus> &cvec)
{
    std::vector<EachFileStatus> vec;
    for (const CEachFileStatus &obj : cvec) {
        EachFileStatus eachFileStatus;
        eachFileStatus.path = std::string(obj.path.cStr, obj.path.len);
        eachFileStatus.reason = obj.reason;
        eachFileStatus.message = std::string(obj.message.cStr, obj.message.len);
        vec.push_back(std::move(eachFileStatus));
    }
    return vec;
}

template<typename T> bool WriteUpdateData(OHOS::NativeRdb::ValuesBucket &insertValues, T *info)
{
    std::vector<uint8_t> eachFileStatusBlob = CEachFileStatusToBlob(info->eachFileStatusPtr, info->eachFileStatusLen);
    // write to insertValues
    insertValues.PutString("mime_type", std::string(info->mimeType.cStr, info->mimeType.len));
    insertValues.PutInt("state", info->progress.commonData.state);
    insertValues.PutLong("idx", info->progress.commonData.index);
    insertValues.PutLong("total_processed", info->progress.commonData.totalProcessed);
    insertValues.PutString("sizes", std::string(info->progress.sizes.cStr, info->progress.sizes.len));
    insertValues.PutString("processed", std::string(info->progress.processed.cStr, info->progress.processed.len));
    insertValues.PutString("extras", std::string(info->progress.extras.cStr, info->progress.extras.len));
    insertValues.PutBlob("each_file_status", eachFileStatusBlob);
    return true;
}

bool WriteMutableData(OHOS::NativeRdb::ValuesBucket &insertValues, CTaskInfo *taskInfo, CTaskConfig *taskConfig)
{
    insertValues.PutLong("mtime", taskInfo->commonData.mtime);
    insertValues.PutInt("reason", taskInfo->commonData.reason);
    insertValues.PutLong("tries", taskInfo->commonData.tries);
    if (!WriteUpdateData(insertValues, taskInfo)) {
        return false;
    }
    // write vectors
    insertValues.PutBlob("form_items", CFormItemToBlob(taskConfig->formItemsPtr, taskConfig->formItemsLen));
    insertValues.PutBlob("file_specs", CFileSpecToBlob(taskConfig->fileSpecsPtr, taskConfig->fileSpecsLen));
    insertValues.PutBlob("body_file_names", CStringToBlob(taskConfig->bodyFileNamesPtr, taskConfig->bodyFileNamesLen));
    insertValues.PutBlob("certs_paths", CStringToBlob(taskConfig->certsPathPtr, taskConfig->certsPathLen));
    return true;
}

inline int64_t GetLong(std::shared_ptr<OHOS::NativeRdb::ResultSet> resultSet, int line)
{
    int64_t value = 0;
    resultSet->GetLong(line, value);
    return value;
}

inline int GetInt(std::shared_ptr<OHOS::NativeRdb::ResultSet> resultSet, int line)
{
    int value = 0;
    resultSet->GetInt(line, value);
    return value;
}

void FillCommonTaskInfo(std::shared_ptr<OHOS::NativeRdb::ResultSet> set, TaskInfo &info)
{
    info.commonData.taskId = static_cast<uint32_t>(GetLong(set, 0));    // Line 0 is 'task_id'
    info.commonData.uid = static_cast<uint64_t>(GetLong(set, 1));       // Line 1 is 'uid'
    info.commonData.action = static_cast<uint8_t>(GetInt(set, 2));      // Line 2 is 'action'
    info.commonData.mode = static_cast<uint8_t>(GetInt(set, 3));        // Line 3 is 'mode'
    info.commonData.ctime = static_cast<uint64_t>(GetLong(set, 4));     // Line 4 is 'ctime'
    info.commonData.mtime = static_cast<uint64_t>(GetLong(set, 5));     // Line 5 is 'mtime'
    info.commonData.reason = static_cast<uint8_t>(GetInt(set, 6));      // Line 6 is 'reason'
    info.commonData.gauge = static_cast<bool>(GetInt(set, 7));          // Line 7 is 'gauge'
    info.commonData.retry = static_cast<bool>(GetInt(set, 8));          // Line 8 is 'retry'
    info.commonData.tries = static_cast<uint64_t>(GetLong(set, 9));     // Line 9 is 'tries'
    info.commonData.version = static_cast<uint8_t>(GetLong(set, 10));   // Line 10 is 'version'
    info.commonData.priority = static_cast<uint32_t>(GetLong(set, 11)); // Line 11 is 'priority'
}

void FillOtherTaskInfo(std::shared_ptr<OHOS::NativeRdb::ResultSet> set, TaskInfo &info)
{
    set->GetString(12, info.bundle);      // Line 12 is 'bundle'
    set->GetString(13, info.url);         // Line 13 is 'url'
    set->GetString(14, info.data);        // Line 14 is 'data'
    set->GetString(15, info.token);       // Line 15 is 'token'
    set->GetString(16, info.title);       // Line 16 is 'title'
    set->GetString(17, info.description); // Line 17 is 'description'
    set->GetString(18, info.mimeType);    // Line 18 is 'mime_type'

    info.progress.commonData.state = static_cast<uint8_t>(GetInt(set, 19));  // Line 19 here is 'state'
    info.progress.commonData.index = static_cast<uint8_t>(GetLong(set, 20)); // Line 20 here is 'idx'
    uintptr_t totalProcessed = static_cast<uintptr_t>(GetLong(set, 21));     // Line 21 is 'totalProcessed'
    info.progress.commonData.totalProcessed = totalProcessed;

    set->GetString(22, info.progress.sizes);     // Line 22 here is 'sizes'
    set->GetString(23, info.progress.processed); // Line 23 here is 'processed'
    set->GetString(24, info.progress.extras);    // Line 24 here is 'extras'

    std::vector<uint8_t> formItemsBlob;
    std::vector<uint8_t> formSpecsBlob;
    std::vector<uint8_t> eachFileStatusBlob;

    set->GetBlob(25, formItemsBlob); // Line 25 is 'form_items'
    info.formItems = VecToFormItem(BlobToCFormItem(formItemsBlob));
    set->GetBlob(26, formSpecsBlob); // Line 26 is 'file_specs'
    info.fileSpecs = VecToFileSpec(BlobToCFileSpec(formSpecsBlob));
    set->GetBlob(27, eachFileStatusBlob); // Line 27 is 'each_file_status'
    info.eachFileStatus = VecToEachFileStatus(BlobToCEachFileStatus(eachFileStatusBlob));
}

CProgress BuildCProgress(const Progress &progress)
{
    return CProgress{
        .commonData = progress.commonData,
        .sizes = WrapperCString(progress.sizes),
        .processed = WrapperCString(progress.processed),
        .extras = WrapperCString(progress.extras),
    };
}

CTaskInfo *BuildCTaskInfo(const TaskInfo &taskInfo)
{
    uint32_t formItemsLen = taskInfo.formItems.size();
    CFormItem *formItemsPtr = new CFormItem[formItemsLen];
    for (uint32_t i = 0; i < formItemsLen; i++) {
        formItemsPtr[i].name = WrapperCString(taskInfo.formItems[i].name);
        formItemsPtr[i].value = WrapperCString(taskInfo.formItems[i].value);
    }

    uint32_t fileSpecsLen = taskInfo.fileSpecs.size();
    CFileSpec *fileSpecsPtr = new CFileSpec[fileSpecsLen];
    CEachFileStatus *eachFileStatusPtr = new CEachFileStatus[fileSpecsLen];
    for (uint32_t i = 0; i < fileSpecsLen; i++) {
        fileSpecsPtr[i].name = WrapperCString(taskInfo.fileSpecs[i].name);
        fileSpecsPtr[i].path = WrapperCString(taskInfo.fileSpecs[i].path);
        fileSpecsPtr[i].fileName = WrapperCString(taskInfo.fileSpecs[i].fileName);
        fileSpecsPtr[i].mimeType = WrapperCString(taskInfo.fileSpecs[i].mimeType);
        fileSpecsPtr[i].is_user_file = taskInfo.fileSpecs[i].is_user_file;
        eachFileStatusPtr[i].path = WrapperCString(taskInfo.eachFileStatus[i].path);
        eachFileStatusPtr[i].reason = taskInfo.eachFileStatus[i].reason;
        eachFileStatusPtr[i].message = WrapperCString(taskInfo.eachFileStatus[i].message);
    }

    CTaskInfo *cTaskInfo = new CTaskInfo;
    cTaskInfo->bundle = WrapperCString(taskInfo.bundle);
    cTaskInfo->url = WrapperCString(taskInfo.url);
    cTaskInfo->data = WrapperCString(taskInfo.data);
    cTaskInfo->token = WrapperCString(taskInfo.token);
    cTaskInfo->formItemsPtr = formItemsPtr;
    cTaskInfo->formItemsLen = formItemsLen;
    cTaskInfo->fileSpecsPtr = fileSpecsPtr;
    cTaskInfo->fileSpecsLen = fileSpecsLen;
    cTaskInfo->title = WrapperCString(taskInfo.title);
    cTaskInfo->description = WrapperCString(taskInfo.description);
    cTaskInfo->mimeType = WrapperCString(taskInfo.mimeType);
    cTaskInfo->progress = BuildCProgress(taskInfo.progress);
    cTaskInfo->eachFileStatusPtr = eachFileStatusPtr;
    cTaskInfo->eachFileStatusLen = fileSpecsLen;
    cTaskInfo->commonData = taskInfo.commonData;
    return cTaskInfo;
}

void BuildRequestTaskConfigWithLong(std::shared_ptr<OHOS::NativeRdb::ResultSet> set, TaskConfig &config)
{
    config.commonData.taskId = static_cast<uint32_t>(GetLong(set, 0));    // Line 0 is 'task_id'
    config.commonData.uid = static_cast<uint64_t>(GetLong(set, 1));       // Line 1 is 'uid'
    config.commonData.tokenId = static_cast<uint64_t>(GetLong(set, 2));   // Line 2 is 'token_id'
    config.commonData.index = static_cast<uint32_t>(GetLong(set, 11));    // Line 11 is 'config_idx'
    config.commonData.begins = static_cast<uint64_t>(GetLong(set, 12));   // Line 12 is 'begins'
    config.commonData.ends = static_cast<int64_t>(GetLong(set, 13));      // Line 13 is 'ends'
    config.commonData.priority = static_cast<uint32_t>(GetLong(set, 16)); // Line 16 is 'priority'
}

void BuildRequestTaskConfigWithInt(std::shared_ptr<OHOS::NativeRdb::ResultSet> set, TaskConfig &config)
{
    config.commonData.action = static_cast<uint8_t>(GetInt(set, 3));   // Line 3 is 'action'
    config.commonData.mode = static_cast<uint8_t>(GetInt(set, 4));     // Line 4 is 'mode'
    config.commonData.cover = static_cast<bool>(GetInt(set, 5));       // Line 5 is 'cover'
    config.commonData.network = static_cast<uint8_t>(GetInt(set, 6));  // Line 6 is 'network'
    config.commonData.metered = static_cast<bool>(GetInt(set, 7));     // Line 7 is 'metered'
    config.commonData.roaming = static_cast<bool>(GetInt(set, 8));     // Line 8 is 'roaming'
    config.commonData.retry = static_cast<bool>(GetInt(set, 9));       // Line 9 is 'retry'
    config.commonData.redirect = static_cast<bool>(GetInt(set, 10));   // Line 10 is 'redirect'
    config.commonData.gauge = static_cast<bool>(GetInt(set, 14));      // Line 14 is 'gauge'
    config.commonData.precise = static_cast<bool>(GetInt(set, 15));    // Line 15 is 'precise'
    config.commonData.background = static_cast<bool>(GetInt(set, 17)); // Line 17 is 'background'
    config.version = static_cast<uint8_t>(GetInt(set, 27));            // Line 27 here is 'version'
}

void BuildRequestTaskConfigWithString(std::shared_ptr<OHOS::NativeRdb::ResultSet> set, TaskConfig &config)
{
    set->GetString(18, config.bundle);          // Line 18 is 'bundle'
    set->GetString(19, config.url);             // Line 19 is 'url'
    set->GetString(20, config.title);           // Line 20 is 'title'
    set->GetString(21, config.description);     // Line 21 is 'description'
    set->GetString(22, config.method);          // Line 22 is 'method'
    set->GetString(23, config.headers);         // Line 23 is 'headers'
    set->GetString(24, config.data);            // Line 24 is 'data'
    set->GetString(25, config.token);           // Line 25 is 'token'
    set->GetString(26, config.extras);          // Line 26 is 'config_extras'
    set->GetString(32, config.proxy);           // Line 32 is 'proxy'
    set->GetString(33, config.certificatePins); // Line 33 is 'certificate_pins'
}

void BuildRequestTaskConfigWithBlob(std::shared_ptr<OHOS::NativeRdb::ResultSet> set, TaskConfig &config)
{
    std::vector<uint8_t> formItemsBlob;
    std::vector<uint8_t> formSpecsBlob;
    std::vector<uint8_t> bodyFileNamesBlob;
    std::vector<uint8_t> certsPathsBlob;

    set->GetBlob(28, formItemsBlob); // Line 28 is 'form_items'
    config.formItems = VecToFormItem(BlobToCFormItem(formItemsBlob));
    set->GetBlob(29, formSpecsBlob); // Line 29 is 'file_specs'
    config.fileSpecs = VecToFileSpec(BlobToCFileSpec(formSpecsBlob));
    set->GetBlob(30, bodyFileNamesBlob); // Line 30 is 'body_file_names'
    config.bodyFileNames = BlobToStringVec(bodyFileNamesBlob);
    set->GetBlob(31, certsPathsBlob); // Line 31 is 'certs_paths'
    config.certsPath = BlobToStringVec(certsPathsBlob);
}

TaskConfig BuildRequestTaskConfig(std::shared_ptr<OHOS::NativeRdb::ResultSet> resultSet)
{
    TaskConfig taskConfig;
    BuildRequestTaskConfigWithLong(resultSet, taskConfig);
    BuildRequestTaskConfigWithInt(resultSet, taskConfig);
    BuildRequestTaskConfigWithString(resultSet, taskConfig);
    BuildRequestTaskConfigWithBlob(resultSet, taskConfig);
    return taskConfig;
}
} // anonymous namespace

bool HasRequestTaskRecord(uint32_t taskId)
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task");
    rdbPredicates.EqualTo("task_id", std::to_string(taskId));
    auto resultSet = OHOS::Request::RequestDataBase::GetInstance().Query(rdbPredicates, { "task_id" });
    if (resultSet == nullptr) {
        REQUEST_HILOGE("HasRequestTaskRecord failed with reason: result set is nullptr, task_id: %{public}d", taskId);
        return false;
    }
    int rowCount = 0;
    resultSet->GetRowCount(rowCount);
    if (rowCount == 0) {
        return false;
    }
    REQUEST_HILOGD("has the task record in database");
    return true;
}

bool RecordRequestTask(CTaskInfo *taskInfo, CTaskConfig *taskConfig)
{
    REQUEST_HILOGD("write to request_task");
    OHOS::NativeRdb::ValuesBucket insertValues;
    insertValues.PutLong("task_id", taskConfig->commonData.taskId);
    insertValues.PutLong("uid", taskConfig->commonData.uid);
    insertValues.PutLong("token_id", taskConfig->commonData.tokenId);
    insertValues.PutInt("action", taskConfig->commonData.action);
    insertValues.PutInt("mode", taskConfig->commonData.mode);
    insertValues.PutInt("cover", taskConfig->commonData.cover);
    insertValues.PutInt("network", taskConfig->commonData.network);
    insertValues.PutInt("metered", taskConfig->commonData.metered);
    insertValues.PutInt("roaming", taskConfig->commonData.roaming);
    insertValues.PutLong("ctime", taskInfo->commonData.ctime);
    insertValues.PutInt("gauge", taskConfig->commonData.gauge);
    insertValues.PutInt("retry", taskInfo->commonData.retry);
    insertValues.PutInt("redirect", taskConfig->commonData.redirect);
    insertValues.PutInt("version", taskConfig->version);
    insertValues.PutLong("config_idx", taskConfig->commonData.index);
    insertValues.PutLong("begins", taskConfig->commonData.begins);
    insertValues.PutLong("ends", taskConfig->commonData.ends);
    insertValues.PutInt("precise", taskConfig->commonData.precise);
    insertValues.PutLong("priority", taskConfig->commonData.priority);
    insertValues.PutInt("background", taskConfig->commonData.background);
    insertValues.PutString("bundle", std::string(taskConfig->bundle.cStr, taskConfig->bundle.len));
    insertValues.PutString("url", std::string(taskConfig->url.cStr, taskConfig->url.len));
    insertValues.PutString("data", std::string(taskConfig->data.cStr, taskConfig->data.len));
    insertValues.PutString("token", std::string(taskConfig->token.cStr, taskConfig->token.len));
    insertValues.PutString("proxy", std::string(taskConfig->proxy.cStr, taskConfig->proxy.len));
    insertValues.PutString(
        "certificate_pins", std::string(taskConfig->certificatePins.cStr, taskConfig->certificatePins.len));
    insertValues.PutString("title", std::string(taskConfig->title.cStr, taskConfig->title.len));
    insertValues.PutString("description", std::string(taskConfig->description.cStr, taskConfig->description.len));
    insertValues.PutString("method", std::string(taskConfig->method.cStr, taskConfig->method.len));
    insertValues.PutString("headers", std::string(taskConfig->headers.cStr, taskConfig->headers.len));
    insertValues.PutString("config_extras", std::string(taskConfig->extras.cStr, taskConfig->extras.len));
    if (!WriteMutableData(insertValues, taskInfo, taskConfig)) {
        REQUEST_HILOGE("write blob data failed");
        return false;
    }
    if (!OHOS::Request::RequestDataBase::GetInstance().Insert(std::string("request_task"), insertValues)) {
        REQUEST_HILOGE("insert to request_task failed, task_id: %{public}d", taskConfig->commonData.taskId);
        return false;
    }
    REQUEST_HILOGD("insert to request_task success");
    return true;
}

bool UpdateRequestTask(uint32_t taskId, CUpdateInfo *updateInfo)
{
    REQUEST_HILOGD("update request_task");
    OHOS::NativeRdb::ValuesBucket values;
    values.PutLong("mtime", updateInfo->mtime);
    values.PutInt("reason", updateInfo->reason);
    values.PutLong("tries", updateInfo->tries);
    if (!WriteUpdateData(values, updateInfo)) {
        REQUEST_HILOGE("update blob data failed");
        return false;
    }

    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task");
    rdbPredicates.EqualTo("task_id", std::to_string(taskId));
    if (!OHOS::Request::RequestDataBase::GetInstance().Update(values, rdbPredicates)) {
        REQUEST_HILOGE("update table1 failed, task_id: %{public}d", taskId);
        return false;
    }
    return true;
}

bool ChangeRequestTaskState(uint32_t taskId, uint64_t uid, State state, Reason reason)
{
    REQUEST_HILOGI(
        "Change task state, task_id is %{public}d, state is %{public}d", taskId, static_cast<int32_t>(state));

    OHOS::NativeRdb::ValuesBucket values;
    values.PutInt("state", static_cast<uint8_t>(state));
    values.PutInt("reason", static_cast<uint8_t>(reason));

    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task");
    rdbPredicates.EqualTo("task_id", std::to_string(taskId))->And()->EqualTo("uid", std::to_string(uid));
    if (!OHOS::Request::RequestDataBase::GetInstance().Update(values, rdbPredicates)) {
        REQUEST_HILOGE("Change request_task state failed, taskid: %{public}d", taskId);
        return false;
    }
    return true;
}

int GetTaskInfoInner(const OHOS::NativeRdb::RdbPredicates &rdbPredicates, TaskInfo &taskInfo)
{
    auto resultSet = OHOS::Request::RequestDataBase::GetInstance().Query(rdbPredicates,
        { "task_id", "uid", "action", "mode", "ctime", "mtime", "reason", "gauge", "retry", "tries", "version",
            "priority", "bundle", "url", "data", "token", "title", "description", "mime_type", "state", "idx",
            "total_processed", "sizes", "processed", "extras", "form_items", "file_specs", "each_file_status" });
    if (resultSet == nullptr || resultSet->GoToFirstRow() != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("result set is nullptr or go to first row failed");
        return OHOS::Request::QUERY_ERR;
    }
    FillCommonTaskInfo(resultSet, taskInfo);
    FillOtherTaskInfo(resultSet, taskInfo);
    resultSet->Close();
    return OHOS::Request::QUERY_OK;
}

CTaskInfo *GetTaskInfo(uint32_t taskId)
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task");
    rdbPredicates.EqualTo("task_id", std::to_string(taskId));

    TaskInfo taskInfo;
    if (GetTaskInfoInner(rdbPredicates, taskInfo) == OHOS::Request::QUERY_ERR) {
        REQUEST_HILOGE("QueryRequestTaskInfo failed with reason: result set is nullptr or go to first row failed, "
                       "task_id: %{public}d",
            taskId);
        return nullptr;
    }

    return BuildCTaskInfo(taskInfo);
}

CVectorWrapper Search(CFilter filter)
{
    CVectorWrapper cVectorWrapper;
    cVectorWrapper.ptr = nullptr;
    cVectorWrapper.len = 0;
    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task");
    std::string bundle = std::string(filter.bundle.cStr, filter.bundle.len);
    rdbPredicates.Between("ctime", std::to_string(filter.commonData.after), std::to_string(filter.commonData.before));
    if (filter.commonData.state != static_cast<uint8_t>(State::ANY)) {
        rdbPredicates.EqualTo("state", std::to_string(filter.commonData.state));
    }
    if (filter.commonData.action != static_cast<uint8_t>(Action::ANY)) {
        rdbPredicates.EqualTo("action", std::to_string(filter.commonData.action));
    }
    if (filter.commonData.mode != static_cast<uint8_t>(Mode::ANY)) {
        rdbPredicates.EqualTo("mode", std::to_string(filter.commonData.mode));
    }
    if (bundle != "*") {
        rdbPredicates.EqualTo("bundle", bundle);
    }
    auto resultSet = OHOS::Request::RequestDataBase::GetInstance().Query(rdbPredicates, { "task_id" });
    if (resultSet == nullptr) {
        REQUEST_HILOGE("Search failed with reason: result set is nullptr");
        return cVectorWrapper;
    }
    int rowCount = 0;
    resultSet->GetRowCount(rowCount);
    cVectorWrapper.ptr = new uint32_t[rowCount];
    cVectorWrapper.len = static_cast<uint64_t>(rowCount);
    for (int i = 0; i < rowCount; i++) {
        if (resultSet->GoToRow(i) != OHOS::NativeRdb::E_OK) {
            REQUEST_HILOGE("result set go to %{public}d row failed", i);
            cVectorWrapper.ptr = nullptr;
            return cVectorWrapper;
        }
        int64_t taskId = 0;
        resultSet->GetLong(0, taskId);
        cVectorWrapper.ptr[i] = static_cast<uint32_t>(taskId);
    }
    return cVectorWrapper;
}

void DeleteCVectorWrapper(uint32_t *ptr)
{
    delete[] ptr;
}

uint32_t QueryAppUncompletedTasksNum(uint64_t uid, uint8_t mode)
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task");
    rdbPredicates.EqualTo("uid", std::to_string(uid));
    rdbPredicates.EqualTo("mode", mode);
    rdbPredicates.BeginWrap();
    rdbPredicates.EqualTo("state", static_cast<uint8_t>(State::WAITING))
        ->Or()
        ->EqualTo("state", static_cast<uint8_t>(State::PAUSED))
        ->Or()
        ->EqualTo("state", static_cast<uint8_t>(State::INITIALIZED))
        ->Or()
        ->EqualTo("state", static_cast<uint8_t>(State::RUNNING))
        ->Or()
        ->EqualTo("state", static_cast<uint8_t>(State::RETRYING));
    rdbPredicates.EndWrap();

    auto resultSet = OHOS::Request::RequestDataBase::GetInstance().Query(rdbPredicates, { "task_id" });
    int rowCount = 0;
    if (resultSet == nullptr || resultSet->GetRowCount(rowCount) != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("WaitingApps result set is nullptr or get row count failed");
    }

    return rowCount;
}

bool HasTaskConfigRecord(uint32_t taskId)
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task");
    rdbPredicates.EqualTo("task_id", std::to_string(taskId));
    auto resultSet = OHOS::Request::RequestDataBase::GetInstance().Query(rdbPredicates, { "task_id" });
    if (resultSet == nullptr) {
        REQUEST_HILOGE("TaskConfig result set is nullptr");
        return false;
    }
    int rowCount = 0;
    if (resultSet->GetRowCount(rowCount) != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("TaskConfig result count row failed");
        return false;
    }
    if (rowCount == 0) {
        return false;
    }
    REQUEST_HILOGI("has the task record in request_task database");
    return true;
}

CTaskConfig **QueryAllTaskConfig(uint32_t &len)
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task");
    rdbPredicates.EqualTo("state", static_cast<uint8_t>(State::WAITING))
        ->Or()
        ->EqualTo("state", static_cast<uint8_t>(State::PAUSED))
        ->Or()
        ->EqualTo("state", static_cast<uint8_t>(State::INITIALIZED));

    std::vector<TaskConfig> taskConfigs;
    if (QueryRequestTaskConfig(rdbPredicates, taskConfigs) == OHOS::Request::QUERY_ERR) {
        return nullptr;
    }

    len = taskConfigs.size();
    return BuildCTaskConfigs(taskConfigs);
}

int QueryRequestTaskConfig(const OHOS::NativeRdb::RdbPredicates &rdbPredicates, std::vector<TaskConfig> &taskConfigs)
{
    auto resultSet = OHOS::Request::RequestDataBase::GetInstance().Query(rdbPredicates,
        { "task_id", "uid", "token_id", "action", "mode", "cover", "network", "metered", "roaming", "retry",
            "redirect", "config_idx", "begins", "ends", "gauge", "precise", "priority", "background", "bundle", "url",
            "title", "description", "method", "headers", "data", "token", "config_extras", "version", "form_items",
            "file_specs", "body_file_names", "certs_paths", "proxy", "certificate_pins" });
    int rowCount = 0;
    if (resultSet == nullptr || resultSet->GetRowCount(rowCount) != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("TaskConfig result set is nullptr or get row count failed");
        return OHOS::Request::QUERY_ERR;
    }
    for (auto i = 0; i < rowCount; i++) {
        if (resultSet->GoToRow(i) != OHOS::NativeRdb::E_OK) {
            REQUEST_HILOGE("TaskConfig result set go to %{public}d row failed", i);
            return OHOS::Request::QUERY_ERR;
        }
        TaskConfig taskConfig = BuildRequestTaskConfig(resultSet);
        taskConfigs.push_back(std::move(taskConfig));
    }
    resultSet->Close();
    return OHOS::Request::QUERY_OK;
}

void BuildCTaskConfig(CTaskConfig *cTaskConfig, const TaskConfig &taskConfig)
{
    cTaskConfig->bundle = WrapperCString(taskConfig.bundle);
    cTaskConfig->url = WrapperCString(taskConfig.url);
    cTaskConfig->title = WrapperCString(taskConfig.title);
    cTaskConfig->description = WrapperCString(taskConfig.description);
    cTaskConfig->method = WrapperCString(taskConfig.method);
    cTaskConfig->headers = WrapperCString(taskConfig.headers);
    cTaskConfig->data = WrapperCString(taskConfig.data);
    cTaskConfig->token = WrapperCString(taskConfig.token);
    cTaskConfig->extras = WrapperCString(taskConfig.extras);
    cTaskConfig->proxy = WrapperCString(taskConfig.proxy);
    cTaskConfig->certificatePins = WrapperCString(taskConfig.certificatePins);
    cTaskConfig->version = taskConfig.version;

    uint32_t formItemsLen = taskConfig.formItems.size();
    CFormItem *formItemsPtr = new CFormItem[formItemsLen];
    for (uint32_t j = 0; j < formItemsLen; j++) {
        formItemsPtr[j].name = WrapperCString(taskConfig.formItems[j].name);
        formItemsPtr[j].value = WrapperCString(taskConfig.formItems[j].value);
    }
    uint32_t fileSpecsLen = taskConfig.fileSpecs.size();
    CFileSpec *fileSpecsPtr = new CFileSpec[fileSpecsLen];
    for (uint32_t j = 0; j < fileSpecsLen; j++) {
        fileSpecsPtr[j].name = WrapperCString(taskConfig.fileSpecs[j].name);
        fileSpecsPtr[j].path = WrapperCString(taskConfig.fileSpecs[j].path);
        fileSpecsPtr[j].fileName = WrapperCString(taskConfig.fileSpecs[j].fileName);
        fileSpecsPtr[j].mimeType = WrapperCString(taskConfig.fileSpecs[j].mimeType);
        fileSpecsPtr[j].is_user_file = taskConfig.fileSpecs[j].is_user_file;
    }
    uint32_t bodyFileNamesLen = taskConfig.bodyFileNames.size();
    CStringWrapper *bodyFileNamesPtr = new CStringWrapper[bodyFileNamesLen];
    for (uint32_t j = 0; j < bodyFileNamesLen; j++) {
        bodyFileNamesPtr[j] = WrapperCString(taskConfig.bodyFileNames[j]);
    }

    uint32_t certsPathLen = taskConfig.certsPath.size();
    CStringWrapper *certsPathPtr = new CStringWrapper[certsPathLen];
    for (uint32_t j = 0; j < certsPathLen; j++) {
        certsPathPtr[j] = WrapperCString(taskConfig.certsPath[j]);
    }

    cTaskConfig->formItemsPtr = formItemsPtr;
    cTaskConfig->formItemsLen = formItemsLen;
    cTaskConfig->fileSpecsPtr = fileSpecsPtr;
    cTaskConfig->fileSpecsLen = fileSpecsLen;
    cTaskConfig->bodyFileNamesPtr = bodyFileNamesPtr;
    cTaskConfig->bodyFileNamesLen = bodyFileNamesLen;
    cTaskConfig->certsPathPtr = certsPathPtr;
    cTaskConfig->certsPathLen = certsPathLen;
    cTaskConfig->commonData = taskConfig.commonData;
}

CTaskConfig **BuildCTaskConfigs(const std::vector<TaskConfig> &taskConfigs)
{
    CTaskConfig **cTaskConfigs = new CTaskConfig *[taskConfigs.size()];
    for (unsigned int i = 0; i < taskConfigs.size(); i++) {
        CTaskConfig *cTaskConfig = new CTaskConfig;
        const TaskConfig &taskConfig = taskConfigs[i];
        BuildCTaskConfig(cTaskConfig, taskConfig);
        cTaskConfigs[i] = std::move(cTaskConfig);
    }
    return cTaskConfigs;
}

CTaskConfig **QueryAllTaskConfigs(void)
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task");
    rdbPredicates.EqualTo("state", static_cast<uint8_t>(State::WAITING))
        ->Or()
        ->EqualTo("state", static_cast<uint8_t>(State::PAUSED))
        ->Or()
        ->EqualTo("state", static_cast<uint8_t>(State::INITIALIZED));

    std::vector<TaskConfig> taskConfigs;
    if (QueryRequestTaskConfig(rdbPredicates, taskConfigs) == OHOS::Request::QUERY_ERR) {
        return nullptr;
    }
    return BuildCTaskConfigs(taskConfigs);
}

int QueryTaskConfigLen()
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task");
    rdbPredicates.EqualTo("state", static_cast<uint8_t>(State::WAITING))
        ->Or()
        ->EqualTo("state", static_cast<uint8_t>(State::PAUSED))
        ->Or()
        ->EqualTo("state", static_cast<uint8_t>(State::INITIALIZED));

    auto resultSet = OHOS::Request::RequestDataBase::GetInstance().Query(rdbPredicates, { "task_id", "uid" });
    int len = 0;
    if (resultSet == nullptr || resultSet->GetRowCount(len) != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("Get TaskConfigs length failed");
        return OHOS::Request::QUERY_ERR;
    }
    return len;
}

CTaskConfig *QueryTaskConfig(uint32_t taskId)
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task");
    rdbPredicates.EqualTo("task_id", std::to_string(taskId));
    auto resultSet = OHOS::Request::RequestDataBase::GetInstance().Query(rdbPredicates,
        { "task_id", "uid", "token_id", "action", "mode", "cover", "network", "metered", "roaming", "retry",
            "redirect", "config_idx", "begins", "ends", "gauge", "precise", "priority", "background", "bundle", "url",
            "title", "description", "method", "headers", "data", "token", "config_extras", "version", "form_items",
            "file_specs", "body_file_names", "certs_paths", "proxy", "certificate_pins" });
    int rowCount = 0;
    if (resultSet == nullptr) {
        REQUEST_HILOGE("QuerySingleTaskConfig failed with reason: result set is nullptr");
        return nullptr;
    }
    if (resultSet->GetRowCount(rowCount) != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("TaskConfig result count row failed");
        return nullptr;
    }
    if (rowCount == 0) {
        REQUEST_HILOGE("TaskConfig result count row is 0");
        return nullptr;
    }
    if (resultSet->GoToRow(0) != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("TaskConfig result set go to 0 row failed");
        return nullptr;
    }

    TaskConfig taskConfig = BuildRequestTaskConfig(resultSet);
    REQUEST_HILOGD(
        "QuerySingleTaskConfig in, after BuildRequestTaskConfig, task_id: %{public}u", taskConfig.commonData.taskId);
    CTaskConfig *cTaskConfig = new CTaskConfig;
    BuildCTaskConfig(cTaskConfig, taskConfig);
    return cTaskConfig;
}

void RequestDBRemoveRecordsFromTime(uint64_t time)
{
    OHOS::NativeRdb::RdbPredicates predicates("request_task");
    predicates.LessThan("mtime", std::to_string(time));

    if (OHOS::Request::RequestDataBase::GetInstance().Delete(predicates)) {
        REQUEST_HILOGI("request_task table deletes records before one week success");
        return;
    }
    REQUEST_HILOGE("request_task table deletes records before one week failed");
    return;
}

bool QueryTaskTokenId(uint32_t taskId, uint64_t &tokenId)
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task");
    rdbPredicates.EqualTo("task_id", std::to_string(taskId));
    auto resultSet = OHOS::Request::RequestDataBase::GetInstance().Query(rdbPredicates, { "token_id" });
    int rowCount = 0;
    if (resultSet == nullptr) {
        REQUEST_HILOGE("QueryTaskTokenId failed with reason: result set is nullptr, taskId: %{public}d", taskId);
        return false;
    }
    if (resultSet->GetRowCount(rowCount) != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("TaskConfig result count row failed");
        return false;
    }
    if (rowCount == 0) {
        REQUEST_HILOGE("TaskConfig result count row is 0");
        return false;
    }
    if (resultSet->GoToRow(0) != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("TaskConfig result set go to 0 row failed");
        return false;
    }
    tokenId = static_cast<uint64_t>(GetLong(resultSet, 0));
    return true;
}

void UpdateTaskStateOnAppStateChange(uint64_t uid, uint8_t appState)
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task");
    if (appState == 2) { // 2 means ApplicationState::Foreground
        rdbPredicates.EqualTo("uid", std::to_string(uid));
        rdbPredicates.EqualTo("mode", static_cast<uint8_t>(Mode::FOREGROUND));
        rdbPredicates.EqualTo("state", static_cast<uint8_t>(State::PAUSED));
        rdbPredicates.EqualTo("reason", static_cast<uint8_t>(6)); // 6 means Reason::AppBackgroundOrTerminate.

        OHOS::NativeRdb::ValuesBucket values;
        values.PutInt("state", static_cast<uint8_t>(State::WAITING));
        values.PutInt("reason", static_cast<uint8_t>(4)); // 4 means Reason::RunningTaskMeetLimits.

        if (!OHOS::Request::RequestDataBase::GetInstance().Update(values, rdbPredicates)) {
            REQUEST_HILOGE("Change request_task state to WaitingForQos on app state change to foreground failed");
            return;
        }
    } else {
        rdbPredicates.EqualTo("uid", std::to_string(uid));
        rdbPredicates.EqualTo("mode", static_cast<uint8_t>(Mode::FOREGROUND));
        rdbPredicates.EqualTo("state", static_cast<uint8_t>(State::WAITING));
        rdbPredicates.EqualTo("reason", static_cast<uint8_t>(4)); // 4 means Reason::RunningTaskMeetLimits.

        OHOS::NativeRdb::ValuesBucket values;
        values.PutInt("state", static_cast<uint8_t>(State::PAUSED));
        values.PutInt("reason", static_cast<uint8_t>(6)); // 6 means Reason::AppBackgroundOrTerminate.

        if (!OHOS::Request::RequestDataBase::GetInstance().Update(values, rdbPredicates)) {
            REQUEST_HILOGE("Change request_task state to WaitingForQos on app state change to background failed");
            return;
        }
    }
}

void UpdateTaskStateOnNetworkChange(NetworkInfo info)
{
    if (info.networkType == NetworkInner::NET_LOST) {
        // change states of all tasks with `Reason::NetworkOffline` or
        // `RunningTaskMeetLimits` state to `Reason::NetworkOffline` state.
        OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task");
        rdbPredicates.EqualTo("state", static_cast<uint8_t>(State::WAITING))
            ->And()
            ->EqualTo("reason", static_cast<uint8_t>(Reason::RUNNING_TASK_MEET_LIMITS));

        OHOS::NativeRdb::ValuesBucket networkOffline;
        networkOffline.PutInt("reason", static_cast<uint8_t>(Reason::NETWORK_OFFLINE));

        if (!OHOS::Request::RequestDataBase::GetInstance().Update(networkOffline, rdbPredicates)) {
            REQUEST_HILOGE("Change request_task state to NetworkOffline on network change failed");
        }
        return;
    }

    // change states of all satisfied task to `RunningTaskMeetLimits` state.
    OHOS::NativeRdb::ValuesBucket satisfied;
    satisfied.PutInt("reason", static_cast<uint8_t>(Reason::RUNNING_TASK_MEET_LIMITS));
    // For WI-FI situation.
    if (info.networkType == NetworkInner::WIFI || info.networkType == NetworkInner::ANY) {
        OHOS::NativeRdb::RdbPredicates satisfiedWifi("request_task");
        satisfiedWifi.BeginWrap()
            ->EqualTo("network", static_cast<uint8_t>(Network::WIFI))
            ->Or()
            ->EqualTo("network", static_cast<uint8_t>(Network::ANY))
            ->EndWrap()
            ->And()
            ->EqualTo("state", static_cast<uint8_t>(State::WAITING))
            ->And()
            ->BeginWrap()
            ->EqualTo("reason", static_cast<uint8_t>(Reason::NETWORK_OFFLINE))
            ->Or()
            ->EqualTo("reason", static_cast<uint8_t>(Reason::UNSUPPORTED_NETWORK_TYPE))
            ->EndWrap();
        if (!OHOS::Request::RequestDataBase::GetInstance().Update(satisfied, satisfiedWifi)) {
            REQUEST_HILOGE("Change WI-FI task to RunningTaskMeetLimits on network change failed");
            return;
        }
    }

    // For CELLULAR situation.
    if (info.networkType == NetworkInner::CELLULAR || info.networkType == NetworkInner::ANY) {
        OHOS::NativeRdb::RdbPredicates satisfiedCellular("request_task");
        satisfiedCellular.BeginWrap()
            ->EqualTo("network", static_cast<uint8_t>(Network::CELLULAR))
            ->Or()
            ->EqualTo("network", static_cast<uint8_t>(Network::ANY))
            ->EndWrap()
            ->And()
            ->EqualTo("state", static_cast<uint8_t>(State::WAITING))
            ->And()
            ->BeginWrap()
            ->EqualTo("reason", static_cast<uint8_t>(Reason::NETWORK_OFFLINE))
            ->Or()
            ->EqualTo("reason", static_cast<uint8_t>(Reason::UNSUPPORTED_NETWORK_TYPE))
            ->EndWrap();

        if (info.isMetered) {
            satisfiedCellular.And()->EqualTo("metered", std::to_string(static_cast<uint8_t>(true)));
        }

        if (info.isRoaming) {
            satisfiedCellular.And()->EqualTo("roaming", std::to_string(static_cast<uint8_t>(true)));
        }

        if (!OHOS::Request::RequestDataBase::GetInstance().Update(satisfied, satisfiedCellular)) {
            REQUEST_HILOGE("Change CELLULAR task to RunningTaskMeetLimits on network change failed");
            return;
        }
    }
}

void BuildTaskQosInfo(TaskQosInfo *info, std::shared_ptr<OHOS::NativeRdb::ResultSet> set)
{
    info->taskId = static_cast<uint32_t>(GetLong(set, 0));   // Line 0 is 'task_id'
    info->action = static_cast<uint8_t>(GetInt(set, 1));     // Line 1 is 'action'
    info->mode = static_cast<uint8_t>(GetInt(set, 2));       // Line 2 is 'mode'
    info->state = static_cast<uint8_t>(GetInt(set, 3));      // Line 3 is 'state'
    info->priority = static_cast<uint32_t>(GetLong(set, 4)); // Line 4 is 'priority'
}

void GetTaskQosInfo(uint64_t uid, uint32_t taskId, TaskQosInfo **info)
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task");
    rdbPredicates.EqualTo("uid", std::to_string(uid))->And()->EqualTo("task_id", std::to_string(taskId));

    auto resultSet = OHOS::Request::RequestDataBase::GetInstance().Query(
        rdbPredicates, { "task_id", "action", "mode", "state", "priority" });
    int rowCount = 0;
    if (resultSet == nullptr || resultSet->GetRowCount(rowCount) != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("GetRunningTasksArray result set is nullptr or get row count failed");
        return;
    }

    if (resultSet->GoToRow(0) != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("GetRunningTasksArray result set go to 0 row failed");
        return;
    }

    *info = new TaskQosInfo;
    BuildTaskQosInfo(*info, resultSet);
}

void GetAppTaskQosInfos(uint64_t uid, TaskQosInfo **array, size_t *len)
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task");
    rdbPredicates.EqualTo("uid", std::to_string(uid))
        ->And()
        ->BeginWrap()
        ->BeginWrap()
        ->EqualTo("state", static_cast<uint8_t>(State::WAITING))
        ->And()
        ->EqualTo("reason", static_cast<uint8_t>(Reason::RUNNING_TASK_MEET_LIMITS))
        ->EndWrap()
        ->Or()
        ->EqualTo("state", static_cast<uint8_t>(State::RUNNING))
        ->Or()
        ->EqualTo("state", static_cast<uint8_t>(State::RETRYING))
        ->EndWrap();

    *array = nullptr;
    *len = 0;

    auto resultSet = OHOS::Request::RequestDataBase::GetInstance().Query(
        rdbPredicates, { "task_id", "action", "mode", "state", "priority" });
    int rowCount = 0;
    if (resultSet == nullptr || resultSet->GetRowCount(rowCount) != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("GetRunningTasksArray result set is nullptr or get row count failed");
        return;
    }

    if (rowCount == 0) {
        return;
    }

    *array = new TaskQosInfo[rowCount];
    *len = rowCount;
    for (auto i = 0; i < rowCount; i++) {
        if (resultSet->GoToRow(i) != OHOS::NativeRdb::E_OK) {
            REQUEST_HILOGE("GetRunningTasksArray result set go to %{public}d row failed", i);
            return;
        }

        BuildTaskQosInfo(&(*array)[i], resultSet);
    }
}

void GetAppArray(AppInfo **apps, size_t *len)
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task");
    rdbPredicates.Distinct();

    *apps = nullptr;
    *len = 0;

    auto resultSet = OHOS::Request::RequestDataBase::GetInstance().Query(rdbPredicates, { "uid", "bundle" });
    int rowCount = 0;
    if (resultSet == nullptr || resultSet->GetRowCount(rowCount) != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("GetAppArray result set is nullptr or get row count failed");
    }

    if (rowCount == 0) {
        return;
    }

    *apps = new AppInfo[rowCount];
    *len = rowCount;
    for (auto i = 0; i < rowCount; i++) {
        if (resultSet->GoToRow(i) != OHOS::NativeRdb::E_OK) {
            REQUEST_HILOGE("GetAppArray result set go to %{public}d row failed", i);
            return;
        }

        std::string temp = "";
        resultSet->GetString(1, temp);                                 // Line 1 is 'bundle'
        (*apps)[i].uid = static_cast<uint32_t>(GetLong(resultSet, 0)); // Line 0 is 'uid'
        (*apps)[i].bundle = WrapperCString(temp);
    }
}

CStringWrapper GetAppBundle(uint64_t uid)
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task");

    // Descending to get the latest bundlename by uid
    rdbPredicates.EqualTo("uid", std::to_string(uid))->OrderByDesc("ctime");

    CStringWrapper res;
    res.cStr = nullptr;
    res.len = 0;

    auto resultSet = OHOS::Request::RequestDataBase::GetInstance().Query(rdbPredicates, { "bundle" });
    int rowCount = 0;
    if (resultSet == nullptr || resultSet->GetRowCount(rowCount) != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("GetAppArray result set is nullptr or get row count failed");
        return res;
    }

    if (rowCount == 0) {
        return res;
    }

    if (resultSet->GoToRow(0) != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("GetAppArray result set go to 0 row failed");
        return res;
    }

    std::string temp = "";
    resultSet->GetString(0, temp); // Line 0 is 'bundle'

    res = WrapperCString(temp);
    return res;
}