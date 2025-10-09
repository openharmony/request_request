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

//! Request Download Server Implementation.
//!
//! This system service is used to assist applications in handling network tasks
//! such as uploading and downloading.

#![cfg_attr(test, feature(future_join))]
#![cfg_attr(test, allow(clippy::redundant_clone))]
#![allow(
    unreachable_pub,
    clippy::new_without_default,
    unknown_lints,
    stable_features
)]
#![warn(
    missing_docs,
    clippy::redundant_static_lifetimes,
    clippy::enum_variant_names,
    clippy::clone_on_copy,
    clippy::unused_async
)]
#![feature(lazy_cell)]

#[macro_use]
mod macros;

#[macro_use]
extern crate request_utils;

cfg_oh! {
    mod trace;
    pub mod ability;
    mod sys_event;
    pub use service::interface;
    pub use utils::form_item::FileSpec;
}

mod database;
mod error;
mod manage;
mod service;
mod task;
mod utils;
pub use task::{config, info};

use hilog_rust::{HiLogLabel, LogType};

pub(crate) const LOG_LABEL: HiLogLabel = HiLogLabel {
    log_type: LogType::LogCore,
    domain: 0xD001C50,
    tag: "RequestService",
};

#[cfg(feature = "oh")]
#[cfg(test)]
mod tests {
    use super::manage::database::RequestDb;
    use super::manage::SystemConfigManager;
    use crate::ability::SYSTEM_CONFIG_MANAGER;
    /// test init
    pub(crate) fn test_init() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
            unsafe { SYSTEM_CONFIG_MANAGER.write(SystemConfigManager::init()) };
        });

        let _ = std::fs::create_dir("test_files/");

        unsafe { SetAccessTokenPermission() };
    }

    pub(crate) fn lock_database<'a>() -> DatabaseLock<'a> {
        let _inner = unsafe {
            match DB_LOCK.lock() {
                Ok(inner) => inner,
                Err(_) => {
                    if let Err(e) = RequestDb::get_instance().execute("DELETE FROM request_task") {
                        error!("lock delete failed: {}", e);
                    }
                    DB_LOCK = std::sync::Mutex::new(());
                    DB_LOCK.lock().unwrap()
                }
            }
        };
        DatabaseLock { _inner }
    }

    pub(crate) struct DatabaseLock<'a> {
        _inner: std::sync::MutexGuard<'a, ()>,
    }

    impl<'a> Drop for DatabaseLock<'a> {
        fn drop(&mut self) {
            if let Err(e) = RequestDb::get_instance().execute("DELETE FROM request_task") {
                error!("drop delete failed: {}", e);
            }
        }
    }

    static mut DB_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    extern "C" {
        fn SetAccessTokenPermission();
    }
}
