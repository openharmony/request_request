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

use crate::error::HttpClientError;
use crate::info::DownloadInfo;
use crate::request::{Request, RequestCallback};
use crate::response::Response;
use crate::task::RequestTask;

// Mock callback implementation for testing
#[derive(Debug, Default, PartialEq)]
struct MockCallback {
    on_success_called: bool,
    on_fail_called: bool,
    on_cancel_called: bool,
    on_data_receive_called: bool,
    on_progress_called: bool,
    on_restart_called: bool,
}

impl RequestCallback for MockCallback {
    fn on_success(&mut self, _response: Response) {
        self.on_success_called = true;
    }

    fn on_fail(&mut self, _error: HttpClientError, _info: DownloadInfo) {
        self.on_fail_called = true;
    }

    fn on_cancel(&mut self) {
        self.on_cancel_called = true;
    }

    fn on_data_receive(&mut self, _data: &[u8], _task: RequestTask) {
        self.on_data_receive_called = true;
    }

    fn on_progress(&mut self, _dl_total: u64, _dl_now: u64, _ul_total: u64, _ul_now: u64) {
        self.on_progress_called = true;
    }

    fn on_restart(&mut self) {
        self.on_restart_called = true;
    }
}

// @tc.name: ut_request_ssl_type
// @tc.desc: Test function ssl_type of Request
// @tc.precon: NA
// @tc.step: 1. Create a Request instance using default()
// 2. Call ssl_type() method
// @tc.expect: No crash happen.
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 1
#[test]
fn ut_request_ssl_type() {
    let mut request: Request<MockCallback> = Request::default();
    request.ssl_type("");
    request.ssl_type("TLS");
    request.ssl_type("TLCP");
    request.ssl_type("TTLLCCPP");
    assert!(!request.inner.is_null());
}

// @tc.name: ut_request_ca_path
// @tc.desc: Test function ca_path of Request
// @tc.precon: NA
// @tc.step: 1. Create a Request instance using default()
// 2. Call ssl_type() method
// @tc.expect: No crash happen.
// @tc.type: FUNC
// @tc.require: issueNumber
// @tc.level: Level 1
#[test]
fn ut_request_ca_path() {
    let mut request: Request<MockCallback> = Request::default();
    request.ca_path("/data");
    request.ca_path("");
    assert!(!request.inner.is_null());
}
