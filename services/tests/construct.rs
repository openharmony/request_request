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

use download_server::config::ConfigBuilder;
use download_server::interface;
use ipc::parcel::MsgParcel;
use ipc::remote::RemoteObj;
const SERVICE_TOKEN: &str = "OHOS.Download.RequestServiceInterface";

fn test_init() -> RemoteObj {
    #[cfg(gn_test)]
    {
        use super::test_init;
        test_init()
    }
    #[cfg(not(gn_test))]
    {
        let ptr = std::ptr::null_mut::<ipc::cxx_share::IRemoteObject>();
        unsafe { RemoteObj::from_ciremote(ptr).unwrap() }
    }
}

#[test]
fn sdv_construct_basic() {
    let download_server = test_init();

    let mut data = MsgParcel::new();
    data.write_interface_token(SERVICE_TOKEN).unwrap();
    let mut reply = download_server
        .send_request(interface::OPEN_CHANNEL, &mut data)
        .unwrap();
    let ret: i32 = reply.read().unwrap();
    assert_eq!(0, ret);
    let mut _file = reply.read_file().unwrap();

    let config = ConfigBuilder::new().build();
    let mut data = MsgParcel::new();
    data.write_interface_token(SERVICE_TOKEN).unwrap();
    data.write(&config).unwrap();
    let mut reply = download_server.send_request(0, &mut data).unwrap();
    let ret: i32 = reply.read().unwrap();
    assert_eq!(ret, 0);
}
