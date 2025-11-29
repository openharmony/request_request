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

//! Serialization framework for Unix Domain Socket communication.
//!
//! This module provides functionality for deserializing binary data received through
//! Unix Domain Sockets. Despite its name, the `Serialize` trait in this module is actually
//! responsible for deserialization of primitive types, enums, and complex data structures
//! used in the download service communication protocol.

// Standard library imports
use std::collections::HashMap;
use std::io::Read;

// External dependencies
use request_core::config::{Action, Version};
use request_core::info::{
    FaultOccur, Faults, NotifyData, Progress, Reason, Response, State, SubscribeType, TaskState,
};

/// Binary deserializer for Unix Domain Socket communications.
///
/// Provides methods to read and deserialize various data types from a byte buffer.
/// This is the main entry point for deserializing messages received from the download service.
///
/// # Type Parameters
/// - `'a`: Lifetime of the referenced byte slice
pub struct UdsSer<'a> {
    /// Internal byte buffer containing the data to be deserialized
    inner: &'a [u8],
}

impl UdsSer<'_> {
    /// Creates a new `UdsSer` instance with the provided byte buffer.
    ///
    /// # Parameters
    /// - `inner`: Byte buffer containing serialized data to be deserialized
    ///
    /// # Returns
    /// A new `UdsSer` instance ready to deserialize data
    ///
    /// # Examples
    ///
    /// ```rust
    /// use request_next::listen::ser::UdsSer;
    ///
    /// let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]; // Example bytes
    /// let mut serializer = UdsSer::new(&data);
    /// // let value: i64 = serializer.read(); // Deserialize data
    /// ```
    pub fn new(inner: &[u8]) -> UdsSer {
        UdsSer { inner }
    }

    /// Deserializes data of the specified type from the buffer.
    ///
    /// Uses the `Serialize` implementation for the target type to read and deserialize
    /// the appropriate number of bytes from the internal buffer.
    ///
    /// # Type Parameters
    /// - `S`: Type implementing the `Serialize` trait to deserialize
    ///
    /// # Returns
    /// Deserialized value of type `S`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use request_next::listen::ser::UdsSer;
    ///
    /// // Example with i32
    /// let data = [0x01, 0x02, 0x03, 0x04];
    /// let mut serializer = UdsSer::new(&data);
    /// // let value: i32 = serializer.read(); // 67305985 in little-endian
    /// ```
    pub fn read<S: Serialize>(&mut self) -> S {
        S::read(self)
    }
}

/// Trait for types that can be deserialized from a `UdsSer` buffer.
///
/// Despite its name, this trait defines the deserialization behavior for types,
/// specifying how to read and interpret bytes from a binary stream.
///
/// # Notes
/// The trait name `Serialize` is somewhat misleading as it actually handles deserialization.
/// It reads from a binary format and constructs the corresponding Rust types.
pub trait Serialize {
    /// Reads and deserializes a value from the provided `UdsSer` instance.
    ///
    /// # Parameters
    /// - `ser`: Mutable reference to the `UdsSer` instance containing the serialized data
    ///
    /// # Returns
    /// Deserialized value of the implementing type
    fn read(ser: &mut UdsSer) -> Self;
}

/// Deserializes an `i64` from the binary stream.
///
/// Reads exactly 8 bytes and interprets them as a little-endian i64 value.
impl Serialize for i64 {
    fn read(ser: &mut UdsSer) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&ser.inner[..8]);
        ser.inner = &ser.inner[8..];
        i64::from_ne_bytes(bytes)
    }
}

/// Deserializes a `u64` from the binary stream.
///
/// Reads exactly 8 bytes and interprets them as a little-endian u64 value.
impl Serialize for u64 {
    fn read(ser: &mut UdsSer) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&ser.inner[..8]);
        ser.inner = &ser.inner[8..];
        u64::from_ne_bytes(bytes)
    }
}

