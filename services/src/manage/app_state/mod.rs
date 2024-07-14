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

// AppManager 逻辑：
// 1. 当新的任务被启动时，从 AppManager 中获取一个句柄（如果 AppManager
//    中没有则创建），
// 任务将从这个句柄中感知到对应的应用状态，如果应用状态不符合运行状态，
// 则停止运行。
// 2. 当应用状态发生变化时（主要是前台切后台，执行定时协程），
//    及时调整对应句柄的应用状态。
// 3. 启动 Listener，从 Listener 中及时获取应用状态。

mod listener;

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use listener::AppStateListener;
use ylong_runtime::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use ylong_runtime::sync::oneshot;
use ylong_runtime::task::JoinHandle;
use ylong_runtime::time::sleep;

use super::task_manager::TaskManagerTx;
use crate::manage::events::{StateEvent, TaskManagerEvent};
use crate::service::client::ClientManagerEntry;
use crate::task::info::ApplicationState;
use crate::utils::{query_app_state, runtime_spawn};

const BACKGROUND_TASK_STOP_INTERVAL: u64 = 60;

pub(crate) struct AppStateManager {
    app_state: HashMap<u64, AppStateInfo>,
    tx: AppStateManagerTx,
    rx: AppStateManagerRx,
    task_manager: TaskManagerTx,
}

impl AppStateManager {
    pub(crate) fn init(
        client_manager: ClientManagerEntry,
        task_manager: TaskManagerTx,
    ) -> AppStateManagerTx {
        let (tx, rx) = unbounded_channel();
        let tx = AppStateManagerTx::new(tx);
        let rx = AppStateManagerRx::new(rx);

        let manager = Self {
            task_manager,
            app_state: HashMap::new(),
            tx: tx.clone(),
            rx,
        };
        runtime_spawn(manager.run());
        AppStateListener::init(client_manager, tx.clone());
        tx
    }

    async fn run(mut self) {
        loop {
            let event = match self.rx.recv().await {
                Ok(event) => event,
                Err(e) => {
                    error!("AppStateManager receives error {:?}", e);
                    continue;
                }
            };

            match event {
                AppStateEvent::GetAppState(uid, tx) => {
                    self.get_app_state(uid, tx);
                }
                AppStateEvent::GetAppRawState(uid, tx) => {
                    self.get_app_raw_state(uid, tx);
                }
                AppStateEvent::ChangeAppState(uid, state) => {
                    self.change_app_state(uid, state);
                }
                AppStateEvent::TriggerAppStateChange(uid, state) => {
                    self.trigger_app_state_change(uid, state);
                }
            }

            debug!("AppStateManager handles events finished");
        }
    }

    fn get_app_state(&mut self, uid: u64, tx: oneshot::Sender<AppState>) {
        // Everytime we get app state, update app state right now.

        if let Some(app) = self.app_state.get(&uid) {
            let _ = tx.send(app.state.clone());
            return;
        }

        let state = query_app_state(uid);

        let app = AppStateInfo::new(AppState::new(uid, state, self.tx.clone()));
        let state = app.state.clone();

        self.app_state.insert(uid, app);

        let _ = tx.send(state);
    }

    fn get_app_raw_state(&mut self, uid: u64, tx: oneshot::Sender<ApplicationState>) {
        // Everytime we get app state, update app state right now.
        if let Some(app) = self.app_state.get(&uid) {
            let _ = tx.send(app.state.state());
            return;
        }

        let state = query_app_state(uid);

        let _ = tx.send(state);
    }

    fn change_app_state(&mut self, uid: u64, state: ApplicationState) {
        if let Some(st) = self.app_state.get_mut(&uid) {
            if state == ApplicationState::Foreground {
                if let Some(handle) = st.handle.take() {
                    handle.cancel();
                }
                {
                    let mut a = st.state.inner.lock().unwrap();
                    if a.state == state {
                        return;
                    }
                    a.state = state;
                }
                self.task_manager
                    .send_event(TaskManagerEvent::State(StateEvent::AppStateChange(
                        uid, state,
                    )));
            } else {
                // Here we need not to change app state immediately.
                st.handle = Some(runtime_spawn(update_background_app(
                    uid,
                    state,
                    self.tx.clone(),
                )));
            }
        }
    }

