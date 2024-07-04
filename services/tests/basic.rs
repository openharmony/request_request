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

use download_server::interface;

#[test]
fn sdv_interface() {
    assert_eq!(0, interface::CONSTRUCT);
    assert_eq!(1, interface::PAUSE);
    assert_eq!(2, interface::QUERY);
    assert_eq!(3, interface::QUERY_MIME_TYPE);
    assert_eq!(4, interface::REMOVE);
    assert_eq!(5, interface::RESUME);
    assert_eq!(6, interface::START);
    assert_eq!(7, interface::STOP);
    assert_eq!(8, interface::SHOW);
    assert_eq!(9, interface::TOUCH);
    assert_eq!(10, interface::SEARCH);
    assert_eq!(11, interface::GET_TASK);
    assert_eq!(12, interface::CLEAR);
    assert_eq!(13, interface::OPEN_CHANNEL);
    assert_eq!(14, interface::SUBSCRIBE);
    assert_eq!(15, interface::UNSUBSCRIBE);
    assert_eq!(16, interface::SUB_RUN_COUNT);
    assert_eq!(17, interface::UNSUB_RUN_COUNT);
}
