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

use std::fmt::Display;

use crate::hash::url_hash;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct TaskId {
    hash: String,
}

impl TaskId {
    pub fn new(hash: String) -> Self {
        Self { hash }
    }

    pub fn from_url(url: &str) -> Self {
        Self {
            hash: url_hash(url),
        }
    }

    pub fn brief(&self) -> &str {
        let len = self.hash.len();
        &self.hash.as_str()[..len / 4]
    }
}

impl Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hash)
    }
}
