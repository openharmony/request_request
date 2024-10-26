// Copyright (C) 2023 Huawei Device Co., Ltd.
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

cfg_oh! {
    pub(crate) mod config;
    pub(crate) use config::{SystemConfig, SystemConfigManager};
}

pub(crate) mod account;
pub(crate) mod app_state;
pub(crate) mod database;
pub(crate) mod events;
pub(crate) mod query;
pub(crate) use task_manager::TaskManager;
pub(crate) mod network;
pub(crate) mod network_manager;
pub(crate) mod notifier;
pub(crate) mod scheduler;
pub(crate) mod task_manager;

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::time::Duration;

    use ylong_runtime::sync::mpsc::unbounded_channel;

    use super::database::RequestDb;
    use super::network::{NetworkInfo, NetworkInner, NetworkType};
    use super::TaskManager;
    use crate::config::{Action, ConfigBuilder, Mode};
    use crate::error::ErrorCode;
    use crate::info::{State, TaskInfo};
    use crate::manage::events::{TaskEvent, TaskManagerEvent};
    use crate::manage::task_manager::{TaskManagerRx, TaskManagerTx};
    use crate::service::client::ClientManagerEntry;
    use crate::service::run_count::RunCountManagerEntry;
    use crate::task::reason::Reason;
    use crate::tests::{lock_database, test_init};

    const GITEE_FILE_LEN: u64 = 1042003;

    fn task_manager() -> TaskManager {
        let (tx, rx) = unbounded_channel();
        let task_manager_tx = TaskManagerTx::new(tx);
        let rx = TaskManagerRx::new(rx);
        let inner = NetworkInner::new();
        inner.notify_online(NetworkInfo {
            network_type: NetworkType::Wifi,
            is_metered: false,
            is_roaming: false,
        });
        let (tx, _rx) = unbounded_channel();
        let run_count = RunCountManagerEntry::new(tx);
        let (tx, _rx) = unbounded_channel();
        let client = ClientManagerEntry::new(tx);
        TaskManager::new(task_manager_tx, rx, run_count, client)
    }

    fn task_into(task_id: u32) -> TaskInfo {
        let db = RequestDb::get_instance();
        db.get_task_info(task_id).unwrap()
    }

    #[test]
    fn ut_manager_task_state_and_reason() {
        test_init();
        let _lock = lock_database();
        let mut manager = task_manager();
        let file_path = "test_files/ut_manager_task_state_and_reason.txt";

        let file = File::create(file_path).unwrap();
        let config = ConfigBuilder::new()
        .action(Action::Download)
        .retry(true)
        .mode(Mode::BackGround)
        .file_spec(file)
        .url("https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt")
        .redirect(true)
        .build();
        let uid = config.common_data.uid;
        let task_id = manager.create(config).unwrap();
        assert_eq!(
            task_into(task_id).progress.common_data.state,
            State::Initialized.repr
        );

        assert_eq!(task_into(task_id).common_data.reason, Reason::Default.repr);

        manager.start(uid, task_id);
        assert_eq!(
            task_into(task_id).progress.common_data.state,
            State::Waiting.repr
        );
        assert_eq!(
            task_into(task_id).common_data.reason,
            Reason::RunningTaskMeetLimits.repr
        );

        manager.pause(uid, task_id);
        assert_eq!(
            task_into(task_id).progress.common_data.state,
            State::Paused.repr
        );
        assert_eq!(
            task_into(task_id).common_data.reason,
            Reason::UserOperation.repr
        );

        manager.resume(uid, task_id);
        assert_eq!(
            task_into(task_id).progress.common_data.state,
            State::Waiting.repr
        );
        assert_eq!(
            task_into(task_id).common_data.reason,
            Reason::RunningTaskMeetLimits.repr
        );

        manager.pause(uid, task_id);
        assert_eq!(
            task_into(task_id).progress.common_data.state,
            State::Paused.repr
        );
        assert_eq!(
            task_into(task_id).common_data.reason,
            Reason::UserOperation.repr
        );

        manager.resume(uid, task_id);
        assert_eq!(
            task_into(task_id).progress.common_data.state,
            State::Waiting.repr
        );
        assert_eq!(
            task_into(task_id).common_data.reason,
            Reason::RunningTaskMeetLimits.repr
        );

        manager.stop(uid, task_id);
        assert_eq!(
            task_into(task_id).progress.common_data.state,
            State::Stopped.repr
        );
        assert_eq!(
            task_into(task_id).common_data.reason,
            Reason::UserOperation.repr
        );

        manager.start(uid, task_id);
        assert_eq!(
            task_into(task_id).progress.common_data.state,
            State::Waiting.repr
        );
        assert_eq!(
            task_into(task_id).common_data.reason,
            Reason::RunningTaskMeetLimits.repr
        );

        manager.scheduler.reschedule();

        ylong_runtime::block_on(async move {
            ylong_runtime::time::sleep(Duration::from_millis(500)).await;
            assert_eq!(
                task_into(task_id).progress.common_data.state,
                State::Running.repr
            );
            assert_eq!(task_into(task_id).common_data.reason, Reason::Default.repr);
            ylong_runtime::time::sleep(Duration::from_secs(10)).await;
            let msg = manager.rx.recv().await.unwrap();
            assert!(matches!(msg, TaskManagerEvent::Reschedule));
            let msg = manager.rx.recv().await.unwrap();
            assert!(matches!(
                msg,
                TaskManagerEvent::Task(TaskEvent::Completed(info_task_id, info_uid,Mode::BackGround)) if uid == info_uid && task_id == info_task_id
            ));
            let file = File::open(file_path).unwrap();
            assert_eq!(file.metadata().unwrap().len(), GITEE_FILE_LEN);
        });
    }

    #[test]
    fn ut_manager_state_change_error() {
        test_init();
        let _lock = lock_database();
        let mut manager = task_manager();
        let file_path = "test_files/ut_manager_state_change_error.txt";

        let file = File::create(file_path).unwrap();
        let config = ConfigBuilder::new()
        .action(Action::Download)
        .retry(true)
        .mode(Mode::BackGround)
        .file_spec(file)
        .url("https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt")
        .redirect(true)
        .build();
        let uid = config.common_data.uid;

        // initialized
        let task_id = manager.create(config.clone()).unwrap();
        assert_eq!(
            task_into(task_id).progress.common_data.state,
            State::Initialized.repr
        );
        assert_eq!(manager.pause(uid, task_id), ErrorCode::TaskStateErr);
        assert_eq!(manager.resume(uid, task_id), ErrorCode::TaskStateErr);
        assert_eq!(manager.stop(uid, task_id), ErrorCode::TaskStateErr);
        assert_eq!(manager.remove(uid, task_id), ErrorCode::ErrOk);
        assert_eq!(
            task_into(task_id).progress.common_data.state,
            State::Removed.repr
        );

        // started
        let task_id = manager.create(config.clone()).unwrap();
        assert_eq!(manager.start(uid, task_id), ErrorCode::ErrOk);
        assert_eq!(manager.resume(uid, task_id), ErrorCode::TaskStateErr);
        assert_eq!(manager.start(uid, task_id), ErrorCode::TaskStateErr);
        assert_eq!(manager.remove(uid, task_id), ErrorCode::ErrOk);
        assert_eq!(
            task_into(task_id).progress.common_data.state,
            State::Removed.repr
        );

        // paused
        let task_id = manager.create(config.clone()).unwrap();
        assert_eq!(manager.start(uid, task_id), ErrorCode::ErrOk);
        assert_eq!(manager.pause(uid, task_id), ErrorCode::ErrOk);
        assert_eq!(manager.pause(uid, task_id), ErrorCode::TaskStateErr);
        assert_eq!(manager.stop(uid, task_id), ErrorCode::TaskStateErr);
        assert_eq!(manager.start(uid, task_id), ErrorCode::TaskStateErr);
        assert_eq!(manager.remove(uid, task_id), ErrorCode::ErrOk);
        assert_eq!(
            task_into(task_id).progress.common_data.state,
            State::Removed.repr
        );

        // stopped
        let task_id = manager.create(config.clone()).unwrap();
        assert_eq!(manager.start(uid, task_id), ErrorCode::ErrOk);
        assert_eq!(manager.stop(uid, task_id), ErrorCode::ErrOk);
        assert_eq!(manager.pause(uid, task_id), ErrorCode::TaskStateErr);
        assert_eq!(manager.stop(uid, task_id), ErrorCode::TaskStateErr);
        assert_eq!(manager.resume(uid, task_id), ErrorCode::TaskStateErr);
        assert_eq!(manager.start(uid, task_id), ErrorCode::ErrOk);
        assert_eq!(manager.stop(uid, task_id), ErrorCode::ErrOk);
        assert_eq!(manager.remove(uid, task_id), ErrorCode::ErrOk);
        assert_eq!(
            task_into(task_id).progress.common_data.state,
            State::Removed.repr
        );

        // resumed
        let task_id = manager.create(config.clone()).unwrap();
        assert_eq!(manager.start(uid, task_id), ErrorCode::ErrOk);
        assert_eq!(manager.pause(uid, task_id), ErrorCode::ErrOk);
        assert_eq!(manager.resume(uid, task_id), ErrorCode::ErrOk);
        assert_eq!(manager.resume(uid, task_id), ErrorCode::TaskStateErr);
        assert_eq!(manager.start(uid, task_id), ErrorCode::TaskStateErr);
        assert_eq!(manager.pause(uid, task_id), ErrorCode::ErrOk);
        assert_eq!(manager.resume(uid, task_id), ErrorCode::ErrOk);
        assert_eq!(manager.remove(uid, task_id), ErrorCode::ErrOk);
        assert_eq!(
            task_into(task_id).progress.common_data.state,
            State::Removed.repr
        );
    }

    #[test]
    fn ut_manager_reschedule() {
        test_init();
        let _lock = lock_database();
        let mut manager = task_manager();
        let file_path = "test_files/ut_manager_reschedule.txt";

        let file = File::create(file_path).unwrap();
        let config = ConfigBuilder::new()
        .action(Action::Download)
        .retry(true)
        .mode(Mode::BackGround)
        .file_spec(file)
        .url("https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt")
        .redirect(true)
        .build();
        let uid = config.common_data.uid;
        let task_id = manager.create(config.clone()).unwrap();
        manager.rx.try_recv().unwrap_err();
        manager.start(uid, task_id);
        assert!(matches!(
            manager.rx.try_recv().unwrap(),
            TaskManagerEvent::Reschedule
        ));
        manager.scheduler.resort_scheduled = false;
        manager.stop(uid, task_id);
        assert!(matches!(
            manager.rx.try_recv().unwrap(),
            TaskManagerEvent::Reschedule
        ));
        manager.scheduler.resort_scheduled = false;
        manager.start(uid, task_id);
        assert!(matches!(
            manager.rx.try_recv().unwrap(),
            TaskManagerEvent::Reschedule
        ));
        manager.stop(uid, task_id);
        manager.rx.try_recv().unwrap_err();
        manager.scheduler.resort_scheduled = false;
        manager.start(uid, task_id);
        assert!(matches!(
            manager.rx.try_recv().unwrap(),
            TaskManagerEvent::Reschedule
        ));
        manager.scheduler.resort_scheduled = false;
        manager.pause(uid, task_id);
        assert!(matches!(
            manager.rx.try_recv().unwrap(),
            TaskManagerEvent::Reschedule
        ));
        manager.scheduler.resort_scheduled = false;
        manager.resume(uid, task_id);
        assert!(matches!(
            manager.rx.try_recv().unwrap(),
            TaskManagerEvent::Reschedule
        ));
    }
}
