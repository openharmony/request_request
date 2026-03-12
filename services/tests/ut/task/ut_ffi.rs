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

// @tc.name: ut_c_structs
// @tc.desc: Test CMinSpeed and CTimeout struct layout and values
// @tc.precon: NA
// @tc.step: 1. Create CMinSpeed with specific values
//           2. Create CTimeout with specific values
// @tc.expect: Both structs have correct field values
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_c_structs() {
    let min_speed = CMinSpeed {
        speed: 1024,
        duration: 5000,
    };
    assert_eq!(min_speed.speed, 1024);
    assert_eq!(min_speed.duration, 5000);

    let timeout = CTimeout {
        connection_timeout: 30000,
        total_timeout: 300000,
    };
    assert_eq!(timeout.connection_timeout, 30000);
    assert_eq!(timeout.total_timeout, 300000);
}

// @tc.name: ut_progress_conversion
// @tc.desc: Test Progress to_c_struct and from_c_struct conversion
// @tc.precon: NA
// @tc.step: 1. Create Progress and convert to CProgress
//           2. Create CProgress and convert to Progress
//           3. Test with empty strings (edge case)
// @tc.expect: Conversions work correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_progress_conversion() {
    let progress = Progress {
        common_data: CommonProgress {
            state: 32,
            index: 0,
            processed: 1024,
            total_processed: 1024,
            size: 2048,
            total_size: 2048,
            extras: Default::default(),
        },
        sizes: vec![2048],
        processed: vec![1024],
        extras: Default::default(),
    };

    let c_progress = progress.to_c_struct("2048", "1024", "{}");
    assert_eq!(c_progress.sizes.to_string(), "2048");
    assert_eq!(c_progress.processed.to_string(), "1024");
    assert_eq!(c_progress.extras.to_string(), "{}");

    let c_progress = CProgress {
        common_data: CommonProgress {
            state: 32,
            index: 0,
            processed: 0,
            total_processed: 0,
            size: 0,
            total_size: 0,
            extras: Default::default(),
        },
        sizes: CStringWrapper::from("2048,4096"),
        processed: CStringWrapper::from("1024,2048"),
        extras: CStringWrapper::from("{}"),
    };

    let progress = Progress::from_c_struct(&c_progress);
    assert_eq!(progress.sizes, vec![2048, 4096]);
    assert_eq!(progress.processed, vec![1024, 2048]);

    let c_progress_empty = CProgress {
        common_data: CommonProgress {
            state: 0,
            index: 0,
            processed: 0,
            total_processed: 0,
            size: 0,
            total_size: 0,
            extras: Default::default(),
        },
        sizes: CStringWrapper::from(""),
        processed: CStringWrapper::from(""),
        extras: CStringWrapper::from(""),
    };

    let progress_empty = Progress::from_c_struct(&c_progress_empty);
    assert_eq!(progress_empty.sizes, vec![0]);
    assert_eq!(progress_empty.processed, vec![0]);
}

// @tc.name: ut_common_c_task_config
// @tc.desc: Test CommonCTaskConfig struct layout
// @tc.precon: NA
// @tc.step: 1. Create CommonCTaskConfig with values
//           2. Verify all fields
// @tc.expect: CommonCTaskConfig has correct values
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_common_c_task_config() {
    let common_config = CommonCTaskConfig {
        task_id: 1,
        uid: 200000,
        token_id: 1001,
        action: Action::Download.repr,
        mode: Mode::BackGround.repr,
        cover: false,
        network: NetworkConfig::Any as u8,
        metered: false,
        roaming: false,
        retry: true,
        redirect: true,
        index: 0,
        begins: 0,
        ends: -1,
        gauge: false,
        precise: false,
        priority: 0,
        background: false,
        multipart: false,
        min_speed: CMinSpeed {
            speed: 0,
            duration: 0,
        },
        timeout: CTimeout {
            connection_timeout: 60000,
            total_timeout: 0,
        },
    };

    assert_eq!(common_config.task_id, 1);
    assert_eq!(common_config.uid, 200000);
    assert_eq!(common_config.action, Action::Download.repr);
    assert_eq!(common_config.mode, Mode::BackGround.repr);
    assert!(common_config.retry);
    assert!(common_config.redirect);
}

