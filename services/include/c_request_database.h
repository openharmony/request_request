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

#include "c_progress.h"
#include "c_task_config.h"
#include "c_task_info.h"
#include "cxx.h"
#include "rdb_errno.h"
#include "rdb_helper.h"
#include "rdb_open_callback.h"
#include "rdb_predicates.h"
#include "rdb_store.h"
#include "result_set.h"
#include "value_object.h"

namespace OHOS::Request {
constexpr const char *DB_NAME = "/data/service/el1/public/database/request/request.db";
constexpr int DATABASE_VERSION = 1;
constexpr const char *REQUEST_DATABASE_VERSION_4_1_RELEASE = "API11_4.1-release";
constexpr const char *REQUEST_DATABASE_VERSION_5_0_RELEASE = "API12_5.0-release";
constexpr const char *REQUEST_DATABASE_VERSION_5_1_RELEASE = "API16_5.1-release";
constexpr const char *REQUEST_DATABASE_VERSION = "API20_6.0-release";
constexpr const char *REQUEST_TASK_TABLE_NAME = "request_task";
constexpr int QUERY_ERR = -1;
constexpr int QUERY_OK = 0;
constexpr int WITHOUT_VERSION_TABLE = 40;
constexpr int API11_4_1_RELEASE = 41;
constexpr int API12_5_0_RELEASE = 50;
constexpr int API16_5_1_RELEASE = 51;
constexpr int API20_6_0_RELEASE = 60;
constexpr int INVALID_VERSION = -50;
constexpr int CHECK_VERSION_FAILED = -1;

constexpr const char *CHECK_REQUEST_VERSION = "SELECT name FROM sqlite_master WHERE type='table' AND "
                                              "name='request_version'";

constexpr const char *CREATE_REQUEST_VERSION_TABLE = "CREATE TABLE IF NOT EXISTS request_version "
                                                     "(id INTEGER PRIMARY KEY AUTOINCREMENT, "
                                                     "version TEXT, "
                                                     "task_table TEXT)";

constexpr const char *CREATE_REQUEST_TASK_TABLE = "CREATE TABLE IF NOT EXISTS request_task "
                                                  "(task_id INTEGER PRIMARY KEY, "
                                                  "uid INTEGER, "
                                                  "token_id INTEGER, "
                                                  "action INTEGER, "
                                                  "mode INTEGER, "
                                                  "cover INTEGER, "
                                                  "network INTEGER, "
                                                  "metered INTEGER, "
                                                  "roaming INTEGER, "
                                                  "ctime INTEGER, "
                                                  "mtime INTEGER, "
                                                  "reason INTEGER, "
                                                  "gauge INTEGER, "
                                                  "retry INTEGER, "
                                                  "redirect INTEGER, "
                                                  "tries INTEGER, "
                                                  "version INTEGER, "
                                                  "config_idx INTEGER, "
                                                  "begins INTEGER, "
                                                  "ends INTEGER, "
                                                  "precise INTEGER, "
                                                  "priority INTEGER, "
                                                  "background INTEGER, "
                                                  "bundle TEXT, "
                                                  "url TEXT, "
                                                  "data TEXT, "
                                                  "token TEXT, "
                                                  "title TEXT, "
                                                  "description TEXT, "
                                                  "method TEXT, "
                                                  "headers TEXT, "
                                                  "config_extras TEXT, "
                                                  "mime_type TEXT, "
                                                  "state INTEGER, "
                                                  "idx INTEGER, "
                                                  "total_processed INTEGER, "
                                                  "sizes TEXT, "
                                                  "processed TEXT, "
                                                  "extras TEXT, "
                                                  "form_items BLOB, "
                                                  "file_specs BLOB, "
                                                  "each_file_status BLOB, "
                                                  "body_file_names BLOB, "
                                                  "certs_paths BLOB)";

constexpr const char *REQUEST_TASK_TABLE_ADD_PROXY = "ALTER TABLE request_task ADD COLUMN proxy TEXT";

constexpr const char *REQUEST_TASK_TABLE_ADD_CERTIFICATE_PINS = "ALTER TABLE request_task ADD COLUMN "
                                                                "certificate_pins TEXT";
constexpr const char *REQUEST_TASK_TABLE_ADD_BUNDLE_TYPE = "ALTER TABLE request_task ADD COLUMN bundle_type TEXT";
constexpr const char *REQUEST_TASK_TABLE_ADD_ATOMIC_ACCOUNT = "ALTER TABLE request_task ADD COLUMN atomic_account "
                                                              "TEXT";

constexpr const char *REQUEST_TASK_TABLE_ADD_UID_INDEX = "CREATE INDEX uid_index on request_task(uid)";

constexpr const char *REQUEST_TASK_TABLE_ADD_MAX_SPEED = "ALTER TABLE request_task ADD COLUMN max_speed INTEGER";
constexpr const char *REQUEST_TASK_TABLE_ADD_MULTIPART = "ALTER TABLE request_task ADD COLUMN multipart INTEGER";
constexpr const char *REQUEST_TASK_TABLE_ADD_MIN_SPEED = "ALTER TABLE request_task ADD COLUMN min_speed INTEGER";
constexpr const char *REQUEST_TASK_TABLE_ADD_MIN_SPEED_DURATION = "ALTER TABLE request_task ADD COLUMN "
                                                                  "min_speed_duration INTEGER";
constexpr const char *REQUEST_TASK_TABLE_ADD_CONNECTION_TIMEOUT = "ALTER TABLE request_task ADD COLUMN "
                                                                  "connection_timeout INTEGER";
constexpr const char *REQUEST_TASK_TABLE_ADD_TOTAL_TIMEOUT = "ALTER TABLE request_task ADD COLUMN total_timeout "
                                                             "INTEGER";
constexpr const char *REQUEST_TASK_TABLE_ADD_TASK_TIME = "ALTER TABLE request_task ADD COLUMN task_time "
                                                         "INTEGER";

constexpr const char *REQUEST_TASK_TABLE_COL_PROXY = "proxy";
constexpr const char *REQUEST_TASK_TABLE_COL_CERTIFICATE_PINS = "certificate_pins";
constexpr const char *REQUEST_TASK_TABLE_COL_BUNDLE_TYPE = "bundle_type";
constexpr const char *REQUEST_TASK_TABLE_COL_ATOMIC_ACCOUNT = "atomic_account";
constexpr const char *REQUEST_TASK_TABLE_COL_MAX_SPEED = "max_speed";
constexpr const char *REQUEST_TASK_TABLE_COL_MULTIPART = "multipart";
constexpr const char *REQUEST_TASK_TABLE_COL_MIN_SPEED = "min_speed";
constexpr const char *REQUEST_TASK_TABLE_COL_MIN_SPEED_DURATION = "min_speed_duration";
constexpr const char *REQUEST_TASK_TABLE_COL_CONNECTION_TIMEOUT = "connection_timeout";
constexpr const char *REQUEST_TASK_TABLE_COL_TOTAL_TIMEOUT = "total_timeout";
constexpr const char *REQUEST_TASK_TABLE_COL_TASK_TIME = "task_time";

struct TaskFilter;
struct NetworkInfo;
struct TaskQosInfo;
class RequestDataBase {
public:
    static RequestDataBase &GetInstance(std::string path, bool encryptStatus);
    RequestDataBase(const RequestDataBase &) = delete;
    RequestDataBase &operator=(const RequestDataBase &) = delete;
    bool Insert(const std::string &table, const OHOS::NativeRdb::ValuesBucket &insertValues);
    bool Update(const OHOS::NativeRdb::ValuesBucket values, const OHOS::NativeRdb::AbsRdbPredicates &predicates);
    std::shared_ptr<OHOS::NativeRdb::ResultSet> Query(
        const OHOS::NativeRdb::AbsRdbPredicates &predicates, const std::vector<std::string> &columns);
    bool Delete(const OHOS::NativeRdb::AbsRdbPredicates &predicates);
    int ExecuteSql(rust::str sql);
    int QueryInteger(rust::str sql, rust::vec<rust::i64> &res);
    int QueryText(rust::str sql, rust::vec<rust::string> &res);
    int GetAppTaskQosInfos(rust::str sql, rust::vec<TaskQosInfo> &res);
    int GetTaskQosInfo(rust::str sql, TaskQosInfo &res);
    void CheckAndRebuildDataBase(int errCode);

private:
    RequestDataBase(std::string path, bool encryptStatus);

private:
    std::shared_ptr<OHOS::NativeRdb::RdbStore> store_;
};

inline RequestDataBase *GetDatabaseInstance(rust::str path, bool encryptStatus)
{
    return &RequestDataBase::GetInstance(std::string(path), encryptStatus);
}

class RequestDBOpenCallback : public OHOS::NativeRdb::RdbOpenCallback {
public:
    int OnCreate(OHOS::NativeRdb::RdbStore &rdbStore) override;
    int OnOpen(OHOS::NativeRdb::RdbStore &rdbStore) override;
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

// Request Database Modify.
bool RecordRequestTask(CTaskInfo *taskInfo, CTaskConfig *taskConfig);
bool UpdateRequestTask(uint32_t taskId, CUpdateInfo *updateInfo);
bool UpdateRequestTaskTime(uint32_t taskId, uint64_t taskTime);
bool UpdateRequestTaskState(uint32_t taskId, CUpdateStateInfo *updateStateInfo);
void RequestDBRemoveRecordsFromTime(uint64_t time);
CTaskInfo *GetTaskInfo(uint32_t taskId);
CTaskConfig *QueryTaskConfig(uint32_t taskId);

#ifdef __cplusplus
}
#endif
#endif // C_REQUEST_DATABASE_H