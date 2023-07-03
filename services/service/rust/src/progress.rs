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

use super::{c_string_wrapper::*, enumration::*, utils::*};
use std::collections::HashMap;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CommonProgress {
    pub state: u8,
    pub index: usize,
    pub total_processed: usize,
}

#[derive(Debug)]
pub struct Progress {
    pub common_data: CommonProgress,
    pub sizes: Vec<i64>,
    pub processed: Vec<usize>,
    pub extras: HashMap<String, String>,
}

#[repr(C)]
pub struct CProgress {
    pub common_data: CommonProgress,
    pub sizes: CStringWrapper,
    pub processed: CStringWrapper,
    pub extras: CStringWrapper,
}

impl Progress {
    pub fn new(sizes: Vec<i64>) -> Self {
        let len = sizes.len();
        Progress {
            common_data: CommonProgress {
                state: State::CREATED as u8,
                index: 0,
                total_processed: 0,
            },
            sizes,
            processed: vec![0; len],
            extras: HashMap::<String, String>::new(),
        }
    }

    pub fn to_c_struct(&self, sizes: &String, processed: &String, extras: &String) -> CProgress {
        CProgress {
            common_data: self.common_data,
            sizes: CStringWrapper::from(sizes),
            processed: CStringWrapper::from(processed),
            extras: CStringWrapper::from(extras),
        }
    }

    pub fn from_c_struct(c_struct: &CProgress) -> Self {
        Progress {
            common_data: c_struct.common_data,
            sizes: {
                let mut str = c_struct.sizes.to_string();
                let mut v = Vec::new();
                for s in split_string(&mut str) {
                    v.push(s.parse::<i64>().unwrap());
                }
                v
            },
            processed: {
                let mut str = c_struct.processed.to_string();
                let mut v = Vec::new();
                for s in split_string(&mut str) {
                    v.push(s.parse::<usize>().unwrap());
                }
                v
            },
            extras: {
                let mut str = c_struct.extras.to_string();
                string_to_hashmap(&mut str)
            },
        }
    }
}

impl Clone for Progress {
    fn clone(&self) -> Self {
        Progress {
            common_data: self.common_data,
            sizes: self.sizes.clone(),
            processed: self.processed.clone(),
            extras: self.extras.clone(),
        }
    }
}

#[repr(C)]
pub struct RequestTaskMsg {
    pub taskId: u32,
    pub uid: i32,
    pub action: u8,
}
