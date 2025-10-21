// Copyright (C) 2023 Huawei Device Co., Ltd.
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

//! Utility module providing common functionality for request handling.
//! 
//! This module includes various helper functions and types used throughout the
//! request processing system, including time utilities, string handling,
//! memory management, and FFI bridges to C++ components.

pub(crate) mod c_wrapper;
pub(crate) mod common_event;
pub(crate) mod form_item;
use std::collections::HashMap;
use std::future::Future;
use std::io::Write;
use std::sync::Once;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub(crate) use common_event::{
    subscribe_common_event, CommonEventSubscriber, Want as CommonEventWant,
};
pub(crate) use ffi::PublishStateChangeEvent;

cfg_oh! {
    pub(crate) mod url_policy;
    #[cfg(not(test))]
    pub(crate) use ffi::GetForegroundAbilities;
}

pub(crate) mod task_event_count;
pub(crate) mod task_id_generator;
use ylong_runtime::sync::oneshot::Receiver;
use ylong_runtime::task::JoinHandle;

/// A wrapper around a oneshot receiver that provides a blocking API.
///
/// This struct provides a simple interface to wait for and retrieve a value
/// from a oneshot channel, blocking the current thread until the value is ready.
pub(crate) struct Recv<T> {
    /// The inner oneshot receiver.
    rx: Receiver<T>,
}

impl<T> Recv<T> {
    /// Creates a new `Recv` wrapper around the given receiver.
    pub(crate) fn new(rx: Receiver<T>) -> Self {
        Self { rx }
    }

    /// Retrieves the value from the oneshot channel, blocking the current thread.
    ///
    /// # Returns
    ///
    /// Returns `Some(value)` if the value was successfully received, or `None`
    /// if the sender was dropped before sending a value.
    ///
    /// # Notes
    ///
    /// This implementation assumes the receiver will never be hung up in the
    /// expected usage pattern.
    pub(crate) fn get(self) -> Option<T> {
        // Here `self.rx` can never be hung up in the expected usage context
        ylong_runtime::block_on(self.rx).ok()
    }
}

/// Safely constructs a vector from a raw pointer and length, applying a conversion function.
///
/// This function provides a safe way to convert C-style arrays (represented by a
/// pointer and length) into Rust vectors, with an optional conversion step for each element.
///
/// # Safety
///
/// The caller must ensure:
/// - If `ptr` is not null, it must point to a valid memory region containing at least `len`
///   consecutive elements of type `A`.
/// - The memory pointed to by `ptr` must not be mutated during the execution of this function.
/// - The memory must remain valid until the function completes.
///
/// # Parameters
///
/// - `ptr`: Pointer to the start of the array
/// - `len`: Number of elements in the array
/// - `func`: Conversion function to transform each element from type `A` to type `B`
///
/// # Returns
///
/// Returns a new `Vec<B>` containing all elements converted from the input array.
/// Returns an empty vector if `ptr` is null or `len` is 0.
///
/// # Examples
///
/// ```rust
/// let data = [1, 2, 3, 4, 5];
/// let ptr = data.as_ptr();
/// let len = data.len();
/// 
/// // Convert to a vector of strings
/// let result = build_vec(ptr, len, |&x| x.to_string());
/// assert_eq!(result, vec!["1", "2", "3", "4", "5"]);
/// ```
pub(crate) fn build_vec<A, B, C>(ptr: *const A, len: usize, func: C) -> Vec<B>
where
    C: Fn(&A) -> B,
{
    if ptr.is_null() || len == 0 {
        return Vec::<B>::new();
    }
    // Safety: Assuming the caller has ensured the pointer is valid for `len` elements
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    slice.iter().map(func).collect()
}

/// Retrieves the current system time as a timestamp in milliseconds since UNIX EPOCH.
///
/// # Returns
///
/// Returns the number of milliseconds since January 1, 1970 UTC.
///
/// # Panics
///
/// Panics if the system time is set before the UNIX EPOCH (January 1, 1970).
///
/// # Examples
///
/// ```rust
/// let timestamp = get_current_timestamp();
/// assert!(timestamp > 0);
/// ```
pub(crate) fn get_current_timestamp() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() as u64,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

