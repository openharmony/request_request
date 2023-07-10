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
    REQUEST_HILOGI("request database insert ret is %{public}d", ret);
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
    REQUEST_HILOGI("request database update ret is %{public}d changedRows %{public}d", ret, changedRows);
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
    REQUEST_HILOGI("has the task record in database");
    return true;
}

bool WriteRequestTaskInfo(CTaskInfo *taskInfo)
{
    REQUEST_HILOGI("write to request_task_info");
    if (!OHOS::Request::RequestDataBase::GetInstance().BeginTransaction()) {
        return false;
    }
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
        OHOS::Request::RequestDataBase::GetInstance().RollBack();
        return false;
    }
    REQUEST_HILOGI("insert to request_task_info success");
    return OHOS::Request::RequestDataBase::GetInstance().Commit();
}

bool WriteTaskInfoAttachment(CTaskInfo *taskInfo)
{
    REQUEST_HILOGI("write to task_info_attachment");
    if (!OHOS::Request::RequestDataBase::GetInstance().BeginTransaction()) {
        return false;
    }
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
            OHOS::Request::RequestDataBase::GetInstance().RollBack();
            return false;
        }
    }
    REQUEST_HILOGI("insert to task_info_attachment success");
    return OHOS::Request::RequestDataBase::GetInstance().Commit();
}

bool RecordRequestTaskInfo(CTaskInfo *taskInfo)
{
    return WriteRequestTaskInfo(taskInfo) && WriteTaskInfoAttachment(taskInfo);
}

bool UpdateRequestTaskInfo(uint32_t taskId, CUpdateInfo *updateInfo)
{
    REQUEST_HILOGI("update task info");
    if (!OHOS::Request::RequestDataBase::GetInstance().BeginTransaction()) {
        return false;
    }
    OHOS::NativeRdb::ValuesBucket values;
    values.PutLong("mtime", updateInfo->mtime);
    values.PutInt("reason", updateInfo->reason);
    values.PutLong("tries", updateInfo->tries);
    values.PutInt("state", updateInfo->progress.commonData.state);
    values.PutLong("idx", updateInfo->progress.commonData.index);
    values.PutLong("total_processed", updateInfo->progress.commonData.totalProcessed);
    values.PutString("sizes", std::string(updateInfo->progress.sizes.cStr, updateInfo->progress.sizes.len));
    values.PutString("processed",
        std::string(updateInfo->progress.processed.cStr, updateInfo->progress.processed.len));
    values.PutString("extras", std::string(updateInfo->progress.extras.cStr, updateInfo->progress.extras.len));

    OHOS::NativeRdb::RdbPredicates rdbPredicates1("request_task_info");
    rdbPredicates1.EqualTo("task_id", std::to_string(taskId));
    if (!OHOS::Request::RequestDataBase::GetInstance().Update(values, rdbPredicates1)) {
        REQUEST_HILOGE("update table1 failed");
        OHOS::Request::RequestDataBase::GetInstance().RollBack();
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
            OHOS::Request::RequestDataBase::GetInstance().RollBack();
            return false;
        }
    }
    return OHOS::Request::RequestDataBase::GetInstance().Commit();
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
    cVectorWrapper.len = rowCount;
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

    resultSet->GetLong(0, taskId);
    taskInfo.commonData.taskId = static_cast<uint32_t>(taskId);
    resultSet->GetLong(1, uid);
    taskInfo.commonData.uid = static_cast<uint64_t>(uid);
    resultSet->GetInt(2, action);
    taskInfo.commonData.action = static_cast<uint8_t>(action);
    resultSet->GetInt(3, mode);
    taskInfo.commonData.mode = static_cast<uint8_t>(mode);
    resultSet->GetLong(4, ctime);
    taskInfo.commonData.ctime = static_cast<uint64_t>(ctime);
    resultSet->GetLong(5, mtime);
    taskInfo.commonData.mtime = static_cast<uint64_t>(mtime);
    resultSet->GetInt(6, reason);
    taskInfo.commonData.reason = static_cast<uint8_t>(reason);
    resultSet->GetInt(7, gauge);
    taskInfo.commonData.gauge = static_cast<bool>(gauge);
    resultSet->GetInt(8, retry);
    taskInfo.commonData.retry = static_cast<bool>(retry);
    resultSet->GetLong(9, tries);
    taskInfo.commonData.tries = static_cast<uint32_t>(tries);
    resultSet->GetInt(10, version);
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
    resultSet->GetString(11, taskInfo.url);
    resultSet->GetString(12, taskInfo.data);
    resultSet->GetString(13, taskInfo.title);
    resultSet->GetString(14, taskInfo.description);
    resultSet->GetString(15, taskInfo.mimeType);
    resultSet->GetInt(16, state);
    taskInfo.progress.commonData.state = static_cast<uint8_t>(state);
    resultSet->GetLong(17, idx);
    taskInfo.progress.commonData.index = static_cast<uintptr_t>(idx);
    resultSet->GetLong(18, totalProcessed);
    taskInfo.progress.commonData.totalProcessed = static_cast<uintptr_t>(totalProcessed);
    resultSet->GetString(19, taskInfo.progress.sizes);
    resultSet->GetString(20, taskInfo.progress.processed);
    resultSet->GetString(21, taskInfo.progress.extras);
    resultSet->GetLong(22, formItemsLen);
    resultSet->GetLong(23, fileSpecsLen);
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
    resultSet->GetString(11, taskInfo.bundle);
    resultSet->GetString(12, taskInfo.title);
    resultSet->GetString(13, taskInfo.description);
    resultSet->GetString(14, taskInfo.mimeType);
    resultSet->GetInt(15, state);
    taskInfo.progress.commonData.state = static_cast<uint8_t>(state);
    resultSet->GetLong(16, idx);
    taskInfo.progress.commonData.index = static_cast<uintptr_t>(idx);
    resultSet->GetLong(17, totalProcessed);
    taskInfo.progress.commonData.totalProcessed = static_cast<uintptr_t>(totalProcessed);
    resultSet->GetString(18, taskInfo.progress.sizes);
    resultSet->GetString(19, taskInfo.progress.processed);
    resultSet->GetString(20, taskInfo.progress.extras);
    resultSet->GetLong(21, formItemsLen);
    resultSet->GetLong(22, fileSpecsLen);
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
            resultSet->GetString(0, formItem.name);
            resultSet->GetString(1, formItem.value);
            taskInfo.formItems.push_back(std::move(formItem));
        }
        if (i < fileSpecsLen) {
            FileSpec fileSpec;
            std::string path;
            resultSet->GetString(2, fileSpec.name);
            resultSet->GetString(3, path);
            resultSet->GetString(4, fileSpec.fileName);
            resultSet->GetString(5, fileSpec.mimeType);
            fileSpec.path = path;
            taskInfo.fileSpecs.push_back(std::move(fileSpec));
            EachFileStatus eachFileStatus;
            eachFileStatus.path = std::move(path);
            int reason = 0;
            resultSet->GetInt(6, reason);
            eachFileStatus.reason = static_cast<uint8_t>(reason);
            resultSet->GetString(7, eachFileStatus.message);
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
        resultSet->GetInt(1, reason);
        eachFileStatus.reason = static_cast<uint8_t>(reason);
        resultSet->GetString(2, eachFileStatus.message);
        taskInfo.eachFileStatus.push_back(std::move(eachFileStatus));
        FileSpec fileSpec;
        fileSpec.path = std::move(path);
        taskInfo.fileSpecs.push_back(std::move(fileSpec));
    }
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