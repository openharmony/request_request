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

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU32, AtomicU8, Ordering};
use std::sync::Arc;

use ylong_runtime::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use ylong_runtime::task::JoinHandle;

use super::events::{
    ConstructMessage, EventMessage, ScheduledMessage, ServiceMessage, StateMessage, TaskMessage,
};
use super::qos::{Qos, QosQueue};
use super::scheduled;
use crate::error::ErrorCode;
use crate::task::config::Version;
use crate::task::info::{ApplicationState, State};
use crate::task::reason::Reason;
use crate::task::request_task::RequestTask;
use crate::utils::c_wrapper::CStringWrapper;

cfg_oh! {
    use crate::manager::Notifier;
}

pub(crate) struct TaskManager {
    pub(crate) tasks: HashMap<u32, Arc<RequestTask>>,
    pub(crate) qos: QosQueue,

    pub(crate) app_task_map: HashMap<u64, HashSet<u32>>,

    pub(crate) app_state_map: HashMap<u64, Arc<AtomicU8>>,

    pub(crate) restoring: bool,
    pub(crate) api10_background_task_count: u32,

    pub(crate) unload_handle: Option<JoinHandle<()>>,

    pub(crate) recording_rdb_num: Arc<AtomicU32>,
    pub(crate) tx: UnboundedSender<EventMessage>,
    pub(crate) rx: UnboundedReceiver<EventMessage>,
}

#[derive(Clone)]
pub(crate) struct TaskManagerEntry {
    tx: UnboundedSender<EventMessage>,
}

impl TaskManagerEntry {
    fn new(tx: UnboundedSender<EventMessage>) -> Self {
        Self { tx }
    }

    pub(crate) fn send_event(&self, event: EventMessage) -> bool {
        if self.tx.send(event).is_err() {
            error!("Sends TaskManager event failed, or TaskManager is unloading");
            return false;
        }
        true
    }
}

impl TaskManager {
    pub(crate) fn init() -> TaskManagerEntry {
        debug!("TaskManager init");

        ylong_runtime::builder::RuntimeBuilder::new_multi_thread()
            .worker_num(4)
            .build_global()
            .unwrap();

        let (tx, rx) = unbounded_channel();

        let mut task_manager = Self::new(tx.clone(), rx);

        // Considers update invalid task in database to FAILED state here?.

        task_manager.restore_all_tasks(task_manager.recording_rdb_num.clone());

        ylong_runtime::spawn(scheduled::clear_timeout_tasks(task_manager.tx.clone()));

        ylong_runtime::spawn(scheduled::log_all_task_info(task_manager.tx.clone()));

        ylong_runtime::spawn(task_manager.run());

        TaskManagerEntry::new(tx)
    }

    fn new(tx: UnboundedSender<EventMessage>, rx: UnboundedReceiver<EventMessage>) -> Self {
        TaskManager {
            qos: QosQueue::new(),
            tasks: HashMap::new(),
            app_task_map: HashMap::new(),
            app_state_map: HashMap::new(),

            unload_handle: None,
            restoring: false,
            api10_background_task_count: 0,
            recording_rdb_num: Arc::new(AtomicU32::new(0)),
            rx,
            tx,
        }
    }

    async fn run(mut self) {
        loop {
            let recv = match self.rx.recv().await {
                Ok(message) => message,
                Err(e) => {
                    error!("TaskManager recv error {:?}", e);
                    continue;
                }
            };

            match recv {
                EventMessage::Service(message) => self.handle_service_command(message),
                EventMessage::State(message) => self.handle_state_change(message),
                EventMessage::Task(message) => self.handle_request_task(message),
                EventMessage::Scheduled(message) => {
                    if self.handle_scheduled_task(message) {
                        info!("TaskManager unload succeed");
                        // If unload_sa success, breaks this loop.
                        return;
                    }
                }
            }

            debug!("TaskManager handle message done");
        }
    }

    fn handle_request_task(&mut self, message: TaskMessage) {
        debug!("TaskManager handle task_message {:?}", message);

        match message {
            TaskMessage::Finished(task_id) => {
                let task = match self.tasks.get(&task_id) {
                    Some(task) => task.clone(),
                    None => return,
                };
                self.after_task_processed(&task);
            }
        }
    }

    fn handle_scheduled_task(&mut self, message: ScheduledMessage) -> bool {
        debug!("TaskManager handle scheduled_message {:?}", message);

        match message {
            ScheduledMessage::ClearTimeoutTasks => self.clear_timeout_tasks(),
            ScheduledMessage::LogTasks => self.log_all_task_info(),
            ScheduledMessage::Unload => return self.unload_sa(),
            ScheduledMessage::UpdateBackgroundApp(uid) => self.update_background_app(uid),
        }
        false
    }

