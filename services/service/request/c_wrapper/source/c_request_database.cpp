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

#include <algorithm>
#include <cstdint>

#include "log.h"

namespace OHOS::Request {
RequestDataBase::RequestDataBase()
{
    int errCode = OHOS::NativeRdb::E_OK;
    OHOS::NativeRdb::RdbStoreConfig config(DB_NAME);
    config.SetSecurityLevel(NativeRdb::SecurityLevel::S1);
    config.SetEncryptStatus(true);
    RequestDBOpenCallback requestDBOpenCallback;
    store_ = OHOS::NativeRdb::RdbHelper::GetRdbStore(config, DATABASE_OPEN_VERSION, requestDBOpenCallback, errCode);
    REQUEST_HILOGI("get request database errcode :%{public}d", errCode);
}

RequestDataBase &RequestDataBase::GetInstance()
{
    static RequestDataBase requestDataBase;
    return requestDataBase;
}

bool RequestDataBase::BeginTransaction()
{
    if (store_ == nullptr) {
        REQUEST_HILOGE("store_ is nullptr");
        return false;
    }
    int ret = store_->BeginTransaction();
    REQUEST_HILOGI("request database begin transaction ret :%{public}d", ret);
    return ret == OHOS::NativeRdb::E_OK;
}

bool RequestDataBase::Commit()
{
    if (store_ == nullptr) {
        REQUEST_HILOGE("store_ is nullptr");
        return false;
    }
    int ret = store_->Commit();
    REQUEST_HILOGI("request database commit ret :%{public}d", ret);
    return ret == OHOS::NativeRdb::E_OK;
}

bool RequestDataBase::RollBack()
{
    if (store_ == nullptr) {
        REQUEST_HILOGE("store_ is nullptr");
        return false;
    }
    int ret = store_->RollBack();
    REQUEST_HILOGI("request database rollback ret :%{public}d", ret);
    return ret == OHOS::NativeRdb::E_OK;
}

bool RequestDataBase::Insert(const std::string &table, const OHOS::NativeRdb::ValuesBucket &insertValues)
{
    int64_t outRowId = 0;
    if (store_ == nullptr) {
        REQUEST_HILOGE("store_ is nullptr");
        return false;
    }

    int ret = store_->Insert(outRowId, table, insertValues);
    return ret == OHOS::NativeRdb::E_OK;
}

bool RequestDataBase::Update(const OHOS::NativeRdb::ValuesBucket values,
    const OHOS::NativeRdb::AbsRdbPredicates &predicates)
{
    int changedRows = 0;
    if (store_ == nullptr) {
        REQUEST_HILOGE("store_ is nullptr");
        return false;
    }

    int ret = store_->Update(changedRows, values, predicates);
    return ret == OHOS::NativeRdb::E_OK;
}

std::shared_ptr<OHOS::NativeRdb::ResultSet> RequestDataBase::Query(const OHOS::NativeRdb::AbsRdbPredicates &predicates,
    const std::vector<std::string> &columns)
{
    if (store_ == nullptr) {
        REQUEST_HILOGE("store_ is  nullptr");
        return nullptr;
    }
    return store_->Query(predicates, columns);
}

bool RequestDataBase::Delete(const OHOS::NativeRdb::AbsRdbPredicates &predicates)
{
    if (store_ == nullptr) {
        REQUEST_HILOGE("store_ is nullptr");
        return false;
    }

    int deletedRows = 0;
    int ret = store_->Delete(deletedRows, predicates);
    REQUEST_HILOGI("request database delete ret is %{public}d, rows: %{public}d", ret, deletedRows);
    return ret == OHOS::NativeRdb::E_OK;
}

int RequestDBOpenCallback::OnCreate(OHOS::NativeRdb::RdbStore &store)
{
    int ret = store.ExecuteSql(CREATE_REQUEST_TABLE1);
    if (ret != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("create table1 error, ret = %{public}d", ret);
        return ret;
    }
    ret = store.ExecuteSql(CREATE_REQUEST_TABLE2);
    if (ret != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("create table2 error, ret = %{public}d", ret);
        return ret;
    }
    REQUEST_HILOGI("create table success");
    return OHOS::NativeRdb::E_OK;
}

int RequestDBOpenCallback::OnOpen(OHOS::NativeRdb::RdbStore &store)
{
    int ret = store.ExecuteSql(CREATE_REQUEST_TABLE3);
    if (ret != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("create table3 error, ret = %{public}d", ret);
        return ret;
    }
    ret = store.ExecuteSql(CREATE_REQUEST_TABLE4);
    if (ret != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("create table4 error, ret = %{public}d", ret);
        return ret;
    }
    ret = store.ExecuteSql(CREATE_PRIORITY_TABLE);
    if (ret != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("create priority table error, ret = %{public}d", ret);
        return ret;
    }
    ret = store.ExecuteSql(CREATE_CERTS_TABLE);
    if (ret != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("create certs table error, ret = %{public}d", ret);
        return ret;
    }
    REQUEST_HILOGI("create config table success");
    return OHOS::NativeRdb::E_OK;
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

bool HasRequestTaskRecord(uint32_t taskId)
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task_info");
    rdbPredicates.EqualTo("task_id", std::to_string(taskId));
    auto resultSet = OHOS::Request::RequestDataBase::GetInstance().Query(rdbPredicates, { "task_id" });
    if (resultSet == nullptr) {
        REQUEST_HILOGE("result set is nullptr");
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

bool WriteRequestTaskInfo(CTaskInfo *taskInfo)
{
    REQUEST_HILOGD("write to request_task_info");
    OHOS::NativeRdb::ValuesBucket insertValues;
    insertValues.PutLong("task_id", taskInfo->commonData.taskId);
    insertValues.PutLong("uid", taskInfo->commonData.uid);
    insertValues.PutInt("action", taskInfo->commonData.action);
    insertValues.PutInt("mode", taskInfo->commonData.mode);
    insertValues.PutLong("ctime", taskInfo->commonData.ctime);
    insertValues.PutLong("mtime", taskInfo->commonData.mtime);
    insertValues.PutInt("reason", taskInfo->commonData.reason);
    insertValues.PutInt("gauge", taskInfo->commonData.gauge);
    insertValues.PutInt("retry", taskInfo->commonData.retry);
    insertValues.PutLong("tries", taskInfo->commonData.tries);
    insertValues.PutInt("version", taskInfo->commonData.version);
    insertValues.PutString("bundle", std::string(taskInfo->bundle.cStr, taskInfo->bundle.len));
    insertValues.PutString("url", std::string(taskInfo->url.cStr, taskInfo->url.len));
    insertValues.PutString("data", std::string(taskInfo->data.cStr, taskInfo->data.len));
    insertValues.PutString("token", std::string(taskInfo->token.cStr, taskInfo->token.len));
    insertValues.PutString("titile", std::string(taskInfo->title.cStr, taskInfo->title.len));
    insertValues.PutString("description", std::string(taskInfo->description.cStr, taskInfo->description.len));
    insertValues.PutString("mime_type", std::string(taskInfo->mimeType.cStr, taskInfo->mimeType.len));
    insertValues.PutInt("state", taskInfo->progress.commonData.state);
    insertValues.PutLong("idx", taskInfo->progress.commonData.index);
    insertValues.PutLong("total_processed", taskInfo->progress.commonData.totalProcessed);
    insertValues.PutString("sizes", std::string(taskInfo->progress.sizes.cStr, taskInfo->progress.sizes.len));
    insertValues.PutString("processed",
        std::string(taskInfo->progress.processed.cStr, taskInfo->progress.processed.len));
    insertValues.PutString("extras", std::string(taskInfo->progress.extras.cStr, taskInfo->progress.extras.len));
    insertValues.PutLong("form_items_len", taskInfo->formItemsLen);
    insertValues.PutLong("file_specs_len", taskInfo->fileSpecsLen);
    if (!OHOS::Request::RequestDataBase::GetInstance().Insert(std::string("request_task_info"), insertValues)) {
        REQUEST_HILOGE("insert to request_task_info failed");
        return false;
    }

    // Inserts temp_table.
    OHOS::NativeRdb::ValuesBucket insertValues2;
    insertValues2.PutLong("task_id", taskInfo->commonData.taskId);
    insertValues2.PutLong("uid", taskInfo->commonData.uid);
    insertValues2.PutLong("priority", taskInfo->commonData.priority);
    if (!OHOS::Request::RequestDataBase::GetInstance().Insert(std::string("priority_table"), insertValues2)) {
        REQUEST_HILOGE("insert to priority_table failed");
        return false;
    }

    REQUEST_HILOGD("insert to request_task_info success");
    return true;
}

bool WriteTaskInfoAttachment(CTaskInfo *taskInfo)
{
    REQUEST_HILOGD("write to task_info_attachment");
    uint64_t len = std::max(taskInfo->formItemsLen, taskInfo->fileSpecsLen);
    for (uint64_t i = 0; i < len; i++) {
        OHOS::NativeRdb::ValuesBucket insertValues;
        insertValues.PutInt("task_id", taskInfo->commonData.taskId);
        insertValues.PutInt("uid", taskInfo->commonData.uid);
        if (i < taskInfo->formItemsLen) {
            insertValues.PutString("form_item_name",
                std::string(taskInfo->formItemsPtr[i].name.cStr, taskInfo->formItemsPtr[i].name.len));
            insertValues.PutString("value",
                std::string(taskInfo->formItemsPtr[i].value.cStr, taskInfo->formItemsPtr[i].value.len));
        }
        if (i < taskInfo->fileSpecsLen) {
            insertValues.PutString("file_spec_name",
                std::string(taskInfo->fileSpecsPtr[i].name.cStr, taskInfo->fileSpecsPtr[i].name.len));
            insertValues.PutString("path",
                std::string(taskInfo->fileSpecsPtr[i].path.cStr, taskInfo->fileSpecsPtr[i].path.len));
            insertValues.PutString("file_name",
                std::string(taskInfo->fileSpecsPtr[i].fileName.cStr, taskInfo->fileSpecsPtr[i].fileName.len));
            insertValues.PutString("mime_type",
                std::string(taskInfo->fileSpecsPtr[i].mimeType.cStr, taskInfo->fileSpecsPtr[i].mimeType.len));
            insertValues.PutInt("reason", taskInfo->eachFileStatusPtr[i].reason);
            insertValues.PutString("message", std::string(taskInfo->eachFileStatusPtr[i].message.cStr,
                taskInfo->eachFileStatusPtr[i].message.len));
        }
        if (!OHOS::Request::RequestDataBase::GetInstance().Insert(std::string("task_info_attachment"), insertValues)) {
            REQUEST_HILOGE("insert to task_info_attachment failed");
            return false;
        }
    }
    REQUEST_HILOGD("insert to task_info_attachment success");
    return true;
}

bool RecordRequestTaskInfo(CTaskInfo *taskInfo)
{
    return WriteRequestTaskInfo(taskInfo) && WriteTaskInfoAttachment(taskInfo);
}

bool UpdateRequestTaskInfo(uint32_t taskId, CUpdateInfo *updateInfo)
{
    REQUEST_HILOGD("update task info");
    OHOS::NativeRdb::ValuesBucket values;
    values.PutLong("mtime", updateInfo->mtime);
    values.PutInt("reason", updateInfo->reason);
    values.PutLong("tries", updateInfo->tries);
    values.PutInt("state", updateInfo->progress.commonData.state);
    values.PutLong("idx", updateInfo->progress.commonData.index);
    values.PutLong("total_processed", updateInfo->progress.commonData.totalProcessed);
    values.PutString("sizes", std::string(updateInfo->progress.sizes.cStr, updateInfo->progress.sizes.len));
    values.PutString("mime_type", std::string(updateInfo->mimeType.cStr, updateInfo->mimeType.len));
    values.PutString("processed",
        std::string(updateInfo->progress.processed.cStr, updateInfo->progress.processed.len));
    values.PutString("extras", std::string(updateInfo->progress.extras.cStr, updateInfo->progress.extras.len));

    OHOS::NativeRdb::RdbPredicates rdbPredicates1("request_task_info");
    rdbPredicates1.EqualTo("task_id", std::to_string(taskId));
    if (!OHOS::Request::RequestDataBase::GetInstance().Update(values, rdbPredicates1)) {
        REQUEST_HILOGE("update table1 failed");
        return false;
    }
    for (uint32_t i = 0; i < updateInfo->eachFileStatusLen; i++) {
        OHOS::NativeRdb::ValuesBucket values1;
        values1.PutInt("reason", updateInfo->eachFileStatusPtr[i].reason);
        values1.PutString("message", std::string(updateInfo->eachFileStatusPtr[i].message.cStr,
            updateInfo->eachFileStatusPtr[i].message.len));
        OHOS::NativeRdb::RdbPredicates rdbPredicates2("task_info_attachment");
        rdbPredicates2.EqualTo("task_id", std::to_string(taskId))
            ->And()
            ->EqualTo("path", std::string(updateInfo->eachFileStatusPtr[i].path.cStr,
                updateInfo->eachFileStatusPtr[i].path.len));
        if (!OHOS::Request::RequestDataBase::GetInstance().Update(values1, rdbPredicates2)) {
            REQUEST_HILOGE("update table2 failed");
            return false;
        }
    }
    return true;
}

CTaskInfo *Show(uint32_t taskId, uint64_t uid)
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates1("request_task_info");
    rdbPredicates1.EqualTo("task_id", std::to_string(taskId))
            ->And()
            ->EqualTo("uid", std::to_string(uid));
    int64_t formItemsLen = 0;
    int64_t fileSpecsLen = 0;
    TaskInfo taskInfo;
    if (TouchRequestTaskInfo(rdbPredicates1, taskInfo, formItemsLen, fileSpecsLen) == OHOS::Request::QUERY_ERR) {
        return nullptr;
    }

    OHOS::NativeRdb::RdbPredicates rdbPredicates2("task_info_attachment");
    rdbPredicates2.EqualTo("task_id", std::to_string(taskId))->And()->EqualTo("uid", std::to_string(uid));
    if (TouchTaskInfoAttachment(rdbPredicates2, taskInfo, formItemsLen, fileSpecsLen) == OHOS::Request::QUERY_ERR) {
        return nullptr;
    }

    OHOS::NativeRdb::RdbPredicates rdbPredicates3("priority_table");
    rdbPredicates2.EqualTo("task_id", std::to_string(taskId))->And()->EqualTo("uid", std::to_string(uid));
    if (GetPriority(rdbPredicates3, taskInfo.commonData.priority) == OHOS::Request::QUERY_ERR) {
        return nullptr;
    }

    return BuildCTaskInfo(taskInfo);
}

CTaskInfo *Touch(uint32_t taskId, uint64_t uid, CStringWrapper token)
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates1("request_task_info");
    rdbPredicates1.EqualTo("task_id", std::to_string(taskId))
        ->And()
        ->EqualTo("uid", std::to_string(uid))
        ->And()
        ->EqualTo("token", std::string(token.cStr, token.len));
    int64_t formItemsLen = 0;
    int64_t fileSpecsLen = 0;
    TaskInfo taskInfo;
    if (TouchRequestTaskInfo(rdbPredicates1, taskInfo, formItemsLen, fileSpecsLen) == OHOS::Request::QUERY_ERR) {
        return nullptr;
    }

    OHOS::NativeRdb::RdbPredicates rdbPredicates2("task_info_attachment");
    rdbPredicates2.EqualTo("task_id", std::to_string(taskId))->And()->EqualTo("uid", std::to_string(uid));
    if (TouchTaskInfoAttachment(rdbPredicates2, taskInfo, formItemsLen, fileSpecsLen) == OHOS::Request::QUERY_ERR) {
        return nullptr;
    }

    OHOS::NativeRdb::RdbPredicates rdbPredicates3("priority_table");
    rdbPredicates3.EqualTo("task_id", std::to_string(taskId))->And()->EqualTo("uid", std::to_string(uid));
    if (GetPriority(rdbPredicates3, taskInfo.commonData.priority) == OHOS::Request::QUERY_ERR) {
        return nullptr;
    }

    return BuildCTaskInfo(taskInfo);
}

CTaskInfo *Query(uint32_t taskId, Action queryAction)
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates1("request_task_info");
    rdbPredicates1.EqualTo("task_id", std::to_string(taskId));
    if (queryAction != Action::ANY) {
        rdbPredicates1.EqualTo("action", std::to_string(static_cast<uint8_t>(queryAction)));
    }
    int64_t formItemsLen = 0;
    int64_t fileSpecsLen = 0;
    TaskInfo taskInfo;
    if (QueryRequestTaskInfo(rdbPredicates1, taskInfo, formItemsLen, fileSpecsLen) == OHOS::Request::QUERY_ERR) {
        return nullptr;
    }
    OHOS::NativeRdb::RdbPredicates rdbPredicates2("task_info_attachment");
    rdbPredicates2.EqualTo("task_id", std::to_string(taskId));
    if (QueryTaskInfoAttachment(rdbPredicates2, taskInfo, fileSpecsLen) == OHOS::Request::QUERY_ERR) {
        return nullptr;
    }

