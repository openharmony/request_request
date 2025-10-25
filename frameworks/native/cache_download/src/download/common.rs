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

//! Common traits for download operations.
//! 
//! This module defines common interfaces used across download implementations,
//! including traits for responses, errors, and operation handles.

/// Common interface for response objects.
///
/// Provides a consistent way to access status codes from different response types.
pub(crate) trait CommonResponse {
    /// Returns the HTTP status code.
    ///
    /// # Returns
    /// The status code as a 32-bit unsigned integer.
    fn code(&self) -> u32;
}

/// Common interface for error objects.
///
/// Provides consistent access to error codes and messages across different error types.
pub(crate) trait CommonError {
    /// Returns the error code.
    ///
    /// # Returns
    /// The error code as a 32-bit signed integer.
    fn code(&self) -> i32;
    
    /// Returns the error message.
    ///
    /// # Returns
    /// A string containing the human-readable error message.
    fn msg(&self) -> String;
}

/// Common interface for download operation handles.
///
/// Provides operations for controlling and tracking download tasks. Requires implementation
/// to be thread-safe with `Send + Sync` bounds.
pub(crate) trait CommonHandle: Send + Sync {
    /// Cancels the associated download operation.
    ///
    /// # Returns
    /// `true` if cancellation was successful, `false` otherwise.
    fn cancel(&self) -> bool;
    
    /// Increments a reference count or usage counter for the operation.
    ///
    /// Typically used to track active references to the handle to manage its lifecycle.
    fn add_count(&self);
    
    /// Resets the download operation state.
    ///
    /// # Notes
    /// Only available when the `netstack` feature is enabled.
    #[cfg(feature = "netstack")]
    fn reset(&self);
}