    fn handle_state_change(&mut self, message: StateMessage) {
        debug!("TaskManager handle state_message {:?}", message);

        match message {
            StateMessage::NetworkChange => {
                self.update_network();
            }
            StateMessage::AppStateChange(uid, state) => {
                self.update_app_state(uid, state);
            }
        }
    }

    fn handle_service_command(&mut self, message: ServiceMessage) {
        debug!("TaskManager handle service_message {:?}", message);

        match message {
            ServiceMessage::Construct(construct_message, tx) => {
                let ConstructMessage {
                    config,
                    files,
                    body_files,
                } = *construct_message;
                let error_code = self.construct_task(config, files, body_files);
                let _ = tx.send(error_code);
            }
            ServiceMessage::Pause(uid, task_id, tx) => {
                let error_code = self.pause(uid, task_id);
                let _ = tx.send(error_code);
            }
            ServiceMessage::Resume(uid, task_id, tx) => {
                let error_code = self.resume(uid, task_id);
                let _ = tx.send(error_code);
            }
            ServiceMessage::Start(uid, task_id, tx) => {
                let error_code = self.start(uid, task_id);
                let _ = tx.send(error_code);
            }
            ServiceMessage::Stop(uid, task_id, tx) => {
                let error_code = self.stop(uid, task_id);
                let _ = tx.send(error_code);
            }

            ServiceMessage::Show(uid, task_id, tx) => {
                let task_info = self.show(uid, task_id);
                let _ = tx.send(task_info);
            }

            ServiceMessage::Query(task_id, query_action, tx) => {
                let task_info = self.query(task_id, query_action);
                let _ = tx.send(task_info);
            }
            ServiceMessage::Search(filter, tx) => {
                let v = self.search(filter);
                let _ = tx.send(v);
            }
            ServiceMessage::Touch(uid, task_id, token, tx) => {
                let task_info = self.touch(uid, task_id, token);
                let _ = tx.send(task_info);
            }
            ServiceMessage::Remove(uid, task_id, tx) => {
                let error_code = self.remove(uid, task_id);
                let _ = tx.send(error_code);
            }
            ServiceMessage::DumpAll(tx) => {
                let dump_all_info = self.query_all_task();
                let _ = tx.send(dump_all_info);
            }
            ServiceMessage::DumpOne(task_id, tx) => {
                let dump_one_info = self.query_one_task(task_id);
                let _ = tx.send(dump_one_info);
            }
            ServiceMessage::QueryMimeType(uid, task_id, tx) => {
                let s = self.query_mime_type(uid, task_id);
                let _ = tx.send(s);
            }
        }
    }

    pub(crate) fn app_state(&mut self, uid: u64, bundle: &str) -> Arc<AtomicU8> {
        match self.app_state_map.get(&uid) {
            Some(state) => state.clone(),
            None => {
                let top_bundle = unsafe { GetTopBundleName() };
                let top_bundle = top_bundle.to_string();
                debug!(
                    "TaskManager try get app_state uid:{} from top_bundle {}",
                    uid, top_bundle
                );
                if top_bundle == bundle {
                    let state = Arc::new(AtomicU8::new(ApplicationState::Foreground as u8));
                    self.app_state_map.insert(uid, state.clone());
                    state
                } else {
                    let state = Arc::new(AtomicU8::new(ApplicationState::Background as u8));
                    self.app_state_map.insert(uid, state.clone());
                    state
                }
            }
        }
    }

    pub(crate) fn get_task(&self, uid: u64, task_id: u32) -> Option<Arc<RequestTask>> {
        self.app_task_map
            .get(&uid)
            .and_then(|set| set.get(&task_id))
            .and_then(|task_id| self.tasks.get(task_id).cloned())
    }

