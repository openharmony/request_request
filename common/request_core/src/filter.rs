// Copyright (c) 2023 Huawei Device Co., Ltd.
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

//! Search filtering for network request tasks.
//! 
//! This module provides structures for filtering network tasks based on various criteria,
//! enabling targeted search and management operations within the request system.

use crate::config::{Action, Mode};
use crate::info::State;

/// Filter criteria for searching network tasks.
///
/// A set of optional filtering parameters used to narrow down task searches based on
/// bundle ownership, time ranges, state, action type, and mode.
///
/// # Examples
///
/// ```rust
/// use request_core::{config::Action, filter::SearchFilter, info::State};
/// 
/// // Create a filter to find all download tasks that completed successfully
/// let mut filter = SearchFilter::new();
/// filter.action = Some(Action::Download);
/// filter.state = Some(State::Success);
///
/// // Create a filter for recent uploads from a specific bundle
/// let mut recent_uploads_filter = SearchFilter::new();
/// recent_uploads_filter.bundle_name = Some("com.example.app".to_string());
/// recent_uploads_filter.action = Some(Action::Upload);
/// recent_uploads_filter.after = Some(1628092800); // Unix timestamp for a specific date
/// ```
pub struct SearchFilter {
    /// The bundle name of the task owner.
    pub bundle_name: Option<String>,
    /// Tasks created before this timestamp (exclusive).
    pub before: Option<i64>,
    /// Tasks created after this timestamp (inclusive).
    pub after: Option<i64>,
    /// Current state of the task.
    pub state: Option<State>,
    /// Type of action performed by the task.
    pub action: Option<Action>,
    /// Operating mode of the task.
    pub mode: Option<Mode>,
}

impl SearchFilter {
    /// Creates a new empty `SearchFilter` with all criteria set to `None`.
    ///
    /// # Notes
    ///
    /// When all fields are `None`, the filter will not restrict the search results.
    /// Callers should set the desired fields to filter the search appropriately.
    pub fn new() -> Self {
        SearchFilter {
            bundle_name: None,
            before: None,
            after: None,
            state: None,
            action: None,
            mode: None,
        }
    }
}
