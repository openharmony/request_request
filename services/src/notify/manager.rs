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

use std::collections::HashMap;

use ipc_rust::{IRemoteObj, InterfaceToken, IpcResult, MsgParcel, RemoteObj};
use ylong_runtime::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use ylong_runtime::sync::oneshot::Sender;

use crate::error::ErrorCode;
use crate::notify::{Event, NotifyData, NotifyEvent};
use crate::service::RequestNotifyInterfaceCode;
use crate::task::info::State;

pub(crate) struct NotifyManager {
    rx: UnboundedReceiver<NotifyEvent>,
    remotes: HashMap<NotifyKey, Notifier>,
    unregistered: HashMap<NotifyKey, Box<NotifyData>>,
}

impl NotifyManager {
    fn new(rx: UnboundedReceiver<NotifyEvent>) -> Self {
        Self {
            rx,
            remotes: HashMap::new(),
            unregistered: HashMap::new(),
        }
    }

    pub(crate) fn init() -> NotifyEntry {
        info!("NotifyManager prepare to be inited");
        let (tx, rx) = unbounded_channel::<NotifyEvent>();
        ylong_runtime::spawn(Self::new(rx).run());
        let entry = NotifyEntry::new(tx);
        info!("NotifyManager is inited");
        entry
    }

    pub(crate) async fn run(mut self) {
        loop {
            let event = match self.rx.recv().await {
                Ok(message) => message,
                Err(e) => {
                    error!("Notifier recv error {:?}", e);
                    continue;
                }
            };
            match event {
                NotifyEvent::Notify(event, data) => self.notify(event, data),
                NotifyEvent::On(event, id, obj, tx) => self.on(event, id, obj, tx),
                NotifyEvent::Off(event, id, sender) => self.off(event, id, sender),
                NotifyEvent::Clear(id) => self.clear(id),
                NotifyEvent::Shutdown => {
                    info!("NotifyManager shuts down");
                    return;
                }
            }
        }
    }
}

impl NotifyManager {
    fn notify(&mut self, event: Event, data: Box<NotifyData>) {
        debug!("NotifyManager gets event: {}", event.as_str());
        let key = NotifyKey::new(event, data.task_id);
        if let Some(notifier) = self.remotes.get(&key) {
            debug!("NotifyManager finds key succeed: {:?}", key);
            // Ignores notify failed.
            notifier.notify(event, data);
        } else {
            debug!("NotifyManager finds key failed: {:?}", key);
            self.unregistered.insert(key, data);
        }
    }

    fn on(&mut self, event: Event, id: u32, obj: RemoteObj, sender: Sender<ErrorCode>) {
        let key = NotifyKey::new(event, id);
        let notifier = Notifier::new(obj);

        if let Some(data) = self.unregistered.remove(&key) {
            debug!("NotifyManager notifies unregistered key: {:?}", key);
            notifier.notify(event, data);
            self.unregistered.remove(&key);
        }
        self.remotes.insert(key, notifier);
        debug!("NotifyManager has inserted key: {:?}", key);
        let _ = sender.send(ErrorCode::ErrOk);
    }

    fn off(&mut self, event: Event, id: u32, sender: Sender<ErrorCode>) {
        let key = NotifyKey::new(event, id);
        if self.remotes.remove(&key).is_some() {
            debug!("NotifyManager removes key: {:?}", key);
            // Sends error code immediately, ignore the result.
            let _ = sender.send(ErrorCode::ErrOk);
        } else {
            error!("NotifyManager removes key failed: {:?}", key);
            // Sends error code immediately, ignore the result.
            let _ = sender.send(ErrorCode::Other);
        }
    }

    fn clear(&mut self, id: u32) {
        let events = [
            Event::Complete,
            Event::Fail,
            Event::HeaderReceive,
            Event::Pause,
            Event::Progress,
            Event::Remove,
            Event::Resume,
        ];
        // Clears objects and unregistered notify data of the target task.
        for event in events {
            let key = NotifyKey::new(event, id);
            self.remotes.remove(&key);
            self.unregistered.remove(&key);
        }
        debug!("NotifyManager has cleared all the key of Task: {:?}", id);
    }
}

#[derive(Clone)]
pub(crate) struct NotifyEntry {
    tx: UnboundedSender<NotifyEvent>,
}

impl NotifyEntry {
    fn new(tx: UnboundedSender<NotifyEvent>) -> Self {
        Self { tx }
    }

    pub(crate) fn shutdown(&self) {
        // Ignore the result.
        self.send_event(NotifyEvent::shutdown());
    }

    pub(crate) fn send_event(&self, event: NotifyEvent) {
        if self.tx.send(event).is_err() {
            error!("Sends NotifyEvent failed");
        }
    }
}

#[derive(Clone)]
struct Notifier {
    obj: RemoteObj,
}

impl Notifier {
    fn new(obj: RemoteObj) -> Self {
        Self { obj }
    }

    fn notify(&self, event: Event, data: Box<NotifyData>) {
        debug!("Notifier gets notify data: {:?}", data);
        if data.progress.common_data.index >= data.progress.sizes.len() {
            error!("During notify: index out of range");
            return;
        }
        let common_data = &data.progress.common_data;
        if (common_data.state == State::Running as u8 || common_data.state == State::Retrying as u8)
            && common_data.total_processed == 0
        {
            return;
        }

        let mut parcel = match MsgParcel::new() {
            Some(parcel) => parcel,
            None => {
                error!("During notify: create MsgParcel failed");
                return;
            }
        };

        if write_parcel(&mut parcel, event, data.as_ref()).is_err() {
            error!("During notify: ipc write failed");
            return;
        }

        debug!("During notify: send request");
        if let Err(e) =
            self.obj
                .send_request(RequestNotifyInterfaceCode::Notify as u32, &parcel, false)
        {
            error!("During notify: send request failed {:?}", e);
        }
        debug!("During notify: send request success");
    }
}

fn write_parcel(parcel: &mut MsgParcel, event: Event, data: &NotifyData) -> IpcResult<()> {
    parcel.write(&InterfaceToken::new("OHOS.Download.NotifyInterface"))?;
    parcel.write(&(event.as_str()))?;
    parcel.write(&(data.task_id.to_string()))?;
    parcel.write(&(data.progress.common_data.state as u32))?;

    let index = data.progress.common_data.index;
    parcel.write(&(index as u32))?;
    parcel.write(&(data.progress.processed[index] as u64))?;

    parcel.write(&(data.progress.common_data.total_processed as u64))?;
    parcel.write(&(data.progress.sizes))?;
    parcel.write(&(data.progress.extras.len() as u32))?;

    for (k, v) in data.progress.extras.iter() {
        parcel.write(&k)?;
        parcel.write(&v)?;
    }
    parcel.write(&(data.action as u32))?;
    parcel.write(&(data.version as u32))?;

    parcel.write(&(data.each_file_status.len() as u32))?;
    for status in data.each_file_status.iter() {
        parcel.write(&(status.path))?;
        parcel.write(&(status.reason as u32))?;
        parcel.write(&(status.message))?;
    }
    Ok(())
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct NotifyKey {
    event: Event,
    task_id: u32,
}

impl NotifyKey {
    fn new(event: Event, task_id: u32) -> Self {
        Self { event, task_id }
    }
}
