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

#[repr(i32)]
pub enum ExceptionErrorCode {
    E_OK = 0,
    E_UNLOADING_SA,
    E_IPC_SIZE_TOO_LARGE,
    E_MIMETYPE_NOT_FOUND,
    E_TASK_INDEX_TOO_LARGE,
    E_CHANNEL_NOT_OPEN = 5,
    E_PERMISSION = 201,
    E_NOT_SYSTEM_APP = 202,
    E_PARAMETER_CHECK = 401,
    E_UNSUPPORTED = 801,
    E_FILE_IO = 13400001,
    E_FILE_PATH = 13400002,
    E_SERVICE_ERROR = 13400003,
    E_OTHER = 13499999,
    E_TASK_QUEUE = 21900004,
    E_TASK_MODE = 21900005,
    E_TASK_NOT_FOUND = 21900006,
    E_TASK_STATE = 21900007,
    E_GROUP_NOT_FOUND = 21900008,
}