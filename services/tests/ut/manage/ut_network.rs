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

use std::sync::Arc;
use std::thread;

use super::*;
use crate::manage::network_manager::NetworkManager;

// @tc.name: ut_network_state_online_with_info
// @tc.desc: Test NetworkState::Online contains correct NetworkInfo
// @tc.precon: NA
// @tc.step: 1. Create NetworkInfo with specific parameters
//           2. Create NetworkState::Online with the info
//           3. Verify all fields are correctly stored
// @tc.expect: Online state correctly stores network information
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_state_online_with_info() {
    let info = NetworkInfo {
        network_type: NetworkType::Wifi,
        is_metered: false,
        is_roaming: false,
    };
    
    let state = NetworkState::Online(info.clone());
    
    match state {
        NetworkState::Online(net_info) => {
            assert_eq!(net_info.network_type, NetworkType::Wifi);
            assert_eq!(net_info.is_metered, false);
            assert_eq!(net_info.is_roaming, false);
        }
        _ => panic!("Expected Online variant"),
    }
}

// @tc.name: ut_network_type_discriminant_values
// @tc.desc: Test NetworkType enum has correct discriminant values for FFI
// @tc.precon: NA
// @tc.step: 1. Check each NetworkType variant discriminant
//           2. Verify values match C++ FFI expectations
// @tc.expect: NetworkType discriminants match system-defined values
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_type_discriminant_values() {
    assert_eq!(NetworkType::Other as u8, 0);
    assert_eq!(NetworkType::Wifi as u8, 1);
    assert_eq!(NetworkType::Cellular as u8, 2);
}

// @tc.name: ut_network_state_offline_to_online_transition
// @tc.desc: Test NetworkInner state transition from Offline to Online
// @tc.precon: NA
// @tc.step: 1. Create NetworkInner (starts Offline)
//           2. Call notify_online with NetworkInfo
//           3. Verify state changes to Online
//           4. Verify notify_online returns true (state changed)
// @tc.expect: State correctly transitions from Offline to Online
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_state_offline_to_online_transition() {
    let inner = NetworkInner::new();
    
    assert!(matches!(*inner.state.read().unwrap(), NetworkState::Offline));
    
    let changed = inner.notify_online(NetworkInfo {
        network_type: NetworkType::Wifi,
        is_metered: false,
        is_roaming: false,
    });
    
    assert!(changed, "notify_online should return true when state changes");
    
    match &*inner.state.read().unwrap() {
        NetworkState::Online(info) => {
            assert_eq!(info.network_type, NetworkType::Wifi);
        }
        _ => panic!("Expected Online state"),
    }
}

// @tc.name: ut_network_state_online_to_offline_transition
// @tc.desc: Test NetworkInner state transition from Online to Offline
// @tc.precon: NA
// @tc.step: 1. Create NetworkInner and set to Online
//           2. Call notify_offline
//           3. Verify state changes to Offline
// @tc.expect: State correctly transitions from Online to Offline
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_state_online_to_offline_transition() {
    let inner = NetworkInner::new();
    
    inner.notify_online(NetworkInfo {
        network_type: NetworkType::Cellular,
        is_metered: true,
        is_roaming: true,
    });
    
    assert!(matches!(*inner.state.read().unwrap(), NetworkState::Online(_)));
    
    inner.notify_offline();
    
    assert!(matches!(*inner.state.read().unwrap(), NetworkState::Offline));
}

// @tc.name: ut_network_notify_online_no_change
// @tc.desc: Test notify_online returns false when network info unchanged
// @tc.precon: NA
// @tc.step: 1. Create NetworkInner and set to Online with specific info
//           2. Call notify_online with same info
//           3. Verify returns false (no state change)
// @tc.expect: notify_online returns false when info is identical
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_notify_online_no_change() {
    let inner = NetworkInner::new();
    
    let info = NetworkInfo {
        network_type: NetworkType::Wifi,
        is_metered: false,
        is_roaming: false,
    };
    
    let first_change = inner.notify_online(info.clone());
    assert!(first_change);
    
    let second_change = inner.notify_online(info);
    assert!(!second_change, "notify_online should return false when info unchanged");
}

// @tc.name: ut_network_notify_offline_when_already_offline
// @tc.desc: Test notify_offline when already offline does not change state
// @tc.precon: NA
// @tc.step: 1. Create NetworkInner (starts Offline)
//           2. Call notify_offline
//           3. Verify state remains Offline
// @tc.expect: notify_offline on already offline state is idempotent
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_notify_offline_when_already_offline() {
    let inner = NetworkInner::new();
    
    assert!(matches!(*inner.state.read().unwrap(), NetworkState::Offline));
    
    inner.notify_offline();
    
    assert!(matches!(*inner.state.read().unwrap(), NetworkState::Offline));
}

