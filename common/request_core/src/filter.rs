// Copyright (c) 2023 Huawei Device Co., Ltd.
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

use crate::config::{Action, Mode};
use crate::info::State;

pub struct SearchFilter {
    pub bundle_name: Option<String>,
    pub before: Option<i64>,
    pub after: Option<i64>,
    pub state: Option<State>,
    pub action: Option<Action>,
    pub mode: Option<Mode>,
}

impl SearchFilter {
    pub fn new() -> Self {
        SearchFilter {
            bundle_name: None,
            before: None,
            after: None,
            state: None,
            action: None,
            mode: None,
        }
    }
}
