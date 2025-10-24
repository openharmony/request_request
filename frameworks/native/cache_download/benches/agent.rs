// Copyright (C) 2024 Huawei Device Co., Ltd.
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

//! Benchmark tests for the cache download agent functionality.
//! 
//! This module contains performance benchmarks for the `CacheDownloadService`,
//! measuring preload operations with both identical and different URLs to evaluate
//! caching efficiency and performance characteristics.

mod utils;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::LazyLock;

use cache_download::services::{CacheDownloadService, DownloadRequest, PreloadCallback};
use cache_download::Downloader;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use utils::{init, test_server};

/// Simple implementation of the `PreloadCallback` trait for benchmarking.
///
/// This implementation provides an empty callback for preload operations
/// to allow benchmarking without additional processing overhead.
struct Callback;

/// Implements the required preload callback functionality for benchmark testing.
impl PreloadCallback for Callback {}

/// Performs a preload operation using the cache download service.
///
/// Creates a download request for the given URL and submits it to the cache download
/// service with a callback. Uses the Ylong downloader implementation.
///
/// # Parameters
/// - `url`: URL of the resource to preload
fn agent_preload(url: &str) {
    let agent = CacheDownloadService::get_instance();
    let request = DownloadRequest::new(&url);
    let callback = Box::new(Callback);
    agent.preload(request, callback, false, Downloader::Ylong);
}

#[allow(unused)]
/// Benchmarks preload performance with different URLs.
///
/// Creates multiple test servers and benchmarks preload operations with unique URLs
/// for each iteration. This measures the performance when handling cache misses
/// or new resources.
///
/// # Parameters
/// - `c`: Criterion benchmark context
fn preload_benchmark_different_url(c: &mut Criterion) {
    // Lazy initialization of test servers to avoid setup overhead during benchmarking
    static SERVER: LazyLock<Vec<String>> = LazyLock::new(|| {
        let mut v = vec![];
        for _ in 0..1000 {
            v.push(test_server(|_| {}));
        }
        v
    });
    // Atomic counter for generating unique URLs in each iteration
    static A: AtomicUsize = AtomicUsize::new(0);
    init();

    c.bench_function("preload", |b| {
        b.iter(|| {
            let a = black_box(A.fetch_add(1, Ordering::SeqCst));
            let server = SERVER[a % 1000].clone();
            let url = format!("{}/{}", server, a);
            agent_preload(&url)
        });
    });
}

/// Benchmarks preload performance with the same URL.
///
/// Uses a single test server and benchmarks preload operations with the same URL
/// for each iteration. This measures the performance when handling cache hits
/// for previously accessed resources.
///
/// # Parameters
/// - `c`: Criterion benchmark context
fn preload_benchmark_same_url(c: &mut Criterion) {
    // Lazy initialization of a single test server
    static SERVER: LazyLock<String> = LazyLock::new(|| test_server(|_| {}));
    init();
    c.bench_function("preload", |b| {
        b.iter(|| agent_preload(&SERVER));
    });
}

/// Configures the benchmark settings.
///
/// Sets up Criterion with a sample size of 1000 iterations to ensure statistically
/// significant results for the benchmarks.
///
/// # Returns
/// Configured Criterion instance
fn config() -> Criterion {
    Criterion::default().sample_size(1000)
}

// Define the benchmark group with the configured settings
criterion_group! {name = agent; config = config();targets =  preload_benchmark_same_url}

// Main entry point for the benchmark
criterion_main!(agent);
