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

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use ylong_runtime::sync::mpsc::unbounded_channel;

use crate::ability::SYSTEM_CONFIG_MANAGER;
use crate::config::{Action, ConfigBuilder, Mode, TaskConfig};
use crate::manage::network::{NetworkInfo, NetworkInner, NetworkType};
use crate::service::client::ClientManagerEntry;
use crate::task::request_task::{check_config, get_rest_time, RequestTask};
use crate::task::upload::upload;
use crate::tests::test_init;

const TEST_CONTENT: &str = "12345678910";

fn build_task(config: TaskConfig) -> Arc<RequestTask> {
    let (tx, _) = unbounded_channel();
    let client_manager = ClientManagerEntry::new(tx);
    let system_config = unsafe { SYSTEM_CONFIG_MANAGER.assume_init_ref().system_config() };
    let inner = NetworkInner::new();
    inner.notify_online(NetworkInfo {
        network_type: NetworkType::Wifi,
        is_metered: false,
        is_roaming: false,
    });

    let rest_time = get_rest_time(&config, 0);

    let (files, client) = check_config(
        &config,
        rest_time,
        #[cfg(feature = "oh")]
        system_config,
    )
    .unwrap();

    let task = Arc::new(RequestTask::new(
        config,
        files,
        client,
        client_manager,
        false,
        rest_time,
    ));
    task
}

fn test_server(test_body: Vec<Vec<String>>) -> String {
    let server = "127.0.0.1";
    let mut port = 7878;
    let listener = loop {
        match TcpListener::bind((server, port)) {
            Ok(listener) => break listener,
            Err(_) => port += 1,
        }
    };
    std::thread::spawn(move || {
        let test_body = test_body.clone();
        for (stream, test_body) in listener.incoming().zip(test_body.iter()) {
            std::thread::sleep(std::time::Duration::from_secs(2));
            let stream = stream.unwrap();
            handle_connection(stream, test_body);
        }
    });
    format!("{}:{}", server, port)
}

fn handle_connection(mut stream: TcpStream, test_body: &Vec<String>) {
    let buf_reader = BufReader::new(&mut stream);
    let mut lines = buf_reader.lines();
    let mut body = vec![];
    let mut count = 0;
    for line in lines.by_ref() {
        let line = line.unwrap();
        if line.is_empty() {
            count += 1;
            continue;
        }
        if count != 2 {
            continue;
        }
        if line.starts_with("--") {
            break;
        }
        body.push(line);
    }
    let response = if &body == test_body {
        "HTTP/1.1 200 OK\r\n\r\n"
    } else {
        "HTTP/1.1 400 Bad Request\r\n\r\n"
    };
    stream.write_all(response.as_bytes()).unwrap();
}

fn create_file(path: &str) -> File {
    File::options()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .unwrap()
}

fn config(server: String, files: Vec<File>) -> TaskConfig {
    let mut builder = ConfigBuilder::new();
    builder
        .action(Action::Upload)
        .method("POST")
        .mode(Mode::BackGround)
        .url(&format!("http://{}/", server))
        .redirect(true)
        .version(1);
    for file in files {
        builder.file_spec(file);
    }
    builder.build()
}

// @tc.name: ut_upload_basic
// @tc.desc: Test basic upload functionality
// @tc.precon: NA
// @tc.step: 1. Initialize test environment
//           2. Create test file with content
//           3. Configure upload task with POST method
//           4. Execute upload asynchronously
//           5. Verify upload result
// @tc.expect: Upload succeeds and returns Ok
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_upload_basic() {
    test_init();
    let server = test_server(vec![vec![TEST_CONTENT.to_string()]]);
    let mut file = create_file("test_files/ut_upload_basic.txt");

    file.write_all(TEST_CONTENT.as_bytes()).unwrap();

    let config = ConfigBuilder::new()
        .action(Action::Upload)
        .method("POST")
        .mode(Mode::BackGround)
        .file_spec(file)
        .url(&format!("http://{}/", server))
        .redirect(true)
        .version(1)
        .build();
    let task = build_task(config);
    ylong_runtime::block_on(async {
        upload(task.clone(), Arc::new(AtomicBool::new(false))).await;
    });
    assert!(task.running_result.lock().unwrap().unwrap().is_ok());
}

