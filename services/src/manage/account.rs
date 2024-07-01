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

const DEFAULT_USER_ID: i32 = 100;
static FOREGROUND_ACCOUNT: AtomicI32 = AtomicI32::new(DEFAULT_USER_ID);
static BACKGOUNFD_ACCOUNTS: Mutex<Vec<i32>> = Mutex::new(Vec::new());

#[allow(unreachable_pub)]
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
        include!("c_request_database.h");
        include!("os_account_subscribe_info.h");

        type OS_ACCOUNT_SUBSCRIBE_TYPE;
        type RequestDataBase;

        fn GetForegroundOsAccount(account: &mut i32) -> i32;
        fn GetBackgroundOsAccounts(accounts: &mut Vec<i32>) -> i32;
        fn GetOsAccountLocalIdFromUid(uid: i32, user_id: &mut i32) -> i32;
        fn GetDatabaseInstance() -> *mut RequestDataBase;
        fn DeleteAllAccountTasks(self: Pin<&mut RequestDataBase>, user_id: i32) -> i32;
        fn OnAccountChange(self: Pin<&mut RequestDataBase>, user_id: i32) -> i32;
        fn RegistryAccountSubscriber(
            subscribe_type: OS_ACCOUNT_SUBSCRIBE_TYPE,
            task_manager: Box<TaskManagerTx>,
            on_accounts_changed: fn(&i32, task_manager: &TaskManagerTx),
            on_accounts_switch: fn(&i32, &i32, task_manager: &TaskManagerTx),
        ) -> i32;
        fn GetOhosAccountUid() -> String;
    }
}

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
    info!("delte database task by user_id: {}", user_id);
    let res =
        unsafe { Pin::new_unchecked(&mut (*GetDatabaseInstance())).DeleteAllAccountTasks(user_id) };
    info!("delete data task finished: {}", res);
    if res != 0 {
        error!("delete account tasks failed: {}", res);
    }
}

pub(crate) fn is_foreground_user(uid: u64) -> bool {
    get_user_id_from_uid(uid) == FOREGROUND_ACCOUNT.load(Ordering::Acquire)
}

pub(crate) fn is_active_user(uid: u64) -> bool {
    let user_id = get_user_id_from_uid(uid);

    user_id <= 100
        || user_id == FOREGROUND_ACCOUNT.load(Ordering::Acquire)
        || BACKGOUNFD_ACCOUNTS.lock().unwrap().contains(&user_id)
}

fn get_user_id_from_uid(uid: u64) -> i32 {
    let mut user_id = 0;
    let res = GetOsAccountLocalIdFromUid(uid as i32, &mut user_id);
    if res != 0 {
        error!("GetOsAccountLocalIdFromUid failed: {}", res);
    }
    user_id
}

pub(crate) fn update_accounts() {
    let mut foreground_account = 0;
    let res = GetForegroundOsAccount(&mut foreground_account);
    if res != 0 {
        error!("GetForegroundOsAccount failed: {}", res);
        foreground_account = DEFAULT_USER_ID;
    }
    unsafe {
        Pin::new_unchecked(&mut (*GetDatabaseInstance())).OnAccountChange(foreground_account);
    }
    {
        let mut accounts = BACKGOUNFD_ACCOUNTS.lock().unwrap();
        accounts.clear();
        GetBackgroundOsAccounts(&mut accounts);
        for account in accounts.iter() {
            unsafe {
                Pin::new_unchecked(&mut (*GetDatabaseInstance())).OnAccountChange(*account);
            }
        }
    }
    FOREGROUND_ACCOUNT.store(foreground_account, Ordering::Release);
}

pub(crate) fn registry_account_subscribe(task_manager: TaskManagerTx) {
    info!("registry_account_subscribe");
    let ret = RegistryAccountSubscriber(
        OS_ACCOUNT_SUBSCRIBE_TYPE::SWITCHED,
        Box::new(task_manager.clone()),
        |_, _| {},
        |_new_id, _old_id, task_manager| {
            task_manager.send_event(TaskManagerEvent::Account(AccountEvent::Switch));
        },
    );

    if ret != 0 {
        error!("registry_account_switch_subscribe  failed: {}", ret);
    }

    let ret = RegistryAccountSubscriber(
        OS_ACCOUNT_SUBSCRIBE_TYPE::ACTIVED,
        Box::new(task_manager.clone()),
        |_id, task_manager| {
            task_manager.send_event(TaskManagerEvent::Account(AccountEvent::Active));
        },
        |_, _, _| {},
    );

    if ret != 0 {
        error!("registry_account_active_subscribe failed: {}", ret);
    }

    let ret = RegistryAccountSubscriber(
        OS_ACCOUNT_SUBSCRIBE_TYPE::REMOVED,
        Box::new(task_manager.clone()),
        |id, task_manager| {
            task_manager.send_event(TaskManagerEvent::Account(AccountEvent::Remove(*id)));
        },
        |_, _, _| {},
    );

    if ret != 0 {
        error!("registry_account_remove_subscribe failed: {}", ret);
    }

    let ret = RegistryAccountSubscriber(
        OS_ACCOUNT_SUBSCRIBE_TYPE::STOPPED,
        Box::new(task_manager),
        |_id, task_manager| {
            task_manager.send_event(TaskManagerEvent::Account(AccountEvent::Stop));
        },
        |_, _, _| {},
    );

    if ret != 0 {
        error!("registry_account_stop_subscribe failed: {}", ret);
    }
}
