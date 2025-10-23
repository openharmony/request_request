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

//! Notification management functionality for the request service.
//! 
//! This module implements methods for creating and managing notification groups
//! for download/upload tasks, as well as controlling task notifications visibility.
//! It integrates with the notification system to provide user feedback during
//! task execution.

use ipc::parcel::MsgParcel;
use ipc::{IpcResult, IpcStatusCode};

use crate::config::Action;
use crate::error::ErrorCode;
use crate::manage::database::RequestDb;
use crate::manage::events::TaskManagerEvent;
use crate::service::notification_bar::NotificationDispatcher;
use crate::service::permission::{ManagerPermission, PermissionChecker};
use crate::service::RequestServiceStub;
use crate::utils::{check_permission, is_system_api};

impl RequestServiceStub {
    /// Creates a new notification group for tasks.
    ///
    /// Creates a notification group that can be used to organize multiple task notifications.
    /// Groups related tasks together in the notification bar for better user experience.
    ///
    /// # Arguments
    ///
    /// * `data` - Input parcel containing group configuration (gauge visibility, title,
    ///   text, intent agent, disable state, and visibility level).
    /// * `reply` - Output parcel to write the newly created group ID.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the group was successfully created.
    /// * `Err(_)` - If reading from or writing to the parcels fails.
    ///
    /// # Notes
    ///
    /// The disable parameter is only respected if the calling process is a system API
    /// and has the `ohos.permission.REQUEST_DISABLE_NOTIFICATION` permission.
    pub(crate) fn create_group(
        &self,
        data: &mut MsgParcel,
        reply: &mut MsgParcel,
    ) -> IpcResult<()> {
        // Read group configuration from input parcel
        let gauge: bool = data.read()?;

        // Read optional title with presence flag
        let title = if data.read::<bool>()? {
            Some(data.read()?)
        } else {
            None
        };

        // Read optional text with presence flag
        let text = if data.read::<bool>()? {
            Some(data.read()?)
        } else {
            None
        };

        // Read optional intent agent with presence flag
        let want_agent = if data.read::<bool>()? {
            Some(data.read()?)
        } else {
            None
        };

        // Read and validate disable flag with permission check
        let mut disable: bool = data.read()?;
        // Only system APIs with special permission can disable notifications
        if disable && (!is_system_api() || !check_permission("ohos.permission.REQUEST_DISABLE_NOTIFICATION")) {
            disable = false;
        }

        let visibility = data.read()?;

        let new_group_id = NotificationDispatcher::get_instance().create_group(
            gauge, title, text, want_agent, disable, visibility);
        reply.write(&new_group_id.to_string())?;
        Ok(())
    }

    /// Attaches tasks to a notification group.
    ///
    /// Associates multiple tasks with a notification group so their notifications
    /// appear organized together in the notification bar.
    ///
    /// # Arguments
    ///
    /// * `data` - Input parcel containing the group ID and list of task IDs to attach.
    /// * `reply` - Output parcel to write the operation result code.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the operation completed successfully (regardless of result code).
    /// * `Err(IpcStatusCode::Failed)` - If sending the event to the task manager failed.
    ///
    /// # Errors
    ///
    /// Returns an error code in the reply parcel if:
    /// * The group ID is invalid (`ErrorCode::GroupNotFound`).
    /// * Any task ID is invalid (`ErrorCode::TaskNotFound`).
    /// * The calling UID does not have permission to access a task (`ErrorCode::TaskNotFound`).
    pub(crate) fn attach_group(
        &self,
        data: &mut MsgParcel,
        reply: &mut MsgParcel,
    ) -> IpcResult<()> {
        // Read and validate group ID
        let Ok(group_id) = data.read::<String>()?.parse::<u32>() else {
            error!("End Service attach_group, group_id, failed: group_id not valid",);
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A38,
                "End Service attach_group, group_id, failed: group_id not valid"
            );
            reply.write(&(ErrorCode::GroupNotFound as i32))?;
            return Ok(());
        };
        
        // Read list of task IDs to attach to the group
        let task_ids = data.read::<Vec<String>>()?;

        // Get calling process UID for permission checks
        let uid = ipc::Skeleton::calling_uid();

        // Prepare vector for validated task IDs
        let mut parse_ids = Vec::with_capacity(task_ids.len());