// @tc.name: ut_upload_begins
// @tc.desc: Test upload with specified begins offset
// @tc.precon: NA
// @tc.step: 1. Initialize test environment
//           2. Create test file with content
//           3. Configure upload task with begins offset
//           4. Execute upload asynchronously
//           5. Verify upload result
// @tc.expect: Upload succeeds with correct partial content and returns Ok
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_upload_begins() {
    test_init();

    let mut file = create_file("test_files/ut_upload_begins.txt");

    file.write_all(TEST_CONTENT.as_bytes()).unwrap();

    let (a, b) = TEST_CONTENT.split_at(2);
    let server = test_server(vec![vec![b.to_string()]]);

    let mut config = config(server, vec![file]);
    config.common_data.begins = a.as_bytes().len() as u64;

    let task = build_task(config);
    ylong_runtime::block_on(async {
        upload(task.clone(), Arc::new(AtomicBool::new(false))).await;
    });
    assert!(task.running_result.lock().unwrap().unwrap().is_ok());
}

// @tc.name: ut_upload_ends
// @tc.desc: Test upload with specified ends offset
// @tc.precon: NA
// @tc.step: 1. Initialize test environment
//           2. Create test file with content
//           3. Configure upload task with ends offset
//           4. Execute upload asynchronously
//           5. Verify upload result
// @tc.expect: Upload succeeds with correct partial content and returns Ok
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_upload_ends() {
    test_init();
    let mut file = create_file("test_files/ut_upload_ends.txt");

    file.write_all(TEST_CONTENT.as_bytes()).unwrap();

    let (a, _) = TEST_CONTENT.split_at(2);
    let server = test_server(vec![vec![a.to_string()]]);

    let mut config = config(server, vec![file]);
    config.common_data.ends = a.as_bytes().len() as i64 - 1;

    let task = build_task(config);
    ylong_runtime::block_on(async {
        upload(task.clone(), Arc::new(AtomicBool::new(false))).await;
    });
    assert!(task.running_result.lock().unwrap().unwrap().is_ok());
}

// @tc.name: ut_upload_range
// @tc.desc: Test upload with specified begins and ends offset range
// @tc.precon: NA
// @tc.step: 1. Initialize test environment
//           2. Create test file with content
//           3. Configure upload task with begins and ends offset
//           4. Execute upload asynchronously
//           5. Verify upload result
// @tc.expect: Upload succeeds with correct range content and returns Ok
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_upload_range() {
    test_init();
    let mut file = create_file("test_files/ut_upload_range.txt");

    file.write_all(TEST_CONTENT.as_bytes()).unwrap();

    let (a, b) = TEST_CONTENT.split_at(2);
    let (b, _) = b.split_at(3);
    let server = test_server(vec![vec![b.to_string()]]);

    let mut config = config(server, vec![file]);
    config.common_data.begins = a.as_bytes().len() as u64;
    config.common_data.ends = (a.as_bytes().len() + b.as_bytes().len()) as i64 - 1;

    let task = build_task(config);
    ylong_runtime::block_on(async {
        upload(task.clone(), Arc::new(AtomicBool::new(false))).await;
    });
    assert!(task.running_result.lock().unwrap().unwrap().is_ok());
}

// @tc.name: ut_upload_index_range
// @tc.desc: Test upload with specified file index and offset range
// @tc.precon: NA
// @tc.step: 1. Initialize test environment
//           2. Create multiple test files with content
//           3. Configure upload task with file index and offset range
//           4. Execute upload asynchronously
//           5. Verify upload result
// @tc.expect: Upload succeeds with correct file index and range content and
// returns Ok @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_upload_index_range() {
    test_init();

    let mut files = vec![];
    for _ in 0..5 {
        let mut file = create_file("test_files/ut_upload_range_index0.txt");
        file.write_all(TEST_CONTENT.as_bytes()).unwrap();
        files.push(file);
    }

    let (a, b) = TEST_CONTENT.split_at(2);
    let (b, _) = b.split_at(3);

    let index = 2;

    let mut test_body = vec![vec![TEST_CONTENT.to_string()]; 5];
    test_body[index] = vec![b.to_string()];

    let server = test_server(test_body);

    let mut config = config(server, files);
    config.common_data.begins = a.as_bytes().len() as u64;
    config.common_data.ends = (a.as_bytes().len() + b.as_bytes().len()) as i64 - 1;
    config.common_data.index = index as u32;

    let task = build_task(config);
    ylong_runtime::block_on(async {
        upload(task.clone(), Arc::new(AtomicBool::new(false))).await;
    });
    assert!(task.running_result.lock().unwrap().unwrap().is_ok());
}