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

//! IPC service stub implementation for request operations.
//! 
//! This module provides the service stub implementation that handles remote IPC requests
//! for download and upload operations, task management, and status monitoring.

use std::fs::File;
use std::sync::Mutex;

use ipc::parcel::MsgParcel;
use ipc::remote::RemoteStub;
use ipc::{IpcResult, IpcStatusCode};
use system_ability_fwk::ability::Handler;

use super::client::ClientManagerEntry;
use super::interface;
use super::permission::PermissionChecker;
use super::run_count::RunCountManagerEntry;
use crate::manage::database::RequestDb;
use crate::manage::task_manager::TaskManagerTx;
use crate::service::active_counter::ActiveCounter;
use crate::task::config::TaskConfig;
use crate::task::info::TaskInfo;

/// Service stub implementation for handling remote IPC requests for request operations.
///
/// This struct manages the service-side implementation of the request system's IPC interface,
/// handling incoming client requests for task management, status tracking, and notification subscriptions.
pub(crate) struct RequestServiceStub {
    /// Mutex-protected task manager for handling download/upload operations.
    pub(crate) task_manager: Mutex<TaskManagerTx>,
    /// System ability handler for managing service lifecycle.
    pub(crate) sa_handler: Handler,
    /// Manager for tracking and interacting with client connections.
    pub(crate) client_manager: ClientManagerEntry,
    /// Manager for tracking and notifying about running task counts.
    pub(crate) run_count_manager: RunCountManagerEntry,
    /// Counter for tracking active operations to prevent premature service termination.
    pub(crate) active_counter: ActiveCounter,
}

impl RequestServiceStub {
    /// Creates a new `RequestServiceStub` instance.
    ///
    /// # Arguments
    ///
    /// * `sa_handler` - System ability handler for managing service lifecycle
    /// * `task_manager` - Task manager transaction handle for task operations
    /// * `client_manager` - Client manager for tracking client connections
    /// * `run_count_manager` - Run count manager for task count notifications
    /// * `active_counter` - Counter for tracking active operations
    ///
    /// # Returns
    ///
    /// A new instance of `RequestServiceStub` with the provided components.
    pub(crate) fn new(
        sa_handler: Handler,
        task_manager: TaskManagerTx,
        client_manager: ClientManagerEntry,
        run_count_manager: RunCountManagerEntry,
        active_counter: ActiveCounter,
    ) -> Self {
        Self {
            task_manager: Mutex::new(task_manager),
            sa_handler,
            client_manager,
            run_count_manager,
            active_counter,
        }
    }

    /// Checks if the specified task belongs to the given user ID.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The ID of the task to check
    /// * `uid` - The user ID to verify against the task owner
    ///
    /// # Returns
    ///
    /// `true` if the task belongs to the specified user ID, `false` otherwise.
    pub(crate) fn check_task_uid(&self, task_id: u32, uid: u64) -> bool {
        let db = RequestDb::get_instance();
        db.query_task_uid(task_id) == Some(uid)
    }

    #[allow(dead_code)]
    /// Checks if the caller has either manager permission or owns the specified task.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The ID of the task to check
    /// * `uid` - The user ID to verify
    ///
    /// # Returns
    ///
    /// `true` if the caller has manager permission or owns the task, `false` otherwise.
    ///
    /// # Notes
    ///
    /// TODO: permission should match action.
    pub(crate) fn check_permission_or_uid(&self, task_id: u32, uid: u64) -> bool {
        let permission = PermissionChecker::check_manager();
        match permission.get_action() {
            Some(_a) => true,  // Manager permission granted
            None => self.check_task_uid(task_id, uid), // Fall back to task ownership check
        }
    }
}

