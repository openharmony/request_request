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

//! CXX bridge for interoperability between Rust and C++ code.
//! 
//! This module provides a bridge for communication between Rust components and C++ code,
//! defining shared data structures and functions for cross-language interaction.

// Internal dependencies
use crate::listener::UdsListener;

fn on_response(response: ffi::Response) {
    // Log detailed response information for debugging and monitoring
    info!(
        "on_response: taskId: {}, version: {}, statusCode: {}, reason: {}, headers: {:?}",
        response.taskId, response.version, response.statusCode, response.reason, response.headers
    );
    // Forward the response to the global UDS listener instance
    UdsListener::get_instance().on_response(response);
}

// CXX bridge definition for interoperability between Rust and C++
#[cxx::bridge(namespace = "OHOS::RequestAni")]
pub mod ffi {
    /// Represents a response from the download service.
    ///
    /// Contains all relevant information about a download task's status and result.
    struct Response {
        /// Unique identifier for the download task.
        taskId: String,
        /// API version used for the request.
        version: String,
        /// HTTP status code of the response.
        statusCode: i32,
        /// Reason phrase associated with the status code.
        reason: String,
        /// Response headers as key-value pairs.
        headers: Vec<String>,
    }

    // Rust functions exposed to C++
    extern "Rust" {
        /// Called by C++ code when a response is received.
        fn on_response(response: Response);
    }

    // C++ functions exposed to Rust
    unsafe extern "C++" {
        // Include necessary C++ headers
        include!("subscribe.h");
        include!("wrapper.h");

        /// Gets the application's base directory path.
        fn GetAppBaseDir() -> String;
        
        /// Sets ACL (Access Control List) permissions for a target.
        fn AclSetAccess(target: &str, entry: &str) -> i32;
        
        /// Opens a communication channel with the given file descriptor.
        fn OpenChannel(fd: i32);
    }
}
