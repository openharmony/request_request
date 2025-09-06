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
use request_client::client::error::CreateTaskError;
use request_client::RequestClient;
use request_client::check::file::DownloadPathError;
use request_core::config::Version;
use request_core::filter::SearchFilter;
use request_utils::context::Context;

use crate::api10::bridge::{Config, Filter, Task, TaskInfo};
use crate::seq::TaskSeq;

#[ani_rs::native]
pub fn create(env: &AniEnv, context: AniRef, config: Config) -> Result<Task, BusinessError> {
    let context = AniObject::from(context);
    let seq = TaskSeq::next();
    info!("Api10 task, seq: {}", seq.0);
    let context = Context::new(env, &context);

    let save_as = match &config.saveas {
        Some(path) if path != "./" => path.to_string(),
        _ => {
            let name = PathBuf::from(&config.url);
            name.file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or(config.url.clone())
        }
    };
    let overwrite = config.overwrite.unwrap_or(false);

    info!("Creating task with config: {:?}", overwrite);

    match RequestClient::get_instance().create_task(
        context,
        Version::API10,
        config.into(),
        &save_as,
        overwrite,
    ) {
        Ok(task_id) => Ok(Task {
            tid: task_id.to_string(),
        }),
        Err(e) => {
            error!("Create task failed: {:?}", e);
            match e {
                CreateTaskError::DownloadPath(err) => {
                    let (code, message) = match err {
                        DownloadPathError::InvalidPath => (401, "Invalid Path"),
                        _ => (13400001, "Invalid file or file system error.")
                    };
                    Err(BusinessError::new_static(code, message))
                },
                CreateTaskError::Code(code) => {
                    Err(BusinessError::new_static(code, "Create Task Failed"))
                }
            }
        }
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
        .map_err(|_| BusinessError::new(401, "Invalid task ID format".to_string()))?;
    RequestClient::get_instance()
        .remove(task_id)
        .map_err(|e| BusinessError::new_static(e, "Failed to remove task"))
}

#[ani_rs::native]
pub fn show(id: String) -> Result<TaskInfo, BusinessError> {
    let task_id = id.parse::<i64>().unwrap();
    RequestClient::get_instance()
        .show_task(task_id)
        .map(|info| {
            info!("Api10 get task info: {:?}", info);
            TaskInfo::from(info)
        })
        .map_err(|e| BusinessError::new(e, "Failed to get download task info".to_string()))
}

#[ani_rs::native]
pub fn touch(id: String, token: String) -> Result<(), BusinessError> {
    Ok(())
}

#[ani_rs::native]
pub fn search(filter: Option<Filter>) -> Result<Vec<String>, BusinessError> {
    let filter = match filter {
        Some(f) => f.into(),
        None => SearchFilter::new(),
    };
    RequestClient::get_instance()
        .search(filter)
        .map(|tasks| {
            info!("Api10 search tasks: {:?}", tasks);
            tasks
        })
        .map_err(|e| BusinessError::new(e, "Failed to search tasks".to_string()))
}

#[ani_rs::native]
pub fn query(id: String) -> Result<TaskInfo, BusinessError> {
    println!("Querying task with id: {}", id);
    todo!()
}
