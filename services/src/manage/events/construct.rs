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

//! Task creation and initialization functionality for the request service.
//! 
//! This module implements the task construction logic for the `TaskManager`,
//! handling task creation, validation, and initialization. It enforces task limits
//! based on task type and manages system configuration integration.

cfg_oh! {
    use crate::ability::SYSTEM_CONFIG_MANAGER;
}

use crate::config::Mode;
use crate::error::ErrorCode;
use crate::manage::database::RequestDb;
use crate::manage::TaskManager;
use crate::task::config::TaskConfig;
use crate::task::request_task::{check_config, get_rest_time, RequestTask};
use crate::utils::task_id_generator::TaskIdGenerator;

/// Maximum number of background tasks allowed per user ID.
///
/// Includes tasks with mode `Mode::Background` and starts counting from 0.
const MAX_BACKGROUND_TASK: usize = 1001;

/// Maximum number of frontend tasks allowed per user ID.
///
/// Includes tasks with mode `Mode::FrontEnd` and starts counting from 0.
const MAX_FRONTEND_TASK: usize = 2001;

impl TaskManager {
    /// Creates a new request task with the provided configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Task configuration containing request details, user information,
    ///   and execution parameters.
    ///
    /// # Returns
    ///
    /// * `Ok(u32)` - The generated task ID if creation is successful.
    /// * `Err(ErrorCode)` - An error code if task creation fails, such as exceeding task limits
    ///   or invalid configuration.
    ///
    /// # Panics
    ///
    /// Panics if `check_config` or task creation fails in unexpected ways.
    ///
    /// # Notes
    ///
    /// This method enforces task limits based on mode and user ID, generates a unique task ID,
    /// validates the configuration, and initializes a new task.
    pub(crate) fn create(&mut self, mut config: TaskConfig) -> Result<u32, ErrorCode> {
        // Generate a unique task ID and assign it to the configuration
        let task_id = TaskIdGenerator::generate();
        config.common_data.task_id = task_id;

        // Extract user ID and version for logging and validation
        let uid = config.common_data.uid;
        let version = config.version;

        debug!(
            "TaskManager construct uid{} tid{}, version:{:?}",
            uid, task_id, version
        );

        // Get or initialize task counters for this user ID
        let (frontend, background) = self
            .task_count
            .entry(config.common_data.uid)
            .or_insert((0, 0));

        // Determine which counter and limit to use based on task mode
        let (task_count, limit) = match config.common_data.mode {
            Mode::FrontEnd => (frontend, MAX_FRONTEND_TASK),
            _ => (background, MAX_BACKGROUND_TASK),
        };

        // The loop starts counting from 0 and ends at limit, not exceeding limit.
        if *task_count >= limit {
            error!(
                "{} task count {} exceeds the limit {}",
                uid, task_count, limit
            );
            return Err(ErrorCode::TaskEnqueueErr);
        } else {
            // Increment the appropriate task counter
            *task_count += 1;
        }

        // Get system configuration for certificate and proxy settings
        #[cfg(feature = "oh")]
        let system_config = unsafe { SYSTEM_CONFIG_MANAGER.assume_init_ref().system_config() };

        // Calculate remaining time and validate task configuration
        let rest_time = get_rest_time(&config, 0);
        let (files, client) = check_config(
            &config,
            rest_time,
            #[cfg(feature = "oh")]
            system_config,
        )?;
        // Create a new request task with validated configuration and resources
        let task = RequestTask::new(
            config,
            files,
            client,
            self.client_manager.clone(),
            false,
            rest_time,
        );
        // New task: State::Initialized, Reason::Default
        // Insert the new task into the database for persistence
        RequestDb::get_instance().insert_task(task);
        Ok(task_id)
    }
}
