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
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;

pub(crate) use ffi::*;

use super::database::RequestDb;
use super::TaskManager;
use crate::manage::events::TaskManagerEvent;
use crate::manage::task_manager::TaskManagerTx;

#[derive(Debug)]
pub(crate) enum AccountEvent {
    Switch,
    Active,
    Stop,
    Remove(i32),
}

static FOREGROUND_ACCOUNT: AtomicI32 = AtomicI32::new(-1);
static BACKGROUND_ACCOUNTS: Mutex<Option<Vec<i32>>> = Mutex::new(None);

impl TaskManager {
    pub(crate) async fn handle_account_event(&mut self, event: AccountEvent) {
        update_accounts();
        match event {
            AccountEvent::Remove(user_id) => remove_account_tasks(user_id),
            _ => self.scheduler.on_user_change().await,
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
    let foreground = FOREGROUND_ACCOUNT.load(Ordering::Acquire);
    if foreground == -1 {
        error!("foreground account is empty update accounts first");
        update_accounts();
        return is_foreground_user(uid);
    }
    get_user_id_from_uid(uid) == foreground
}

pub(crate) fn is_background_user(uid: u64) -> bool {
    let background = BACKGROUND_ACCOUNTS.lock().unwrap();
    match background.as_ref() {
        Some(accounts) => accounts.contains(&get_user_id_from_uid(uid)),
        None => {
            error!("background accounts is empty update accounts first");
            update_accounts();
            is_background_user(uid)
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
    let mut user_id = 0;
    let res = GetOsAccountLocalIdFromUid(uid as i32, &mut user_id);
    if res != 0 {
        error!("GetOsAccountLocalIdFromUid failed: {} retry", res);
        return get_user_id_from_uid(uid);
    }
    user_id
}

pub(crate) fn update_accounts() {
    let mut foreground_account = 0;
    let res = GetForegroundOsAccount(&mut foreground_account);
    if res != 0 {
        error!("GetForegroundOsAccount failed: {} retry", res);
        return update_accounts();
    }
    FOREGROUND_ACCOUNT.store(foreground_account, Ordering::Release);

    let request_db = RequestDb::get_instance();
    request_db.on_account_change(foreground_account);

    let mut new_accounts = vec![];
    let res = GetBackgroundOsAccounts(&mut new_accounts);
    if res != 0 {
        error!("GetBackgroundOsAccount failed: {} retry", res);
        return update_accounts();
    }

    for account in new_accounts.iter() {
        request_db.on_account_change(*account);
    }
    *BACKGROUND_ACCOUNTS.lock().unwrap() = Some(new_accounts);
}

pub(crate) fn registry_account_subscribe(task_manager: TaskManagerTx) {
    info!("registry_account_subscribe");

    loop {
        let ret = RegistryAccountSubscriber(
            OS_ACCOUNT_SUBSCRIBE_TYPE::SWITCHED,
            Box::new(task_manager.clone()),
            |_, _| {},
            |_new_id, _old_id, task_manager| {
                task_manager.send_event(TaskManagerEvent::Account(AccountEvent::Switch));
            },
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
            |_id, task_manager| {
                task_manager.send_event(TaskManagerEvent::Account(AccountEvent::Active));
            },
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
            |_id, task_manager| {
                task_manager.send_event(TaskManagerEvent::Account(AccountEvent::Stop));
            },
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

    update_accounts();
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
        assert_eq!(-1, FOREGROUND_ACCOUNT.load(Ordering::SeqCst));
        assert!(BACKGROUND_ACCOUNTS.lock().unwrap().is_none());
        assert!(is_foreground_user(USER_100));
        assert!(is_system_user(USER_SYSTEM));
        assert!(!is_background_user(USER_101));
        assert!(BACKGROUND_ACCOUNTS.lock().unwrap().is_some());
    }

    #[test]
    fn ut_account_registry_oh() {
        let (tx, _rx) = mpsc::unbounded_channel();
        let task_manager = TaskManagerTx { tx };
        test_init();
        registry_account_subscribe(task_manager)
    }
}
