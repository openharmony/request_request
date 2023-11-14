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

#ifndef C_REQUEST_DATABASE_H
#define C_REQUEST_DATABASE_H

#include <cstdint>
#include <vector>

#include "c_enumration.h"
#include "c_filter.h"
#include "c_progress.h"
#include "c_task_info.h"
#include "c_task_config.h"
#include "rdb_errno.h"
#include "rdb_helper.h"
#include "rdb_open_callback.h"
#include "rdb_predicates.h"
#include "rdb_store.h"
#include "result_set.h"
#include "value_object.h"

namespace OHOS::Request {
constexpr const char *DB_NAME = "/data/service/el1/public/database/request/request.db";
constexpr int DATABASE_OPEN_VERSION = 1;
constexpr int DATABASE_NEW_VERSION = 2;
constexpr int QUERY_ERR = -1;
constexpr int QUERY_OK = 0;

constexpr const char *CREATE_REQUEST_TABLE1 = "CREATE TABLE IF NOT EXISTS request_task_info "
                                              "(id INTEGER PRIMARY KEY AUTOINCREMENT, "
                                              "task_id INTEGER, "
                                              "uid INTEGER, "
                                              "action INTEGER, "
                                              "mode INTEGER, "
                                              "ctime INTEGER, "
                                              "mtime INTEGER, "
                                              "reason INTEGER, "
                                              "gauge INTEGER, "
                                              "retry INTEGER, "
                                              "tries INTEGER, "
                                              "version INTEGER, "
                                              "bundle TEXT, "
                                              "url TEXT, "
                                              "data TEXT, "
                                              "token TEXT, "
                                              "titile TEXT, "
                                              "description TEXT, "
                                              "mime_type TEXT, "
                                              "state INTEGER, "
                                              "idx INTEGER, "
                                              "total_processed INTEGER, "
                                              "sizes TEXT, "
                                              "processed TEXT, "
                                              "extras TEXT, "
                                              "form_items_len INTEGER, "
                                              "file_specs_len INTEGER, "
                                              "body_file_names_len INTEGER)";

constexpr const char *CREATE_REQUEST_TABLE2 = "CREATE TABLE IF NOT EXISTS task_info_attachment "
                                              "(id INTEGER PRIMARY KEY AUTOINCREMENT, "
                                              "task_id INTEGER, "
                                              "uid INTEGER, "
                                              "form_item_name TEXT, "
                                              "value TEXT, "
                                              "file_spec_name TEXT, "
                                              "path TEXT, "
                                              "file_name TEXT, "
                                              "mime_type TEXT, "
                                              "reason INTEGER, "
                                              "message TEXT, "
                                              "body_file_name TEXT)";

constexpr const char *CREATE_REQUEST_TABLE3 = "CREATE TABLE IF NOT EXISTS request_task_config "
                                              "(id INTEGER PRIMARY KEY AUTOINCREMENT, "
                                              "task_id INTEGER, "
                                              "uid INTEGER, "
                                              "action INTEGER, "
                                              "mode INTEGER, "
                                              "cover INTEGER, "
                                              "network INTEGER, "
                                              "meterd INTEGER, "
                                              "roaming INTEGER, "
                                              "retry INTEGER, "
                                              "redirect INTEGER, "
                                              "idx INTEGER, "
                                              "begins INTEGER, "
                                              "ends INTEGER, "
                                              "gauge INTEGER, "
                                              "precise INTEGER, "
                                              "background INTEGER, "
                                              "bundle TEXT, "
                                              "url TEXT, "
                                              "titile TEXT, "
                                              "description TEXT, "
                                              "method TEXT, "
                                              "headers TEXT, "
                                              "data TEXT, "
                                              "token TEXT, "
                                              "extras TEXT, "
                                              "version INTEGER, "
                                              "form_items_len INTEGER, "
                                              "file_specs_len INTEGER, "
                                              "body_file_names_len INTEGER)";

class RequestDataBase {
public:
    static RequestDataBase &GetInstance();
    RequestDataBase(const RequestDataBase &) = delete;
    RequestDataBase &operator=(const RequestDataBase &) = delete;
    bool Insert(const std::string &table, const OHOS::NativeRdb::ValuesBucket &insertValues);
    bool Update(const OHOS::NativeRdb::ValuesBucket values, const OHOS::NativeRdb::AbsRdbPredicates &predicates);
    std::shared_ptr<OHOS::NativeRdb::ResultSet> Query(const OHOS::NativeRdb::AbsRdbPredicates &predicates,
        const std::vector<std::string> &columns);
    bool Delete(const OHOS::NativeRdb::AbsRdbPredicates &predicates);
    bool BeginTransaction();
    bool Commit();
    bool RollBack();

private:
    RequestDataBase();

private:
    std::shared_ptr<OHOS::NativeRdb::RdbStore> store_;
};

class RequestDBOpenCallback : public OHOS::NativeRdb::RdbOpenCallback {
public:
    int OnCreate(OHOS::NativeRdb::RdbStore &rdbStore) override;
    int OnUpgrade(OHOS::NativeRdb::RdbStore &rdbStore, int oldVersion, int newVersion) override;
    int OnDowngrade(OHOS::NativeRdb::RdbStore &rdbStore, int currentVersion, int targetVersion) override;
};
} // namespace OHOS::Request

