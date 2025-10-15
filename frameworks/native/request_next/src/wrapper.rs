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
// use crate::listener::UdsListener;

// fn on_response(response: ffi::Response) {
//     info!(
//         "on_response: taskId: {}, version: {}, statusCode: {}, reason: {}, headers: {:?}",
//         response.taskId, response.version, response.statusCode, response.reason, response.headers
//     );
//     UdsListener::get_instance().on_response(response);
// }

#[cxx::bridge(namespace = "OHOS::Request")]
pub mod ffi {
    unsafe extern "C++" {
        include!("file_uri.h");

        #[namespace = "OHOS::AppFileService::ModuleFileUri"]
        type FileUri;

        #[namespace = "OHOS::AppFileService::ModuleFileUri"]
        fn FileUri(uri: &CxxString) -> FileUri;

        #[namespace = "OHOS::AppFileService::ModuleFileUri"]
        fn GetRealPath(self: &FileUri) -> CxxString;
    }


    // struct Response {
    //     taskId: String,
    //     version: String,
    //     statusCode: i32,
    //     reason: String,
    //     headers: Vec<String>,
    // }

    // extern "Rust" {
    //     fn on_response(response: Response);
    // }

    // unsafe extern "C++" {
    //     include!("subscribe.h");
    //     include!("wrapper.h");

    //     fn GetAppBaseDir() -> String;
    //     fn AclSetAccess(target: &str, entry: &str) -> i32;
    //     fn OpenChannel(fd: i32);
    // }
}
