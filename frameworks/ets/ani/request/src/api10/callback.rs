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

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use ani_rs::objects::{AniFnObject, GlobalRefCallback};
use ani_rs::AniEnv;
use request_client::RequestClient;
use request_core::info::{Progress, Response};

use crate::api10::bridge::{self, Task};

#[ani_rs::native]
pub fn on_event(
    env: &AniEnv,
    this: Task,
    event: String,
    callback: AniFnObject,
) -> Result<(), ani_rs::business_error::BusinessError> {
    let task_id = this.tid.parse().unwrap();
    info!("on_event called with event: {}", event);
    let callback_mgr = CallbackManager::get_instance();
    let callback = callback.into_global_callback(env).unwrap();
    let coll = match event.as_str() {
        "completed" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_complete.lock().unwrap().push(callback);
                return Ok(());
            } else {
                Arc::new(CallbackColl {
                    on_progress: Mutex::new(vec![]),
                    on_complete: Mutex::new(vec![callback]),
                    on_pause: Mutex::new(vec![]),
                    on_resume: Mutex::new(vec![]),
                    on_remove: Mutex::new(vec![]),
                    on_fail: Mutex::new(vec![]),
                    on_response: Mutex::new(vec![]),
                })
            }
        }
        "pause" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_pause.lock().unwrap().push(callback);
                return Ok(());
            } else {
                Arc::new(CallbackColl {
                    on_progress: Mutex::new(vec![]),
                    on_complete: Mutex::new(vec![]),
                    on_pause: Mutex::new(vec![callback]),
                    on_resume: Mutex::new(vec![]),
                    on_remove: Mutex::new(vec![]),
                    on_fail: Mutex::new(vec![]),
                    on_response: Mutex::new(vec![]),
                })
            }
        }
        "failed" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_fail.lock().unwrap().push(callback);
                return Ok(());
            } else {
                Arc::new(CallbackColl {
                    on_progress: Mutex::new(vec![]),
                    on_complete: Mutex::new(vec![]),
                    on_pause: Mutex::new(vec![]),
                    on_resume: Mutex::new(vec![]),
                    on_remove: Mutex::new(vec![]),
                    on_fail: Mutex::new(vec![callback]),
                    on_response: Mutex::new(vec![]),
                })
            }
        }
        "remove" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_remove.lock().unwrap().push(callback);
                return Ok(());
            } else {
                Arc::new(CallbackColl {
                    on_progress: Mutex::new(vec![]),
                    on_complete: Mutex::new(vec![]),
                    on_pause: Mutex::new(vec![]),
                    on_resume: Mutex::new(vec![]),
                    on_remove: Mutex::new(vec![callback]),
                    on_fail: Mutex::new(vec![]),
                    on_response: Mutex::new(vec![]),
                })
            }
        }
        "progress" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_progress.lock().unwrap().push(callback);
                return Ok(());
            } else {
                Arc::new(CallbackColl {
                    on_progress: Mutex::new(vec![callback]),
                    on_complete: Mutex::new(vec![]),
                    on_pause: Mutex::new(vec![]),
                    on_resume: Mutex::new(vec![]),
                    on_remove: Mutex::new(vec![]),
                    on_fail: Mutex::new(vec![]),
                    on_response: Mutex::new(vec![]),
                })
            }
        }
        "resume" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_resume.lock().unwrap().push(callback);
                return Ok(());
            } else {
                Arc::new(CallbackColl {
                    on_progress: Mutex::new(vec![]),
                    on_complete: Mutex::new(vec![]),
                    on_pause: Mutex::new(vec![]),
                    on_resume: Mutex::new(vec![callback]),
                    on_remove: Mutex::new(vec![]),
                    on_fail: Mutex::new(vec![]),
                    on_response: Mutex::new(vec![]),
                })
            }
        }
        _ => unimplemented!(),
    };
    RequestClient::get_instance().register_callback(task_id, coll.clone());
    callback_mgr.tasks.lock().unwrap().insert(task_id, coll);
    Ok(())
}

