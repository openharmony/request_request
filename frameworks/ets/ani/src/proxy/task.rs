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

use std::collections::HashMap;
use std::os::fd::{IntoRawFd, RawFd};

use ipc::parcel::MsgParcel;
use request_core::{interface, TaskConfig};

use super::{RequestProxy, SERVICE_TOKEN};
use crate::api9::DownloadConfig;

impl RequestProxy {
    pub(crate) fn create<C: Into<TaskConfig>>(&self, config: C) -> String {
        let Some(remote) = self.remote() else { todo!() };
        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();
        let config: TaskConfig = config.into();
        data.write(&1u32).unwrap();
        data.write(&config).unwrap();
        data.write(&false).unwrap();
        data.write(&false).unwrap();
        let mut reply = remote
            .send_request(interface::CONSTRUCT, &mut data)
            .unwrap();

        let code = reply.read::<i32>().unwrap();
        let code = reply.read::<i32>().unwrap();
        let task_id = reply.read::<u32>().unwrap();
        task_id.to_string()
    }

    pub(crate) fn start(&self, task_id: String) {
        let Some(remote) = self.remote() else { todo!() };
        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&1u32).unwrap();
        data.write(&task_id).unwrap();
        let mut reply = remote.send_request(interface::START, &mut data).unwrap();
    }

    pub(crate) fn open_channel(&self) -> RawFd {
        let Some(remote) = self.remote() else { todo!() };
        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        let mut reply = remote
            .send_request(interface::OPEN_CHANNEL, &mut data)
            .unwrap();
        let code = reply.read::<i32>().unwrap();
        let fd = reply.read_file().unwrap();
        info!("open channel fd: {:?}", fd);
        fd.into_raw_fd()
    }

    pub(crate) fn subscribe(&self, task_id: String) {
        let Some(remote) = self.remote() else { todo!() };
        let mut data = MsgParcel::new();

        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&task_id).unwrap();
        let mut reply = remote
            .send_request(interface::SUBSCRIBE, &mut data)
            .unwrap();
    }
}