    OHOS::NativeRdb::RdbPredicates rdbPredicates3("priority_table");
    rdbPredicates3.EqualTo("task_id", std::to_string(taskId));
    if (GetPriority(rdbPredicates3, taskInfo.commonData.priority) == OHOS::Request::QUERY_ERR) {
        return nullptr;
    }

    return BuildCTaskInfo(taskInfo);
}

CVectorWrapper Search(CFilter filter)
{
    CVectorWrapper cVectorWrapper;
    cVectorWrapper.ptr = nullptr;
    cVectorWrapper.len = 0;
    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task_info");
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
        REQUEST_HILOGE("result set is nullptr");
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

void GetCommonTaskInfo(std::shared_ptr<OHOS::NativeRdb::ResultSet> resultSet, TaskInfo &taskInfo)
{
    int64_t taskId = 0;
    int64_t uid = 0;
    int action = 0;
    int mode = 0;
    int64_t ctime = 0;
    int64_t mtime = 0;
    int reason = 0;
    int gauge = 0;
    int retry = 0;
    int64_t tries = 0;
    int version = 0;

    resultSet->GetLong(0, taskId); // Line 0 here is 'task_id'
    taskInfo.commonData.taskId = static_cast<uint32_t>(taskId);
    resultSet->GetLong(1, uid); // Line 1 here is 'uid'
    taskInfo.commonData.uid = static_cast<uint64_t>(uid);
    resultSet->GetInt(2, action); // Line 2 here is 'action'
    taskInfo.commonData.action = static_cast<uint8_t>(action);
    resultSet->GetInt(3, mode); // Line 3 here is 'mode'
    taskInfo.commonData.mode = static_cast<uint8_t>(mode);
    resultSet->GetLong(4, ctime); // Line 4 here is 'ctime'
    taskInfo.commonData.ctime = static_cast<uint64_t>(ctime);
    resultSet->GetLong(5, mtime); // Line 5 here is 'mtime'
    taskInfo.commonData.mtime = static_cast<uint64_t>(mtime);
    resultSet->GetInt(6, reason); // Line 6 here is 'reason'
    taskInfo.commonData.reason = static_cast<uint8_t>(reason);
    resultSet->GetInt(7, gauge); // Line 7 here is 'gauge'
    taskInfo.commonData.gauge = static_cast<bool>(gauge);
    resultSet->GetInt(8, retry); // Line 8 here is 'retry'
    taskInfo.commonData.retry = static_cast<bool>(retry);
    resultSet->GetLong(9, tries); // Line 9 here is 'tries'
    taskInfo.commonData.tries = static_cast<uint32_t>(tries);
    resultSet->GetInt(10, version); // Line 10 here is 'version'
    taskInfo.commonData.version = static_cast<uint8_t>(version);
}

int TouchRequestTaskInfo(const OHOS::NativeRdb::RdbPredicates &rdbPredicates, TaskInfo &taskInfo,
    int64_t &formItemsLen, int64_t &fileSpecsLen)
{
    auto resultSet = OHOS::Request::RequestDataBase::GetInstance().Query(rdbPredicates,
        { "task_id", "uid", "action", "mode", "ctime", "mtime", "reason", "gauge", "retry", "tries", "version", "url",
            "data", "titile", "description", "mime_type", "state", "idx", "total_processed", "sizes", "processed",
            "extras", "form_items_len", "file_specs_len" });
    if (resultSet == nullptr || resultSet->GoToFirstRow() != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("result set is nullptr or go to first row failed");
        return OHOS::Request::QUERY_ERR;
    }
    int state = 0;
    int64_t idx = 0;
    int64_t totalProcessed = 0;
    GetCommonTaskInfo(resultSet, taskInfo);
    resultSet->GetString(11, taskInfo.url); // Line 11 here is 'url'
    resultSet->GetString(12, taskInfo.data); // Line 12 here is 'data'
    resultSet->GetString(13, taskInfo.title); // Line 13 here is 'title'
    resultSet->GetString(14, taskInfo.description); // Line 14 here is 'description'
    resultSet->GetString(15, taskInfo.mimeType); // Line 15 here is 'mimeType'
    resultSet->GetInt(16, state); // Line 16 here is 'state'
    taskInfo.progress.commonData.state = static_cast<uint8_t>(state);
    resultSet->GetLong(17, idx); // Line 17 here is 'idx'
    taskInfo.progress.commonData.index = static_cast<uintptr_t>(idx);
    resultSet->GetLong(18, totalProcessed); // Line 18 here is 'totalProcessed'
    taskInfo.progress.commonData.totalProcessed = static_cast<uintptr_t>(totalProcessed);
    resultSet->GetString(19, taskInfo.progress.sizes); // Line 19 here is 'sizes'
    resultSet->GetString(20, taskInfo.progress.processed); // Line 20 here is 'processed'
    resultSet->GetString(21, taskInfo.progress.extras); // Line 21 here is 'extras'
    resultSet->GetLong(22, formItemsLen); // Line 22 here is 'formItemsLen'
    resultSet->GetLong(23, fileSpecsLen); // Line 23 here is 'fileSpecsLen'
    resultSet->Close();
    return OHOS::Request::QUERY_OK;
}

int QueryRequestTaskInfo(const OHOS::NativeRdb::RdbPredicates &rdbPredicates, TaskInfo &taskInfo,
    int64_t &formItemsLen, int64_t &fileSpecsLen)
{
    auto resultSet = OHOS::Request::RequestDataBase::GetInstance().Query(rdbPredicates,
        { "task_id", "uid", "action", "mode", "ctime", "mtime", "reason", "gauge", "retry", "tries", "version",
            "bundle", "titile", "description", "mime_type", "state", "idx", "total_processed", "sizes", "processed",
            "extras", "form_items_len", "file_specs_len" });
    if (resultSet == nullptr || resultSet->GoToFirstRow() != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("result set is nullptr or go to first row failed");
        return OHOS::Request::QUERY_ERR;
    }
    int state = 0;
    int64_t idx = 0;
    int64_t totalProcessed = 0;
    GetCommonTaskInfo(resultSet, taskInfo);
    resultSet->GetString(11, taskInfo.bundle); // Line 11 here is 'bundle'
    resultSet->GetString(12, taskInfo.title); // Line 12 here is 'title'
    resultSet->GetString(13, taskInfo.description); // Line 13 here is 'description'
    resultSet->GetString(14, taskInfo.mimeType); // Line 14 here is 'mimeType'
    resultSet->GetInt(15, state); // Line 15 here is 'state'
    taskInfo.progress.commonData.state = static_cast<uint8_t>(state);
    resultSet->GetLong(16, idx); // Line 16 here is 'idx'
    taskInfo.progress.commonData.index = static_cast<uintptr_t>(idx);
    resultSet->GetLong(17, totalProcessed); // Line 17 here is 'totalProcessed'
    taskInfo.progress.commonData.totalProcessed = static_cast<uintptr_t>(totalProcessed);
    resultSet->GetString(18, taskInfo.progress.sizes); // Line 18 here is 'sizes'
    resultSet->GetString(19, taskInfo.progress.processed); // Line 19 here is 'processed'
    resultSet->GetString(20, taskInfo.progress.extras); // Line 20 here is 'extras'
    resultSet->GetLong(21, formItemsLen); // Line 21 here is 'formItemsLen'
    resultSet->GetLong(22, fileSpecsLen); // Line 22 here is 'fileSpecsLen'
    resultSet->Close();
    return OHOS::Request::QUERY_OK;
}

int TouchTaskInfoAttachment(const OHOS::NativeRdb::RdbPredicates &rdbPredicates, TaskInfo &taskInfo,
    int64_t formItemsLen, int64_t fileSpecsLen)
{
    auto resultSet = OHOS::Request::RequestDataBase::GetInstance().Query(rdbPredicates,
        { "form_item_name", "value", "file_spec_name", "path", "file_name", "mime_type", "reason", "message" });
    if (resultSet == nullptr) {
        REQUEST_HILOGE("result set is nullptr");
        return OHOS::Request::QUERY_ERR;
    }
    int64_t len = std::max(formItemsLen, fileSpecsLen);
    for (int64_t i = 0; i < len; i++) {
        if (resultSet->GoToRow(i) != OHOS::NativeRdb::E_OK) {
            REQUEST_HILOGE("result set go to %{public}" PRId64 "row failed", i);
            return OHOS::Request::QUERY_ERR;
        }
        if (i < formItemsLen) {
            FormItem formItem;
            resultSet->GetString(0, formItem.name); // Line 0 here is 'name'
            resultSet->GetString(1, formItem.value); // Line 1 here is 'value'
            taskInfo.formItems.push_back(std::move(formItem));
        }
        if (i < fileSpecsLen) {
            FileSpec fileSpec;
            std::string path;
            resultSet->GetString(2, fileSpec.name); // Line 2 here is 'name'
            resultSet->GetString(3, path); // Line 3 here is 'path'
            resultSet->GetString(4, fileSpec.fileName); // Line 4 here is 'fileName'
            resultSet->GetString(5, fileSpec.mimeType); // Line 5 here is 'mimeType'
            fileSpec.path = path;
            taskInfo.fileSpecs.push_back(std::move(fileSpec));
            EachFileStatus eachFileStatus;
            eachFileStatus.path = std::move(path);
            int reason = 0;
            resultSet->GetInt(6, reason); // Line 6 here is 'reason'
            eachFileStatus.reason = static_cast<uint8_t>(reason);
            resultSet->GetString(7, eachFileStatus.message); // Line 7 here is 'message'
            taskInfo.eachFileStatus.push_back(std::move(eachFileStatus));
        }
    }
    resultSet->Close();
    return OHOS::Request::QUERY_OK;
}

int QueryTaskInfoAttachment(const OHOS::NativeRdb::RdbPredicates &rdbPredicates, TaskInfo &taskInfo,
    int64_t fileSpecsLen)
{
    auto resultSet =
        OHOS::Request::RequestDataBase::GetInstance().Query(rdbPredicates, { "path", "reason", "message" });
    if (resultSet == nullptr) {
        REQUEST_HILOGE("result set is nullptr");
        return OHOS::Request::QUERY_ERR;
    }
    for (int64_t i = 0; i < fileSpecsLen; i++) {
        if (resultSet->GoToRow(i) != OHOS::NativeRdb::E_OK) {
            REQUEST_HILOGE("result set go to %{public}" PRId64 "row failed", i);
            return OHOS::Request::QUERY_ERR;
        }
        EachFileStatus eachFileStatus;
        std::string path;
        resultSet->GetString(0, path);
        eachFileStatus.path = path;
        int reason = 0;
        resultSet->GetInt(1, reason); // Line 1 here is 'reason'
        eachFileStatus.reason = static_cast<uint8_t>(reason);
        resultSet->GetString(2, eachFileStatus.message); // Line 2 here is 'message'
        taskInfo.eachFileStatus.push_back(std::move(eachFileStatus));
        FileSpec fileSpec;
        fileSpec.path = std::move(path);
        taskInfo.fileSpecs.push_back(std::move(fileSpec));
    }
    resultSet->Close();
    return OHOS::Request::QUERY_OK;
}

int GetPriority(const OHOS::NativeRdb::RdbPredicates &rdbPredicates, uint32_t &priority)
{
    auto resultSet = OHOS::Request::RequestDataBase::GetInstance().Query(rdbPredicates, { "priority" });
    if (resultSet == nullptr) {
        REQUEST_HILOGE("result set is nullptr");
        return OHOS::Request::QUERY_ERR;
    }
    int64_t pri = 0;
    resultSet->GetLong(0, pri); // Line 0 here is 'priority'
    priority = static_cast<uint32_t>(pri);
    resultSet->Close();
    return OHOS::Request::QUERY_OK;
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

CProgress BuildCProgress(const Progress &progress)
{
    return CProgress{
        .commonData = progress.commonData,
        .sizes = WrapperCString(progress.sizes),
        .processed = WrapperCString(progress.processed),
        .extras = WrapperCString(progress.extras),
    };
}

bool HasTaskConfigRecord(uint32_t taskId)
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task_config");
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
    REQUEST_HILOGI("has the task record in task_config database");
    return true;
}

bool WriteRequestTaskConfig(CTaskConfig *taskConfig)
{
    REQUEST_HILOGI("write to request_task_config");
    OHOS::NativeRdb::ValuesBucket insertValues;
    insertValues.PutLong("task_id", taskConfig->commonData.taskId);
    insertValues.PutLong("uid", taskConfig->commonData.uid);
    insertValues.PutInt("action", taskConfig->commonData.action);
    insertValues.PutInt("mode", taskConfig->commonData.mode);
    insertValues.PutInt("cover", taskConfig->commonData.cover);
    insertValues.PutInt("network", taskConfig->commonData.network);
    insertValues.PutInt("meterd", taskConfig->commonData.meterd);
    insertValues.PutInt("roaming", taskConfig->commonData.roaming);
    insertValues.PutInt("retry", taskConfig->commonData.retry);
    insertValues.PutInt("redirect", taskConfig->commonData.redirect);
    insertValues.PutLong("idx", taskConfig->commonData.index);
    insertValues.PutLong("begins", taskConfig->commonData.begins);
    insertValues.PutLong("ends", taskConfig->commonData.ends);
    insertValues.PutInt("gauge", taskConfig->commonData.gauge);
    insertValues.PutInt("precise", taskConfig->commonData.precise);
    insertValues.PutInt("background", taskConfig->commonData.background);
    insertValues.PutString("bundle", std::string(taskConfig->bundle.cStr, taskConfig->bundle.len));
    insertValues.PutString("url", std::string(taskConfig->url.cStr, taskConfig->url.len));
    insertValues.PutString("titile", std::string(taskConfig->title.cStr, taskConfig->title.len));
    insertValues.PutString("description", std::string(taskConfig->description.cStr, taskConfig->description.len));
    insertValues.PutString("method", std::string(taskConfig->method.cStr, taskConfig->method.len));
    insertValues.PutString("headers", std::string(taskConfig->headers.cStr, taskConfig->headers.len));
    insertValues.PutString("data", std::string(taskConfig->data.cStr, taskConfig->data.len));
    insertValues.PutString("token", std::string(taskConfig->token.cStr, taskConfig->token.len));
    insertValues.PutString("extras", std::string(taskConfig->extras.cStr, taskConfig->extras.len));
    insertValues.PutInt("version", taskConfig->version);
    insertValues.PutLong("form_items_len", taskConfig->formItemsLen);
    insertValues.PutLong("file_specs_len", taskConfig->fileSpecsLen);
    insertValues.PutLong("body_file_names_len", taskConfig->bodyFileNamesLen);

    if (!OHOS::Request::RequestDataBase::GetInstance().Insert(std::string("request_task_config"), insertValues)) {
        REQUEST_HILOGE("insert to request_task_config failed");
        return false;
    }

    // Inserts temp_table.
    OHOS::NativeRdb::ValuesBucket insertValues2;
    insertValues2.PutLong("task_id", taskConfig->commonData.taskId);
    insertValues2.PutLong("uid", taskConfig->commonData.uid);
    insertValues2.PutLong("priority", taskConfig->commonData.priority);
    if (!OHOS::Request::RequestDataBase::GetInstance().Insert(std::string("priority_table"), insertValues2)) {
        REQUEST_HILOGE("insert to priority_table failed");
        return false;
    }

    REQUEST_HILOGI("insert to request_task_config success");
    return true;
}

bool WriteTaskConfigAttachment(CTaskConfig *taskConfig)
{
    REQUEST_HILOGD("write to task_config_attachment");
    uint64_t len = std::max({taskConfig->formItemsLen, taskConfig->fileSpecsLen,
        taskConfig->bodyFileNamesLen});
    for (uint64_t i = 0; i < len; i++) {
        OHOS::NativeRdb::ValuesBucket insertValues;
        insertValues.PutInt("task_id", taskConfig->commonData.taskId);
        insertValues.PutInt("uid", taskConfig->commonData.uid);
        if (i < taskConfig->formItemsLen) {
            insertValues.PutString("form_item_name",
                std::string(taskConfig->formItemsPtr[i].name.cStr, taskConfig->formItemsPtr[i].name.len));
            insertValues.PutString("value",
                std::string(taskConfig->formItemsPtr[i].value.cStr, taskConfig->formItemsPtr[i].value.len));
        }
        if (i < taskConfig->fileSpecsLen) {
            insertValues.PutString("file_spec_name",
                std::string(taskConfig->fileSpecsPtr[i].name.cStr, taskConfig->fileSpecsPtr[i].name.len));
            insertValues.PutString("path",
                std::string(taskConfig->fileSpecsPtr[i].path.cStr, taskConfig->fileSpecsPtr[i].path.len));
            insertValues.PutString("file_name",
                std::string(taskConfig->fileSpecsPtr[i].fileName.cStr, taskConfig->fileSpecsPtr[i].fileName.len));
            insertValues.PutString("mime_type",
                std::string(taskConfig->fileSpecsPtr[i].mimeType.cStr, taskConfig->fileSpecsPtr[i].mimeType.len));
        }
        if (i < taskConfig->bodyFileNamesLen) {
            insertValues.PutString("body_file_name",
                std::string(taskConfig->bodyFileNamesPtr[i].cStr, taskConfig->bodyFileNamesPtr[i].len));
        }
        if (!OHOS::Request::RequestDataBase::GetInstance().Insert(std::string("task_config_attachment"),
            insertValues)) {
            REQUEST_HILOGE("insert to task_config_attachment failed");
            return false;
        }
    }

    for (uint64_t i = 0; i < taskConfig->certsPathLen; i++) {
        OHOS::NativeRdb::ValuesBucket insertValues;
        insertValues.PutInt("task_id", taskConfig->commonData.taskId);
        insertValues.PutInt("uid", taskConfig->commonData.uid);
        insertValues.PutString("cert_path",
            std::string(taskConfig->certsPathPtr[i].cStr, taskConfig->certsPathPtr[i].len));
        if (!OHOS::Request::RequestDataBase::GetInstance().Insert(std::string("certs_table"),
            insertValues)) {
            REQUEST_HILOGE("insert to certs_table failed");
            return false;
        }
    }
    REQUEST_HILOGD("insert to task_config_attachment success");
    return true;
}

bool RecordRequestTaskConfig(CTaskConfig *taskConfig)
{
    return WriteRequestTaskConfig(taskConfig) && WriteTaskConfigAttachment(taskConfig);
}

void GetCommonTaskConfig(std::shared_ptr<OHOS::NativeRdb::ResultSet> resultSet, TaskConfig &taskConfig)
{
    int64_t taskId = 0;
    int64_t uid = 0;
    int action = 0;
    int mode = 0;
    int cover = 0;
    int network = 0;
    int meterd = 0;
    int roaming = 0;
    int retry = 0;
    int redirect = 0;
    int64_t index = 0;
    int64_t begins = 0;
    int64_t ends = 0;
    int gauge = 0;
    int precise = 0;
    int background = 0;

    resultSet->GetLong(0, taskId); // Line 0 here is 'taskId'
    taskConfig.commonData.taskId = static_cast<uint32_t>(taskId);
    resultSet->GetLong(1, uid); // Line 1 here is 'uid'
    taskConfig.commonData.uid = static_cast<uint64_t>(uid);
    resultSet->GetInt(2, action); // Line 2 here is 'action'
    taskConfig.commonData.action = static_cast<uint8_t>(action);
    resultSet->GetInt(3, mode); // Line 3 here is 'mode'
    taskConfig.commonData.mode = static_cast<uint8_t>(mode);
    resultSet->GetInt(4, cover); // Line 4 here is 'cover'
    taskConfig.commonData.cover = static_cast<bool>(cover);
    resultSet->GetInt(5, network); // Line 5 here is 'network'
    taskConfig.commonData.network = static_cast<uint8_t>(network);
    resultSet->GetInt(6, meterd); // Line 6 here is 'meterd'
    taskConfig.commonData.meterd = static_cast<bool>(meterd);
    resultSet->GetInt(7, roaming); // Line 7 here is 'roaming'
    taskConfig.commonData.roaming = static_cast<bool>(roaming);
    resultSet->GetInt(8, retry); // Line 8 here is 'retry'
    taskConfig.commonData.retry = static_cast<bool>(retry);
    resultSet->GetInt(9, redirect); // Line 9 here is 'redirect'
    taskConfig.commonData.redirect = static_cast<bool>(redirect);
    resultSet->GetLong(10, index); // Line 10 here is 'index'
    taskConfig.commonData.index = static_cast<uint32_t>(index);
    resultSet->GetLong(11, begins); // Line 11 here is 'begins'
    taskConfig.commonData.begins = static_cast<uint64_t>(begins);
    resultSet->GetLong(12, ends); // Line 12 here is 'ends'
    taskConfig.commonData.ends = static_cast<int64_t>(ends);
    resultSet->GetInt(13, gauge); // Line 13 here is 'gauge'
    taskConfig.commonData.gauge = static_cast<bool>(gauge);
    resultSet->GetInt(14, precise); // Line 14 here is 'precise'
    taskConfig.commonData.precise = static_cast<bool>(precise);
    resultSet->GetInt(15, background); // Line 15 here is 'background'
    taskConfig.commonData.background = static_cast<bool>(background);
}

CTaskConfig **QueryAllTaskConfig()
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task_config");
    std::vector<TaskConfig> taskConfigs;
    if (QueryRequestTaskConfig(rdbPredicates, taskConfigs) == OHOS::Request::QUERY_ERR) {
        return nullptr;
    }
    return BuildCTaskConfigs(taskConfigs);
}

