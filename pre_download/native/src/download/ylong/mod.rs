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

mod client;

use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};

use client::client;
use ylong_http_client::async_impl::{
    Body, DownloadOperator, Downloader, PercentEncoder, RequestBuilder,
};
use ylong_http_client::{ErrorKind, HttpClientError, StatusCode};

use super::common::DownloadCallback;

struct Operator<'a> {
    callback: &'a mut DownloadCallback,
    abort_flag: Arc<AtomicBool>,
    headers: String,
}

impl<'a> DownloadOperator for Operator<'a> {
    fn poll_download(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        data: &[u8],
    ) -> Poll<Result<usize, HttpClientError>> {
        let me = self.get_mut();
        me.callback.on_data_receive(
            data,
            RequestTask {
                headers: me.headers.clone(),
            },
        );
        Poll::Ready(Ok(data.len()))
    }

    fn poll_progress(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        downloaded: u64,
        total: Option<u64>,
    ) -> Poll<Result<(), HttpClientError>> {
        let me = self.get_mut();
        me.callback
            .on_progress(total.unwrap_or_default(), downloaded, 0, 0);
        if me.abort_flag.load(Ordering::Acquire) {
            Poll::Ready(Err(HttpClientError::user_aborted()))
        } else {
            Poll::Ready(Ok(()))
        }
    }
}

pub struct RequestTask {
    headers: String,
}

impl RequestTask {
    pub(crate) fn headers(&self) -> String {
        self.headers.clone()
    }
}

pub struct DownloadTask;

impl DownloadTask {
    pub(crate) fn run(url: &str, mut callback: DownloadCallback) -> CancelHandle {
        let url = match PercentEncoder::encode(url) {
            Ok(url) => url,
            Err(e) => {
                callback.on_fail(e);
                return CancelHandle {
                    inner: Arc::new(AtomicBool::new(false)),
                };
            }
        };
        let flag = Arc::new(AtomicBool::new(false));
        let handle = CancelHandle {
            inner: flag.clone(),
        };
        ylong_runtime::spawn(async move {
            if let Err(e) = download(url, &mut callback, flag).await {
                if e.error_kind() == ErrorKind::UserAborted {
                    callback.on_cancel();
                } else {
                    callback.on_fail(e);
                }
            }
        });
        handle
    }
}

pub async fn download(
    url: String,
    callback: &mut DownloadCallback,
    abort_flag: Arc<AtomicBool>,
) -> Result<(), HttpClientError> {
    let request = RequestBuilder::new()
        .url(url.as_str())
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let response = client().request(request).await?;
    let status = response.status();
    let operator = Operator {
        callback: callback,
        abort_flag: abort_flag,
        headers: response.headers().to_string(),
    };
    let mut downloader = Downloader::builder()
        .body(response)
        .operator(operator)
        .build();
    downloader.download().await?;

    let response = Response { status: status };
    callback.on_success(response);
    Ok(())
}

pub struct Response {
    status: StatusCode,
}

impl Response {
    pub fn status(&self) -> StatusCode {
        self.status
    }
}

#[derive(Clone)]
pub struct CancelHandle {
    inner: Arc<AtomicBool>,
}

impl CancelHandle {
    pub fn cancel(&mut self) {
        self.inner.store(true, Ordering::Release);
    }
}

/// RequestCallback
#[allow(unused_variables)]
pub trait RequestCallback {
    /// Called when the request is successful.
    fn on_success(&mut self, response: Response) {}
    /// Called when the request fails.
    fn on_fail(&mut self, error: HttpClientError) {}
    /// Called when the request is canceled.
    fn on_cancel(&mut self) {}
    /// Called when data is received.
    fn on_data_receive(&mut self, data: &[u8], mut task: RequestTask) {}
    /// Called when progress is made.
    fn on_progress(&mut self, dl_total: u64, dl_now: u64, ul_total: u64, ul_now: u64) {}
}