impl RemoteStub for RequestServiceStub {
    /// Handles incoming remote IPC requests.
    ///
    /// This method processes all incoming IPC requests by:
    /// 1. Preventing service idle timeout
    /// 2. Incrementing active operation counter
    /// 3. Verifying interface token
    /// 4. Routing request to appropriate handler based on operation code
    /// 5. Decrementing active counter after processing
    ///
    /// # Arguments
    ///
    /// * `code` - Operation code identifying the requested action
    /// * `data` - Message parcel containing request data
    /// * `reply` - Message parcel to write response data to
    ///
    /// # Returns
    ///
    /// `0` on success, or an error code on failure.
    fn on_remote_request(&self, code: u32, data: &mut MsgParcel, reply: &mut MsgParcel) -> i32 {
        // Prevent service from going idle during request processing
        self.sa_handler.cancel_idle();
        // Track active operation to prevent premature service termination
        self.active_counter.increment();
        
        const SERVICE_TOKEN: &str = "OHOS.Download.RequestServiceInterface";
        debug!("Processes on_remote_request, code: {}", code);
        
        // Verify interface token to ensure client is communicating with correct service
        match data.read_interface_token() {
            Ok(token) if token == SERVICE_TOKEN => {}
            _ => {
                error!("Gets invalid token");
                sys_event!(ExecError, DfxCode::INVALID_IPC_MESSAGE_A00, "Gets invalid token");
                self.active_counter.decrement();
                return IpcStatusCode::Failed as i32;
            }
        };
        
        // Route request to appropriate handler based on operation code
        let res = match code {
            interface::CONSTRUCT => self.construct(data, reply),
            interface::PAUSE => self.pause(data, reply),
            interface::QUERY => self.query(data, reply),
            interface::QUERY_MIME_TYPE => self.query_mime_type(data, reply),
            interface::REMOVE => self.remove(data, reply),
            interface::RESUME => self.resume(data, reply),
            interface::START => self.start(data, reply),
            interface::STOP => self.stop(data, reply),
            interface::SHOW => self.show(data, reply),
            interface::TOUCH => self.touch(data, reply),
            interface::SEARCH => self.search(data, reply),
            interface::GET_TASK => self.get_task(data, reply),
            interface::CLEAR => Ok(()),
            interface::OPEN_CHANNEL => self.open_channel(reply),
            interface::SUBSCRIBE => self.subscribe(data, reply),
            interface::UNSUBSCRIBE => self.unsubscribe(data, reply),
            interface::SUB_RUN_COUNT => self.subscribe_run_count(data, reply),
            interface::UNSUB_RUN_COUNT => self.unsubscribe_run_count(reply),
            interface::CREATE_GROUP => self.create_group(data, reply),
            interface::ATTACH_GROUP => self.attach_group(data, reply),
            interface::DELETE_GROUP => self.delete_group(data, reply),
            interface::SET_MAX_SPEED => self.set_max_speed(data, reply),
            interface::SET_MODE => self.set_mode(data, reply),
            interface::DISABLE_TASK_NOTIFICATION => self.disable_task_notifications(data, reply),
            _ => Err(IpcStatusCode::Failed),
        };

        // Decrement active counter after request processing is complete
        self.active_counter.decrement();
        
        // Convert result to IPC status code
        match res {
            Ok(_) => 0,
            Err(e) => e as i32,
        }
    }

    /// Dumps service state information to a file.
    ///
    /// # Arguments
    ///
    /// * `file` - File to write dump information to
    /// * `args` - Command-line arguments for configuring dump behavior
    ///
    /// # Returns
    ///
    /// `0` on success, or an error code on failure.
    fn dump(&self, file: File, args: Vec<String>) -> i32 {
        match self.dump(file, args) {
            Ok(()) => 0,
            Err(e) => e as i32,
        }
    }
}

/// Serializes task information into a message parcel for IPC transmission.
///
/// This function converts a `TaskInfo` struct into a format suitable for IPC transmission
/// by writing each field sequentially to the provided message parcel.
///
/// # Arguments
///
/// * `tf` - The task information to serialize
/// * `reply` - The message parcel to write the serialized data to
///
/// # Returns
///
/// `Ok(())` on successful serialization, or an `IpcResult` error if any field fails to write.
pub(crate) fn serialize_task_info(tf: TaskInfo, reply: &mut MsgParcel) -> IpcResult<()> {
    // Serialize common data fields
    reply.write(&(tf.common_data.gauge))?;
    reply.write(&(tf.common_data.retry))?;
    reply.write(&(tf.common_data.action as u32))?;
    reply.write(&(tf.common_data.mode as u32))?;
    reply.write(&(tf.common_data.reason as u32))?;
    reply.write(&(tf.common_data.tries))?;
    reply.write(&(tf.common_data.uid.to_string()))?;
    reply.write(&(tf.bundle))?;
    reply.write(&(tf.url))?;
    reply.write(&(tf.common_data.task_id.to_string()))?;
    reply.write(&tf.title)?;
    reply.write(&tf.mime_type)?;
    reply.write(&(tf.common_data.ctime))?;
    reply.write(&(tf.common_data.mtime))?;
    reply.write(&(tf.data))?;
    reply.write(&(tf.description))?;
    reply.write(&(tf.common_data.priority))?;

    // Serialize form items array with length prefix
    reply.write(&(tf.form_items.len() as u32))?;
    for i in 0..tf.form_items.len() {
        reply.write(&(tf.form_items[i].name))?;
        reply.write(&(tf.form_items[i].value))?;
    }

    // Serialize file specifications array with length prefix
    reply.write(&(tf.file_specs.len() as u32))?;
    for i in 0..tf.file_specs.len() {
        reply.write(&(tf.file_specs[i].name))?;
        reply.write(&(tf.file_specs[i].path))?;
        reply.write(&(tf.file_specs[i].file_name))?;
        reply.write(&(tf.file_specs[i].mime_type))?;
    }

    // Serialize progress information
    reply.write(&(tf.progress.common_data.state as u32))?;
    let index = tf.progress.common_data.index;
    reply.write(&(index as u32))?;
    reply.write(&(tf.progress.processed[index] as u64))?;
    reply.write(&(tf.progress.common_data.total_processed as u64))?;
    reply.write(&(tf.progress.sizes))?;

    // Serialize progress extras map with length prefix
    reply.write(&(tf.progress.extras.len() as u32))?;
    for (k, v) in tf.progress.extras.iter() {
        reply.write(k)?;
        reply.write(v)?;
    }

    // Serialize task extras map with length prefix
    reply.write(&(tf.extras.len() as u32))?;
    for (k, v) in tf.extras.iter() {
        reply.write(k)?;
        reply.write(v)?;
    }
    
    // Serialize version and file status information
    reply.write(&(tf.common_data.version as u32))?;
    let each_file_status = tf.build_each_file_status();
    reply.write(&(each_file_status.len() as u32))?;
    for item in each_file_status.iter() {
        reply.write(&(item.path))?;
        reply.write(&(item.reason.repr as u32))?;
        reply.write(&(item.message))?;
    }
    Ok(())
}

