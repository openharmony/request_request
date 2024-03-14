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

use std::os::fd::RawFd;

#[derive(Clone, Debug)]
pub(crate) struct FileSpec {
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) file_name: String,
    pub(crate) mime_type: String,
    pub(crate) is_user_file: bool,
    // Only for user file.
    pub(crate) fd: Option<RawFd>,
}

#[derive(Clone, Debug)]
pub(crate) struct FormItem {
    pub(crate) name: String,
    pub(crate) value: String,
}
