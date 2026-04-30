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

//! Cache download functionality for animation resources.
//!
//! This module provides functions for downloading, canceling, and configuring
//! cache settings for animation resources. It serves as a bridge between the
//! ETS interface and the native cache download service.

use std::sync::OnceLock;

use ani_rs::business_error::BusinessError;
use ani_rs::objects::AniFnObject;
use ani_rs::AniEnv;
use preload_native_rlib::{CacheDownloadService, DownloadRequest, Downloader, PreloadCallback};
use preload_permission_verify::permission_check;

use crate::bridge::{CacheDownloadOptions, CacheStrategy, DownloadInfo, SslType};
use crate::callback::{self, CallbackManager, CallbackWrapper};

const MAX_FILE_SIZE: i64 = 4294967296;
const MAX_MEM_SIZE: i64 = 1073741824;
const MAX_URL_LENGTH: usize = 8192;
const MAX_INFO_LIST_SIZE: u16 = 8192;

// Retry constants
const DEFAULT_MAX_RETRY_COUNT: i32 = 1;
const MIN_RETRY_COUNT: i32 = 0;
const MAX_RETRY_COUNT: i32 = 10;

// Timeout constants
const DEFAULT_NETWORK_CHECK_TIMEOUT: i32 = 20;
const MIN_NETWORK_CHECK_TIMEOUT: i32 = 0;
const MAX_NETWORK_CHECK_TIMEOUT: i32 = 20;
const DEFAULT_HTTP_TOTAL_TIMEOUT: i32 = 60;
const MIN_HTTP_TOTAL_TIMEOUT: i32 = 1;
const MAX_HTTP_TOTAL_TIMEOUT: i32 = (u32::MAX / 1000) as i32;

static HAS_INTERNET_PERM: OnceLock<bool> = OnceLock::new();

static HAS_GET_NETWORK_INFO_PERM: OnceLock<bool> = OnceLock::new();

fn has_internet_permission() -> bool {
    *HAS_INTERNET_PERM.get_or_init(|| permission_check::CheckInternetPermission())
}

fn has_get_network_info_permission() -> bool {
    *HAS_GET_NETWORK_INFO_PERM.get_or_init(|| permission_check::CheckGetNetworkInfoPermission())
}

