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

use std::io;
use std::ops::Deref;

use super::common::CommonError;

#[derive(Debug)]
pub struct CacheDownloadError {
    code: Option<i32>,
    message: String,
    kind: ErrorKind,
}

impl CacheDownloadError {
    pub fn code(&self) -> i32 {
        self.code.unwrap_or(0)
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn ffi_kind(&self) -> i32 {
        self.kind.clone() as i32
    }
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Http,
    Io,
}

impl From<io::Error> for CacheDownloadError {
    fn from(err: io::Error) -> Self {
        CacheDownloadError {
            code: err.raw_os_error(),
            message: err.to_string(),
            kind: ErrorKind::Io,
        }
    }
}

impl<'a, E> From<&'a E> for CacheDownloadError
where
    E: CommonError,
{
    fn from(err: &'a E) -> Self {
        CacheDownloadError {
            code: Some(err.code()),
            message: err.msg().to_string(),
            kind: ErrorKind::Http,
        }
    }
}
