// Copyright (C) 2024 Huawei Device Co., Ltd.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::database::REQUEST_DB;

const CREATE_TASK_CONFIG_TABLE: &str =
    "CREATE TABLE IF NOT EXISTS task_config (task_id INTEGER PRIMARY KEY, display BOOLEAN)";

const CREATE_GROUP_TABLE: &str =
    "CREATE TABLE IF NOT EXISTS group_notification (task_id INTEGER PRIMARY KEY, group_id INTEGER)";

const CREATE_GROUP_CONFIG_TABLE: &str =
    "CREATE TABLE IF NOT EXISTS group_notification_config (group_id INTEGER PRIMARY KEY, gauge BOOLEAN, attach_able BOOLEAN, ctime INTEGER)";

const CREATE_TASK_CONTENT_TABLE: &str =
    "CREATE TABLE IF NOT EXISTS task_notification_content (task_id INTEGER PRIMARY KEY, title TEXT, text TEXT)";

const CREATE_GROUP_CONTENT_TABLE: &str =
    "CREATE TABLE IF NOT EXISTS group_notification_content (group_id INTEGER PRIMARY KEY, title TEXT, text TEXT)";

const ALERT_GROUP_CONFIG_TABLE: &str =
    "ALTER TABLE group_notification_config ADD COLUMN display BOOLEAN DEFAULT TRUE";

use std::time::{SystemTime, UNIX_EPOCH};

const MILLIS_IN_A_WEEK: u64 = 7 * 24 * 60 * 60 * 1000;

pub(crate) struct NotificationDb {
    inner: &'static rdb::RdbStore<'static>,
}

#[derive(Default, Clone)]
pub(crate) struct CustomizedNotification {
    pub title: Option<String>,
    pub text: Option<String>,
}

impl NotificationDb {
    pub(crate) fn new() -> Self {
        let me = Self { inner: &REQUEST_DB };
        if let Err(e) = me.create_db() {
            error!("Failed to create notification database: {}", e);
            sys_event!(
                ExecFault,
                DfxCode::RDB_FAULT_04,
                &format!("Failed to create notification database: {}", e)
            );
        }

        me.update();
        me
    }

    fn create_db(&self) -> Result<(), i32> {
        self.inner.execute(CREATE_TASK_CONFIG_TABLE, ())?;
        self.inner.execute(CREATE_GROUP_CONTENT_TABLE, ())?;
        self.inner.execute(CREATE_GROUP_TABLE, ())?;
        self.inner.execute(CREATE_TASK_CONTENT_TABLE, ())?;
        self.inner.execute(CREATE_GROUP_CONFIG_TABLE, ())?;
        Ok(())
    }

    fn update(&self) {
        let _ = self.inner.execute(ALERT_GROUP_CONFIG_TABLE, ());
    }

    pub(crate) fn clear_task_info(&self, task_id: u32) {
        let sqls = [
            "DELETE FROM task_config WHERE task_id = ?",
            "DELETE FROM task_notification_content WHERE task_id = ?",
            "DELETE FROM group_notification WHERE task_id = ?",
        ];
        for sql in sqls.iter() {
            if let Err(e) = self.inner.execute(sql, task_id) {
                error!("Failed to clear task {} notification info: {}", task_id, e);
            }
        }
    }

    pub(crate) fn clear_group_info(&self, group_id: u32) {
        let sqls = [
            "DELETE FROM group_notification WHERE group_id = ?",
            "DELETE FROM group_notification_content WHERE group_id = ?",
            "DELETE FROM group_notification_config WHERE group_id = ?",
        ];
        for sql in sqls.iter() {
            if let Err(e) = self.inner.execute(sql, group_id) {
                error!(
                    "Failed to clear group {} notification info: {}",
                    group_id, e
                );
            }
        }
    }

