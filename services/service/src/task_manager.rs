/*
 * Copyright (C) 2023 Huawei Device Co., Ltd.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use super::{
    enumration::*, request_task::*, task_config::*, utils::*, task_info::*, progress::*, request_binding::*,
    log_debug, log_info, log_error
};
use crate::log::LOG_LABEL;
use std::{collections::HashMap, ffi::CString, ffi::c_char, fs::File, time::Duration};
use std::sync::atomic::{AtomicU32, Ordering, AtomicBool};
use std::sync::{Arc, Mutex, MutexGuard, Once};
use ylong_runtime::{builder::RuntimeBuilder, executor::Runtime, timer::sleep::sleep};
static MAX_TASK_COUNT: u32 = 300;
static MAX_TASK_COUNT_EACH_APP: u8 = 10;
static MAX_RUNNING_TASK_COUNT_EACH_APP: u8 = 5;
static INTERVAL_SECONDS: u64 = 30 * 60; // half hour
static SECONDS_IN_ONE_DAY: u64 = 24 * 60 * 60;
static SECONDS_IN_ONE_MONTH: u64 = 30 * 24 * 60 * 60;

type AppTask = HashMap<u32, Arc<RequestTask>>;
pub struct TaskManager {
    task_map: Arc<Mutex<HashMap<u64, AppTask>>>,
    event_cb: Option<Box<dyn Fn(String, &NotifyData) + Send + Sync + 'static>>,
    pub global_front_task: Option<Arc<RequestTask>>,
    pub front_app_uid: Option<u64>,
    rt: Runtime,
    pub front_notify_time: u64,
    pub unloading: AtomicBool,
    pub ipc_request: Mutex<bool>,
    total_task_count: AtomicU32, // for unload sa
    api10_background_task_count: AtomicU32,
}

pub fn monitor_task() {
    let task_manager = TaskManager::get_instance();
    task_manager.rt.spawn(async {
        let mut remove_task = Vec::<Arc<RequestTask>>::new();
        loop {
            {
                let manager = TaskManager::get_instance();
                let task_map_guard = manager.task_map.lock().unwrap();
                for (_, app_task) in task_map_guard.iter() {
                    for (_, task) in app_task.iter() {
                        if task.conf.version == Version::API9 {
                            continue;
                        }
                        let current_time = get_current_timestamp();
                        if current_time - task.ctime > SECONDS_IN_ONE_MONTH {
                            task.set_status(State::STOPPED, Reason::TaskSurvivalOneMonth);
                            remove_task.push(task.clone());
                            continue;
                        }
                        let (state, time) = {
                            let guard = task.status.lock().unwrap();
                            (guard.state, guard.waitting_network_time.clone())
                        };
                        if state == State::WAITING {
                            if let Some(t) = time {
                                if current_time - t > SECONDS_IN_ONE_DAY {
                                    task.set_status(State::STOPPED, Reason::WaittingNetWorkOneday);
                                    remove_task.push(task.clone());
                                }
                            }
                        }
                    }
                }
            }
            for task in remove_task.iter() {
                TaskManager::get_instance().after_task_processed(task.clone());
            }
            remove_task.clear();
            sleep(Duration::from_secs(INTERVAL_SECONDS)).await;
        }
    });
}

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            task_map: Arc::new(Mutex::new(HashMap::<u64, AppTask>::new())),
            event_cb: None,
            global_front_task: None,
            front_app_uid: None,
            rt: RuntimeBuilder::new_multi_thread()
                .thread_number(4)
                .build()
                .unwrap(),
            front_notify_time: get_current_timestamp(),
            unloading: AtomicBool::new(false),
            ipc_request: Mutex::new(false),
            total_task_count: AtomicU32::new(0),
            api10_background_task_count: AtomicU32::new(0),
        }
    }

    pub fn get_instance() -> &'static mut Self {
        static mut TASK_MANAGER: Option<TaskManager> = None;
        static ONCE: Once = Once::new();
        ONCE.call_once(|| unsafe {
            TASK_MANAGER = Some(Self::new());
        });

        unsafe { TASK_MANAGER.as_mut().unwrap() }
    }

    pub fn register_callback(
        &mut self,
        cb: Box<dyn Fn(String, &NotifyData) + Send + Sync + 'static>,
    ) {
        self.event_cb = Some(cb);
    }

    // app callback
    pub fn front_notify(&mut self, event: String, notify_data: &NotifyData) {
        if self.event_cb.is_none()
            || !self.is_front_app(notify_data.uid, notify_data.bundle.as_str())
        {
            return;
        }
        log_debug!("front notify");
        self.front_notify_time = get_current_timestamp();
        self.event_cb.as_ref().unwrap()(event, notify_data);
    }

    fn is_front_app(&self, uid: u64, bundle: &str) -> bool {
        if self.front_app_uid.is_none() {
            let top_bundle = unsafe { GetTopBundleName() };
            let top_bundle = convert_to_string(top_bundle);
            let x: String = bundle.into();
            log_debug!("top_bundle {} bundle {}", top_bundle, x);
            if !top_bundle.eq(bundle) {
                return false;
            }
        } else if uid != *self.front_app_uid.as_ref().unwrap() {
            return false;
        }
        true
    }

    pub fn construct_task(
        &mut self,
        conf: Arc<TaskConfig>,
        uid: u64,
        task_id: &mut u32,
        files: Vec<File>,
    ) -> ErrorCode {
        log_debug!("Begin construct a task");
        *task_id = generate_task_id();
        let bundle = conf.bundle.clone();
        let task = RequestTask::constructor(conf, uid, *task_id, files);
        if task.conf.common_data.mode == Mode::FRONTEND {
            task.set_status(State::INITIALIZED, Reason::Default);
            if !self.is_front_app(uid, bundle.as_str()) {
                return ErrorCode::TaskModeErr;
            }
            if self.global_front_task.is_none() {
                self.global_front_task = Some(Arc::new(task));
                self.total_task_count.fetch_add(1, Ordering::SeqCst);
                return ErrorCode::ErrOk;
            }
            self.global_front_task
                .take()
                .unwrap()
                .set_status(State::STOPPED, Reason::StoppedByNewFrontTask);
            self.global_front_task = Some(Arc::new(task));
            return ErrorCode::ErrOk;
        }

        log_debug!("uid {} task_id {} version {:?}", uid, task_id, task.conf.version);
        match task.conf.version {
            Version::API10 => {
                if !self.add_task(uid, *task_id, Arc::new(task)) {
                    return ErrorCode::TaskEnqueueErr;
                }
                self.total_task_count.fetch_add(1, Ordering::SeqCst);
                self.api10_background_task_count
                    .fetch_add(1, Ordering::SeqCst);
                return ErrorCode::ErrOk;
            }
            Version::API9 => {
                self.add_task_api9(uid, *task_id, Arc::new(task));
                self.total_task_count.fetch_add(1, Ordering::SeqCst);
                return ErrorCode::ErrOk;
            }
        }
    }

    fn add_task_api9(&self, uid: u64, task_id: u32, task: Arc<RequestTask>) {
        log_debug!("Begin add a v9 task");
        let mut task_map_guard = self.task_map.lock().unwrap();
        let app_task = task_map_guard.get_mut(&uid);
        match app_task {
            Some(map) => {
                task.set_status(State::INITIALIZED, Reason::Default);
                map.insert(task_id, task);
                log_debug!(
                    "Add v9 task sccuess, the current number of tasks which belongs to the app is {}",
                    map.len() as u8
                );
            }
            None => {
                let mut app_task = AppTask::new();
                task.set_status(State::INITIALIZED, Reason::Default);
                app_task.insert(task_id, task);
                task_map_guard.insert(uid, app_task);
                log_debug!("Add v9 task sccuess, there is one task which belongs to the app");
            }
        }
    }

    fn add_task(&self, uid: u64, task_id: u32, task: Arc<RequestTask>) -> bool {
        log_debug!("Begin add a v10 task");
        let mut task_map_guard = self.task_map.lock().unwrap();
        if self.api10_background_task_count.load(Ordering::SeqCst) >= MAX_TASK_COUNT {
            log_error!("Add v10 task failed, the number of tasks has reached the limit of the maximum number of tasks in the system");
            return false;
        }
        let app_task = task_map_guard.get_mut(&uid);
        match app_task {
            Some(map) => {
                if (map.len() as u8) == MAX_TASK_COUNT_EACH_APP {
                    log_error!("Add v10 task failed, the maximum value for each application processing task has been reached");
                    return false;
                }
                task.set_status(State::INITIALIZED, Reason::Default);
                map.insert(task_id, task);
                log_debug!(
                    "Add v10 task sccuess, the current number of tasks which belongs to the app is {}",
                    map.len() as u8
                );
                return true;
            }
            None => {
                let mut app_task = AppTask::new();
                task.set_status(State::INITIALIZED, Reason::Default);
                app_task.insert(task_id, task);
                task_map_guard.insert(uid, app_task);
                log_debug!("Add v10 task sccuess, there is one task which belongs to the app");
                return true;
            }
        }
    }

    fn get_task(
        &self,
        uid: u64,
        task_id: u32,
        guard: &MutexGuard<HashMap<u64, AppTask>>,
    ) -> Option<Arc<RequestTask>> {
        if let Some(v) = &self.global_front_task {
            if v.task_id == task_id {
                log_debug!("Get the global front task");
                return Some(v.clone());
            }
        }
        let app_task = guard.get(&uid);
        if let None = app_task {
            log_error!("The Application has not any task");
            return None;
        }
        log_debug!("task_id: {}", task_id);
        let task = app_task.unwrap().get(&task_id);
        match task {
            Some(v) => {
                log_debug!("Get the task by uid and task id success");
                return Some(v.clone());
            }
            None => {
                log_error!("Can not found the task which belongs to the application");
                return None;
            }
        }
    }

    fn get_running_background_task_count(
        &self,
        uid: u64,
        guard: &MutexGuard<HashMap<u64, AppTask>>,
    ) -> u8 {
        let app_task = guard.get(&uid);
        let mut count = 0;
        if let None = app_task {
            log_info!("The Application has not any background task");
            return count;
        }
        for (_, task) in app_task.unwrap().iter() {
            let state = task.status.lock().unwrap().state;
            if task.conf.version == Version::API10
                && (state == State::RETRYING || state == State::RUNNING)
            {
                count += 1;
            }
        }
        log_debug!(
            "The running background task which belongs to the app is {}",
            count
        );
        count
    }

    fn start_inner(
        &self,
        uid: u64,
        task: Arc<RequestTask>,
        guard: &MutexGuard<HashMap<u64, AppTask>>,
    ) {
        if !task.check_net_work_status() {
            log_error!("check net work failed");
            return;
        }

        let vesion = task.conf.version;
        if vesion == Version::API10 && task.conf.common_data.mode == Mode::BACKGROUND {
            let running_task_count = self.get_running_background_task_count(uid, guard);
            if running_task_count >= MAX_RUNNING_TASK_COUNT_EACH_APP {
                log_debug!("Too many task in running state");
                task.set_status(State::WAITING, Reason::RunningTaskMeetLimits);
                return;
            }
        }

        let (state, reason) = {
            let status = task.status.lock().unwrap();
            (status.state, status.reason.clone())
        };

        if state == State::WAITING
            && (reason == Reason::NetWorkOffline || reason == Reason::UnSupportedNetWorkType)
        {
            task.retry.store(true, Ordering::SeqCst);
            task.tries.fetch_add(1, Ordering::SeqCst);
            task.set_status(State::RETRYING, Reason::Default);
        } else {
            task.set_status(State::RUNNING, Reason::Default);
        }
        self.rt.spawn(async move {
            run(task.clone()).await;
            TaskManager::get_instance().after_task_processed(task);
        });
        log_debug!("Start the task success");
        return;
    }

    pub fn start(&self, uid: u64, task_id: u32) -> ErrorCode {
        log_debug!("Start a task");
        let task_map_guard = self.task_map.lock().unwrap();
        let task = self.get_task(uid, task_id, &task_map_guard);
        if let None = task {
            log_error!("Start failed");
            return ErrorCode::TaskNotFound;
        }
        let task = task.unwrap();
        let task_state = task.status.lock().unwrap().state;
        if task_state != State::INITIALIZED {
            log_error!("Can not start a task which state is {}", task_state as u32);
            return ErrorCode::TaskStateErr;
        }
        self.start_inner(uid, task, &task_map_guard);
        ErrorCode::ErrOk
    }

    fn process_app_waitting_task(&self, uid: u64) {
        let task_map_guard = self.task_map.lock().unwrap();
        let app_task = task_map_guard.get(&uid);
        if let None = app_task {
            return;
        }
        let app_task = app_task.unwrap();
        for (_, request_task) in app_task.iter() {
            let state = request_task.status.lock().unwrap().state;
            if state == State::WAITING {
                log_debug!("Begin process the task which in waitting state");
                self.start_inner(request_task.uid, request_task.clone(), &task_map_guard);
            }
        }
    }

    fn remove_task_from_map(&mut self, task: Arc<RequestTask>) {
        let state = task.status.lock().unwrap().state;
        if state == State::COMPLETED
            || state == State::FAILED
            || state == State::REMOVED
            || state == State::STOPPED
        {
            log_debug!("remove task from map");
            if task.conf.version == Version::API9 && state != State::REMOVED {
                let mut file = task.files.lock().unwrap();
                file.clear();
                self.total_task_count.fetch_sub(1, Ordering::SeqCst);
                return;
            }

            {
                let mut task_map_guard = self.task_map.lock().unwrap();
                if let Some(v) = &self.global_front_task {
                    if task.task_id == v.task_id {
                        self.global_front_task.take();
                        self.total_task_count.fetch_sub(1, Ordering::SeqCst);
                        return;
                    }
                }
                let app_task = task_map_guard.get_mut(&task.uid);
                if let None = app_task {
                    return;
                }
                let app_task = app_task.unwrap();
                log_debug!("Task has been processed, begin remove task from task map");
                let removed_task = app_task.remove(&task.task_id);
                if !removed_task.is_none() {
                    self.total_task_count.fetch_sub(1, Ordering::SeqCst);
                    if removed_task.unwrap().conf.version == Version::API10 {
                        self.api10_background_task_count
                            .fetch_sub(1, Ordering::SeqCst);
                    }
                }

                if (app_task.len() as u8) == 0 {
                    log_debug!("All task of the app has been processed, begin remove app task map");
                    task_map_guard.remove(&task.uid);
                    return;
                }
            }
            if task.conf.version == Version::API10 {
                self.process_app_waitting_task(task.uid);
            }
        }
    }

    // fn unload_sa(&self) {
    //     if !self.global_front_task.is_none() ||
    //         self.total_task_count.load(Ordering::SeqCst) != 0 {
    //         return;
    //     }
    //     static ONCE: Once = Once::new();
    //     ONCE.call_once(|| {
    //         TaskManager::get_instance().rt.spawn(async move {
    //             loop {
    //                 sleep(Duration::from_secs(60)).await;
    //                 let task_manager = TaskManager::get_instance();
    //                 if task_manager.total_task_count.load(Ordering::SeqCst) != 0 {
    //                    continue;
    //                 }
    //                 match task_manager.ipc_request.try_lock() {
    //                     Ok(_) => {
    //                         task_manager.unloading.store(true, Ordering::SeqCst);
    //                         unload_systemability();
    //                         return;
    //                     }
    //                     Err(_) => continue,
    //                 }
    //             }
    //         });
    //     });
    // }

    fn after_task_processed(&mut self, task: Arc<RequestTask>) {
        self.remove_task_from_map(task);
        // self.unload_sa();
    }

    pub fn pause(&self, uid: u64, task_id: u32) -> ErrorCode {
        log_debug!("Pause a task");
        let task_map_guard = self.task_map.lock().unwrap();
        let task = self.get_task(uid, task_id, &task_map_guard);
        if let None = task {
            log_error!("Pause failed");
            return ErrorCode::TaskNotFound;
        }
        let task = task.unwrap();
        if task.conf.common_data.mode == Mode::FRONTEND {
            log_error!("Front task is not support pause action");
            return ErrorCode::TaskModeErr;
        }
        if !task.set_status(State::PAUSED, Reason::UserOperation) {
            log_error!("Can not pause a task which state is not meet the requirements");
            return ErrorCode::TaskStateErr;
        }
        log_info!("Pause the task success");
        ErrorCode::ErrOk
    }

    pub fn resume(&self, uid: u64, task_id: u32) -> ErrorCode {
        log_debug!("Resume a task");
        let task_map_guard = self.task_map.lock().unwrap();
        let task = self.get_task(uid, task_id, &task_map_guard);
        if let None = task {
            log_error!("Resume failed");
            return ErrorCode::TaskNotFound;
        }
        let task = task.unwrap();
        if task.conf.common_data.mode == Mode::FRONTEND {
            log_error!("Front task is not support resume action");
            return ErrorCode::TaskModeErr;
        }
        let state = task.status.lock().unwrap().state;
        if state != State::PAUSED {
            log_error!("Can not resume a task which state is not paused");
            return ErrorCode::TaskStateErr;
        }
        log_info!("Resume the task success");
        self.start_inner(uid, task, &task_map_guard);
        ErrorCode::ErrOk
    }

    pub fn stop(&mut self, uid: u64, task_id: u32) -> ErrorCode {
        log_debug!("Stop a task");
        let task = {
            let task_map_guard = self.task_map.lock().unwrap();
            let task = self.get_task(uid, task_id, &task_map_guard);
            if let None = task {
                log_error!("Stop failed");
                return ErrorCode::TaskNotFound;
            }
            let task = task.unwrap();
            if !task.set_status(State::STOPPED, Reason::UserOperation) {
                log_error!("Can not stop a task which state is not meet the requirements");
                return ErrorCode::TaskStateErr;
            }
            task
        };
        self.after_task_processed(task);
        println!("Stopped success");
        ErrorCode::ErrOk
    }

    pub fn remove(&mut self, uid: u64, task_id: u32) -> ErrorCode {
        log_debug!("Remove a task");
        let task = {
            let task_map_guard = self.task_map.lock().unwrap();
            let task = self.get_task(uid, task_id, &task_map_guard);
            if let None = task {
                log_error!("Remove failed");
                return ErrorCode::TaskNotFound;
            }
            let task = task.unwrap();
            task.set_status(State::REMOVED, Reason::UserOperation);
            task
        };
        self.after_task_processed(task);
        log_info!("remove success");
        ErrorCode::ErrOk
    }

    pub fn show(&self, uid: u64, task_id: u32) -> Option<TaskInfo> {
        log_debug!("Show a task");
        let task_map_guard = self.task_map.lock().unwrap();
        let task = self.get_task(uid, task_id, &task_map_guard);
        match task {
            Some(value) => {
                log_debug!("Query task info by memory");
                let task_info = value.show();
                return Some(task_info);
            }
            None => {
                // API10 query in database
                return None;
            }
        }
    }

    // use only for download
    pub fn query_mime_type(&self, uid: u64, task_id: u32) -> String {
        log_debug!("Query a task mime type");
        let task_map_guard = self.task_map.lock().unwrap();
        let task = self.get_task(uid, task_id, &task_map_guard);
        match task {
            Some(value) => {
                log_debug!("Query task mime type by memory");
                let mimt_type = value.query_mime_type();
                return mimt_type;
            }
            None => {
                // API10 query in database
                return "".into();
            }
        }
    }
}

pub fn monitor_network() {
    log_info!("monitor_network");
    unsafe {
        RegisterNetworkCallback(resume_task_by_network);
    }

}

// 前台任务不会恢复
extern "C" fn resume_task_by_network() {
    if unsafe { !IsOnline() } {
        return;
    }
    log_info!("resume task by network");
    let task_manager = TaskManager::get_instance();
    let guard = task_manager.task_map.lock().unwrap();
    for (uid, app_task) in guard.iter() {
        let uid = *uid;
        for (_, task) in app_task.iter() {
            let state = task.status.lock().unwrap().state;
            if state == State::WAITING && task.is_satisfied_configuration() {
                log_debug!("Begin try resume task as network condition resume");
                let task = task.clone();
                task_manager.rt.spawn(async move {
                    sleep(Duration::from_secs(10)).await;
                    let manager = TaskManager::get_instance();
                    let guard = manager.task_map.lock().unwrap();
                    manager.start_inner(uid, task, &guard);
                });
            }
        }
    }
}

pub fn monitor_app_state() {
    log_info!("monitor_app_state");
    unsafe {
        RegisterAPPStateCallback(update_app_state);
    }
}

extern "C" fn update_app_state(uid: i32, state: i32) {
    log_info!("Update app state, uid = {}, state = {}", uid, state);
    let task_manager = TaskManager::get_instance();
    if is_foreground(state) {
        log_debug!("Save front app uid");
        task_manager.front_app_uid = Some(uid as u64);
    } else if is_background_or_terminated(state) {
        if let Some(v) = task_manager.front_app_uid {
            if v as i32 == uid {
                task_manager.front_app_uid = None;
            }
        }
        if task_manager.global_front_task.is_none() {
            return;
        }
        if uid as u64 != task_manager.global_front_task.as_ref().unwrap().uid {
            return;
        }
        task_manager
            .global_front_task
            .take()
            .unwrap()
            .set_status(State::STOPPED, Reason::AppBackgroundOrTerminate);
    }
}

fn is_foreground(state: i32) -> bool {
    let app_state = ApplicationState::AppStateForeground as i32;
    app_state == state
}

fn is_background_or_terminated(state: i32) -> bool {
    (state == ApplicationState::AppStateBackground as i32)
        || (state == ApplicationState::AppStateTerminated as i32)
}

