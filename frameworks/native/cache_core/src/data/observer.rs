// Copyright (C) 2025 Huawei Device Co., Ltd.
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

use std::{fs, path::PathBuf, sync::Arc};

use crate::data::file::HistoryDir;

pub struct DirRebuilder {
    curr: PathBuf,
    history: Arc<HistoryDir>,
}

impl DirRebuilder {
    pub fn new(curr: PathBuf, history: Arc<HistoryDir>) -> Self {
        Self { curr, history }
    }

    pub fn recreate_store_dir(&self) {
        if self.curr.is_dir() {
            // Don't care about the failed deletion.
            if let Err(e) = fs::remove_dir_all(self.curr.as_path()) {
                error!("remove local store directory fail, err: {:?}", e);
            };
        }
        if let Err(e) = fs::create_dir_all(self.curr.as_path()) {
            error!("recreate local store directory fail, err: {:?}", e);
        };
    }

    pub fn history_exist_or_create(&self) -> bool {
        if !self.history.exist() {
            return self.history.create();
        }
        true
    }

    pub fn stop_history_observe(&self) {
        self.history.stop_observe();
    }
}

impl Drop for DirRebuilder {
    fn drop(&mut self) {
        self.stop_history_observe();
    }
}
