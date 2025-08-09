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
mod ut_observe {
    use mockall::mock;
    use request_utils::observe::network;
    use std::sync::Arc;

    // Mock CacheDownloadService
    mock! {
        pub CacheDownloadService {
            pub fn get_instance() -> Arc<Self> {}
            pub fn reset_all_tasks(&self) {}
        }
    }

    use cache_download::observe::NetObserver;
    use cache_download::services::CacheDownloadService;

    // @tc.name: ut_net_observer_net_available
    // @tc.desc: Test net_available method of NetObserver
    // @tc.precon: NA
    // @tc.step: 1. Create a mock CacheDownloadService instance
    // 2. Set expectation for reset_all_tasks method
    // 3. Create a NetObserver instance
    // 4. Call net_available method with valid net_id
    // @tc.expect: reset_all_tasks method is called once
    // @tc.type: FUNC
    // @tc.require: NA
    // @tc.level: Level 1
    #[test]
    fn ut_net_observer_net_available_001() {
        let mut mock_service = MockCacheDownloadService::new();
        mock_service.expect_reset_all_tasks().times(1).return_const(());
        let mock_service_arc = Arc::new(mock_service);

        MockCacheDownloadService::expect_get_instance().return_const(mock_service_arc.clone());

        let observer = NetObserver;
        observer.net_available(123);

    }

    // @tc.name: ut_net_observer_net_available_zero_id
    // @tc.desc: Test net_available method with zero net_id
    // @tc.precon: NA
    // @tc.step: 1. Create a mock CacheDownloadService instance
    // 2. Set expectation for reset_all_tasks method
    // 3. Create a NetObserver instance
    // 4. Call net_available method with net_id = 0
    // @tc.expect: reset_all_tasks method is called once
    // @tc.type: FUNC
    // @tc.require: NA
    // @tc.level: Level 2
    #[test]
    fn ut_net_observer_net_available_zero_id_001() {
        let mut mock_service = MockCacheDownloadService::new();
        mock_service.expect_reset_all_tasks().times(1).return_const(());
        let mock_service_arc = Arc::new(mock_service);

        MockCacheDownloadService::expect_get_instance().return_const(mock_service_arc.clone());

        let observer = NetObserver;
        observer.net_available(0);

    }

    // @tc.desc: Test net_available method with zero net_id
    // @tc.precon: NA
    // @tc.step: 1. Create a mock CacheDownloadService instance
    // 2. Set expectation for reset_all_tasks method
    // 3. Create a NetObserver instance
    // 4. Call net_available method with net_id = 0
    // @tc.expect: reset_all_tasks method is called once
    // @tc.type: FUNC
    // @tc.require: NA
    // @tc.level: Level 2
    #[test]
    fn ut_net_observer_net_available_zero_id_001() {
        let mut mock_service = MockCacheDownloadService::new();
        mock_service.expect_reset_all_tasks().times(1).return_const(());
        let mock_service_arc = Arc::new(mock_service);

        MockCacheDownloadService::expect_get_instance().return_const(mock_service_arc.clone());

        let observer = NetObserver;
        observer.net_available(0);

    }

    // @tc.name: ut_net_observer_net_available_negative_id
    // @tc.desc: Test net_available method with negative net_id
    // @tc.precon: NA
    // @tc.step: 1. Create a mock CacheDownloadService instance
    // 2. Set expectation for reset_all_tasks method
    // 3. Create a NetObserver instance
    // 4. Call net_available method with negative net_id
    // @tc.expect: reset_all_tasks method is called once
    // @tc.type: FUNC
    // @tc.require: NA
    // @tc.level: Level 2
    #[test]
    fn ut_net_observer_net_available_negative_id_001() {
        let mut mock_service = MockCacheDownloadService::new();
        mock_service.expect_reset_all_tasks().times(1).return_const(());
        let mock_service_arc = Arc::new(mock_service);

        MockCacheDownloadService::expect_get_instance().return_const(mock_service_arc.clone());

        let observer = NetObserver;
        observer.net_available(-456);

    }
}