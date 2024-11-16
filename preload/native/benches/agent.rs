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

mod utils;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::LazyLock;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use preload_native::{CustomCallback, DownloadAgent, DownloadRequest};
use utils::{init, test_server};

struct Callback;

impl CustomCallback for Callback {}

fn agent_preload(url: &str) {
    let agent = DownloadAgent::get_instance();
    let request = DownloadRequest::new(&url);
    let callback = Box::new(Callback);
    agent.preload(request, callback, false);
}

fn preload_benchmark_different_url(c: &mut Criterion) {
    static SERVER: LazyLock<Vec<String>> = LazyLock::new(|| {
        let mut v = vec![];
        for _ in 0..1000 {
            v.push(test_server(|_| {}));
        }
        v
    });
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

fn preload_benchmark_same_url(c: &mut Criterion) {
    static SERVER: LazyLock<String> = LazyLock::new(|| test_server(|_| {}));
    init();
    c.bench_function("preload", |b| {
        b.iter(|| agent_preload(&SERVER));
    });
}

fn config() -> Criterion {
    Criterion::default().sample_size(1000)
}

criterion_group! {name = agent; config = config();targets =  preload_benchmark_same_url}
criterion_main!(agent);