#ifdef __cplusplus
extern "C" {
#endif

struct CVectorWrapper {
    uint32_t *ptr;
    uint64_t len;
};
bool HasRequestTaskRecord(uint32_t taskId);
bool RecordRequestTaskInfo(CTaskInfo *taskInfo);
bool UpdateRequestTaskInfo(uint32_t taskId, CUpdateInfo *updateInfo);
CTaskInfo *Touch(uint32_t taskId, uint64_t uid, CStringWrapper token);
CTaskInfo *Query(uint32_t taskId, Action queryAction);
CVectorWrapper Search(CFilter filter);
void DeleteCVectorWrapper(uint32_t *ptr);
void GetCommonTaskInfo(std::shared_ptr<OHOS::NativeRdb::ResultSet> resultSet, TaskInfo &taskInfo);
int TouchRequestTaskInfo(const OHOS::NativeRdb::RdbPredicates &rdbPredicates, TaskInfo &taskInfo,
    int64_t &formItemsLen, int64_t &fileSpecsLen);
int QueryRequestTaskInfo(const OHOS::NativeRdb::RdbPredicates &rdbPredicates, TaskInfo &taskInfo,
    int64_t &formItemsLen, int64_t &fileSpecsLen);
int TouchTaskInfoAttachment(const OHOS::NativeRdb::RdbPredicates &rdbPredicates, TaskInfo &taskInfo,
    int64_t formItemsLen, int64_t fileSpecsLen);
int QueryTaskInfoAttachment(const OHOS::NativeRdb::RdbPredicates &rdbPredicates, TaskInfo &taskInfo,
    int64_t fileSpecsLen);
CTaskInfo *BuildCTaskInfo(const TaskInfo &taskInfo);
CProgress BuildCProgress(const Progress &progress);
bool HasTaskConfigRecord(uint32_t taskId);
bool RecordRequestTaskConfig(CTaskConfig *taskConfig);
void GetCommonTaskConfig(std::shared_ptr<OHOS::NativeRdb::ResultSet> resultSet, TaskConfig &taskConfig);
CTaskConfig **QueryAllTaskConfig();
int QueryTaskConfigLen();
int QueryRequestTaskConfig(const OHOS::NativeRdb::RdbPredicates &rdbPredicates, std::vector<TaskConfig> &taskConfigs);
int QueryTaskConfigAttachment(const OHOS::NativeRdb::RdbPredicates &rdbPredicates, TaskConfig &taskConfig,
    int64_t formItemsLen, int64_t fileSpecsLen, int64_t bodyFileNamesLen);
CTaskConfig **BuildCTaskConfigs(const std::vector<TaskConfig> &taskConfigs);
bool CleanTaskConfigTable(uint32_t taskId, uint64_t uid);
void DeleteCTaskConfigs(CTaskConfig **ptr);

#ifdef __cplusplus
}
#endif
#endif // C_REQUEST_DATABASE_H