/// Initiates a download of a resource with the specified URL and options.
///
/// Creates a new download request, configures it with any provided headers, and
/// submits it to the cache download service for preloading.
///
/// # Parameters
///
/// * `url` - The URL of the resource to download
/// * `options` - Configuration options for the download, including optional
///   headers
///
/// # Returns
///
/// * `Ok(())` if the download was successfully initiated
/// * `Err(BusinessError)` if there was an error initiating the download
///
/// # Examples
///
/// ```rust
/// use ani_cache_download::cache_download::{download, CacheDownloadOptions};
/// use ani_rs::business_error::BusinessError;
///
/// // Basic download
/// let result: Result<(), BusinessError> = download(
///     "https://example.com/resource.mp4".to_string(),
///     CacheDownloadOptions { header: None },
/// );
///
/// // Download with headers
/// let mut headers = std::collections::HashMap::new();
/// headers.insert("Authorization".to_string(), "Bearer token123".to_string());
/// let result: Result<(), BusinessError> = download(
///     "https://example.com/resource.mp4".to_string(),
///     CacheDownloadOptions {
///         header: Some(headers),
///     },
/// );
/// ```
#[ani_rs::native]
pub fn download(url: String, options: CacheDownloadOptions) -> Result<(), BusinessError> {
    if !has_internet_permission() {
        return Err(BusinessError::new(
            201,
            "internet permission denied".to_string(),
        ));
    }
    if url.len() > MAX_URL_LENGTH {
        return Err(BusinessError::new(
            401,
            "url exceeds the maximum length".to_string(),
        ));
    }
    let mut request = DownloadRequest::new(&url);

    let headers = options.headers.unwrap_or_default();
    let headers_vec: Vec<(String, String)> = headers.into_iter().collect();
    let borrowed: Vec<(&str, &str)> = headers_vec
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    if !borrowed.is_empty() {
        request.headers(borrowed);
    }

    if let Some(ssl_type) = options.ssl_type {
        match ssl_type {
            SslType::TLS => request.ssl_type("TLS"),
            SslType::TLCP => request.ssl_type("TLCP"),
        };
    }
    if let Some(ref ca_path) = options.ca_path {
        if !ca_path.is_empty() {
            request.ca_path(ca_path.as_str());
        }
    }

    let is_update = options.cache_strategy != Some(CacheStrategy::LAZY);

    if let Some(ref retry) = options.retry {
        if let Some(max_retry_count) = retry.max_retry_count {
            if max_retry_count >= MIN_RETRY_COUNT && max_retry_count <= MAX_RETRY_COUNT {
                request.max_retry(max_retry_count as usize);
            }
        }
    }

    if let Some(ref timeout) = options.timeout {
        if let Some(network_check_timeout) = timeout.network_check_timeout {
            if network_check_timeout >= MIN_NETWORK_CHECK_TIMEOUT
                && network_check_timeout <= MAX_NETWORK_CHECK_TIMEOUT
            {
                request.network_check_timeout(network_check_timeout as u32);
            }
        }
        if let Some(http_total_timeout) = timeout.http_total_timeout {
            if http_total_timeout >= MIN_HTTP_TOTAL_TIMEOUT
                && http_total_timeout <= MAX_HTTP_TOTAL_TIMEOUT
            {
                request.http_total_timeout(http_total_timeout as u32);
            }
        }
    }

    CacheDownloadService::get_instance().preload(
        request,
        Box::new(CallbackWrapper::new(url.clone())),
        is_update,
        Downloader::Netstack,
    );
    Ok(())
}

/// Cancels a previously initiated download by URL.
///
/// Sends a cancel request to the cache download service for the specified URL.
///
/// # Parameters
///
/// * `url` - The URL of the resource download to cancel
///
/// # Returns
///
/// * `Ok(())` if the cancel request was successfully submitted
/// * `Err(BusinessError)` if there was an error submitting the cancel request
///
/// # Examples
///
/// ```rust
/// use ani_cache_download::cache_download::cancel;
/// use ani_rs::business_error::BusinessError;
///
/// // Cancel a download
/// let result: Result<(), BusinessError> = cancel("https://example.com/resource.mp4".to_string());
/// ```
#[ani_rs::native]
pub fn cancel(url: String) -> Result<(), BusinessError> {
    if (url.len() > MAX_URL_LENGTH as usize) {
        return Err(BusinessError::new(
            401,
            "url exceeds the maximum length".to_string(),
        ));
    }
    CacheDownloadService::get_instance().cancel(&url);
    Ok(())
}

/// Sets the maximum memory (RAM) cache size in bytes.
///
/// Configures the RAM cache size for the cache download service.
///
/// # Parameters
///
/// * `size` - The maximum size of the memory cache in bytes
///
/// # Returns
///
/// * `Ok(())` if the cache size was successfully updated
/// * `Err(BusinessError)` if there was an error updating the cache size
///
/// # Examples
///
/// ```rust
/// use ani_cache_download::cache_download::set_memory_cache_size;
/// use ani_rs::business_error::BusinessError;
///
/// // Set memory cache size to 50MB
/// let result: Result<(), BusinessError> = set_memory_cache_size(50 * 1024 * 1024);
/// ```
#[ani_rs::native]
pub fn set_memory_cache_size(size: i64) -> Result<(), BusinessError> {
    if (size > MAX_MEM_SIZE) {
        return Err(BusinessError::new(
            401,
            "memory cache size exceeds the maximum value".to_string(),
        ));
    }
    // Convert signed i64 to unsigned u64 for cache size
    CacheDownloadService::get_instance().set_ram_cache_size(size as u64);
    Ok(())
}

