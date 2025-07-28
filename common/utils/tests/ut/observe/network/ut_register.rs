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

// @tc.name: ut_net_observer_callback
// @tc.desc: Test network observer callback functions
// @tc.precon: NA
// @tc.step: 1. Create NetRegistrar instance
//           2. Add multiple TestObserver instances
//           3. Call register method
//           4. Verify callback assertions
// @tc.expect: All observer callbacks receive correct network events
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level1
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
