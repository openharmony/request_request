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

use ipc::parcel::MsgParcel;
use ipc::{IpcResult, IpcStatusCode};

use crate::error::ErrorCode;
use crate::manage::events::TaskManagerEvent;
use crate::service::notification_bar::NotificationDispatcher;
use crate::service::permission::PermissionChecker;
use crate::service::RequestServiceStub;
use crate::task::config::TaskConfig;

impl RequestServiceStub {
    pub(crate) fn construct(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        info!("Service construct");

        if !PermissionChecker::check_internet() {
            error!("End Service construct, failed: no INTERNET permission");
            reply.write(&(ErrorCode::Permission as i32))?;
            return Err(IpcStatusCode::Failed);
        }
        let task_config: TaskConfig = match data.read() {
            Ok(config) => config,
            Err(_e) => {
                reply.write(&(ErrorCode::IpcSizeTooLarge as i32))?;
                return Err(IpcStatusCode::Failed);
            }
        };
        debug!("Service construct: task_config constructed");

        let (event, rx) = TaskManagerEvent::construct(task_config);
        if !self.task_manager.lock().unwrap().send_event(event) {
            return Err(IpcStatusCode::Failed);
        }
        let ret = match rx.get() {
            Some(ret) => ret,
            None => {
                error!("End Service construct, failed: receives ret failed");
                return Err(IpcStatusCode::Failed);
            }
        };

        let task_id = match ret {
            Ok(id) => id,
            Err(err_code) => {
                error!("End Service construct, failed: {:?}", err_code);
                reply.write(&(err_code as i32))?;
                return Err(IpcStatusCode::Failed);
            }
        };

        let customized_notification = data.read::<bool>()?;
        if customized_notification {
            let title = data.read::<String>()?;
            let text = data.read::<String>()?;
            NotificationDispatcher::get_instance()
                .update_task_customized_notification(task_id, title, text);
        }

        debug!("Service construct: construct event sent to manager");

        let uid = ipc::Skeleton::calling_uid();
        let token_id = ipc::Skeleton::calling_full_token_id();
        let pid = ipc::Skeleton::calling_pid();

        let ret = self.client_manager.subscribe(task_id, pid, uid, token_id);
        if ret != ErrorCode::ErrOk {
            error!("End Service subscribe, tid: {}, failed: {:?}", task_id, ret);
            reply.write(&(ret as i32))?;
            reply.write(&(task_id as i32))?;
            return Ok(());
        }

        reply.write(&(ErrorCode::ErrOk as i32))?;
        debug!("End Service construct, succeed with tid: {}", task_id);
        reply.write(&(task_id as i32))?;
        Ok(())
    }
}
