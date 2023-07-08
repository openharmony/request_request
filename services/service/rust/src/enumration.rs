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

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum Action {
    DOWNLOAD = 0,
    UPLOAD,
    ANY,
}

impl From<u8> for Action {
    fn from(value: u8) -> Self {
        match value {
            0 => Action::DOWNLOAD,
            1 => Action::UPLOAD,
            _ => Action::ANY,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum Mode {
    BACKGROUND = 0,
    FRONTEND,
    ANY,
}

impl From<u8> for Mode {
    fn from(value: u8) -> Self {
        match value {
            0 => Mode::BACKGROUND,
            1 => Mode::FRONTEND,
            _ => Mode::ANY,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8, C)]
pub enum Network {
    ANY = 0,
    WIFI,
    CELLULAR,
}

#[derive(Debug)]
#[repr(C)]
pub struct NetworkInfo {
    pub networkType: Network,
    pub isMetered: bool,
    pub isRoaming: bool,
}

impl From<u8> for Network {
    fn from(value: u8) -> Self {
        match value {
            0 => Network::ANY,
            2 => Network::CELLULAR,
            _ => Network::WIFI,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum State {
    INITIALIZED = 0x00,
    WAITING = 0x10,
    RUNNING = 0x20,
    RETRYING = 0x21,
    PAUSED = 0x30,
    STOPPED = 0x31,
    COMPLETED = 0x40,
    FAILED = 0x41,
    REMOVED = 0x50,
    CREATED = 0x60,
    ANY = 0x61,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum ApplicationState {
    AppStateBegin = 0,
    AppStateReady,
    AppStateForeground,
    AppStateFocus,
    AppStateBackground,
    AppStateTerminated,
    AppStateEnd,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum ErrorCode {
    ErrOk = 0,
    UnloadingSA = 1,
    Ipc_size_too_large = 2,
    MimeType_not_found = 3,
    Task_index_too_large = 4,
    Permission = 201,
    SystemApi = 202,
    Parameter_check = 401,
    FileOperationErr = 13400001,
    ServiceAbilityErr = 13400003,
    Other = 13499999,
    TaskEnqueueErr = 21900004,
    TaskModeErr,
    TaskNotFound,
    TaskStateErr,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum Version {
    API9 = 1,
    API10,
}

impl From<u8> for Version {
    fn from(value: u8) -> Self {
        match value {
            2 => Version::API10,
            _ => Version::API9,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum Reason {
    Default = 0,
    TaskSurvivalOneMonth,
    WaittingNetWorkOneday,
    StoppedByNewFrontTask,
    RunningTaskMeetLimits,
    UserOperation,
    AppBackgroundOrTerminate,
    NetWorkOffline,
    UnSupportedNetWorkType,
    BuildClientFailed,
    BuildRequestFailed,
    GetFileSizeFailed,
    ContinuousTaskTimeOut,
    ConnectError,
    RequestError,
    UploadFileError,
    RedirectError,
    ProtocolError,
    IoError,
    UnSupportRangeRequest,
    OthersError,
}

impl From<u8> for Reason {
    fn from(value: u8) -> Self {
        match value {
            0 => Reason::Default,
            1 => Reason::TaskSurvivalOneMonth,
            2 => Reason::WaittingNetWorkOneday,
            3 => Reason::StoppedByNewFrontTask,
            4 => Reason::RunningTaskMeetLimits,
            5 => Reason::UserOperation,
            6 => Reason::AppBackgroundOrTerminate,
            7 => Reason::NetWorkOffline,
            8 => Reason::UnSupportedNetWorkType,
            9 => Reason::BuildClientFailed,
            10 => Reason::BuildRequestFailed,
            11 => Reason::GetFileSizeFailed,
            12 => Reason::ContinuousTaskTimeOut,
            13 => Reason::ConnectError,
            14 => Reason::RequestError,
            15 => Reason::UploadFileError,
            16 => Reason::RedirectError,
            17 => Reason::ProtocolError,
            18 => Reason::IoError,
            19 => Reason::UnSupportRangeRequest,
            _ => Reason::OthersError,
        }
    }
}

impl Reason {
    pub fn to_str(&self) -> &'static str {
        match self {
            Reason::Default => "".into(),
            Reason::TaskSurvivalOneMonth => "The task has not been completed for a month yet",
            Reason::WaittingNetWorkOneday => "The task waiting for network recovery has not been completed for a day yet",
            Reason::StoppedByNewFrontTask => "Stopped by a new front task",
            Reason::RunningTaskMeetLimits => "Too many task in running state",
            Reason::UserOperation => "User operation",
            Reason::AppBackgroundOrTerminate => "The app is background or terminate",
            Reason::NetWorkOffline => "NetWork is offline",
            Reason::UnSupportedNetWorkType => "NetWork type not meet the task config",
            Reason::BuildClientFailed => "Build client error",
            Reason::BuildRequestFailed => "Build request error",
            Reason::GetFileSizeFailed => "Failed because cannot get the file size from the server and the precise is setted true by user",
            Reason::ContinuousTaskTimeOut => "Continuous processing task time out",
            Reason::ConnectError => "Connect error",
            Reason::RequestError => "Request error",
            Reason::UploadFileError => "There are some files upload failed",
            Reason::RedirectError => "Redirect error",
            Reason::ProtocolError => "Http protocol error",
            Reason::IoError => "Io Error",
            Reason::UnSupportRangeRequest => "The server is not support range request",
            Reason::OthersError => "Some other error occured",
        }
    }
}

#[derive(PartialEq)]
#[repr(u8, C)]
pub enum QueryPermission {
    NoPermisson = 0,
    QueryDownLoad,
    QueryUpload,
    QueryAll,
}