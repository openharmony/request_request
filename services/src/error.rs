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

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum ErrorCode {
    ErrOk = 0,
    _UnloadingSA = 1,
    IpcSizeTooLarge = 2,
    ChannelNotOpen = 5,
    Permission = 201,
    SystemApi = 202,
    ParameterCheck = 401,
    FileOperationErr = 13400001,
    Other = 13499999,
    TaskEnqueueErr = 21900004,
    _TaskModeErr,
    TaskNotFound,
    TaskStateErr,
}
