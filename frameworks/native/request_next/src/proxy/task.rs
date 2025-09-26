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
use request_core::config::TaskConfig;
use request_core::interface;

use super::{RequestProxy, SERVICE_TOKEN};
use crate::client::error::CreateTaskError;

impl RequestProxy {
    pub(crate) fn create(&self, config: TaskConfig) -> Result<i64, CreateTaskError> {
        let remote = self.remote()?;
        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&1u32).unwrap();
        data.write(&config).unwrap();

        data.write(&false).unwrap();
        data.write(&false).unwrap();

        let mut reply = remote
            .send_request(interface::CONSTRUCT, &mut data)
            .unwrap();

        let code = reply.read::<i32>().unwrap();
        if code != 0 {
            return Err(CreateTaskError::Code(code));
        }

        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(CreateTaskError::Code(code));
        }
        let task_id = reply.read::<u32>().unwrap();

        Ok(task_id as i64)
    }

    pub(crate) fn start(&self, task_id: i64) -> Result<(), i32> {
        let remote = self.remote()?;
        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&1u32).unwrap();
        data.write(&task_id.to_string()).unwrap();

        let mut reply = remote.send_request(interface::START, &mut data).unwrap();
        let code = reply.read::<i32>().unwrap(); // error code
        if code == 0 {
            Ok(())
        } else {
            Err(code)
        }
    }

    pub(crate) fn pause(&self, task_id: i64) -> Result<(), i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&1u32).unwrap(); // version
        data.write(&1u32).unwrap(); // task count
        data.write(&task_id.to_string()).unwrap();

        let mut reply = remote.send_request(interface::PAUSE, &mut data).unwrap();

        let code = reply.read::<i32>().unwrap(); // error code
        if code == 0 {
            Ok(())
        } else {
            Err(code)
        }
    }

    pub(crate) fn resume(&self, task_id: i64) -> Result<(), i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&1u32).unwrap(); // task count
        data.write(&task_id.to_string()).unwrap();

        let mut reply = remote.send_request(interface::RESUME, &mut data).unwrap();

        let code = reply.read::<i32>().unwrap(); // error code
        if code == 0 {
            Ok(())
        } else {
            Err(code)
        }
    }

    pub(crate) fn remove(&self, task_id: i64) -> Result<(), i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&1u32).unwrap(); // task count
        data.write(&task_id.to_string()).unwrap();

        let mut reply = remote.send_request(interface::REMOVE, &mut data).unwrap();

        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }

        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }

        Ok(())
    }

    pub(crate) fn stop(&self, task_id: i64) -> Result<(), i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&1u32).unwrap(); // task count
        data.write(&task_id.to_string()).unwrap();

        let mut reply = remote.send_request(interface::STOP, &mut data).unwrap();

        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }

        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }

        Ok(())
    }

    pub(crate) fn set_max_speed(&self, task_id: i64, speed: i64) -> Result<(), i32> {
        let remote = self.remote()?;

        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&1u32).unwrap(); // task count
        data.write(&task_id.to_string()).unwrap();
        data.write(&speed).unwrap();

        let mut reply = remote
            .send_request(interface::SET_MAX_SPEED, &mut data)
            .unwrap();

        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }

        let code = reply.read::<i32>().unwrap(); // error code
        if code != 0 {
            return Err(code);
        }
        Ok(())
    }
}
