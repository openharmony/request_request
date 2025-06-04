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

use ani_rs::ani_constructor;

pub mod api10;
pub mod api9;
mod seq;

#[macro_use]
extern crate request_utils;

const TAG: &str = "RequestAni\0";
const DOMAIN: u32 = 0xD001C50;

ani_constructor!(
    namespace "L@ohos/request/request"
    [
        "downloadFileSync": api9::download::download_file,
        "uploadFileSync": api9::upload::upload_file,
    ]
    class "L@ohos/request/request/DownloadTaskInner"
    [
        "onProgress": api9::callback::on_progress,
        "onEvent": api9::callback::on_event,
        "onFail": api9::callback::on_fail,
        "deleteSync": api9::download::delete,
        "suspendSync": api9::download::suspend,
        "restoreSync": api9::download::restore,
        "getTaskInfoSync": api9::download::get_task_info,
        "getTaskMimeTypeSync": api9::download::get_task_mime_type,
    ]
    class "L@ohos/request/request/UploadTaskInner"
    [
        "deleteSync": api9::upload::delete,
    ]
    namespace "L@ohos/request/request/agent"
    [
        "createSync": api10::agent::create,
        "getTaskSync": api10::agent::get_task,
        "removeSync": api10::agent::remove,
        "showSync": api10::agent::show,
        "touchSync": api10::agent::touch,
        "searchSync": api10::agent::search,
        "querySync": api10::agent::query,
        "createGroupSync": api10::notification::create_group,
        "attachGroupSync": api10::notification::attach_group,
        "deleteGroupSync": api10::notification::delete_group,
    ]
    class "L@ohos/request/request/agent/TaskInner"
    [
        "startSync": api10::task::start,
        "pauseSync": api10::task::pause,
        "resumeSync": api10::task::resume,
        "stopSync": api10::task::stop,
        "setMaxSpeedSync": api10::task::set_max_speed,
    ]
);

#[used]
#[link_section = ".init_array"]
static A: extern "C" fn() = {
    #[link_section = ".text.startup"]
    extern "C" fn init() {
        info!("begin request service init");
        std::panic::set_hook(Box::new(|info| {
            info!("Panic occurred: {:?}", info);
        }));
    }
    init
};
