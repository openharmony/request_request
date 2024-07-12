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
    IpcSizeTooLarge = 2,
    ChannelNotOpen = 5,
    Permission = 201,
    SystemApi = 202,
    ParameterCheck = 401,
    FileOperationErr = 13400001,
    Other = 13499999,
    TaskEnqueueErr = 21900004,
    TaskNotFound = 21900006,
    TaskStateErr = 21900007,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ut_enum_error_code() {
        assert_eq!(ErrorCode::ErrOk as i32, 0);
        assert_eq!(ErrorCode::IpcSizeTooLarge as i32, 2);
        assert_eq!(ErrorCode::ChannelNotOpen as i32, 5);
        assert_eq!(ErrorCode::Permission as i32, 201);
        assert_eq!(ErrorCode::SystemApi as i32, 202);
        assert_eq!(ErrorCode::ParameterCheck as i32, 401);
        assert_eq!(ErrorCode::FileOperationErr as i32, 13400001);
        assert_eq!(ErrorCode::Other as i32, 13499999);
        assert_eq!(ErrorCode::TaskEnqueueErr as i32, 21900004);
        assert_eq!(ErrorCode::TaskNotFound as i32, 21900006);
        assert_eq!(ErrorCode::TaskStateErr as i32, 21900007);
    }
}
