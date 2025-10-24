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

//! CXX bridge for Rust-C++ interoperability in directory monitoring.
//! 
//! This module defines the CXX bridge that enables communication between Rust code
//! and C++ components for directory monitoring functionality. It exposes Rust types
//! to C++ and makes C++ functionality accessible to Rust.

use crate::data::observer::DirRebuilder;

// CXX bridge defining the FFI interface between Rust and C++
#[cxx::bridge(namespace = "OHOS::Request")]
pub(crate) mod ffi {
    // Rust types and functions exposed to C++
    extern "Rust" {
        /// Directory rebuilder type for handling directory reconstruction events.
        ///
        /// Used by C++ to interact with the Rust implementation of directory rebuilding
        /// functionality when directory changes are detected.
        type DirRebuilder;

        /// Removes the store directory managed by this rebuilder.
        ///
        /// Cleans up the associated directory resources when called from C++ code.
        fn remove_store_dir(self: &DirRebuilder);
    }

    // C++ types and functions exposed to Rust
    unsafe extern "C++" {
        include!("inotify_event_listener.h");
        include!("native_ffi.h");
        
        /// C++ directory monitor type for observing file system events.
        ///
        /// Provides native directory monitoring capabilities through C++ implementation.
        type DirectoryMonitor;

        /// Creates a new directory monitor with the specified target and callback.
        ///
        /// # Safety
        ///
        /// This function is marked unsafe because it interfaces with C++ code and
        /// manages raw pointers internally. Proper lifetime management of the callback
        /// is required.
        ///
        /// # Parameters
        /// - `target`: Path to the directory to monitor
        /// - `callback`: Rebuilder instance to handle directory events
        ///
        /// # Returns
        /// A unique pointer to the created DirectoryMonitor instance
        fn NewDirectoryMonitor(
            target: &CxxString,
            callback: Box<DirRebuilder>,
        ) -> UniquePtr<DirectoryMonitor>;
        
        /// Starts monitoring the directory for changes.
        ///
        /// # Safety
        ///
        /// This function is marked unsafe because it calls into C++ code that may
        /// have side effects or thread safety considerations.
        ///
        /// # Parameters
        /// - `monitor`: Pin reference to the directory monitor instance
        fn StartObserve(monitor: Pin<&mut DirectoryMonitor>);
    }
}
