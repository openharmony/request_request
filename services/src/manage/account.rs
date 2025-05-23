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

use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::{Mutex, Once};

pub(crate) use ffi::*;

use super::database::RequestDb;
use crate::manage::events::TaskManagerEvent;
use crate::manage::task_manager::TaskManagerTx;
use crate::utils::{call_once, runtime_spawn};

#[derive(Debug)]
pub(crate) enum AccountEvent {
    Remove(i32),
    Changed,
}

pub(crate) static FOREGROUND_ACCOUNT: AtomicI32 = AtomicI32::new(0);
pub(crate) static BACKGROUND_ACCOUNTS: Mutex<Option<Vec<i32>>> = Mutex::new(None);
static UPDATE_FLAG: AtomicBool = AtomicBool::new(false);
static mut TASK_MANAGER_TX: Option<TaskManagerTx> = None;

pub(crate) fn remove_account_tasks(user_id: i32) {
    info!("delete database task, uid {}", user_id);
    let request_db = RequestDb::get_instance();
    request_db.delete_all_account_tasks(user_id);
}

pub(crate) fn update_accounts(task_manager: TaskManagerTx) {
    if UPDATE_FLAG
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        runtime_spawn(AccountUpdater::new(task_manager).update());
    }
}

pub(crate) fn query_active_accounts() -> (u64, HashSet<u64>) {
    let mut active_accounts = HashSet::new();
    let foreground_account = FOREGROUND_ACCOUNT.load(Ordering::SeqCst) as u64;
    active_accounts.insert(foreground_account);
    if let Some(background_accounts) = BACKGROUND_ACCOUNTS.lock().unwrap().as_ref() {
        for account in background_accounts.iter() {
            active_accounts.insert(*account as u64);
        }
    }
    (foreground_account, active_accounts)
}

struct AccountUpdater {
    change_flag: bool,
    task_manager: TaskManagerTx,
}

impl AccountUpdater {
    fn new(task_manager: TaskManagerTx) -> Self {
        Self {
            change_flag: false,
            task_manager,
        }
    }

    #[cfg_attr(not(feature = "oh"), allow(unused))]
    async fn update(mut self) {
        info!("AccountUpdate Start");
        let old_foreground = FOREGROUND_ACCOUNT.load(Ordering::SeqCst);
        let old_background = BACKGROUND_ACCOUNTS.lock().unwrap().clone();

        #[cfg(feature = "oh")]
        if let Some(foreground_account) = get_foreground_account().await {
            if old_foreground != foreground_account {
                self.change_flag = true;
                FOREGROUND_ACCOUNT.store(foreground_account, Ordering::SeqCst);
            }
        }

        #[cfg(feature = "oh")]
        if let Some(background_accounts) = get_background_accounts().await {
            if !old_background.is_some_and(|old_background| old_background == background_accounts) {
                self.change_flag = true;
                *BACKGROUND_ACCOUNTS.lock().unwrap() = Some(background_accounts);
            }
        }
    }
}

impl Drop for AccountUpdater {
    fn drop(&mut self) {
        info!("AccountUpdate Finished");
        UPDATE_FLAG.store(false, Ordering::SeqCst);
        if self.change_flag {
            info!("AccountInfo changed, notify task manager");
            self.task_manager
                .send_event(TaskManagerEvent::Account(AccountEvent::Changed));
        }
    }
}

