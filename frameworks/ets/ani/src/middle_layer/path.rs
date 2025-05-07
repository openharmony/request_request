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

use request_core::TaskConfig;

// TODO： 标准化路径， 各种入参路径统一变为完全体沙箱路径
pub struct JsContext;
pub(crate) fn standardize_path(_context: JsContext, config: &mut TaskConfig) -> String {
    "".to_owned()
}
