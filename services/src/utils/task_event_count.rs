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

use std::sync::{Arc, Mutex, Once};

struct RequestTaskCount {
    completed_task_count: i32,
    failed_task_count: i32,
    load_state: bool,
}

impl RequestTaskCount {
    fn get_instance() -> Arc<Mutex<RequestTaskCount>> {
        static mut TASK_COUNT: Option<Arc<Mutex<RequestTaskCount>>> = None;
        static ONCE: Once = Once::new();
        ONCE.call_once(|| {
            unsafe {
                TASK_COUNT = Some(Arc::new(Mutex::new(RequestTaskCount {
                    completed_task_count: 0,
                    failed_task_count: 0,
                    load_state: false,
                })))
            };
        });

        unsafe { TASK_COUNT.as_ref().unwrap().clone() }
    }
}

pub(crate) fn task_complete_add() {
    let instance = RequestTaskCount::get_instance();
    let mut task_count = instance.lock().unwrap();
    task_count.completed_task_count += 1;
    task_count.load_state = true;
}

pub(crate) fn task_fail_add() {
    let instance = RequestTaskCount::get_instance();
    let mut task_count = instance.lock().unwrap();
    task_count.failed_task_count += 1;
    task_count.load_state = true;
}

pub(crate) fn task_unload() {
    let instance = RequestTaskCount::get_instance();
    let mut task_count = instance.lock().unwrap();
    if task_count.load_state {
        let completed = task_count.completed_task_count;
        let failed = task_count.failed_task_count;
        sys_event!(
            ExecError,
            DfxCode::TASK_STATISTICS,
            &format!("Task Completed {}, failed {}", completed, failed)
        );
        task_count.completed_task_count = 0;
        task_count.failed_task_count = 0;
        task_count.load_state = false;
    }
}