    fn process_waiting_task(&mut self, uid: u64, version: Version) {
        match version {
            Version::API10 => {
                let tasks = match self.app_task_map.get(&uid) {
                    Some(v) => v.iter().copied().collect::<Vec<_>>(),
                    None => return,
                };
                for task in tasks {
                    let request_task = match self.tasks.get(&task) {
                        Some(task) => task,
                        None => {
                            error!(
                                "TaskManager process waiting task, task_id:{} not found",
                                task
                            );
                            continue;
                        }
                    };
                    if request_task.conf.version == Version::API10 {
                        let state = request_task.status.lock().unwrap().state;
                        if state == State::Waiting {
                            debug!(
                                "TaskManager begin process v10 task_id:{} which in waitting state",
                                task
                            );
                            self.start_inner(request_task.clone());
                            return;
                        }
                    }
                }
            }
            Version::API9 => {
                for task in self.tasks.values() {
                    if task.conf.version == Version::API9 {
                        let state = task.status.lock().unwrap().state;
                        if state == State::Waiting {
                            debug!(
                                "TaskManager begin process v9 task_id:{} which in waitting state",
                                task.conf.common_data.task_id
                            );
                            let task = task.clone();
                            self.start_inner(task);
                            return;
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn after_task_processed(&mut self, task: &Arc<RequestTask>) {
        let state = task.status.lock().unwrap().state;
        if state != State::Completed
            && state != State::Failed
            && state != State::Removed
            && state != State::Stopped
        {
            return;
        }
        debug!(
            "TaskManager remove task_id:{} from map",
            task.conf.common_data.task_id
        );

        let remove_task = self.tasks.remove(&task.conf.common_data.task_id).unwrap();

        let uid = &task.conf.common_data.uid;
        match self.app_task_map.get_mut(uid) {
            Some(map) => {
                map.remove(&task.conf.common_data.task_id);
            }
            None => {
                error!("TaskManager after_task_processed get uid:{} failed", uid);
                return;
            }
        }

        match self.app_task_map.get(&task.conf.common_data.uid) {
            Some(map) => {
                if map.is_empty() {
                    self.app_task_map.remove(&task.conf.common_data.uid);
                    self.app_state_map.remove(&remove_task.conf.common_data.uid);
                }
            }
            None => {
                error!("TaskManger where is my map");
                return;
            }
        }

        if remove_task.conf.version == Version::API10 {
            self.api10_background_task_count -= 1;
        }

        let app_state = ApplicationState::from(remove_task.app_state.load(Ordering::SeqCst));
        if !(app_state == ApplicationState::Background
            && remove_task.conf.version == Version::API10)
        {
            #[cfg(feature = "oh")]
            Notifier::remove_notify(&remove_task);
        }

        // Notifies NotifyManager to remove RemoteObj when task has been removed.

        #[cfg(feature = "oh")]
        Notifier::clear_notify(&remove_task);

        let map = self
            .qos
            .remove(task.conf.common_data.uid, task.conf.common_data.task_id);

        self.change_qos(map);

        if self.check_unload_sa() {
            self.schedule_unload_sa();
        } else {
            self.process_waiting_task(remove_task.conf.common_data.uid, remove_task.conf.version);
        }
    }

    pub(crate) fn pause_task(&self, task: Arc<RequestTask>, reason: Reason) -> ErrorCode {
        let uid = task.conf.common_data.uid;
        let task_id = task.conf.common_data.task_id;

        if !task.set_status(State::Paused, reason) {
            let state = task.status.lock().unwrap();
            error!(
                "TaskManager pause a task, uid:{}, task_id:{} failed which state is {:?}",
                uid, task_id, state
            );
            ErrorCode::TaskStateErr
        } else {
            task.resume.store(false, Ordering::SeqCst);
            debug!(
                "TaskManager pause a task, uid:{}, task_id:{} success",
                uid, task_id
            );
            ErrorCode::ErrOk
        }
    }

    pub(crate) fn resume_waiting_task(&mut self, task: Arc<RequestTask>) {
        let state = task.status.lock().unwrap().state;
        if state == State::Waiting && task.is_satisfied_configuration() {
            info!("Begin try resume task as network condition resume");
            task.resume.store(true, Ordering::SeqCst);
            let notify_data = task.build_notify_data();

            #[cfg(feature = "oh")]
            Notifier::service_front_notify(
                "resume".into(),
                notify_data,
                &self.app_state(task.conf.common_data.uid, &task.conf.bundle),
            );
            self.start_inner(task.clone());
        }
    }

    pub(crate) fn change_qos(&mut self, new_qos: Vec<(u32, Qos)>) {
        for (task_id, qos) in new_qos.iter() {
            if let Some(task) = self.tasks.get(task_id) {
                match qos {
                    Qos::High => {
                        info!("Qos task_id:{} set to High Qos", task_id);
                        task.rate_limiting.store(false, Ordering::SeqCst);
                    }
                    Qos::Low => {
                        info!("Qos task_id:{} set to Low Qos", task_id);
                        task.rate_limiting.store(true, Ordering::SeqCst);
                    }
                }
            }
        }
    }
}

extern "C" {
    pub(crate) fn GetTopBundleName() -> CStringWrapper;
}
