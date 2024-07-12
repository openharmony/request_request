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

pub(crate) use ffi::Reason;

#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {
    #[derive(Clone, Copy, PartialEq, Debug)]
    #[repr(u8)]
    pub(crate) enum Reason {
        Default = 0,
        TaskSurvivalOneMonth,
        WaitingNetworkOneDay,  // unused
        StoppedByNewFrontTask, // unused
        RunningTaskMeetLimits,
        UserOperation,
        AppBackgroundOrTerminate,
        NetworkOffline,
        UnsupportedNetworkType,
        BuildClientFailed, // unused
        BuildRequestFailed,
        GetFileSizeFailed,
        ContinuousTaskTimeout,
        ConnectError, // unused
        RequestError,
        UploadFileError,
        RedirectError,
        ProtocolError,
        IoError,
        UnsupportedRangeRequest,
        OthersError,
        AccountStopped,
        Dns = 23,
        Tcp,
        Ssl,
        InsufficientSpace,
    }
}

impl From<u8> for Reason {
    fn from(value: u8) -> Self {
        match value {
            0 => Reason::Default,
            1 => Reason::TaskSurvivalOneMonth,
            2 => Reason::WaitingNetworkOneDay,
            3 => Reason::StoppedByNewFrontTask,
            4 => Reason::RunningTaskMeetLimits,
            5 => Reason::UserOperation,
            6 => Reason::AppBackgroundOrTerminate,
            7 => Reason::NetworkOffline,
            8 => Reason::UnsupportedNetworkType,
            9 => Reason::BuildClientFailed,
            10 => Reason::BuildRequestFailed,
            11 => Reason::GetFileSizeFailed,
            12 => Reason::ContinuousTaskTimeout,
            13 => Reason::ConnectError,
            14 => Reason::RequestError,
            15 => Reason::UploadFileError,
            16 => Reason::RedirectError,
            17 => Reason::ProtocolError,
            18 => Reason::IoError,
            19 => Reason::UnsupportedRangeRequest,
            21 => Reason::AccountStopped,
            23 => Reason::Dns,
            24 => Reason::Tcp,
            25 => Reason::Ssl,
            26 => Reason::InsufficientSpace,
            _ => Reason::OthersError,
        }
    }
}

impl Reason {
    pub(crate) fn to_str(self) -> &'static str {
        match self {
            Reason::Default => "",
            Reason::TaskSurvivalOneMonth => "The task has not been completed for a month yet",
            Reason::WaitingNetworkOneDay => "The task waiting for network recovery has not been completed for a day yet",
            Reason::StoppedByNewFrontTask => "Stopped by a new front task",
            Reason::RunningTaskMeetLimits => "Too many task in running state",
            Reason::UserOperation => "User operation",
            Reason::AppBackgroundOrTerminate => "The app is background or terminate",
            Reason::NetworkOffline => "NetWork is offline",
            Reason::UnsupportedNetworkType => "NetWork type not meet the task config",
            Reason::BuildClientFailed => "Build client error",
            Reason::BuildRequestFailed => "Build request error",
            Reason::GetFileSizeFailed => "Failed because cannot get the file size from the server and the precise is setted true by user",
            Reason::ContinuousTaskTimeout => "Continuous processing task time out",
            Reason::ConnectError => "Connect error",
            Reason::RequestError => "Request error",
            Reason::UploadFileError => "There are some files upload failed",
            Reason::RedirectError => "Redirect error",
            Reason::ProtocolError => "Http protocol error",
            Reason::IoError => "Io Error",
            Reason::UnsupportedRangeRequest => "The server is not support range request",
            Reason::OthersError => "Some other error occured",
            Reason::AccountStopped => "Account stopped",
            Reason::Dns => "DNS error",
            Reason::Tcp => "TCP error",
            Reason::Ssl => "TSL/SSL error",
            Reason::InsufficientSpace => "Insufficient space",
            _ => "unknown error",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ut_enum_reason() {
        assert_eq!(Reason::Default.repr, 0);
        assert_eq!(Reason::TaskSurvivalOneMonth.repr, 1);
        assert_eq!(Reason::WaitingNetworkOneDay.repr, 2);
        assert_eq!(Reason::StoppedByNewFrontTask.repr, 3);
        assert_eq!(Reason::RunningTaskMeetLimits.repr, 4);
        assert_eq!(Reason::UserOperation.repr, 5);
        assert_eq!(Reason::AppBackgroundOrTerminate.repr, 6);
        assert_eq!(Reason::NetworkOffline.repr, 7);
        assert_eq!(Reason::UnsupportedNetworkType.repr, 8);
        assert_eq!(Reason::BuildClientFailed.repr, 9);
        assert_eq!(Reason::BuildRequestFailed.repr, 10);
        assert_eq!(Reason::GetFileSizeFailed.repr, 11);
        assert_eq!(Reason::ContinuousTaskTimeout.repr, 12);
        assert_eq!(Reason::ConnectError.repr, 13);
        assert_eq!(Reason::RequestError.repr, 14);
        assert_eq!(Reason::UploadFileError.repr, 15);
        assert_eq!(Reason::RedirectError.repr, 16);
        assert_eq!(Reason::ProtocolError.repr, 17);
        assert_eq!(Reason::IoError.repr, 18);
        assert_eq!(Reason::UnsupportedRangeRequest.repr, 19);
        assert_eq!(Reason::OthersError.repr, 20);
        assert_eq!(Reason::AccountStopped.repr, 21);
        assert_eq!(Reason::Dns.repr, 23);
        assert_eq!(Reason::Tcp.repr, 24);
        assert_eq!(Reason::Ssl.repr, 25);
        assert_eq!(Reason::InsufficientSpace.repr, 26);
    }
}