// @tc.name: ut_task_config_conversion
// @tc.desc: Test TaskConfig to_c_struct and from_c_struct conversion
// @tc.precon: NA
// @tc.step: 1. Test action conversion
//           2. Test mode conversion
// @tc.expect: Action and Mode are correctly converted
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_config_conversion() {
    let mut config = TaskConfig::default();
    config.common_data.action = Action::Download;
    let set = config.build_config_set();
    let c_config = config.to_c_struct(1, 200000, &set);
    assert_eq!(c_config.common_data.action, Action::Download.repr);

    let mut config = TaskConfig::default();
    config.common_data.action = Action::Upload;
    let set = config.build_config_set();
    let c_config = config.to_c_struct(1, 200000, &set);
    assert_eq!(c_config.common_data.action, Action::Upload.repr);

    let mut config = TaskConfig::default();
    config.common_data.mode = Mode::BackGround;
    let set = config.build_config_set();
    let c_config = config.to_c_struct(1, 200000, &set);
    assert_eq!(c_config.common_data.mode, Mode::BackGround.repr);

    let mut config = TaskConfig::default();
    config.common_data.mode = Mode::FrontEnd;
    let set = config.build_config_set();
    let c_config = config.to_c_struct(1, 200000, &set);
    assert_eq!(c_config.common_data.mode, Mode::FrontEnd.repr);

    let c_config = CTaskConfig {
        bundle: CStringWrapper::from("test.bundle"),
        bundle_type: 0,
        atomic_account: CStringWrapper::from(""),
        url: CStringWrapper::from("http://example.com"),
        title: CStringWrapper::from("test"),
        description: CStringWrapper::from(""),
        method: CStringWrapper::from("GET"),
        headers: CStringWrapper::from("{}"),
        data: CStringWrapper::from(""),
        token: CStringWrapper::from(""),
        proxy: CStringWrapper::from(""),
        certificate_pins: CStringWrapper::from(""),
        extras: CStringWrapper::from("{}"),
        version: Version::API10 as u8,
        form_items_ptr: std::ptr::null(),
        form_items_len: 0,
        file_specs_ptr: std::ptr::null(),
        file_specs_len: 0,
        body_file_names_ptr: std::ptr::null(),
        body_file_names_len: 0,
        certs_path_ptr: std::ptr::null(),
        certs_path_len: 0,
        common_data: CommonCTaskConfig {
            task_id: 1,
            uid: 200000,
            token_id: 1001,
            action: Action::Download.repr,
            mode: Mode::BackGround.repr,
            cover: false,
            network: NetworkConfig::Any as u8,
            metered: false,
            roaming: false,
            retry: false,
            redirect: true,
            index: 0,
            begins: 0,
            ends: -1,
            gauge: false,
            precise: false,
            priority: 0,
            background: false,
            multipart: false,
            min_speed: CMinSpeed {
                speed: 0,
                duration: 0,
            },
            timeout: CTimeout {
                connection_timeout: 0,
                total_timeout: 0,
            },
        },
    };

    let config = TaskConfig::from_c_struct(&c_config);
    assert_eq!(config.common_data.action, Action::Download);
}

// @tc.name: ut_update_info_conversion
// @tc.desc: Test UpdateInfo to_c_struct conversion
// @tc.precon: NA
// @tc.step: 1. Create UpdateInfo with values
//           2. Convert to CUpdateInfo
// @tc.expect: CUpdateInfo has correct values
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_update_info_conversion() {
    let update_info = UpdateInfo {
        mtime: 1234567890,
        reason: Reason::Default.repr,
        tries: 3,
        mime_type: "application/json".to_string(),
        progress: Progress {
            common_data: CommonProgress {
                state: 32,
                index: 0,
                processed: 0,
                total_processed: 0,
                size: 0,
                total_size: 0,
                extras: Default::default(),
            },
            sizes: vec![],
            processed: vec![],
            extras: Default::default(),
        },
    };

    let c_update = update_info.to_c_struct("", "", "{}");

    assert_eq!(c_update.mtime, 1234567890);
    assert_eq!(c_update.reason, Reason::Default.repr);
    assert_eq!(c_update.tries, 3);
    assert_eq!(c_update.mime_type.to_string(), "application/json");
}

// @tc.name: ut_enum_conversions
// @tc.desc: Test Version and NetworkConfig From<u8> conversion
// @tc.precon: NA
// @tc.step: 1. Convert u8 values to Version
//           2. Convert u8 values to NetworkConfig
// @tc.expect: Enums are correctly converted
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_enum_conversions() {
    assert_eq!(Version::from(1), Version::API9);
    assert_eq!(Version::from(2), Version::API10);
    assert_eq!(Version::from(0), Version::API9);
    assert_eq!(Version::from(3), Version::API9);

    assert_eq!(NetworkConfig::from(0), NetworkConfig::Any);
    assert_eq!(NetworkConfig::from(1), NetworkConfig::Wifi);
    assert_eq!(NetworkConfig::from(2), NetworkConfig::Cellular);
    assert_eq!(NetworkConfig::from(3), NetworkConfig::Wifi);
}
