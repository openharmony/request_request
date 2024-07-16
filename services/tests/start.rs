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

use std::fs::File;
use std::time::Duration;

use download_server::config::{Action, ConfigBuilder, Mode};
use download_server::FileSpec;
use test_common::test_init;

#[test]
fn sdv_start_basic() {
    let agent = test_init();
    let file = File::create("sdv_network_resume.txt").unwrap();
    let config = ConfigBuilder::new()
        .action(Action::Download)
        .mode(Mode::BackGround)
        .file_spec(|v| v.push(FileSpec::user_file(&file)))
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
        assert_eq!(1042003, file.metadata().unwrap().len());
    })
}
