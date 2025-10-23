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

//! Account management for task scheduling and user isolation.
//! 
//! This module handles OS account state tracking, subscription to account events,
//! and management of tasks associated with specific user accounts. It ensures tasks
//! are properly isolated between user accounts and handles account lifecycle events.

use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::{Mutex, Once};

pub(crate) use ffi::*;

use super::database::RequestDb;
use crate::manage::events::TaskManagerEvent;
use crate::manage::task_manager::TaskManagerTx;
use crate::utils::{call_once, runtime_spawn};

/// Account-related events that require task manager attention.
#[derive(Debug)]
pub(crate) enum AccountEvent {
    /// Triggered when a user account is removed from the system.
    Remove(i32),
    /// Triggered when account state (foreground/background) changes.
    Changed,
}

/// Currently active foreground user account ID.
pub(crate) static FOREGROUND_ACCOUNT: AtomicI32 = AtomicI32::new(0);

/// List of background user account IDs that are active but not in foreground.
pub(crate) static BACKGROUND_ACCOUNTS: Mutex<Option<Vec<i32>>> = Mutex::new(None);

/// Flag indicating if an account update operation is in progress.
static UPDATE_FLAG: AtomicBool = AtomicBool::new(false);

/// Task manager transmitter for account event notifications.
/// 
/// # Safety
/// 
/// This static variable is accessed using unsafe operations and should only be
/// modified during initialization via `registry_account_subscribe`.
static mut TASK_MANAGER_TX: Option<TaskManagerTx> = None;

/// Removes all tasks associated with the specified user account.
/// 
/// # Arguments
/// 
/// * `user_id` - The identifier of the user account whose tasks should be removed.
/// 
/// # Notes
/// 
/// This function is typically called when a user account is removed from the system
/// to ensure proper cleanup of associated resources. It deletes all tasks in the
/// database belonging to the specified user account.
pub(crate) fn remove_account_tasks(user_id: i32) {
    info!("delete database task, uid {}", user_id);
    let request_db = RequestDb::get_instance();
    request_db.delete_all_account_tasks(user_id);
}

/// Initiates an asynchronous update of account information.
/// 
/// # Arguments
/// 
/// * `task_manager` - Transmitter for sending account change events to the task manager.
/// 
/// # Notes
/// 
/// This function ensures only one account update operation runs at a time using
/// an atomic flag. If an update is already in progress, this call will be ignored.
/// The actual update is performed asynchronously in a separate task.
pub(crate) fn update_accounts(task_manager: TaskManagerTx) {
    // Use compare_exchange to ensure only one update runs at a time
    if UPDATE_FLAG
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        runtime_spawn(AccountUpdater::new(task_manager).update());
    }
}

/// Retrieves the current set of active accounts.
/// 
/// # Returns
/// 
/// A tuple containing:
/// - The ID of the currently active foreground account as a `u64`
/// - A `HashSet` of all active account IDs (both foreground and background)
/// 
/// # Notes
/// 
/// This function safely accesses the global account state to provide the current
/// active account information. This is typically used for task filtering and
/// permission checks based on user identity.
pub(crate) fn query_active_accounts() -> (u64, HashSet<u64>) {
    let mut active_accounts = HashSet::new();
    let foreground_account = FOREGROUND_ACCOUNT.load(Ordering::SeqCst) as u64;
    active_accounts.insert(foreground_account);
    
    // Add background accounts to the active set if they exist
    if let Some(background_accounts) = BACKGROUND_ACCOUNTS.lock().unwrap().as_ref() {
        for account in background_accounts.iter() {
            active_accounts.insert(*account as u64);
        }
    }
    
    (foreground_account, active_accounts)
}

/// Internal utility for updating account information asynchronously.
struct AccountUpdater {
    /// Flag indicating if any account information has changed during the update.
    change_flag: bool,
    /// Transmitter for sending events to the task manager.
    task_manager: TaskManagerTx,
}

impl AccountUpdater {
    /// Creates a new AccountUpdater instance.
    /// 
    /// # Arguments
    /// 
    /// * `task_manager` - Transmitter for sending account change events.
    fn new(task_manager: TaskManagerTx) -> Self {
        Self {
            change_flag: false,
            task_manager,
        }
    }

    /// Performs the asynchronous account information update.
    /// 
    /// This method retrieves the current foreground and background accounts
    /// and updates the global state if changes are detected.
    #[cfg_attr(not(feature = "oh"), allow(unused))]
    async fn update(mut self) {
        info!("AccountUpdate Start");
        // Store previous account state for comparison
        let old_foreground = FOREGROUND_ACCOUNT.load(Ordering::SeqCst);
        let old_background = BACKGROUND_ACCOUNTS.lock().unwrap().clone();

        // Update foreground account if changed
        #[cfg(feature = "oh")]
        if let Some(foreground_account) = get_foreground_account().await {
            if old_foreground != foreground_account {
                self.change_flag = true;
                FOREGROUND_ACCOUNT.store(foreground_account, Ordering::SeqCst);
            }
        }

        // Update background accounts if changed
        #[cfg(feature = "oh")]
        if let Some(background_accounts) = get_background_accounts().await {
            if !old_background.is_some_and(|old_background| old_background == background_accounts) {
                self.change_flag = true;
                *BACKGROUND_ACCOUNTS.lock().unwrap() = Some(background_accounts);
            }
        }
        
        // The change notification is handled in the Drop implementation
    }
}

