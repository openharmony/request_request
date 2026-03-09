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

use request_client::client::native_task::{NativeTask, NativeTaskManager};
use request_core::config::TaskConfig;

// @tc.name: ut_native_task_manager_default
// @tc.desc: Test NativeTaskManager default creation
// @tc.precon: NA
// @tc.step: 1. Create NativeTaskManager using default
//           2. Verify initial state is empty
// @tc.expect: NativeTaskManager is created with empty maps
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_native_task_manager_default() {
    let manager = NativeTaskManager::default();
    let inner = manager.inner.lock().unwrap();
    assert!(inner.tasks.is_empty());
    assert!(inner.tids.is_empty());
}

// @tc.name: ut_native_task_manager_insert
// @tc.desc: Test NativeTaskManager insert operation
// @tc.precon: NA
// @tc.step: 1. Create NativeTaskManager
//           2. Create NativeTask with config
//           3. Insert task with sequence number
//           4. Verify task is stored
// @tc.expect: Task is inserted and can be retrieved
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_native_task_manager_insert() {
    let manager = NativeTaskManager::default();
    let config = TaskConfig::default();
    let task = NativeTask {
        config,
        token: vec![],
    };
    
    let seq: u64 = 1;
    manager.insert(seq, task);
    
    let inner = manager.inner.lock().unwrap();
    assert_eq!(inner.tasks.len(), 1);
    assert!(inner.tasks.contains_key(&seq));
}

// @tc.name: ut_native_task_manager_remove
// @tc.desc: Test NativeTaskManager remove operation
// @tc.precon: NA
// @tc.step: 1. Create NativeTaskManager and insert task
//           2. Remove task by sequence number
//           3. Verify task is removed
// @tc.expect: Task is removed from manager
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_native_task_manager_remove() {
    let manager = NativeTaskManager::default();
    let config = TaskConfig::default();
    let task = NativeTask {
        config,
        token: vec![],
    };
    
    let seq: u64 = 1;
    manager.insert(seq, task);
    
    manager.remove(&seq);
    
    let inner = manager.inner.lock().unwrap();
    assert!(inner.tasks.is_empty());
}

// @tc.name: ut_native_task_manager_bind
// @tc.desc: Test NativeTaskManager bind operation
// @tc.precon: NA
// @tc.step: 1. Create NativeTaskManager and insert task
//           2. Bind task_id to sequence number
//           3. Verify binding is stored
// @tc.expect: Task ID is bound to sequence number
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_native_task_manager_bind() {
    let manager = NativeTaskManager::default();
    let config = TaskConfig::default();
    let task = NativeTask {
        config,
        token: vec![],
    };
    
    let seq: u64 = 1;
    let task_id: i64 = 100;
    
    manager.insert(seq, task);
    manager.bind(task_id, seq);
    
    let inner = manager.inner.lock().unwrap();
    assert_eq!(inner.tids.get(&task_id), Some(&seq));
}

// @tc.name: ut_native_task_manager_remove_task
// @tc.desc: Test NativeTaskManager remove_task operation
// @tc.precon: NA
// @tc.step: 1. Create NativeTaskManager, insert and bind task
//           2. Remove task by task_id
//           3. Verify both task and binding are removed
// @tc.expect: Task and binding are removed
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_native_task_manager_remove_task() {
    let manager = NativeTaskManager::default();
    let config = TaskConfig::default();
    let task = NativeTask {
        config,
        token: vec![],
    };
    
    let seq: u64 = 1;
    let task_id: i64 = 100;
    
    manager.insert(seq, task);
    manager.bind(task_id, seq);
    
    manager.remove_task(&task_id);
    
    let inner = manager.inner.lock().unwrap();
    assert!(inner.tasks.is_empty());
    assert!(inner.tids.is_empty());
}

// @tc.name: ut_native_task_manager_get_by_seq
// @tc.desc: Test NativeTaskManager get_by_seq operation
// @tc.precon: NA
// @tc.step: 1. Create NativeTaskManager and insert task
//           2. Get task by sequence number
//           3. Verify correct task is returned
// @tc.expect: Task is retrieved by sequence number
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_native_task_manager_get_by_seq() {
    let manager = NativeTaskManager::default();
    let config = TaskConfig::default();
    let task = NativeTask {
        config,
        token: vec![],
    };
    
    let seq: u64 = 1;
    manager.insert(seq, task);
    
    let result = manager.get_by_seq(&seq);
    assert!(result.is_some());
    
    let result = manager.get_by_seq(&999);
    assert!(result.is_none());
}

