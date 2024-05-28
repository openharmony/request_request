// Copyright (C) 2023 Huawei Device Co., Ltd.
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

use std::error::Error;

use ylong_http_client::async_impl::Client;
use ylong_http_client::{Certificate, Proxy, PubKeyPins, Redirect, Timeout, TlsVersion};

use crate::manage::SystemConfig;
use crate::task::config::TaskConfig;
use crate::task::files::convert_path;

const CONNECT_TIMEOUT: u64 = 60;
const SECONDS_IN_ONE_WEEK: u64 = 7 * 24 * 60 * 60;

pub(crate) fn build_client(
    config: &TaskConfig,
    mut system: SystemConfig,
) -> Result<Client, Box<dyn Error + Send + Sync>> {
    let mut client = Client::builder()
        .connect_timeout(Timeout::from_secs(CONNECT_TIMEOUT))
        .request_timeout(Timeout::from_secs(SECONDS_IN_ONE_WEEK))
        .min_tls_version(TlsVersion::TLS_1_2);

    // Set redirect strategy.
    if config.common_data.redirect {
        client = client.redirect(Redirect::limited(usize::MAX));
    } else {
        client = client.redirect(Redirect::none());
    }

    // Set HTTP proxy.
    if let Some(proxy) = build_task_proxy(config)? {
        client = client.proxy(proxy);
    } else if let Some(proxy) = build_system_proxy(&system)? {
        client = client.proxy(proxy);
    }

    // HTTP url that contains redirects also require a certificate when
    // redirected to HTTPS.

    // Set system certs.
    if let Some(certs) = system.certs.take() {
        for cert in certs.into_iter() {
            client = client.add_root_certificate(cert)
        }
    }

    // Set task certs.
    let certificates = build_task_certs(config)?;
    for cert in certificates.into_iter() {
        client = client.add_root_certificate(cert)
    }

    // Set task certificate pinned_key.
    if let Some(pinned_key) = build_task_certificate_pins(config)? {
        client = client.add_public_key_pins(pinned_key);
    }

    // Build client.
    Ok(cvt_res_error!(
        client.build().map_err(Box::new),
        "Build client failed",
    ))
}

fn build_task_proxy(config: &TaskConfig) -> Result<Option<Proxy>, Box<dyn Error + Send + Sync>> {
    if config.proxy.is_empty() {
        return Ok(None);
    }

    Ok(Some(cvt_res_error!(
        Proxy::all(&config.proxy).build().map_err(Box::new),
        "Create task proxy failed",
    )))
}

fn build_task_certificate_pins(
    config: &TaskConfig,
) -> Result<Option<PubKeyPins>, Box<dyn Error + Send + Sync>> {
    if config.certificate_pins.is_empty() {
        return Ok(None);
    }

    Ok(Some(cvt_res_error!(
        PubKeyPins::builder()
            .add(&config.url, &config.certificate_pins)
            .build()
            .map_err(Box::new),
        "Create task certificate pinned_key failed",
    )))
}

fn build_system_proxy(
    system: &SystemConfig,
) -> Result<Option<Proxy>, Box<dyn Error + Send + Sync>> {
    let proxy_host = &system.proxy_host;

    if proxy_host.is_empty() {
        return Ok(None);
    }

    let proxy_port = &system.proxy_port;
    let proxy_url = match proxy_port.is_empty() {
        true => proxy_host.clone(),
        false => format!("{}:{}", proxy_host, proxy_port),
    };
    let no_proxy = &system.proxy_exlist;
    Ok(Some(cvt_res_error!(
        Proxy::all(&proxy_url)
            .no_proxy(no_proxy)
            .build()
            .map_err(Box::new),
        "Create system proxy failed",
    )))
}

fn build_task_certs(config: &TaskConfig) -> Result<Vec<Certificate>, Box<dyn Error + Send + Sync>> {
    let uid = config.common_data.uid;
    let bundle = config.bundle.as_str();
    let paths = config.certs_path.as_slice();

    let mut certs = Vec::new();
    for (idx, path) in paths.iter().enumerate() {
        let path = convert_path(uid, bundle, path);
        let cert = cvt_res_error!(
            Certificate::from_path(&path).map_err(Box::new),
            "Parse task cert failed - idx: {}, path: {}",
            idx,
            path,
        );
        certs.push(cert);
    }
    Ok(certs)
}
