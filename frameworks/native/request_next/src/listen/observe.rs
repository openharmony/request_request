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
use std::fs::File;
use std::sync::{Arc, Mutex};

use request_core::info::{Progress, SubscribeType, Response};
use ylong_runtime::task::JoinHandle;

use crate::listen::uds::{Message, UdsListener};

pub struct Observer {
    callbacks: Arc<Mutex<HashMap<i64, Arc<dyn Callback + Send + Sync + 'static>>>>,
    listener: Mutex<Option<JoinHandle<()>>>,
}

pub trait Callback {
    fn on_progress(&self, progress: &Progress) {}
    fn on_completed(&self, progress: &Progress) {}
    fn on_failed(&self, progress: &Progress, error_code: i32) {}
    fn on_pause(&self, progress: &Progress) {}
    fn on_resume(&self, progress: &Progress) {}
    fn on_remove(&self, progress: &Progress) {}
    fn on_response(&self, response: &Response) {}
    fn on_header_receive(&self) {}
}

impl Observer {
    pub fn new() -> Self {
        Observer {
            callbacks: Arc::new(Mutex::new(HashMap::new())),
            listener: Mutex::new(None),
        }
    }

    pub fn set_listenr(&self, file: File) {
        let mut listener = UdsListener::new(file);
        let callbacks = self.callbacks.clone();
        let handle = ylong_runtime::spawn(async move {
            loop {
                match listener.recv().await {
                    Ok(message) => match message {
                        Message::HttpResponse(response) => {
                            let task_id = response.task_id.parse().unwrap();
                            if let Some(callback) = callbacks.lock().unwrap().get(&task_id) {
                                callback.on_response(&response);
                            }
                        }
                        Message::NotifyData(data) => {
                            let task_id = data.task_id as i64;
                            let progress = data.progress;
                            if let Some(callback) = callbacks.lock().unwrap().get(&task_id) {
                                match data.subscribe_type {
                                    SubscribeType::Progress => {
                                        callback.on_progress(&progress);
                                    }
                                    SubscribeType::Completed => {
                                        callback.on_completed(&progress);
                                    }
                                    SubscribeType::Failed => {
                                        callback.on_failed(
                                            &progress,
                                            data.task_states[0].response_code as i32,
                                        );
                                    }
                                    SubscribeType::Pause => {
                                        callback.on_pause(&progress);
                                    }
                                    SubscribeType::Resume => {
                                        callback.on_resume(&progress);
                                    }
                                    SubscribeType::Remove => {
                                        callback.on_remove(&progress);
                                    }
                                    SubscribeType::HeaderReceive => {
                                        callback.on_header_receive();
                                    }
                                    _ => {}
                                }
                            }
                        }
                    },
                    Err(e) => eprintln!("Error receiving message: {}", e),
                }
            }
        });
        if let Some(old_listener) = self.listener.lock().unwrap().replace(handle) {
            old_listener.cancel();
        }
    }

    pub fn register_callback(
        &self,
        task_id: i64,
        callback: Arc<dyn Callback + Send + Sync + 'static>,
    ) {
        self.callbacks.lock().unwrap().insert(task_id, callback);
    }

    pub fn unregister_callback(&self, task_id: i64) {
        self.callbacks.lock().unwrap().remove(&task_id);
    }
}
