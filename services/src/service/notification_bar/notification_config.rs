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

#[cfg(feature = "oh")]
use ipc::parcel::Deserialize;

pub(crate) struct NotificationConfig {
    pub(crate) task_id: u32,
    pub(crate) title: Option<String>,
    pub(crate) text: Option<String>,
    pub(crate) disable: bool,
    pub(crate) visibility: u32,
}

#[cfg(test)]
impl NotificationConfig {
    pub(crate) fn new(
        task_id: u32,
        title: Option<String>,
        text: Option<String>,
        disable: bool,
        visibility: u32,
    ) -> Self {
        Self {
            task_id,
            title,
            text,
            disable,
            visibility,
        }
    }
}

#[cfg(feature = "oh")]
impl Deserialize for NotificationConfig {
    fn deserialize(parcel: &mut ipc::parcel::MsgParcel) -> ipc::IpcResult<Self> {
        let title = if parcel.read::<bool>()? {
            Some(parcel.read::<String>()?)
        } else {
            None
        };

        let text = if parcel.read::<bool>()? {
            Some(parcel.read::<String>()?)
        } else {
            None
        };
        let disable = parcel.read::<bool>()?;
        let visibility = parcel.read::<u32>()?;

        let config = NotificationConfig {
            task_id: 0,
            title,
            text,
            disable,
            visibility,
        };
        Ok(config)
    }
}
