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

pub(crate) mod task_manager;
pub(crate) use task_manager::TaskManager;

pub(crate) mod keeper;
pub(crate) mod monitor;
pub(crate) mod unload;

pub(crate) mod cert_manager;
pub(crate) mod events;
pub(crate) mod qos;
pub(crate) mod scheduled;
pub(crate) mod system_proxy;

cfg_oh! {
    pub(crate) mod notifier;
}
