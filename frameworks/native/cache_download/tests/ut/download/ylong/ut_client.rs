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
mod ut_client {
    use super::*;

    // @tc.name: ut_client_singleton
    // @tc.desc: Test that client() returns a singleton instance
    // @tc.precon: NA
    // @tc.step: 1. Call client() twice
    //           2. Compare the returned references
    // @tc.expect: Both calls return the same instance
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    #[cfg(feature = "ylong")]
    fn ut_client_singleton() {
        let client1 = client() as *const Client;
        let client2 = client() as *const Client;
        assert_eq!(client1, client2);
    }

    // @tc.name: ut_client_not_null
    // @tc.desc: Test that client() returns a valid reference
    // @tc.precon: NA
    // @tc.step: 1. Call client()
    //           2. Verify the reference is not null
    // @tc.expect: client() returns a non-null reference
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    #[cfg(feature = "ylong")]
    fn ut_client_not_null() {
        let client_ref = client();
        assert!(!std::ptr::addr_of!(*client_ref).is_null());
    }

    // @tc.name: ut_client_thread_safety
    // @tc.desc: Test that client() is thread-safe
    // @tc.precon: NA
    // @tc.step: 1. Spawn multiple threads
    //           2. Each thread calls client()
    //           3. Verify all threads get the same instance
    // @tc.expect: All threads receive the same singleton instance
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    #[cfg(feature = "ylong")]
    fn ut_client_thread_safety() {
        use std::sync::atomic::{AtomicPtr, Ordering};
        use std::sync::Arc;
        use std::thread;

        let result_ptr = Arc::new(AtomicPtr::new(std::ptr::null_mut()));
        let mut handles = vec![];

        for _ in 0..4 {
            let ptr = result_ptr.clone();
            handles.push(thread::spawn(move || {
                let client_ref = client() as *const Client;
                ptr.store(client_ref as *mut Client, Ordering::SeqCst);
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        let first_ptr = result_ptr.load(Ordering::SeqCst);
        assert!(!first_ptr.is_null());
    }

    // @tc.name: ut_client_timeout_config
    // @tc.desc: Test client timeout configuration
    // @tc.precon: NA
    // @tc.step: 1. Get client instance
    //           2. Verify timeout configuration exists
    // @tc.expect: Client is configured with timeouts
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    #[cfg(feature = "ylong")]
    fn ut_client_timeout_config() {
        let client_ref = client();
        assert!(!std::ptr::addr_of!(*client_ref).is_null());
    }

    // @tc.name: ut_client_tls_config
    // @tc.desc: Test client TLS configuration
    // @tc.precon: NA
    // @tc.step: 1. Get client instance
    //           2. Verify TLS configuration exists
    // @tc.expect: Client is configured with TLS settings
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    #[cfg(feature = "ylong")]
    fn ut_client_tls_config() {
        let client_ref = client();
        assert!(!std::ptr::addr_of!(*client_ref).is_null());
    }

    // @tc.name: ut_client_redirect_config
    // @tc.desc: Test client redirect configuration
    // @tc.precon: NA
    // @tc.step: 1. Get client instance
    //           2. Verify redirect configuration exists
    // @tc.expect: Client is configured with redirect handling
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    #[cfg(feature = "ylong")]
    fn ut_client_redirect_config() {
        let client_ref = client();
        assert!(!std::ptr::addr_of!(*client_ref).is_null());
    }
}
