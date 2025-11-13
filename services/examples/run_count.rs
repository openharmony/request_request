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

//! run count example
use ipc::parcel::MsgParcel;
use ipc::remote::{RemoteObj, RemoteStub};
use samgr::definition::DOWNLOAD_SERVICE_ID;
use samgr::manage::SystemAbilityManager;

struct RunCount;
const SERVICE_TOKEN: &str = "OHOS.Download.RequestServiceInterface";
impl RemoteStub for RunCount {
    fn on_remote_request(
        &self,
        _code: u32,
        data: &mut ipc::parcel::MsgParcel,
        _reply: &mut ipc::parcel::MsgParcel,
    ) -> i32 {
        let token = data.read_interface_token().unwrap();
        assert_eq!(token, "OHOS.Download.NotifyInterface");
        let run_count: i64 = data.read().unwrap();
        println!("Run count: {}", run_count);
        0
    }
}

fn main() {
    let download_server = loop {
        if let Some(download_server) =
            SystemAbilityManager::check_system_ability(DOWNLOAD_SERVICE_ID)
        {
            break download_server;
        }
        SystemAbilityManager::load_system_ability(DOWNLOAD_SERVICE_ID, 15000).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
    };
    let mut data = MsgParcel::new();
    data.write_interface_token(SERVICE_TOKEN).unwrap();
    data.write_remote(RemoteObj::from_stub(RunCount).unwrap())
        .unwrap();
    download_server.send_request(16, &mut data).map_err(|_| 13400003)?;
    std::thread::sleep(std::time::Duration::from_secs(30000));
}
