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

use std::io::Write;

use cxx::UniquePtr;
use netstack_rs::error::HttpClientError;
use netstack_rs::request::{Request, RequestCallback};
use netstack_rs::response::Response;
use netstack_rs::task::RequestTask;

use super::wrapper::ffi::PreDownloadTaskCallback;
use super::wrapper::UserCallback;
use crate::cache::{Cache, CacheManager};

pub(super) struct PreDownloadCallback {
    cache: Option<Cache>,
    user_callback: Option<UserCallback>,
}

impl RequestCallback for PreDownloadCallback {
    fn on_success(&mut self, response: Response) {
        if let Some(ref callback) = self.user_callback {
            callback.on_success();
        }
        self.cache.as_mut().unwrap().update_cache_size();
        CacheManager::get_instance().update_cache(
            "http://192.168.0.101/aaa.png".to_string(),
            self.cache.take().unwrap(),
        );
    }

    fn on_fail(&mut self, response: Response, error: HttpClientError) {
        if let Some(ref callback) = self.user_callback {
            callback.on_fail();
        }
    }

    fn on_cancel(&mut self, response: Response) {
        if let Some(ref callback) = self.user_callback {
            callback.on_cancel();
        }
    }

    fn on_data_receive(&mut self, data: &[u8]) {
        if self.cache.is_none() {
            self.cache = Some(CacheManager::get_instance().apply_for_cache(None));
        }
        let cache = self.cache.as_mut().unwrap();
        cache.write_all(data);
    }
    fn on_progress(&mut self, dl_total: u64, dl_now: u64, ul_total: u64, ul_now: u64) {}
}

pub(super) struct DownloadTask;

impl DownloadTask {
    pub(super) fn run(
        mut request: Request<PreDownloadCallback>,
        user_callback: Option<UserCallback>,
    ) -> TaskHandle {
        request.callback(PreDownloadCallback {
            cache: None,
            user_callback,
        });
        let mut task = request.build();
        task.start();
        TaskHandle { inner: task }
    }
}

pub struct TaskHandle {
    inner: RequestTask,
}

impl TaskHandle {
    pub(crate) fn cancel(&mut self) {
        self.inner.cancel();
    }
}

#[cfg(test)]
mod test {
    use std::io::{Read, Seek};
    use std::time::Duration;

    use netstack_rs::request::Request;

    use super::{DownloadTask, PreDownloadCallback};
    use crate::cache::CacheManager;

    const TEST_URL: &str = "http://192.168.0.101/aaa.png";
    const FILE_LEN: usize = 9561;
    #[test]
    fn ut_predownload() {
        let mut request = Request::new();
        request.url(TEST_URL);
        let task = DownloadTask::run(request, None);

        std::thread::sleep(Duration::from_secs(10));
        let cache = CacheManager::get_instance()
            .get_cache(TEST_URL.to_string())
            .unwrap();

        let mut buf = vec![];
        let size = cache.reader().read_to_end(&mut buf).unwrap();
        assert_eq!(buf.len(), FILE_LEN);
    }
}
