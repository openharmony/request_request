/*
 * Copyright (C) 2023 Huawei Device Co., Ltd.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::ffi::{ c_char, CStr };

pub fn get_current_timestamp() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

pub fn convert_to_string(ptr: *const c_char) -> String {
    let c_str: &CStr = unsafe { CStr::from_ptr(ptr) };
    let str_slice: &str = c_str.to_str().unwrap();
    str_slice.to_owned()
}

pub fn generate_task_id() -> u32 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos()
}

pub fn hashmap_to_string(map: &HashMap<String, String>) -> String {
    let len = map.len();
    if len == 0 {
        return "".to_string();
    }
    let mut index = 0;
    let mut res = String::new();

    for (k, v) in map.iter() {
        res.push_str(k);
        res.push('\t');
        res.push_str(v);
        if index < len - 1 {
            res.push_str("\r\n");
        }
        index += 1;
    }
    res
}

pub fn string_to_hashmap(str: &mut String) -> HashMap<String, String> {
    let mut map = HashMap::<String, String>::new();
    if str.is_empty() {
        return map;
    }
    let v: Vec<&str> = str.split("\r\n").collect();
    for item in v.into_iter() {
        let x: Vec<&str> = item.split('\t').collect();
        map.insert(x[0].into(), x[1].into());
    }
    map
}

pub fn split_string(str: &mut String) -> std::str::Split<'_, &str> {
    let pat: &[_] = &['[', ']'];
    str.trim_matches(pat).split(", ")
}
