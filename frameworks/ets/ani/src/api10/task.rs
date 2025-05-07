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

use ani_rs::objects::{AniFnObject, AniObject, AniRef};

use super::{Config, Task};
use crate::cstr;
use crate::listener::UdsListener;
use crate::proxy::RequestProxy;

pub const CREATE_SYNC: &CStr = cstr(b"createSync\0");
pub const START_SYNC: &CStr = cstr(b"startSync\0");
pub const ON: &CStr = cstr(b"on\0");

pub fn create_sync<'local>(
    env: ani_rs::AniEnv<'local>,
    _ani_this: AniRef<'local>,
    config: AniObject<'local>,
) -> ani_rs::objects::AniRef<'local> {
    let proxy = RequestProxy::get_instance();
    let config: Config = env.deserialize(config).unwrap();
    // let res = env.undefined();
    // env.throw_business_error(code, message)
    let task_id = proxy.create(config);
    let task = Task { tid: task_id };
    env.serialize(&task).unwrap()
}

pub fn start_sync<'local>(env: ani_rs::AniEnv<'local>, ani_this: AniRef<'local>) {
    let proxy = RequestProxy::get_instance();
    let task: Task = env.deserialize(ani_this.into()).unwrap();
    proxy.start(task.tid);
    std::thread::sleep(std::time::Duration::from_secs(100000));
}

pub fn on<'local>(
    env: ani_rs::AniEnv<'local>,
    ani_this: AniRef<'local>,
    method: AniObject<'local>,
    callback: AniFnObject,
) {
    let task: Task = env.deserialize(ani_this.into()).unwrap();
    let method: String = env.deserialize(method).unwrap();

    UdsListener::get_instance().ensure_channel_open();
    RequestProxy::get_instance().subscribe(task.tid.clone());
    info!("task {} on method: {}", task.tid, method);
}
