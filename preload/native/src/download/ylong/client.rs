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

use std::sync::LazyLock;

use ylong_http_client::async_impl::Client;
use ylong_http_client::{Redirect, Timeout, TlsVersion};

const CONNECT_TIMEOUT: u64 = 60;
const SECONDS_IN_ONE_WEEK: u64 = 7 * 24 * 60 * 60;

pub(crate) fn client() -> &'static Client {
    static CLIENT: LazyLock<Client> = LazyLock::new(|| {
        let client = Client::builder()
            .connect_timeout(Timeout::from_secs(CONNECT_TIMEOUT))
            .request_timeout(Timeout::from_secs(SECONDS_IN_ONE_WEEK))
            .min_tls_version(TlsVersion::TLS_1_2)
            .redirect(Redirect::limited(usize::MAX))
            .tls_built_in_root_certs(true);
        client.build().unwrap()
    });
    &CLIENT
}
