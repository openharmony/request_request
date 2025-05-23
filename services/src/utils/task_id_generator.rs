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

cfg_oh! {
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};
    use crate::manage::database::RequestDb;
}

pub(crate) struct TaskIdGenerator;

impl TaskIdGenerator {
    #[cfg(feature = "oh")]
    pub(crate) fn generate() -> u32 {
        loop {
            debug!("generate task_id");
            let task_id = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(time) => time.subsec_nanos(),
                Err(e) => {
                    static ID: AtomicU32 = AtomicU32::new(0);
                    error!("Generate task id from system time failed {:?}", e);
                    sys_event!(
                        ExecFault,
                        DfxCode::SA_ERROR_00,
                        &format!("Generate task id from system time failed {:?}", e)
                    );
                    ID.fetch_add(1, Ordering::Relaxed)
                }
            };
            if !RequestDb::get_instance().contains_task(task_id) {
                return task_id;
            }
        }
    }
    #[cfg(not(feature = "oh"))]
    pub(crate) fn generate() -> u32 {
        rand::random()
    }
}
