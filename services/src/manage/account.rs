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

use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::{Mutex, Once};

pub(crate) use ffi::*;

use super::database::RequestDb;
use super::TaskManager;
use crate::manage::events::TaskManagerEvent;
use crate::manage::task_manager::TaskManagerTx;
use crate::utils::runtime_spawn;

#[derive(Debug)]
pub(crate) enum AccountEvent {
    Remove(i32),
    Changed,
}

static FOREGROUND_ACCOUNT: AtomicI32 = AtomicI32::new(0);
static BACKGROUND_ACCOUNTS: Mutex<Option<Vec<i32>>> = Mutex::new(None);
static UPDATE_FLAG: AtomicBool = AtomicBool::new(false);
static mut TASK_MANAGER_TX: Option<TaskManagerTx> = None;

impl TaskManager {
    pub(crate) async fn handle_account_event(&mut self, event: AccountEvent) {
        match event {
            AccountEvent::Remove(user_id) => remove_account_tasks(user_id),
            AccountEvent::Changed => self.scheduler.on_user_change().await,
        }
    }
}

pub(crate) fn remove_account_tasks(user_id: i32) {
    info!("delete database task by user_id: {}", user_id);
    let mut request_db = RequestDb::get_instance();
    let res = request_db.delete_all_account_tasks(user_id);
    info!("delete data task finished: {}", res);
    if res != 0 {
        error!("delete account tasks failed: {}", res);
    }
}

pub(crate) fn is_foreground_user(uid: u64) -> bool {
    let foreground = FOREGROUND_ACCOUNT.load(Ordering::SeqCst);
    if foreground == 0 {
        error!("foreground account is none");
        if let Some(tx) = unsafe { TASK_MANAGER_TX.as_ref() } {
            update_accounts(tx.clone());
        }
        return true;
    }
    get_user_id_from_uid(uid) == foreground
}

pub(crate) fn is_background_user(uid: u64) -> bool {
    let background = BACKGROUND_ACCOUNTS.lock().unwrap();
    match background.as_ref() {
        Some(accounts) => accounts.contains(&get_user_id_from_uid(uid)),
        None => {
            error!("background accounts is empty");
            if let Some(tx) = unsafe { TASK_MANAGER_TX.as_ref() } {
                update_accounts(tx.clone());
            }
            true
        }
    }
}

pub(crate) fn is_system_user(uid: u64) -> bool {
    get_user_id_from_uid(uid) <= 100
}

pub(crate) fn is_active_user(uid: u64) -> bool {
    is_foreground_user(uid) || is_background_user(uid) || is_system_user(uid)
}

fn get_user_id_from_uid(uid: u64) -> i32 {
    const SYSTEM_USER_ID: i32 = 0;
    let mut user_id = 0;
    let res = GetOsAccountLocalIdFromUid(uid as i32, &mut user_id);
    if res != 0 {
        // When uid as i32 gets a negative number, it can goes to this branch.
        // maybe there's a memory problem happen.
        error!("GetOsAccountLocalIdFromUid failed: {}", res);
        return SYSTEM_USER_ID;
    }
    user_id
}

pub(crate) fn update_accounts(task_manager: TaskManagerTx) {
    if UPDATE_FLAG
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        runtime_spawn(AccountUpdater::new(task_manager).update());
    }
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

    async fn update(mut self) {
        info!("AccountUpdate Start");
        let old_forground = FOREGROUND_ACCOUNT.load(Ordering::SeqCst);
        let old_background = BACKGROUND_ACCOUNTS.lock().unwrap().clone();

        if let Some(foreground_account) = get_foreground_account().await {
            if old_forground != foreground_account {
                self.change_flag = true;
                FOREGROUND_ACCOUNT.store(foreground_account, Ordering::SeqCst);
                let request_db = RequestDb::get_instance();
                request_db.on_account_change(foreground_account);
            }
        }

        if let Some(background_accounts) = get_background_accounts().await {
            if !old_background.is_some_and(|old_background| old_background == background_accounts) {
                self.change_flag = true;
                let request_db = RequestDb::get_instance();
                for account in background_accounts.iter() {
                    request_db.on_account_change(*account);
                }
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
            info!("AccountInfo changed notify task manager");
            self.task_manager
                .send_event(TaskManagerEvent::Account(AccountEvent::Changed));
        }
    }
}

