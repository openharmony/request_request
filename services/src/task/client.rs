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

use ylong_http_client::async_impl::{Client, Request};
use ylong_http_client::{
    Certificate, HttpClientError, Interceptor, Proxy, PubKeyPins, Redirect, Timeout, TlsVersion,
};

cfg_oh! {
    use crate::manage::SystemConfig;
    use crate::utils::url_policy::check_url_domain;
}

use super::files::BundleCache;
use crate::service::notification_bar::NotificationDispatcher;
use crate::task::config::{Action, TaskConfig};
use crate::task::files::convert_path;

const SECONDS_IN_TEN_MINUTES: u64 = 10 * 60;
const DEFAULT_CONNECTION_TIMEOUT: u64 = 60;
const DEFAULT_TOTAL_TIMEOUT: u64 = 60 * 60 * 24 * 7;

pub(crate) fn build_client(
    config: &TaskConfig,
    task_time: u64,
    #[cfg(feature = "oh")] mut system: SystemConfig,
) -> Result<Client, Box<dyn Error + Send + Sync>> {
    let mut rest_timeout = 0;
    let mut connection_timeout = config.common_data.timeout.connection_timeout;
    if connection_timeout == 0 {
        connection_timeout = DEFAULT_CONNECTION_TIMEOUT;
    }

    let mut total_timeout = config.common_data.timeout.total_timeout;

    if total_timeout == 0 {
        if !NotificationDispatcher::get_instance()
            .check_task_notification_available(config.common_data.task_id) {
            total_timeout = SECONDS_IN_TEN_MINUTES;
        } else {
            total_timeout = DEFAULT_TOTAL_TIMEOUT;
        }
    }

    if total_timeout > task_time {
        rest_timeout = total_timeout - task_time;
    }
    let mut client = Client::builder()
        .connect_timeout(Timeout::from_secs(connection_timeout))
        .request_timeout(Timeout::from_secs(rest_timeout))
        .min_tls_version(TlsVersion::TLS_1_2);
    client = client.sockets_owner(config.common_data.uid as u32, config.common_data.uid as u32);
    // Set redirect strategy.
    if config.common_data.redirect {
        client = client.redirect(Redirect::limited(usize::MAX));
    } else {
        client = client.redirect(Redirect::none());
    }

    if config.common_data.min_speed.speed > 0 && config.common_data.min_speed.duration > 0 {
        client = client
            .min_speed_limit(config.common_data.min_speed.speed as u64)
            .min_speed_interval(config.common_data.min_speed.duration as u64);
    }

    // Set HTTP proxy.
    #[cfg(feature = "oh")]
    if let Some(proxy) = build_task_proxy(config)? {
        client = client.proxy(proxy);
    } else if let Some(proxy) = build_system_proxy(&system)? {
        client = client.proxy(proxy);
    }

    // HTTP url that contains redirects also require a certificate when
    // redirected to HTTPS.

    // Set system certs.
    #[cfg(feature = "oh")]
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

    const ATOMIC_SERVICE: u32 = 1;
    if config.bundle_type == ATOMIC_SERVICE {
        let domain_type = action_to_domain_type(config.common_data.action);
        info!(
            "ApiPolicy Domain check, tid {}, bundle {}, domain_type {}, url {}",
            config.common_data.task_id, &config.bundle, &domain_type, &config.url
        );
        #[cfg(feature = "oh")]
        if let Some(is_accessed) = check_url_domain(&config.bundle, &domain_type, &config.url) {
            if !is_accessed {
                error!(
                    "Intercept request by domain check, tid {}, bundle {}, domain_type {}, url {}",
                    config.common_data.task_id, &config.bundle, &domain_type, &config.url
                );
                sys_event!(
                    ExecFault,
                    DfxCode::URL_POLICY_FAULT_00,
                    &format!(
                    "Intercept request by domain check, tid {}, bundle {}, domain_type {}, url {}",
                config.common_data.task_id, &config.bundle, &domain_type, &config.url)
                );

                return Err(Box::new(HttpClientError::other(
                    "Intercept request by domain check",
                )));
            }
        } else {
            info!(
                "Intercept request by domain check, tid {}, domain_type {}, url {}",
                config.common_data.task_id, &domain_type, &config.url
            );
        }

        #[cfg(feature = "oh")]
        {
            let interceptors = DomainInterceptor::new(config.bundle.clone(), domain_type);
            client = client.interceptor(interceptors);
        }

        info!(
            "add interceptor domain check, tid {}",
            config.common_data.task_id
        );
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

#[cfg(feature = "oh")]
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
    let paths = config.certs_path.as_slice();
    let mut bundle_cache = BundleCache::new(config);

    let mut certs = Vec::new();
    for (idx, path) in paths.iter().enumerate() {
        let bundle_name = bundle_cache.get_value()?;
        let path = convert_path(uid, &bundle_name, path);
        let cert = cvt_res_error!(
            Certificate::from_path(&path).map_err(Box::new),
            "Parse task cert failed - idx: {}",
            idx,
        );
        certs.push(cert);
    }
    Ok(certs)
}

fn action_to_domain_type(action: Action) -> String {
    match action {
        Action::Download => "download".to_string(),
        Action::Upload => "upload".to_string(),
        Action::Any => "".to_string(),
        _ => unreachable!(),
    }
}

struct DomainInterceptor {
    app_id: String,
    domain_type: String,
}

impl DomainInterceptor {
    fn new(app_id: String, domain_type: String) -> Self {
        DomainInterceptor {
            app_id,
            domain_type,
        }
    }
}

#[cfg(feature = "oh")]
impl Interceptor for DomainInterceptor {
    /// Intercepts the redirect request.
    fn intercept_redirect_request(&self, request: &Request) -> Result<(), HttpClientError> {
        let url = &request.uri().to_string();
        info!(
            "ApiPolicy Domain check redirect, bundle {}, domain_type {}, url {}",
            &self.app_id, &self.domain_type, &url
        );
        match check_url_domain(&self.app_id, &self.domain_type, url).unwrap_or(true) {
            true => Ok(()),
            false => Err(HttpClientError::other(
                "Intercept redirect request by domain check",
            )),
        }
    }
}
