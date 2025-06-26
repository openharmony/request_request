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

use ipc::parcel::MsgParcel;
use ipc::remote;
use request_core::config::Action;
use request_core::filter::SearchFilter;
use request_core::info::{State, TaskInfo};
use request_core::interface;

use crate::proxy::{RequestProxy, SERVICE_TOKEN};

impl RequestProxy {
    pub(crate) fn query(&self, task_id: i64) -> Result<(), i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&1u32).unwrap();
        data.write(&task_id.to_string()).unwrap();

        let mut reply = remote.send_request(interface::QUERY, &mut data).unwrap();

        let code = reply.read::<i32>().unwrap(); // error code

        todo!()
    }

    pub(crate) fn query_mime_type(&self, task_id: i64) -> Result<String, i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&task_id.to_string()).unwrap();

        let mut reply = remote
            .send_request(interface::QUERY_MIME_TYPE, &mut data)
            .unwrap();

        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }

        let mime_type = reply.read::<String>().unwrap();
        Ok(mime_type)
    }

    pub(crate) fn show(&self, task_id: i64) -> Result<TaskInfo, i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&1u32).unwrap();
        data.write(&task_id.to_string()).unwrap();

        let mut reply = remote.send_request(interface::SHOW, &mut data).unwrap();

        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }

        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }
        let task_info = reply.read::<TaskInfo>().unwrap(); // task info
        Ok(task_info)
    }

    pub(crate) fn touch(&self, task_id: i64, token: String) -> Result<(), i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&1u32).unwrap();
        data.write(&task_id.to_string()).unwrap();
        data.write(&token).unwrap(); // token?

        let mut reply = remote.send_request(interface::TOUCH, &mut data).unwrap();

        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }
        todo!()
    }

    pub(crate) fn search(&self, filter: SearchFilter) -> Result<Vec<String>, i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        match filter.bundle_name {
            Some(ref bundle) => data.write(bundle).unwrap(),
            None => data.write(&"*".to_string()).unwrap(),
        }

        // Serialize the filter into the parcel
        match filter.before {
            Some(before) => data.write(&before).unwrap(),
            None => data.write(&-1i64).unwrap(),
        }

        match filter.after {
            Some(after) => data.write(&after).unwrap(),
            None => data.write(&-1i64).unwrap(),
        }

        match filter.state {
            Some(state) => data.write(&(state as u32)).unwrap(),
            None => data.write(&(State::Any as u32)).unwrap(),
        }

        match filter.action {
            Some(action) => data.write(&(action as u32)).unwrap(),
            None => data.write(&(2u32)).unwrap(),
        }

        match filter.mode {
            Some(mode) => data.write(&(mode as u32)).unwrap(),
            None => data.write(&02u32).unwrap(),
        }

        let mut reply = remote.send_request(interface::SEARCH, &mut data).unwrap();

        let len = reply.read::<u32>().unwrap(); // error code
        let mut ids = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let id = reply.read::<String>().unwrap();
            ids.push(id);
        }
        Ok(ids)
    }

    pub(crate) fn get_task(&self, task_id: i64, token: String) -> Result<(), i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&task_id.to_string()).unwrap();

        data.write(&token).unwrap(); // token

        let mut reply = remote.send_request(interface::GET_TASK, &mut data).unwrap();

        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }

        // Handle the task details if needed
        todo!()
    }
}
