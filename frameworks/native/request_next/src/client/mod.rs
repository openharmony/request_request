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

pub mod error;

use std::sync::{Arc, OnceLock};

use request_core::config::{TaskConfig, Version};
use request_core::file::FileSpec;
use request_core::info::TaskInfo;
use request_utils::context::Context;

use crate::client::error::CreateTaskError;
use crate::listen::Observer;
use crate::proxy::RequestProxy;
use crate::{check, Callback};

pub struct RequestClient<'a> {
    listener: Observer,
    proxy: &'a RequestProxy,
}

impl<'a> RequestClient<'a> {
    pub fn get_instance() -> &'static Self {
        static INSTANCE: OnceLock<RequestClient> = OnceLock::new();

        INSTANCE.get_or_init(|| {
            let listener = Observer::new();
            RequestClient {
                listener,
                proxy: RequestProxy::get_instance(),
            }
        })
    }

    pub fn crate_task(
        &self,
        context: Context,
        version: Version,
        mut config: TaskConfig,
        save_as: &str,
        overwrite: bool,
    ) -> Result<i64, CreateTaskError> {
        info!("Creating task with config: {:?}", config);

        let path = check::file::get_download_path(version, &context, &save_as, overwrite)?;
        info!("Download file path: {:?}", path);

        let file_specs = FileSpec {
            name: "".to_string(),
            path: path.to_string_lossy().to_string(),
            file_name: path
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| path.to_string_lossy().to_string()),
            mime_type: "".to_string(),
            is_user_file: false,
            fd: None,
        };
        config.file_specs.push(file_specs);
        self.open_channel();
        self.proxy.create(config)
    }

    pub fn start(&self, task_id: i64) -> Result<(), i32> {
        self.proxy.start(task_id)
    }

    pub fn pause(&self, task_id: i64) -> Result<(), i32> {
        self.proxy.pause(task_id)
    }

    pub fn resume(&self, task_id: i64) -> Result<(), i32> {
        self.proxy.resume(task_id)
    }

    pub fn remove(&self, task_id: i64) -> Result<(), i32> {
        self.proxy.remove(task_id)
    }

    pub fn stop(&self, task_id: i64) -> Result<(), i32> {
        self.proxy.stop(task_id)
    }

    pub fn set_max_speed(&self, task_id: i64, speed: i64) -> Result<(), i32> {
        self.proxy.set_max_speed(task_id, speed)
    }

    pub fn register_callback(
        &self,
        task_id: i64,
        callback: Arc<dyn Callback + Send + Sync + 'static>,
    ) {
        self.listener.register_callback(task_id, callback);
    }

    pub fn open_channel(&self) {
        let file = self.proxy.open_channel().unwrap();
        self.listener.set_listenr(file);
    }

    pub fn show_task(&self, task_id: i64) -> Result<TaskInfo, i32> {
        self.proxy.show(task_id)
    }
}
