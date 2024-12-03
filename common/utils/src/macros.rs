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

#[macro_export]
macro_rules! cfg_test {
    ($($item:item)*) => {
        $(
            #[cfg(test)]
            $item
        )*
    }
}

#[macro_export]
macro_rules! cfg_not_test {
    ($($item:item)*) => {
        $(
            #[cfg(not(test))]
            $item
        )*
    }
}

#[macro_use]
#[macro_export]
macro_rules! cfg_ohos {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "ohos")]
            $item
        )*
    }
}

#[macro_use]
#[macro_export]
macro_rules! cfg_not_ohos {
    ($($item:item)*) => {
        $(
            #[cfg(not(feature = "ohos"))]
            $item
        )*
    }
}