#[cfg(feature = "oh")]
async fn get_foreground_account() -> Option<i32> {
    let mut foreground_account = 0;
    for i in 0..10 {
        let res = GetForegroundOsAccount(&mut foreground_account);
        if res == 0 {
            return Some(foreground_account);
        } else {
            error!("GetForegroundOsAccount failed: {} retry {} times", res, i);
            sys_event!(
                ExecFault,
                DfxCode::OS_ACCOUNT_FAULT_01,
                &format!("GetForegroundOsAccount failed: {} retry {} times", res, i)
            );
            ylong_runtime::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    }
    None
}

#[cfg(feature = "oh")]
async fn get_background_accounts() -> Option<Vec<i32>> {
    for i in 0..10 {
        let mut accounts = vec![];
        let res = GetBackgroundOsAccounts(&mut accounts);
        if res == 0 {
            return Some(accounts);
        } else {
            error!("GetBackgroundOsAccounts failed: {} retry {} times", res, i);
            sys_event!(
                ExecFault,
                DfxCode::INVALID_IPC_MESSAGE_A00,
                &format!("GetBackgroundOsAccounts failed: {} retry {} times", res, i)
            );

            ylong_runtime::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    }
    None
}

#[cfg(feature = "oh")]
pub(crate) fn registry_account_subscribe(task_manager: TaskManagerTx) {
    static ONCE: Once = Once::new();

    call_once(&ONCE, || unsafe {
        TASK_MANAGER_TX = Some(task_manager.clone());
    });

    info!("registry_account_subscribe");

    loop {
        let ret = RegistryAccountSubscriber(
            OS_ACCOUNT_SUBSCRIBE_TYPE::SWITCHED,
            Box::new(task_manager.clone()),
            |_, _| {},
            |_new_id, _old_id, task_manager| update_accounts(task_manager.clone()),
        );

        if ret != 0 {
            error!(
                "registry_account_switch_subscribe failed: {} retry 500ms later",
                ret
            );
            sys_event!(
                ExecFault,
                DfxCode::OS_ACCOUNT_FAULT_00,
                &format!(
                    "registry_account_switch_subscribe failed: {} retry 500ms later",
                    ret
                )
            );
            std::thread::sleep(std::time::Duration::from_millis(500));
        } else {
            break;
        }
    }

    loop {
        let ret = RegistryAccountSubscriber(
            OS_ACCOUNT_SUBSCRIBE_TYPE::ACTIVATED,
            Box::new(task_manager.clone()),
            |_id, task_manager| update_accounts(task_manager.clone()),
            |_, _, _| {},
        );

        if ret != 0 {
            error!(
                "registry_account_active_subscribe failed: {} retry 500ms later",
                ret
            );
            sys_event!(
                ExecFault,
                DfxCode::OS_ACCOUNT_FAULT_00,
                &format!(
                    "registry_account_active_subscribe failed: {} retry 500ms later",
                    ret
                )
            );
            std::thread::sleep(std::time::Duration::from_millis(500));
        } else {
            break;
        }
    }

    loop {
        let ret = RegistryAccountSubscriber(
            OS_ACCOUNT_SUBSCRIBE_TYPE::REMOVED,
            Box::new(task_manager.clone()),
            |id, task_manager| {
                task_manager.send_event(TaskManagerEvent::Account(AccountEvent::Remove(*id)));
            },
            |_, _, _| {},
        );

        if ret != 0 {
            error!(
                "registry_account_remove_subscribe failed: {} retry 500ms later",
                ret
            );
            sys_event!(
                ExecFault,
                DfxCode::OS_ACCOUNT_FAULT_00,
                &format!(
                    "registry_account_remove_subscribe failed: {} retry 500ms later",
                    ret
                )
            );

            std::thread::sleep(std::time::Duration::from_millis(500));
        } else {
            break;
        }
    }

    loop {
        let ret = RegistryAccountSubscriber(
            OS_ACCOUNT_SUBSCRIBE_TYPE::STOPPED,
            Box::new(task_manager.clone()),
            |_id, task_manager| update_accounts(task_manager.clone()),
            |_, _, _| {},
        );

        if ret != 0 {
            error!(
                "registry_account_stop_subscribe failed: {} retry 500ms later",
                ret
            );
            sys_event!(
                ExecFault,
                DfxCode::OS_ACCOUNT_FAULT_00,
                &format!(
                    "registry_account_stop_subscribe failed: {} retry 500ms later",
                    ret
                )
            );

            std::thread::sleep(std::time::Duration::from_millis(500));
        } else {
            break;
        }
    }

    update_accounts(task_manager.clone());
}

impl RequestDb {
    pub(crate) fn delete_all_account_tasks(&self, user_id: i32) {
        let sql = format!("DELETE from request_task WHERE uid/200000 = {}", user_id);
        if let Err(e) = self.execute(&sql) {
            error!("delete_all_account_tasks failed: {}", e);
            sys_event!(
                ExecFault,
                DfxCode::RDB_FAULT_04,
                &format!("delete_all_account_tasks failed: {}", e)
            );
        };
    }
}

#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {
    #[repr(i32)]
    enum OS_ACCOUNT_SUBSCRIBE_TYPE {
        INVALID_TYPE = -1,
        ACTIVATED = 0,
        ACTIVATING,
        UNLOCKED,
        CREATED,
        REMOVED,
        STOPPING,
        STOPPED,
        SWITCHING,
        SWITCHED,
    }

    extern "Rust" {
        type TaskManagerTx;
    }

    unsafe extern "C++" {
        include!("account.h");
        include!("os_account_subscribe_info.h");
        include!("c_request_database.h");

        type OS_ACCOUNT_SUBSCRIBE_TYPE;
        fn GetForegroundOsAccount(account: &mut i32) -> i32;
        fn GetBackgroundOsAccounts(accounts: &mut Vec<i32>) -> i32;

        fn RegistryAccountSubscriber(
            subscribe_type: OS_ACCOUNT_SUBSCRIBE_TYPE,
            task_manager: Box<TaskManagerTx>,
            on_accounts_changed: fn(&i32, task_manager: &TaskManagerTx),
            on_accounts_switch: fn(&i32, &i32, task_manager: &TaskManagerTx),
        ) -> i32;

        fn GetOhosAccountUid() -> String;
    }
}

#[cfg(feature = "oh")]
#[cfg(test)]
mod test {
    use ylong_runtime::sync::mpsc;

    use super::*;
    use crate::tests::test_init;

    #[test]
    fn ut_account_check_oh() {
        test_init();

        assert_eq!(0, FOREGROUND_ACCOUNT.load(Ordering::SeqCst));
        assert!(BACKGROUND_ACCOUNTS.lock().unwrap().is_none());

        let (tx, mut rx) = mpsc::unbounded_channel();
        let task_manager = TaskManagerTx { tx };
        registry_account_subscribe(task_manager);
        ylong_runtime::block_on(async {
            let msg = rx.recv().await.unwrap();
            assert!(matches!(
                msg,
                TaskManagerEvent::Account(AccountEvent::Changed)
            ));
            assert_ne!(FOREGROUND_ACCOUNT.load(Ordering::SeqCst), 0);
            assert!(BACKGROUND_ACCOUNTS.lock().unwrap().is_some());
        })
    }

    #[test]
    fn ut_account_update() {
        test_init();
        ylong_runtime::block_on(async {
            let (tx, mut rx) = mpsc::unbounded_channel();
            let task_manager = TaskManagerTx { tx };
            let updater = AccountUpdater::new(task_manager.clone());
            drop(updater);
            ylong_runtime::time::sleep(std::time::Duration::from_secs(2)).await;
            assert!(rx.is_empty());
            let mut updater = AccountUpdater::new(task_manager);
            updater.change_flag = true;
            drop(updater);
            let msg = rx.recv().await.unwrap();
            assert!(matches!(
                msg,
                TaskManagerEvent::Account(AccountEvent::Changed)
            ));
        })
    }

    #[test]
    fn ut_account_update_branch() {
        let old_background = Option::<Vec<i32>>::None;
        let background_accounts = vec![100];
        assert!(!old_background.is_some_and(|old_background| old_background == background_accounts));
        let old_background = Option::<Vec<i32>>::Some(vec![101]);
        assert!(!old_background.is_some_and(|old_background| old_background == background_accounts));
        let old_background = Option::<Vec<i32>>::Some(vec![100]);
        assert!(old_background.is_some_and(|old_background| old_background == background_accounts));
    }
}
