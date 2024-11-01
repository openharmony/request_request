// Copyright (C) 2024 Huawei Device Co., Ltd.
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

use crate::agent::CustomCallback;
use crate::download::{download, CancelHandle, TaskHandle};

pub(crate) struct Updater {
    handle: TaskHandle,
}

impl Updater {
    pub(crate) fn new(task_id: u64, url: &str, callback: Box<dyn CustomCallback>) -> Self {
        let task_handle = download(task_id, url, Some(callback));
        Self {
            handle: task_handle,
        }
    }

    pub(crate) fn try_add_callback(
        &mut self,
        callback: Box<dyn CustomCallback>,
    ) -> Result<(), Box<dyn CustomCallback>> {
        self.handle.try_add_callback(callback)
    }

    pub(crate) fn cancel(&mut self) {
        self.handle.cancel();
    }

    pub(crate) fn cancel_handle(&self) -> CancelHandle {
        self.handle.cancel_handle()
    }
}
