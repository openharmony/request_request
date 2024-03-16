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

use crate::service::ability::RequestAbility;
use crate::task::notify::{NotifyData, SubscribeType};

pub(crate) struct Notifier;

impl Notifier {
    pub(crate) fn service_front_notify(
        subscribe_type: SubscribeType,
        notify_data: NotifyData
    ) {
        let total_processed = notify_data.progress.common_data.total_processed;
        let file_total_size: i64 = notify_data.progress.sizes.iter().sum();
        if total_processed == 0 && file_total_size < 0 && subscribe_type.eq(&SubscribeType::Progress) {
            return;
        }

        RequestAbility::client_manager().send_notify_data(subscribe_type, notify_data)
    }

    pub(crate) fn remove_notify(data: NotifyData) {
        let tid = data.task_id;
        RequestAbility::client_manager().send_notify_data(SubscribeType::Remove, data);
        RequestAbility::client_manager().notify_task_finished(tid);
    }
}