    pub(crate) fn clear_group_info_a_week_ago(&self) {
        let current_time = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration,
            Err(e) => {
                error!("Failed to get current time: {}", e);
                return;
            }
        }
        .as_millis() as u64;
        let group_ids = match self.inner.query::<u32>(
            "SELECT group_id FROM group_notification_config WHERE ctime < ?",
            current_time - MILLIS_IN_A_WEEK,
        ) {
            Ok(rows) => rows,
            Err(e) => {
                error!("Failed to clear group info: {}", e);
                return;
            }
        };
        for group_id in group_ids {
            let mut count = match self.inner.query::<u32>(
                "SELECT COUNT(*) FROM group_notification WHERE group_id = ?",
                group_id,
            ) {
                Ok(rows) => rows,
                Err(e) => {
                    error!("Failed to clear group info: {}", e);
                    continue;
                }
            };
            if !count.next().is_some_and(|x| x == 0) {
                continue;
            }

            info!(
                "clear group {} info for have been overdue for more than a week.",
                group_id
            );
            self.clear_group_info(group_id);
        }
    }

    pub(crate) fn check_group_notification_available(&self, group_id: &u32) -> bool {
        let mut set = match self.inner.query::<bool>(
            "SELECT display FROM group_notification_config WHERE group_id = ?",
            group_id,
        ) {
            Ok(set) => set,
            Err(e) => {
                error!("Failed to query group {} notification: {}", group_id, e);
                return true;
            }
        };
        set.next().unwrap_or(true)
    }

    pub(crate) fn check_task_notification_available(&self, task_id: &u32) -> bool {
        if let Some(group) = self.query_task_gid(*task_id) {
            return self.check_group_notification_available(&group);
        }

        let mut set = match self
            .inner
            .query::<bool>("SELECT display FROM task_config WHERE task_id = ?", task_id)
        {
            Ok(set) => set,
            Err(e) => {
                error!("Failed to query task {} notification: {}", task_id, e);
                return true;
            }
        };
        set.next().unwrap_or(true)
    }

    pub(crate) fn disable_task_notification(&self, task_id: u32) {
        if let Err(e) = self.inner.execute(
            "INSERT INTO task_config (task_id, display) VALUES (?, ?) ON CONFLICT(task_id) DO UPDATE SET display = excluded.display",
            (task_id, false),
        ) {
            error!("Failed to update {} notification: {}", task_id, e);
            sys_event!(ExecFault, DfxCode::RDB_FAULT_04, &format!("Failed to update {} notification: {}", task_id, e));
        }
    }

    pub(crate) fn update_task_group(&self, task_id: u32, group_id: u32) {
        if let Err(e) = self.inner.execute(
            "INSERT INTO group_notification (task_id, group_id) VALUES (?, ?) ON CONFLICT(task_id) DO UPDATE SET group_id = excluded.group_id",
            (task_id, group_id),
        ) {
            error!("Failed to update {} notification: {}", task_id, e);
            sys_event!(ExecFault, DfxCode::RDB_FAULT_04, &format!("Failed to update {} notification: {}", task_id, e));
        }
    }

    pub(crate) fn query_group_tasks(&self, group_id: u32) -> Vec<u32> {
        let set = match self.inner.query::<u32>(
            "SELECT task_id FROM group_notification WHERE group_id = ?",
            group_id,
        ) {
            Ok(set) => set,
            Err(e) => {
                error!("Failed to query group tasks: {}", e);
                sys_event!(
                    ExecFault,
                    DfxCode::RDB_FAULT_04,
                    &format!("Failed to query group tasks: {}", e)
                );
                return Vec::new();
            }
        };
        set.collect()
    }

    pub(crate) fn query_task_gid(&self, task_id: u32) -> Option<u32> {
        let mut set = match self.inner.query::<u32>(
            "SELECT group_id FROM group_notification WHERE task_id = ?",
            task_id,
        ) {
            Ok(set) => set,
            Err(e) => {
                error!("Failed to query task group id: {}", e);
                sys_event!(
                    ExecFault,
                    DfxCode::RDB_FAULT_04,
                    &format!("Failed to query task group id: {}", e)
                );
                return None;
            }
        };
        set.next()
    }

    pub(crate) fn query_task_customized_notification(
        &self,
        task_id: u32,
    ) -> Option<CustomizedNotification> {
        let mut set = match self.inner.query::<(Option<String>, Option<String>)>(
            "SELECT title, text FROM task_notification_content WHERE task_id = ?",
            task_id,
        ) {
            Ok(set) => set,
            Err(e) => {
                error!("Failed to query task customized notification: {}", e);
                sys_event!(
                    ExecFault,
                    DfxCode::RDB_FAULT_04,
                    &format!("Failed to query task customized notification: {}", e)
                );
                return None;
            }
        };
        set.next()
            .map(|(title, text)| CustomizedNotification { title, text })
    }

    pub(crate) fn update_task_customized_notification(
        &self,
        task_id: u32,
        title: Option<String>,
        text: Option<String>,
    ) {
        if let Err(e) = self.inner.execute(
            "INSERT INTO task_notification_content (task_id, title, text) VALUES (?, ?, ?) ON CONFLICT(task_id) DO UPDATE SET title = excluded.title, text = excluded.text",
            (task_id, title, text),
        ) {
            error!("Failed to insert {} notification: {}", task_id, e);
            sys_event!(ExecFault, DfxCode::RDB_FAULT_04, &format!("Failed to insert {} notification: {}", task_id, e));
        }
    }

    pub(crate) fn query_group_customized_notification(
        &self,
        group_id: u32,
    ) -> Option<CustomizedNotification> {
        let mut set = match self.inner.query::<(Option<String>, Option<String>)>(
            "SELECT title, text FROM group_notification_content WHERE group_id = ?",
            group_id,
        ) {
            Ok(set) => set,
            Err(e) => {
                error!("Failed to query task customized notification: {}", e);
                sys_event!(
                    ExecFault,
                    DfxCode::RDB_FAULT_04,
                    &format!("Failed to query task customized notification: {}", e)
                );
                return None;
            }
        };
        set.next()
            .map(|(title, text)| CustomizedNotification { title, text })
    }

    pub(crate) fn update_group_customized_notification(
        &self,
        group_id: u32,
        title: Option<String>,
        text: Option<String>,
    ) {
        if let Err(e) = self.inner.execute(
            "INSERT INTO group_notification_content (group_id, title, text) VALUES (?, ?, ?) ON CONFLICT(group_id) DO UPDATE SET title = excluded.title, text = excluded.text",
            (group_id, title, text),
        ) {
            error!("Failed to insert {} notification: {}", group_id, e);
            sys_event!(ExecFault, DfxCode::RDB_FAULT_04, &format!("Failed to insert {} notification: {}", group_id, e));
        }
    }

    pub(crate) fn update_group_config(
        &self,
        group_id: u32,
        gauge: bool,
        ctime: u64,
        display: bool,
    ) {
        if let Err(e) = self.inner.execute(
            "INSERT INTO group_notification_config (group_id, gauge, attach_able, ctime, display) VALUES (?, ?, ?, ?, ?) ON CONFLICT(group_id) DO UPDATE SET gauge = excluded.gauge , ctime = excluded.ctime, display = excluded.display",
            (group_id, gauge, true, ctime, display),
        ) {
            error!("Failed to update {} notification: {}", group_id, e);
            sys_event!(ExecFault, DfxCode::RDB_FAULT_04, &format!("Failed to update {} notification: {}", group_id, e));
        }
    }

    pub(crate) fn contains_group(&self, group_id: u32) -> bool {
        let mut set = match self.inner.query::<u32>(
            "SELECT group_id FROM group_notification_config where group_id = ?",
            group_id,
        ) {
            Ok(set) => set,
            Err(e) => {
                error!("Failed to query group {} notification: {}", group_id, e);
                return false;
            }
        };
        set.row_count() == 1
    }

    pub(crate) fn attach_able(&self, group_id: u32) -> bool {
        let mut set = match self.inner.query::<bool>(
            "SELECT attach_able FROM group_notification_config where group_id = ?",
            group_id,
        ) {
            Ok(set) => set,
            Err(e) => {
                error!("Failed to query group {} notification: {}", group_id, e);
                return false;
            }
        };
        set.next().unwrap_or(false)
    }

    pub(crate) fn disable_attach_group(&self, group_id: u32) {
        if let Err(e) = self.inner.execute(
            " UPDATE group_notification_config SET attach_able = ? where group_id = ?",
            (false, group_id),
        ) {
            error!("Failed to update {} notification: {}", group_id, e);
            sys_event!(
                ExecFault,
                DfxCode::RDB_FAULT_04,
                &format!("Failed to update {} notification: {}", group_id, e)
            );
        }
    }

    pub(crate) fn is_gauge(&self, group_id: u32) -> bool {
        let mut set = match self.inner.query::<bool>(
            "SELECT gauge FROM group_notification_config where group_id = ?",
            group_id,
        ) {
            Ok(set) => set,
            Err(e) => {
                error!("Failed to query group {} notification: {}", group_id, e);
                return false;
            }
        };
        set.next().unwrap_or(false)
    }
}

#[cfg(test)]
mod ut_database {
    include!("../../../tests/ut/service/notification_bar/ut_database.rs");
}