// @tc.name: ut_network_info_different_types
// @tc.desc: Test NetworkInfo with different network type combinations
// @tc.precon: NA
// @tc.step: 1. Create NetworkInfo with Wifi type
//           2. Create NetworkInfo with Cellular type
//           3. Verify they are not equal
// @tc.expect: Different network types create different NetworkInfo
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_info_different_types() {
    let wifi_info = NetworkInfo {
        network_type: NetworkType::Wifi,
        is_metered: false,
        is_roaming: false,
    };
    
    let cellular_info = NetworkInfo {
        network_type: NetworkType::Cellular,
        is_metered: true,
        is_roaming: false,
    };
    
    assert_ne!(wifi_info, cellular_info);
}

// @tc.name: ut_network_info_metered_flag_change
// @tc.desc: Test NetworkInfo equality with different metered flag
// @tc.precon: NA
// @tc.step: 1. Create two NetworkInfo with same type but different metered flag
//           2. Verify they are not equal
//           3. Test state change detection
// @tc.expect: Metered flag change triggers state update
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_info_metered_flag_change() {
    let inner = NetworkInner::new();
    
    let info_unmetered = NetworkInfo {
        network_type: NetworkType::Cellular,
        is_metered: false,
        is_roaming: false,
    };
    
    let info_metered = NetworkInfo {
        network_type: NetworkType::Cellular,
        is_metered: true,
        is_roaming: false,
    };
    
    assert_ne!(info_unmetered, info_metered);
    
    inner.notify_online(info_unmetered.clone());
    let changed = inner.notify_online(info_metered);
    
    assert!(changed, "Metered flag change should trigger state update");
}

// @tc.name: ut_network_info_roaming_flag_change
// @tc.desc: Test NetworkInfo equality with different roaming flag
// @tc.precon: NA
// @tc.step: 1. Create two NetworkInfo with same type but different roaming flag
//           2. Verify they are not equal
//           3. Test state change detection
// @tc.expect: Roaming flag change triggers state update
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_info_roaming_flag_change() {
    let inner = NetworkInner::new();
    
    let info_not_roaming = NetworkInfo {
        network_type: NetworkType::Cellular,
        is_metered: true,
        is_roaming: false,
    };
    
    let info_roaming = NetworkInfo {
        network_type: NetworkType::Cellular,
        is_metered: true,
        is_roaming: true,
    };
    
    assert_ne!(info_not_roaming, info_roaming);
    
    inner.notify_online(info_not_roaming.clone());
    let changed = inner.notify_online(info_roaming);
    
    assert!(changed, "Roaming flag change should trigger state update");
}

// @tc.name: ut_network_state_equality_semantics
// @tc.desc: Test NetworkState equality for state comparison logic
// @tc.precon: NA
// @tc.step: 1. Create identical NetworkState instances
//           2. Create different NetworkState instances
//           3. Verify equality comparison
// @tc.expect: Equality comparison works correctly for state detection
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_state_equality_semantics() {
    let offline1 = NetworkState::Offline;
    let offline2 = NetworkState::Offline;
    assert_eq!(offline1, offline2);
    
    let wifi_info = NetworkInfo {
        network_type: NetworkType::Wifi,
        is_metered: false,
        is_roaming: false,
    };
    
    let online1 = NetworkState::Online(wifi_info.clone());
    let online2 = NetworkState::Online(wifi_info);
    assert_eq!(online1, online2);
    
    assert_ne!(offline1, online1);
    
    let cellular_info = NetworkInfo {
        network_type: NetworkType::Cellular,
        is_metered: true,
        is_roaming: true,
    };
    let online_cellular = NetworkState::Online(cellular_info);
    assert_ne!(online1, online_cellular);
}

