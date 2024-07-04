// Copyright (C) 2024 Huawei Device Co., Ltd.
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

#![cfg(gn_test)]

use ipc::remote::RemoteObj;
use samgr::definition::DOWNLOAD_SERVICE_ID;
use samgr::manage::SystemAbilityManager;
mod basic;
mod construct;
/// test init
pub fn test_init() -> RemoteObj {
    unsafe { SetAccessTokenPermission() };
    let mut count = 0;
    loop {
        if let Some(download_server) =
            SystemAbilityManager::check_system_ability(DOWNLOAD_SERVICE_ID)
        {
            return download_server;
        }
        SystemAbilityManager::load_system_ability(DOWNLOAD_SERVICE_ID, 15000).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        count += 1;
        println!("load download service {} seconds", count);
    }
}

extern "C" {
    fn SetAccessTokenPermission();
}
