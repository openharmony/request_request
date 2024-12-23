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
use ipc::IpcResult;

use crate::service::notification_bar::NotificationDispatcher;
use crate::service::RequestServiceStub;

impl RequestServiceStub {
    pub(crate) fn create_group(
        &self,
        data: &mut MsgParcel,
        reply: &mut MsgParcel,
    ) -> IpcResult<()> {
        let gauge: bool = data.read()?;
        let customized: bool = data.read()?;
        let title: String = data.read()?;
        let text: String = data.read()?;
        let new_group_id =
            NotificationDispatcher::get_instance().create_group(gauge, customized, title, text);
        reply.write(&new_group_id.to_string())?;
        Ok(())
    }
}
