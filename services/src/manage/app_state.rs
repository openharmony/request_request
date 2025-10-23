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

//! Application state monitoring and event handling.
//! 
//! This module provides functionality to listen for application lifecycle events,
//! including app state changes (foreground/background), process termination, and
//! app uninstallation. It ensures proper cleanup and resource management when
//! applications change state or are removed from the system.

use std::mem::MaybeUninit;
use std::sync::Once;

use super::task_manager::TaskManagerTx;
use crate::manage::events::{StateEvent, TaskManagerEvent};
use crate::service::client::ClientManagerEntry;
use crate::utils::c_wrapper::CStringWrapper;
use crate::utils::{call_once, CommonEventSubscriber, CommonEventWant};

/// Listens for application state changes and process termination events.
/// 
/// This struct maintains references to the client manager and task manager to
/// properly handle application state transitions and process lifecycle events.
pub(crate) struct AppStateListener {
    /// Client manager for handling client-related operations.
    client_manager: ClientManagerEntry,
    /// Task manager transmitter for sending app state notifications.
    task_manager: TaskManagerTx,
}

/// Global instance of the application state listener.
/// 
/// # Safety
/// 
/// This static variable is initialized once via `AppStateListener::init()` and should only
/// be accessed after initialization is complete. Access before initialization is unsafe.
static mut APP_STATE_LISTENER: MaybeUninit<AppStateListener> = MaybeUninit::uninit();

/// Ensures the app state listener is initialized exactly once.
static ONCE: Once = Once::new();

impl AppStateListener {
    /// Initializes the application state listener with managers.
    /// 
    /// # Arguments
    /// 
    /// * `client_manager` - Manager for handling client processes.
    /// * `task_manager` - Transmitter for sending state events to the task manager.
    /// 
    /// # Safety
    /// 
    /// This function uses unsafe operations to initialize a global static variable.
    pub(crate) fn init(client_manager: ClientManagerEntry, task_manager: TaskManagerTx) {
        unsafe {
            // Initialize the global instance exactly once
            call_once(&ONCE, || {
                APP_STATE_LISTENER.write(AppStateListener {
                    client_manager,
                    task_manager,
                });
            });
            // Register callbacks for application state and process death notifications
            RegisterAPPStateCallback(app_state_change_callback);
            RegisterProcessDiedCallback(process_died_callback);
        }
    }

    /// Registers the application state callbacks with the system.
    /// 
    /// # Notes
    /// 
    /// This function will fail with an error log if called before `init()` has completed.
    pub(crate) fn register() {
        // Only register callbacks if initialization is complete
        if ONCE.is_completed() {
            unsafe {
                RegisterAPPStateCallback(app_state_change_callback);
                RegisterProcessDiedCallback(process_died_callback);
            }
        } else {
            error!("ONCE not completed");
        }
    }
}

/// C callback invoked when an application's state changes.
/// 
/// # Arguments
/// 
/// * `uid` - User ID of the application whose state changed.
/// * `state` - New state code of the application (2 = foreground, 4 = background).
/// * `_pid` - Process ID of the application (unused).
/// 
/// # Safety
/// 
/// This function assumes `APP_STATE_LISTENER` has been properly initialized.
extern "C" fn app_state_change_callback(uid: i32, state: i32, _pid: i32) {
    // State 2 corresponds to application entering foreground
    if state == 2 {
        unsafe {
            APP_STATE_LISTENER
                .assume_init_ref()
                .task_manager
                .notify_foreground_app_change(uid as u64)
        };
    } else if state == 4 {
        unsafe {
            APP_STATE_LISTENER
                .assume_init_ref()
                .task_manager
                .notify_app_background(uid as u64)
        };
    }
}

/// C callback invoked when a process changes state or dies.
/// 
/// # Arguments
/// 
/// * `uid` - User ID of the process.
/// * `state` - New state code of the process (5 = process died).
/// * `pid` - Process ID of the process.
/// * `bundle_name` - Bundle name of the application associated with the process.
/// 
/// # Safety
/// 
/// This function assumes `APP_STATE_LISTENER` has been properly initialized.
extern "C" fn process_died_callback(uid: i32, state: i32, pid: i32, bundle_name: CStringWrapper) {
    debug!(
        "Receives process change, uid {} pid {} state {}",
        uid, pid, state
    );
    let name = bundle_name.to_string();
    
    // Special handling for hiviewx system process termination
    if name.starts_with("com.") && name.ends_with(".hmos.hiviewx") {
        unsafe {
            APP_STATE_LISTENER
                .assume_init_ref()
                .task_manager
                .notify_special_process_terminate(uid as u64);
        }
        info!("hiviewx terminate. {:?}, {:?}", uid, pid);
    }

    // State 5 corresponds to process termination
    if state == 5 {
        info!("Receives process died, uid {} pid {}", uid, pid);
        unsafe {
            APP_STATE_LISTENER
                .assume_init_ref()
                .client_manager
                .notify_process_terminate(pid as u64)
        };
    }
}

/// Subscriber for application uninstallation events.
/// 
/// This struct listens for app uninstall events and notifies the task manager
/// to clean up any resources associated with the uninstalled application.
pub(crate) struct AppUninstallSubscriber {
    /// Task manager transmitter for sending app uninstallation events.
    task_manager: TaskManagerTx,
}

impl AppUninstallSubscriber {
    /// Creates a new application uninstall subscriber.
    /// 
    /// # Arguments
    /// 
    /// * `task_manager` - Transmitter for sending uninstallation events to the task manager.
    pub(crate) fn new(task_manager: TaskManagerTx) -> Self {
        Self { task_manager }
    }
}

impl CommonEventSubscriber for AppUninstallSubscriber {
    /// Handles received application uninstall events.
    /// 
    /// # Arguments
    /// 
    /// * `_code` - Event code (unused).
    /// * `_data` - Event data (unused).
    /// * `want` - Event data structure containing the UID of the uninstalled app.
    fn on_receive_event(&self, _code: i32, _data: String, want: CommonEventWant) {
        // Extract the UID from the event data if available
        if let Some(uid) = want.get_int_param("uid") {
            info!("Receive app uninstall event, uid: {}", uid);
            // Notify task manager about the app uninstallation
            self.task_manager
                .send_event(TaskManagerEvent::State(StateEvent::AppUninstall(
                    uid as u64,
                )));
        }
    }
}

// C function declarations for registering callbacks with the OS
#[cfg(feature = "oh")]
extern "C" {
    // Registers a callback for application state changes
    fn RegisterAPPStateCallback(f: extern "C" fn(i32, i32, i32));
    // Registers a callback for process termination events
    fn RegisterProcessDiedCallback(f: extern "C" fn(i32, i32, i32, CStringWrapper));
}
