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

use crate::agent::{CustomCallback, DownloadRequest, TaskId};
use crate::download::{download, TaskHandle};

pub(crate) struct Updater {
    pub(crate) remove_flag: bool,
    pub(crate) seq: usize,
    pub(crate) handle: TaskHandle,
}

impl Updater {
    pub(crate) fn new(
        task_id: TaskId,
        request: DownloadRequest,
        callback: Box<dyn CustomCallback>,
        seq: usize,
    ) -> Self {
        info!("new preload task {} seq {}", task_id.brief(), seq);
        let task_handle = download(task_id, request, Some(callback), seq);
        Self {
            handle: task_handle,
            remove_flag: false,
            seq,
        }
    }

    pub(crate) fn cancel(&self) {
        self.handle.cancel();
    }

    pub(crate) fn task_handle(&self) -> TaskHandle {
        self.handle.clone()
    }

    pub(crate) fn try_add_callback(
        &mut self,
        callback: Box<dyn CustomCallback>,
    ) -> Result<(), Box<dyn CustomCallback>> {
        self.handle.try_add_callback(callback)
    }
}
