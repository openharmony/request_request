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

use std::ops::Deref;
use std::sync::Arc;

use crate::task::request_task::RequestTask;

pub(crate) struct NotifyTask {
    task: Arc<RequestTask>,
}

impl NotifyTask {
    pub(crate) fn new(task: Arc<RequestTask>) -> Self {
        Self { task }
    }
}

impl Deref for NotifyTask {
    type Target = Arc<RequestTask>;

    fn deref(&self) -> &Self::Target {
        &self.task
    }
}

impl Drop for NotifyTask {
    fn drop(&mut self) {
        self.task.state_change_notify();
    }
}
