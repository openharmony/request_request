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

use std::collections::HashSet;

use super::*;
use crate::manage::network::{NetworkInfo, NetworkState, NetworkType};
use crate::manage::scheduler::qos::RssCapacity;

// @tc.name: ut_state_record_new
// @tc.desc: Test StateRecord creation with new()
// @tc.precon: NA
// @tc.step: 1. Create StateRecord using new()
//           2. Verify all fields have default values
// @tc.expect: All fields have correct default values
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_state_record_new() {
    let record = StateRecord::new();
    
    assert!(record.foreground_abilities.is_empty());
    assert!(record.foreground_users.is_empty());
    assert!(matches!(record.network, NetworkState::Offline));
    assert!(record.active_accounts.is_empty());
    assert_eq!(record.rss_level, 0);
}

// @tc.name: ut_state_record_update_rss_level
// @tc.desc: Test RSS level update with change detection
// @tc.precon: NA
// @tc.step: 1. Create StateRecord
//           2. Update RSS level and verify change detection
//           3. Update to same level and verify no change
// @tc.expect: RSS level update works correctly with change detection
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_state_record_update_rss_level() {
    let mut record = StateRecord::new();
    
    let result = record.update_rss_level(1);
    assert!(result.is_some());
    assert_eq!(record.rss_level, 1);
    let capacity = result.unwrap();
    assert_eq!(capacity.m1(), 8);
    assert_eq!(capacity.m2(), 32);
    
    let result = record.update_rss_level(1);
    assert!(result.is_none());
    assert_eq!(record.rss_level, 1);
    
    let result = record.update_rss_level(7);
    assert!(result.is_some());
    assert_eq!(record.rss_level, 7);
    let capacity = result.unwrap();
    assert_eq!(capacity.m1(), 4);
    assert_eq!(capacity.m2(), 4);
}

// @tc.name: ut_state_record_update_network
// @tc.desc: Test network state update with change detection
// @tc.precon: NA
// @tc.step: 1. Create StateRecord
//           2. Update network state and verify change detection
//           3. Update to same state and verify no change
// @tc.expect: Network state update works correctly with change detection
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_state_record_update_network() {
    let mut record = StateRecord::new();
    
    let online_state = NetworkState::Online(NetworkInfo {
        network_type: NetworkType::Wifi,
        is_metered: false,
        is_roaming: false,
    });
    
    let result = record.update_network(online_state.clone());
    assert!(result.is_some());
    assert!(matches!(record.network, NetworkState::Online(_)));
    
    let result = record.update_network(online_state.clone());
    assert!(result.is_none());
    
    let cellular_state = NetworkState::Online(NetworkInfo {
        network_type: NetworkType::Cellular,
        is_metered: true,
        is_roaming: true,
    });
    
    let result = record.update_network(cellular_state);
    assert!(result.is_some());
}

// @tc.name: ut_state_record_update_accounts
// @tc.desc: Test account state update with change detection
// @tc.precon: NA
// @tc.step: 1. Create StateRecord
//           2. Update accounts and verify change detection
//           3. Update to same accounts and verify no change
// @tc.expect: Account state update works correctly with change detection
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_state_record_update_accounts() {
    let mut record = StateRecord::new();
    
    let foreground_accounts: HashSet<u64> = [100].into_iter().collect();
    let active_accounts: HashSet<u64> = [100, 101].into_iter().collect();
    
    let result = record.update_accounts(foreground_accounts.clone(), active_accounts.clone());
    assert!(result.is_some());
    assert_eq!(record.active_accounts.len(), 2);
    assert_eq!(record.foreground_users.len(), 1);
    
    let result = record.update_accounts(foreground_accounts, active_accounts);
    assert!(result.is_none());
    
    let new_active: HashSet<u64> = [100, 101, 102].into_iter().collect();
    let result = record.update_accounts(HashSet::new(), new_active);
    assert!(result.is_some());
    assert_eq!(record.active_accounts.len(), 3);
}

// @tc.name: ut_state_record_update_top_uid
// @tc.desc: Test foreground UID update
// @tc.precon: NA
// @tc.step: 1. Create StateRecord
//           2. Update top UID (app comes to foreground)
//           3. Verify UID is added to foreground_abilities
// @tc.expect: Foreground UID is added correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_state_record_update_top_uid() {
    let mut record = StateRecord::new();
    
    let result = record.update_top_uid(1000);
    assert!(result.is_some());
    assert!(record.foreground_abilities.contains(&1000));
    
    let result = record.update_top_uid(2000);
    assert!(result.is_some());
    assert!(record.foreground_abilities.contains(&2000));
    assert_eq!(record.foreground_abilities.len(), 2);
}

