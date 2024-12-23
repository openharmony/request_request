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

macro_rules! cfg_oh {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "oh")]
            $item
        )*
    }
}

macro_rules! cfg_not_oh {
    ($($item:item)*) => {
        $(
            #[cfg(not(feature = "oh"))]
            $item
        )*
    }
}

macro_rules! cvt_res_error {
    ($res: expr, $($args:tt)*) => {{
        match $res {
            Ok(value) => value,
            Err(e) => {
                error!($($args)*);
                error!("Error msg: {:?}", e);
                return Err(e);
            }
        }
    }}
}
