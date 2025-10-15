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

//! Unix Domain Socket communication module.
//! 
//! This module provides functionality for receiving messages from the download service
//! through Unix Domain Sockets. It handles message validation, deserialization, and
//! provides a structured interface for accessing different types of messages.

// Standard library imports
use std::fs::File;
use std::io;
use std::os::fd::{FromRawFd, IntoRawFd};
use std::os::unix;

use request_core::info::{FaultOccur, Faults, NotifyData, Response};
use ylong_runtime::net::UnixDatagram;

// Local dependencies
use crate::listen::ser::UdsSer;

/// Magic number for message validation.
///
/// Used as a header identifier to validate the authenticity of received messages.
/// Value is "CCFF" in ASCII hexadecimal.
const MAGIC_NUM: i32 = 0x43434646;

/// Message type identifier for HTTP responses.
///
/// Indicates that the message contains an HTTP response from the download service.
const HTTP_RESPONSE: i16 = 0;

/// Message type identifier for notification data.
///
/// Indicates that the message contains notification data about download tasks.
const NOTIFY_DATA: i16 = 1;
const FAULTS: i16 = 2;

/// Listener for Unix Domain Socket messages.
///
/// Provides methods to receive and process messages from the download service.
/// Maintains message sequence tracking and handles validation of incoming data.
pub struct UdsListener {
    /// The Unix Domain Socket used for receiving messages
    socket: UnixDatagram,

    /// Tracks the expected message ID for sequential validation
    message_id: i32,
}

impl UdsListener {
    /// Creates a new `UdsListener` from a file descriptor.
    ///
    /// Takes ownership of the provided file and initializes a Unix Domain Socket from it.
    /// Converts the standard library socket to an asynchronous socket from ylong_runtime.
    ///
    /// # Parameters
    /// - `file`: File object representing the socket file descriptor
    ///
    /// # Returns
    /// A new `UdsListener` instance configured with the provided socket
    ///
    /// # Safety
    /// Uses unsafe code to convert from a raw file descriptor to a socket.
    /// The caller must ensure the file descriptor is valid and properly initialized.
    pub fn new(file: File) -> Self {
        // Convert file descriptor to Unix datagram socket
        let socket = unsafe { unix::net::UnixDatagram::from_raw_fd(file.into_raw_fd()) };

        // Convert standard socket to async socket using ylong_runtime
        let socket = ylong_runtime::block_on(async { UnixDatagram::from_std(socket).unwrap() });

        Self {
            socket,
            message_id: 1, // Start with message ID 1
        }
    }

    /// Receives and processes a message from the socket.
    ///
    /// Reads data from the socket, sends an acknowledgment with the received size,
    /// validates the message header, and deserializes the appropriate message type.
    ///
    /// # Returns
    /// A `Result` containing either:
    /// - `Ok(Message)` with the deserialized message
    /// - `Err(io::Error)` if there was an error receiving or processing the message
    ///
    /// # Errors
    /// - Returns `io::ErrorKind::InvalidData` if message validation fails or if the message type is unknown
    /// - Returns other `io::Error` variants for socket operation failures
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::fs::File;
    /// use request_next::listen::uds::UdsListener;
    /// 
    /// async fn example() -> Result<(), std::io::Error> {
    ///     // Assuming socket_file is a valid file descriptor
    ///     let socket_file = File::open("/path/to/socket")?;
    ///     let mut listener = UdsListener::new(socket_file);
    ///     
    ///     // Receive a message
    ///     match listener.recv().await? {
    ///         request_next::listen::uds::Message::HttpResponse(response) => {
    ///             println!("Received HTTP response for task: {}", response.task_id);
    ///         }
    ///         request_next::listen::uds::Message::NotifyData(notify_data) => {
    ///             println!("Received notification for task: {}", notify_data.task_id);
    ///         }
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn recv(&mut self) -> Result<Message, io::Error> {
        // Buffer for receiving data
        let mut buf = [0u8; 4096];
        // Receive data from socket
        let size = self.socket.recv(&mut buf).await?;
        // Send acknowledgment with received size
        let ret = (size as u32).to_ne_bytes();
        self.socket.send(&ret).await?;

        // Create deserializer with received data
        let mut uds = UdsSer::new(&buf[..size]);

        // Variable to store message type
        let mut msg_type: i16 = 0;

        // Validate message header
        if !message_check(&mut uds, size as i16, self.message_id, &mut msg_type) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Message check failed",
            ));
        }

        // Increment message ID for next expected message
        self.message_id += 1;

        info!("Message ID: {}, Type: {}", self.message_id, msg_type);

        // Deserialize based on message type
        if msg_type == HTTP_RESPONSE {
            let response: Response = uds.read();
            Ok(Message::HttpResponse(response))
        } else if msg_type == NOTIFY_DATA {
            let notify_data: NotifyData = uds.read();
            Ok(Message::NotifyData(notify_data))
        } else if msg_type == FAULTS {
            let fault_occur: FaultOccur = uds.read();
            Ok(Message::Faults(fault_occur))
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown message type: {}", msg_type),
            ))
        }
    }
}

/// Enum representing the types of messages received from the download service.
///
/// Provides a structured way to handle different message types with pattern matching.
pub enum Message {
    /// HTTP response message containing response data for a download task
    HttpResponse(Response),
    /// Notification data message containing status updates for download tasks
    NotifyData(NotifyData),
    Faults(FaultOccur),
}

/// Validates the header of a received message.
///
/// Checks the magic number, message ID, and body size to ensure message integrity.
/// Updates the message type pointer with the extracted value.
///
/// # Parameters
/// - `uds`: Deserializer to read the message header
/// - `size`: Size of the received message in bytes
/// - `message_id`: Expected message ID for validation
/// - `msg_type`: Output parameter to store the extracted message type
///
/// # Returns
/// `true` if message validation succeeded, `false` if validation failed
///
/// # Notes
/// Message ID mismatches are logged but do not cause validation failure.
fn message_check(uds: &mut UdsSer, size: i16, message_id: i32, msg_type: &mut i16) -> bool {
    // Validate magic number
    let magic_num: i32 = uds.read();
    if magic_num != MAGIC_NUM as i32 {
        error!("Invalid magic number: {}", magic_num);
        return false;
    }

    // Check message ID (log but don't fail on mismatch)
    let msg_id: i32 = uds.read();
    if msg_id != message_id {
        error!(
            "Message ID mismatch: expected {}, got {}",
            message_id, msg_id
        );
    }

    // Extract message type
    *msg_type = uds.read();

    // Validate body size
    let body_size: i16 = uds.read();
    if body_size != size as i16 {
        error!("Body size mismatch: expected {}, got {}", size, body_size);
        return false;
    }
    true
}
