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

pub(crate) mod c_wrapper;
pub(crate) mod filter;
pub(crate) mod form_item;
pub(crate) mod task_id_generator;

use std::collections::HashMap;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use ylong_runtime::sync::oneshot::Receiver;

pub(crate) struct Recv<T> {
    rx: Receiver<T>,
}

impl<T> Recv<T> {
    pub(crate) fn new(rx: Receiver<T>) -> Self {
        Self { rx }
    }

    pub(crate) fn get(self) -> Option<T> {
        // Here `self.rx` can never be hung up.
        ylong_runtime::block_on(self.rx).ok()
    }
}

pub(crate) fn build_vec<A, B, C>(ptr: *const A, len: usize, func: C) -> Vec<B>
where
    C: Fn(&A) -> B,
{
    if ptr.is_null() || len == 0 {
        return Vec::<B>::new();
    }
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    slice.iter().map(func).collect()
}

pub(crate) fn get_current_timestamp() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() as u64,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

pub(crate) fn hashmap_to_string(map: &HashMap<String, String>) -> String {
    let mut res = Vec::new();
    for (n, (k, v)) in map.iter().enumerate() {
        if n != 0 {
            let _ = write!(res, "\r\n");
        }
        let _ = write!(res, "{k}\t{v}");
    }
    unsafe { String::from_utf8_unchecked(res) }
}

pub(crate) fn string_to_hashmap(str: &mut String) -> HashMap<String, String> {
    let mut map = HashMap::<String, String>::new();
    if str.is_empty() {
        return map;
    }
    for item in str.split("\r\n") {
        let (k, v) = item.split_once('\t').unwrap();
        map.insert(k.into(), v.into());
    }
    map
}

pub(crate) fn split_string(str: &mut str) -> std::str::Split<'_, &str> {
    let pat: &[_] = &['[', ']'];
    str.trim_matches(pat).split(", ")
}
