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

mod file;
mod ram;
mod space;

pub mod observer;

pub use file::{
    get_curr_store_dir, init_curr_store_dir, init_history_store_dir, is_history_init, FileStoreDir,
    HistoryDir,
};
pub(crate) use file::{restore_files, FileCache};
pub use ram::RamCache;
pub(crate) use space::ResourceManager;

pub(crate) const MAX_CACHE_SIZE: u64 = 20971520;
