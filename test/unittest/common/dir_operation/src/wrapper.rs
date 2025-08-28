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

use crate::directory::{create_dir, is_dir_exist, is_file_exist, remove_dir};

#[cxx::bridge(namespace = "OHOS::Request::DirOperation")]
pub(crate) mod ffi {
    extern "Rust" {
        fn create_dir(path: &str) -> bool;
        fn remove_dir(path: &str) -> bool;
        fn is_dir_exist(path: &str) -> bool;
        fn is_file_exist(path: &str) -> bool;
    }
}
