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

//! Unix Domain Socket (UDS) communication for the RequestProxy.
//! 
//! This module implements methods for establishing and managing Unix Domain Socket
//! communication channels with the download service, as well as task subscription
//! and unsubscription functionality.

// Standard library dependencies
use std::fs::File;
use std::os::fd::{IntoRawFd, RawFd};

// IPC and download core dependencies
use ipc::parcel::MsgParcel;
use request_core::interface;

// Local dependencies
use super::{RequestProxy, SERVICE_TOKEN};

impl RequestProxy {
    /// Opens a Unix Domain Socket communication channel with the download service.
    ///
    /// Requests the download service to create a new file descriptor for establishing
    /// a direct communication channel. Returns the file descriptor wrapped in a `File`
    /// object if successful.
    ///
    /// # Returns
    /// - `Ok(File)` with the file descriptor for the communication channel if successful
    /// - `Err(i32)` with the error code if opening the channel failed
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use request_next::proxy::RequestProxy;
    ///
    /// fn example() {
    ///     let proxy = RequestProxy::get_instance();
    ///     
    ///     match proxy.open_channel() {
    ///         Ok(file) => println!("Communication channel opened successfully"),
    ///         Err(code) => eprintln!("Failed to open channel with error code: {}", code),
    ///     }
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error with code from the download service if the channel cannot be opened
    ///
    /// # Panics
    /// - Panics if parcel operations fail due to IPC errors
    pub(crate) fn open_channel(&self) -> Result<File, i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        // Write interface token to identify the service
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        // Request to open a new communication channel
        let mut reply = remote
            .send_request(interface::OPEN_CHANNEL, &mut data)
            .map_err(|_| 13400003)?;

        // Check if the channel was opened successfully
        let code = reply.read::<i32>().unwrap();
        if code != 0 {
            error!("open channel failed: {}", code);
            return Err(code);
        }

        // Read and return the file descriptor
        let file = reply.read_file().unwrap();
        Ok(file)
    }

    /// Subscribes to updates for a specific download task.
    ///
    /// Registers to receive status updates for the specified task ID.
    ///
    /// # Parameters
    /// - `task_id`: The unique identifier of the task to subscribe to, as a string
    ///
    /// # Returns
    /// - `Ok(())` if subscription was successful
    /// - `Err(i32)` with the error code if subscription failed
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use request_next::proxy::RequestProxy;
    ///
    /// fn example() {
    ///     let proxy = RequestProxy::get_instance();
    ///     let task_id = "12345678".to_string();
    ///     
    ///     match proxy.subscribe(task_id) {
    ///         Ok(_) => println!("Successfully subscribed to task updates"),
    ///         Err(code) => eprintln!("Failed to subscribe with error code: {}", code),
    ///     }
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error with code from the download service if subscription fails
    ///
    /// # Panics
    /// - Panics if parcel operations fail due to IPC errors
    pub(crate) fn subscribe(&self, task_id: String) -> Result<(), i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        // Write interface token to identify the service
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        // Write the task ID to subscribe to
        data.write(&task_id).unwrap();

        // Send subscription request
        let mut reply = remote
            .send_request(interface::SUBSCRIBE, &mut data)
            .map_err(|_| 13400003)?;
        
        // Check subscription result
        let code = reply.read::<i32>().unwrap();
        if code != 0 {
            error!("subscribe task failed: {}", code);
            return Err(code);
        }

        Ok(())
    }

    /// Unsubscribes from updates for a specific download task.
    ///
    /// Cancels the registration to receive status updates for the specified task ID.
    ///
    /// # Parameters
    /// - `task_id`: The unique identifier of the task to unsubscribe from
    ///
    /// # Returns
    /// - `Ok(())` if unsubscription was successful
    /// - `Err(i32)` with the error code if unsubscription failed
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use request_next::proxy::RequestProxy;
    ///
    /// fn example() {
    ///     let proxy = RequestProxy::get_instance();
    ///     let task_id = 12345678i64;
    ///     
    ///     match proxy.Unsubscribe(task_id) {
    ///         Ok(_) => println!("Successfully unsubscribed from task updates"),
    ///         Err(code) => eprintln!("Failed to unsubscribe with error code: {}", code),
    ///     }
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error with code from the download service if unsubscription fails
    ///
    /// # Notes
    /// - Note that the method name starts with an uppercase 'U', which is unconventional for Rust
    ///   but preserved to maintain compatibility with the existing API
    ///
    /// # Panics
    /// - Panics if parcel operations fail due to IPC errors
    pub(crate) fn Unsubscribe(&self, task_id: i64) -> Result<(), i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        // Write interface token to identify the service
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        // Convert task ID to string and write to parcel
        data.write(&task_id.to_string()).unwrap();
        
        // Send unsubscription request
        let mut reply = remote
            .send_request(interface::UNSUBSCRIBE, &mut data)
            .map_err(|_| 13400003)?;

        // Check unsubscription result
        let code = reply.read::<i32>().unwrap();
        if code != 0 {
            error!("unsubscribe task failed: {}", code);
            return Err(code);
        }
        Ok(())
    }
}
