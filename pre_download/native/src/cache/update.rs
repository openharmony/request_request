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

use std::sync::Mutex;

use crate::agent::{CustomCallback, DownloadRequest, TaskId};
use crate::download::{download, TaskHandle};

pub(crate) struct Updater {
    pub(crate) handle: Mutex<TaskHandle>,
}

impl Updater {
    pub(crate) fn new(
        task_id: TaskId,
        request: DownloadRequest,
        callback: Box<dyn CustomCallback>,
    ) -> Self {
        let task_handle = download(task_id, request, Some(callback));
        Self {
            handle: Mutex::new(task_handle),
        }
    }

    pub(crate) fn cancel(&self) {
        self.handle.lock().unwrap().cancel();
    }

    pub(crate) fn task_handle(&self) -> TaskHandle {
        self.handle.lock().unwrap().clone()
    }
}
