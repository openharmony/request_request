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

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

use request_utils::storage;

use crate::file::FileManager;

// todo
const SA_PERMISSION_RWX: &str = "g:3815:rwx";
const SA_PERMISSION_X: &str = "g:3815:x";
const SA_PERMISSION_CLEAN: &str = "g:3815:---";


pub struct PermissionManager {
    paths: Mutex<HashMap<String, i32>>,
    granter: Box<dyn Granter>,
}

impl PermissionManager {
    pub(crate) fn new() -> Self {
        Self {
            paths: Mutex::new(HashMap::new()),
            granter: Box::new(AclGranter {}),
        }
    }

    pub(crate) fn set_granter(&mut self, granter: Box<dyn Granter>) {
        self.granter = granter;
    }

    pub(crate) fn grant(&self, path: &PathBuf) -> Result<PermissionToken, i32> {
        let mut paths = self.paths.lock().unwrap();
        let mut path_clone = path.clone();
        let mut completed_path: Vec<String> = vec![];

        // The permission for the entire path as a file will be set after the loop.
        // Redundant permission setting operations are performed here to ensure that
        // some permissions are not lost in concurrent scenarios.
        while path_clone.pop() && path_clone.to_string_lossy().to_string().len() >= 10 {
            debug!("Current path: {:?}", path_clone);
            let temp_path = path_clone.to_string_lossy().to_string();

            if let Err(e) = self.granter.grant(&temp_path, SA_PERMISSION_X) {
                // for path in &completed_path {
                //     if let Some(count) = paths.get_mut(path) {
                //         *count -= 1;
                //         if *count == 0 {
                //             info!("drop, path: {}", path);
                //             self.granter.grant(path, SA_PERMISSION_CLEAN);
                //             paths.remove(path);
                //         }
                //     }
                // }
                // todo
                debug!("grant path: {}, error: {}", temp_path, e);
                // return Err(13400001);
            }
            match paths.entry(temp_path.clone()) {
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() += 1;
                }
                Entry::Vacant(entry) => {
                    entry.insert(1);
                }
            }
            completed_path.push(temp_path);
        }

        debug!("Setting ACL access for path: {:?}", path);
        if let Err(e) =
            self.granter.grant(&path.to_string_lossy().to_string(), SA_PERMISSION_RWX)
        {
            error!("grant file: {}, error: {}", path.to_string_lossy().to_string(), e);
            return Err(13400001);
        }
        match paths.entry(path.to_string_lossy().to_string()) {
            Entry::Occupied(mut entry) => {
                *entry.get_mut() += 1;
            }
            Entry::Vacant(entry) => {
                entry.insert(1);
            }
        }
        Ok(PermissionToken::new(path.clone()))
    }

    pub(crate) fn revoke(&self, path: &PathBuf) {
        let mut paths = self.paths.lock().unwrap();
        let mut path_clone = path.clone();
        while true {
            let temp_path = path_clone.to_string_lossy().to_string();
            if let Some(count) = paths.get_mut(&temp_path) {
                *count -= 1;
                if *count == 0 {
                    info!("revoke, path: {}", temp_path);
                    self.granter.grant(&temp_path, SA_PERMISSION_CLEAN);
                    paths.remove(&temp_path);
                }
            }
            if !path_clone.pop() {
                break;
            }
        }
    }
}

pub(crate) trait Granter: Send + Sync {
    fn grant(&self, path: &str, permission: &str) -> Result<(), i32>;
}

struct AclGranter {}

impl Granter for AclGranter {
    fn grant(&self, path: &str, permission: &str) -> Result<(), i32> {
        storage::acl_set_access(path, permission)
    }
}

pub struct PermissionToken {
    path: PathBuf,
}

impl PermissionToken {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Drop for PermissionToken {
    fn drop(&mut self) {
        FileManager::get_instance().permission_manager.revoke(&self.path);
    }
}