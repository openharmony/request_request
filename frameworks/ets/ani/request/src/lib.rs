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

//! Request service ANI (Ark Native Interface) implementation.
//! 
//! This crate provides the native implementation of the request service API
//! for the OpenHarmony operating system, supporting both API version 9 and API version 10.
//! It includes functionality for download and upload tasks, task management,
//! callbacks, and sequence ID generation.

use ani_rs::ani_constructor;

// Public API modules
pub mod api10; // API version 10 implementation
pub mod api9;  // API version 9 implementation
mod seq;       // Internal sequence ID generation

#[macro_use]
extern crate request_utils;

use hilog_rust::{HiLogLabel, LogType};

/// Logger configuration for the request service.
///
/// Defines the log label used for all logging operations within the service,
/// with core log type, domain identifier, and module tag.
pub(crate) const LOG_LABEL: HiLogLabel = HiLogLabel {
    log_type: LogType::LogCore,
    domain: 0xD001C50,
    tag: "RequestAni",
};

// Register Rust functions with the ANI framework
// This macro binds Rust implementations to JavaScript/TypeScript interfaces
ani_constructor!(
    // API 9 namespace bindings for direct function calls
    namespace "L@ohos/request/request"
    [
        "downloadFileSync": api9::download::download_file, // Synchronous file download
        "uploadFileSync": api9::upload::upload_file,       // Synchronous file upload
    ]
    // API 9 DownloadTaskInner class method bindings
    class "L@ohos/request/request/DownloadTaskInner"
    [
        "onProgress": api9::callback::on_progress,                      // Progress callback registration
        "onEvent": api9::callback::on_event,                            // Event callback registration
        "onFail": api9::callback::on_fail,                              // Failure callback registration
        "deleteSync": api9::download::delete,                           // Delete download task
        "suspendSync": api9::download::suspend,                         // Suspend download task
        "restoreSync": api9::download::restore,                         // Resume download task
        "getTaskInfoSync": api9::download::get_task_info,               // Get task information
        "getTaskMimeTypeSync": api9::download::get_task_mime_type,       // Get task MIME type
    ]
    // API 9 UploadTaskInner class method bindings
    class "L@ohos/request/request/UploadTaskInner"
    [
        "deleteSync": api9::upload::delete, // Delete upload task
    ]
    // API 10 namespace bindings for agent operations
    namespace "L@ohos/request/request/agent"
    [
        "createSync": api10::agent::create,                   // Create new task
        "getTaskSync": api10::agent::get_task,                // Get existing task
        "removeSync": api10::agent::remove,                   // Remove task
        "showSync": api10::agent::show,                       // Show task notification
        "touchSync": api10::agent::touch,                     // Update task timestamp
        "searchSync": api10::agent::search,                   // Search tasks
        "querySync": api10::agent::query,                     // Query task details
        "createGroupSync": api10::notification::create_group, // Create notification group
        "attachGroupSync": api10::notification::attach_group, // Attach task to notification group
        "deleteGroupSync": api10::notification::delete_group, // Delete notification group
    ]
    // API 10 TaskInner class method bindings
    class "L@ohos/request/request/agent/TaskInner"
    [
        "startSync": api10::task::start,                       // Start task
        "pauseSync": api10::task::pause,                       // Pause task
        "resumeSync": api10::task::resume,                     // Resume task
        "stopSync": api10::task::stop,                         // Stop task
        "onEvent": api10::callback::on_event,                  // Register event callback
        "onResponseEvent": api10::callback::on_response_event, // Register response event callback
        "setMaxSpeedSync": api10::task::set_max_speed,         // Set task speed limit
    ]
);

// Service initialization code that runs at startup
// The .init_array section ensures this runs early during initialization
#[used]
#[link_section = ".init_array"]
static A: extern "C" fn() = {
    #[link_section = ".text.startup"]
    extern "C" fn init() {
        // Log service initialization
        info!("begin request service init");
        
        // Set up panic hook to log panic information
        // This ensures that panics are logged rather than silently terminating the process
        std::panic::set_hook(Box::new(|info| {
            info!("Panic occurred: {:?}", info);
        }));
    }
    init
};
