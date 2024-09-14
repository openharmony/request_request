// Copyright (C) 2023 Huawei Device Co., Ltd.
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

cfg_oh! {
    use crate::ability::SYSTEM_CONFIG_MANAGER;
}

use crate::config::Mode;
use crate::error::ErrorCode;
use crate::manage::database::RequestDb;
use crate::manage::TaskManager;
use crate::task::config::TaskConfig;
use crate::task::request_task::{check_config, RequestTask};
use crate::utils::task_id_generator::TaskIdGenerator;

const MAX_BACKGROUND_TASK: usize = 1000;
const MAX_FRONTEND_TASK: usize = 2000;

impl TaskManager {
    pub(crate) fn create(&mut self, mut config: TaskConfig) -> Result<u32, ErrorCode> {
        let task_id = TaskIdGenerator::generate();
        config.common_data.task_id = task_id;

        let uid = config.common_data.uid;
        let version = config.version;

        debug!(
            "TaskManager Construct, uid:{}, task_id:{}, version:{:?}",
            uid, task_id, version
        );

        let (frontend, background) = self
            .task_count
            .entry(config.common_data.uid)
            .or_insert((0, 0));

        let (task_count, limit) = match config.common_data.mode {
            Mode::FrontEnd => (frontend, MAX_FRONTEND_TASK),
            _ => (background, MAX_BACKGROUND_TASK),
        };

        if *task_count > limit {
            error!(
                "{} task count {} exceeds the limit {}",
                uid, task_count, limit
            );
            return Err(ErrorCode::TaskEnqueueErr);
        } else {
            *task_count += 1;
        }

        #[cfg(feature = "oh")]
        let system_config = unsafe { SYSTEM_CONFIG_MANAGER.assume_init_ref().system_config() };

        let (files, client) = check_config(
            &config,
            #[cfg(feature = "oh")]
            system_config,
        )?;
        let task = RequestTask::new(
            config,
            files,
            client,
            self.client_manager.clone(),
            self.network.clone(),
        );
        // New task: State::Initialized, Reason::Default

        RequestDb::get_instance().insert_task(task);
        Ok(task_id)
    }
}
