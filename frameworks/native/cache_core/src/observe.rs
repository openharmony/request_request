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

//! Directory observation and file monitoring.
//! 
//! This module provides functionality for observing directory changes, specifically
//! for monitoring image file deletions and maintaining history directories. It integrates
//! with the native file system monitoring capabilities through FFI calls.

use crate::data::observer::DirRebuilder;
use crate::data::{init_history_store_dir, is_history_init, HistoryDir};
use crate::wrapper::ffi::{NewDirectoryMonitor, StartObserve};
use cxx::let_cxx_string;
use std::{path::PathBuf, sync::Arc};

/// Observes image file deletion events for the specified path.
///
/// Initializes history directory tracking if not already initialized, creating a history
/// directory for the provided image path. This function sets up the necessary infrastructure
/// for monitoring image file deletions to maintain cache consistency.
///
/// # Parameters
/// - `path`: Path to the image directory to monitor for deletion events
///
/// # Examples
///
/// ```rust
/// use cache_core::observe::observe_image_file_delete;
///
/// // Start observing an image directory for deletions
/// observe_image_file_delete("/path/to/images".to_string());
/// ```
pub fn observe_image_file_delete(path: String) {
    // Only initialize history tracking if it hasn't been done already
    if !is_history_init() {
        let image_path = PathBuf::from(path);
        let history = Arc::new(HistoryDir::new(image_path));
        init_history_store_dir(history.clone(), start_history_dir_observe);
    }
}

/// Starts directory observation for the history directory.
///
/// Spawns a new thread to monitor the history directory for changes using the system's
/// directory monitoring capabilities. The directory monitor is set up to rebuild the
/// directory structure when changes are detected.
///
/// # Parameters
/// - `curr`: Current directory path to monitor
/// - `history`: History directory manager for handling directory changes
///
/// # Safety
/// Spawns a background thread that runs indefinitely until the program terminates
/// or the monitor is stopped externally.
pub fn start_history_dir_observe(curr: PathBuf, history: Arc<HistoryDir>) {
    // Use ffrt_spawn for thread creation to ensure proper resource management
    ffrt_rs::ffrt_spawn(move || {
        // Only proceed if a valid image directory path is available
        if let Some(image_dir) = history.dir_path() {
            let_cxx_string!(image_dir = image_dir);
            // Create directory rebuilder to handle directory structure changes
            let rebuilder = Box::new(DirRebuilder::new(curr, history));
            let mut monitor = NewDirectoryMonitor(&image_dir, rebuilder);
            // Start observation if the monitor was successfully created
            if let Some(ptr) = monitor.as_mut() {
                StartObserve(ptr);
            }
        }
    });
}
