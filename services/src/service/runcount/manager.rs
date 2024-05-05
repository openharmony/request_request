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

use ipc::remote::RemoteObj;
use ylong_runtime::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use ylong_runtime::sync::oneshot::Sender;

use super::{RunCountEvent, SubClient, SubKey};
use crate::error::ErrorCode;
use crate::init::PANIC_INFO;

#[derive(Clone)]
pub(crate) struct RunCountManagerEntry {
    tx: UnboundedSender<RunCountEvent>,
}

impl RunCountManagerEntry {
    fn new(tx: UnboundedSender<RunCountEvent>) -> Self {
        Self { tx }
    }

    pub(crate) fn send_event(&self, event: RunCountEvent) -> bool {
        if self.tx.send(event).is_err() {
            unsafe {
                if let Some(e) = PANIC_INFO.as_ref() {
                    error!("Sends RunCountManager event failed {}", e);
                } else {
                    info!("RunCountManager is unloading");
                }
            }
            return false;
        }
        true
    }
}

pub(crate) struct RunCountManager {
    runcount: i64,
    remotes: HashMap<SubKey, SubClient>,
    rx: UnboundedReceiver<RunCountEvent>,
}

impl RunCountManager {
    pub(crate) fn init() -> RunCountManagerEntry {
        debug!("RunCountManager init");
        let (tx, rx) = unbounded_channel();
        let runcount_manager = RunCountManager {
            runcount: 0,
            remotes: HashMap::new(),
            rx,
        };
        ylong_runtime::spawn(runcount_manager.run());
        RunCountManagerEntry::new(tx)
    }

    async fn run(mut self) {
        loop {
            let recv = match self.rx.recv().await {
                Ok(message) => message,
                Err(e) => {
                    error!("RunCountManager recv error {:?}", e);
                    continue;
                }
            };

            match recv {
                RunCountEvent::Sub(subkey, obj, tx) => self.handle_sub_runcount(subkey, obj, tx),
                RunCountEvent::Unsub(subkey, tx) => self.handle_unsub_runcount(subkey, tx),
                RunCountEvent::Change(change) => self.handle_change_runcount(change),
            }

            debug!("RunCountManager handle message done");
        }
    }

    fn handle_sub_runcount(&mut self, subkey: SubKey, obj: RemoteObj, tx: Sender<ErrorCode>) {
        debug!("handle sub runcount in");
        let subclient = SubClient::new(obj);

        subclient.notify_runcount(self.runcount);
        if self.remotes.get(&subkey).is_none() {
            self.remotes.insert(subkey, subclient);
            debug!("RunCountManager has inserted subkey: {:?}", subkey);
        }

        let _ = tx.send(ErrorCode::ErrOk);
        // Need to notify client immediately, then client get runcount by its
        // callback
    }

    fn handle_unsub_runcount(&mut self, subkey: SubKey, tx: Sender<ErrorCode>) {
        if self.remotes.remove(&subkey).is_some() {
            debug!("RunCountManager removes subkey: {:?}", subkey);
            // Sends error code immediately, ignore the result.
            let _ = tx.send(ErrorCode::ErrOk);
        } else {
            error!("RunCountManager removes subkey failed: {:?}", subkey);
            // Sends error code immediately, ignore the result.
            let _ = tx.send(ErrorCode::Other);
        }
    }

    fn handle_change_runcount(&mut self, change: i64) {
        debug!("handle change runcount in");
        self.runcount += change;
        self.handle_notify_runcount();
    }

    fn handle_notify_runcount(&self) {
        debug!("handle notify runcount to all subclient");
        for (_, subclient) in self.remotes.iter() {
            subclient.notify_runcount(self.runcount)
        }
    }
}
