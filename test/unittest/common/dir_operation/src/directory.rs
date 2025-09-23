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

use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub(crate) fn is_dir_exist(path: &str) -> bool {
    let path = PathBuf::from(path);
    path.is_dir()
}

pub(crate) fn create_dir(path: &str) -> bool {
    fs::create_dir_all(path).is_ok()
}

pub(crate) fn remove_dir(path: &str) -> bool {
    fs::remove_dir_all(path).is_ok()
}

pub(crate) fn is_file_exist(path: &str) -> bool {
    Path::new(path).exists()
}
