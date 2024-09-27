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

use std::fmt::Display;

use cxx::UniquePtr;
use ffi::WantWrapper;
pub struct EventHandler {
    inner: Box<dyn CommonEventSubscriber>,
}

impl EventHandler {
    #[inline]
    fn new(inner: Box<dyn CommonEventSubscriber>) -> Self {
        Self { inner }
    }
}

pub trait CommonEventSubscriber {
    fn on_receive_event(&self, code: i32, data: String, want: Want);
}

impl EventHandler {
    #[inline]
    fn on_receive_event(&self, code: i32, data: String, want: UniquePtr<WantWrapper>) {
        self.inner.on_receive_event(code, data, Want::new(want));
    }
}

pub struct Want {
    inner: UniquePtr<ffi::WantWrapper>,
}

impl Want {
    #[inline]
    fn new(inner: UniquePtr<WantWrapper>) -> Self {
        Self { inner }
    }

    pub(crate) fn get_int_param(&self, key: &str) -> Option<i32> {
        let res = self.inner.GetIntParam(key);
        if res == -1 {
            None
        } else {
            Some(res)
        }
    }
}

// VALUE_TYPE_BOOLEAN = 1,
// VALUE_TYPE_BYTE = 2,
// VALUE_TYPE_CHAR = 3,
// VALUE_TYPE_SHORT = 4,
// VALUE_TYPE_INT = 5,
// VALUE_TYPE_LONG = 6,
// VALUE_TYPE_FLOAT = 7,
// VALUE_TYPE_DOUBLE = 8,
// VALUE_TYPE_STRING = 9,
// VALUE_TYPE_ARRAY = 102,
impl Display for Want {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.ToString())
    }
}

pub fn subscribe_common_event<T: CommonEventSubscriber + 'static>(
    events: Vec<&str>,
    handler: T,
) -> Result<(), i32> {
    let res = ffi::SubscribeCommonEvent(events, Box::new(EventHandler::new(Box::new(handler))));
    if res == 0 {
        Ok(())
    } else {
        Err(res)
    }
}

#[allow(unused)]
#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {

    extern "Rust" {
        type EventHandler;
        fn on_receive_event(&self, code: i32, data: String, want: UniquePtr<WantWrapper>);
    }
    unsafe extern "C++" {
        include!("common_event.h");
        include!("common_event_data.h");
        type WantWrapper;

        fn ToString(self: &WantWrapper) -> String;
        fn GetIntParam(self: &WantWrapper, key: &str) -> i32;

        fn SubscribeCommonEvent(events: Vec<&str>, handler: Box<EventHandler>) -> i32;
    }
}
