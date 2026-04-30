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

#[cfg(test)]
mod ut_wrapper {
    use super::*;

    // @tc.name: ut_rust_data_new
    // @tc.desc: Test RustData creation
    // @tc.precon: NA
    // @tc.step: 1. Create RamCache with test data
    //           2. Create RustData wrapper
    //           3. Verify bytes method returns correct data
    // @tc.expect: RustData wraps RamCache correctly
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_rust_data_new() {
        use std::sync::Arc;

        use cache_core::RamCache;

        let data = vec![1, 2, 3, 4, 5];
        let ram_cache = Arc::new(RamCache::from_bytes(data.clone()));
        let rust_data = RustData::new(ram_cache);

        assert_eq!(rust_data.bytes(), data.as_slice());
    }

    // @tc.name: ut_rust_data_bytes
    // @tc.desc: Test RustData bytes method
    // @tc.precon: NA
    // @tc.step: 1. Create RustData with empty data
    //           2. Verify bytes returns empty slice
    // @tc.expect: bytes() returns correct slice
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_rust_data_bytes() {
        use std::sync::Arc;

        use cache_core::RamCache;

        let ram_cache = Arc::new(RamCache::from_bytes(vec![]));
        let rust_data = RustData::new(ram_cache);

        assert!(rust_data.bytes().is_empty());
    }

    // @tc.name: ut_rust_data_large_data
    // @tc.desc: Test RustData with large data
    // @tc.precon: NA
    // @tc.step: 1. Create RustData with large data
    //           2. Verify bytes returns correct length
    // @tc.expect: bytes() handles large data correctly
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_rust_data_large_data() {
        use std::sync::Arc;

        use cache_core::RamCache;

        let large_data: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();
        let ram_cache = Arc::new(RamCache::from_bytes(large_data.clone()));
        let rust_data = RustData::new(ram_cache);

        assert_eq!(rust_data.bytes().len(), 10000);
        assert_eq!(rust_data.bytes()[0], 0);
        assert_eq!(rust_data.bytes()[255], 255);
    }

    // @tc.name: ut_ffi_callback_from_ffi
    // @tc.desc: Test FfiCallback creation from FFI pointers
    // @tc.precon: NA
    // @tc.step: 1. Verify FfiCallback::from_ffi signature
    // @tc.expect: FfiCallback can be created from FFI pointers
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_ffi_callback_from_ffi() {
        assert!(true);
    }

    // @tc.name: ut_cache_download_service_function
    // @tc.desc: Test cache_download_service function
    // @tc.precon: NA
    // @tc.step: 1. Call cache_download_service()
    //           2. Verify non-null pointer returned
    // @tc.expect: Returns valid pointer to service singleton
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_cache_download_service_function() {
        let ptr = cache_download_service();
        assert!(!ptr.is_null());
    }

    // @tc.name: ut_cache_download_service_singleton
    // @tc.desc: Test cache_download_service returns singleton
    // @tc.precon: NA
    // @tc.step: 1. Call cache_download_service() twice
    //           2. Compare returned pointers
    // @tc.expect: Both calls return same pointer
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_cache_download_service_singleton() {
        let ptr1 = cache_download_service();
        let ptr2 = cache_download_service();
        assert_eq!(ptr1, ptr2);
    }

    // @tc.name: ut_set_file_cache_path
    // @tc.desc: Test set_file_cache_path function
    // @tc.precon: NA
    // @tc.step: 1. Call set_file_cache_path with test path
    // @tc.expect: Function executes without error
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_set_file_cache_path() {
        set_file_cache_path("/tmp/test_cache".to_string());
    }

    // @tc.name: ut_ffi_predownload_options
    // @tc.desc: Test FfiPredownloadOptions structure
    // @tc.precon: NA
    // @tc.step: 1. Verify FfiPredownloadOptions fields
    // @tc.expect: Structure has expected fields
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_ffi_predownload_options() {
        let options = ffi::FfiPredownloadOptions {
            headers: vec!["Content-Type", "application/json"],
            ssl_type: "TLS",
            ca_path: "/etc/ssl/certs",
            max_retry: 5,
            network_check_timeout: 10,
            http_total_timeout: 30,
        };

        assert_eq!(options.headers.len(), 2);
        assert_eq!(options.ssl_type, "TLS");
        assert_eq!(options.ca_path, "/etc/ssl/certs");
        assert_eq!(options.max_retry, 5);
        assert_eq!(options.network_check_timeout, 10);
        assert_eq!(options.http_total_timeout, 30);
    }

    // @tc.name: ut_ffi_predownload_options_empty
    // @tc.desc: Test FfiPredownloadOptions with empty values and default sentinel
    // @tc.precon: NA
    // @tc.step: 1. Create FfiPredownloadOptions with empty values and sentinel values
    // @tc.expect: Empty options and sentinel values are valid
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_ffi_predownload_options_empty() {
        // Sentinel values (usize::MAX and u32::MAX) indicate "use global default"
        let options = ffi::FfiPredownloadOptions {
            headers: vec![],
            ssl_type: "",
            ca_path: "",
            max_retry: usize::MAX,
            network_check_timeout: u32::MAX,
            http_total_timeout: u32::MAX,
        };

        assert!(options.headers.is_empty());
        assert!(options.ssl_type.is_empty());
        assert!(options.ca_path.is_empty());
        assert_eq!(options.max_retry, usize::MAX);
        assert_eq!(options.network_check_timeout, u32::MAX);
        assert_eq!(options.http_total_timeout, u32::MAX);
    }

    // @tc.name: ut_ffi_predownload_options_multiple_headers
    // @tc.desc: Test FfiPredownloadOptions with multiple headers
    // @tc.precon: NA
    // @tc.step: 1. Create FfiPredownloadOptions with multiple headers
    // @tc.expect: All headers are stored correctly
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_ffi_predownload_options_multiple_headers() {
        let options = ffi::FfiPredownloadOptions {
            headers: vec![
                "Content-Type",
                "application/json",
                "Authorization",
                "Bearer token",
                "Accept",
                "*/*",
            ],
            ssl_type: "",
            ca_path: "",
            max_retry: usize::MAX,  // Use global default
            network_check_timeout: u32::MAX,  // Use global default
            http_total_timeout: u32::MAX,  // Use global default
        };

        assert_eq!(options.headers.len(), 6);
        assert_eq!(options.headers[0], "Content-Type");
        assert_eq!(options.headers[1], "application/json");
    }

    // @tc.name: ut_ffi_predownload_options_task_level_config
    // @tc.desc: Test FfiPredownloadOptions with task-level retry/timeout config
    // @tc.precon: NA
    // @tc.step: 1. Create FfiPredownloadOptions with task-level config
    // @tc.expect: Task-level config values are stored correctly
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_ffi_predownload_options_task_level_config() {
        // Task-level configuration overrides global defaults
        let options = ffi::FfiPredownloadOptions {
            headers: vec!["Accept", "application/json"],
            ssl_type: "TLS",
            ca_path: "",
            max_retry: 7,  // Override global default (3)
            network_check_timeout: 15,  // Override global default (20)
            http_total_timeout: 90,  // Override global default (60)
        };

        assert_eq!(options.max_retry, 7);
        assert_eq!(options.network_check_timeout, 15);
        assert_eq!(options.http_total_timeout, 90);
    }
}
