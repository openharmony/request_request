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

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, Once, OnceLock};

use cache_core::{CacheManager, RamCache};
use netstack_rs::info::{DownloadInfo, DownloadInfoMgr};
use request_utils::observe::network::NetRegistrar;
use request_utils::task_id::TaskId;

use crate::download::task::{DownloadTask, Downloader, TaskHandle};
use crate::download::CacheDownloadError;
use crate::observe::NetObserver;

#[allow(unused_variables)]
pub trait PreloadCallback: Send {
    fn on_success(&mut self, data: Arc<RamCache>, task_id: &str) {}
    fn on_fail(&mut self, error: CacheDownloadError, task_id: &str) {}
    fn on_cancel(&mut self) {}
    fn on_progress(&mut self, progress: u64, total: u64) {}
}

pub struct CacheDownloadService {
    running_tasks: Mutex<HashMap<TaskId, Arc<Mutex<DownloadTask>>>>,
    cache_manager: CacheManager,
    info_mgr: Arc<DownloadInfoMgr>,
    net_registrar: NetRegistrar,
}

pub struct DownloadRequest<'a> {
    pub url: &'a str,
    pub headers: Option<Vec<(&'a str, &'a str)>>,
}

impl<'a> DownloadRequest<'a> {
    pub fn new(url: &'a str) -> Self {
        Self { url, headers: None }
    }

    pub fn headers(&mut self, headers: Vec<(&'a str, &'a str)>) -> &mut Self {
        self.headers = Some(headers);
        self
    }
}

impl CacheDownloadService {
    fn new() -> Self {
        Self {
            running_tasks: Mutex::new(HashMap::new()),
            cache_manager: CacheManager::new(),
            info_mgr: Arc::new(DownloadInfoMgr::new()),
            net_registrar: NetRegistrar::new(),
        }
    }

    pub fn get_instance() -> &'static Self {
        static DOWNLOAD_AGENT: OnceLock<CacheDownloadService> = OnceLock::new();
        static ONCE: Once = Once::new();
        let cache_download = DOWNLOAD_AGENT.get_or_init(CacheDownloadService::new);

        ONCE.call_once(|| {
            let old_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(move |info| {
                error!("Panic occurred {:?}", info);
                old_hook(info);
            }));
            cache_download.cache_manager.restore_files();
            cache_download.net_registrar.add_observer(NetObserver);
            if let Err(e) = cache_download.net_registrar.register() {
                error!("Failed to register network observer: {:?}", e);
            }
        });

        cache_download
    }

    pub fn cancel(&self, url: &str) {
        let task_id = TaskId::from_url(url);
        if let Some(updater) = self.running_tasks.lock().unwrap().get(&task_id).cloned() {
            updater.lock().unwrap().cancel();
        }
    }

    pub(crate) fn reset_all_tasks(&self) {
        let running_tasks = self.running_tasks.lock().unwrap();
        for task in running_tasks.values() {
            task.lock().unwrap().handle.reset();
        }
    }

    pub fn remove(&self, url: &str) {
        let task_id = TaskId::from_url(url);
        self.cache_manager.remove(task_id);
    }

    pub fn contains(&self, url: &str) -> bool {
        let task_id = TaskId::from_url(url);
        self.cache_manager.contains(&task_id)
    }

    pub fn preload(
        &'static self,
        request: DownloadRequest,
        mut callback: Box<dyn PreloadCallback>,
        update: bool,
        downloader: Downloader,
    ) -> Option<TaskHandle> {
        let url = request.url;
        let task_id = TaskId::from_url(url);
        info!("preload task {}", task_id.brief());

        if !update {
            if let Err(ret) = self.fetch_with_callback(&task_id, callback) {
                callback = ret;
            } else {
                info!("{} fetch success", task_id.brief());
                let handle = TaskHandle::new(task_id);
                handle.set_completed();
                return Some(handle);
            }
        }

        loop {
            let updater = match self.running_tasks.lock().unwrap().entry(task_id.clone()) {
                Entry::Occupied(entry) => entry.get().clone(),
                Entry::Vacant(entry) => {
                    let download_task = DownloadTask::new(
                        task_id.clone(),
                        &self.cache_manager,
                        self.info_mgr.clone(),
                        request,
                        callback,
                        downloader,
                        0,
                    );
                    match download_task {
                        Some(task) => {
                            let updater = Arc::new(Mutex::new(task));
                            let handle = updater.lock().unwrap().task_handle();
                            entry.insert(updater);
                            return Some(handle);
                        }
                        None => return None,
                    }
                }
            };

            let mut updater = updater.lock().unwrap();
            match updater.try_add_callback(callback) {
                Ok(()) => return Some(updater.task_handle()),
                Err(mut cb) => {
                    if update {
                        info!("add callback failed, update task {}", task_id.brief());
                    } else if let Err(callback) = self.fetch_with_callback(&task_id, cb) {
                        error!("{} fetch fail after update", task_id.brief());
                        cb = callback;
                    } else {
                        info!("{} fetch success", task_id.brief());
                        let handle = TaskHandle::new(task_id);
                        handle.set_completed();
                        return Some(handle);
                    }

                    if !updater.remove_flag {
                        let seq = updater.seq + 1;
                        let download_task = DownloadTask::new(
                            task_id.clone(),
                            &self.cache_manager,
                            self.info_mgr.clone(),
                            request,
                            cb,
                            downloader,
                            seq,
                        );
                        match download_task {
                            Some(task) => {
                                *updater = task;
                                return Some(updater.task_handle());
                            }
                            None => return None,
                        }
                    } else {
                        callback = cb;
                    }
                }
            };
        }
    }

    pub fn fetch(&'static self, url: &str) -> Option<Arc<RamCache>> {
        let task_id = TaskId::from_url(url);
        self.cache_manager.fetch(&task_id)
    }

    pub(crate) fn task_finish(&self, task_id: &TaskId, seq: usize) {
        let Some(updater) = self.running_tasks.lock().unwrap().get(task_id).cloned() else {
            return;
        };
        let mut updater = updater.lock().unwrap();
        if updater.seq == seq {
            updater.remove_flag = true;
            self.running_tasks.lock().unwrap().remove(task_id);
        }
    }

    pub fn set_file_cache_size(&self, size: u64) {
        info!("set file cache size to {}", size);
        self.cache_manager.set_file_cache_size(size);
    }

    pub fn set_ram_cache_size(&self, size: u64) {
        info!("set ram cache size to {}", size);
        self.cache_manager.set_ram_cache_size(size);
    }

    pub fn set_info_list_size(&self, size: u16) {
        self.info_mgr.update_info_list_size(size);
    }

    pub fn get_download_info(&self, url: &str) -> Option<DownloadInfo> {
        let task_id = TaskId::from_url(url);
        self.info_mgr.get_download_info(task_id)
    }

    fn fetch_with_callback(
        &'static self,
        task_id: &TaskId,
        mut callback: Box<dyn PreloadCallback>,
    ) -> Result<(), Box<dyn PreloadCallback>> {
        let task_id = task_id.clone();
        if let Some(cache) = self.cache_manager.fetch(&task_id) {
            crate::spawn(move || callback.on_success(cache, task_id.brief()));
            Ok(())
        } else {
            Err(callback)
        }
    }
}

#[cfg(test)]
mod ut_services {
    include!("../tests/ut/ut_services.rs");
}
