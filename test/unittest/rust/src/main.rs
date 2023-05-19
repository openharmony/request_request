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
 
#![allow(unused_assignments)]
mod common;
extern crate request;
use request::{enumration::*, task_manager::*};

static MAX_TASK_COUNT_API10: u32 = 300;
static MAX_TASK_COUNT_EACH_APP_API10: u32 = 10;

#[test]
fn register_notify_callback_test() {
    let task_manager = TaskManager::get_instance();
    assert!(!task_manager.has_event_callback());
    task_manager.register_callback(Box::new(common::notify_callback));
    assert!(task_manager.has_event_callback());
}

#[test]
fn create_test1() {
    let task_manager = TaskManager::get_instance();
    let mut task_id: u32 = 0;
    let uid: u64 = 10;
    let mut paths: Vec<String> = Vec::new();
    for i in 0..10 {
        let file_name = format!("create_test1{}", i);
        let code = common::construct_download_task(
            &mut task_id,
            uid,
            file_name.as_str(),
            Mode::BACKGROUND,
            Version::API10,
        );
        paths.push(file_name);
        assert_eq!(code, ErrorCode::ErrOk);
    }
    let code = common::construct_download_task(
        &mut task_id,
        uid,
        "test",
        Mode::BACKGROUND,
        Version::API10,
    );
    assert_eq!(code, ErrorCode::TaskEnqueueErr);
    assert_eq!(
        task_manager.get_total_task_count(),
        MAX_TASK_COUNT_EACH_APP_API10
    );
    assert_eq!(
        task_manager.get_api10_background_task_count(),
        MAX_TASK_COUNT_EACH_APP_API10
    );
    common::remove_files(paths);
}

#[test]
fn create_test2() {
    let task_manager = TaskManager::get_instance();
    task_manager.clear_all_task();
    assert_eq!(task_manager.get_total_task_count(), 0);
    assert_eq!(task_manager.get_api10_background_task_count(), 0);
    let mut task_id: u32 = 0;
    let mut paths: Vec<String> = Vec::new();
    for i in 0..300 {
        let file_name = format!("create_test2{}", i);
        let code = common::construct_download_task(
            &mut task_id,
            i,
            file_name.as_str(),
            Mode::BACKGROUND,
            Version::API10,
        );
        paths.push(file_name);
        assert_eq!(code, ErrorCode::ErrOk);
    }
    assert_eq!(task_manager.get_total_task_count(), MAX_TASK_COUNT_API10);
    assert_eq!(
        task_manager.get_api10_background_task_count(),
        MAX_TASK_COUNT_API10
    );
    let code = common::construct_download_task(
        &mut task_id,
        11,
        "test",
        Mode::BACKGROUND,
        Version::API10,
    );
    assert_eq!(code, ErrorCode::TaskEnqueueErr);
    assert_eq!(task_manager.get_total_task_count(), MAX_TASK_COUNT_API10);
    assert_eq!(
        task_manager.get_api10_background_task_count(),
        MAX_TASK_COUNT_API10
    );
    common::remove_files(paths);
}

#[test]
fn create_test3() {
    let task_manager = TaskManager::get_instance();
    task_manager.clear_all_task();
    let mut task_id: u32 = 0;
    let uid: u64 = 10;
    let mut paths: Vec<String> = Vec::new();
    for i in 0..10 {
        let file_name = format!("create_test3{}", i);
        let code = common::construct_download_task(
            &mut task_id,
            uid,
            file_name.as_str(),
            Mode::BACKGROUND,
            Version::API10,
        );
        paths.push(file_name);
        assert_eq!(code, ErrorCode::ErrOk);
    }
    assert_eq!(task_manager.get_total_task_count(), 10);
    assert_eq!(
        task_manager.get_api10_background_task_count(),
        MAX_TASK_COUNT_EACH_APP_API10
    );
    let mut code = common::construct_download_task(
        &mut task_id,
        uid,
        "test",
        Mode::FRONTEND,
        Version::API10,
    );
    assert_eq!(code, ErrorCode::TaskModeErr);
    assert_eq!(task_manager.get_total_task_count(), 10);
    assert_eq!(
        task_manager.get_api10_background_task_count(),
        MAX_TASK_COUNT_EACH_APP_API10
    );
    code = common::construct_download_task(
        &mut task_id,
        uid,
        "test",
        Mode::BACKGROUND,
        Version::API9,
    );
    assert_eq!(code, ErrorCode::ErrOk);
    assert_eq!(task_manager.get_total_task_count(), 11);
    assert_eq!(
        task_manager.get_api10_background_task_count(),
        MAX_TASK_COUNT_EACH_APP_API10
    );
    common::remove_files(paths);
}

