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
use test::RegisterNetObserver;

use super::wrapper::ffi::NetUnregistration;
#[cfg(not(test))]
use super::wrapper::ffi::RegisterNetObserver;
use super::wrapper::NetObserverWrapper;
use super::Observer;

pub struct NetRegistrar {
    observer: Arc<Mutex<Vec<Box<dyn Observer>>>>,
    unregistration: Mutex<Option<UniquePtr<NetUnregistration>>>,
}

impl NetRegistrar {
    pub fn new() -> Self {
        Self {
            observer: Arc::new(Mutex::new(Vec::new())),
            unregistration: Mutex::new(None),
        }
    }

    pub fn add_observer(&self, observer: impl Observer + 'static) {
        self.observer.lock().unwrap().push(Box::new(observer));
    }

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

#[derive(Debug, PartialEq, Eq)]
pub enum NetRegisterError {
    AlreadyRegistered,
    RegisterFailed(i32),
}

#[derive(Debug)]
pub enum NetUnregisterError {
    NotRegistered,
    UnregisterFailed(i32),
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::observe::network::{NetBearType, NetCap, NetInfo};
    const TEST_NET_ID: i32 = 100;

    #[allow(non_snake_case, clippy::boxed_local)]
    pub fn RegisterNetObserver(
        wrapper: Box<NetObserverWrapper>,
        error: &mut i32,
    ) -> UniquePtr<NetUnregistration> {
        wrapper.net_available(TEST_NET_ID);
        wrapper.net_lost(TEST_NET_ID);
        wrapper.net_capability_changed(
            TEST_NET_ID,
            NetInfo {
                caps: vec![NetCap::NET_CAPABILITY_INTERNET],
                bear_types: vec![NetBearType::BEARER_WIFI],
            },
        );
        *error = 0;
        UniquePtr::null()
    }

    struct TestObserver;

    impl Observer for TestObserver {
        fn net_available(&self, net_id: i32) {
            assert_eq!(net_id, TEST_NET_ID);
        }
        fn net_lost(&self, net_id: i32) {
            assert_eq!(net_id, TEST_NET_ID);
        }
        fn net_capability_changed(&self, net_id: i32, net_info: &NetInfo) {
            assert_eq!(net_id, TEST_NET_ID);
            assert_eq!(net_info.caps, vec![NetCap::NET_CAPABILITY_INTERNET]);
            assert_eq!(net_info.bear_types, vec![NetBearType::BEARER_WIFI]);
        }
    }

    #[test]
    fn ut_net_observer_callback() {
        let registrar = NetRegistrar::new();
        for _ in 0..10 {
            let observer = TestObserver;
            registrar.add_observer(observer);
        }
        assert_eq!(
            registrar.register(),
            Err(NetRegisterError::RegisterFailed(0))
        );
        let observer = TestObserver;
        registrar.add_observer(observer);
        assert_eq!(
            registrar.register(),
            Err(NetRegisterError::RegisterFailed(0))
        );
    }
}
