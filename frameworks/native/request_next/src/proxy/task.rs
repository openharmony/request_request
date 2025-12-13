// Copyright (C) 2025 Huawei Device Co., Ltd.
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

//! Task management operations for the RequestProxy.
//! 
//! This module implements methods for managing download tasks through the RequestProxy,
//! including creating, starting, pausing, resuming, removing, stopping, and setting speed limits
//! for tasks.

// IPC and parcel dependencies
use ipc::parcel::MsgParcel;
// Download core dependencies
use request_core::config::TaskConfig;
use request_core::interface;

// Local dependencies
use super::{RequestProxy, SERVICE_TOKEN};
use crate::client::error::CreateTaskError;

impl RequestProxy {
    /// Creates a new download task with the provided configuration.
    ///
    /// Sends a request to the download service to create a new task based on the provided
    /// `TaskConfig`. Returns the unique task ID if successful.
    ///
    /// # Parameters
    /// - `config`: The task configuration containing download parameters
    ///
    /// # Returns
    /// - `Ok(i64)` with the task ID if the task was created successfully
    /// - `Err(CreateTaskError)` if an error occurred during task creation
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use request_core::config::TaskConfig;
    /// use request_next::proxy::RequestProxy;
    ///
    /// fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let proxy = RequestProxy::get_instance();
    ///     let config = TaskConfig::default();
    ///     
    ///     match proxy.create(&config) {
    ///         Ok(task_id) => println!("Task created with ID: {}", task_id),
    ///         Err(e) => eprintln!("Failed to create task: {:?}", e),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Panics
    /// - Panics if parcel operations fail due to IPC errors
    pub(crate) fn create(&self, config: &TaskConfig) -> Result<i64, CreateTaskError> {
        let remote = self.remote()?;
        let mut data = MsgParcel::new();
        // Write interface token to identify the service
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        // Write version information and task configuration
        data.write(&1u32).unwrap();
        data.write(config).unwrap();

        // Send request to construct the task
        let mut reply = remote
            .send_request(interface::CONSTRUCT, &mut data)
            .map_err(|_| 13400003)?;

        // Check first error code
        let code = reply.read::<i32>().unwrap();
        if code != 0 {
            return Err(CreateTaskError::Code(code));
        }

        // Check second error code
        let code = reply.read::<i32>().unwrap();
        if code != 0 {
            return Err(CreateTaskError::Code(code));
        }
        // Read and return the task ID
        let task_id = reply.read::<u32>().unwrap();

        Ok(task_id as i64)
    }

    /// Starts a download task identified by the given task ID.
    ///
    /// # Parameters
    /// - `task_id`: The unique identifier of the task to start
    ///
    /// # Returns
    /// - `Ok(())` if the task started successfully
    /// - `Err(i32)` with the error code if starting the task failed
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use request_next::proxy::RequestProxy;
    ///
    /// fn example(task_id: i64) {
    ///     let proxy = RequestProxy::get_instance();
    ///     
    ///     match proxy.start(task_id) {
    ///         Ok(_) => println!("Task {} started successfully", task_id),
    ///         Err(code) => eprintln!("Failed to start task {} with error code: {}", task_id, code),
    ///     }
    /// }
    /// ```
    ///
    /// # Panics
    /// - Panics if parcel operations fail due to IPC errors
    pub(crate) fn start(&self, task_id: i64) -> Result<(), i32> {
        let remote = self.remote()?;
        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        // Write task count and ID
        data.write(&1u32).unwrap();
        data.write(&task_id.to_string()).unwrap();

        // Send start request
        let mut reply = remote.send_request(interface::START, &mut data).map_err(|_| 13400003)?;
        let code = reply.read::<i32>().unwrap(); // error code
        if code == 0 {
            let code = reply.read::<i32>().unwrap(); // error code
            if code == 0 {
                Ok(())
            } else {
                Err(code)
            }
        } else {
            Err(code)
        }
    }

