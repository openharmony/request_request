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

use std::ffi::CStr;

use ani_rs::business_error::BusinessError;
use ani_rs::objects::{AniFnObject, AniObject, AniRef};
use ani_rs::AniEnv;

use super::{Config, Task};
use crate::cstr;
use crate::listener::UdsListener;
use crate::proxy::RequestProxy;

pub const CREATE_SYNC: &CStr = cstr(b"createSync\0");
pub const START_SYNC: &CStr = cstr(b"startSync\0");
pub const ON: &CStr = cstr(b"on\0");

#[ani_rs::native]
pub fn create_sync(config: Config) -> Result<Task, BusinessError> {
    let proxy = RequestProxy::get_instance();
    let task_id = proxy.create(config);
    let task = Task { tid: task_id };

    Ok(task)
}

#[ani_rs::native]
pub fn start_sync(this: Task) -> Result<(), BusinessError> {
    let proxy = RequestProxy::get_instance();
    proxy.start(this.tid);
    std::thread::sleep(std::time::Duration::from_secs(100000));
    Ok(())
}

#[ani_rs::native]
pub fn on(env: &AniEnv, this: Task, method: String, callback: AniFnObject)-> Result<(), BusinessError> {
    UdsListener::get_instance().ensure_channel_open();
    RequestProxy::get_instance().subscribe(this.tid.clone());
    info!("task {} on method: {}", this.tid, method);
    Ok(())
}