#[ani_rs::native]
pub fn on_response_event(
    env: &AniEnv,
    this: Task,
    event: String,
    callback: AniFnObject,
) -> Result<(), ani_rs::business_error::BusinessError> {
    let task_id = this.tid.parse().unwrap();
    info!("on_event called with event: {}", event);
    let callback_mgr = CallbackManager::get_instance();
    let callback = callback.into_global_callback(env).unwrap();
    let coll = match event.as_str() {
        "response" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_response.lock().unwrap().push(callback);
                return Ok(());
            } else {
                Arc::new(CallbackColl {
                    on_progress: Mutex::new(vec![]),
                    on_complete: Mutex::new(vec![]),
                    on_pause: Mutex::new(vec![]),
                    on_resume: Mutex::new(vec![]),
                    on_remove: Mutex::new(vec![]),
                    on_fail: Mutex::new(vec![]),
                    on_response: Mutex::new(vec![callback]),
                })
            }
        }
        _ => unimplemented!(),
    };
    RequestClient::get_instance().register_callback(task_id, coll.clone());
    callback_mgr.tasks.lock().unwrap().insert(task_id, coll);
    Ok(())
}

pub struct CallbackColl {
    on_progress: Mutex<Vec<GlobalRefCallback<(bridge::Progress,)>>>,
    on_complete: Mutex<Vec<GlobalRefCallback<(bridge::Progress,)>>>,
    on_pause: Mutex<Vec<GlobalRefCallback<(bridge::Progress,)>>>,
    on_resume: Mutex<Vec<GlobalRefCallback<(bridge::Progress,)>>>,
    on_remove: Mutex<Vec<GlobalRefCallback<(bridge::Progress,)>>>,
    on_fail: Mutex<Vec<GlobalRefCallback<(bridge::Progress,)>>>,
    on_response: Mutex<Vec<GlobalRefCallback<(bridge::HttpResponse,)>>>,
}

impl request_client::Callback for CallbackColl {
    fn on_progress(&self, progress: &Progress) {
        let callbacks = self.on_progress.lock().unwrap();
        for callback in callbacks.iter() {
            callback.execute((progress.into(),));
        }
    }

    fn on_completed(&self, progress: &Progress) {
        let callbacks = self.on_complete.lock().unwrap();
        for callback in callbacks.iter() {
            callback.execute((progress.into(),));
        }
    }

    fn on_pause(&self, progress: &Progress) {
        let callbacks = self.on_pause.lock().unwrap();
        for callback in callbacks.iter() {
            callback.execute((progress.into(),));
        }
    }

    fn on_resume(&self, progress: &Progress) {
        let callbacks = self.on_resume.lock().unwrap();
        for callback in callbacks.iter() {
            callback.execute((progress.into(),));
        }
    }

    fn on_remove(&self, progress: &Progress) {
        let callbacks = self.on_remove.lock().unwrap();
        for callback in callbacks.iter() {
            callback.execute((progress.into(),));
        }
    }

    fn on_response(&self, response: &Response) {
        let callbacks = self.on_response.lock().unwrap();
        for callback in callbacks.iter() {
            callback.execute((response.into(),));
        }
    }

    fn on_failed(&self, progress: &Progress, _error_code: i32)  {
        let callbacks = self.on_fail.lock().unwrap();
        for callback in callbacks.iter() {
            callback.execute((progress.into(),));
        }
    }
}

pub struct CallbackManager {
    tasks: Mutex<HashMap<i64, Arc<CallbackColl>>>,
}

impl CallbackManager {
    pub fn get_instance() -> &'static Self {
        static INSTANCE: OnceLock<CallbackManager> = OnceLock::new();

        INSTANCE.get_or_init(|| CallbackManager {
            tasks: Mutex::new(HashMap::new()),
        })
    }
}