int QueryTaskConfigLen()
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task_config");
    auto resultSet = OHOS::Request::RequestDataBase::GetInstance().Query(rdbPredicates, { "task_id", "uid" });
    int len = 0;
    if (resultSet == nullptr || resultSet->GetRowCount(len) != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("Get TaskConfigs length failed");
        return OHOS::Request::QUERY_ERR;
    }
    return len;
}

void QuerySingleTaskConfig(std::shared_ptr<OHOS::NativeRdb::ResultSet> resultSet, TaskConfig &taskConfig)
{
    resultSet->GetString(16, taskConfig.bundle); // Line 16 here is 'bundle'
    resultSet->GetString(17, taskConfig.url); // Line 17 here is 'url'
    resultSet->GetString(18, taskConfig.title); // Line 18 here is 'title'
    resultSet->GetString(19, taskConfig.description); // Line 19 here is 'description'
    resultSet->GetString(20, taskConfig.method); // Line 20 here is 'method'
    resultSet->GetString(21, taskConfig.headers); // Line 21 here is 'headers'
    resultSet->GetString(22, taskConfig.data); // Line 22 here is 'data'
    resultSet->GetString(23, taskConfig.token); // Line 23 here is 'token'
    resultSet->GetString(24, taskConfig.extras); // Line 24 here is 'extras'
}

