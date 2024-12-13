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

pub(crate) const INIT: usize = 0;
pub(crate) const RUNNING: usize = 1;
pub(crate) const SUCCESS: usize = 2;
pub(crate) const FAIL: usize = 3;
pub(crate) const CANCEL: usize = 4;

mod callback;

cfg_netstack! {
    mod netstack;
}

cfg_ylong! {
    mod ylong;
}

pub(crate) mod common;
mod error;

pub(crate) use error::CacheDownloadError;
pub(crate) mod task;
