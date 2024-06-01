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

pub(crate) struct QosChanges {
    pub(crate) download: Option<Vec<QosDirection>>,
    pub(crate) upload: Option<Vec<QosDirection>>,
}

impl QosChanges {
    pub(crate) fn new() -> Self {
        Self {
            upload: None,
            download: None,
        }
    }
}

pub(crate) struct QosDirection {
    task_id: u32,
    direction: QosLevel,
}

impl QosDirection {
    pub(crate) fn task_id(&self) -> u32 {
        self.task_id
    }

    pub(crate) fn direction(&self) -> QosLevel {
        self.direction
    }

    pub(crate) fn new(task_id: u32, direction: QosLevel) -> Self {
        Self { task_id, direction }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) enum QosLevel {
    FullSpeed = 0,
    HighSpeed,
    MiddleSpeed,
    LowSpeed,
    BUTT,
}
