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

use ani_rs::business_error::BusinessError;
use ani_rs::objects::{AniFnObject, GlobalRefCallback};
use ani_rs::AniEnv;
use request_client::RequestClient;
use request_core::info::Progress;

use crate::api9::bridge::DownloadTask;

#[ani_rs::native]
pub fn on_progress(
    env: &AniEnv,
    this: DownloadTask,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    info!("on_progress called for task_id: {}", this.task_id);
    let callback_mgr = CallbackManager::get_instance();
    let callback = callback.into_global_callback(env).unwrap();
    if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&this.task_id) {
        coll.on_progress.lock().unwrap().push(callback);
    } else {
        let coll = Arc::new(CallbackColl {
            on_progress: Mutex::new(vec![callback]),
            on_complete: Mutex::new(vec![]),
            on_pause: Mutex::new(vec![]),
            on_resume: Mutex::new(vec![]),
            on_fail: Mutex::new(vec![]),
        });
        RequestClient::get_instance().register_callback(this.task_id, coll.clone());
        callback_mgr
            .tasks
            .lock()
            .unwrap()
            .insert(this.task_id, coll);
    }
    Ok(())
}

#[ani_rs::native]
pub fn on_event(
    env: &AniEnv,
    this: DownloadTask,
    event: String,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    let callback_mgr = CallbackManager::get_instance();
    let callback = callback.into_global_callback(env).unwrap();
    info!(
        "on_event called for task_id: {}, event: {}",
        this.task_id, event
    );
    let coll = if event == "complete" {
        if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&this.task_id) {
            coll.on_complete.lock().unwrap().push(callback);
            return Ok(());
        } else {
            Arc::new(CallbackColl {
                on_progress: Mutex::new(vec![]),
                on_complete: Mutex::new(vec![callback]),
                on_pause: Mutex::new(vec![]),
                on_resume: Mutex::new(vec![]),
                on_fail: Mutex::new(vec![]),
            })
        }
    } else if event == "pause" {
        if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&this.task_id) {
            coll.on_pause.lock().unwrap().push(callback);
            return Ok(());
        } else {
            Arc::new(CallbackColl {
                on_progress: Mutex::new(vec![]),
                on_complete: Mutex::new(vec![]),
                on_pause: Mutex::new(vec![callback]),
                on_resume: Mutex::new(vec![]),
                on_fail: Mutex::new(vec![]),
            })
        }
    } else if event == "resume" {
        if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&this.task_id) {
            coll.on_resume.lock().unwrap().push(callback);
            return Ok(());
        } else {
            Arc::new(CallbackColl {
                on_progress: Mutex::new(vec![]),
                on_complete: Mutex::new(vec![]),
                on_pause: Mutex::new(vec![]),
                on_resume: Mutex::new(vec![callback]),
                on_fail: Mutex::new(vec![]),
            })
        }
    } else {
        return Err(BusinessError::new(
            -1,
            format!("Unsupported event type: {}", event),
        ));
    };
    RequestClient::get_instance().register_callback(this.task_id, coll.clone());
    callback_mgr
        .tasks
        .lock()
        .unwrap()
        .insert(this.task_id, coll);
    Ok(())
}

#[ani_rs::native]
pub fn on_fail(
    env: &AniEnv,
    this: DownloadTask,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    let callback_mgr = CallbackManager::get_instance();
    let callback = callback.into_global_callback(env).unwrap();
    if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&this.task_id) {
        coll.on_fail.lock().unwrap().push(callback);
    } else {
        let coll = Arc::new(CallbackColl {
            on_progress: Mutex::new(vec![]),
            on_complete: Mutex::new(vec![]),
            on_pause: Mutex::new(vec![]),
            on_resume: Mutex::new(vec![]),
            on_fail: Mutex::new(vec![callback]),
        });
        RequestClient::get_instance().register_callback(this.task_id, coll.clone());

        callback_mgr
            .tasks
            .lock()
            .unwrap()
            .insert(this.task_id, coll);
    }
    Ok(())
}

pub struct CallbackColl {
    on_progress: Mutex<Vec<GlobalRefCallback<(i64, i64)>>>,
    on_complete: Mutex<Vec<GlobalRefCallback<()>>>,
    on_pause: Mutex<Vec<GlobalRefCallback<()>>>,
    on_resume: Mutex<Vec<GlobalRefCallback<()>>>,
    on_fail: Mutex<Vec<GlobalRefCallback<(i32,)>>>,
}

impl request_client::Callback for CallbackColl {
    fn on_progress(&self, progress: &Progress) {
        let callbacks = self.on_progress.lock().unwrap();
        for callback in callbacks.iter() {
            callback.execute((progress.processed as i64, progress.sizes[0]));
        }
    }

    fn on_completed(&self, _progress: &Progress) {
        let callbacks = self.on_complete.lock().unwrap();
        for callback in callbacks.iter() {
            callback.execute(());
        }
    }

    fn on_failed(&self, _progress: &Progress, error_code: i32) {
        let callbacks = self.on_fail.lock().unwrap();
        for callback in callbacks.iter() {
            callback.execute((error_code,));
        }
    }

    fn on_pause(&self, _progress: &Progress) {
        let callbacks = self.on_pause.lock().unwrap();
        for callback in callbacks.iter() {
            callback.execute(());
        }
    }

    fn on_resume(&self, _progress: &Progress) {
        let callbacks = self.on_resume.lock().unwrap();
        for callback in callbacks.iter() {
            callback.execute(());
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
