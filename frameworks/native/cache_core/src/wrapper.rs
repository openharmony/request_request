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

use crate::data::observer::DirRebuilder;

#[cxx::bridge(namespace = "OHOS::Request")]
pub(crate) mod ffi {
    struct FfiPredownloadOptions<'a> {
        headers: Vec<&'a str>,
    }

    extern "Rust" {
        type DirRebuilder;

        fn recreate_store_dir(self: &DirRebuilder);
        fn history_exist_or_create(self: &DirRebuilder) -> bool;
    }

    unsafe extern "C++" {
        include!("inotify_event_listener.h");
        include!("native_ffi.h");
        type DirectoryMonitor;

        fn NewDirectoryMonitor(
            target: &CxxString,
            callback: Box<DirRebuilder>,
        ) -> UniquePtr<DirectoryMonitor>;
        fn StartObserve(monitor: Pin<&mut DirectoryMonitor>);
    }
}
