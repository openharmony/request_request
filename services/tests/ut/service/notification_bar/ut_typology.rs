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

use super::*;
use crate::info::State;
use crate::task::config::Version;
const EXAMPLE_FILE: &str = "2024_12_10_15_56";
const TASK_ID: u32 = 2024;
const UID: u32 = 12;
const GROUP_ID: u32 = 20;

// @tc.name: ut_notify_typology_default_task_eventual
// @tc.desc: Test default task completion notification formatting
// @tc.precon: NA
// @tc.step: 1. Create task eventual notifications for download/upload
// success/failure scenarios
//           2. Verify title, text, and notification properties match expected
//              defaults
// @tc.expect: Notification title reflects action result, text shows filename,
// and properties are correctly set @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_notify_typology_default_task_eventual() {
    let content = NotifyContent::task_eventual_notify(
        None,
        Action::Download,
        TASK_ID,
        UID,
        EXAMPLE_FILE.to_string(),
        false,
    );
    assert_eq!(content.title, "下载失败");
    assert_eq!(content.text, EXAMPLE_FILE);
    assert_eq!(content.live_view, false);
    assert_eq!(content.progress_circle.open, false);
    assert_eq!(content.x_mark, false);
    assert_eq!(content.request_id, TASK_ID);
    assert_eq!(content.uid, UID);

    let content = NotifyContent::task_eventual_notify(
        None,
        Action::Download,
        0,
        0,
        EXAMPLE_FILE.to_string(),
        true,
    );
    assert_eq!(content.title, "下载成功");
    assert_eq!(content.text, EXAMPLE_FILE);

    let content = NotifyContent::task_eventual_notify(
        None,
        Action::Upload,
        0,
        0,
        EXAMPLE_FILE.to_string(),
        false,
    );
    assert_eq!(content.title, "上传失败");
    assert_eq!(content.text, EXAMPLE_FILE);

    let content = NotifyContent::task_eventual_notify(
        None,
        Action::Upload,
        0,
        0,
        EXAMPLE_FILE.to_string(),
        true,
    );

    assert_eq!(content.title, "上传成功");
    assert_eq!(content.text, EXAMPLE_FILE);
}

// @tc.name: ut_notify_typology_default_progress
// @tc.desc: Test default task progress notification formatting
// @tc.precon: NA
// @tc.step: 1. Create ProgressNotify instances with various processed/total
// values and multi-upload scenarios
//           2. Generate task progress notifications for download and upload
//              actions
//           3. Verify title, text, and progress circle properties match
//              expected formatting
// @tc.expect: Notification title shows correct percentage/size, progress circle
// is properly configured @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_notify_typology_default_progress() {
    let mut progress_info = ProgressNotify {
        action: Action::Download,
        task_id: TASK_ID,
        uid: UID as u64,
        file_name: EXAMPLE_FILE.to_string(),
        processed: 1,
        total: Some(10),
        multi_upload: None,
        version: Version::API10,
    };
    let content = NotifyContent::task_progress_notify(None, &progress_info);
    assert_eq!(content.title, "下载文件 10.00%");
    assert_eq!(content.text, EXAMPLE_FILE);
    assert_eq!(content.live_view, true);
    assert_eq!(content.progress_circle.open, true);
    assert_eq!(content.x_mark, true);
    assert_eq!(content.request_id, TASK_ID);

    progress_info.processed = 1001;
    progress_info.total = Some(10000);
    let content = NotifyContent::task_progress_notify(None, &progress_info);
    assert_eq!(content.title, "下载文件 10.01%");

    progress_info.processed = 1010;
    let content = NotifyContent::task_progress_notify(None, &progress_info);
    assert_eq!(content.title, "下载文件 10.10%");

    progress_info.processed = 1;
    progress_info.total = None;
    let content = NotifyContent::task_progress_notify(None, &progress_info);
    assert_eq!(content.title, "下载文件 1B");

    progress_info.processed = 1024;

    let content = NotifyContent::task_progress_notify(None, &progress_info);
    assert_eq!(content.title, "下载文件 1.00KB");

    progress_info.processed = 1024 * 1024;
    let content = NotifyContent::task_progress_notify(None, &progress_info);
    assert_eq!(content.title, "下载文件 1.00MB");

    progress_info.processed = 1024 * 1024 * 1024;
    let content = NotifyContent::task_progress_notify(None, &progress_info);
    assert_eq!(content.title, "下载文件 1.00GB");

    progress_info.action = Action::Upload;
    progress_info.processed = 1;
    progress_info.total = Some(10);
    let content = NotifyContent::task_progress_notify(None, &progress_info);
    assert_eq!(content.title, "上传文件 10.00%");

    progress_info.multi_upload = Some((1, 10));
    let content = NotifyContent::task_progress_notify(None, &progress_info);
    assert_eq!(content.title, "上传文件 1/10");
}

// @tc.name: ut_notify_typology_default_group_progress
// @tc.desc: Test default group progress notification formatting
// @tc.precon: NA
// @tc.step: 1. Create GroupProgress instance and update task states/progress
//           2. Generate group progress notifications for download action
//           3. Verify title shows correct size and text displays task counts
//           4. Update task states and verify notification text updates
//              accordingly
// @tc.expect: Notification title shows total processed size, text shows correct
// successful/failed task counts @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_notify_typology_default_group_progress() {
    let mut group_info = GroupProgress::new();
    group_info.update_task_state(1, State::Completed);
    group_info.update_task_progress(1, 100);
    let content =
        NotifyContent::group_progress_notify(None, Action::Download, GROUP_ID, UID, &group_info);

    assert_eq!(content.title, "下载文件 100B");

    let text_task_count = GetSystemResourceString(TASK_COUNT);
    let text_count = if text_task_count.contains("%d") {
        text_task_count
            .replacen("%d", "1", 1)
            .replacen("%d", "0", 1)
    } else {
        text_task_count.replace("%1$d", "1").replace("%2$d", "0")
    };
    assert_eq!(content.text, text_count);

    for i in 1..4 {
        group_info.update_task_state(i, State::Failed);
    }
    for i in 2..5 {
        group_info.update_task_state(i, State::Completed);
    }
    let content =
        NotifyContent::group_progress_notify(None, Action::Download, GROUP_ID, UID, &group_info);

    assert_eq!(content.title, "下载文件 100B");
    let text_task_count = GetSystemResourceString(TASK_COUNT);
    let text_count = if text_task_count.contains("%d") {
        text_task_count
            .replacen("%d", "3", 1)
            .replacen("%d", "1", 1)
    } else {
        text_task_count.replace("%1$d", "3").replace("%2$d", "1")
    };
    assert_eq!(content.text, text_count);
}
