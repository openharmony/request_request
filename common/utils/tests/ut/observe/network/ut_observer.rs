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

#[cfg(test)]
mod ut_observer {
    use mockall::automock;
    use super::*;
    use mockall::mock;

    mock! {
        pub Observer {
            fn net_available(&self, net_id: i32);
            fn net_lost(&self, net_id: i32);
            fn net_capability_changed(&self, net_id: i32, net_info: &super::super::wrapper::ffi::NetInfo);
        }

        impl Send for Observer {}
        impl Sync for Observer {}
    }

    // @tc.name: ut_observer_trait_default_impls
    // @tc.desc: Test default implementations of all Observer trait methods
    // @tc.precon: NA
    // @tc.step: 1. Create mock observer
    // 2. Call net_available(0)
    // 3. Call net_lost(0)
    // 4. Call net_capability_changed(0, empty NetInfo)
    // @tc.expect: No panic occurs and default implementations are called
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_observer_trait_default_impls() {
        struct DefaultObserver;
        impl Observer for DefaultObserver {}

        let observer = DefaultObserver;
        observer.net_available(0);
        observer.net_lost(0);
        observer.net_capability_changed(0, &super::super::wrapper::ffi::NetInfo { caps: vec![], bear_types: vec![] });
    }

    // @tc.name: ut_observer_net_available_custom_impl
    // @tc.desc: Test custom implementation of net_available method
    // @tc.precon: NA
    // @tc.step: 1. Create mock observer with custom net_available
    // 2. Call net_available(100)
    // @tc.expect: Custom implementation is called with correct net_id
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_observer_net_available_custom_impl() {
        let mut mock = MockObserver::new();
        mock.expect_net_available()
            .with(eq(100))
            .times(1)
            .return_const(());

        mock.net_available(100);
    }

    // @tc.name: ut_observer_net_lost_custom_impl
    // @tc.desc: Test custom implementation of net_lost method
    // @tc.precon: NA
    // @tc.step: 1. Create mock observer with custom net_lost
    // 2. Call net_lost(200)
    // @tc.expect: Custom implementation is called with correct net_id
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_observer_net_lost_custom_impl() {
        let mut mock = MockObserver::new();
        mock.expect_net_lost()
            .with(eq(200))
            .times(1)
            .return_const(());

        mock.net_lost(200);
    }

    // @tc.name: ut_observer_net_capability_changed_custom_impl
    // @tc.desc: Test custom implementation of net_capability_changed method
    // @tc.precon: NA
    // @tc.step: 1. Create NetInfo with INTERNET capability
    // 2. Create mock observer with custom net_capability_changed
    // 3. Call net_capability_changed(300, net_info)
    // @tc.expect: Custom implementation is called with correct parameters
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_observer_net_capability_changed_custom_impl() {
        use super::super::wrapper::ffi::{NetBearType, NetCap, NetInfo};

        let net_info = NetInfo {
            caps: vec![NetCap::NET_CAPABILITY_INTERNET],
            bear_types: vec![NetBearType::BEARER_WIFI],
        };

        let mut mock = MockObserver::new();
        mock.expect_net_capability_changed()
            .with(eq(300), eq(&net_info))
            .times(1)
            .return_const(());

        mock.net_capability_changed(300, &net_info);
    }

    // @tc.name: ut_observer_trait_object_safety
    // @tc.desc: Verify Observer trait is object-safe
    // @tc.precon: NA
    // @tc.step: 1. Create trait object from mock observer
    // 2. Call methods on trait object
    // @tc.expect: No compilation errors and methods can be called
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 3
    #[test]
    fn ut_observer_trait_object_safety() {
        let mock = MockObserver::new();
        let observer: Box<dyn Observer> = Box::new(mock);
        observer.net_available(0);
    }
}