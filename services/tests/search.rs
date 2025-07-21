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
#![cfg(feature = "oh")]
use std::time::{SystemTime, UNIX_EPOCH};

use download_server::config::{Action, ConfigBuilder, Mode};
use download_server::info::State;
use test_common::test_init;

fn get_current_timestamp() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() as u64,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

// @tc.name: sdv_search_user
// @tc.desc: Test task search functionality with different parameters
// @tc.precon: NA
// @tc.step: 1. Initialize test agent and create download task
//           2. Search tasks with valid time range and parameters
//           3. Search tasks with invalid time range and parameters
//           4. Verify search results match expected task IDs
// @tc.expect: Valid searches return created task ID, invalid searches return
// empty
// @tc.type: FUNC
// @tc.require: issues#ICN31I
#[test]
fn sdv_search_user() {
    let agent = test_init();

    let config = ConfigBuilder::new()
        .action(Action::Download)
        .mode(Mode::FrontEnd)
        .build();
    let task_id = agent.construct(config);
    let current = get_current_timestamp() as i64;
    let v = agent.search(current, current - 3000, State::Any, Action::Any, Mode::Any);
    assert!(v.contains(&task_id));
    let v = agent.search(current + 3000, current, State::Any, Action::Any, Mode::Any);
    assert!(!v.contains(&task_id));
    let v = agent.search(
        current,
        current - 3000,
        State::Initialized,
        Action::Download,
        Mode::FrontEnd,
    );
    assert!(v.contains(&task_id));
    let v = agent.search(
        current,
        current - 3000,
        State::Running,
        Action::Download,
        Mode::FrontEnd,
    );
    assert!(!v.contains(&task_id));
    let v = agent.search(
        current,
        current - 3000,
        State::Initialized,
        Action::Upload,
        Mode::FrontEnd,
    );
    assert!(!v.contains(&task_id));

    let v = agent.search(
        current,
        current - 3000,
        State::Initialized,
        Action::Download,
        Mode::BackGround,
    );
    assert!(!v.contains(&task_id));
}