int QueryRequestTaskConfig(const OHOS::NativeRdb::RdbPredicates &rdbPredicates, std::vector<TaskConfig> &taskConfigs)
{
    auto resultSet = OHOS::Request::RequestDataBase::GetInstance().Query(rdbPredicates,
        { "task_id", "uid", "action", "mode", "cover", "network", "meterd", "roaming", "retry", "redirect", "idx",
            "begins", "ends", "gauge", "precise", "background", "bundle", "url", "titile", "description", "method",
            "headers", "data", "token", "extras", "version",
            "form_items_len", "file_specs_len", "body_file_names_len" });
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
        TaskConfig taskConfig;
        int version = 0;
        int64_t formItemsLen = 0;
        int64_t fileSpecsLen = 0;
        int64_t bodyFileNamesLen = 0;
        GetCommonTaskConfig(resultSet, taskConfig);
        QuerySingleTaskConfig(resultSet, taskConfig);
        resultSet->GetInt(25, version); // Line 25 here is 'version'
        taskConfig.version = static_cast<uint8_t>(version);
        resultSet->GetLong(26, formItemsLen); // Line 26 here is 'formItemsLen'
        resultSet->GetLong(27, fileSpecsLen); // Line 27 here is 'fileSpecsLen'
        resultSet->GetLong(28, bodyFileNamesLen); // Line 28 here is 'bodyFileNamesLen'
        OHOS::NativeRdb::RdbPredicates attachPredicates("task_config_attachment");
        attachPredicates.EqualTo("task_id", std::to_string(taskConfig.commonData.taskId))
            ->And()->EqualTo("uid", std::to_string(taskConfig.commonData.uid));
        if (QueryTaskConfigAttachment(attachPredicates, taskConfig, formItemsLen,
            fileSpecsLen, bodyFileNamesLen) == OHOS::Request::QUERY_ERR) {
            return OHOS::Request::QUERY_ERR;
        }

        OHOS::NativeRdb::RdbPredicates attachPredicates2("priority_table");
        attachPredicates2.EqualTo("task_id", std::to_string(taskConfig.commonData.taskId))
                ->And()->EqualTo("uid", std::to_string(taskConfig.commonData.uid));
        if (GetPriority(attachPredicates, taskConfig.commonData.priority) == OHOS::Request::QUERY_ERR) {
            return OHOS::Request::QUERY_ERR;
        }

        OHOS::NativeRdb::RdbPredicates attachPredicates3("certs_table");
        attachPredicates3.EqualTo("task_id", std::to_string(taskConfig.commonData.taskId))
                ->And()->EqualTo("uid", std::to_string(taskConfig.commonData.uid));
        if (GetCertsPath(attachPredicates, taskConfig) == OHOS::Request::QUERY_ERR) {
            return OHOS::Request::QUERY_ERR;
        }
        taskConfigs.push_back(std::move(taskConfig));
    }
    resultSet->Close();
    return OHOS::Request::QUERY_OK;
}