/// Retrieves the current system time as a `Duration` since UNIX EPOCH.
///
/// # Returns
///
/// Returns a `Duration` representing the time elapsed since January 1, 1970 UTC.
///
/// # Panics
///
/// Panics if the system time is set before the UNIX EPOCH (January 1, 1970).
///
/// # Examples
///
/// ```rust
/// let duration = get_current_duration();
/// assert!(duration.as_secs() > 0);
/// ```
pub(crate) fn get_current_duration() -> Duration {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(dur) => dur,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

/// Converts a HashMap<String, String> to a tab-delimited string representation.
///
/// This function serializes a hash map into a string where each key-value pair
/// is represented as "key\tvalue", with pairs separated by "\r\n".
///
/// # Parameters
///
/// - `map`: The hash map to convert
///
/// # Returns
///
/// Returns a string representation of the hash map.
///
/// # Safety
///
/// This function assumes that all written data is valid UTF-8, which is guaranteed
/// since we're only writing String values.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
/// 
/// let mut map = HashMap::new();
/// map.insert("key1".to_string(), "value1".to_string());
/// map.insert("key2".to_string(), "value2".to_string());
/// 
/// let result = hashmap_to_string(&map);
/// // The result will be either "key1\tvalue1\r\nkey2\tvalue2" or
/// // "key2\tvalue2\r\nkey1\tvalue1" (order is not guaranteed for HashMap)
/// ```
pub(crate) fn hashmap_to_string(map: &HashMap<String, String>) -> String {
    let mut res = Vec::new();
    for (n, (k, v)) in map.iter().enumerate() {
        if n != 0 {
            // Add line separator between entries except for the first one
            let _ = write!(res, "\r\n");
        }
        let _ = write!(res, "{k}\t{v}");
    }
    // Safety: We're only writing valid UTF-8 strings, so this is safe
    unsafe { String::from_utf8_unchecked(res) }
}

/// Parses a tab-delimited string into a HashMap<String, String>.
///
/// This function deserializes a string where each key-value pair is represented as
/// "key\tvalue", with pairs separated by "\r\n", into a hash map.
///
/// # Parameters
///
/// - `str`: The string to parse, with format "key1\tvalue1\r\nkey2\tvalue2"
///
/// # Returns
///
/// Returns a `HashMap<String, String>` containing the parsed key-value pairs.
/// Returns an empty map if the input string is empty.
///
/// # Panics
///
/// Panics if any line in the input string does not contain exactly one tab character.
///
/// # Examples
///
/// ```rust
/// let input = "key1\tvalue1\r\nkey2\tvalue2";
/// let result = string_to_hashmap(&mut input.to_string());
/// 
/// assert_eq!(result.get("key1"), Some(&"value1".to_string()));
/// assert_eq!(result.get("key2"), Some(&"value2".to_string()));
/// ```
pub(crate) fn string_to_hashmap(str: &mut str) -> HashMap<String, String> {
    let mut map = HashMap::<String, String>::new();
    if str.is_empty() {
        return map;
    }
    for item in str.split("\r\n") {
        // Panics if the item doesn't contain exactly one tab character
        let (k, v) = item.split_once('\t').unwrap();
        map.insert(k.into(), v.into());
    }
    map
}

/// Splits a string by removing surrounding brackets and then splitting by ", ".
///
/// This function processes a string by first trimming any leading and trailing
/// '[' and ']' characters, then splitting the remaining content by ", ".
///
/// # Parameters
///
/// - `str`: The string to process, typically in format "[item1, item2, item3]"
///
/// # Returns
///
/// Returns an iterator over the split strings.
///
/// # Examples
///
/// ```rust
/// let input = "[apple, banana, cherry]";
/// let result: Vec<_> = split_string(&mut input.to_string()).collect();
/// 
/// assert_eq!(result, vec!["apple", "banana", "cherry"]);
/// ```
pub(crate) fn split_string(str: &mut str) -> std::str::Split<'_, &str> {
    let pat: &[_] = &['[', ']'];
    // Trim surrounding brackets and split by ", " delimiter
    str.trim_matches(pat).split(", ")
}

/// Calls the given closure exactly once, ensuring thread safety.
///
/// This function is a wrapper around `std::sync::Once::call_once` that
/// boxes the closure to erase its type.
///
/// # Parameters
///
/// - `once`: A reference to a `Once` synchronization primitive
/// - `func`: The closure to call exactly once
///
/// # Examples
///
/// ```rust
/// use std::sync::Once;
/// 
/// static INIT: Once = Once::new();
/// let mut initialized = false;
/// 
/// call_once(&INIT, || {
///     initialized = true;
/// });
/// 
/// assert!(initialized);
/// ```
pub(crate) fn call_once<F: FnOnce()>(once: &Once, func: F) {
    once.call_once(Box::new(func) as Box<dyn FnOnce()>)
}

/// Spawns a future on the ylong runtime, returning a join handle.
///
/// This function boxes and pins the provided future before spawning it,
/// allowing for dynamic dispatch of the future.
///
/// # Parameters
///
/// - `fut`: The future to spawn
///
/// # Returns
///
/// Returns a `JoinHandle<()>` that can be used to await the completion
/// of the spawned future or cancel it.
///
/// # Examples
///
/// ```rust
/// async fn example_task() {
///     // Task implementation
/// }
/// 
/// let handle = runtime_spawn(example_task());
/// // Later, we can await the task
/// // ylong_runtime::block_on(handle);
/// ```
pub(crate) fn runtime_spawn<F: Future<Output = ()> + Send + Sync + 'static>(
    fut: F,
) -> JoinHandle<()> {
    ylong_runtime::spawn(Box::into_pin(
        // Box the future for dynamic dispatch
        Box::new(fut) as Box<dyn Future<Output = ()> + Send + Sync>
    ))
}

