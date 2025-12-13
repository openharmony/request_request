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
pub mod constant;

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
        "checkDownloadConfig": api9::download::check_config,  
        "checkUploadConfig": api9::upload::check_config, 
        "downloadFileSync": api9::download::download_file, // Synchronous file download
        "uploadFileSync": api9::upload::upload_file,       // Synchronous file upload
    ]
    // API 9 DownloadTaskInner class method bindings
    class "L@ohos/request/request/DownloadTaskInner"
    [
        "onProgressInner": api9::callback::on_progress,
        "onEvent": api9::callback::on_event,
        "onFailInner": api9::callback::on_fail,
        "offProgressInner": api9::callback::off_progress,
        "offEvent": api9::callback::off_event,
        "offFailInner": api9::callback::off_fail,
        "deleteSync": api9::download::delete,
        "suspendSync": api9::download::suspend,
        "restoreSync": api9::download::restore,
        "getTaskInfoSync": api9::download::get_task_info,
        "getTaskMimeTypeSync": api9::download::get_task_mime_type,
        "offEvents": api9::callback::off_events,
    ]
    // API 9 UploadTaskInner class method bindings
    class "L@ohos/request/request/UploadTaskInner"
    [
        "deleteSync": api9::upload::delete,
        "onProgressInner": api9::callback::on_progress_uploadtask,
        "onEventInner": api9::callback::on_event_uploadtask,
        "onHeaderReceiveInner": api9::callback::on_header_receive,
        "offProgressInner": api9::callback::off_progress_uploadtask,
        "offEventInner": api9::callback::off_event_uploadtask,
        "offHeaderReceiveInner": api9::callback::off_header_receive,
        "offEvents": api9::callback::off_events,
    ]
    // API 10 namespace bindings for agent operations
    namespace "L@ohos/request/request/agent"
    [
        "checkConfig": api10::agent::check_config,            // Verify config
        "createSync": api10::agent::create,                   // Create new task
        "getTaskSync": api10::agent::get_task,                // Get existing task
        "removeSync": api10::agent::remove,                   // Remove task
        "showSync": api10::agent::show,                       // Show task notification
        "checkToken": api10::agent::check_token,              // Check Touch Config
        "checkTid": api10::agent::check_tid,                  // Check Task Id
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
        "startSync": api10::task::start,
        "pauseSync": api10::task::pause,
        "resumeSync": api10::task::resume,
        "stopSync": api10::task::stop,
        "onEvent": api10::callback::on_event,
        "onResponseEvent": api10::callback::on_response_event,
        "onFaultEvent": api10::callback::on_fault_event,
        "setMaxSpeedSync": api10::task::set_max_speed,
        "offEvent": api10::callback::off_event,
        "offResponseEvent": api10::callback::off_response_event,
        "offFaultEvent": api10::callback::off_fault_event,
        "offEvents": api10::callback::off_events,
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
