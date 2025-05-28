// Copyright (C) 2025 Huawei Device Co., Ltd.
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

use std::collections::{hash_map, HashMap};
use std::sync::Mutex;

use crate::middle_layer::acl_set_access;

static SA_PERMISSION_RWX: &str = "g:3815:rwx";
static SA_PERMISSION_X: &str = "g:3815:x";
static SA_PERMISSION_CLEAN: &str = "g:3815:---";
static ACL_SUCC: i32 = 0;

pub(crate) struct AclMgr {
    // The key is (path, Whether it is file).
    path_map: Mutex<HashMap<(String, bool), usize>>,
}

impl AclMgr {
    pub(crate) fn add(&mut self, path: &str, is_file: bool) -> bool {
        let mut guard = self.path_map.lock().unwrap();
        match guard.entry((path.to_owned(), is_file)) {
            hash_map::Entry::Occupied(mut entry) => {
                if !acl_set_on(path, is_file) {
                    return false;
                };
                *entry.get_mut() += 1;
            }
            hash_map::Entry::Vacant(entry) => {
                entry.insert(1);
            }
        }
        true
    }

    pub(crate) fn sub(&mut self, path: &str, is_file: bool) {
        let mut guard = self.path_map.lock().unwrap();
        match guard.entry((path.to_owned(), is_file)) {
            hash_map::Entry::Occupied(mut entry) => {
                let count = entry.get_mut();
                if *count >= 1 {
                    *count -= 1;
                }
                acl_set_off(path);
            }
            hash_map::Entry::Vacant(_entry) => {
                error!("AclMgr sub not found, {}", path);
            }
        }
    }
}

fn acl_set_on(path: &str, is_file: bool) -> bool {
    let entry = if is_file {
        SA_PERMISSION_RWX
    } else {
        SA_PERMISSION_X
    };
    let res = acl_set_access(path, entry);
    if res != ACL_SUCC {
        error!("acl on failed: {}, {}, {}", res, path, is_file);
        return false;
    }
    true
}

fn acl_set_off(path: &str) -> bool {
    let res = acl_set_access(path, SA_PERMISSION_CLEAN);

    if res != ACL_SUCC {
        error!("acl off failed: {}, {}", res, path);
        return false;
    }
    true
}
