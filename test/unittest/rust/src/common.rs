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

#![allow(unused_must_use)]
extern crate request;

use request::{enumration::*, form_item::*, task_config::*, task_manager::*};
use std::{collections::HashMap, fs::File, sync::Arc};

pub fn construct_download_task(
    task_id: &mut u32,
    uid: u64,
    file_name: &str,
    mode: Mode,
    version: Version,
) -> ErrorCode {
    let conf = TaskConfig {
        bundle: "xxx".into(),
        url: "http://110.41.6.210:9029/fota-tmp/UltraEdit_x64.xp510.com.rar".into(),
        title: "test".into(),
        description: "xxxx".into(),
        method: "get".into(),
        headers: HashMap::<String, String>::new(),
        data: "xxx".into(),
        token: "12312".into(),
        extras: HashMap::<String, String>::new(),
        version,
        form_items: vec![FormItem {
            name: "name".to_string(),
            value: "123".to_string(),
        }],
        file_specs: {
            vec![FileSpec {
                name: "file".to_string(),
                path: "test.txt".to_string(),
                file_name: "test.txt".to_string(),
                mime_type: "txt".to_string(),
            }]
        },
        common_data: CommonTaskConfig {
            action: Action::DOWNLOAD,
            mode,
            cover: true,
            network: Network::ANY,
            metered: false,
            roaming: true,
            retry: true,
            redirect: true,
            index: 10,
            begins: 0,
            ends: -1,
            gauge: false,
            precise: false,
            background: true,
        },
    };
    let files = vec![File::create(file_name).expect("create file failed")];
    TaskManager::get_instance().construct_task(Arc::new(conf), uid, task_id, files)
}

pub fn remove_files(paths: Vec<String>) {
    TaskManager::get_instance().clear_all_task();
    for path in paths.iter() {
        std::fs::remove_file(path);
    }
}
