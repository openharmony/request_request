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

pub(crate) use ffi::TaskFilter;

use crate::manage::account::GetDatabaseInstance;
use crate::manage::TaskManager;

impl TaskManager {
    pub(crate) fn search(&self, filter: TaskFilter, method: SearchMethod) -> Vec<u32> {
        debug!("Search task by filter: {:?} method: {:?}", filter, method);

        let database = GetDatabaseInstance();

        match method {
            SearchMethod::User(uid) => unsafe { (*database).SearchTask(filter, uid) },
            SearchMethod::System(bundle_name) => unsafe {
                (*database).SystemSearchTask(filter, bundle_name.as_str())
            },
        }
    }
}

#[derive(Debug)]
pub(crate) enum SearchMethod {
    User(u64),
    System(String),
}

#[allow(unreachable_pub)]
#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {

    #[derive(Debug)]
    struct TaskFilter {
        before: i64,
        after: i64,
        state: u8,
        action: u8,
        mode: u8,
    }

    unsafe extern "C++" {
        include!("c_request_database.h");
        type RequestDataBase = crate::manage::account::RequestDataBase;

        fn SearchTask(self: &RequestDataBase, filter: TaskFilter, uid: u64) -> Vec<u32>;

        fn SystemSearchTask(
            self: &RequestDataBase,
            filter: TaskFilter,
            bundle_name: &str,
        ) -> Vec<u32>;
    }
}
