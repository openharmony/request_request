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

use crate::data::observer::DirRebuilder;
use crate::data::{init_history_store_dir, is_history_init, HistoryDir};
use crate::wrapper::ffi::{NewDirectoryMonitor, StartObserve};
use cxx::let_cxx_string;
use std::{path::PathBuf, sync::Arc};

pub fn observe_image_file_delete(path: String) {
    if !is_history_init() {
        let image_path = PathBuf::from(path);
        let history = Arc::new(HistoryDir::new(image_path));
        init_history_store_dir(history.clone(), start_history_dir_observe);
    }
}

pub fn start_history_dir_observe(curr: PathBuf, history: Arc<HistoryDir>) {
    ffrt_rs::ffrt_spawn(move || {
        if let Some(image_dir) = history.dir_path() {
            let_cxx_string!(image_dir = image_dir);
            let rebuilder = Box::new(DirRebuilder::new(curr, history));
            let mut monitor = NewDirectoryMonitor(&image_dir, rebuilder);
            if let Some(ptr) = monitor.as_mut() {
                StartObserve(ptr);
            }
        }
    });
}
