// Copyright (C) 2024 Huawei Device Co., Ltd.
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

//! Channel opening functionality for inter-process communication.
//! 
//! This module implements methods to establish a communication channel between
//! client processes and the request service, enabling efficient data transfer and
//! task status updates through file descriptors.

use std::fs::File;
use std::os::fd::AsRawFd;
use std::os::unix::io::FromRawFd;

use ipc::parcel::MsgParcel;
use ipc::{IpcResult, IpcStatusCode};

use crate::error::ErrorCode;
use crate::service::RequestServiceStub;

impl RequestServiceStub {
    /// Opens an IPC communication channel for the calling process.
    ///
    /// Establishes a communication channel between the service and a client process
    /// by creating and returning a file descriptor that can be used for subsequent
    /// data exchange and notifications.
    ///
    /// # Arguments
    ///
    /// * `reply` - Output parcel to write the operation result code and file descriptor.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the channel was successfully opened and the file descriptor
    ///   was written to the reply parcel.
    /// * `Err(IpcStatusCode::Failed)` - If opening the channel failed.
    ///
    /// # Errors
    ///
    /// Returns an error code in the reply parcel if:
    /// * The channel could not be opened (`ErrorCode::ParameterCheck`).
    ///
    /// # Notes
    ///
    /// This method performs file descriptor manipulation with `unsafe` blocks to
    /// convert between raw file descriptors and `File` objects. The ownership of
    /// the file descriptor is transferred to the caller through the parcel.
    pub(crate) fn open_channel(&self, reply: &mut MsgParcel) -> IpcResult<()> {
        // Get the PID of the calling process for identification
        let pid = ipc::Skeleton::calling_pid();
        info!("Service open_channel pid {}", pid);
        // Attempt to open a communication channel for the client process
        match self.client_manager.open_channel(pid) {
            Ok(ud_fd) => {
                // Convert the UnixDatagram fd to a raw file descriptor
                // `as_raw_fd` does not track the ownership or life cycle of this fd.
                let fd = ud_fd.as_raw_fd();
                
                // Convert raw file descriptor to a File object
                // Safety: The fd is valid as it was obtained from open_channel and
                // ownership is transferred to the reply parcel
                let file = unsafe { File::from_raw_fd(fd) };
                info!("End open_channel fd {}", fd);
                reply.write(&(ErrorCode::ErrOk as i32))?;
                reply.write_file(file)?;
                Ok(())
            }
            Err(err) => {
                error!("End Service open_channel, failed: {:?}", err);
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A26,
                    &format!("End Service open_channel, failed: {:?}", err)
                );
                reply.write(&(ErrorCode::ParameterCheck as i32))?;
                Err(IpcStatusCode::Failed)
            }
        }
    }
}