impl Drop for AccountUpdater {
    /// Cleans up after an account update operation.
    /// 
    /// This implementation:
    /// 1. Resets the global update flag to allow new updates
    /// 2. Sends a change notification to the task manager if any account state changed
    fn drop(&mut self) {
        info!("AccountUpdate Finished");
        // Reset the update flag to allow new update operations
        UPDATE_FLAG.store(false, Ordering::SeqCst);
        
        // Notify task manager only if actual changes occurred
        if self.change_flag {
            info!("AccountInfo changed, notify task manager");
            self.task_manager
                .send_event(TaskManagerEvent::Account(AccountEvent::Changed));
        }
    }
}

#[cfg(feature = "oh")]
/// Retrieves the currently active foreground OS account.
/// 
/// # Returns
/// 
/// `Some(account_id)` if the foreground account was successfully retrieved,
/// `None` if the operation failed after multiple retries.
/// 
/// # Notes
/// 
/// This function attempts to retrieve the foreground account up to 10 times
/// with a 500ms delay between retries. It logs errors and reports system events
/// when retrieval fails.
async fn get_foreground_account() -> Option<i32> {
    let mut foreground_account = 0;
    // Retry up to 10 times with 500ms delay
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
/// Retrieves all currently active background OS accounts.
/// 
/// # Returns
/// 
/// `Some(Vec<account_ids>)` if background accounts were successfully retrieved,
/// `None` if the operation failed after multiple retries.
/// 
/// # Notes
/// 
/// This function attempts to retrieve background accounts up to 10 times
/// with a 500ms delay between retries. It logs errors and reports system events
/// when retrieval fails.
async fn get_background_accounts() -> Option<Vec<i32>> {
    // Retry up to 10 times with 500ms delay
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
/// Registers subscriptions for OS account state changes.
/// 
/// # Arguments
/// 
/// * `task_manager` - Transmitter for sending account events to the task manager.
/// 
/// # Notes
/// 
/// This function:
/// 1. Stores the task manager transmitter for future use
/// 2. Subscribes to various account events (switched, activated, removed, stopped)
/// 3. Sets up appropriate handlers for each event type
/// 4. Performs an initial account update
/// 
/// The function will retry subscription operations indefinitely with a 500ms delay
/// between attempts until successful.
pub(crate) fn registry_account_subscribe(task_manager: TaskManagerTx) {
    static ONCE: Once = Once::new();

    // Store task manager reference once during initialization
    call_once(&ONCE, || unsafe {
        TASK_MANAGER_TX = Some(task_manager.clone());
    });

    info!("registry_account_subscribe");

    // Subscribe to account switched events
    loop {
        let ret = RegistryAccountSubscriber(
            OS_ACCOUNT_SUBSCRIBE_TYPE::SWITCHED,
            Box::new(task_manager.clone()),
            |_, _| {}, // No action needed for switched notification
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

    // Subscribe to account activated events
    loop {
        let ret = RegistryAccountSubscriber(
            OS_ACCOUNT_SUBSCRIBE_TYPE::ACTIVATED,
            Box::new(task_manager.clone()),
            |_id, task_manager| update_accounts(task_manager.clone()),
            |_, _, _| {}, // No action needed for activation switch callback
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

    // Subscribe to account removed events
    loop {
        let ret = RegistryAccountSubscriber(
            OS_ACCOUNT_SUBSCRIBE_TYPE::REMOVED,
            Box::new(task_manager.clone()),
            |id, task_manager| {
                // Send specific remove event with account ID
                task_manager.send_event(TaskManagerEvent::Account(AccountEvent::Remove(*id)));
            },
            |_, _, _| {}, // No action needed for remove switch callback
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

    // Subscribe to account stopped events
    loop {
        let ret = RegistryAccountSubscriber(
            OS_ACCOUNT_SUBSCRIBE_TYPE::STOPPED,
            Box::new(task_manager.clone()),
            |_id, task_manager| update_accounts(task_manager.clone()),
            |_, _, _| {}, // No action needed for stopped switch callback
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

    // Perform initial account state update
    update_accounts(task_manager.clone());
}

impl RequestDb {
    /// Deletes all tasks associated with a specific user account from the database.
    /// 
    /// # Arguments
    /// 
    /// * `user_id` - The identifier of the user account whose tasks should be deleted.
    /// 
    /// # Notes
    /// 
    /// This method calculates the actual user ID by dividing the UID by 200000,
    /// which appears to be a system-specific way of extracting the base user ID.
    /// Errors during execution are logged and reported as system events.
    pub(crate) fn delete_all_account_tasks(&self, user_id: i32) {
        // Calculate the actual user ID component from the full UID
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

// Foreign function interface for interacting with OS account services
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

// Test module for account management functionality
#[cfg(feature = "oh")]
#[cfg(test)]
mod ut_account {
    include!("../../tests/ut/manage/ut_account.rs");
}
