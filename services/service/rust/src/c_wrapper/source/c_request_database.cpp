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

#include "c_request_database.h"

#include <algorithm>
#include <cstdint>

#include "log.h"

namespace OHOS::Request {
std::shared_ptr<RequestDataBase> RequestDataBase::instance_ = nullptr;
std::shared_ptr<OHOS::NativeRdb::RdbStore> RequestDataBase::store_ = nullptr;

RequestDataBase::RequestDataBase()
{
    int errCode = OHOS::NativeRdb::E_OK;
    OHOS::NativeRdb::RdbStoreConfig config(DB_NAME);
    config.SetSecurityLevel(NativeRdb::SecurityLevel::S1);
//    config.SetEncryptStatus(true);
    RequestDBOpenCallback requestDBOpenCallback;
    store_ = OHOS::NativeRdb::RdbHelper::GetRdbStore(config, DATABASE_OPEN_VERSION, requestDBOpenCallback, errCode);
    REQUEST_HILOGI("get request database errcode :%{public}d", errCode);
}

std::shared_ptr<RequestDataBase> RequestDataBase::GetInstance()
{
    if (instance_ == nullptr) {
        instance_.reset(new RequestDataBase());
        return instance_;
    }
    return instance_;
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

bool HasTaskRecord(uint32_t taskId)
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates("request_task_info");
    rdbPredicates.EqualTo("task_id", std::to_string(taskId));
    auto resultSet = OHOS::Request::RequestDataBase::GetInstance()->Query(rdbPredicates, { "task_id" });
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
    if (!OHOS::Request::RequestDataBase::GetInstance()->BeginTransaction()) {
        return false;
    }
    OHOS::NativeRdb::ValuesBucket insertValues;
    insertValues.PutLong("task_id", taskInfo->common_data.task_id);
    insertValues.PutLong("uid", taskInfo->common_data.uid);
    insertValues.PutInt("action", taskInfo->common_data.action);
    insertValues.PutInt("mode", taskInfo->common_data.mode);
    insertValues.PutLong("ctime", taskInfo->common_data.ctime);
    insertValues.PutLong("mtime", taskInfo->common_data.mtime);
    insertValues.PutInt("reason", taskInfo->common_data.reason);
    insertValues.PutInt("gauge", taskInfo->common_data.gauge);
    insertValues.PutInt("retry", taskInfo->common_data.retry);
    insertValues.PutLong("tries", taskInfo->common_data.tries);
    insertValues.PutInt("version", taskInfo->common_data.version);
    insertValues.PutString("bundle", std::string(taskInfo->bundle.c_str, taskInfo->bundle.len));
    insertValues.PutString("url", std::string(taskInfo->url.c_str, taskInfo->url.len));
    insertValues.PutString("data", std::string(taskInfo->data.c_str, taskInfo->data.len));
    insertValues.PutString("token", std::string(taskInfo->token.c_str, taskInfo->token.len));
    insertValues.PutString("titile", std::string(taskInfo->title.c_str, taskInfo->title.len));
    insertValues.PutString("description", std::string(taskInfo->description.c_str, taskInfo->description.len));
    insertValues.PutString("mime_type", std::string(taskInfo->mime_type.c_str, taskInfo->mime_type.len));
    insertValues.PutInt("state", taskInfo->progress.common_data.state);
    insertValues.PutLong("idx", taskInfo->progress.common_data.index);
    insertValues.PutLong("total_processed", taskInfo->progress.common_data.total_processed);
    insertValues.PutString("sizes", std::string(taskInfo->progress.sizes.c_str, taskInfo->progress.sizes.len));
    insertValues.PutString("processed",
        std::string(taskInfo->progress.processed.c_str, taskInfo->progress.processed.len));
    insertValues.PutString("extras", std::string(taskInfo->progress.extras.c_str, taskInfo->progress.extras.len));
    insertValues.PutLong("form_items_len", taskInfo->form_items_len);
    insertValues.PutLong("file_specs_len", taskInfo->file_specs_len);
    if (!OHOS::Request::RequestDataBase::GetInstance()->Insert(std::string("request_task_info"), insertValues)) {
        REQUEST_HILOGE("insert to request_task_info failed");
        OHOS::Request::RequestDataBase::GetInstance()->RollBack();
        return false;
    }
    REQUEST_HILOGI("insert to request_task_info success");
    return OHOS::Request::RequestDataBase::GetInstance()->Commit();
}

bool WriteTaskInfoAttachment(CTaskInfo *taskInfo)
{
    REQUEST_HILOGI("write to task_info_attachment");
    if (!OHOS::Request::RequestDataBase::GetInstance()->BeginTransaction()) {
        return false;
    }
    uint64_t len = std::max(taskInfo->form_items_len, taskInfo->file_specs_len);
    for (uint64_t i = 0; i < len; i++) {
        OHOS::NativeRdb::ValuesBucket insertValues;
        insertValues.PutInt("task_id", taskInfo->common_data.task_id);
        insertValues.PutInt("uid", taskInfo->common_data.uid);
        if (i < taskInfo->form_items_len) {
            insertValues.PutString("form_item_name",
                std::string(taskInfo->form_items_ptr[i].name.c_str, taskInfo->form_items_ptr[i].name.len));
            insertValues.PutString("value",
                std::string(taskInfo->form_items_ptr[i].value.c_str, taskInfo->form_items_ptr[i].value.len));
        }
        if (i < taskInfo->file_specs_len) {
            insertValues.PutString("file_spec_name",
                std::string(taskInfo->file_specs_ptr[i].name.c_str, taskInfo->file_specs_ptr[i].name.len));
            insertValues.PutString("path",
                std::string(taskInfo->file_specs_ptr[i].path.c_str, taskInfo->file_specs_ptr[i].path.len));
            insertValues.PutString("file_name",
                std::string(taskInfo->file_specs_ptr[i].file_name.c_str, taskInfo->file_specs_ptr[i].file_name.len));
            insertValues.PutString("mime_type",
                std::string(taskInfo->file_specs_ptr[i].mime_type.c_str, taskInfo->file_specs_ptr[i].mime_type.len));
            insertValues.PutInt("reason", taskInfo->each_file_status_ptr[i].reason);
            insertValues.PutString("message", std::string(taskInfo->each_file_status_ptr[i].message.c_str,
                                                  taskInfo->each_file_status_ptr[i].message.len));
        }
        if (!OHOS::Request::RequestDataBase::GetInstance()->Insert(std::string("task_info_attachment"), insertValues)) {
            REQUEST_HILOGE("insert to task_info_attachment failed");
            OHOS::Request::RequestDataBase::GetInstance()->RollBack();
            return false;
        }
    }
    REQUEST_HILOGI("insert to task_info_attachment success");
    return OHOS::Request::RequestDataBase::GetInstance()->Commit();
}

bool RecordTaskInfo(CTaskInfo *taskInfo)
{
    return WriteRequestTaskInfo(taskInfo) && WriteTaskInfoAttachment(taskInfo);
}

bool UpdateTaskInfo(uint32_t taskId, CUpdateInfo *updateInfo)
{
    REQUEST_HILOGI("update task info");
    if (!OHOS::Request::RequestDataBase::GetInstance()->BeginTransaction()) {
        return false;
    }
    OHOS::NativeRdb::ValuesBucket values;
    values.PutLong("mtime", updateInfo->mtime);
    values.PutInt("reason", updateInfo->reason);
    values.PutLong("tries", updateInfo->tries);
    values.PutInt("state", updateInfo->progress.common_data.state);
    values.PutLong("idx", updateInfo->progress.common_data.index);
    values.PutLong("total_processed", updateInfo->progress.common_data.total_processed);
    values.PutString("sizes", std::string(updateInfo->progress.sizes.c_str, updateInfo->progress.sizes.len));
    values.PutString("processed",
        std::string(updateInfo->progress.processed.c_str, updateInfo->progress.processed.len));
    values.PutString("extras", std::string(updateInfo->progress.extras.c_str, updateInfo->progress.extras.len));

    OHOS::NativeRdb::RdbPredicates rdbPredicates1("request_task_info");
    rdbPredicates1.EqualTo("task_id", std::to_string(taskId));
    if (!OHOS::Request::RequestDataBase::GetInstance()->Update(values, rdbPredicates1)) {
        REQUEST_HILOGE("update table1 failed");
        OHOS::Request::RequestDataBase::GetInstance()->RollBack();
        return false;
    }
    for (uint32_t i = 0; i < updateInfo->each_file_status_len; i++) {
        OHOS::NativeRdb::ValuesBucket values1;
        values1.PutInt("reason", updateInfo->each_file_status_ptr[i].reason);
        values1.PutString("message", std::string(updateInfo->each_file_status_ptr[i].message.c_str,
                                         updateInfo->each_file_status_ptr[i].message.len));
        OHOS::NativeRdb::RdbPredicates rdbPredicates2("task_info_attachment");
        rdbPredicates2.EqualTo("task_id", std::to_string(taskId))
            ->And()
            ->EqualTo("path", std::string(updateInfo->each_file_status_ptr[i].path.c_str,
                                  updateInfo->each_file_status_ptr[i].path.len));
        if (!OHOS::Request::RequestDataBase::GetInstance()->Update(values1, rdbPredicates2)) {
            REQUEST_HILOGE("update table2 failed");
            OHOS::Request::RequestDataBase::GetInstance()->RollBack();
            return false;
        }
    }
    return OHOS::Request::RequestDataBase::GetInstance()->Commit();
}

CTaskInfo *Touch(uint32_t taskId, uint64_t uid, CStringWrapper token)
{
    OHOS::NativeRdb::RdbPredicates rdbPredicates1("request_task_info");
    rdbPredicates1.EqualTo("task_id", std::to_string(taskId))
        ->And()
        ->EqualTo("uid", std::to_string(uid))
        ->And()
        ->EqualTo("token", std::string(token.c_str, token.len));
    int64_t form_items_len = 0;
    int64_t file_specs_len = 0;
    TaskInfo taskInfo;
    if (TouchRequestTaskInfo(rdbPredicates1, taskInfo, form_items_len, file_specs_len) == OHOS::Request::QUERY_ERR) {
        return nullptr;
    }
    OHOS::NativeRdb::RdbPredicates rdbPredicates2("task_info_attachment");
    rdbPredicates2.EqualTo("task_id", std::to_string(taskId))->And()->EqualTo("uid", std::to_string(uid));
    if (TouchTaskInfoAttachment(rdbPredicates2, taskInfo, form_items_len, file_specs_len) == OHOS::Request::QUERY_ERR) {
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
    int64_t form_items_len = 0;
    int64_t file_specs_len = 0;
    TaskInfo taskInfo;
    if (QueryRequestTaskInfo(rdbPredicates1, taskInfo, form_items_len, file_specs_len) == OHOS::Request::QUERY_ERR) {
        return nullptr;
    }
    OHOS::NativeRdb::RdbPredicates rdbPredicates2("task_info_attachment");
    rdbPredicates2.EqualTo("task_id", std::to_string(taskId));
    if (QueryTaskInfoAttachment(rdbPredicates2, taskInfo, file_specs_len) == OHOS::Request::QUERY_ERR) {
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
    std::string bundle = std::string(filter.bundle.c_str, filter.bundle.len);
    rdbPredicates.Between("ctime", std::to_string(filter.common_data.after), std::to_string(filter.common_data.before));
    if (filter.common_data.state != static_cast<uint8_t>(State::ANY)) {
        rdbPredicates.EqualTo("state", std::to_string(filter.common_data.state));
    }
    if (filter.common_data.action != static_cast<uint8_t>(Action::ANY)) {
        rdbPredicates.EqualTo("action", std::to_string(filter.common_data.action));
    }
    if (filter.common_data.mode != static_cast<uint8_t>(Mode::ANY)) {
        rdbPredicates.EqualTo("mode", std::to_string(filter.common_data.mode));
    }
    if (bundle != "*") {
        rdbPredicates.EqualTo("bundle", bundle);
    }
    auto resultSet = OHOS::Request::RequestDataBase::GetInstance()->Query(rdbPredicates, { "task_id" });
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
    taskInfo.commonData.task_id = static_cast<uint32_t>(taskId);
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
    int64_t &form_items_len, int64_t &file_specs_len)
{
    auto resultSet = OHOS::Request::RequestDataBase::GetInstance()->Query(rdbPredicates,
        { "task_id", "uid", "action", "mode", "ctime", "mtime", "reason", "gauge", "retry", "tries", "version", "url",
            "data", "titile", "description", "mime_type", "state", "idx", "total_processed", "sizes", "processed",
            "extras", "form_items_len", "file_specs_len" });

    if (resultSet == nullptr || resultSet->GoToFirstRow() != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("result set is nullptr or go to first row failed");
        return OHOS::Request::QUERY_ERR;
    }

    int state = 0;
    int64_t idx = 0;
    int64_t total_processed = 0;

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
    resultSet->GetLong(18, total_processed);
    taskInfo.progress.commonData.total_processed = static_cast<uintptr_t>(total_processed);
    resultSet->GetString(19, taskInfo.progress.sizes);
    resultSet->GetString(20, taskInfo.progress.processed);
    resultSet->GetString(21, taskInfo.progress.extras);
    resultSet->GetLong(22, form_items_len);
    resultSet->GetLong(23, file_specs_len);
    resultSet->Close();
    return OHOS::Request::QUERY_OK;
}

int QueryRequestTaskInfo(const OHOS::NativeRdb::RdbPredicates &rdbPredicates, TaskInfo &taskInfo,
    int64_t &form_items_len, int64_t &file_specs_len)
{
    auto resultSet = OHOS::Request::RequestDataBase::GetInstance()->Query(rdbPredicates,
        { "task_id", "uid", "action", "mode", "ctime", "mtime", "reason", "gauge", "retry", "tries", "version",
            "bundle", "titile", "description", "mime_type", "state", "idx", "total_processed", "sizes", "processed",
            "extras", "form_items_len", "file_specs_len" });

    if (resultSet == nullptr || resultSet->GoToFirstRow() != OHOS::NativeRdb::E_OK) {
        REQUEST_HILOGE("result set is nullptr or go to first row failed");
        return OHOS::Request::QUERY_ERR;
    }
    int state = 0;
    int64_t idx = 0;
    int64_t total_processed = 0;
    GetCommonTaskInfo(resultSet, taskInfo);
    resultSet->GetString(11, taskInfo.bundle);
    resultSet->GetString(12, taskInfo.title);
    resultSet->GetString(13, taskInfo.description);
    resultSet->GetString(14, taskInfo.mimeType);
    resultSet->GetInt(15, state);
    taskInfo.progress.commonData.state = static_cast<uint8_t>(state);
    resultSet->GetLong(16, idx);
    taskInfo.progress.commonData.index = static_cast<uintptr_t>(idx);
    resultSet->GetLong(17, total_processed);
    taskInfo.progress.commonData.total_processed = static_cast<uintptr_t>(total_processed);
    resultSet->GetString(18, taskInfo.progress.sizes);
    resultSet->GetString(19, taskInfo.progress.processed);
    resultSet->GetString(20, taskInfo.progress.extras);
    resultSet->GetLong(21, form_items_len);
    resultSet->GetLong(22, file_specs_len);
    resultSet->Close();
    return OHOS::Request::QUERY_OK;
}

int TouchTaskInfoAttachment(const OHOS::NativeRdb::RdbPredicates &rdbPredicates, TaskInfo &taskInfo,
    int64_t form_items_len, int64_t file_specs_len)
{
    auto resultSet = OHOS::Request::RequestDataBase::GetInstance()->Query(rdbPredicates,
        { "form_item_name", "value", "file_spec_name", "path", "file_name", "mime_type", "reason", "message" });
    if (resultSet == nullptr) {
        REQUEST_HILOGE("result set is nullptr");
        return OHOS::Request::QUERY_ERR;
    }
    int64_t len = std::max(form_items_len, file_specs_len);
    for (int64_t i = 0; i < len; i++) {
        if (resultSet->GoToRow(i) != OHOS::NativeRdb::E_OK) {
            REQUEST_HILOGE("result set go to %{public}llu row failed", i);
            return OHOS::Request::QUERY_ERR;
        }
        if (i < form_items_len) {
            FormItem formItem;
            resultSet->GetString(0, formItem.name);
            resultSet->GetString(1, formItem.value);
            taskInfo.formItems.push_back(std::move(formItem));
        }
        if (i < file_specs_len) {
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
    int64_t file_specs_len)
{
    auto resultSet =
        OHOS::Request::RequestDataBase::GetInstance()->Query(rdbPredicates, { "path", "reason", "message" });
    if (resultSet == nullptr) {
        REQUEST_HILOGE("result set is nullptr");
        return OHOS::Request::QUERY_ERR;
    }
    for (int64_t i = 0; i < file_specs_len; i++) {
        if (resultSet->GoToRow(i) != OHOS::NativeRdb::E_OK) {
            REQUEST_HILOGE("result set go to %{public}llu row failed", i);
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
    uint32_t form_items_len = taskInfo.formItems.size();
    CFormItem *form_items_ptr = new CFormItem[form_items_len];
    for (uint32_t i = 0; i < form_items_len; i++) {
        form_items_ptr[i].name = WrapperCString(taskInfo.formItems[i].name);
        form_items_ptr[i].value = WrapperCString(taskInfo.formItems[i].value);
    }

    uint32_t file_specs_len = taskInfo.fileSpecs.size();
    CFileSpec *file_specs_ptr = new CFileSpec[file_specs_len];
    CEachFileStatus *each_file_status_ptr = new CEachFileStatus[file_specs_len];
    for (uint32_t i = 0; i < file_specs_len; i++) {
        file_specs_ptr[i].name = WrapperCString(taskInfo.fileSpecs[i].name);
        file_specs_ptr[i].path = WrapperCString(taskInfo.fileSpecs[i].path);
        file_specs_ptr[i].file_name = WrapperCString(taskInfo.fileSpecs[i].fileName);
        file_specs_ptr[i].mime_type = WrapperCString(taskInfo.fileSpecs[i].mimeType);
        each_file_status_ptr[i].path = WrapperCString(taskInfo.eachFileStatus[i].path);
        each_file_status_ptr[i].reason = taskInfo.eachFileStatus[i].reason;
        each_file_status_ptr[i].message = WrapperCString(taskInfo.eachFileStatus[i].message);
    }

    CTaskInfo *cTaskInfo = new CTaskInfo;
    cTaskInfo->bundle = WrapperCString(taskInfo.bundle);
    cTaskInfo->url = WrapperCString(taskInfo.url);
    cTaskInfo->data = WrapperCString(taskInfo.data);
    cTaskInfo->token = WrapperCString(taskInfo.token);
    cTaskInfo->form_items_ptr = form_items_ptr;
    cTaskInfo->form_items_len = form_items_len;
    cTaskInfo->file_specs_ptr = file_specs_ptr;
    cTaskInfo->file_specs_len = file_specs_len;
    cTaskInfo->title = WrapperCString(taskInfo.title);
    cTaskInfo->description = WrapperCString(taskInfo.description);
    cTaskInfo->mime_type = WrapperCString(taskInfo.mimeType);
    cTaskInfo->progress = BuildCProgress(taskInfo.progress);
    cTaskInfo->each_file_status_ptr = each_file_status_ptr;
    cTaskInfo->each_file_status_len = file_specs_len;
    cTaskInfo->common_data = taskInfo.commonData;
    return cTaskInfo;
}

CProgress BuildCProgress(const Progress &progress)
{
    return CProgress{
        .common_data = progress.commonData,
        .sizes = WrapperCString(progress.sizes),
        .processed = WrapperCString(progress.processed),
        .extras = WrapperCString(progress.extras),
    };
}