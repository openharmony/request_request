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

use ani_rs::business_error::BusinessError;
use preload_native::services::{CacheDownloadService, DownloadRequest};

use crate::bridge::CacheDownloadOptions;

struct Callback;

impl PreloadCallback for Callback {}

#[ani_rs::native]
fn download(url: String, options: CacheDownloadOptions) -> Result<(), BusinessError> {
    let mut request = DownloadRequest::new(&url);
    let callback = Box::new(Callback);
    if let Some(headers) = options.header {
        request.headers(headers);
    }
    CacheDownloadService::get_instance().preload(
        request,
        callback,
        true,
        cache_download::Downloader::Netstack,
    );
    Ok(())
}

#[ani_rs::native]
fn cancel(url: String) -> Result<(), BusinessError> {
    CacheDownloadService::get_instance().cancel(&url);
    Ok(())
}

#[ani_rs::native]
fn set_memory_cache_size(size: i64) -> Result<(), BusinessError> {
    CacheDownloadService::get_instance().set_ram_cache_size(size as u64);
    Ok(())
}

#[ani_rs::native]
fn set_file_cache_size(size: i64) -> Result<(), BusinessError> {
    CacheDownloadService::get_instance().set_file_cache_size(size as u64);
    Ok(())
}
