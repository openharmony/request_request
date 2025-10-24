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

//! IPC interface codes for request operations.
//! 
//! This module defines constants used as IPC command codes for the request service interface.
//! These codes are used to identify different operations in IPC communication between clients
//! and the request service.

/// Constructs a new request.
pub const CONSTRUCT: u32 = 0;
/// Pauses a specific request.
pub const PAUSE: u32 = 1;
/// Queries information about a request.
pub const QUERY: u32 = 2;
/// Queries the MIME type of a request's content.
pub const QUERY_MIME_TYPE: u32 = 3;
/// Removes a request from the system.
pub const REMOVE: u32 = 4;
/// Resumes a paused request.
pub const RESUME: u32 = 5;
/// Starts a new request.
pub const START: u32 = 6;
/// Stops an active request.
pub const STOP: u32 = 7;
/// Shows a request's details in the notification area.
pub const SHOW: u32 = 8;
/// Updates a request's last accessed timestamp.
pub const TOUCH: u32 = 9;
/// Searches for requests matching specific criteria.
pub const SEARCH: u32 = 10;
/// Retrieves a task associated with a request.
pub const GET_TASK: u32 = 11;
/// Clears a request from the system.
pub const CLEAR: u32 = 12;
/// Opens a communication channel for request updates.
pub const OPEN_CHANNEL: u32 = 13;
/// Subscribes to updates for a specific request.
pub const SUBSCRIBE: u32 = 14;
/// Unsubscribes from updates for a specific request.
pub const UNSUBSCRIBE: u32 = 15;
/// Subscribes to running task count updates.
pub const SUB_RUN_COUNT: u32 = 16;
/// Unsubscribes from running task count updates.
pub const UNSUB_RUN_COUNT: u32 = 17;
/// Creates a new request group.
pub const CREATE_GROUP: u32 = 18;
/// Attaches a request to an existing group.
pub const ATTACH_GROUP: u32 = 19;
/// Deletes a request group.
pub const DELETE_GROUP: u32 = 20;
/// Sets the maximum speed limit for a task.
pub const SET_MAX_SPEED: u32 = 21;
/// Shows the progress of a task.
pub const SHOW_PROGRESS: u32 = 22;
/// Changes the mode of a task.
pub const SET_MODE: u32 = 100;
/// Disables notifications for a specific task.
pub const DISABLE_TASK_NOTIFICATION: u32 = 101;

/// Function code for the request notification interface to notify run count changes.
pub(crate) const NOTIFY_RUN_COUNT: u32 = 2;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ut_interface() {
        assert_eq!(0, CONSTRUCT);
        assert_eq!(1, PAUSE);
        assert_eq!(2, QUERY);
        assert_eq!(3, QUERY_MIME_TYPE);
        assert_eq!(4, REMOVE);
        assert_eq!(5, RESUME);
        assert_eq!(6, START);
        assert_eq!(7, STOP);
        assert_eq!(8, SHOW);
        assert_eq!(9, TOUCH);
        assert_eq!(10, SEARCH);
        assert_eq!(11, GET_TASK);
        assert_eq!(12, CLEAR);
        assert_eq!(13, OPEN_CHANNEL);
        assert_eq!(14, SUBSCRIBE);
        assert_eq!(15, UNSUBSCRIBE);
        assert_eq!(16, SUB_RUN_COUNT);
        assert_eq!(17, UNSUB_RUN_COUNT);
        assert_eq!(18, CREATE_GROUP);
        assert_eq!(19, ATTACH_GROUP);
        assert_eq!(20, DELETE_GROUP);
        assert_eq!(100, SET_MODE);
        assert_eq!(101, DISABLE_TASK_NOTIFICATION);
    }
}
