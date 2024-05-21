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

use crate::manage::database::Database;
use crate::manage::TaskManager;
use crate::utils::filter::Filter;

impl TaskManager {
    pub(crate) fn search(&self, filter: Filter) -> Vec<u32> {
        debug!("TaskManager Search, filter:{:?}", filter);
        Database::get_instance().search_tasks(filter)
    }
}
