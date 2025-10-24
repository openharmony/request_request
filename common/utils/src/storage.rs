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

//! Storage and file access control utilities.
//! 
//! This module provides functions for managing file access control lists (ACLs)
//! on the file system, allowing for fine-grained permission management for files.

use cxx::let_cxx_string;

use crate::wrapper;

/// Sets access control entries for a target file.
///
/// Configures the access control list (ACL) for the specified file using the
/// provided ACL entry string. ACLs provide more fine-grained access control
/// than standard file permissions.
///
/// # Parameters
///
/// * `target_file` - Path to the file for which to set access controls
/// * `entry_txt` - String representation of ACL entries to apply
///
/// # Returns
///
/// Returns `Ok(())` on success, or `Err(i32)` with an error code on failure.
///
/// # Errors
///
/// Returns non-zero error codes from the underlying `wrapper::AclSetAccess` function
/// when the operation fails. Specific error codes depend on the platform implementation.
///
/// # Examples
///
/// ```rust
/// use request_utils::storage::acl_set_access;
///
/// fn configure_file_access() -> Result<(), i32> {
///     let file_path = "/path/to/sensitive_file.txt";
///     let acl_entry = "user::rwx,user:admin:rwx,group::rx,other::-";
///     
///     acl_set_access(file_path, acl_entry)?;
///     println!("Access controls set successfully");
///     Ok(())
/// }
/// ```
pub fn acl_set_access(target_file: &str, entry_txt: &str) -> Result<(), i32> {
    // Convert Rust strings to C++ strings for FFI call
    let_cxx_string!(target_file = target_file);
    let_cxx_string!(entry_txt = entry_txt);
    let res = wrapper::AclSetAccess(&target_file, &entry_txt);
    if res != 0 {
        // Return the error code from the underlying implementation
        return Err(res);
    }
    Ok(())
}

/// Sets default access control entries for a target file.
///
/// Configures the default access control list (ACL) for the specified file using
/// the provided ACL entry string. Default ACLs are applied to new files created
/// within a directory.
///
/// # Parameters
///
/// * `target_file` - Path to the file (typically a directory) for which to set default access controls
/// * `entry_txt` - String representation of default ACL entries to apply
///
/// # Returns
///
/// Returns `Ok(())` on success, or `Err(i32)` with an error code on failure.
///
/// # Errors
///
/// Returns non-zero error codes from the underlying `wrapper::AclSetDefault` function
/// when the operation fails. Specific error codes depend on the platform implementation.
///
/// # Examples
///
/// ```rust
/// use request_utils::storage::acl_set_default;
///
/// fn configure_directory_defaults() -> Result<(), i32> {
///     let dir_path = "/path/to/shared_directory";
///     let default_acl = "user::rwx,group::rwx,other::rx";
///     
///     acl_set_default(dir_path, default_acl)?;
///     println!("Default access controls set successfully");
///     Ok(())
/// }
/// ```
pub fn acl_set_default(target_file: &str, entry_txt: &str) -> Result<(), i32> {
    // Convert Rust strings to C++ strings for FFI call
    let_cxx_string!(target_file = target_file);
    let_cxx_string!(entry_txt = entry_txt);
    let res = wrapper::AclSetDefault(&target_file, &entry_txt);
    if res != 0 {
        // Return the error code from the underlying implementation
        return Err(res);
    }
    Ok(())
}
