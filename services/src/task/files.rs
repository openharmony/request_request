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

use std::fs::{File, OpenOptions};
use std::io;
use std::os::fd::FromRawFd;
use std::sync::{Arc, Mutex};

use crate::task::bundle::get_name_and_index;
use crate::task::config::{Action, TaskConfig};
use crate::task::ATOMIC_SERVICE;

pub(crate) struct AttachedFiles {
    pub(crate) files: Files,
    pub(crate) sizes: Vec<i64>,
    pub(crate) body_files: Files,
}

impl AttachedFiles {
    pub(crate) fn open(config: &TaskConfig) -> io::Result<AttachedFiles> {
        let tid = config.common_data.task_id;
        let (files, sizes) = cvt_res_error!(
            open_task_files(config),
            "open task files failed - task_id: {}",
            tid
        );

        let body_files = cvt_res_error!(
            open_body_files(config),
            "open body files failed - task_id: {}",
            tid
        );

        Ok(Self {
            files,
            sizes,
            body_files,
        })
    }
}

fn open_task_files(config: &TaskConfig) -> io::Result<(Files, Vec<i64>)> {
    let tid = config.common_data.task_id;
    let uid = config.common_data.uid;
    let bundle_name = convert_bundle_name(config);

    let mut files = Vec::new();
    let mut sizes = Vec::new();

    for (idx, fs) in config.file_specs.iter().enumerate() {
        match config.common_data.action {
            Action::Upload => {
                let file = if fs.is_user_file {
                    match fs.fd {
                        Some(fd) => unsafe { File::from_raw_fd(fd) },
                        None => {
                            error!("None user file failed - task_id: {}, idx: {}", tid, idx);
                            sys_event!(
                                ExecFault,
                                DfxCode::SA_ERROR_01,
                                &format!("None user file failed - task_id: {}, idx: {}", tid, idx)
                            );
                            return Err(io::Error::new(io::ErrorKind::Other, "none user file"));
                        }
                    }
                } else {
                    cvt_res_error!(
                        open_file_readonly(uid, &bundle_name, &fs.path),
                        "Open file RO failed - task_id: {}, idx: {}",
                        tid,
                        idx
                    )
                };
                let size = cvt_res_error!(
                    file.metadata().map(|data| data.len()),
                    "Cannot get upload file's size - task_id: {}, idx: {}",
                    tid,
                    idx
                );
                files.push(Arc::new(Mutex::new(file)));
                debug!(
                    "Get file size succeed - task_id: {}, idx: {}, size: {}",
                    tid, idx, size
                );
                sizes.push(size as i64);
            }
            Action::Download => {
                let file = if fs.is_user_file {
                    match fs.fd {
                        Some(fd) => unsafe { File::from_raw_fd(fd) },
                        None => {
                            error!("None user file failed - task_id: {}, idx: {}", tid, idx);
                            sys_event!(
                                ExecFault,
                                DfxCode::SA_ERROR_01,
                                &format!("None user file failed - task_id: {}, idx: {}", tid, idx)
                            );
                            return Err(io::Error::new(io::ErrorKind::Other, "none user file"));
                        }
                    }
                } else {
                    cvt_res_error!(
                        open_file_readwrite(uid, &bundle_name, &fs.path),
                        "Open file RW failed - task_id: {}, idx: {}",
                        tid,
                        idx
                    )
                };
                files.push(Arc::new(Mutex::new(file)));
                sizes.push(-1)
            }
            _ => unreachable!("Action::Any in open_task_files should never reach"),
        }
    }
    Ok((Files::new(files), sizes))
}

fn open_body_files(config: &TaskConfig) -> io::Result<Files> {
    let tid = config.common_data.task_id;
    let uid = config.common_data.uid;
    let bundle_name = convert_bundle_name(config);

    let mut body_files = Vec::new();
    for (idx, path) in config.body_file_paths.iter().enumerate() {
        let file = cvt_res_error!(
            open_file_readwrite(uid, &bundle_name, path),
            "Open body_file failed - task_id: {}, idx: {}",
            tid,
            idx
        );
        body_files.push(Arc::new(Mutex::new(file)))
    }
    Ok(Files::new(body_files))
}

fn open_file_readwrite(uid: u64, bundle_name: &str, path: &str) -> io::Result<File> {
    Ok(cvt_res_error!(
        OpenOptions::new()
            .read(true)
            .append(true)
            .open(convert_path(uid, bundle_name, path)),
        "open_file_readwrite failed"
    ))
}

fn open_file_readonly(uid: u64, bundle_name: &str, path: &str) -> io::Result<File> {
    Ok(cvt_res_error!(
        OpenOptions::new()
            .read(true)
            .open(convert_path(uid, bundle_name, path)),
        "open_file_readonly failed"
    ))
}

pub(crate) fn convert_path(uid: u64, bundle_name: &str, path: &str) -> String {
    let uuid = uid / 200000;
    let base_replace = format!("{}/base/{}", uuid, bundle_name);
    let real_path = path
        .replacen("storage", "app", 1)
        .replacen("base", &base_replace, 1);
    real_path
}

pub(crate) fn convert_bundle_name(config: &TaskConfig) -> String {
    let is_account = config.bundle_type == ATOMIC_SERVICE;
    let bundle_name = config.bundle.as_str();
    if is_account {
        let atomic_account = config.atomic_account.as_str();
        format!("+auid-{}+{}", atomic_account, bundle_name)
    } else {
        let uid = config.common_data.uid;
        check_app_clone_bundle_name(uid, bundle_name)
    }
}

fn check_app_clone_bundle_name(uid: u64, bundle_name: &str) -> String {
    let mut ret_name = bundle_name.to_string();
    if let Some((index, name)) = get_name_and_index(uid as i32) {
        if bundle_name != name {
            info!("bundle name not matching. {:?}, {:?}", bundle_name, name);
        }
        if index > 0 {
            ret_name = format!("+clone-{}+{}", index, bundle_name);
        }
    }
    ret_name
}

pub(crate) struct Files(Vec<Arc<Mutex<File>>>);

impl Files {
    fn new(files: Vec<Arc<Mutex<File>>>) -> Self {
        Self(files)
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn get(&self, index: usize) -> Option<Arc<Mutex<File>>> {
        self.0.get(index).cloned()
    }
}
