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

use std::fmt::Display;

use cxx::UniquePtr;
use ffi::WantWrapper;
/// Internal handler for converting between C++ and Rust event callbacks.
///
/// Wraps a `CommonEventSubscriber` trait object to handle event reception from C++.
pub struct EventHandler {
    /// The wrapped event subscriber implementation.
    inner: Box<dyn CommonEventSubscriber>,
}

impl EventHandler {
    /// Creates a new event handler wrapping the provided subscriber.
    ///
    /// # Parameters
    /// - `inner`: Boxed `CommonEventSubscriber` implementation to handle events.
    ///
    /// # Returns
    /// A new `EventHandler` instance.
    #[inline]
    fn new(inner: Box<dyn CommonEventSubscriber>) -> Self {
        Self { inner }
    }
}

/// Trait for handling common events received from the system.
///
/// Implement this trait to receive and process common events when subscribed.
pub trait CommonEventSubscriber {
    /// Called when a subscribed event is received.
    ///
    /// # Parameters
    /// - `code`: Event code indicating the event type.
    /// - `data`: Additional string data associated with the event.
    /// - `want`: Container for event parameters and extra data.
    fn on_receive_event(&self, code: i32, data: String, want: Want);
}

impl EventHandler {
    /// Handles events received from C++ by converting to Rust types and delegating.
    ///
    /// # Parameters
    /// - `code`: Event code from C++.
    /// - `data`: Event data from C++.
    /// - `want`: C++ `WantWrapper` to be wrapped in a Rust `Want`.
    #[inline]
    fn on_receive_event(&self, code: i32, data: String, want: UniquePtr<WantWrapper>) {
        // Convert C++ WantWrapper to Rust Want before passing to subscriber
        self.inner.on_receive_event(code, data, Want::new(want));
    }
}

/// Wrapper around C++ Want object for accessing event parameters.
///
/// Provides safe access to event data and parameters from C++.
pub struct Want {
    /// The underlying C++ WantWrapper.
    inner: UniquePtr<ffi::WantWrapper>,
}

impl Want {
    /// Creates a new Rust Want wrapper from a C++ WantWrapper.
    ///
    /// # Parameters
    /// - `inner`: Unique pointer to C++ WantWrapper to wrap.
    ///
    /// # Returns
    /// A new `Want` instance.
    #[inline]
    fn new(inner: UniquePtr<WantWrapper>) -> Self {
        Self { inner }
    }

    /// Retrieves an integer parameter from the event.
    ///
    /// # Parameters
    /// - `key`: The parameter name to retrieve.
    ///
    /// # Returns
    /// The integer value if found, or `None` if not found (indicated by -1).
    ///
    /// # Note
    /// This assumes that -1 is not a valid value for parameters. If -1 could be
    /// a valid parameter value, this function may incorrectly return `None`.
    pub(crate) fn get_int_param(&self, key: &str) -> Option<i32> {
        let res = self.inner.GetIntParam(key);
        // -1 is used as a sentinel value indicating the parameter was not found
        if res == -1 {
            None
        } else {
            Some(res)
        }
    }
}

// Parameter value types available in Want objects
// VALUE_TYPE_BOOLEAN = 1,
// VALUE_TYPE_BYTE = 2,
// VALUE_TYPE_CHAR = 3,
// VALUE_TYPE_SHORT = 4,
// VALUE_TYPE_INT = 5,
// VALUE_TYPE_LONG = 6,
// VALUE_TYPE_FLOAT = 7,
// VALUE_TYPE_DOUBLE = 8,
// VALUE_TYPE_STRING = 9,
// VALUE_TYPE_ARRAY = 102,
impl Display for Want {
    /// Formats the Want object as a string.
    ///
    /// # Returns
    /// The formatted string representation of the Want object.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.ToString())
    }
}

/// Subscribes to system common events.
///
/// Registers a handler to receive notifications when specific events occur.
///
/// # Parameters
/// - `events`: List of event names to subscribe to.
/// - `handler`: Event handler implementing `CommonEventSubscriber`.
///
/// # Returns
/// - `Ok(())` on successful subscription.
/// - `Err(i32)` with an error code on failure.
///
/// # Examples
/// ```
/// use request_services::utils::common_event::{CommonEventSubscriber, Want, subscribe_common_event};
/// 
/// struct MyEventHandler;
/// 
/// impl CommonEventSubscriber for MyEventHandler {
///     fn on_receive_event(&self, code: i32, data: String, want: Want) {
///         println!("Received event with code: {}, data: {}", code, data);
///         if let Some(value) = want.get_int_param("key") {
///             println!("Parameter value: {}", value);
///         }
///     }
/// }
/// 
/// // Subscribe to events
/// let result = subscribe_common_event(vec!["ohos.event.action.TEST"], MyEventHandler);
/// match result {
///     Ok(_) => println!("Successfully subscribed to events"),
///     Err(code) => println!("Failed to subscribe, error code: {}", code),
/// }
/// ```
pub fn subscribe_common_event<T: CommonEventSubscriber + 'static>(
    events: Vec<&str>,
    handler: T,
) -> Result<(), i32> {
    // Create a new EventHandler wrapping the provided subscriber
    // and register it with the C++ common event system
    let res = ffi::SubscribeCommonEvent(events, Box::new(EventHandler::new(Box::new(handler))));
    if res == 0 {
        Ok(())
    } else {
        Err(res)
    }
}

/// FFI bridge for C++ common event interactions.
///
/// Defines the interface between Rust and C++ for common event operations.
#[allow(unused)]
#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {
    extern "Rust" {
        /// Type representing a Rust event handler.
        type EventHandler;
        /// Method called by C++ when an event is received.
        fn on_receive_event(&self, code: i32, data: String, want: UniquePtr<WantWrapper>);
    }
    
    unsafe extern "C++" {
        /// Include necessary C++ headers.
        include!("common_event.h");
        include!("common_event_data.h");
        
        /// C++ wrapper for Want objects.
        type WantWrapper;

        /// Converts a WantWrapper to a string representation.
        fn ToString(self: &WantWrapper) -> String;
        
        /// Retrieves an integer parameter from a WantWrapper.
        fn GetIntParam(self: &WantWrapper, key: &str) -> i32;

        /// Subscribes to common events using C++ implementation.
        fn SubscribeCommonEvent(events: Vec<&str>, handler: Box<EventHandler>) -> i32;
    }
}
