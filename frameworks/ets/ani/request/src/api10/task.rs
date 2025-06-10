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

use ani_rs::business_error::BusinessError;
use request_client::RequestClient;

use crate::api10::bridge::Task;

#[ani_rs::native]
pub fn start(this: Task) -> Result<(), BusinessError> {
    let task_id = this.tid.parse().unwrap();
    RequestClient::get_instance().start(task_id);
    Ok(())
}

#[ani_rs::native]
pub fn pause(this: Task) -> Result<(), BusinessError> {
    let task_id = this.tid.parse().unwrap();
    RequestClient::get_instance().pause(task_id);
    Ok(())
}

#[ani_rs::native]
pub fn resume(this: Task) -> Result<(), BusinessError> {
    let task_id = this.tid.parse().unwrap();
    RequestClient::get_instance().resume(task_id);
    Ok(())
}

#[ani_rs::native]
pub fn stop(this: Task) -> Result<(), BusinessError> {
    let task_id = this.tid.parse().unwrap();
    RequestClient::get_instance().stop(task_id);
    Ok(())
}

#[ani_rs::native]
pub fn set_max_speed(this: Task, speed: i64) -> Result<(), BusinessError> {
    let task_id = this.tid.parse().unwrap();
    RequestClient::get_instance().set_max_speed(task_id, speed);
    Ok(())
}
