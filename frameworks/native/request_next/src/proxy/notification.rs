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

use crate::proxy::RequestProxy;

impl RequestProxy {
    pub(crate) fn create_group(&self) -> Result<(), i32> {
        todo!()
    }

    pub(crate) fn delete_group(&self, group_id: i64) -> Result<(), i32> {
        todo!()
    }

    pub(crate) fn attach_group(&self, group_id: i64, task_ids: Vec<i64>) -> Result<(), i32> {
        todo!()
    }
}
