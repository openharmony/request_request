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

//! Directory observation and management for cache system.
//! 
//! This module provides functionality for monitoring and managing cache directories,
//! including cleanup operations and observation control.

use std::{fs, path::PathBuf, sync::Arc};

use crate::data::file::HistoryDir;

/// Manages directory observation and cleanup operations for cache directories.
///
/// This struct provides methods to remove store directories and control
/// history directory observation. It implements the Drop trait to automatically
/// stop observation when dropped.
pub struct DirRebuilder {
    /// Path to the current cache directory
    curr: PathBuf,
    /// Reference to the history directory being observed
    history: Arc<HistoryDir>,
}

impl DirRebuilder {
    /// Creates a new DirRebuilder with the specified directories.
    ///
    /// # Parameters
    /// - `curr`: Path to the current cache directory
    /// - `history`: Reference to the history directory to observe
    pub fn new(curr: PathBuf, history: Arc<HistoryDir>) -> Self {
        Self { curr, history }
    }

    /// Removes the store directory if it exists.
    ///
    /// Silently continues if deletion fails, only logging an error.
    pub fn remove_store_dir(&self) {
        if self.curr.is_dir() {
            // Don't care about the failed deletion - continue even if deletion fails
            if let Err(e) = fs::remove_dir_all(self.curr.as_path()) {
                error!("remove local store directory fail, err: {:?}", e);
            };
        }
    }

    /// Stops observation of the history directory.
    ///
    /// Calls the stop_observe method on the history directory.
    pub fn stop_history_observe(&self) {
        self.history.stop_observe();
    }
}

impl Drop for DirRebuilder {
    /// Stops history directory observation when dropped.
    ///
    /// Ensures that directory observation is properly stopped when the DirRebuilder
    /// instance goes out of scope, preventing resource leaks.
    fn drop(&mut self) {
        self.stop_history_observe();
    }
}