    /// Pauses a running download task.
    ///
    /// # Parameters
    /// - `task_id`: The unique identifier of the task to pause
    ///
    /// # Returns
    /// - `Ok(())` if the task was paused successfully
    /// - `Err(i32)` with the error code if pausing the task failed
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use request_next::proxy::RequestProxy;
    ///
    /// fn example(task_id: i64) {
    ///     let proxy = RequestProxy::get_instance();
    ///     
    ///     match proxy.pause(task_id) {
    ///         Ok(_) => println!("Task {} paused successfully", task_id),
    ///         Err(code) => eprintln!("Failed to pause task {} with error code: {}", task_id, code),
    ///     }
    /// }
    /// ```
    ///
    /// # Panics
    /// - Panics if parcel operations fail due to IPC errors
    pub(crate) fn pause(&self, task_id: i64) -> Result<(), i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&1u32).unwrap(); // version
        data.write(&1u32).unwrap(); // task count
        data.write(&task_id.to_string()).unwrap();

        // Send pause request
        let mut reply = remote.send_request(interface::PAUSE, &mut data).map_err(|_| 13400003)?;

        // Check first error code
        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }

        // Check second error code
        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }

        Ok(())
    }

    /// Resumes a paused download task.
    ///
    /// # Parameters
    /// - `task_id`: The unique identifier of the task to resume
    ///
    /// # Returns
    /// - `Ok(())` if the task was resumed successfully
    /// - `Err(i32)` with the error code if resuming the task failed
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use request_next::proxy::RequestProxy;
    ///
    /// fn example(task_id: i64) {
    ///     let proxy = RequestProxy::get_instance();
    ///     
    ///     match proxy.resume(task_id) {
    ///         Ok(_) => println!("Task {} resumed successfully", task_id),
    ///         Err(code) => eprintln!("Failed to resume task {} with error code: {}", task_id, code),
    ///     }
    /// }
    /// ```
    ///
    /// # Panics
    /// - Panics if parcel operations fail due to IPC errors
    pub(crate) fn resume(&self, task_id: i64) -> Result<(), i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&1u32).unwrap(); // task count
        data.write(&task_id.to_string()).unwrap();

        // Send resume request
        let mut reply = remote.send_request(interface::RESUME, &mut data).map_err(|_| 13400003)?;

        // Check first error code
        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }

        // Check second error code
        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }

        Ok(())
    }

    /// Removes a download task from the system.
    ///
    /// # Parameters
    /// - `task_id`: The unique identifier of the task to remove
    ///
    /// # Returns
    /// - `Ok(())` if the task was removed successfully
    /// - `Err(i32)` with the error code if removing the task failed
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use request_next::proxy::RequestProxy;
    ///
    /// fn example(task_id: i64) {
    ///     let proxy = RequestProxy::get_instance();
    ///     
    ///     match proxy.remove(task_id) {
    ///         Ok(_) => println!("Task {} removed successfully", task_id),
    ///         Err(code) => eprintln!("Failed to remove task {} with error code: {}", task_id, code),
    ///     }
    /// }
    /// ```
    ///
    /// # Panics
    /// - Panics if parcel operations fail due to IPC errors
    pub(crate) fn remove(&self, task_id: i64) -> Result<(), i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&2u32).unwrap(); // version
        data.write(&1u32).unwrap(); // task count
        data.write(&task_id.to_string()).unwrap();

        // Send remove request
        let mut reply = remote.send_request(interface::REMOVE, &mut data).map_err(|_| 13400003)?;

        // Check first error code
        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }

        // Check second error code
        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }

        Ok(())
    }

    /// Stops a download task.
    ///
    /// # Parameters
    /// - `task_id`: The unique identifier of the task to stop
    ///
    /// # Returns
    /// - `Ok(())` if the task was stopped successfully
    /// - `Err(i32)` with the error code if stopping the task failed
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use request_next::proxy::RequestProxy;
    ///
    /// fn example(task_id: i64) {
    ///     let proxy = RequestProxy::get_instance();
    ///     
    ///     match proxy.stop(task_id) {
    ///         Ok(_) => println!("Task {} stopped successfully", task_id),
    ///         Err(code) => eprintln!("Failed to stop task {} with error code: {}", task_id, code),
    ///     }
    /// }
    /// ```
    ///
    /// # Panics
    /// - Panics if parcel operations fail due to IPC errors
    pub(crate) fn stop(&self, task_id: i64) -> Result<(), i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&1u32).unwrap(); // task count
        data.write(&task_id.to_string()).unwrap();

        // Send stop request
        let mut reply = remote.send_request(interface::STOP, &mut data).map_err(|_| 13400003)?;

        // Check first error code
        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }

        // Check second error code
        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }

        Ok(())
    }

    /// Sets the maximum download speed for a task.
    ///
    /// # Parameters
    /// - `task_id`: The unique identifier of the task
    /// - `speed`: The maximum speed limit to set
    ///
    /// # Returns
    /// - `Ok(())` if the speed limit was set successfully
    /// - `Err(i32)` with the error code if setting the speed limit failed
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use request_next::proxy::RequestProxy;
    ///
    /// fn example(task_id: i64) {
    ///     let proxy = RequestProxy::get_instance();
    ///     // Set maximum speed to 1024000 bytes per second (1MB/s)
    ///     let max_speed = 1024000i64;
    ///     
    ///     match proxy.set_max_speed(task_id, max_speed) {
    ///         Ok(_) => println!("Speed limit set for task {}", task_id),
    ///         Err(code) => eprintln!("Failed to set speed limit with error code: {}", code),
    ///     }
    /// }
    /// ```
    ///
    /// # Panics
    /// - Panics if parcel operations fail due to IPC errors
    pub(crate) fn set_max_speed(&self, task_id: i64, speed: i64) -> Result<(), i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&1u32).unwrap(); // task count
        data.write(&task_id.to_string()).unwrap();
        data.write(&speed).unwrap(); // maximum speed

        // Send set max speed request
        let mut reply = remote
            .send_request(interface::SET_MAX_SPEED, &mut data)
            .map_err(|_| 13400003)?;

        // Check first error code
        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }

        // Check second error code
        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }
        Ok(())
    }
}