// @tc.name: ut_state_record_update_background
// @tc.desc: Test background state update
// @tc.precon: NA
// @tc.step: 1. Create StateRecord with foreground UIDs
//           2. Update background (app goes to background)
//           3. Verify UID is removed from foreground_abilities
// @tc.expect: Background update removes UID from foreground
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_state_record_update_background() {
    let mut record = StateRecord::new();
    record.foreground_abilities.insert(1000);
    record.foreground_abilities.insert(2000);
    
    record.update_background(1000);
    assert!(!record.foreground_abilities.contains(&1000));
    assert!(record.foreground_abilities.contains(&2000));
    
    record.update_background(999);
    assert_eq!(record.foreground_abilities.len(), 1);
}

// @tc.name: ut_state_record_update_background_timeout
// @tc.desc: Test background timeout check
// @tc.precon: NA
// @tc.step: 1. Create StateRecord
//           2. Check timeout for UID not in foreground
//           3. Add UID to foreground and check timeout again
// @tc.expect: Background timeout check works correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_state_record_update_background_timeout() {
    let mut record = StateRecord::new();
    
    let result = record.update_background_timeout(1000);
    assert!(result.is_some());
    
    record.foreground_abilities.insert(1000);
    let result = record.update_background_timeout(1000);
    assert!(result.is_none());
}

// @tc.name: ut_state_record_foreground_abilities
// @tc.desc: Test foreground_abilities HashSet operations
// @tc.precon: NA
// @tc.step: 1. Create StateRecord
//           2. Insert and remove UIDs from foreground_abilities
// @tc.expect: foreground_abilities operations work correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_state_record_foreground_abilities() {
    let mut record = StateRecord::new();
    
    record.foreground_abilities.insert(1000);
    record.foreground_abilities.insert(2000);
    
    assert!(record.foreground_abilities.contains(&1000));
    assert!(record.foreground_abilities.contains(&2000));
    assert!(!record.foreground_abilities.contains(&3000));
    assert_eq!(record.foreground_abilities.len(), 2);
    
    record.foreground_abilities.remove(&1000);
    assert!(!record.foreground_abilities.contains(&1000));
    assert_eq!(record.foreground_abilities.len(), 1);
}

// @tc.name: ut_state_record_active_accounts
// @tc.desc: Test active_accounts HashSet operations
// @tc.precon: NA
// @tc.step: 1. Create StateRecord
//           2. Insert UIDs into active_accounts
// @tc.expect: active_accounts operations work correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_state_record_active_accounts() {
    let mut record = StateRecord::new();
    
    record.active_accounts.insert(100);
    record.active_accounts.insert(101);
    record.active_accounts.insert(102);
    
    assert_eq!(record.active_accounts.len(), 3);
    assert!(record.active_accounts.contains(&101));
}

// @tc.name: ut_state_record_foreground_users
// @tc.desc: Test foreground_users HashSet operations
// @tc.precon: NA
// @tc.step: 1. Create StateRecord
//           2. Insert user IDs into foreground_users
// @tc.expect: foreground_users operations work correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_state_record_foreground_users() {
    let mut record = StateRecord::new();
    
    record.foreground_users.insert(0);
    record.foreground_users.insert(100);
    
    assert_eq!(record.foreground_users.len(), 2);
    assert!(record.foreground_users.contains(&100));
}

// @tc.name: ut_state_record_network_offline
// @tc.desc: Test network state is Offline by default
// @tc.precon: NA
// @tc.step: 1. Create StateRecord
//           2. Verify network state is Offline
// @tc.expect: Network state is Offline by default
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_state_record_network_offline() {
    let record = StateRecord::new();
    
    match record.network {
        NetworkState::Offline => assert!(true),
        NetworkState::Online(_) => panic!("Expected Offline"),
    }
}

// @tc.name: ut_state_record_rss_capacity_integration
// @tc.desc: Test RssCapacity integration with StateRecord
// @tc.precon: NA
// @tc.step: 1. Create StateRecord
//           2. Update RSS level and verify RssCapacity values
// @tc.expect: RssCapacity integration works correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_state_record_rss_capacity_integration() {
    let mut record = StateRecord::new();
    
    let capacity = record.update_rss_level(0).unwrap();
    assert_eq!(capacity.m1(), 8);
    assert_eq!(capacity.m2(), 32);
    assert_eq!(capacity.m3(), 8);
    
    let capacity = record.update_rss_level(7).unwrap();
    assert_eq!(capacity.m1(), 4);
    assert_eq!(capacity.m2(), 4);
    assert_eq!(capacity.m3(), 2);
}