/// Deserializes an `i32` from the binary stream.
///
/// Reads exactly 4 bytes and interprets them as a little-endian i32 value.
impl Serialize for i32 {
    fn read(ser: &mut UdsSer) -> Self {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&ser.inner[..4]);
        ser.inner = &ser.inner[4..];
        i32::from_ne_bytes(bytes)
    }
}

/// Deserializes a `u32` from the binary stream.
///
/// Reads exactly 4 bytes and interprets them as a little-endian u32 value.
impl Serialize for u32 {
    fn read(ser: &mut UdsSer) -> Self {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&ser.inner[..4]);
        ser.inner = &ser.inner[4..];
        u32::from_ne_bytes(bytes)
    }
}

/// Deserializes an `i16` from the binary stream.
///
/// Reads exactly 2 bytes and interprets them as a little-endian i16 value.
impl Serialize for i16 {
    fn read(ser: &mut UdsSer) -> Self {
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(&ser.inner[..2]);
        ser.inner = &ser.inner[2..];
        i16::from_ne_bytes(bytes)
    }
}

/// Deserializes a `State` enum from the binary stream.
///
/// Reads a u32 value and converts it to the corresponding `State` variant.
impl Serialize for State {
    fn read(ser: &mut UdsSer) -> Self {
        let state: u32 = ser.read();
        State::from(state)
    }
}

/// Deserializes an `Action` enum from the binary stream.
///
/// Reads a u32 value and converts it to the corresponding `Action` variant.
impl Serialize for Action {
    fn read(ser: &mut UdsSer) -> Self {
        let action: u32 = ser.read();
        Action::from(action)
    }
}

/// Deserializes a `Version` enum from the binary stream.
///
/// Reads a u32 value and converts it to the corresponding `Version` variant.
impl Serialize for Version {
    fn read(ser: &mut UdsSer) -> Self {
        let version: u32 = ser.read();
        Version::from(version)
    }
}

/// Deserializes a `SubscribeType` enum from the binary stream.
///
/// Reads a u32 value and converts it to the corresponding `SubscribeType` variant.
impl Serialize for SubscribeType {
    fn read(ser: &mut UdsSer) -> Self {
        let subscribe_type: u32 = ser.read();
        SubscribeType::from(subscribe_type)
    }
}

impl Serialize for FaultOccur {
    fn read(ser: &mut UdsSer) -> Self {
        // let task_id = ser.read::<i32>() as i64;
        let task_id = ser.read::<i32>();
        let subscribe_type = ser.read::<SubscribeType>();
        let faults: Faults = ser.read::<Reason>().into();
        FaultOccur {
            task_id,
            subscribe_type,
            faults,
        }
    }
}

impl Serialize for Reason {
    fn read(ser: &mut UdsSer) -> Self {
        let reason: u32 = ser.read();
        Reason::from(reason)
    }
}

/// Deserializes a `String` from the binary stream.
///
/// Reads bytes until a null terminator (\0) is found, then converts to a String.
/// Handles invalid UTF-8 by replacing with lossy UTF-8 representations.
impl Serialize for String {
    fn read(ser: &mut UdsSer) -> Self {
        if let Some(s) = ser.inner.split(|a| *a == b'\0').next() {
            ser.inner = &ser.inner[s.len() + 1..];
            String::from_utf8_lossy(s).to_string()
        } else {
            String::new()
        }
    }
}

/// Deserializes a header `HashMap<String, Vec<String>>` from the binary stream.
///
/// Reads the entire remaining buffer as text, then parses each line as a header entry.
/// Headers are expected to be in the format `Key: Value1,Value2,...`.
impl Serialize for HashMap<String, Vec<String>> {
    fn read(ser: &mut UdsSer) -> Self {
        let mut map = HashMap::new();
        let mut s = String::new();
        let _ = ser.inner.read_to_string(&mut s);
        info!("headers {}", s);
        for line in s.lines() {
            let Some(index) = line.find(':') else {
                map.insert(line.to_string(), vec![]);
                continue;
            };
            let (key, value) = line.split_at(index);
            let value = &value[1..];
            let value: Vec<String> = value.split(',').map(String::from).collect();
            map.insert(key.to_string(), value);
        }
        map
    }
}

