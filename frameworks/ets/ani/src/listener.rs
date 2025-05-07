// Copyright (C) 2025 Huawei Device Co., Ltd.
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

use std::sync::{LazyLock, Mutex};

use crate::proxy::RequestProxy;
use crate::wrapper;

pub struct UdsListener {
    state: Mutex<ChannelState>,
}

enum ChannelState {
    Open,
    Close,
}

pub struct ResponseCallback {}

pub struct NotifyCallback {}

pub enum NotifyType {}

impl UdsListener {
    pub(crate) fn get_instance() -> &'static UdsListener {
        static INSTANCE: LazyLock<UdsListener> = LazyLock::new(|| UdsListener {
            state: Mutex::new(ChannelState::Close),
        });
        &INSTANCE
    }

    pub(crate) fn add_response_listener(&self, task_id: &str, callback: ResponseCallback) {}

    pub(crate) fn remove_response_listener(&self, task_id: &str, callback: ResponseCallback) {}

    pub(crate) fn add_notify_data_listener(&self, task_id: &str, callback: NotifyCallback) {
        self.ensure_channel_open();
    }

    pub(crate) fn remove_notify_data_listener(&self, task_id: &str, callback: NotifyCallback) {}

    pub(crate) fn on_response(&self, response: wrapper::ffi::Response) {
        info!(
            "on_response: taskId: {}, version: {}, statusCode: {}, reason: {}, headers: {:?}",
            response.taskId,
            response.version,
            response.statusCode,
            response.reason,
            response.headers
        );
    }

    pub(crate) fn on_notify(&self, task_id: &str) {}

    pub(crate) fn ensure_channel_open(&self) {
        let channel = RequestProxy::get_instance().open_channel();
        wrapper::ffi::OpenChannel(channel);
    }
}
