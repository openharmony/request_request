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

use hisysevent::{build_number_param, build_str_param, write, EventType, HiSysEventParam};

const DOMAIN: &str = "REQUEST";
const DONWLOAD_SA: &str = "DOWNLOAD_SERVER";

pub(crate) const ERROR_INFO: &str = "ERROR_INFO";
pub(crate) const TASKS_TYPE: &str = "TASKS_TYPE";
pub(crate) const TOTAL_FILE_NUM: &str = "TOTAL_FILE_NUM";
pub(crate) const FAIL_FILE_NUM: &str = "FAIL_FILE_NUM";
pub(crate) const SUCCESS_FILE_NUM: &str = "SUCCESS_FILE_NUM";

pub(crate) const PARAM_DFX_CODE: &str = "CODE";
pub(crate) const PARAM_BUNDLE_NAME: &str = "BUNDLE_NAME";
pub(crate) const PARAM_MODULE_NAME: &str = "MODULE_NAME";
pub(crate) const PARAM_EXTRA_INFO: &str = "EXTRA_INFO";

/// System events structure which base on `Hisysevent`.
pub(crate) struct SysEvent<'a> {
    event_kind: EventKind,
    inner_type: EventType,
    params: Vec<HiSysEventParam<'a>>,
}

impl<'a> SysEvent<'a> {
    pub(crate) fn task_fault() -> Self {
        Self {
            event_kind: EventKind::TaskFault,
            inner_type: EventType::Fault,
            params: Vec::new(),
        }
    }

    pub(crate) fn exec_error() -> Self {
        Self {
            event_kind: EventKind::ExecError,
            inner_type: EventType::Statistic,
            params: Vec::new(),
        }
    }

    pub(crate) fn exec_fault() -> Self {
        Self {
            event_kind: EventKind::ExecFault,
            inner_type: EventType::Fault,
            params: Vec::new(),
        }
    }

    pub(crate) fn param(mut self, param: HiSysEventParam<'a>) -> Self {
        self.params.push(param);
        self
    }

    pub(crate) fn write(self) {
        write(
            DOMAIN,
            self.event_kind.as_str(),
            self.inner_type,
            self.params.as_slice(),
        );
    }
}

pub(crate) enum EventKind {
    TaskFault,
    ExecError,
    ExecFault,
}

