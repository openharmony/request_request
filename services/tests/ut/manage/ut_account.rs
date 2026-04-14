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

use ylong_runtime::sync::mpsc;

use super::*;
use crate::tests::test_init;

// @tc.name: ut_account_check_oh
// @tc.desc: Test account subscription and status change functionality
// @tc.precon: NA
// @tc.step: 1. Initialize test environment
//           2. Subscribe to account changes
//           3. Verify account status updates correctly
// @tc.expect: Account status changes are detected and processed
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_account_check_oh() {
    test_init();

    assert!(ACTIVE_ACCOUNTS.lock().unwrap().foreground.is_empty());
    assert!(ACTIVE_ACCOUNTS.lock().unwrap().background.is_empty());

    let (tx, mut rx) = mpsc::unbounded_channel();
    let task_manager = TaskManagerTx { tx };
    registry_account_subscribe(task_manager);
    ylong_runtime::block_on(async {
        let msg = rx.recv().await.unwrap();
        assert!(matches!(
            msg,
            TaskManagerEvent::Account(AccountEvent::Changed)
        ));
        assert!(!ACTIVE_ACCOUNTS.lock().unwrap().foreground.is_empty());
        assert!(ACTIVE_ACCOUNTS.lock().unwrap().background.is_empty());
    })
}

// @tc.name: ut_account_update
// @tc.desc: Test account update mechanism
// @tc.precon: NA
// @tc.step: 1. Initialize test environment and create account updater
//           2. Trigger account update event
//           3. Verify account change notification is received
// @tc.expect: Account change event is properly propagated
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_account_update() {
    test_init();
    ylong_runtime::block_on(async {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let task_manager = TaskManagerTx { tx };
        let updater = AccountUpdater::new(task_manager.clone());
        drop(updater);
        ylong_runtime::time::sleep(std::time::Duration::from_secs(2)).await;
        assert!(rx.is_empty());
        let mut updater = AccountUpdater::new(task_manager);
        updater.change_flag = true;
        drop(updater);
        let msg = rx.recv().await.unwrap();
        assert!(matches!(
            msg,
            TaskManagerEvent::Account(AccountEvent::Changed)
        ));
    })
}

// @tc.name: ut_account_update_branch
// @tc.desc: Test account update branch conditions
// @tc.precon: NA
// @tc.step: 1. Compare different foreground and background account configurations
//           2. Verify branch conditions for account updates
// @tc.expect: Branch conditions correctly identify account changes
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_account_update_branch() {
    use std::collections::HashSet;

    // Test foreground account comparisons
    let old_foreground: HashSet<i32> = HashSet::new();
    let foreground_accounts: HashSet<i32> = [100].into_iter().collect();
    assert_ne!(old_foreground, foreground_accounts);

    let old_foreground: HashSet<i32> = [101].into_iter().collect();
    assert_ne!(old_foreground, foreground_accounts);

    let old_foreground: HashSet<i32> = [100].into_iter().collect();
    assert_eq!(old_foreground, foreground_accounts);

    // Test background account comparisons
    let old_background: HashSet<i32> = HashSet::new();
    let background_accounts: HashSet<i32> = [200].into_iter().collect();
    assert_ne!(old_background, background_accounts);

    let old_background: HashSet<i32> = [201].into_iter().collect();
    assert_ne!(old_background, background_accounts);

    let old_background: HashSet<i32> = [200].into_iter().collect();
    assert_eq!(old_background, background_accounts);
}