async fn get_foreground_account() -> Option<i32> {
    let mut foreground_account = 0;
    for i in 0..10 {
        let res = GetForegroundOsAccount(&mut foreground_account);
        if res == 0 {
            return Some(foreground_account);
        } else {
            error!("GetForegroundOsAccount failed: {} retry {} times", res, i);
            ylong_runtime::time::sleep(std::time::Duration::from_millis(500));
        }
    }
    None
}

async fn get_background_accounts() -> Option<Vec<i32>> {
    for i in 0..10 {
        let mut accounts = vec![];
        let res = GetBackgroundOsAccounts(&mut accounts);
        if res == 0 {
            return Some(accounts);
        } else {
            error!("GetBackgroundOsAccounts failed: {} retry {} times", res, i);
            ylong_runtime::time::sleep(std::time::Duration::from_millis(500));
        }
    }
    None
}

pub(crate) fn registry_account_subscribe(task_manager: TaskManagerTx) {
    static ONCE: Once = Once::new();

    ONCE.call_once(|| unsafe {
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
                "registry_account_switch_subscribe  failed: {} retry 500ms later",
                ret
            );
            std::thread::sleep(std::time::Duration::from_millis(500));
        } else {
            break;
        }
    }

    loop {
        let ret = RegistryAccountSubscriber(
            OS_ACCOUNT_SUBSCRIBE_TYPE::ACTIVED,
            Box::new(task_manager.clone()),
            |_id, task_manager| update_accounts(task_manager.clone()),
            |_, _, _| {},
        );

        if ret != 0 {
            error!(
                "registry_account_active_subscribe failed: {} retry 500ms later",
                ret
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
            std::thread::sleep(std::time::Duration::from_millis(500));
        } else {
            break;
        }
    }

    update_accounts(task_manager.clone());
}

impl RequestDb {
    pub(crate) fn on_account_change(&self, user_id: i32) {
        let res = unsafe { Pin::new_unchecked(&mut (*self.inner)).OnAccountChange(user_id) };
        if res != 0 {
            error!("on_account_change failed: {}", res);
        }
    }

    pub(crate) fn delete_all_account_tasks(&mut self, user_id: i32) -> i32 {
        unsafe { Pin::new_unchecked(&mut (*self.inner)).DeleteAllAccountTasks(user_id) }
    }
}

#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {

    #[repr(i32)]
    enum OS_ACCOUNT_SUBSCRIBE_TYPE {
        INVALID_TYPE = -1,
        ACTIVED = 0,
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
        type RequestDataBase = crate::manage::database::RequestDataBase;
        fn DeleteAllAccountTasks(self: Pin<&mut RequestDataBase>, user_id: i32) -> i32;
        fn OnAccountChange(self: Pin<&mut RequestDataBase>, user_id: i32) -> i32;

        fn GetForegroundOsAccount(account: &mut i32) -> i32;
        fn GetBackgroundOsAccounts(accounts: &mut Vec<i32>) -> i32;
        fn GetOsAccountLocalIdFromUid(uid: i32, user_id: &mut i32) -> i32;

        fn RegistryAccountSubscriber(
            subscribe_type: OS_ACCOUNT_SUBSCRIBE_TYPE,
            task_manager: Box<TaskManagerTx>,
            on_accounts_changed: fn(&i32, task_manager: &TaskManagerTx),
            on_accounts_switch: fn(&i32, &i32, task_manager: &TaskManagerTx),
        ) -> i32;

        fn GetOhosAccountUid() -> String;
    }
}

#[cfg(test)]
mod test {
    use ylong_runtime::sync::mpsc;

    use super::*;
    use crate::tests::test_init;

    const USER_100: u64 = 20012345;
    const USER_101: u64 = 20212345;
    const USER_SYSTEM: u64 = 2021234;

    #[test]
    fn ut_account_user_id() {
        assert_eq!(100, get_user_id_from_uid(USER_100));
        assert_eq!(101, get_user_id_from_uid(USER_101));
        assert_eq!(10, get_user_id_from_uid(USER_SYSTEM));
    }

    #[test]
    fn ut_account_check_oh() {
        test_init();

        assert_eq!(0, FOREGROUND_ACCOUNT.load(Ordering::SeqCst));
        assert!(BACKGROUND_ACCOUNTS.lock().unwrap().is_none());
        assert!(is_foreground_user(USER_100));
        assert!(is_system_user(USER_SYSTEM));

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
