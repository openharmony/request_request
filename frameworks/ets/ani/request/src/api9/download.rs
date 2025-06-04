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

#![allow(unused)]

use std::path::PathBuf;

use ani_rs::business_error::BusinessError;
use ani_rs::objects::{AniObject, AniRef};
use ani_rs::AniEnv;
use request_client::RequestClient;
use request_core::config::Version;
use request_utils::context::{is_stage_context, Context};

use super::bridge::{DownloadConfig, DownloadTask};
use crate::api9::bridge::DownloadInfo;
use crate::seq::TaskSeq;

#[ani_rs::native]
pub fn download_file(
    env: &AniEnv,
    context: AniRef,
    config: DownloadConfig,
) -> Result<DownloadTask, BusinessError> {
    let context = AniObject::from(context);
    info!("is {}", is_stage_context(env, &context));

    let seq = TaskSeq::next();
    info!("Api9 task, seq: {}", seq.0);

    let context = Context::new(env, &context);

    let save_as = match &config.file_path {
        Some(path) => path.to_string(),
        None => {
            let name = PathBuf::from(&config.url);
            name.file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or(config.url.clone())
        }
    };

    let task = match RequestClient::get_instance().crate_task(
        context,
        Version::API9,
        config,
        &save_as,
        false,
    ) {
        Ok(task_id) => DownloadTask { task_id },
        Err(e) => {
            return Err(BusinessError::new(-1, format!("Download failed")));
        }
    };

    match RequestClient::get_instance().start(task.task_id) {
        Ok(_) => {
            info!("Api9 download started successfully, seq: {}", seq.0);
            Ok(task)
        }
        Err(e) => {
            error!("Api9 download start failed, error: {}", e);
            Err(BusinessError::new(
                e,
                format!("Download start failed with error code: {}", e),
            ))
        }
    }
}

#[ani_rs::native]
pub fn delete(this: DownloadTask) -> Result<(), BusinessError> {
    RequestClient::get_instance()
        .remove(this.task_id)
        .map_err(|e| BusinessError::new(e, "Failed to delete download task".to_string()))
}

#[ani_rs::native]
pub fn suspend(this: DownloadTask) -> Result<(), BusinessError> {
    RequestClient::get_instance()
        .pause(this.task_id)
        .map_err(|e| BusinessError::new(e, "Failed to suspend download task".to_string()))
}

#[ani_rs::native]
pub fn restore(this: DownloadTask) -> Result<(), BusinessError> {
    RequestClient::get_instance()
        .resume(this.task_id)
        .map_err(|e| BusinessError::new(e, "Failed to restore download task".to_string()))
}

#[ani_rs::native]
pub fn get_task_info(this: DownloadTask) -> Result<DownloadTask, BusinessError> {
    Ok(this)
}

#[ani_rs::native]
pub fn get_task_mime_type(this: DownloadTask) -> Result<String, BusinessError> {
    Ok("application/octet-stream".to_string())
}