/// Sets the maximum file cache size in bytes.
///
/// Configures the file system cache size for the cache download service.
///
/// # Parameters
///
/// * `size` - The maximum size of the file cache in bytes
///
/// # Returns
///
/// * `Ok(())` if the cache size was successfully updated
/// * `Err(BusinessError)` if there was an error updating the cache size
///
/// # Examples
///
/// ```rust
/// use ani_cache_download::cache_download::set_file_cache_size;
/// use ani_rs::business_error::BusinessError;
///
/// // Set file cache size to 500MB
/// let result: Result<(), BusinessError> = set_file_cache_size(500 * 1024 * 1024);
/// ```
#[ani_rs::native]
pub fn set_file_cache_size(size: i64) -> Result<(), BusinessError> {
    if (size > MAX_FILE_SIZE) {
        return Err(BusinessError::new(
            401,
            "file cache size exceeds the maximum value".to_string(),
        ));
    }
    // Convert signed i64 to unsigned u64 for cache size
    CacheDownloadService::get_instance().set_file_cache_size(size as u64);
    Ok(())
}

#[ani_rs::native]
pub fn get_download_info(url: String) -> Result<Option<DownloadInfo>, BusinessError> {
    if !has_get_network_info_permission() {
        return Err(BusinessError::new(
            201,
            "GET_NETWORK_INFO permission denied".to_string(),
        ));
    }
    if (url.len() > MAX_URL_LENGTH as usize) {
        return Err(BusinessError::new(
            401,
            "url exceeds the maximum length".to_string(),
        ));
    }
    let info = CacheDownloadService::get_instance()
        .get_download_info(&url)
        .map(|info| {
            DownloadInfo::from_native(
                preload_native_rlib::info::RustDownloadInfo::from_download_info(info),
            )
        });
    Ok(info)
}

#[ani_rs::native]
pub fn set_download_info_list_size(size: i64) -> Result<(), BusinessError> {
    if (size > MAX_INFO_LIST_SIZE as i64) {
        return Err(BusinessError::new(
            401,
            "info list size exceeds the maximum value".to_string(),
        ));
    }
    if (size < 0) {
        return Err(BusinessError::new(
            401,
            "info list size is negative".to_string(),
        ));
    }
    CacheDownloadService::get_instance().set_info_list_size(size as u16);
    Ok(())
}

/// Clears all entries from the memory (RAM) cache.
///
/// Removes all cached resources from the in-memory cache.
///
/// # Examples
///
/// ```rust
/// use ani_cache_download::cache_download::clear_memory_cache;
/// use ani_rs::business_error::BusinessError;
///
/// // Clear memory cache
/// let result: Result<(), BusinessError> = clear_memory_cache();
/// ```
#[ani_rs::native]
pub fn clear_memory_cache() -> Result<(), BusinessError> {
    CacheDownloadService::get_instance().clear_memory_cache();
    Ok(())
}

/// Clears all entries from the file system cache.
///
/// Removes all cached resources from the file system cache.
///
/// # Examples
///
/// ```rust
/// use ani_cache_download::cache_download::clear_file_cache;
/// use ani_rs::business_error::BusinessError;
///
/// // Clear file cache
/// let result: Result<(), BusinessError> = clear_file_cache();
/// ```
#[ani_rs::native]
pub fn clear_file_cache() -> Result<(), BusinessError> {
    CacheDownloadService::get_instance().clear_file_cache();
    Ok(())
}

#[ani_rs::native]
pub fn on_download_success(
    env: &AniEnv,
    url: String,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    check_url_length(url.as_str())?;
    let callback = callback.into_global_callback(env)?;
    CallbackManager::get_instance().register_success_callback(&url.as_str(), callback);
    Ok(())
}

