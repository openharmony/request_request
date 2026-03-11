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

//! Unit tests for proxy module (mod.rs)
//!
//! Tests the RequestProxy singleton pattern and service token configuration.

use request_next::proxy::RequestProxy;
use request_next::proxy::SERVICE_TOKEN;

// @tc.name: ut_service_token_value
// @tc.desc: Test SERVICE_TOKEN constant matches expected service identifier
// @tc.precon: NA
// @tc.step: 1. Check SERVICE_TOKEN value
//           2. Verify it matches OHOS download service identifier format
// @tc.expect: SERVICE_TOKEN equals "OHOS.Download.RequestServiceInterface"
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_service_token_value() {
    assert_eq!(SERVICE_TOKEN, "OHOS.Download.RequestServiceInterface");
}

// @tc.name: ut_service_token_format
// @tc.desc: Test SERVICE_TOKEN follows OHOS naming convention
// @tc.precon: NA
// @tc.step: 1. Verify SERVICE_TOKEN contains required parts
//           2. Check format follows "OHOS.{Subsystem}.{Service}" pattern
// @tc.expect: SERVICE_TOKEN contains "OHOS", "Download", and "RequestServiceInterface"
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_service_token_format() {
    assert!(SERVICE_TOKEN.starts_with("OHOS."), "Service token should start with 'OHOS.'");
    assert!(SERVICE_TOKEN.contains("Download"), "Service token should contain 'Download'");
    assert!(SERVICE_TOKEN.contains("RequestServiceInterface"), "Service token should contain service name");
}

// @tc.name: ut_request_proxy_singleton
// @tc.desc: Test RequestProxy implements singleton pattern correctly
// @tc.precon: NA
// @tc.step: 1. Get RequestProxy instance twice
//           2. Compare memory addresses
// @tc.expect: Both references point to the same singleton instance
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_request_proxy_singleton() {
    let proxy1 = RequestProxy::get_instance();
    let proxy2 = RequestProxy::get_instance();
    
    assert!(std::ptr::eq(proxy1, proxy2), "RequestProxy should return same instance");
}

// @tc.name: ut_request_proxy_singleton_thread_safety
// @tc.desc: Test RequestProxy singleton is thread-safe
// @tc.precon: NA
// @tc.step: 1. Spawn multiple threads to get RequestProxy instance
//           2. Collect all instance addresses
//           3. Verify all addresses are identical
// @tc.expect: All threads receive the same singleton instance
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_request_proxy_singleton_thread_safety() {
    use std::sync::Arc;
    use std::thread;
    
    let handles: Vec<_> = (0..10)
        .map(|_| {
            thread::spawn(|| {
                let proxy = RequestProxy::get_instance();
                proxy as *const RequestProxy as usize
            })
        })
        .collect();
    
    let addresses: Vec<usize> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    
    let first = addresses[0];
    for addr in &addresses[1..] {
        assert_eq!(*addr, first, "All threads should get same singleton instance");
    }
}

// @tc.name: ut_request_proxy_multiple_calls_consistency
// @tc.desc: Test RequestProxy returns consistent instance across multiple calls
// @tc.precon: NA
// @tc.step: 1. Call get_instance() 100 times
//           2. Verify all return same instance
// @tc.expect: All calls return identical instance pointer
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_request_proxy_multiple_calls_consistency() {
    let first = RequestProxy::get_instance();
    
    for _ in 0..100 {
        let current = RequestProxy::get_instance();
        assert!(std::ptr::eq(first, current));
    }
}

// @tc.name: ut_service_token_not_empty
// @tc.desc: Test SERVICE_TOKEN is not empty string
// @tc.precon: NA
// @tc.step: 1. Check SERVICE_TOKEN length
// @tc.expect: SERVICE_TOKEN has non-zero length
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_service_token_not_empty() {
    assert!(!SERVICE_TOKEN.is_empty(), "SERVICE_TOKEN should not be empty");
    assert!(SERVICE_TOKEN.len() > 10, "SERVICE_TOKEN should have meaningful length");
}

// @tc.name: ut_service_token_parts_count
// @tc.desc: Test SERVICE_TOKEN has correct number of dot-separated parts
// @tc.precon: NA
// @tc.step: 1. Split SERVICE_TOKEN by '.'
//           2. Count parts
// @tc.expect: SERVICE_TOKEN has exactly 3 parts (OHOS, Download, RequestServiceInterface)
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_service_token_parts_count() {
    let parts: Vec<&str> = SERVICE_TOKEN.split('.').collect();
    assert_eq!(parts.len(), 3, "SERVICE_TOKEN should have 3 dot-separated parts");
    assert_eq!(parts[0], "OHOS");
    assert_eq!(parts[1], "Download");
    assert_eq!(parts[2], "RequestServiceInterface");
}
