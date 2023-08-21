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

extern crate hisysevent;

use hisysevent::{EventType, HiSysEventParam, write};
pub(crate) use hisysevent::{build_str_param, build_number_param, build_bool_param};

/// System events structure which base on `Hisysevent`.
pub(crate) struct SysEvent<'a> {
    event_kind: EventKind,
    inner_type: EventType,
    params: Vec<HiSysEventParam<'a>>
}

impl<'a> SysEvent<'a> {
    const DOMAIN: &str = "REQUEST";

    pub(crate) const ERROR_INFO: &str = "ERROR_INFO";
    pub(crate) const TASKS_TYPE: &str = "TASKS_TYPE";
    pub(crate) const TOTAL_FILE_NUM: &str = "TOTAL_FILE_NUM";
    pub(crate) const FAIL_FILE_NUM: &str = "FAIL_FILE_NUM";
    pub(crate) const SUCCESS_FILE_NUM: &str = "SUCCESS_FILE_NUM";

    pub(crate) fn task_fault() -> Self {
        Self {
            event_kind: EventKind::TaskFault,
            inner_type: EventType::Fault,
            params: Vec::new(),
        }
    }

    pub(crate) fn task_info_statistics() -> Self {
        Self {
            event_kind: EventKind::TaskInfoStatistics,
            inner_type: EventType::Statistic,
            params: Vec::new(),
        }
    }

    pub(crate) fn param(mut self, param: HiSysEventParam<'a>) -> Self {
        self.params.push(param);
        self
    }

    pub(crate) fn write(self) {
        write(
            Self::DOMAIN,
            self.event_kind.as_str(),
            self.inner_type,
            self.params.as_slice(),
        );
    }
}

enum EventKind {
    TaskFault,
    TaskInfoStatistics,
}

impl EventKind {
    const TASK_FAULT: &str = "TASK_FAULT";
    const TASK_INFO_STATISTICS: &str = "TASK_INFO_STATISTICS";

    fn as_str(&self) -> &str {
        match self {
            EventKind::TaskFault => Self::TASK_FAULT,
            EventKind::TaskInfoStatistics => Self::TASK_INFO_STATISTICS,
        }
    }
}