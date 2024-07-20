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

use crate::ability::SYSTEM_CONFIG_MANAGER;
use crate::config::Mode;
use crate::error::ErrorCode;
use crate::manage::app_state::AppState;
use crate::manage::database::{Database, RequestDb};
use crate::manage::TaskManager;
use crate::task::config::TaskConfig;
use crate::task::request_task::{check_config, RequestTask};
use crate::utils::query_app_state;
use crate::utils::task_id_generator::TaskIdGenerator;

const MAX_BACKGROUND_TASK: usize = 1000;
const MAX_FRONTEND_TASK: usize = 2000;

impl TaskManager {
    pub(crate) async fn create(&mut self, mut config: TaskConfig) -> Result<u32, ErrorCode> {
        let task_id = TaskIdGenerator::generate();
        config.common_data.task_id = task_id;

        let uid = config.common_data.uid;
        let version = config.version;

        debug!(
            "TaskManager Construct, uid:{}, task_id:{}, version:{:?}",
            uid, task_id, version
        );

        let mut database = RequestDb::get_instance();

        match config.common_data.mode {
            Mode::BackGround => {
                if database.query_app_uncompleted_task_num(uid, Mode::BackGround)
                    >= MAX_BACKGROUND_TASK
                {
                    debug!("TaskManager background enqueue error");
                    return Err(ErrorCode::TaskEnqueueErr);
                }
            }
            _ => {
                if database.query_app_uncompleted_task_num(uid, Mode::FrontEnd) >= MAX_FRONTEND_TASK
                {
                    debug!("TaskManager frontend enqueue error");
                    return Err(ErrorCode::TaskEnqueueErr);
                }
            }
        }

        let state = query_app_state(uid);
        let app_state = AppState::new(uid, state, self.app_state_manager.clone());

        let system_config = unsafe { SYSTEM_CONFIG_MANAGER.assume_init_ref().system_config() };

        let (files, client) = check_config(&config, system_config)?;
        let task = RequestTask::new(
            config,
            app_state,
            files,
            client,
            self.client_manager.clone(),
            self.network.clone(),
        );
        // New task: State::Initialized, Reason::Default

        Database::get_instance().insert_task(task);
        Ok(task_id)
    }
}