// @tc.name: ut_network_inner_concurrent_access
// @tc.desc: Test NetworkInner concurrent read/write safety
// @tc.precon: NA
// @tc.step: 1. Create NetworkInner
//           2. Spawn multiple threads for concurrent reads
//           3. Perform state changes in main thread
//           4. Verify no data races or panics
// @tc.expect: RwLock provides safe concurrent access
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_inner_concurrent_access() {
    let inner = Arc::new(NetworkInner::new());
    let mut handles = vec![];
    
    for _ in 0..5 {
        let inner_clone = Arc::clone(&inner);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let state = inner_clone.state.read().unwrap();
                let _ = format!("{:?}", *state);
            }
        });
        handles.push(handle);
    }
    
    for i in 0..10 {
        if i % 2 == 0 {
            inner.notify_online(NetworkInfo {
                network_type: NetworkType::Wifi,
                is_metered: false,
                is_roaming: false,
            });
        } else {
            inner.notify_offline();
        }
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

// @tc.name: ut_network_manager_singleton
// @tc.desc: Test NetworkManager singleton pattern
// @tc.precon: NA
// @tc.step: 1. Get NetworkManager instance twice
//           2. Verify both references point to same instance
// @tc.expect: NetworkManager returns same singleton instance
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_manager_singleton() {
    let instance1 = NetworkManager::get_instance();
    let instance2 = NetworkManager::get_instance();
    
    let addr1 = instance1 as *const _ as usize;
    let addr2 = instance2 as *const _ as usize;
    assert_eq!(addr1, addr2, "NetworkManager should return same singleton instance");
}

// @tc.name: ut_network_manager_initial_state
// @tc.desc: Test NetworkManager initial network state
// @tc.precon: NA
// @tc.step: 1. Get NetworkManager instance
//           2. Check initial state is Offline
// @tc.expect: NetworkManager starts with Offline state
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_manager_initial_state() {
    let network_manager = NetworkManager::get_instance().lock().unwrap();
    let state = network_manager.network.state();
    
    assert!(matches!(state, NetworkState::Offline));
}

// @tc.name: ut_network_manager_state_query
// @tc.desc: Test NetworkManager query_network functionality
// @tc.precon: NA
// @tc.step: 1. Get NetworkManager instance
//           2. Query network state
//           3. Verify state is returned correctly
// @tc.expect: query_network returns correct state
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_manager_state_query() {
    let state = NetworkManager::query_network();
    
    assert!(matches!(state, NetworkState::Offline));
}

// @tc.name: ut_network_manager_is_online_check
// @tc.desc: Test NetworkManager is_online functionality
// @tc.precon: NA
// @tc.step: 1. Check is_online when network is Offline
//           2. Set network to Online
//           3. Check is_online returns true
// @tc.expect: is_online correctly reflects network state
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_manager_is_online_check() {
    assert!(!NetworkManager::is_online());
    
    {
        let mut network_manager = NetworkManager::get_instance().lock().unwrap();
        network_manager.network.inner.notify_online(NetworkInfo {
            network_type: NetworkType::Wifi,
            is_metered: false,
            is_roaming: false,
        });
    }
    
    assert!(NetworkManager::is_online());
    
    {
        let mut network_manager = NetworkManager::get_instance().lock().unwrap();
        network_manager.network.inner.notify_offline();
    }
    
    assert!(!NetworkManager::is_online());
}

// @tc.name: ut_network_state_transition_sequence
// @tc.desc: Test complete network state transition sequence
// @tc.precon: NA
// @tc.step: 1. Start Offline
//           2. Transition to Online (Wifi)
//           3. Transition to Online (Cellular)
//           4. Transition to Offline
//           5. Verify each transition
// @tc.expect: State transitions follow correct sequence
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_state_transition_sequence() {
    let inner = NetworkInner::new();
    
    assert!(matches!(*inner.state.read().unwrap(), NetworkState::Offline));
    
    let changed = inner.notify_online(NetworkInfo {
        network_type: NetworkType::Wifi,
        is_metered: false,
        is_roaming: false,
    });
    assert!(changed);
    
    let changed = inner.notify_online(NetworkInfo {
        network_type: NetworkType::Cellular,
        is_metered: true,
        is_roaming: true,
    });
    assert!(changed);
    
    inner.notify_offline();
    assert!(matches!(*inner.state.read().unwrap(), NetworkState::Offline));
    
    let changed = inner.notify_online(NetworkInfo {
        network_type: NetworkType::Wifi,
        is_metered: false,
        is_roaming: false,
    });
    assert!(changed, "Transition from Offline to Online should return true");
}

// @tc.name: ut_network_type_all_variants
// @tc.desc: Test all NetworkType variants are distinct
// @tc.precon: NA
// @tc.step: 1. Create all NetworkType variants
//           2. Verify they are all distinct
// @tc.expect: All NetworkType variants are unique
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_type_all_variants() {
    let types = [NetworkType::Other, NetworkType::Wifi, NetworkType::Cellular];
    
    for (i, t1) in types.iter().enumerate() {
        for (j, t2) in types.iter().enumerate() {
            if i != j {
                assert_ne!(t1, t2);
            } else {
                assert_eq!(t1, t2);
            }
        }
    }
}
