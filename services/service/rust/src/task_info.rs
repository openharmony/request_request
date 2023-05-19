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

use std::collections::HashMap;
use super::{enumration::*, form_item::*, progress::*};
#[derive(Debug)]
pub struct TaskInfo {
    pub uid: u64,
    pub bundle: String,
    pub url: String,
    pub data: String,
    pub file_items: Vec<FormItem>,
    pub file_specs: Vec<FileSpec>,
    pub task_id: u32,
    pub title: String,
    pub description: String,
    pub mime_type: String,
    pub progress: Progress,
    pub ctime: u64,
    pub mtime: u64,
    pub reason: Reason,
    pub extras: HashMap<String, String>,
    pub common_data: CommonTaskInfo,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CommonTaskInfo {
    pub action: Action,
    pub mode: Mode,
    pub gauge: bool,
    pub retry: bool,
    pub tries: u32,
}
