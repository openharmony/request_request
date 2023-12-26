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

use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

use super::TaskManager;
use crate::manager::scheduled;
use crate::task::config::Action;
use crate::task::info::{ApplicationState, Mode, State};
use crate::task::reason::Reason;

cfg_oh! {
    use crate::manager::Notifier;
}

impl TaskManager {
    pub(crate) fn update_app_state(&mut self, uid: u64, state: ApplicationState) {
        if self.app_task_map.get(&uid).is_none() {
            return;
        }

        match state {
            ApplicationState::Foreground => {
                match self.app_state_map.get(&uid) {
                    Some(state) => {
                        state.store(ApplicationState::Foreground as u8, Ordering::SeqCst)
                    }
                    None => {
                        self.app_state_map.insert(
                            uid,
                            Arc::new(AtomicU8::new(ApplicationState::Foreground as u8)),
                        );
                    }
                }
                let qos_changes = self.qos.change_state(uid, state);
                self.change_qos(qos_changes);

                self.update_foreground_app(uid);
            }

            ApplicationState::Background => {
                match self.app_state_map.get(&uid) {
                    Some(state) => {
                        state.store(ApplicationState::Background as u8, Ordering::SeqCst)
                    }
                    None => {
                        self.app_state_map.insert(
                            uid,
                            Arc::new(AtomicU8::new(ApplicationState::Background as u8)),
                        );
                    }
                }

                let tx = self.tx.clone();
                ylong_runtime::spawn(scheduled::update_background_app(uid, tx));

                let qos_changes = self.qos.change_state(uid, state);
                self.change_qos(qos_changes);
            }

            ApplicationState::Terminated => {
                match self.app_state_map.get(&uid) {
                    Some(state) => {
                        state.store(ApplicationState::Terminated as u8, Ordering::SeqCst)
                    }
                    None => {
                        self.app_state_map.insert(
                            uid,
                            Arc::new(AtomicU8::new(ApplicationState::Terminated as u8)),
                        );
                    }
                }

                let qos_changes = self.qos.change_state(uid, ApplicationState::Background);
                self.change_qos(qos_changes);
                self.update_terminated_app(uid);
            }
        }
    }

    fn update_foreground_app(&mut self, uid: u64) {
        debug!("TaskManager begin update_foreground_app uid:{}", uid);

        let tasks = match self.app_task_map.get(&uid) {
            Some(set) => {
                let mut v = vec![];
                for task_id in set {
                    match self.tasks.get(task_id) {
                        Some(task) => {
                            if task.conf.common_data.mode == Mode::FrontEnd {
                                v.push(task.clone())
                            }
                        }
                        None => {
                            error!("TaskManager update_foreground_app uid:{}, task_id:{} not found int tasks", uid, task_id);
                            return;
                        }
                    }
                }
                v
            }
            None => {
                error!("TaskManager update_foreground_app uid:{} not found", uid);
                return;
            }
        };

        tasks.into_iter().for_each(|task| {
            let state = task.status.lock().unwrap().state;
            let reason = task.status.lock().unwrap().reason;
            if state == State::Paused && reason == Reason::AppBackgroundOrTerminate {
                info!("Begin try resume task as app switch to background");
                task.resume.store(true, Ordering::SeqCst);

                let notify_data = task.build_notify_data();

                #[cfg(feature = "oh")]
                Notifier::service_front_notify(
                    "resume".into(),
                    notify_data,
                    &self.app_state(uid, &task.conf.bundle),
                );
                self.start_inner(task);
            }
        });
    }

    pub(crate) fn update_background_app(&mut self, uid: u64) {
        if !self.app_state_map.contains_key(&uid) {
            return;
        }

        debug!("TaskManager begin update_background_app uid:{}", uid);

        if ApplicationState::from(self.app_state_map.get(&uid).unwrap().load(Ordering::SeqCst))
            == ApplicationState::Foreground
        {
            debug!(
                "TaskManager abort update_background_app uid:{} that has changed to Foreground",
                uid
            );
            return;
        }

        if self.app_task_map.get(&uid).is_none() {
            return;
        }

        let tasks = match self.app_task_map.get(&uid) {
            Some(set) => {
                let mut v = vec![];
                for task_id in set {
                    match self.tasks.get(task_id) {
                        Some(task) => {
                            if task.conf.common_data.mode == Mode::FrontEnd {
                                v.push(task.clone())
                            }
                        }
                        None => {
                            error!("TaskManager update_foreground_app uid:{}, task_id:{} not found int tasks", uid, task_id);
                            return;
                        }
                    }
                }
                v
            }
            None => {
                error!("TaskManager update_foreground_app uid:{} not found", uid);
                return;
            }
        };
        tasks.into_iter().for_each(|task| {
            if task.conf.common_data.action == Action::UpLoad {
                task.set_status(State::Failed, Reason::AppBackgroundOrTerminate);
                self.after_task_processed(&task);
            } else if task.conf.common_data.action == Action::DownLoad {
                self.pause_task(task, Reason::AppBackgroundOrTerminate);
            }
        });
    }

    fn update_terminated_app(&mut self, uid: u64) {
        debug!("TaskManager begin update_terminated_app uid:{}", uid);

        let tasks = match self.app_task_map.get(&uid) {
            Some(set) => {
                let mut v = vec![];
                for task_id in set {
                    match self.tasks.get(task_id) {
                        Some(task) => {
                            if task.conf.common_data.mode == Mode::FrontEnd {
                                v.push(task.clone())
                            }
                        }
                        None => {
                            error!("TaskManager update_foreground_app uid:{}, task_id:{} not found int tasks", uid, task_id);
                            return;
                        }
                    }
                }
                v
            }
            None => {
                error!("TaskManager update_foreground_app uid:{} not found", uid);
                return;
            }
        };

        tasks.into_iter().for_each(|task| {
            task.set_status(State::Failed, Reason::AppBackgroundOrTerminate);
            self.after_task_processed(&task);
        });
    }

    pub(crate) fn update_network(&mut self) {
        let tasks = self.tasks.values().cloned().collect::<Vec<_>>();
        self.schedule_unload_sa();

        for task in tasks {
            if unsafe { IsOnline() } {
                self.resume_waiting_task(task.clone());
            }
        }
    }
}

extern "C" {
    pub(crate) fn IsOnline() -> bool;
}
