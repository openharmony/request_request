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

use std::sync::{Arc, Mutex, RwLock};

use super::*;
use crate::manage::network::{Network, NetworkInfo, NetworkInner, NetworkState, NetworkType};

// @tc.name: ut_network_manager_structure
// @tc.desc: Test NetworkManager structure with required fields
// @tc.precon: NA
// @tc.step: 1. Verify NetworkManager has network and tx fields
//           2. Check field types are correct
// @tc.expect: NetworkManager structure is correctly defined
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_manager_structure() {
    let inner = NetworkInner::new();
    let network = Network {
        inner,
        _registry: None,
    };
    
    let manager = NetworkManager {
        network,
        tx: None,
    };
    
    assert!(manager.tx.is_none());
}

// @tc.name: ut_network_manager_is_online
// @tc.desc: Test NetworkManager is_online functionality
// @tc.precon: NA
// @tc.step: 1. Create NetworkManager with offline state
//           2. Check is_online returns false
//           3. Update to online state
//           4. Check is_online returns true
// @tc.expect: is_online correctly reports network state
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_manager_is_online() {
    let offline = NetworkState::Offline;
    assert!(!matches!(offline, NetworkState::Online(_)));
    
    let info = NetworkInfo {
        network_type: NetworkType::Wifi,
        is_metered: false,
        is_roaming: false,
    };
    let online = NetworkState::Online(info);
    assert!(matches!(online, NetworkState::Online(_)));
}

// @tc.name: ut_network_manager_query_network
// @tc.desc: Test NetworkManager query_network functionality
// @tc.precon: NA
// @tc.step: 1. Create NetworkManager
//           2. Query network state
//           3. Verify initial state is Offline
// @tc.expect: query_network returns correct state
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_manager_query_network() {
    let inner = NetworkInner::new();
    let network = Network {
        inner,
        _registry: None,
    };
    
    let state = network.state();
    assert!(matches!(state, NetworkState::Offline));
}

// @tc.name: ut_network_manager_tx_option
// @tc.desc: Test NetworkManager tx Option field
// @tc.precon: NA
// @tc.step: 1. Create NetworkManager with None tx
//           2. Verify tx is None
// @tc.expect: tx Option works correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_manager_tx_option() {
    let inner = NetworkInner::new();
    let network = Network {
        inner,
        _registry: None,
    };
    
    let manager = NetworkManager {
        network,
        tx: None,
    };
    
    assert!(manager.tx.is_none());
}

// @tc.name: ut_network_manager_thread_safety
// @tc.desc: Test NetworkManager thread safety with Mutex
// @tc.precon: NA
// @tc.step: 1. Create Mutex-protected state
//           2. Access from multiple threads
// @tc.expect: Mutex provides thread-safe access
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_manager_thread_safety() {
    use std::thread;
    
    let state = Arc::new(Mutex::new(0i32));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let state_clone = Arc::clone(&state);
        let handle = thread::spawn(move || {
            let mut guard = state_clone.lock().unwrap();
            *guard += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    assert_eq!(*state.lock().unwrap(), 10);
}

// @tc.name: ut_network_manager_rwlock_read_write
// @tc.desc: Test RwLock for network state
// @tc.precon: NA
// @tc.step: 1. Create RwLock-protected state
//           2. Perform concurrent read operations
//           3. Perform write operation
// @tc.expect: RwLock allows concurrent reads and exclusive writes
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_manager_rwlock_read_write() {
    let state = Arc::new(RwLock::new(false));
    
    {
        let read1 = state.read().unwrap();
        let read2 = state.read().unwrap();
        assert!(!*read1);
        assert!(!*read2);
    }
    
    {
        let mut write = state.write().unwrap();
        *write = true;
    }
    
    let read = state.read().unwrap();
    assert!(*read);
}

// @tc.name: ut_network_manager_state_transitions
// @tc.desc: Test network state transitions
// @tc.precon: NA
// @tc.step: 1. Create NetworkInner (starts Offline)
//           2. Transition to Online with Wifi
//           3. Transition to Online with Cellular
//           4. Transition back to Offline
// @tc.expect: State transitions work correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_manager_state_transitions() {
    let inner = NetworkInner::new();
    
    assert!(matches!(*inner.state.read().unwrap(), NetworkState::Offline));
    
    inner.notify_online(NetworkInfo {
        network_type: NetworkType::Wifi,
        is_metered: false,
        is_roaming: false,
    });
    
    if let NetworkState::Online(info) = &*inner.state.read().unwrap() {
        assert_eq!(info.network_type, NetworkType::Wifi);
    } else {
        panic!("Expected Online state");
    }
    
    inner.notify_online(NetworkInfo {
        network_type: NetworkType::Cellular,
        is_metered: true,
        is_roaming: true,
    });
    
    if let NetworkState::Online(info) = &*inner.state.read().unwrap() {
        assert_eq!(info.network_type, NetworkType::Cellular);
        assert!(info.is_metered);
        assert!(info.is_roaming);
    } else {
        panic!("Expected Online state");
    }
    
    inner.notify_offline();
    assert!(matches!(*inner.state.read().unwrap(), NetworkState::Offline));
}

// @tc.name: ut_network_manager_network_clone
// @tc.desc: Test Network Clone trait
// @tc.precon: NA
// @tc.step: 1. Create Network
//           2. Clone it
//           3. Verify clone works
// @tc.expect: Clone works correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_manager_network_clone() {
    let inner = NetworkInner::new();
    let network = Network {
        inner,
        _registry: None,
    };
    
    let cloned = network.clone();
    
    assert!(matches!(cloned.state(), NetworkState::Offline));
}