int GetCertsPath(const OHOS::NativeRdb::RdbPredicates &rdbPredicates, TaskConfig &config)
{
    auto resultSet = OHOS::Request::RequestDataBase::GetInstance().Query(rdbPredicates, { "cert_path" });
    if (resultSet == nullptr) {
        REQUEST_HILOGE("result set is nullptr");
        return OHOS::Request::QUERY_ERR;
    }
    while (resultSet->GoToNextRow()) {
        std::string path;
        resultSet->GetString(0, path); // Line 0 here is 'path'
        config.certsPath.push_back(std::move(path));
    }
    resultSet->Close();
    return OHOS::Request::QUERY_OK;
}

int QueryTaskConfigAttachment(
    const OHOS::NativeRdb::RdbPredicates &rdbPredicates,
    TaskConfig &taskConfig, int64_t formItemsLen, int64_t fileSpecsLen,
    int64_t bodyFileNamesLen)
{
    auto resultSet = OHOS::Request::RequestDataBase::GetInstance().Query(rdbPredicates,
        { "form_item_name", "value", "file_spec_name", "path", "file_name", "mime_type",
		"body_file_name" });
    if (resultSet == nullptr) {
        REQUEST_HILOGE("ConfigAttach result set is nullptr");
        return OHOS::Request::QUERY_ERR;
    }
    int rowCount = 0;
    if (resultSet->GetRowCount(rowCount) != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGI("query task_config_attachment get row count failed");
    }
    int64_t len = std::max({formItemsLen, fileSpecsLen, bodyFileNamesLen});
    if (rowCount != len) {
        REQUEST_HILOGI("query task_config_attachment row count != max len");
        resultSet->Close();
        return OHOS::Request::QUERY_ERR;
    }
    for (int64_t i = 0; i < len; i++) {
        if (resultSet->GoToRow(i) != OHOS::NativeRdb::E_OK) {
            REQUEST_HILOGE("ConfigAttach result set go to %{public}" PRId64 "row failed", i);
            resultSet->Close();
            return OHOS::Request::QUERY_ERR;
        }
        if (i < formItemsLen) {
            FormItem formItem;
            resultSet->GetString(0, formItem.name); // Line 0 here is 'name'
            resultSet->GetString(1, formItem.value); // Line 1 here is 'value'
            taskConfig.formItems.push_back(std::move(formItem));
        }
        if (i < fileSpecsLen) {
            FileSpec fileSpec;
            resultSet->GetString(2, fileSpec.name); // Line 2 here is 'name'
            resultSet->GetString(3, fileSpec.path); // Line 3 here is 'path'
            resultSet->GetString(4, fileSpec.fileName); // Line 4 here is 'fileName'
            resultSet->GetString(5, fileSpec.mimeType); // Line 5 here is 'mimeType'
            taskConfig.fileSpecs.push_back(std::move(fileSpec));
        }
        if (i < bodyFileNamesLen) {
            std::string bodyFileName;
            resultSet->GetString(6, bodyFileName);
            taskConfig.bodyFileNames.push_back(std::move(bodyFileName));
        }
    }
    resultSet->Close();
    return OHOS::Request::QUERY_OK;
}

