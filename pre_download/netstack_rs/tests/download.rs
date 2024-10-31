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

#![allow(unused)]

use std::time::Duration;

use netstack_rs::request::{Request, RequestCallback};
use netstack_rs::response::Response;

struct Callback {}
impl RequestCallback for Callback {
    fn on_fail(&mut self, error: netstack_rs::error::HttpClientError) {}
}

#[test]
fn download() {
    let mut request = Request::new();
    request
        .url("http://192.168.0.101/bind.png")
        .method("GET")
        .callback(Callback {});
    let mut task = request.build();
    task.start();
    let status = task.status();
    println!("{:?}", status);
}
