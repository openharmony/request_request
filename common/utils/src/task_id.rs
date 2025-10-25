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

//! Task identifier utilities.
//! 
//! This module provides types for uniquely identifying tasks within the request system,
//! with functionality for creating IDs from hash strings or URLs and displaying them
//! in full or abbreviated form.

use std::fmt::Display;

use crate::hash::url_hash;

/// A unique identifier for tasks.
///
/// Wraps a hash string that uniquely identifies a task. Provides methods for creating
/// IDs from hash strings or URLs, and for displaying them in full or abbreviated form.
///
/// # Examples
///
/// ```rust
/// use request_utils::task_id::TaskId;
///
/// // Create a TaskId from a hash string
/// let task_id = TaskId::new("abcdef1234567890abcdef1234567890".to_string());
///
/// // Create a TaskId by hashing a URL
/// let url_id = TaskId::from_url("https://example.com/api/data");
///
/// // Get abbreviated form (first 1/4 of the hash)
/// println!("Brief ID: {}", task_id.brief());
///
/// // Display full ID
/// println!("Full ID: {}", task_id);
/// ```
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct TaskId {
    /// The hash string that uniquely identifies the task.
    hash: String,
}

impl TaskId {
    /// Creates a new task ID from an existing hash string.
    ///
    /// # Parameters
    ///
    /// * `hash` - The hash string to use as the task identifier
    ///
    /// # Returns
    ///
    /// A new `TaskId` instance containing the provided hash.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use request_utils::task_id::TaskId;
    ///
    /// let hash_value = "deadbeef1234567890abcdef01234567".to_string();
    /// let task_id = TaskId::new(hash_value);
    /// assert_eq!(task_id.to_string(), "deadbeef1234567890abcdef01234567");
    /// ```
    pub fn new(hash: String) -> Self {
        Self { hash }
    }

    /// Creates a new task ID by hashing a URL.
    ///
    /// Uses the `url_hash` function to generate a hash from the provided URL string.
    ///
    /// # Parameters
    ///
    /// * `url` - The URL string to hash for task identification
    ///
    /// # Returns
    ///
    /// A new `TaskId` instance with a hash derived from the URL.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use request_utils::task_id::TaskId;
    ///
    /// let task_id = TaskId::from_url("https://example.com/download/file.zip");
    /// // The hash will be consistent for the same URL
    /// let same_id = TaskId::from_url("https://example.com/download/file.zip");
    /// assert_eq!(task_id, same_id);
    /// ```
    pub fn from_url(url: &str) -> Self {
        Self {
            hash: url_hash(url),
        }
    }

    /// Returns a shortened version of the task ID.
    ///
    /// Provides an abbreviated form of the hash, containing the first 1/4 of the characters.
    /// This can be useful for display purposes when the full hash would be too long.
    ///
    /// # Returns
    ///
    /// A string slice containing the first quarter of the hash.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use request_utils::task_id::TaskId;
    ///
    /// let task_id = TaskId::new("0123456789abcdef".to_string());
    /// // For a 16-character hash, brief() returns the first 4 characters
    /// assert_eq!(task_id.brief(), "0123");
    /// ```
    pub fn brief(&self) -> &str {
        let len = self.hash.len();
        // Return the first quarter of the hash for display purposes
        &self.hash.as_str()[..len / 4]
    }
}

impl Display for TaskId {
    /// Formats the task ID as a string.
    ///
    /// Displays the full hash string of the task ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use request_utils::task_id::TaskId;
    ///
    /// let task_id = TaskId::new("abc123def456".to_string());
    /// assert_eq!(format!("{}", task_id), "abc123def456");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hash)
    }
}