    fn trigger_app_state_change(&mut self, uid: u64, state: ApplicationState) {
        if let Some(st) = self.app_state.get_mut(&uid) {
            st.handle = None;
            {
                let mut a = st.state.inner.lock().unwrap();
                a.state = state;
            }
            self.task_manager
                .send_event(TaskManagerEvent::State(StateEvent::AppStateChange(
                    uid, state,
                )));
        }
    }
}

#[derive(Clone)]
pub(crate) struct AppStateManagerTx {
    tx: UnboundedSender<AppStateEvent>,
}

impl AppStateManagerTx {
    fn new(tx: UnboundedSender<AppStateEvent>) -> Self {
        Self { tx }
    }

    pub(crate) async fn get_app_state(&self, uid: u64) -> AppState {
        let (tx, rx) = oneshot::channel();
        let _ = self.tx.send(AppStateEvent::GetAppState(uid, tx));
        // Here we must ensure that `AppStateManager` is working properly!
        rx.await.unwrap()
    }

    pub(crate) async fn get_app_raw_state(&self, uid: u64) -> ApplicationState {
        let (tx, rx) = oneshot::channel();
        let _ = self.tx.send(AppStateEvent::GetAppRawState(uid, tx));
        // Here we must ensure that `AppStateManager` is working properly!
        rx.await.unwrap()
    }

    pub(crate) fn change_app_state(&self, uid: u64, state: ApplicationState) {
        let _ = self.tx.send(AppStateEvent::ChangeAppState(uid, state));
    }

    pub(crate) fn trigger_app_state_change(&self, uid: u64, state: ApplicationState) {
        let _ = self
            .tx
            .send(AppStateEvent::TriggerAppStateChange(uid, state));
    }
}

impl Deref for AppStateManagerTx {
    type Target = UnboundedSender<AppStateEvent>;

    fn deref(&self) -> &Self::Target {
        &self.tx
    }
}

struct AppStateManagerRx {
    rx: UnboundedReceiver<AppStateEvent>,
}

impl AppStateManagerRx {
    fn new(rx: UnboundedReceiver<AppStateEvent>) -> Self {
        Self { rx }
    }
}

impl Deref for AppStateManagerRx {
    type Target = UnboundedReceiver<AppStateEvent>;

    fn deref(&self) -> &Self::Target {
        &self.rx
    }
}

impl DerefMut for AppStateManagerRx {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rx
    }
}

pub(crate) enum AppStateEvent {
    GetAppState(u64, oneshot::Sender<AppState>), // TaskManager 启动任务时获取应用状态
    GetAppRawState(u64, oneshot::Sender<ApplicationState>),
    ChangeAppState(u64, ApplicationState), /* 应用状态变化，如果是前台切换到后台，
                                            * 则启动定时器。
                                            * 如果在定时器定时过程中重新恢复到前台，
                                            * 则取消定时器 */
    TriggerAppStateChange(u64, ApplicationState), // 前台切后台定时器触发，改变任务状态。
}

struct AppStateInfo {
    state: AppState,
    handle: Option<JoinHandle<()>>,
}

impl AppStateInfo {
    fn new(state: AppState) -> Self {
        Self {
            state,
            handle: None,
        }
    }
}

pub(crate) struct AppState {
    uid: u64,
    inner: Arc<Mutex<Inner>>,
    app_state_manager: AppStateManagerTx,
}

impl AppState {
    pub(crate) fn new(
        uid: u64,
        state: ApplicationState,
        app_state_manager: AppStateManagerTx,
    ) -> Self {
        Self {
            uid,
            inner: Arc::new(Mutex::new(Inner::new(state))),
            app_state_manager,
        }
    }

    pub(crate) fn state(&self) -> ApplicationState {
        self.inner.lock().unwrap().state
    }

    pub(crate) fn is_foreground(&self) -> bool {
        let lock = self.inner.lock().unwrap();
        lock.state == ApplicationState::Foreground
    }
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            uid: self.uid,
            inner: self.inner.clone(),
            app_state_manager: self.app_state_manager.clone(),
        }
    }
}

struct Inner {
    state: ApplicationState,
}

impl Inner {
    fn new(state: ApplicationState) -> Self {
        Self { state }
    }
}

async fn update_background_app(uid: u64, state: ApplicationState, tx: AppStateManagerTx) {
    sleep(Duration::from_secs(BACKGROUND_TASK_STOP_INTERVAL)).await;
    tx.trigger_app_state_change(uid, state);
}
