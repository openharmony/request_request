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

use std::sync::Arc;
use std::time::{self, Instant};

use ipc::remote::RemoteObj;
use samgr::definition::DOWNLOAD_SERVICE_ID;
use samgr::manage::SystemAbilityManager;

pub(crate) enum SaState {
    Ready(Arc<RemoteObj>),
    Invalid(time::Instant),
}

impl SaState {
    pub(crate) fn update() -> Self {
        for _ in 0..10 {
            match SystemAbilityManager::load_system_ability(DOWNLOAD_SERVICE_ID, 1000) {
                Some(remote) => {
                    return SaState::Ready(Arc::new(remote));
                }
                None => {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        }
        SaState::Invalid(Instant::now())
    }
}