#[test]
fn start_test1() {
    let task_manager = TaskManager::get_instance();
    task_manager.clear_all_task();
    let mut task_id: u32 = 0;
    let uid: u64 = 1;
    let mut code = common::construct_download_task(
        &mut task_id,
        uid,
        "test_file",
        Mode::BACKGROUND,
        Version::API10,
    );
    assert_eq!(code, ErrorCode::ErrOk);
    assert_eq!(task_manager.get_total_task_count(), 1);
    assert_eq!(task_manager.get_api10_background_task_count(), 1);
    code = task_manager.start(uid, task_id);
    assert_eq!(code, ErrorCode::ErrOk);
    code = task_manager.start(uid, task_id);
    assert_eq!(code, ErrorCode::TaskStateErr);
    code = task_manager.start(uid, task_id + 1);
    assert_eq!(code, ErrorCode::TaskNotFound);
}

#[test]
fn pause_test() {
    let task_manager = TaskManager::get_instance();
    task_manager.clear_all_task();
    let mut task_id: u32 = 0;
    let uid: u64 = 1;
    let mut code = common::construct_download_task(
        &mut task_id,
        uid,
        "test_file",
        Mode::BACKGROUND,
        Version::API10,
    );
    assert_eq!(code, ErrorCode::ErrOk);
    code = task_manager.pause(uid, task_id);
    assert_eq!(code, ErrorCode::TaskStateErr);
    code = task_manager.start(uid, task_id);
    assert_eq!(code, ErrorCode::ErrOk);
    code = task_manager.pause(uid, task_id);
    assert_eq!(code, ErrorCode::ErrOk);
}

#[test]
fn resume_test() {
    let task_manager = TaskManager::get_instance();
    task_manager.clear_all_task();
    let mut task_id: u32 = 0;
    let uid: u64 = 1;
    let mut code = common::construct_download_task(
        &mut task_id,
        uid,
        "test_file",
        Mode::BACKGROUND,
        Version::API10,
    );
    assert_eq!(code, ErrorCode::ErrOk);
    code = task_manager.resume(uid, task_id);
    assert_eq!(code, ErrorCode::TaskStateErr);
    task_manager.start(uid, task_id);
    code = task_manager.pause(uid, task_id);
    assert_eq!(code, ErrorCode::ErrOk);
    code = task_manager.resume(uid, task_id);
    assert_eq!(code, ErrorCode::ErrOk);
}

#[test]
fn stop_test() {
    let task_manager = TaskManager::get_instance();
    task_manager.clear_all_task();
    let mut task_id: u32 = 0;
    let uid: u64 = 1;
    let mut code = common::construct_download_task(
        &mut task_id,
        uid,
        "test_file",
        Mode::BACKGROUND,
        Version::API10,
    );
    assert_eq!(code, ErrorCode::ErrOk);
    code = task_manager.stop(uid, task_id);
    assert_eq!(code, ErrorCode::TaskStateErr);
    task_manager.start(uid, task_id);
    code = task_manager.stop(uid, task_id);
    assert_eq!(code, ErrorCode::ErrOk);
    assert_eq!(task_manager.get_total_task_count(), 0);
    assert_eq!(task_manager.get_api10_background_task_count(), 0);
    code = common::construct_download_task(
        &mut task_id,
        uid,
        "test_file",
        Mode::BACKGROUND,
        Version::API9,
    );
    assert_eq!(code, ErrorCode::ErrOk);
    assert_eq!(task_manager.get_total_task_count(), 1);
    assert_eq!(task_manager.get_api10_background_task_count(), 0);
    task_manager.start(uid, task_id);
    code = task_manager.stop(uid, task_id);
    assert_eq!(task_manager.get_total_task_count(), 0);
    assert_eq!(task_manager.get_api10_background_task_count(), 0);
}

#[test]
fn remove_test() {
    let task_manager = TaskManager::get_instance();
    task_manager.clear_all_task();
    let mut task_id1: u32 = 0;
    let mut task_id2: u32 = 0;
    let uid: u64 = 1;
    let mut code = common::construct_download_task(
        &mut task_id1,
        uid,
        "test_file1",
        Mode::BACKGROUND,
        Version::API10,
    );
    assert_eq!(code, ErrorCode::ErrOk);
    code = common::construct_download_task(
        &mut task_id2,
        uid,
        "test_file2",
        Mode::BACKGROUND,
        Version::API9,
    );
    assert_eq!(code, ErrorCode::ErrOk);
    assert_eq!(task_manager.get_total_task_count(), 2);
    assert_eq!(task_manager.get_api10_background_task_count(), 1);
    code = task_manager.remove(uid, task_id1);
    assert_eq!(task_manager.get_total_task_count(), 1);
    assert_eq!(task_manager.get_api10_background_task_count(), 0);
    code = task_manager.remove(uid, task_id2);
    assert_eq!(task_manager.get_total_task_count(), 0);
    assert_eq!(task_manager.get_api10_background_task_count(), 0);
}
