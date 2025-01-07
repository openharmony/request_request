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

#![allow(missing_docs)]

use ffi::{DisableTaskNotification, SetAccessTokenPermission};

fn main() {
    SetAccessTokenPermission();
    println!("Disable Task Notification Bar TEST");
    loop {
        println!("please input task");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        DisableTaskNotification(&input);
    }
}

#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {

    unsafe extern "C++" {
        include!("wrapper.h");
        fn DisableTaskNotification(task_id: &str);
        fn SetAccessTokenPermission();
    }
}
