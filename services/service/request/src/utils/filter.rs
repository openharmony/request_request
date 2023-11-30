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

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub(crate) struct CommonFilter {
    pub(crate) before: i64,
    pub(crate) after: i64,
    pub(crate) state: u8,
    pub(crate) action: u8,
    pub(crate) mode: u8,
}

#[derive(Debug)]
pub(crate) struct Filter {
    pub(crate) bundle: String,
    pub(crate) common_data: CommonFilter,
}