// @tc.name: ut_native_task_manager_get_by_id
// @tc.desc: Test NativeTaskManager get_by_id operation
// @tc.precon: NA
// @tc.step: 1. Create NativeTaskManager, insert and bind task
//           2. Get task by task_id
//           3. Verify correct task is returned
// @tc.expect: Task is retrieved by task_id
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_native_task_manager_get_by_id() {
    let manager = NativeTaskManager::default();
    let config = TaskConfig::default();
    let task = NativeTask {
        config,
        token: vec![],
    };
    
    let seq: u64 = 1;
    let task_id: i64 = 100;
    
    manager.insert(seq, task);
    manager.bind(task_id, seq);
    
    let result = manager.get_by_id(&task_id);
    assert!(result.is_some());
    
    let result = manager.get_by_id(&999);
    assert!(result.is_none());
}

// @tc.name: ut_native_task_manager_multiple_tasks
// @tc.desc: Test NativeTaskManager with multiple tasks
// @tc.precon: NA
// @tc.step: 1. Create NativeTaskManager
//           2. Insert multiple tasks with different sequences
//           3. Bind different task_ids
//           4. Verify all operations work correctly
// @tc.expect: All tasks are managed correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_native_task_manager_multiple_tasks() {
    let manager = NativeTaskManager::default();
    
    for i in 0..5 {
        let config = TaskConfig::default();
        let task = NativeTask {
            config,
            token: vec![],
        };
        let seq = i as u64;
        let task_id = (i + 100) as i64;
        
        manager.insert(seq, task);
        manager.bind(task_id, seq);
    }
    
    let inner = manager.inner.lock().unwrap();
    assert_eq!(inner.tasks.len(), 5);
    assert_eq!(inner.tids.len(), 5);
    drop(inner);
    
    assert!(manager.get_by_seq(&2).is_some());
    assert!(manager.get_by_id(&102).is_some());
    
    manager.remove_task(&100);
    
    let inner = manager.inner.lock().unwrap();
    assert_eq!(inner.tasks.len(), 4);
    assert_eq!(inner.tids.len(), 4);
}

// @tc.name: ut_native_task_manager_insert_overwrite
// @tc.desc: Test NativeTaskManager insert overwrites existing
// @tc.precon: NA
// @tc.step: 1. Create NativeTaskManager
//           2. Insert task with same sequence twice
//           3. Verify second insert overwrites first
// @tc.expect: Only one task exists after overwrite
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_native_task_manager_insert_overwrite() {
    let manager = NativeTaskManager::default();
    
    let config1 = TaskConfig::default();
    let task1 = NativeTask {
        config: config1,
        token: vec![],
    };
    
    let config2 = TaskConfig::default();
    let task2 = NativeTask {
        config: config2,
        token: vec![],
    };
    
    let seq: u64 = 1;
    manager.insert(seq, task1);
    manager.insert(seq, task2);
    
    let inner = manager.inner.lock().unwrap();
    assert_eq!(inner.tasks.len(), 1);
}

// @tc.name: ut_native_task_manager_bind_overwrite
// @tc.desc: Test NativeTaskManager bind overwrites existing
// @tc.precon: NA
// @tc.step: 1. Create NativeTaskManager and insert tasks
//           2. Bind same task_id to different sequences
//           3. Verify second bind overwrites first
// @tc.expect: Latest binding is stored
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_native_task_manager_bind_overwrite() {
    let manager = NativeTaskManager::default();
    
    let config = TaskConfig::default();
    let task = NativeTask {
        config,
        token: vec![],
    };
    
    let seq1: u64 = 1;
    let seq2: u64 = 2;
    let task_id: i64 = 100;
    
    manager.insert(seq1, task);
    manager.bind(task_id, seq1);
    manager.bind(task_id, seq2);
    
    let inner = manager.inner.lock().unwrap();
    assert_eq!(*inner.tids.get(&task_id).unwrap(), seq2);
}

// @tc.name: ut_native_task_manager_remove_nonexistent
// @tc.desc: Test NativeTaskManager remove non-existent task
// @tc.precon: NA
// @tc.step: 1. Create empty NativeTaskManager
//           2. Remove non-existent sequence
//           3. Verify no error occurs
// @tc.expect: Operation completes without error
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_native_task_manager_remove_nonexistent() {
    let manager = NativeTaskManager::default();
    let seq: u64 = 999;
    
    manager.remove(&seq);
    
    let inner = manager.inner.lock().unwrap();
    assert!(inner.tasks.is_empty());
}

// @tc.name: ut_native_task_manager_remove_task_nonexistent
// @tc.desc: Test NativeTaskManager remove_task non-existent task_id
// @tc.precon: NA
// @tc.step: 1. Create empty NativeTaskManager
//           2. Remove non-existent task_id
//           3. Verify no error occurs
// @tc.expect: Operation completes without error
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_native_task_manager_remove_task_nonexistent() {
    let manager = NativeTaskManager::default();
    let task_id: i64 = 999;
    
    manager.remove_task(&task_id);
    
    let inner = manager.inner.lock().unwrap();
    assert!(inner.tasks.is_empty());
    assert!(inner.tids.is_empty());
}
