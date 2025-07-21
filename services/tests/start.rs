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
#![cfg(feature = "oh")]
use std::fs::File;
use std::time::Duration;

use download_server::config::{Action, ConfigBuilder, Mode};
use test_common::test_init;
const FILE_SIZE: u64 = 1042003;

// @tc.name: sdv_start_basic
// @tc.desc: Test basic download task starting functionality
// @tc.precon: NA
// @tc.step: 1. Initialize test agent and create test file
//           2. Build download configuration with background mode
//           3. Construct and start the download task
//           4. Wait for task completion and verify file size
// @tc.expect: Download completes successfully with correct file size
// @tc.type: FUNC
// @tc.require: issues#ICN31I
#[test]
fn sdv_start_basic() {
    let file_path = "sdv_network_resume.txt";

    let agent = test_init();
    let file = File::create(file_path).unwrap();
    let config = ConfigBuilder::new()
        .action(Action::Download)
        .mode(Mode::BackGround)
        .file_spec(file)
        .url("https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt")
        .redirect(true)
        .build();
    let task_id = agent.construct(config);
    agent.start(task_id);
    agent.subscribe(task_id);
    ylong_runtime::block_on(async {
        'main: loop {
            let messages = agent.pop_task_info(task_id);
            for message in messages {
                message.check_correct();
                if message.is_finished() {
                    break 'main;
                }
            }
            ylong_runtime::time::sleep(Duration::from_secs(1)).await;
        }
        let file = File::open(file_path).unwrap();
        assert_eq!(FILE_SIZE, file.metadata().unwrap().len());
    });
}
