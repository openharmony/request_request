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

use crate::error::ErrorCode;
use crate::init::SYSTEM_CONFIG_MANAGER;
use crate::manage::app_state::{AppState, GetTopBundleName};
use crate::manage::database::Database;
use crate::manage::TaskManager;
use crate::task::config::TaskConfig;
use crate::task::info::{ApplicationState, Mode, State};
use crate::task::reason::Reason;
use crate::task::request_task::RequestTask;
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

        let database = Database::get_instance();

        match config.common_data.mode {
            Mode::BackGround => {
                if database.app_uncompleted_tasks_num(uid, Mode::BackGround) == MAX_BACKGROUND_TASK
                {
                    debug!("TaskManager background enqueue error");
                    return Err(ErrorCode::TaskEnqueueErr);
                }
            }
            _ => {
                if database.app_uncompleted_tasks_num(uid, Mode::FrontEnd) == MAX_FRONTEND_TASK {
                    debug!("TaskManager frontend enqueue error");
                    return Err(ErrorCode::TaskEnqueueErr);
                }
            }
        }

        // Here we don not need to run the task, just add it to database.
        let top_bundle = unsafe { GetTopBundleName() }.to_string();
        let app_state = if top_bundle == config.bundle {
            AppState::new(
                uid,
                ApplicationState::Foreground,
                self.app_state_manager.clone(),
            )
        } else {
            AppState::new(
                uid,
                ApplicationState::Background,
                self.app_state_manager.clone(),
            )
        };
        let system_config = unsafe { SYSTEM_CONFIG_MANAGER.assume_init_ref().system_config() };
        let task = match RequestTask::new(
            config,
            system_config,
            app_state,
            None,
            self.client_manager.clone(),
        ) {
            Ok(task) => task,
            Err(e) => return Err(e),
        };

        task.set_status(State::Initialized, Reason::Default);

        database.insert_task(task);

        Ok(task_id)
    }
}
