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

use crate::info::State;
use crate::service::client::ClientManagerEntry;
use crate::task::notify::{NotifyData, SubscribeType, WaitingCause};
use crate::task::reason::Reason;
pub(crate) struct Notifier;

impl Notifier {
    pub(crate) fn complete(client_manager: &ClientManagerEntry, notify_data: NotifyData) {
        #[cfg(feature = "oh")]
        let _ = publish_state_change_event(
            notify_data.bundle.as_str(),
            notify_data.task_id,
            State::Completed.repr as i32,
            notify_data.uid,
        );
        client_manager.send_notify_data(SubscribeType::Complete, notify_data)
    }

    pub(crate) fn fail(client_manager: &ClientManagerEntry, notify_data: NotifyData) {
        #[cfg(feature = "oh")]
        let _ = publish_state_change_event(
            notify_data.bundle.as_str(),
            notify_data.task_id,
            State::Failed.repr as i32,
            notify_data.uid,
        );
        client_manager.send_notify_data(SubscribeType::Fail, notify_data)
    }

    pub(crate) fn faults(tid: u32, client_manager: &ClientManagerEntry, reason: Reason) {
        client_manager.send_faults(tid, SubscribeType::FaultOccur, reason)
    }

    pub(crate) fn pause(client_manager: &ClientManagerEntry, notify_data: NotifyData) {
        client_manager.send_notify_data(SubscribeType::Pause, notify_data)
    }

    pub(crate) fn resume(client_manager: &ClientManagerEntry, notify_data: NotifyData) {
        client_manager.send_notify_data(SubscribeType::Resume, notify_data)
    }

    pub(crate) fn header_receive(client_manager: &ClientManagerEntry, notify_data: NotifyData) {
        client_manager.send_notify_data(SubscribeType::HeaderReceive, notify_data)
    }

    pub(crate) fn progress(client_manager: &ClientManagerEntry, notify_data: NotifyData) {
        let total_processed = notify_data.progress.common_data.total_processed;
        let file_total_size: i64 = notify_data.progress.sizes.iter().sum();
        if total_processed == 0 && file_total_size < 0 {
            return;
        }
        client_manager.send_notify_data(SubscribeType::Progress, notify_data)
    }

    pub(crate) fn remove(client_manager: &ClientManagerEntry, notify_data: NotifyData) {
        let task_id = notify_data.task_id;
        client_manager.send_notify_data(SubscribeType::Remove, notify_data);
        client_manager.notify_task_finished(task_id);
    }

    pub(crate) fn waiting(client_manager: &ClientManagerEntry, task_id: u32, cause: WaitingCause) {
        client_manager.send_wait_reason(task_id, cause);
    }
}

#[cfg(feature = "oh")]
pub(crate) fn publish_state_change_event(
    bundle_name: &str,
    task_id: u32,
    state: i32,
    uid: u64,
) -> Result<(), ()> {
    match crate::utils::PublishStateChangeEvent(bundle_name, task_id, state, uid as i32) {
        true => Ok(()),
        false => Err(()),
    }
}
#[allow(unused)]
#[cfg(test)]
mod ut_notifier {
    include!("../../tests/ut/manage/ut_notifier.rs");
}
