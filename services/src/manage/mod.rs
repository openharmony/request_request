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
    pub(crate) mod config;
    pub(crate) use config::{SystemConfig, SystemConfigManager};
}

pub(crate) mod account;
pub(crate) mod app_state;
pub(crate) mod database;
pub(crate) mod events;
pub(crate) mod query;
pub(crate) use task_manager::TaskManager;
pub(crate) mod network;
pub(crate) mod network_manager;
pub(crate) mod notifier;
pub(crate) mod scheduler;
pub(crate) mod task_manager;

#[cfg(test)]
mod ut_mod {
    include!("../../tests/ut/manage/ut_mod.rs");
}