/// Deserializes a `HashMap<String, String>` from the binary stream.
///
/// First reads the number of entries (u32), then reads each key-value pair sequentially.
impl Serialize for HashMap<String, String> {
    fn read(ser: &mut UdsSer) -> Self {
        let mut map = HashMap::new();
        let length: u32 = ser.read();

        for _ in 0..length {
            let key = ser.read::<String>();
            let value = ser.read::<String>();
            map.insert(key, value);
        }
        map
    }
}

/// Deserializes a `Vec<i64>` from the binary stream.
///
/// First reads the length of the vector (u32), then reads each i64 value sequentially.
impl Serialize for Vec<i64> {
    fn read(ser: &mut UdsSer) -> Self {
        let length: u32 = ser.read();

        let mut vec = Vec::with_capacity(length as usize);
        for _ in 0..length {
            vec.push(ser.read());
        }
        vec
    }
}

/// Deserializes a `Response` from the binary stream.
///
/// Reads all fields of a Response sequentially: task_id, version, status_code, reason, and headers.
/// The task_id is converted from i32 to String as part of the deserialization process.
impl Serialize for Response {
    fn read(ser: &mut UdsSer) -> Self {
        let task_id = ser.read::<i32>();
        let version = ser.read::<String>();
        let status_code: i32 = ser.read();

        let reason = ser.read::<String>();
        let headers: HashMap<String, Vec<String>> = ser.read();

        info!("headers {:?}", headers);

        Response {
            task_id: task_id.to_string(),
            version,
            status_code,
            reason,
            headers,
        }
    }
}

/// Deserializes a `Vec<TaskState>` from the binary stream.
///
/// First reads the length of the vector (u32), then reads each TaskState sequentially.
impl Serialize for Vec<TaskState> {
    fn read(ser: &mut UdsSer) -> Self {
        let length: u32 = ser.read();
        let mut vec = Vec::with_capacity(length as usize);

        for _ in 0..length {
            let path = ser.read::<String>();

            let response_code: u32 = ser.read();
            let message = ser.read::<String>();

            vec.push(TaskState {
                path,
                response_code,
                message,
            });
        }
        vec
    }
}

/// Deserializes a `Progress` from the binary stream.
///
/// Reads all fields of a Progress sequentially: state, index, processed, total_processed,
/// sizes, and extras.
impl Serialize for Progress {
    fn read(ser: &mut UdsSer) -> Self {
        let state: State = ser.read();
        let index: u32 = ser.read();
        let processed: u64 = ser.read();
        let total_processed: u64 = ser.read();
        let sizes: Vec<i64> = ser.read();
        let extras: HashMap<String, String> = ser.read();
        // let body_bytes: Vec<u8> = ser.read();

        Progress {
            state,
            index,
            processed,
            total_processed,
            sizes,
            extras,
            body_bytes: Vec::new(),
        }
    }
}

/// Deserializes a `NotifyData` from the binary stream.
///
/// Reads all fields of a NotifyData sequentially: subscribe_type, task_id, progress,
/// action, version, and task_states.
impl Serialize for NotifyData {
    fn read(ser: &mut UdsSer) -> Self {
        let subscribe_type: SubscribeType = ser.read();
        let task_id: u32 = ser.read();

        let progress: Progress = ser.read();

        let action: Action = ser.read();
        let version: Version = ser.read();

        let task_states = ser.read::<Vec<TaskState>>();

        NotifyData {
            subscribe_type,
            task_id,
            progress,
            action,
            version,
            task_states,
        }
    }
}
