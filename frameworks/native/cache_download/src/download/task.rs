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

use cache_core::CacheManager;
use netstack_rs::info::DownloadInfoMgr;
use request_utils::info;
use request_utils::task_id::TaskId;

use super::callback::PrimeCallback;
use super::common::CommonHandle;
use super::{INIT, SUCCESS};

cfg_ylong! {
    use crate::download::ylong;
}

cfg_netstack! {
    use crate::download::netstack;
}

use crate::services::{DownloadRequest, PreloadCallback};

pub enum Downloader {
    Netstack,
    Ylong,
}

pub(crate) struct DownloadTask {
    pub(crate) remove_flag: bool,
    pub(crate) seq: usize,
    pub(crate) handle: TaskHandle,
}

impl DownloadTask {
    pub(crate) fn new(
        task_id: TaskId,
        cache_manager: &'static CacheManager,
        info_mgr: Arc<DownloadInfoMgr>,
        request: DownloadRequest,
        callback: Box<dyn PreloadCallback>,
        downloader: Downloader,
        seq: usize,
    ) -> Option<DownloadTask> {
        info!("new preload task {} seq {}", task_id.brief(), seq);
        let mut handle = None;
        match downloader {
            Downloader::Netstack => {
                #[cfg(feature = "netstack")]
                {
                    handle = download_inner(
                        task_id,
                        cache_manager,
                        info_mgr,
                        request,
                        Some(callback),
                        netstack::DownloadTask::run,
                        seq,
                    );
                }
            }
            Downloader::Ylong => {
                #[cfg(feature = "ylong")]
                {
                    handle = Some(download_inner(
                        task_id,
                        cache_manager,
                        request,
                        Some(callback),
                        ylong::DownloadTask::run,
                        seq,
                    ));
                }
            }
        };
        handle.map(|handle| DownloadTask {
            remove_flag: false,
            seq,
            handle,
        })
    }

    pub(crate) fn cancel(&mut self) {
        self.handle.cancel();
    }

    pub(crate) fn task_handle(&self) -> TaskHandle {
        self.handle.clone()
    }

    pub(crate) fn try_add_callback(
        &mut self,
        callback: Box<dyn PreloadCallback>,
    ) -> Result<(), Box<dyn PreloadCallback>> {
        self.handle.try_add_callback(callback)
    }
}

#[derive(Clone)]
pub struct TaskHandle {
    task_id: TaskId,
    handle: Option<Arc<dyn CommonHandle>>,
    state: Arc<AtomicUsize>,
    finish: Arc<AtomicBool>,
    callbacks: Arc<Mutex<VecDeque<Box<dyn PreloadCallback>>>>,
}

impl TaskHandle {
    pub(crate) fn new(task_id: TaskId) -> Self {
        Self {
            state: Arc::new(AtomicUsize::new(INIT)),
            task_id,
            handle: None,
            finish: Arc::new(AtomicBool::new(false)),
            callbacks: Arc::new(Mutex::new(VecDeque::with_capacity(1))),
        }
    }
    pub(crate) fn cancel(&mut self) {
        if let Some(handle) = self.handle.take() {
            info!("cancel task {}", self.task_id.brief());
            if self.finish.load(Ordering::Acquire) {
                return;
            }
            let _callback = self.callbacks.lock().unwrap();
            if self.finish.load(Ordering::Acquire) {
                return;
            }
            if handle.cancel() {
                self.finish.store(true, Ordering::Release);
            }
        } else {
            error!("cancel task {} not exist", self.task_id.brief());
        }
    }

    pub(crate) fn reset(&mut self) {
        if self.finish.load(Ordering::Acquire) {
            return;
        }
        if let Some(handle) = self.handle.as_ref() {
            handle.reset();
        }
    }

    pub fn task_id(&self) -> String {
        self.task_id.to_string()
    }

    pub fn is_finish(&self) -> bool {
        self.finish.load(Ordering::Acquire)
    }

    pub fn state(&self) -> usize {
        self.state.load(Ordering::Acquire)
    }

    pub(crate) fn set_completed(&self) {
        self.state.store(SUCCESS, Ordering::Relaxed);
        self.finish.store(true, Ordering::Relaxed);
    }

    pub(crate) fn try_add_callback(
        &mut self,
        callback: Box<dyn PreloadCallback>,
    ) -> Result<(), Box<dyn PreloadCallback>> {
        let mut callbacks = self.callbacks.lock().unwrap();
        if !self.finish.load(Ordering::Acquire) {
            info!("add callback to task {}", self.task_id.brief());
            callbacks.push_back(callback);
            if let Some(handle) = self.handle.as_ref() {
                handle.add_count();
            }
            Ok(())
        } else {
            Err(callback)
        }
    }

    #[inline]
    fn state_flag(&self) -> Arc<AtomicUsize> {
        self.state.clone()
    }

    #[inline]
    fn finish_flag(&self) -> Arc<AtomicBool> {
        self.finish.clone()
    }

    #[inline]
    fn callbacks(&self) -> Arc<Mutex<VecDeque<Box<dyn PreloadCallback>>>> {
        self.callbacks.clone()
    }

    #[inline]
    fn set_handle(&mut self, handle: Arc<dyn CommonHandle>) {
        self.handle = Some(handle);
    }
}

fn download_inner<F>(
    task_id: TaskId,
    cache_manager: &'static CacheManager,
    info_mgr: Arc<DownloadInfoMgr>,
    request: DownloadRequest,
    callback: Option<Box<dyn PreloadCallback>>,
    downloader: F,
    seq: usize,
) -> Option<TaskHandle>
where
    F: Fn(DownloadRequest, PrimeCallback, Arc<DownloadInfoMgr>) -> Option<Arc<dyn CommonHandle>>,
{
    let mut handle = TaskHandle::new(task_id.clone());
    if let Some(callback) = callback {
        handle.callbacks.lock().unwrap().push_back(callback);
    }

    let callback = PrimeCallback::new(
        task_id,
        cache_manager,
        handle.finish_flag(),
        handle.state_flag(),
        handle.callbacks(),
        seq,
    );
    downloader(request, callback, info_mgr).map(move |command| {
        handle.set_handle(command);
        handle
    })
}

#[cfg(test)]
mod ut_task {
    include!("../../tests/ut/download/ut_task.rs");
}