/// Queries the bundle name of the calling process using its token ID.
///
/// This function retrieves the bundle name associated with the calling process's
/// token ID through the FFI bridge to C++ code.
///
/// # Returns
///
/// Returns the bundle name as a `String`.
///
/// # Availability
///
/// This function is only available when the `oh` feature is enabled.
#[cfg(feature = "oh")]
pub(crate) fn query_calling_bundle() -> String {
    let token_id = ipc::Skeleton::calling_full_token_id();
    ffi::GetCallingBundle(token_id)
}

/// Determines if the calling process is using a system API based on its token ID.
///
/// This function checks whether the calling process has system API privileges
/// by verifying its token ID through the FFI bridge to C++ code.
///
/// # Returns
///
/// Returns `true` if the calling process has system API privileges, otherwise `false`.
///
/// # Availability
///
/// This function is only available when the `oh` feature is enabled.
#[cfg(feature = "oh")]
pub(crate) fn is_system_api() -> bool {
    let token_id = ipc::Skeleton::calling_full_token_id();
    ffi::IsSystemAPI(token_id)
}

/// Checks if the calling process has a specific permission.
///
/// This function verifies whether the calling process has been granted a specific
/// permission by checking its token ID against the permission system through the
/// FFI bridge to C++ code.
///
/// # Parameters
///
/// - `permission`: The name of the permission to check
///
/// # Returns
///
/// Returns `true` if the calling process has the specified permission,
/// otherwise `false`.
///
/// # Availability
///
/// This function is only available when the `oh` feature is enabled.
#[cfg(feature = "oh")]
pub(crate) fn check_permission(permission: &str) -> bool {
    let token_id = ipc::Skeleton::calling_full_token_id();
    ffi::CheckPermission(token_id, permission)
}

/// Updates the system policy based on whether any tasks are active.
///
/// This function notifies the system policy manager about the status of tasks
/// through the FFI bridge to C++ code.
///
/// # Parameters
///
/// - `any_tasks`: Boolean indicating whether any tasks are currently active
///
/// # Returns
///
/// Returns an integer status code from the underlying C++ implementation.
///
/// # Availability
///
/// This function is only available when the `oh` feature is enabled.
#[cfg(feature = "oh")]
pub(crate) fn update_policy(any_tasks: bool) -> i32 {
    ffi::UpdatePolicy(any_tasks)
}

/// Determines if the calling process is a HarmonyOS Ability Package (HAP).
///
/// This function checks whether the calling process is a HarmonyOS Ability Package
/// by examining its token ID through the FFI bridge to C++ code.
///
/// # Returns
///
/// Returns `true` if the calling process is a HAP, otherwise `false`.
///
/// # Availability
///
/// This function is only available when the `oh` feature is enabled.
#[cfg(feature = "oh")]
pub(crate) fn is_called_by_hap() -> bool {
    let token_id = ipc::Skeleton::calling_token_id();
    ffi::IsCalledByHAP(token_id)
}

/// CXX FFI bridge to C++ utilities.
///
/// This module defines the interface to C++ utility functions used throughout
/// the request system.
///
/// # Safety
///
/// All functions in this module are marked as `unsafe` because they interface
/// with C++ code and may have additional safety requirements not enforced by Rust.
#[allow(unused)]
#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {

    unsafe extern "C++" {
        include!("request_utils.h");

        /// Publishes a state change event for a task.
        fn PublishStateChangeEvent(bundleName: &str, taskId: u32, state: i32, uid: i32) -> bool;
        
        /// Retrieves the list of foreground abilities for a given UID.
        fn GetForegroundAbilities(uid: &mut Vec<i32>) -> i32;
        
        /// Gets the bundle name associated with a token ID.
        fn GetCallingBundle(token_id: u64) -> String;
        
        /// Checks if a token ID has system API privileges.
        fn IsSystemAPI(token_id: u64) -> bool;
        
        /// Checks if a token ID has a specific permission.
        fn CheckPermission(token_id: u64, permission: &str) -> bool;
        
        /// Updates system policy based on task status.
        fn UpdatePolicy(any_tasks: bool) -> i32;
        
        /// Checks if a token ID belongs to a HarmonyOS Ability Package.
        fn IsCalledByHAP(token_id: u32) -> bool;
    }
}

/// Unit test module included conditionally when `oh` feature and tests are enabled.
///
/// This module includes external test code when running in test mode with the
/// `oh` feature enabled.
#[cfg(feature = "oh")]
#[cfg(test)]
mod ut_mod {
    include!("../../tests/ut/utils/ut_mod.rs");
}
