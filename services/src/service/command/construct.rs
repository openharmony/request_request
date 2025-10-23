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

//! Construction command handling for the request service.
//! 
//! This module implements the task construction logic for the request service, including
//! permission checking, task creation, notification configuration, and client subscription.

use ipc::parcel::MsgParcel;
use ipc::{IpcResult, IpcStatusCode};

use crate::config::Mode;
use crate::error::ErrorCode;
use crate::manage::events::TaskManagerEvent;
use crate::service::command::{set_code_with_index_other, CONSTRUCT_MAX};
use crate::service::notification_bar::{NotificationConfig, NotificationDispatcher};
use crate::service::permission::PermissionChecker;
use crate::service::RequestServiceStub;
use crate::task::config::TaskConfig;
use crate::utils::{check_permission, is_system_api};

impl RequestServiceStub {
    /// Constructs new request tasks from client parameters.
    ///
    /// # Arguments
    ///
    /// * `data` - Input parcel containing task configurations and notification settings.
    /// * `reply` - Output parcel to write results back to the client.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the method completes successfully.
    /// * `Err(IpcStatusCode::Failed)` - If permission checks fail or other critical errors occur.
    ///
    /// # Notes
    ///
    /// This method handles multiple task constructions in a batch operation, processing each
    /// task configuration sequentially. It performs permission validation, creates tasks
    /// through the task manager, configures notifications, and subscribes clients to task events.
    pub(crate) fn construct(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        debug!("Service construct");
        // Check required permissions before processing any tasks
        let download_permission = PermissionChecker::check_down_permission();
        if !PermissionChecker::check_internet() && !download_permission {
            error!("Service start: no INTERNET permission.");
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A01,
                "Service start: no INTERNET permission."
            );
            reply.write(&(ErrorCode::Permission as i32))?;
            return Err(IpcStatusCode::Failed);
        }
        // Read the number of tasks to construct
        let len: u32 = data.read()?;
        // Convert to usize for array indexing and comparison
        let len = len as usize;

        // Validate the number of tasks against the maximum allowed
        if len > CONSTRUCT_MAX {
            info!("Service construct: out of size: {}", len);
            reply.write(&(ErrorCode::Other as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        // Get caller information for permission checks and task association
        let uid = ipc::Skeleton::calling_uid();
        let token_id = ipc::Skeleton::calling_full_token_id();
        let pid = ipc::Skeleton::calling_pid();
        // Initialize results vector with default error values
        let mut vec = vec![(ErrorCode::Other, 0u32); len];

        // Check if this is a system API call and if notification permissions exist
        let is_system_api = is_system_api();
        let notification_permission = 
            check_permission("ohos.permission.REQUEST_DISABLE_NOTIFICATION");

        for i in 0..len {
            // Read both configurations before processing to ensure complete data retrieval
            let task_config = data.read::<TaskConfig>();
            let notification_config = data.read::<NotificationConfig>();

            // Validate task configuration
            let task_config = match task_config {
                Ok(config) => config,
                Err(e) => {
                    // Set error code for this task and continue to next task
                    set_code_with_index_other(&mut vec, i, ErrorCode::ParameterCheck);
                    error!("task_config read err, {}, {}", i, e);
                    continue;
                }
            };

            // Validate notification configuration
            let mut notification_config = match notification_config {
                Ok(config) => config,
                Err(e) => {
                    set_code_with_index_other(&mut vec, i, ErrorCode::ParameterCheck);
                    error!("notification_config read err, {}, {}", i, e);
                    continue;
                }
            };

            debug!("Service construct: task_config constructed");
            // Extract task mode for notification configuration
            let mode = task_config.common_data.mode;
            // Create construction event and response channel
            let (event, rx) = TaskManagerEvent::construct(task_config);
            // Send construction event to task manager
            if !self.task_manager.lock().unwrap().send_event(event) {
                set_code_with_index_other(&mut vec, i, ErrorCode::Other);
                continue;
            }
            // Wait for task creation result
            let ret = match rx.get() {
                Some(ret) => ret,
                None => {
                    error!("End Service construct, failed: receives ret failed");
                    sys_event!(
                        ExecError,
                        DfxCode::INVALID_IPC_MESSAGE_A02,
                        "End Service construct, failed: receives ret failed"
                    );
                    set_code_with_index_other(&mut vec, i, ErrorCode::Other);
                    continue;
                }
            };

            // Extract task ID or handle construction error
            let task_id = match ret {
                Ok(id) => id,
                Err(err_code) => {
                    error!("End Service construct, failed: {:?}", err_code);
                    sys_event!(
                        ExecError,
                        DfxCode::INVALID_IPC_MESSAGE_A02,
                        &format!("End Service construct, failed: {:?}", err_code)
                    );
                    set_code_with_index_other(&mut vec, i, err_code);
                    continue;
                }
            };

            // Associate notification config with the newly created task
            notification_config.task_id = task_id;
            // Update notification settings for this task
            NotificationDispatcher::get_instance()
                .update_task_customized_notification(&notification_config);

            // Handle notification disabling for system API calls
            if notification_config.disable && is_system_api {
                if !notification_permission {
                    error!("End Service construct, notify permission: {}", task_id);
                    if let Some((c, tid)) = vec.get_mut(i) {
                        *c = ErrorCode::Permission;
                        *tid = task_id;
                    }
                    continue;
                }
                // Only disable notifications for background tasks
                if matches!(mode, Mode::BackGround) {
                    NotificationDispatcher::get_instance().disable_task_notification(uid, task_id);
                }
            }

            debug!("Service construct: construct event sent to manager");

            // Subscribe the client to receive task notifications
            let ret = self.client_manager.subscribe(task_id, pid, uid, token_id);
            if ret != ErrorCode::ErrOk {
                error!("End Service subscribe, tid: {}, failed: {:?}", task_id, ret);
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A02,
                    &format!("End Service subscribe, tid: {}, failed: {:?}", task_id, ret)
                );
            }
            // Store the result for this task
            if let Some((c, tid)) = vec.get_mut(i) {
                *c = ret;
                *tid = task_id;
            }
            debug!("End Service construct, succeed with tid: {}", task_id);
        }
        // Write overall success code
        reply.write(&(ErrorCode::ErrOk as i32))?;
        // Write individual task results
        for (c, tid) in vec {
            reply.write(&(c as i32))?;
            reply.write(&tid)?;
        }
        Ok(())
    }
}