CTaskConfig **BuildCTaskConfigs(
    const std::vector<TaskConfig> &taskConfigs)
{
    CTaskConfig **cTaskConfigs = new CTaskConfig *[taskConfigs.size()];
    for (unsigned int i = 0; i < taskConfigs.size(); i++) {
        CTaskConfig *cTaskConfig = new CTaskConfig;
        const TaskConfig &taskConfig = taskConfigs[i];
        cTaskConfig->bundle = WrapperCString(taskConfig.bundle);
        cTaskConfig->url = WrapperCString(taskConfig.url);
        cTaskConfig->title = WrapperCString(taskConfig.title);
        cTaskConfig->description = WrapperCString(taskConfig.description);
        cTaskConfig->method = WrapperCString(taskConfig.method);
        cTaskConfig->headers = WrapperCString(taskConfig.headers);
        cTaskConfig->data = WrapperCString(taskConfig.data);
        cTaskConfig->token = WrapperCString(taskConfig.token);
        cTaskConfig->extras = WrapperCString(taskConfig.extras);
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
        cTaskConfigs[i] = std::move(cTaskConfig);
    }
    return cTaskConfigs;
}

bool CleanTaskConfigTable(uint32_t taskId, uint64_t uid)
{
    OHOS::NativeRdb::RdbPredicates predicates1("request_task_config");
    OHOS::NativeRdb::RdbPredicates predicates2("task_config_attachment");
    OHOS::NativeRdb::RdbPredicates predicates3("certs_table");
    predicates1.EqualTo("task_id", std::to_string(taskId))->And()->EqualTo("uid", std::to_string(uid));
    predicates2.EqualTo("task_id", std::to_string(taskId))->And()->EqualTo("uid", std::to_string(uid));
    predicates3.EqualTo("task_id", std::to_string(taskId))->And()->EqualTo("uid", std::to_string(uid));
    if (OHOS::Request::RequestDataBase::GetInstance().Delete(predicates1) &&
        OHOS::Request::RequestDataBase::GetInstance().Delete(predicates2) &&
        OHOS::Request::RequestDataBase::GetInstance().Delete(predicates3)) {
        REQUEST_HILOGE("task_config table deleted task_id: %{public}u", taskId);
        return true;
    }
    return false;
}