impl EventKind {
    fn as_str(&self) -> &str {
        match self {
            EventKind::TaskFault => "TASK_FAULT",
            EventKind::ExecError => "EXEC_ERROR",
            EventKind::ExecFault => "EXEC_FAULT",
        }
    }
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub(crate) enum DfxCode {
    INVALID_IPC_MESSAGE_A00 = 0x001FFFFF,
    INVALID_IPC_MESSAGE_A01 = 0x001F0000,
    INVALID_IPC_MESSAGE_A02 = 0x001F0001,
    INVALID_IPC_MESSAGE_A03 = 0x001F0100,
    INVALID_IPC_MESSAGE_A04 = 0x001F0101,
    INVALID_IPC_MESSAGE_A05 = 0x001F0200,
    INVALID_IPC_MESSAGE_A06 = 0x001F0201,
    INVALID_IPC_MESSAGE_A07 = 0x001F0300,
    INVALID_IPC_MESSAGE_A08 = 0x001F0301,
    INVALID_IPC_MESSAGE_A09 = 0x001F0400,
    INVALID_IPC_MESSAGE_A10 = 0x001F0401,
    INVALID_IPC_MESSAGE_A11 = 0x001F0500,
    INVALID_IPC_MESSAGE_A12 = 0x001F0501,
    INVALID_IPC_MESSAGE_A13 = 0x001F0600,
    INVALID_IPC_MESSAGE_A14 = 0x001F0601,
    INVALID_IPC_MESSAGE_A15 = 0x001F0700,
    INVALID_IPC_MESSAGE_A16 = 0x001F0701,
    INVALID_IPC_MESSAGE_A17 = 0x001F0800,
    INVALID_IPC_MESSAGE_A18 = 0x001F0801,
    INVALID_IPC_MESSAGE_A19 = 0x001F0900,
    INVALID_IPC_MESSAGE_A20 = 0x001F0901,
    INVALID_IPC_MESSAGE_A21 = 0x001F0A00,
    INVALID_IPC_MESSAGE_A22 = 0x001F0A01,
    INVALID_IPC_MESSAGE_A23 = 0x001F0B00,
    INVALID_IPC_MESSAGE_A24 = 0x001F0B01,
    INVALID_IPC_MESSAGE_A25 = 0x001F0C00,
    INVALID_IPC_MESSAGE_A26 = 0x001F0C01,
    INVALID_IPC_MESSAGE_A27 = 0x001F0D00,
    INVALID_IPC_MESSAGE_A28 = 0x001F0D01,
    INVALID_IPC_MESSAGE_A29 = 0x001F0E00,
    INVALID_IPC_MESSAGE_A30 = 0x001F0E01,
    INVALID_IPC_MESSAGE_A31 = 0x001F0F00,
    INVALID_IPC_MESSAGE_A32 = 0x001F0F01,
    INVALID_IPC_MESSAGE_A33 = 0x001F1000,
    INVALID_IPC_MESSAGE_A34 = 0x001F1001,
    INVALID_IPC_MESSAGE_A35 = 0x001F1100,
    INVALID_IPC_MESSAGE_A36 = 0x001F1101,
    INVALID_IPC_MESSAGE_A37 = 0x001F1200,
    INVALID_IPC_MESSAGE_A38 = 0x001F1201,
    INVALID_IPC_MESSAGE_A39 = 0x001F1300,
    INVALID_IPC_MESSAGE_A40 = 0x001F1301,
    INVALID_IPC_MESSAGE_A41 = 0x001F1400,
    INVALID_IPC_MESSAGE_A42 = 0x001F1401,
    INVALID_IPC_MESSAGE_A43 = 0x001F1500,
    INVALID_IPC_MESSAGE_A44 = 0x001F1501,
    INVALID_IPC_MESSAGE_A45 = 0x001F1600,
    INVALID_IPC_MESSAGE_A46 = 0x001F1601,
    TASK_STATISTICS = 0x002F0000,
    TASK_FAULT_00 = 0x002F00FF,
    TASK_FAULT_01 = 0x002F01FF,
    TASK_FAULT_02 = 0x002F02FF,
    TASK_FAULT_03 = 0x002F03FF,
    TASK_FAULT_04 = 0x002F04FF,
    TASK_FAULT_05 = 0x002F05FF,
    TASK_FAULT_06 = 0x002F06FF,
    TASK_FAULT_07 = 0x002F07FF,
    TASK_FAULT_08 = 0x002F08FF,
    TASK_FAULT_09 = 0x002FFFFF,
    UDS_FAULT_00 = 0x00300000,
    UDS_FAULT_01 = 0x00300001,
    UDS_FAULT_02 = 0x00300002,
    UDS_FAULT_03 = 0x003F0000,
    UDS_FAULT_04 = 0x003F0001,
    SA_ERROR_00 = 0x004F0000,
    SA_ERROR_01 = 0x004F0001,
    SA_ERROR_02 = 0x004F0002,
    SA_FAULT_00 = 0x005F0000,
    SA_FAULT_01 = 0x005F0001,
    SAMGR_FAULT_A00 = 0xF02F0000,
    SAMGR_FAULT_A01 = 0xF02F0001,
    SAMGR_FAULT_A02 = 0xF02F0002,
    ABMS_FAULT_A00 = 0xF03F0000,
    ABMS_FAULT_A01 = 0xF03F0001,
    BMS_FAULT_00 = 0xF04F0000,
    OS_ACCOUNT_FAULT_00 = 0xF05F0000,
    OS_ACCOUNT_FAULT_01 = 0xF05F0001,
    OS_ACCOUNT_FAULT_02 = 0xF05F0002,
    RDB_FAULT_00 = 0xF06F0000,
    RDB_FAULT_01 = 0xF06F0001,
    RDB_FAULT_02 = 0xF06F0002,
    RDB_FAULT_03 = 0xF06F0003,
    RDB_FAULT_04 = 0xF06F0004,
    RDB_FAULT_05 = 0xF06F0005,
    RDB_FAULT_06 = 0xF06F0006,
    RDB_FAULT_07 = 0xF06F0007,
    RDB_FAULT_08 = 0xF06F0008,
    RDB_FAULT_09 = 0xF06F0009,
    RDB_FAULT_10 = 0xF06F000A,
    RDB_FAULT_11 = 0xF06F000B,
    RDB_FAULT_12 = 0xF06F000C,
    RDB_FAULT_13 = 0xF06FFFFF,
    EVENT_FAULT_00 = 0xF07F0000,
    EVENT_FAULT_01 = 0xF07F0001,
    EVENT_FAULT_02 = 0xF07F0002,
    NET_CONN_CLIENT_FAULT_00 = 0xF08F0000,
    NET_CONN_CLIENT_FAULT_01 = 0xF08F0001,
    NET_CONN_CLIENT_FAULT_02 = 0xF08F0002,
    NET_CONN_CLIENT_FAULT_03 = 0xF08F0003,
    TELEPHONY_FAULT_00 = 0xF09F0000,
    TELEPHONY_FAULT_01 = 0xF09F0001,
    SYSTEM_RESOURCE_FAULT_00 = 0xF0AF0000,
    SYSTEM_RESOURCE_FAULT_01 = 0xF0AF0001,
    SYSTEM_RESOURCE_FAULT_02 = 0xF0AF0002,
    MEDIA_FAULT_00 = 0xF0BF0000,
    MEDIA_FAULT_01 = 0xF0BF0001,
    NOTIFICATION_FAULT_00 = 0xF0CF0000,
    CERT_MANAGER_FAULT_00 = 0xF0DF0000,
    CERT_MANAGER_FAULT_01 = 0xF0DF0001,
    ACCESS_TOKEN_FAULT_00 = 0xF0EF0000,
    ACCESS_TOKEN_FAULT_01 = 0xF0EF0001,
    ACCESS_TOKEN_FAULT_02 = 0xF0EF0002,
    URL_POLICY_FAULT_00 = 0xF0FF0000,
    STANDARD_FAULT_00 = 0xF1000000,
    STANDARD_FAULT_01 = 0xF1000001,
    STANDARD_FAULT_02 = 0xF1000002,
    STANDARD_FAULT_03 = 0xF1000003,
    STANDARD_FAULT_04 = 0xF1000004,
    STANDARD_FAULT_05 = 0xF1000005,
    STANDARD_FAULT_06 = 0xF1000006,
    STANDARD_FAULT_A01 = 0xF10F0000,
}

pub(crate) fn sys_task_fault(
    action: &str,
    total_file: i32,
    fail_file: i32,
    succ_file: i32,
    reason_err: i32,
) {
    SysEvent::task_fault()
        .param(build_str_param!(TASKS_TYPE, action))
        .param(build_number_param!(TOTAL_FILE_NUM, total_file))
        .param(build_number_param!(FAIL_FILE_NUM, fail_file))
        .param(build_number_param!(SUCCESS_FILE_NUM, succ_file))
        .param(build_number_param!(ERROR_INFO, reason_err))
        .write();
}

pub(crate) fn isys_fault(dfx_code: DfxCode, extra_info: &str) {
    SysEvent::exec_fault()
        .param(build_number_param!(PARAM_DFX_CODE, dfx_code as u32))
        .param(build_str_param!(PARAM_BUNDLE_NAME, DONWLOAD_SA))
        .param(build_str_param!(PARAM_MODULE_NAME, DONWLOAD_SA))
        .param(build_str_param!(PARAM_EXTRA_INFO, extra_info))
        .write();
}

pub(crate) fn isys_error(dfx_code: DfxCode, extra_info: &str) {
    SysEvent::exec_error()
        .param(build_number_param!(PARAM_DFX_CODE, dfx_code as u32))
        .param(build_str_param!(PARAM_BUNDLE_NAME, DONWLOAD_SA))
        .param(build_str_param!(PARAM_MODULE_NAME, DONWLOAD_SA))
        .param(build_str_param!(PARAM_EXTRA_INFO, extra_info))
        .write();
}
