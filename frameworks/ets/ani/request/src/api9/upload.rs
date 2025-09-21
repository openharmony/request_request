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
use request_client::client::error::CreateTaskError;
use request_core::config::Version;
use request_core::info::TaskInfo;
use request_utils::context::{is_stage_context, Context};

use super::bridge::{DownloadConfig, DownloadTask};
use crate::api9::bridge::DownloadInfo;
use crate::seq::TaskSeq;

use crate::api9::bridge::{UploadConfig, UploadTask};

#[ani_rs::native]
pub fn upload_file(env: &AniEnv, context: AniRef, config: UploadConfig) -> Result<UploadTask, BusinessError> {
    let context = AniObject::from(context);
    let seq = TaskSeq::next();
    info!("Api9 task, seq: {}", seq.0);
    let context = Context::new(env, &context);

    let task = match RequestClient::get_instance().create_task(
        context,
        Version::API9,
        config.into(),
        &"",
        false,
    ) {
        Ok(task_id) => UploadTask { task_id: task_id.to_string() },
        Err(CreateTaskError::DownloadPath(_)) => {
            return Err(BusinessError::new(
                13400001,
                "Invalid file or file system error.".to_string(),
            ))
        },
        Err(CreateTaskError::Code(code)) => {
            return Err(BusinessError::new(
                code,
                "Upload failed.".to_string(),
            ))
        }
    };

    let tid = task.task_id.parse().unwrap();
    match RequestClient::get_instance().start(tid) {
        Ok(_) => {
            info!("Api9 upload started successfully, seq: {}", seq.0);
            Ok(task)
        }
        Err(e) => {
            error!("Api9 upload start failed, error: {}", e);
            Err(BusinessError::new(
                e,
                format!("Upload start failed with error code: {}", e),
            ))
        }
    }
}

#[ani_rs::native]
pub fn delete(this: UploadTask) -> Result<(), BusinessError> {
    Ok(())
}
