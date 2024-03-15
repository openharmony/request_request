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

// SAID: 3706
/// Function code of RequestInterfaceCode
pub(crate) enum RequestInterfaceCode {
    /// request construct & api10 create task
    Construct = 0,
    /// pause task
    Pause,
    /// query task || system api Queries specified task details
    Query,
    /// query mime type
    QueryMimeType,
    /// remove task || removes specifed task belongs to the caller
    Remove,
    /// resume task
    Resume,
    /// ap10 start task
    Start,
    /// stop task
    Stop,
    ///  Shows specified task details belongs to the caller
    Show,
    /// Touches specified task with token
    Touch,
    ///  Searches tasks, for system
    Search,
    ///  get task
    GetTask,
    ///  system api deletes specifed tasks
    Clear,
    ///  open the channel for ipc
    OpenChannel,
    ///  subscribe response
    Subscribe,
    ///  unsubscribe response
    Unsubscribe,
    /// subscribe running task count
    SubRunCount,
    /// unsubscribe running task count
    UnsubRunCount,
}

/// Function code of RequestNotifyInterfaceCode
pub(crate) enum RequestNotifyInterfaceCode {
    /// callback notification
    Notify = 0,
    /// Cache callback notification
    DoneNotify,
    /// run count notification
    NotifyRunCount,
}
