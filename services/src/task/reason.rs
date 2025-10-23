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

//! Error and state reason codes for request tasks.
//! 
//! This module defines the `Reason` enumeration that represents various states and errors
//! that can occur during task execution, including network issues, user operations, and system conditions.

// Re-export the Reason enum from the FFI module
pub(crate) use ffi::Reason;

// C++ FFI bridge for the Reason enum
#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {
    /// Enum representing task state and error reasons.
    #[derive(Clone, Copy, PartialEq, Debug)]
    #[repr(u8)]
    pub(crate) enum Reason {
        /// Default reason (no specific reason).
        Default = 0,
        /// Task has been pending for one month without completion.
        TaskSurvivalOneMonth,
        /// Too many tasks are running simultaneously.
        RunningTaskMeetLimits = 4,
        /// Action performed by the user.
        UserOperation,
        /// Application is in background or has terminated.
        AppBackgroundOrTerminate,
        /// Network connection is offline.
        NetworkOffline,
        /// Network type is not supported for the task.
        UnsupportedNetworkType,
        /// Failed to construct the request.
        BuildRequestFailed = 10,
        /// Failed to retrieve file size information from server.
        GetFileSizeFailed,
        /// Continuous task processing timed out.
        ContinuousTaskTimeout = 12,
        /// General request error.
        RequestError = 14,
        /// File upload failed.
        UploadFileError,
        /// HTTP redirect processing error.
        RedirectError,
        /// HTTP protocol violation.
        ProtocolError,
        /// Input/output operation failed.
        IoError,
        /// Server does not support range requests.
        UnsupportedRangeRequest,
        /// Catch-all for other errors not explicitly defined.
        OthersError,
        /// User account is stopped.
        AccountStopped,
        /// DNS resolution failed.
        Dns = 23,
        /// TCP connection error.
        Tcp,
        /// SSL/TLS handshake or connection error.
        Ssl,
        /// Insufficient storage space available.
        InsufficientSpace,
        /// Combined condition: network offline and app in background/terminated.
        NetworkApp = 27,
        /// Combined condition: network offline and account stopped.
        NetworkAccount = 28,
        /// Combined condition: app in background/terminated and account stopped.
        AppAccount = 29,
        /// Combined condition: network offline, app in background/terminated, and account stopped.
        NetworkAppAccount = 30,
        /// Transfer speed below configured minimum threshold.
        LowSpeed = 31,
    }
}

/// Converts a raw byte value to a Reason enum variant.
/// 
/// # Arguments
/// 
/// * `value` - The raw byte value to convert.
/// 
/// # Returns
/// 
/// The corresponding Reason variant, or `Reason::OthersError` if the value is unrecognized.
impl From<u8> for Reason {
    fn from(value: u8) -> Self {
        match value {
            0 => Reason::Default,
            1 => Reason::TaskSurvivalOneMonth,
            4 => Reason::RunningTaskMeetLimits,
            5 => Reason::UserOperation,
            6 => Reason::AppBackgroundOrTerminate,
            7 => Reason::NetworkOffline,
            8 => Reason::UnsupportedNetworkType,
            10 => Reason::BuildRequestFailed,
            11 => Reason::GetFileSizeFailed,
            12 => Reason::ContinuousTaskTimeout,
            14 => Reason::RequestError,
            15 => Reason::UploadFileError,
            16 => Reason::RedirectError,
            17 => Reason::ProtocolError,
            18 => Reason::IoError,
            19 => Reason::UnsupportedRangeRequest,
            21 => Reason::AccountStopped,
            23 => Reason::Dns,
            24 => Reason::Tcp,
            25 => Reason::Ssl,
            26 => Reason::InsufficientSpace,
            27 => Reason::NetworkApp,
            28 => Reason::NetworkAccount,
            29 => Reason::AppAccount,
            30 => Reason::NetworkAppAccount,
            31 => Reason::LowSpeed,
            _ => Reason::OthersError, // Fallback for unrecognized values
        }
    }
}

impl Reason {
    /// Converts the reason to a descriptive string.
    /// 
    /// # Returns
    /// 
    /// A static string describing the reason.
    pub(crate) fn to_str(self) -> &'static str {
        match self {
            Reason::Default => "",
            Reason::TaskSurvivalOneMonth => "The task has not been completed for a month yet",
            Reason::RunningTaskMeetLimits => "Too many task in running state",
            Reason::UserOperation => "User operation",
            Reason::AppBackgroundOrTerminate => "The app is background or terminate",
            Reason::NetworkOffline => "NetWork is offline",
            Reason::UnsupportedNetworkType => "NetWork type not meet the task config",
            Reason::BuildRequestFailed => "Build request error",
            Reason::GetFileSizeFailed => "Failed because cannot get the file size from the server and the precise is setted true by user",
            Reason::ContinuousTaskTimeout => "Continuous processing task time out",
            Reason::RequestError => "Request error",
            Reason::UploadFileError => "There are some files upload failed",
            Reason::RedirectError => "Redirect error",
            Reason::ProtocolError => "Http protocol error",
            Reason::IoError => "Io Error",
            Reason::UnsupportedRangeRequest => "The server is not support range request",
            Reason::OthersError => "Some other error occured",
            Reason::AccountStopped => "Account stopped",
            Reason::Dns => "DNS error",
            Reason::Tcp => "TCP error",
            Reason::Ssl => "TSL/SSL error",
            Reason::InsufficientSpace => "Insufficient space",
            Reason::NetworkApp => "NetWork is offline and the app is background or terminate",
            Reason::NetworkAccount => "NetWork is offline and the account is stopped",
            Reason::AppAccount => "The app is background or terminate and the account is stopped",
            Reason::NetworkAppAccount => "NetWork is offline and the app is background or terminate and the account is stopped",
            Reason::LowSpeed => "Below low speed limit",
            _ => "unknown error",
        }
    }
}

// Test module for Reason
#[cfg(test)]
mod ut_reason {
    include!("../../tests/ut/task/ut_reason.rs");
}
