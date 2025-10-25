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

//! File path control and validation utilities.
//! 
//! This module provides functions for validating file paths, checking if paths
//! belong to application base directories, and other file path-related utilities.

use std::path::Path;

// Common application base storage areas
static AREA1: &str = "/data/storage/el1/base";
static AREA2: &str = "/data/storage/el2/base";
static AREA5: &str = "/data/storage/el5/base";

/// Checks if a path exists in the filesystem.
///
/// # Arguments
///
/// * `path` - The path to check for existence
///
/// # Examples
///
/// ```rust
/// use request_utils::file_control::path_exists;
///
/// assert!(path_exists("/")); // Root directory should exist
/// assert!(!path_exists("/this/path/almost/certainly/does/not/exist"));
/// ```
pub fn path_exists<P: AsRef<Path>>(path: P) -> bool {
    Path::new(path.as_ref()).exists()
}

/// Determines whether a path belongs to any of the application base directories.
///
/// Returns `true` if the path starts with any of the known application base
/// directory prefixes (EL1, EL2, or EL5).
///
/// # Arguments
///
/// * `path` - The path to check
///
/// # Examples
///
/// ```rust
/// use request_utils::file_control::belong_app_base;
///
/// assert!(belong_app_base("/data/storage/el1/base/com.example.app"));
/// assert!(belong_app_base("/data/storage/el2/base/com.example.app"));
/// assert!(belong_app_base("/data/storage/el5/base/com.example.app"));
/// assert!(!belong_app_base("/system/app"));
/// ```
pub fn belong_app_base(path: &str) -> bool {
    path.starts_with(AREA1) || path.starts_with(AREA2) || path.starts_with(AREA5)
}

/// Validates that a path follows the standardized format.
///
/// A standardized path must:
/// - Not be empty
/// - Start with a forward slash (`/`)
/// - Not end with a forward slash
/// - Not contain double forward slashes (`//`)
/// - Not contain any relative path segments (`.` or `..`)
///
/// # Arguments
///
/// * `path` - The path to validate
///
/// # Examples
///
/// ```rust
/// use request_utils::file_control::check_standardized_path;
///
/// assert!(check_standardized_path("/valid/path"));
/// assert!(!check_standardized_path("")); // Empty path
/// assert!(!check_standardized_path("relative/path")); // No leading slash
/// assert!(!check_standardized_path("/path/with/trailing/")); // Trailing slash
/// assert!(!check_standardized_path("/path//with//double//slashes")); // Double slashes
/// assert!(!check_standardized_path("/path/with/../parent")); // Relative segments
/// ```
pub fn check_standardized_path(path: &str) -> bool {
    // Basic path validation checks
    if path.is_empty() || !path.starts_with('/') || path.ends_with('/') || path.contains("//") {
        return false;
    }
    
    // Patterns that indicate relative path traversal
    // These are not allowed for security reasons to prevent directory traversal attacks
    static NOT_ALLOWED: [&str; 11] = [
        r".", r".\", r"\.", r"..", r"\..", r"\.\.", r"\.\.\", r"\..\", r".\.", r".\.\", r"..\",
    ];
    
    // Check each path segment for forbidden patterns
    for segment in path.split('/').filter(|s| !s.is_empty()) {
        if NOT_ALLOWED.contains(&segment) {
            return false;
        }
    }
    true
}

/// Filters a list of strings to retain only those longer than the length of AREA1.
///
/// This is typically used to filter out paths that don't contain sufficient path
/// information beyond the application base directory.
///
/// # Arguments
///
/// * `v` - The list of strings to filter
///
/// # Examples
///
/// ```rust
/// use request_utils::file_control::delete_base_for_list;
///
/// let mut paths = vec![
///     "/data/storage/el1/base",
///     "/data/storage/el1/base/com.example.app",
///     "/short"
/// ];
/// delete_base_for_list(&mut paths);
/// 
/// assert_eq!(paths, vec!["/data/storage/el1/base/com.example.app"]);
/// ```
pub fn delete_base_for_list(v: &mut Vec<&str>) {
    v.retain(|s| s.len() > AREA1.len());
}

#[cfg(test)]
mod ut_file_control {
    include!("../tests/ut/ut_file_control.rs");
}