#[ani_rs::native]
pub fn on_download_error(
    env: &AniEnv,
    url: String,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    check_url_length(url.as_str())?;
    let callback = callback.into_global_callback(env)?;
    CallbackManager::get_instance().register_error_callback(&url.as_str(), callback);
    Ok(())
}

#[ani_rs::native]
pub fn off_download_success(
    env: &AniEnv,
    url: String,
    success_callback: Option<AniFnObject>,
) -> Result<(), BusinessError> {
    check_url_length(url.as_str())?;
    let callback = success_callback
        .map(|cb| cb.into_global_callback(env))
        .transpose()?;
    CallbackManager::get_instance().unregister_success_callback(&url.as_str(), callback);
    Ok(())
}

#[ani_rs::native]
pub fn off_download_error(
    env: &AniEnv,
    url: String,
    err_callback: Option<AniFnObject>,
) -> Result<(), BusinessError> {
    check_url_length(url.as_str())?;
    let callback = err_callback
        .map(|cb| cb.into_global_callback(env))
        .transpose()?;
    CallbackManager::get_instance().unregister_error_callback(&url.as_str(), callback);
    Ok(())
}

fn check_url_length(url: &str) -> Result<(), BusinessError> {
    let url_len = url.len();
    if url_len == 0 {
        return Err(BusinessError::new(401, "url is empty".to_string()));
    }
    if url_len > MAX_URL_LENGTH {
        return Err(BusinessError::new(
            401,
            "url exceeds the maximum length".to_string(),
        ));
    }
    Ok(())
}

#[ani_rs::native]
pub fn set_global_retry_options(
    options: Option<crate::bridge::RetryOptions>,
) -> Result<(), BusinessError> {
    // 参数缺失时使用默认值
    let max_retry_count = match options {
        None => DEFAULT_MAX_RETRY_COUNT,
        Some(o) => match o.max_retry_count {
            None => DEFAULT_MAX_RETRY_COUNT,
            Some(v) => {
                // 参数范围校验
                if v < MIN_RETRY_COUNT || v > MAX_RETRY_COUNT {
                    return Err(BusinessError::new(
                        401,
                        "maxRetryCount out of range [0, 10]".to_string(),
                    ));
                }
                v
            }
        },
    };
    CacheDownloadService::get_instance().set_global_retry_options(max_retry_count as usize);
    Ok(())
}

#[ani_rs::native]
pub fn set_global_timeout_options(
    options: Option<crate::bridge::TimeoutOptions>,
) -> Result<(), BusinessError> {
    // 参数缺失时使用默认值
    let network_check_timeout = match options {
        None => DEFAULT_NETWORK_CHECK_TIMEOUT,
        Some(ref o) => match o.network_check_timeout {
            None => DEFAULT_NETWORK_CHECK_TIMEOUT,
            Some(v) => {
                // 参数范围校验
                if v < MIN_NETWORK_CHECK_TIMEOUT || v > MAX_NETWORK_CHECK_TIMEOUT {
                    return Err(BusinessError::new(
                        401,
                        "networkCheckTimeout out of range [0, 20]".to_string(),
                    ));
                }
                v
            }
        },
    };
    let http_total_timeout = match options {
        None => DEFAULT_HTTP_TOTAL_TIMEOUT,
        Some(ref o) => match o.http_total_timeout {
            None => DEFAULT_HTTP_TOTAL_TIMEOUT,
            Some(v) => {
                if v < MIN_HTTP_TOTAL_TIMEOUT || v > MAX_HTTP_TOTAL_TIMEOUT {
                    return Err(BusinessError::new(
                        401,
                        "httpTotalTimeout out of range [1, u32::MAX/1000]".to_string(),
                    ));
                }
                v
            }
        },
    };
    CacheDownloadService::get_instance()
        .set_global_timeout_options(network_check_timeout as u32, http_total_timeout as u32);
    Ok(())
}
