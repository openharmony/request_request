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

use crate::check::file::DownloadPathError;

#[derive(Debug)]
pub enum CreateTaskError {
    DownloadPath(DownloadPathError),
    Code(i32),
}

impl From<DownloadPathError> for CreateTaskError {
    fn from(error: DownloadPathError) -> Self {
        CreateTaskError::DownloadPath(error)
    }
}

impl From<i32> for CreateTaskError {
    fn from(code: i32) -> Self {
        CreateTaskError::Code(code)
    }
}
