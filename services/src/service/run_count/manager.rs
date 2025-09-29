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

use std::collections::HashMap;

use ylong_runtime::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use ylong_runtime::sync::oneshot::{self, Sender};
cfg_oh! {
    use ipc::remote::RemoteObj;
    use crate::ability::PANIC_INFO;
}

use super::{Client, RunCountEvent};
use crate::error::ErrorCode;
use crate::utils::runtime_spawn;

#[derive(Clone)]
pub(crate) struct RunCountManagerEntry {
    tx: UnboundedSender<RunCountEvent>,
}

impl RunCountManagerEntry {
    pub(crate) fn new(tx: UnboundedSender<RunCountEvent>) -> Self {
        Self { tx }
    }

    pub(crate) fn send_event(&self, event: RunCountEvent) -> bool {
        if self.tx.send(event).is_err() {
            #[cfg(feature = "oh")]
            unsafe {
                if let Some(e) = PANIC_INFO.as_ref() {
                    error!("Sends RunCountManager event failed {}", e);
                    sys_event!(
                        ExecFault,
                        DfxCode::UDS_FAULT_02,
                        &format!("Sends RunCountManager event failed {}", e)
                    );
                } else {
                    info!("RunCountManager is unloading");
                }
            }
            return false;
        }
        true
    }
    #[cfg(feature = "oh")]
    pub(crate) fn subscribe_run_count(&self, pid: u64, obj: RemoteObj) -> ErrorCode {
        let (tx, rx) = oneshot::channel::<ErrorCode>();
        let event = RunCountEvent::Subscribe(pid, obj, tx);
        self.send_event(event);
        match ylong_runtime::block_on(rx) {
            Ok(error_code) => error_code,
            Err(error) => {
                error!("In `subscribe_run_count`, block on failed, err {}", error);
                // todo: may be another error code
                ErrorCode::Other
            }
        }
    }

    pub(crate) fn unsubscribe_run_count(&self, pid: u64) -> ErrorCode {
        let (tx, rx) = oneshot::channel::<ErrorCode>();
        let event = RunCountEvent::Unsubscribe(pid, tx);
        self.send_event(event);
        ylong_runtime::block_on(rx).unwrap()
    }

    #[cfg(feature = "oh")]
    pub(crate) fn notify_run_count(&self, new_count: usize) {
        let event = RunCountEvent::Change(new_count);
        self.send_event(event);
    }
}

pub(crate) struct RunCountManager {
    count: usize,
    remotes: HashMap<u64, Client>,
    rx: UnboundedReceiver<RunCountEvent>,
}

impl RunCountManager {
    pub(crate) fn init() -> RunCountManagerEntry {
        debug!("RunCountManager init");
        let (tx, rx) = unbounded_channel();
        let run_count_manager = RunCountManager {
            count: 0,
            remotes: HashMap::new(),
            rx,
        };
        runtime_spawn(run_count_manager.run());
        RunCountManagerEntry::new(tx)
    }

    async fn run(mut self) {
        loop {
            let recv = match self.rx.recv().await {
                Ok(message) => message,
                Err(e) => {
                    error!("RunCountManager recv error {:?}", e);
                    sys_event!(
                        ExecFault,
                        DfxCode::UDS_FAULT_03,
                        &format!("RunCountManager recv error {:?}", e)
                    );
                    continue;
                }
            };

            match recv {
                #[cfg(feature = "oh")]
                RunCountEvent::Subscribe(pid, obj, tx) => self.subscribe_run_count(pid, obj, tx),
                RunCountEvent::Unsubscribe(pid, tx) => self.unsubscribe_run_count(pid, tx),
                #[cfg(feature = "oh")]
                RunCountEvent::Change(change) => self.change_run_count(change),
            }

            debug!("RunCountManager handle message done");
        }
    }

    #[cfg(feature = "oh")]
    fn subscribe_run_count(&mut self, pid: u64, obj: RemoteObj, tx: Sender<ErrorCode>) {
        let client = Client::new(obj);

        let _ = client.notify_run_count(self.count as i64);
        self.remotes.insert(pid, client);

        let _ = tx.send(ErrorCode::ErrOk);
    }

    fn unsubscribe_run_count(&mut self, subscribe_pid: u64, tx: Sender<ErrorCode>) {
        if self.remotes.remove(&subscribe_pid).is_some() {
            let _ = tx.send(ErrorCode::ErrOk);
        } else {
            let _ = tx.send(ErrorCode::Other);
        }
    }

    #[cfg(feature = "oh")]
    fn change_run_count(&mut self, new_count: usize) {
        if self.count == new_count {
            return;
        }
        self.count = new_count;
        self.remotes
            .retain(|_, remote| remote.notify_run_count(self.count as i64).is_ok());
    }
}
