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

use crate::notify::{Event, NotifyEvent};
use crate::service::ability::RequestAbility;
use crate::task::config::Version;
use crate::task::info::ApplicationState;
use crate::task::notify::NotifyData;
use crate::task::RequestTask;
pub(crate) struct Notifier;

impl Notifier {
    pub(crate) fn service_front_notify(
        event: String,
        notify_data: NotifyData,
        app_state: &Arc<AtomicU8>,
    ) {
        let total_processed = notify_data.progress.common_data.total_processed;
        let file_total_size: i64 = notify_data.progress.sizes.iter().sum();
        if total_processed == 0 && file_total_size < 0 && event.eq("progress") {
            return;
        }

        if ApplicationState::from(app_state.load(Ordering::SeqCst)) != ApplicationState::Foreground
            && (notify_data.version == Version::API10 || event.eq("progress"))
        {
            return;
        }
        let event = match event.try_into() {
            Ok(event) => event,
            Err(e) => {
                error!("TaskManager notify try_into failed {:?}", e);
                return;
            }
        };

        let event = NotifyEvent::notify(event, notify_data);
        RequestAbility::notify().send_event(event);
    }

    pub(crate) fn remove_notify(task: &Arc<RequestTask>) {
        let data = task.build_notify_data();
        let event = NotifyEvent::notify(Event::Remove, data);
        RequestAbility::notify().send_event(event);
    }

    pub(crate) fn clear_notify(task: &Arc<RequestTask>) {
        let event = NotifyEvent::clear(task.conf.common_data.task_id);
        RequestAbility::notify().send_event(event);
    }
}
