/*
 * Copyright (C) 2023 Huawei Device Co., Ltd.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use super::{enumration::*, form_item::*};
use std::collections::HashMap;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CommonTaskConfig {
    pub action: Action,
    pub mode: Mode,
    pub cover: bool,
    pub network: Network,
    pub metered: bool,
    pub roaming: bool,
    pub retry: bool,
    pub redirect: bool,
    pub index: u32,
    pub begins: u64,
    pub ends: i64,
    pub gauge: bool,
    pub precise: bool,
    pub background: bool,
}
#[derive(Debug)]
pub struct TaskConfig {
    pub bundle: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub data: String, // use for download
    pub ability: String,
    pub token: String,
    pub extras: HashMap<String, String>,
    pub version: Version,
    pub form_items: Vec<FormItem>,
    pub file_specs: Vec<FileSpec>,
    pub common_data: CommonTaskConfig,
}
