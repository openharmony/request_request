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

use ffi::{DisableTaskNotification, SetAccessTokenPermission, SetMode};

fn main() {
    SetAccessTokenPermission();
    println!("Please Input Test CASE");
    println!("1. Disable Task Notification Bar");
    println!("2. Set Task Mode");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        "1" => loop {
            println!("please input TaskId");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            DisableTaskNotification(input.trim());
        },
        "2" => loop {
            println!("please input TaskId");
            let mut task_id = String::new();
            std::io::stdin().read_line(&mut task_id).unwrap();
            println!("please input Mode 0 for background 1 for foreground");
            let mut mode = String::new();
            std::io::stdin().read_line(&mut mode).unwrap();
            let mode = match mode.trim() {
                "0" => 0,
                "1" => 1,
                _ => {
                    println!("invalid mode");
                    continue;
                }
            };
            SetMode(task_id.trim(), mode);
        },
        _ => {
            println!("invalid inpu");
        }
    }
}

#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {

    unsafe extern "C++" {
        include!("wrapper.h");
        fn DisableTaskNotification(task_id: &str);
        fn SetMode(task_id: &str, mode: i32);
        fn SetAccessTokenPermission();
    }
}
