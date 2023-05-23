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
//! This create implement the request proxy and stub
#![allow(dead_code, unused_imports, unused_variables)]
extern crate ipc_rust;
#[macro_use]
extern crate hilog_rust;
pub mod enumration;
mod request_binding;
mod request_service;
mod request_service_ability;
pub mod request_task;
pub mod task_config;
pub mod task_info;
pub mod task_manager;
pub mod form_item;
mod log;
pub mod progress;
mod utils;

use enumration::ErrorCode;
use hilog_rust::*;
use ipc_rust::{
    define_remote_object, BorrowedMsgParcel, IRemoteBroker, InterfaceToken, IpcResult,
    IpcStatusCode, RemoteObj, RemoteStub,
};
pub use log::LOG_LABEL;
pub use request_service::{start, stop, RequestService};
use std::convert::{TryFrom, TryInto};
use std::{
    ffi::{c_char, CString},
    file,
    option::Option,
};
use task_manager::*;

/// Function code of RequestCode
pub enum RequestCode {
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
    /// on task
    On,
    /// off task
    Off,
    /// ap10 start task
    Start,
    /// stop task
    Stop,
    ///  Shows specified task details belongs to the caller
    Show,
    /// Touches specified task with token.
    Touch,
    ///  Searches tasks, for system.
    Search,
    ///  system api deletes specifed tasks.
    Clear,
}

impl TryFrom<u32> for RequestCode {
    type Error = IpcStatusCode;
    fn try_from(code: u32) -> IpcResult<Self> {
        match code {
            _ if code == RequestCode::Construct as u32 => Ok(RequestCode::Construct),
            _ if code == RequestCode::Pause as u32 => Ok(RequestCode::Pause),
            _ if code == RequestCode::Query as u32 => Ok(RequestCode::Query),
            _ if code == RequestCode::QueryMimeType as u32 => Ok(RequestCode::QueryMimeType),
            _ if code == RequestCode::Remove as u32 => Ok(RequestCode::Remove),
            _ if code == RequestCode::Resume as u32 => Ok(RequestCode::Resume),
            _ if code == RequestCode::On as u32 => Ok(RequestCode::On),
            _ if code == RequestCode::Off as u32 => Ok(RequestCode::Off),
            _ if code == RequestCode::Start as u32 => Ok(RequestCode::Start),
            _ if code == RequestCode::Stop as u32 => Ok(RequestCode::Stop),
            _ if code == RequestCode::Show as u32 => Ok(RequestCode::Show),
            _ if code == RequestCode::Touch as u32 => Ok(RequestCode::Touch),
            _ if code == RequestCode::Search as u32 => Ok(RequestCode::Search),
            _ if code == RequestCode::Clear as u32 => Ok(RequestCode::Clear),
            _ => Err(IpcStatusCode::Failed),
        }
    }
}

/// Function between proxy and stub of RequestServiceInterface
pub trait RequestServiceInterface: IRemoteBroker {
    /// request construct--create task
    fn construct(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()>;
    /// pause--task object
    fn pause(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()>;
    /// resume--task object
    fn resume(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()>;
    /// on--task object
    fn on(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()>;
    /// off--task object
    fn off(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()>;
    /// start task--task object
    fn start(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()>;
    /// stop task--task object
    fn stop(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()>;
    /// query mime type
    fn query_mime_type(
        &self,
        data: &BorrowedMsgParcel,
        reply: &mut BorrowedMsgParcel,
    ) -> IpcResult<()>;
    /// remove
    fn remove(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()>;
    /// Shows specified task details belongs to the caller.
    fn show(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()>;
    /// Touches specified task with token.
    fn touch(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()>;
    /// Searches tasks, for system.
    fn search(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()>;
    /// Deletes tasks  system api
    fn clear(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()>;
}

fn on_remote_request(
    stub: &dyn RequestServiceInterface,
    code: u32,
    data: &BorrowedMsgParcel,
    reply: &mut BorrowedMsgParcel,
) -> IpcResult<()> {
    info!(LOG_LABEL, "on_remote_request code {}", @public(code));
    let service_token: InterfaceToken =
        InterfaceToken::new("OHOS.Download.RequestServiceInterface");
    let token: InterfaceToken = match data.read::<InterfaceToken>() {
        Ok(i) => i,
        _ => InterfaceToken::new("token error"),
    };
    if service_token.get_token() != token.get_token() {
        error!(LOG_LABEL, "token error");
        return Err(IpcStatusCode::Failed);
    }
    match code.try_into()? {
        RequestCode::Construct => stub.construct(data, reply),
        RequestCode::Pause => stub.pause(data, reply),
        RequestCode::Query => stub.show(data, reply),
        RequestCode::QueryMimeType => stub.query_mime_type(data, reply),
        RequestCode::Remove => stub.remove(data, reply),
        RequestCode::Resume => stub.resume(data, reply),
        RequestCode::On => stub.on(data, reply),
        RequestCode::Off => stub.off(data, reply),
        RequestCode::Start => stub.start(data, reply),
        RequestCode::Stop => stub.stop(data, reply),
        RequestCode::Show => stub.show(data, reply),
        RequestCode::Touch => stub.touch(data, reply),
        RequestCode::Search => stub.search(data, reply),
        RequestCode::Clear => stub.clear(data, reply),
    }
}

define_remote_object!(
    RequestServiceInterface["ohos.request.service"] {
        stub: RequestServiceStub(on_remote_request),
        proxy: RequestServiceProxy,
    }
);

// Make RemoteStub<RequestServiceStub> object can call RequestServiceInterface function directly.
impl RequestServiceInterface for RemoteStub<RequestServiceStub> {
    fn construct(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        self.0.construct(data, reply)
    }

    fn pause(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        self.0.pause(data, reply)
    }

    fn query_mime_type(
        &self,
        data: &BorrowedMsgParcel,
        reply: &mut BorrowedMsgParcel,
    ) -> IpcResult<()> {
        self.0.query_mime_type(data, reply)
    }

    fn remove(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        self.0.remove(data, reply)
    }

    fn resume(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        self.0.resume(data, reply)
    }

    fn on(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        self.0.on(data, reply)
    }

    fn off(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        self.0.off(data, reply)
    }

    fn start(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        self.0.start(data, reply)
    }

    fn stop(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        self.0.stop(data, reply)
    }

    fn search(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        self.0.search(data, reply)
    }

    fn show(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        self.0.show(data, reply)
    }

    fn touch(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        self.0.touch(data, reply)
    }

    fn clear(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        self.0.clear(data, reply)
    }
}

impl RequestServiceInterface for RequestServiceProxy {
    fn construct(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }

    fn pause(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }

    fn query_mime_type(
        &self,
        data: &BorrowedMsgParcel,
        reply: &mut BorrowedMsgParcel,
    ) -> IpcResult<()> {
        Ok(())
    }

    fn remove(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }

    fn resume(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }

    fn on(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }

    fn off(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }

    fn start(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }

    fn stop(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }

    fn search(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }

    fn show(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }

    fn touch(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }

    fn clear(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }
}
