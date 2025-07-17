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

use super::*;
use crate::tests::test_init;

// @tc.name: test_cert_manager
// @tc.desc: Test certificate manager initialization and certificate retrieval
// @tc.precon: NA
// @tc.step: 1. Initialize test environment
//           2. Create CertManager instance
//           3. Check certificate existence
//           4. Force update if certificate is none
//           5. Verify certificate exists after update
// @tc.expect: Certificate manager initializes successfully and certificate is available
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn test_cert_manager() {
    test_init();
    let cert_manager = CertManager::init();
    let cert = cert_manager.certificate();
    if cert.is_none() {
        cert_manager.force_update();
    }
    assert!(cert_manager.certificate().is_some());
}