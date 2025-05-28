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

mod state;
mod task;
mod uds;

const SERVICE_TOKEN: &str = "OHOS.Download.RequestServiceInterface";

use std::sync::{Arc, LazyLock, Mutex};

use ipc::remote::RemoteObj;
use state::SaState;

pub struct RequestProxy {
    remote: Mutex<SaState>,
}

impl RequestProxy {
    pub fn get_instance() -> &'static Self {
        static REQUEST_PROXY: LazyLock<RequestProxy> = LazyLock::new(|| RequestProxy {
            remote: Mutex::new(SaState::update()),
        });
        &REQUEST_PROXY
    }

    pub(crate) fn remote(&self) -> Option<Arc<RemoteObj>> {
        let mut remote = self.remote.lock().unwrap();
        match *remote {
            SaState::Ready(ref obj) => return Some(obj.clone()),
            SaState::Invalid(ref time) => {
                if time.elapsed().as_secs() > 10 {
                    *remote = SaState::update();
                    if let SaState::Ready(ref obj) = *remote {
                        return Some(obj.clone());
                    }
                }
            }
        }
        None
    }
}