        for task_id in task_ids.iter() {
            let Ok(task_id) = task_id.parse::<u32>() else {
                error!("End Service attach_group, task_id, failed: task_id not valid");
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A38,
                    "End Service attach_group, task_id, failed: task_id not valid"
                );
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                return Ok(());
            };
            if !self.check_task_uid(task_id, uid) {
                error!(
                    "End Service attach_group, task_id: {}, failed: task_id not belong to uid",
                    task_id
                );
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A38,
                    &format!("End Service attach_group, task_id: {}, failed: task_id not belong to uid", task_id)
                );
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                return Ok(());
            }
            parse_ids.push(task_id);
        }
        // Create and send attach group event to task manager
        let (event, rx) = TaskManagerEvent::attach_group(uid, parse_ids, group_id);
        if !self.task_manager.lock().unwrap().send_event(event) {
            return Err(IpcStatusCode::Failed);
        }

        let ret = match rx.get() {
            Some(ret) => ret,
            None => {
                error!(
                    "End Service attach_group, task_id: {:?}, group_id: {}, failed: receives ret failed",
                    task_ids, group_id
                );
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A38, 
                    &format!("End Service attach_group, task_id: {:?}, group_id: {}, failed: receives ret failed",task_ids, group_id)
                );
                ErrorCode::Other
            }
        };
        if ret != ErrorCode::ErrOk {
            error!(
                "End Service attach_group, task_id: {:?}, group_id: {}, failed: ret is not ErrOk",
                task_ids, group_id
            );
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A38,
                &format!("End Service attach_group, task_id: {:?}, group_id: {}, failed: ret is not ErrOk",task_ids, group_id)
            );
        }
        reply.write(&(ret as i32))?;
        Ok(())
    }

    /// Deletes a notification group.
    ///
    /// Removes a notification group and dissociates any tasks from it. Only groups
    /// created by the calling UID can be deleted.
    ///
    /// # Arguments
    ///
    /// * `data` - Input parcel containing the group ID to delete.
    /// * `reply` - Output parcel to write the operation result code.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Always returns `Ok` regardless of operation success.
    ///   Check the result code in the reply parcel for actual status.
    ///
    /// # Notes
    ///
    /// Returns `ErrorCode::GroupNotFound` if the group ID is invalid or
    /// does not belong to the calling UID.
    pub(crate) fn delete_group(
        &self,
        data: &mut MsgParcel,
        reply: &mut MsgParcel,
    ) -> IpcResult<()> {
        let Ok(group_id) = data.read::<String>()?.parse::<u32>() else {
            reply.write(&(ErrorCode::GroupNotFound as i32))?;
            return Ok(());
        };
        let mut ret = ErrorCode::ErrOk;
        let uid = ipc::Skeleton::calling_uid();
        if !NotificationDispatcher::get_instance().delete_group(group_id, uid) {
            ret = ErrorCode::GroupNotFound;
        }
        reply.write(&(ret as i32))?;
        Ok(())
    }

    /// Disables notifications for multiple tasks.
    ///
    /// Disables user notifications for each task in the provided list, following
    /// permission checks for each task individually.
    ///
    /// # Arguments
    ///
    /// * `data` - Input parcel containing a list of task IDs to disable notifications for.
    /// * `reply` - Output parcel to write result codes for each task.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If all operations completed (results for each task are written to the reply).
    /// * `Err(_)` - If reading from or writing to the parcels fails.
    ///
    /// # Notes
    ///
    /// The reply parcel contains a result code for each task in the same order as
    /// the input task list.
    pub(crate) fn disable_task_notifications(
        &self,
        data: &mut MsgParcel,
        reply: &mut MsgParcel,
    ) -> IpcResult<()> {
        // Initialize permission cache and read task IDs
        let mut permission = None; // Cache permission check result to avoid redundant checks
        let task_ids = data.read::<Vec<String>>()?;
        let calling_uid = ipc::Skeleton::calling_uid();

        for task_id in task_ids.iter() {
            match self.disable_task_notification_inner(calling_uid, task_id, &mut permission) {
                Ok(()) => reply.write(&(ErrorCode::ErrOk as i32)),
                Err(e) => {
                    error!("End Service disable_task_notifications, failed: {:?}", e);
                    sys_event!(
                        ExecError,
                        DfxCode::INVALID_IPC_MESSAGE_A46,
                        &format!("End Service disable_task_notifications, failed: {:?}", e)
                    );
                    reply.write(&(e as i32))
                }
            }?;
        }
        Ok(())
    }

    /// Disables notifications for a single task with permission checking.
    ///
    /// Internal helper method that performs validation and permission checks
    /// before disabling notifications for a specific task.
    ///
    /// # Arguments
    ///
    /// * `calling_uid` - UID of the calling process.
    /// * `task_id` - String representation of the task ID.
    /// * `permission` - Mutable reference to a permission cache to avoid redundant checks.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If notifications were successfully disabled.
    /// * `Err(ErrorCode::TaskNotFound)` - If the task ID is invalid or the task doesn't exist.
    /// * `Err(ErrorCode::Permission)` - If the calling process lacks permission to modify
    ///   notifications for this task.
    fn disable_task_notification_inner(
        &self,
        calling_uid: u64,
        task_id: &str,
        permission: &mut Option<ManagerPermission>,
    ) -> Result<(), ErrorCode> {
        // Parse and validate task ID
        let Ok(task_id) = task_id.parse::<u32>() else {
            return Err(ErrorCode::TaskNotFound);
        };
        
        // Get the task owner's UID from database
        let Some(task_uid) = RequestDb::get_instance().query_task_uid(task_id) else {
            return Err(ErrorCode::TaskNotFound);
        };
        // Check if caller owns the task or has management permissions
        if task_uid != calling_uid {
            // Use cached permission or perform permission check
            let permission = match permission {
                Some(permission) => *permission,
                None => {
                    // Cache permission result to avoid repeated checks for multiple tasks
                    *permission = Some(PermissionChecker::check_manager());
                    permission.unwrap()
                }
            };
            match permission {
                ManagerPermission::ManagerAll => {}
                ManagerPermission::ManagerDownLoad => {
                    let Some(action) = RequestDb::get_instance().query_task_action(task_id) else {
                        return Err(ErrorCode::TaskNotFound);
                    };
                    if action != Action::Download {
                        return Err(ErrorCode::Permission);
                    }
                }
                ManagerPermission::ManagerUpload => {
                    let Some(action) = RequestDb::get_instance().query_task_action(task_id) else {
                        return Err(ErrorCode::TaskNotFound);
                    };
                    if action != Action::Upload {
                        return Err(ErrorCode::Permission);
                    }
                }
                ManagerPermission::NoPermission => {
                    return Err(ErrorCode::Permission);
                }
            }
        }
        NotificationDispatcher::get_instance().disable_task_notification(task_uid, task_id);
        Ok(())
    }
}
