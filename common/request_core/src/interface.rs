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

//! Request Ipc Code

/// Construct a new request.
pub const CONSTRUCT: u32 = 0;
/// Pause A Request.
pub const PAUSE: u32 = 1;
/// Query a request.
pub const QUERY: u32 = 2;
/// Query a request's mime type.
pub const QUERY_MIME_TYPE: u32 = 3;
/// Remove a request.
pub const REMOVE: u32 = 4;
/// Resume a request.
pub const RESUME: u32 = 5;
/// Start a request.
pub const START: u32 = 6;
/// Stop a request.
pub const STOP: u32 = 7;
/// Show a request.
pub const SHOW: u32 = 8;
/// Touch a request.
pub const TOUCH: u32 = 9;
/// Search a request.
pub const SEARCH: u32 = 10;
/// Get a task.
pub const GET_TASK: u32 = 11;
/// Clear a request.
pub const CLEAR: u32 = 12;
/// Open a channel.
pub const OPEN_CHANNEL: u32 = 13;
/// Subscribe a request.
pub const SUBSCRIBE: u32 = 14;
/// Unsubscribe a request.
pub const UNSUBSCRIBE: u32 = 15;
/// Subscribe run count.
pub const SUB_RUN_COUNT: u32 = 16;
/// Unsubscribe run count.
pub const UNSUB_RUN_COUNT: u32 = 17;
/// Create a group.
pub const CREATE_GROUP: u32 = 18;
/// Attach a group.
pub const ATTACH_GROUP: u32 = 19;
/// Delete a group.
pub const DELETE_GROUP: u32 = 20;
/// Set the max speed of a task
pub const SET_MAX_SPEED: u32 = 21;
/// Change task mode.
pub const SET_MODE: u32 = 100;
/// Change task mode.
pub const DISABLE_TASK_NOTIFICATION: u32 = 101;

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
