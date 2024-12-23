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

#![allow(clippy::bool_assert_comparison)]

const DOWNLOAD_FILE: &str = "request_agent_download_file\0";
const DOWNLOAD_SUCCESS: &str = "request_agent_download_success\0";
const DOWNLOAD_FAIL: &str = "request_agent_download_fail\0";
const UPLOAD_FILE: &str = "request_agent_upload_file\0";
const UPLOAD_SUCCESS: &str = "request_agent_upload_success\0";
const UPLOAD_FAIL: &str = "request_agent_upload_fail\0";

use super::ffi::{GetSystemResourceString, NotifyContent, ProgressCircle};
use super::notify_flow::{GroupProgress, ProgressNotify};
use crate::config::Action;

fn progress_percentage(current: u64, total: u64) -> String {
    format!(
        "{}.{:02}%",
        current * 100 / total,
        current * 100 % total * 100 / total
    )
}

fn progress_size(current: u64) -> String {
    if current < 1024 {
        format!("{}B", current)
    } else if current < 1024 * 1024 {
        format!("{:.2}KB", current as f64 / 1024.0)
    } else if current < 1024 * 1024 * 1024 {
        format!("{:.2}MB", current as f64 / 1024.0 / 1024.0)
    } else {
        format!("{:.2}GB", current as f64 / 1024.0 / 1024.0 / 1024.0)
    }
}

impl NotifyContent {
    pub(crate) fn default_task_eventual_notify(
        action: Action,
        task_id: u32,
        uid: u32,
        file_name: String,
        is_successful: bool,
    ) -> Self {
        let title = match action {
            Action::Download => {
                if is_successful {
                    GetSystemResourceString(DOWNLOAD_SUCCESS)
                } else {
                    GetSystemResourceString(DOWNLOAD_FAIL)
                }
            }
            Action::Upload => {
                if is_successful {
                    GetSystemResourceString(UPLOAD_SUCCESS)
                } else {
                    GetSystemResourceString(UPLOAD_FAIL)
                }
            }
            _ => unreachable!(),
        };
        Self {
            title,
            text: file_name,
            request_id: task_id,
            uid,
            live_view: false,
            progress_circle: ProgressCircle::close(),
            x_mark: false,
        }
    }

    pub(crate) fn default_task_progress_notify(info: &ProgressNotify) -> Self {
        let title = match info.action {
            Action::Download => {
                let title = GetSystemResourceString(DOWNLOAD_FILE);
                match info.total {
                    Some(total) => title.replace("%s", &progress_percentage(info.processed, total)),
                    None => title.replace("%s", &progress_size(info.processed)),
                }
            }
            Action::Upload => {
                let title = GetSystemResourceString(UPLOAD_FILE);
                if let Some((current_count, total_count)) = info.multi_upload {
                    title.replace("%s", &format!("{}/{}", current_count, total_count))
                } else {
                    match info.total {
                        Some(total) => {
                            title.replace("%s", &progress_percentage(info.processed, total))
                        }
                        None => title.replace("%s", &progress_size(info.processed)),
                    }
                }
            }
            _ => unreachable!(),
        };
        let progress_circle = match info.total {
            Some(total) => ProgressCircle::open(info.processed, total),
            None => ProgressCircle::close(),
        };
        Self {
            title,
            text: info.file_name.clone(),
            request_id: info.task_id,
            uid: info.uid as u32,
            live_view: true,
            progress_circle,
            x_mark: true,
        }
    }

    pub(crate) fn default_group_eventual_notify(
        action: Action,
        group_id: u32,
        uid: u32,
        current_size: u64,
        successful_count: i32,
        failed_count: i32,
    ) -> Self {
        let title = match action {
            Action::Download => format!("下载完成 {}", progress_size(current_size)),
            Action::Upload => format!("上传完成 {}", progress_size(current_size)),
            _ => unreachable!(),
        };
        let text = format!("成功 {} 个, 失败 {} 个", successful_count, failed_count);
        Self {
            title,
            text,
            request_id: group_id,
            uid,
            live_view: false,
            progress_circle: ProgressCircle::close(),
            x_mark: false,
        }
    }

    pub(crate) fn default_group_progress_notify(
        action: Action,
        group_id: u32,
        uid: u32,
        group_progress: &GroupProgress,
    ) -> Self {
        let title = match action {
            Action::Download => {
                let title = GetSystemResourceString(DOWNLOAD_FILE);
                title.replace("%s", &progress_size(group_progress.processed()))
            }
            Action::Upload => {
                let title = GetSystemResourceString(UPLOAD_FILE);
                title.replace("%s", &progress_size(group_progress.processed()))
            }
            _ => unreachable!(),
        };
        let (successful, failed) = (group_progress.successful(), group_progress.failed());
        let text = format!("成功 {} 个, 失败 {} 个", successful, failed);

        let progress_circle =
            ProgressCircle::open((successful + failed) as u64, group_progress.total() as u64);
        Self {
            title,
            text,
            request_id: group_id,
            uid,
            live_view: true,
            progress_circle,
            x_mark: false,
        }
    }

    pub(crate) fn customized_notify(
        request_id: u32,
        uid: u32,
        title: String,
        text: String,
        live_view: bool,
    ) -> Self {
        Self {
            title,
            text,
            request_id,
            uid,
            live_view,
            progress_circle: ProgressCircle::close(),
            x_mark: false,
        }
    }
}

impl ProgressCircle {
    pub(crate) fn close() -> Self {
        Self {
            open: false,
            current: 0,
            total: 0,
        }
    }
    pub(crate) fn open(mut current: u64, mut total: u64) -> Self {
        while total > i32::MAX as u64 {
            total >>= 1;
            current >>= 1;
        }
        Self {
            open: true,
            current,
            total,
        }
    }
}
