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

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use cache_core::{CacheManager, Updater};
use request_utils::task_id::TaskId;

use super::common::{CommonError, CommonResponse};
use super::{CacheDownloadError, RUNNING};
use crate::download::{CANCEL, FAIL, SUCCESS};
use crate::services::{CacheDownloadService, PreloadCallback};

const PROGRESS_INTERVAL: usize = 8;

pub(crate) struct PrimeCallback {
    task_id: TaskId,
    finish: Arc<AtomicBool>,
    state: Arc<AtomicUsize>,
    cache_handle: Updater,
    callbacks: Arc<Mutex<VecDeque<Box<dyn PreloadCallback>>>>,
    progress_restriction: ProgressRestriction,
    seq: usize,
}

struct ProgressRestriction {
    processed: u64,
    count: usize,
    data_receive: bool,
}

impl ProgressRestriction {
    fn new() -> Self {
        Self {
            processed: 0,
            count: 0,
            data_receive: false,
        }
    }
}

impl PrimeCallback {
    pub(crate) fn new(
        task_id: TaskId,
        cache_manager: &'static CacheManager,
        finish: Arc<AtomicBool>,
        state: Arc<AtomicUsize>,
        callbacks: Arc<Mutex<VecDeque<Box<dyn PreloadCallback>>>>,
        seq: usize,
    ) -> Self {
        Self {
            task_id: task_id.clone(),
            finish,
            state,
            cache_handle: Updater::new(task_id, cache_manager),
            callbacks,
            progress_restriction: ProgressRestriction::new(),
            seq,
        }
    }

    pub(crate) fn set_running(&self) {
        self.state.store(RUNNING, Ordering::Release);
    }

    pub(crate) fn task_id(&self) -> TaskId {
        self.task_id.clone()
    }
}

impl PrimeCallback {
    pub(crate) fn common_success<R>(&mut self, response: R)
    where
        R: CommonResponse,
    {
        let code = response.code();
        info!("{} status code {}", self.task_id.brief(), code);

        let cache = self.cache_handle.cache_finish();
        self.state.store(SUCCESS, Ordering::Release);
        self.finish.store(true, Ordering::Release);
        let mut callbacks = self.callbacks.lock().unwrap();

        while let Some(mut callback) = callbacks.pop_front() {
            let clone_cache = cache.clone();
            let task_id = self.task_id.brief().to_string();
            crate::spawn(move || {
                callback.on_progress(clone_cache.size() as u64, clone_cache.size() as u64);
                callback.on_success(clone_cache, &task_id)
            });
        }
        drop(callbacks);
        self.notify_agent_finish();
    }

    pub(crate) fn common_fail<E>(&mut self, error: E)
    where
        E: CommonError,
    {
        info!("{} download failed {}", self.task_id.brief(), error.code());
        self.state.store(FAIL, Ordering::Release);
        self.finish.store(true, Ordering::Release);
        let mut callbacks = self.callbacks.lock().unwrap();

        while let Some(mut callback) = callbacks.pop_front() {
            let task_id = self.task_id.brief().to_string();
            let error = CacheDownloadError::from(&error);
            crate::spawn(move || callback.on_fail(error, &task_id));
        }
        drop(callbacks);
        self.notify_agent_finish();
    }

    pub(crate) fn common_cancel(&mut self) {
        info!("{} is cancel", self.task_id.brief());
        self.state.store(CANCEL, Ordering::Release);

        self.finish.store(true, Ordering::Release);
        let mut callbacks = self.callbacks.lock().unwrap();

        while let Some(mut callback) = callbacks.pop_front() {
            crate::spawn(move || callback.on_cancel());
        }
        drop(callbacks);
        self.notify_agent_finish();
    }

    pub(crate) fn common_progress(
        &mut self,
        dl_total: u64,
        dl_now: u64,
        _ul_total: u64,
        _ul_now: u64,
    ) {
        if !self.progress_restriction.data_receive
            || dl_now == self.progress_restriction.processed
            || dl_now == dl_total
        {
            return;
        }
        self.progress_restriction.processed = dl_now;

        let count = self.progress_restriction.count;
        self.progress_restriction.count += 1;
        if count % PROGRESS_INTERVAL != 0 {
            return;
        }
        self.progress_restriction.count = 1;

        let mut callbacks = self.callbacks.lock().unwrap();
        for callback in callbacks.iter_mut() {
            callback.on_progress(dl_now, dl_total);
        }
    }

    pub(crate) fn common_data_receive<F>(&mut self, data: &[u8], content_length: F)
    where
        F: FnOnce() -> Option<usize>,
    {
        self.progress_restriction.data_receive = true;
        self.cache_handle.cache_receive(data, content_length);
    }

    #[cfg(feature = "netstack")]
    pub(crate) fn common_restart(&mut self) {
        self.cache_handle.reset_cache();
    }

    fn notify_agent_finish(&self) {
        CacheDownloadService::get_instance().task_finish(&self.task_id, self.seq);
    }
}
