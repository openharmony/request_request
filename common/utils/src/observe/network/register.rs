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

use std::sync::{Arc, Mutex};

use cxx::UniquePtr;
#[cfg(test)]
use ut_register::RegisterNetObserver;

use super::wrapper::ffi::NetUnregistration;
#[cfg(not(test))]
use super::wrapper::ffi::RegisterNetObserver;
use super::wrapper::NetObserverWrapper;
use super::Observer;

/// Manages registration of network observers with the native network service.
///
/// Holds the set of observers to be notified and the handle returned by the
/// native registration call, allowing the registration to be undone later.
pub struct NetRegistrar {
    observer: Arc<Mutex<Vec<Box<dyn Observer>>>>,
    unregistration: Mutex<Option<UniquePtr<NetUnregistration>>>,
}

impl NetRegistrar {
    /// Creates a new registrar with no observers and no active registration.
    pub fn new() -> Self {
        Self {
            observer: Arc::new(Mutex::new(Vec::new())),
            unregistration: Mutex::new(None),
        }
    }

    /// Adds an observer that will receive network change notifications once registered.
    pub fn add_observer(&self, observer: impl Observer + 'static) {
        self.observer.lock().unwrap().push(Box::new(observer));
    }

    /// Registers the collected observers with the native network service.
    ///
    /// Returns an error if a registration is already active or the native
    /// registration call fails.
    pub fn register(&self) -> Result<(), NetRegisterError> {
        let mut unregistration = self.unregistration.lock().unwrap();
        if unregistration.is_some() {
            return Err(NetRegisterError::AlreadyRegistered);
        }
        let wrapper = Box::new(NetObserverWrapper::new(self.observer.clone()));
        let mut ret = 0;
        let handle = RegisterNetObserver(wrapper, &mut ret);
        if handle.is_null() {
            return Err(NetRegisterError::RegisterFailed(ret));
        }
        if ret != 0 {
            return Err(NetRegisterError::RegisterFailed(ret));
        }
        *unregistration = Some(handle);
        Ok(())
    }

    /// Unregisters the observers from the native network service.
    ///
    /// Returns an error if no registration is active or the native
    /// unregistration call fails.
    pub fn unregister(&self) -> Result<(), NetUnregisterError> {
        let mut handle = self.unregistration.lock().unwrap();
        if let Some(inner) = handle.take() {
            let ret = inner.unregister();
            if ret != 0 {
                *handle = Some(inner);
                return Err(NetUnregisterError::UnregisterFailed(ret));
            }
            Ok(())
        } else {
            Err(NetUnregisterError::NotRegistered)
        }
    }
}

/// Errors that can occur when registering network observers.
#[derive(Debug, PartialEq, Eq)]
pub enum NetRegisterError {
    /// A registration is already active and must be unregistered first.
    AlreadyRegistered,
    /// The native registration call failed, carrying its error code.
    RegisterFailed(i32),
}

/// Errors that can occur when unregistering network observers.
#[derive(Debug)]
pub enum NetUnregisterError {
    /// No registration is currently active.
    NotRegistered,
    /// The native unregistration call failed, carrying its error code.
    UnregisterFailed(i32),
}

#[cfg(test)]
mod ut_register {
    include!("../../../tests/ut/observe/network/ut_register.rs");
}