/// Serializes task configuration into a message parcel for IPC transmission.
///
/// This function converts a `TaskConfig` struct into a format suitable for IPC transmission
/// by writing each field sequentially to the provided message parcel.
///
/// # Arguments
///
/// * `config` - The task configuration to serialize
/// * `reply` - The message parcel to write the serialized data to
///
/// # Returns
///
/// `Ok(())` on successful serialization, or an `IpcResult` error if any field fails to write.
pub(crate) fn serialize_task_config(config: TaskConfig, reply: &mut MsgParcel) -> IpcResult<()> {
    // Serialize common configuration data
    reply.write(&(config.common_data.action.repr as u32))?;
    reply.write(&(config.common_data.mode.repr as u32))?;
    reply.write(&(config.bundle_type))?;
    reply.write(&(config.common_data.cover))?;
    reply.write(&(config.common_data.network_config as u32))?;
    reply.write(&(config.common_data.metered))?;
    reply.write(&(config.common_data.roaming))?;
    reply.write(&(config.common_data.retry))?;
    reply.write(&(config.common_data.redirect))?;
    reply.write(&(config.common_data.index))?;
    reply.write(&(config.common_data.begins))?;
    reply.write(&(config.common_data.ends))?;
    reply.write(&(config.common_data.gauge))?;
    reply.write(&(config.common_data.precise))?;
    reply.write(&(config.common_data.priority))?;
    reply.write(&(config.common_data.background))?;
    reply.write(&(config.common_data.multipart))?;
    
    // Serialize task identification and metadata
    reply.write(&(config.bundle))?;
    reply.write(&(config.url))?;
    reply.write(&(config.title))?;
    reply.write(&(config.description))?;
    reply.write(&(config.method))?;
    
    // Serialize HTTP headers map with length prefix
    reply.write(&(config.headers.len() as u32))?;
    for (k, v) in config.headers.iter() {
        reply.write(k)?;
        reply.write(v)?;
    }
    
    // Serialize body data and authentication token
    reply.write(&(config.data))?;
    reply.write(&(config.token))?;
    
    // Serialize extras map with length prefix
    reply.write(&(config.extras.len() as u32))?;
    for (k, v) in config.extras.iter() {
        reply.write(k)?;
        reply.write(v)?;
    }
    
    // Serialize version information
    reply.write(&(config.version as u32))?;
    
    // Serialize form items array with length prefix
    reply.write(&(config.form_items.len() as u32))?;
    for i in 0..config.form_items.len() {
        reply.write(&(config.form_items[i].name))?;
        reply.write(&(config.form_items[i].value))?;
    }
    
    // Serialize file specifications array with length prefix
    reply.write(&(config.file_specs.len() as u32))?;
    for i in 0..config.file_specs.len() {
        reply.write(&(config.file_specs[i].name))?;
        reply.write(&(config.file_specs[i].path))?;
        reply.write(&(config.file_specs[i].file_name))?;
        reply.write(&(config.file_specs[i].mime_type))?;
    }
    
    // Serialize body file paths array with length prefix
    reply.write(&(config.body_file_paths.len() as u32))?;
    for i in 0..config.body_file_paths.len() {
        reply.write(&(config.body_file_paths[i]))?;
    }
    
    // Serialize minimum speed requirements
    reply.write(&(config.common_data.min_speed.speed))?;
    reply.write(&(config.common_data.min_speed.duration))?;
    Ok(())
}
