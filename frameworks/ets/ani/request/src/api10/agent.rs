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

use std::path::PathBuf;

use ani_rs::business_error::BusinessError;
use ani_rs::objects::{AniObject, AniRef};
use ani_rs::AniEnv;
use request_client::RequestClient;
use request_core::config::Version;
use request_utils::context::{is_stage_context, Context};

use crate::api10::bridge::{Config, Filter, Task, TaskInfo};
use crate::seq::TaskSeq;

#[ani_rs::native]
pub fn create(env: &AniEnv, context: AniRef, config: Config) -> Result<Task, BusinessError> {
    let context = AniObject::from(context);
    info!("is {}", is_stage_context(env, &context));

    let seq = TaskSeq::next();

    info!("Api10 task, seq: {}", seq.0);
    let context = Context::new(env, &context);

    let save_as = match &config.saveas {
        Some(path) => path.to_string(),
        None => {
            let name = PathBuf::from(&config.url);
            name.file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or(config.url.clone())
        }
    };
    let overwrite = config.overwrite.unwrap_or(false);

    match RequestClient::get_instance().crate_task(context, Version::API10, config, &save_as, overwrite) {
        Ok(task_id) => Ok(Task { tid: task_id }),
        Err(e) => Err(BusinessError::new(-1, format!("Create task failed"))),
    }
}

#[ani_rs::native]
pub fn get_task(
    context: AniRef,
    task_id: String,
    token: Option<String>,
) -> Result<Task, BusinessError> {
    todo!()
}

#[ani_rs::native]
pub fn remove(id: String) -> Result<(), BusinessError> {
    let task_id = id
        .parse::<i64>()
        .map_err(|_| BusinessError::new(-1, "Invalid task ID format".to_string()))?;
    let _ = RequestClient::get_instance().remove(task_id);
    Ok(())
}

#[ani_rs::native]
pub fn show(id: String) -> Result<TaskInfo, BusinessError> {
    todo!()
}

#[ani_rs::native]
pub fn touch(id: String, token: String) -> Result<(), BusinessError> {
    Ok(())
}

#[ani_rs::native]
pub fn search(filter: Option<Filter>) -> Result<Vec<String>, BusinessError> {
    info!("Searching tasks");
    Ok(vec!["hello".to_string()])
}

#[ani_rs::native]
pub fn query(id: String) -> Result<TaskInfo, BusinessError> {
    println!("Querying task with id: {}", id);
    todo!()
}
