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

mod bridge;
mod cache_download;
ani_rs::ani_constructor! {
    namespace "L@ohos/request/cacheDownload/cacheDownload"
    [
        "download" : cache_download::download,
        "cancel" : cache_download::cancel,
        "setMemoryCacheSize" : cache_download::set_memory_cache_size,
        "setFileCacheSize" : cache_download::set_file_cache_size,
    ]
}
