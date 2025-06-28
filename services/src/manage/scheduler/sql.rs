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

use crate::config::{Action, Mode};
use crate::info::State;
use crate::task::reason::Reason;

pub(super) fn start_task(task_id: u32) -> String {
    format!(
        "UPDATE request_task SET state = {}, reason = {} where task_id = {} AND (state = {} OR state = {} OR (action = {} AND (state = {} OR state = {} )))",
        State::Waiting.repr,
        Reason::RunningTaskMeetLimits.repr,
        task_id,
        State::Initialized.repr,
        State::Paused.repr,
        Action::Download.repr,
        State::Failed.repr,
        State::Stopped.repr,
    )
}

pub(super) fn start_tasks(task_ids: &[u32]) -> String {
    format!(
        "UPDATE request_task SET state = {}, reason = {} where task_id IN ({})",
        State::Waiting.repr,
        Reason::RunningTaskMeetLimits.repr,
        task_ids
            .iter()
            .map(|&id| id.to_string())
            .collect::<Vec<_>>()
            .join(", "),
    )
}

pub(super) fn pause_task(task_id: u32) -> String {
    format!(
        "UPDATE request_task SET state = {}, reason = {} where task_id = {} AND (state = {} OR state = {} OR state = {})",
        State::Paused.repr,
        Reason::UserOperation.repr,
        task_id,
        State::Running.repr,
        State::Retrying.repr,
        State::Waiting.repr,
    )
}

pub(super) fn pause_tasks(task_ids: &[u32]) -> String {
    format!(
        "UPDATE request_task SET state = {}, reason = {} where task_id IN ({})",
        State::Paused.repr,
        Reason::UserOperation.repr,
        task_ids
            .iter()
            .map(|&id| id.to_string())
            .collect::<Vec<_>>()
            .join(", "),
    )
}

pub(super) fn stop_task(task_id: u32) -> String {
    format!(
        "UPDATE request_task SET state = {}, reason = {} where task_id = {} AND (state = {} OR state = {} OR state = {})",
        State::Stopped.repr,
        Reason::UserOperation.repr,
        task_id,
        State::Running.repr,
        State::Retrying.repr,
        State::Waiting.repr,
    )
}

pub(super) fn stop_tasks(task_ids: &[u32]) -> String {
    format!(
        "UPDATE request_task SET state = {}, reason = {} where task_id IN ({})",
        State::Stopped.repr,
        Reason::UserOperation.repr,
        task_ids
            .iter()
            .map(|&id| id.to_string())
            .collect::<Vec<_>>()
            .join(", "),
    )
}

pub(super) fn remove_task(task_id: u32) -> String {
    format!(
        "UPDATE request_task SET state = {}, reason = {} where task_id = {}",
        State::Removed.repr,
        Reason::UserOperation.repr,
        task_id,
    )
}

pub(super) fn remove_tasks(task_ids: &[u32]) -> String {
    format!(
        "UPDATE request_task SET state = {}, reason = {} where task_id IN ({})",
        State::Removed.repr,
        Reason::UserOperation.repr,
        task_ids
            .iter()
            .map(|&id| id.to_string())
            .collect::<Vec<_>>()
            .join(", "),
    )
}

pub(super) fn task_set_mode(task_id: u32, mode: Mode) -> String {
    format!(
        "UPDATE request_task SET mode = {} where task_id = {}",
        mode.repr, task_id,
    )
}

#[cfg(all(not(feature = "oh"), test))]
mod ut_sql {
    include!("../../../tests/ut/manage/scheduler/ut_sql.rs");
}
