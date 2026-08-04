// Copyright (C) 2026 Huawei Device Co., Ltd.
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

//! Callback management for cache download operations.
//!
//! This module defines structures and methods to manage success and error
//! callbacks associated with download URLs.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use ani_rs::objects::GlobalRefCallback;
use preload_native_rlib::info::RustDownloadInfo;
use preload_native_rlib::{CacheDownloadError, PreloadCallback, RamCache};

use crate::bridge::{DownloadError, ErrorCode};

pub struct CallbackUnit {
    error_callbacks: Mutex<Vec<GlobalRefCallback<(DownloadError,)>>>,
    success_callbacks: Mutex<Vec<GlobalRefCallback<()>>>,
}

impl CallbackUnit {
    pub fn new() -> Self {
        Self {
            error_callbacks: Mutex::new(Vec::new()),
            success_callbacks: Mutex::new(Vec::new()),
        }
    }

    pub fn add_success_callback(&self, callback: GlobalRefCallback<()>) {
        let mut cbs = self.success_callbacks.lock().unwrap();
        for cb in cbs.iter() {
            if callback.eq(cb) {
                return;
            }
        }
        cbs.push(callback);
    }

    pub fn add_error_callback(&self, callback: GlobalRefCallback<(DownloadError,)>) {
        let mut cbs = self.error_callbacks.lock().unwrap();
        for cb in cbs.iter() {
            if callback.eq(cb) {
                return;
            }
        }
        cbs.push(callback);
    }

    pub fn remove_success_callback(&self, callback: Option<GlobalRefCallback<()>>) {
        let mut cbs = self.success_callbacks.lock().unwrap();
        if let Some(callback) = callback {
            cbs.retain(|cb| !cb.eq(&callback));
        } else {
            cbs.clear();
        }
    }

    pub fn remove_error_callback(&self, callback: Option<GlobalRefCallback<(DownloadError,)>>) {
        let mut cbs = self.error_callbacks.lock().unwrap();
        if let Some(callback) = callback {
            cbs.retain(|cb| !cb.eq(&callback));
        } else {
            cbs.clear();
        }
    }

    pub fn call_success_callbacks(&self) {
        let cbs = self.success_callbacks.lock().unwrap().clone();
        for cb in cbs.iter() {
            cb.execute(());
        }
    }

    pub fn call_error_callbacks(&self, error: DownloadError) {
        let cbs = self.error_callbacks.lock().unwrap().clone();
        for cb in cbs.iter() {
            cb.execute((error.clone(),));
        }
    }
}

pub struct CallbackWrapper {
    url: String,
}

impl CallbackWrapper {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

impl PreloadCallback for CallbackWrapper {
    fn on_success(&mut self, _: Arc<RamCache>, _: &str) {
        CallbackManager::get_instance().call_success_callbacks(self.url.as_str());
    }

    fn on_fail(&mut self, error: CacheDownloadError, _: RustDownloadInfo, _: &str) {
        let error = DownloadError::from_native(error);
        CallbackManager::get_instance().call_error_callbacks(self.url.as_str(), error);
    }
}

pub struct CallbackManager {
    callbacks: Mutex<HashMap<String, Arc<CallbackUnit>>>,
}

impl CallbackManager {
    pub fn new() -> Self {
        Self {
            callbacks: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_instance() -> &'static CallbackManager {
        static INSTANCE: OnceLock<CallbackManager> = OnceLock::new();
        INSTANCE.get_or_init(|| CallbackManager::new())
    }

    pub fn call_success_callbacks(&self, url: &str) {
        let callbacks = self.callbacks.lock().unwrap();
        if let Some(unit) = callbacks.get(url) {
            let unit = unit.clone();
            drop(callbacks);
            unit.call_success_callbacks();
        }
    }

    pub fn call_error_callbacks(&self, url: &str, error: DownloadError) {
        let callbacks = self.callbacks.lock().unwrap();
        if let Some(unit) = callbacks.get(url) {
            let unit = unit.clone();
            drop(callbacks);
            unit.call_error_callbacks(error);
        }
    }

    pub fn register_success_callback(&self, url: &str, callback: GlobalRefCallback<()>) {
        let mut callbacks = self.callbacks.lock().unwrap();
        let unit = callbacks
            .entry(url.to_string())
            .or_insert_with(|| Arc::new(CallbackUnit::new()))
            .clone();
        drop(callbacks);
        unit.add_success_callback(callback);
    }

    pub fn register_error_callback(
        &self,
        url: &str,
        callback: GlobalRefCallback<(DownloadError,)>,
    ) {
        let mut callbacks = self.callbacks.lock().unwrap();
        let unit = callbacks
            .entry(url.to_string())
            .or_insert_with(|| Arc::new(CallbackUnit::new()))
            .clone();
        drop(callbacks);
        unit.add_error_callback(callback);
    }

    pub fn unregister_success_callback(&self, url: &str, callback: Option<GlobalRefCallback<()>>) {
        let mut callbacks = self.callbacks.lock().unwrap();
        if let Some(unit) = callbacks.get_mut(url) {
            let unit = unit.clone();
            drop(callbacks);
            // Remove the specific callback from the unit's list
            unit.remove_success_callback(callback);
        }
    }

    pub fn unregister_error_callback(
        &self,
        url: &str,
        callback: Option<GlobalRefCallback<(DownloadError,)>>,
    ) {
        let mut callbacks = self.callbacks.lock().unwrap();
        if let Some(unit) = callbacks.get_mut(url) {
            let unit = unit.clone();
            drop(callbacks);
            // Remove the specific callback from the unit's list
            unit.remove_error_callback(callback);
        }
    }
}
