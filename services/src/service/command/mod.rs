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

use crate::error::ErrorCode;

mod construct;
mod dump;
mod get_task;
mod open_channel;
mod pause;
mod query;
mod query_mime_type;
mod remove;
mod resume;
mod search;
mod show;
mod start;
mod stop;
mod sub_runcount;
mod subscribe;
mod touch;
mod unsub_runcount;
mod unsubscribe;

pub(crate) const CONTROL_MAX: usize = 500;
pub(crate) const GET_INFO_MAX: usize = 100;

pub(crate) fn set_code_with_index(vec: &mut [ErrorCode], index: usize, code: ErrorCode) {
    if let Some(c) = vec.get_mut(index) {
        *c = code;
    } else {
        error!("out index: {}", index);
    }
}

pub(crate) fn set_code_with_index_other<T>(
    vec: &mut [(ErrorCode, T)],
    index: usize,
    code: ErrorCode,
) {
    if let Some((c, _t)) = vec.get_mut(index) {
        *c = code;
    } else {
        error!("out index: {}", index);
    }
}
