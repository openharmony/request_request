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
#include "c_task_config.h"
#include "c_task_info.h"
#include "network_adapter.h"
#include "rdb_errno.h"
#include "rdb_helper.h"
#include "rdb_open_callback.h"
#include "rdb_predicates.h"
#include "rdb_store.h"
#include "result_set.h"
#include "value_object.h"

namespace OHOS::Request {
#ifndef REQUEST_DATABASE_TEST
constexpr const char *DB_NAME = "/data/service/el1/public/database/request/request.db";
constexpr int DATABASE_VERSION = 1;
#else
constexpr const char *DB_NAME = "/data/test/request.db";
constexpr int DATABASE_VERSION = 2;
#endif
constexpr const char *REQUEST_DATABASE_VERSION_4_1_RELEASE = "API11_4.1-release";
constexpr const char *REQUEST_DATABASE_VERSION = "API12_5.0-release";
constexpr const char *REQUEST_TASK_TABLE_NAME = "request_task";
constexpr int QUERY_ERR = -1;
constexpr int QUERY_OK = 0;
constexpr int WITHOUT_VERSION_TABLE = 40;
constexpr int API11_4_1_RELEASE = 41;
constexpr int API12_5_0_RELEASE = 50;
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

constexpr const char *REQUEST_TASK_TABLE_ADD_CERTIFICATE_PINS = "ALTER TABLE request_task ADD COLUMN certificate_pins "
                                                                "TEXT";

class RequestDataBase {
public:
    static RequestDataBase &GetInstance();
    RequestDataBase(const RequestDataBase &) = delete;
    RequestDataBase &operator=(const RequestDataBase &) = delete;
    bool Insert(const std::string &table, const OHOS::NativeRdb::ValuesBucket &insertValues);
    bool Update(const OHOS::NativeRdb::ValuesBucket values, const OHOS::NativeRdb::AbsRdbPredicates &predicates);
    std::shared_ptr<OHOS::NativeRdb::ResultSet> Query(
        const OHOS::NativeRdb::AbsRdbPredicates &predicates, const std::vector<std::string> &columns);
    bool Delete(const OHOS::NativeRdb::AbsRdbPredicates &predicates);

private:
    RequestDataBase();

private:
    std::shared_ptr<OHOS::NativeRdb::RdbStore> store_;
};

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
bool ChangeRequestTaskState(uint32_t taskId, uint64_t uid, State state, Reason reason);
bool HasRequestTaskRecord(uint32_t taskId);
bool QueryTaskTokenId(uint32_t taskId, uint64_t &tokenId);
bool RecordRequestTask(CTaskInfo *taskInfo, CTaskConfig *taskConfig);
bool UpdateRequestTask(uint32_t taskId, CUpdateInfo *updateInfo);
void DeleteCVectorWrapper(uint32_t *ptr);
void GetCommonTaskInfo(std::shared_ptr<OHOS::NativeRdb::ResultSet> resultSet, TaskInfo &taskInfo);
int TouchRequestTaskInfo(const OHOS::NativeRdb::RdbPredicates &rdbPredicates, TaskInfo &taskInfo);
int QueryRequestTaskInfo(const OHOS::NativeRdb::RdbPredicates &rdbPredicates, TaskInfo &taskInfo);
uint32_t QueryAppUncompletedTasksNum(uint64_t uid, uint8_t mode);
bool HasTaskConfigRecord(uint32_t taskId);
CTaskConfig **QueryAllTaskConfig(uint32_t &len);
CTaskConfig *QuerySingleTaskConfig(uint32_t taskId);
int QueryRequestTaskConfig(const OHOS::NativeRdb::RdbPredicates &rdbPredicates, std::vector<TaskConfig> &taskConfigs);
CTaskConfig **BuildCTaskConfigs(const std::vector<TaskConfig> &taskConfigs);
bool CleanTaskConfigTable(uint32_t taskId, uint64_t uid);
void RequestDBRemoveRecordsFromTime(uint64_t time);
int QueryTaskConfigLen();
uint32_t QueryAppUncompletedTasksNum(uint64_t uid, uint8_t mode);
CVectorWrapper Search(CFilter filter);
CTaskInfo *GetTaskInfo(uint32_t taskId);
CTaskConfig *QueryTaskConfig(uint32_t taskId);
CTaskConfig **QueryAllTaskConfigs(void);
void UpdateTaskStateOnAppStateChange(uint64_t uid, uint8_t appState);
void UpdateTaskStateOnNetworkChange(NetworkInfo info);
void GetTaskQosInfo(uint64_t uid, uint32_t taskId, TaskQosInfo **info);
void GetAppTaskQosInfos(uint64_t uid, TaskQosInfo **array, size_t *len);
void GetAppArray(AppInfo **apps, size_t *len);
CStringWrapper GetAppBundle(uint64_t uid);

#ifdef __cplusplus
}
#endif
#endif // C_REQUEST_DATABASE_H