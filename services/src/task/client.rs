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

//! HTTP client configuration utilities for request tasks.
//! 
//! This module provides functionality to build and configure HTTP clients
//! with appropriate settings based on task configurations, including
//! timeouts, redirect policies, certificates, proxies, and domain policies.
//! 
//! Key features include:
//! - Secure TLS configuration and certificate management
//! - Proxy settings support with task-specific and system-wide options
//! - Domain policy enforcement for atomic services
//! - Redirect handling with domain validation
//! - Public key pinning for enhanced security
//! - Connection timeout and speed monitoring

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
use crate::task::config::{Action, TaskConfig};
use crate::task::files::convert_path;

/// Builds an HTTP client with configuration based on the provided task settings.
///
/// # Arguments
///
/// * `config` - The task configuration containing connection parameters, certificates,
///             proxy settings, and other client options.
/// * `total_timeout` - The total timeout in seconds for the entire client operation.
/// * `system` - [Only in OHOS] System configuration containing system-wide settings.
///
/// # Returns
///
/// Returns `Ok(Client)` with the configured client if successful, or an error if any
/// configuration step fails.
///
/// # Examples
///
/// ```rust
/// use crate::task::config::TaskConfig;
///
/// // Assuming a TaskConfig instance
/// let config = TaskConfig::default();
/// let total_timeout = 300; // 5 minutes
/// #[cfg(feature = "oh")]
/// let system = SystemConfig::default();
///
/// #[cfg(feature = "oh")]
/// let client = build_client(&config, total_timeout, system)?;
/// #[cfg(not(feature = "oh"))]
/// let client = build_client(&config, total_timeout)?;
/// ```
///
/// # Errors
///
/// Returns an error if any configuration step fails, including:
/// - Proxy creation failures
/// - Certificate loading issues
/// - Client build errors
/// - Domain policy validation failures
pub(crate) fn build_client(
    config: &TaskConfig,
    total_timeout: u64,
    #[cfg(feature = "oh")] mut system: SystemConfig,
) -> Result<Client, Box<dyn Error + Send + Sync>> {
    const DEFAULT_CONNECTION_TIMEOUT: u64 = 60;

    // Use default timeout if none specified
    let mut connection_timeout = config.common_data.timeout.connection_timeout;
    if connection_timeout == 0 {
        connection_timeout = DEFAULT_CONNECTION_TIMEOUT;
    }

    // Set up basic client configuration with required timeouts and TLS version
    // Ensure connections are established within a reasonable time and operations complete promptly
    let mut client = Client::builder()
        .connect_timeout(Timeout::from_secs(connection_timeout))  // Time to establish connection
        .total_timeout(Timeout::from_secs(total_timeout))         // Total time limit for entire request
        .min_tls_version(TlsVersion::TLS_1_2);                    // Enforce secure TLS version
    
    // Set socket ownership for proper resource management
    client = client.sockets_owner(config.common_data.uid as u32, config.common_data.uid as u32);
    
    // Configure redirect strategy based on task settings
    if config.common_data.redirect {
        // Allow unlimited redirects when explicitly requested
        client = client.redirect(Redirect::limited(usize::MAX));
    } else {
        // Disable redirects by default for security and predictability
        client = client.redirect(Redirect::none());
    }

    // Configure minimum speed requirements if specified to detect stalled connections
    if config.common_data.min_speed.speed > 0 && config.common_data.min_speed.duration > 0 {
        client = client
            .min_speed_limit(config.common_data.min_speed.speed as u64)    // Minimum bytes per second
            .min_speed_interval(config.common_data.min_speed.duration as u64); // Check interval in seconds
    }

    // Configure proxy settings with task-specific settings taking precedence over system-wide settings
    #[cfg(feature = "oh")]
    if let Some(proxy) = build_task_proxy(config)? {
        client = client.proxy(proxy); // Use task-specific proxy when available
    } else if let Some(proxy) = build_system_proxy(&system)? {
        client = client.proxy(proxy); // Fall back to system proxy if no task proxy
    }

    // HTTP url that contains redirects also require a certificate when
    // redirected to HTTPS.

    // Add system certificates if available
    #[cfg(feature = "oh")]
    if let Some(certs) = system.certs.take() {
        // Load and trust system-provided CA certificates
        for cert in certs.into_iter() {
            client = client.add_root_certificate(cert)
        }
    }

    // Add task-specific certificates
    // These certificates override or supplement the system certificates
    // The ? operator automatically converts errors from build_task_certs into the expected error type
    let certificates = build_task_certs(config)?;
    for cert in certificates.into_iter() {
        client = client.add_root_certificate(cert)  // Trust each provided certificate
    }

    // Configure public key pinning if specified
    // This enhances security by limiting which certificates are accepted
    if let Some(pinned_key) = build_task_certificate_pins(config)? {
        client = client.add_public_key_pins(pinned_key);
    }

    // Apply domain policy checks for atomic services (system-specific security check)
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
                // Log policy violation and return error
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

                // Wrap the HttpClientError in a Box to fit the function's return type requirement
                // This conversion allows us to return a trait object implementing Error + Send + Sync
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

        // Add interceptor to check redirects against domain policy
        // This ensures that any URLs encountered during redirects also comply with
        // the domain access policies, providing comprehensive security coverage
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

    // Finalize client construction
    // All configuration steps are complete including timeouts, redirect policy,
    // proxy settings, certificates, public key pinning, and domain policy enforcement
    // cvt_res_error! macro handles error conversion and adds context to the error message
    // map_err(Box::new) converts any build errors to a Box<dyn Error + Send + Sync>
    Ok(cvt_res_error!(
        client.build().map_err(Box::new),
        "Build client failed",
    ))
}

/// Creates a proxy configuration from task settings.
///
/// # Arguments
///
/// * `config` - The task configuration containing proxy settings.
///
/// # Returns
///
/// Returns `Ok(Some(Proxy))` with the configured proxy if proxy settings exist,
/// `Ok(None)` if no proxy is configured, or an error if proxy creation fails.
///
/// # Examples
///
/// ```rust
/// use crate::task::config::TaskConfig;
///
/// // With proxy
/// let mut config = TaskConfig::default();
/// config.proxy = "http://proxy.example.com:8080".to_string();
/// let proxy = build_task_proxy(&config)?; // Returns Some(proxy)
///
/// // Without proxy
/// let config = TaskConfig::default();
/// let proxy = build_task_proxy(&config)?; // Returns None
/// ```

fn build_task_proxy(config: &TaskConfig) -> Result<Option<Proxy>, Box<dyn Error + Send + Sync>> {
    // Check if proxy is configured
    if config.proxy.is_empty() {
        return Ok(None);
    }

    // Create proxy configuration for all protocols
    Ok(Some(cvt_res_error!(
        Proxy::all(&config.proxy).build().map_err(Box::new),
        "Create task proxy failed",
    )))
}

/// Creates public key pinning configuration from task settings.
///
/// # Arguments
///
/// * `config` - The task configuration containing certificate pinning data.
///
/// # Returns
///
/// Returns `Ok(Some(PubKeyPins))` with the configured pinning if certificate pins exist,
/// `Ok(None)` if no pinning is configured, or an error if pinning creation fails.
///
/// # Examples
///
/// ```rust
/// use crate::task::config::TaskConfig;
///
/// // With certificate pins
/// let mut config = TaskConfig::default();
/// config.url = "https://example.com".to_string();
/// config.certificate_pins = vec!["sha256/AAAA...".to_string()];
/// let pins = build_task_certificate_pins(&config)?; // Returns Some(pins)
///
/// // Without certificate pins
/// let config = TaskConfig::default();
/// let pins = build_task_certificate_pins(&config)?; // Returns None
/// ```

fn build_task_certificate_pins(
    config: &TaskConfig,
) -> Result<Option<PubKeyPins>, Box<dyn Error + Send + Sync>> {
    // Check if certificate pins are configured
    if config.certificate_pins.is_empty() {
        return Ok(None);
    }

    // Create public key pinning for the target URL
    Ok(Some(cvt_res_error!(
        PubKeyPins::builder()
            .add(&config.url, &config.certificate_pins)
            .build()
            .map_err(Box::new),
        "Create task certificate pinned_key failed",
    )))
}

/// Creates a proxy configuration from system settings.
///
/// # Arguments
///
/// * `system` - The system configuration containing proxy settings.
///
/// # Returns
///
/// Returns `Ok(Some(Proxy))` with the configured proxy if system proxy is set,
/// `Ok(None)` if no system proxy is configured, or an error if proxy creation fails.
///
/// # Examples
///
/// ```rust
/// use crate::manage::SystemConfig;
///
/// // With system proxy
/// let mut system = SystemConfig::default();
/// system.proxy_host = "proxy.system.com".to_string();
/// system.proxy_port = "8080".to_string();
/// system.proxy_exlist = vec!["localhost", "127.0.0.1"];
/// let proxy = build_system_proxy(&system)?; // Returns Some(proxy)
///
/// // Without system proxy
/// let system = SystemConfig::default();
/// let proxy = build_system_proxy(&system)?; // Returns None
/// ```
///
/// # Feature
///
/// This function is only available when the `oh` feature is enabled.
#[cfg(feature = "oh")]
fn build_system_proxy(
    system: &SystemConfig,
) -> Result<Option<Proxy>, Box<dyn Error + Send + Sync>> {
    let proxy_host = &system.proxy_host;

    // Check if proxy host is configured
    if proxy_host.is_empty() {
        return Ok(None);
    }

    // Construct full proxy URL with optional port
    // Handles both host-only and host:port formats
    let proxy_port = &system.proxy_port;
    let proxy_url = match proxy_port.is_empty() {
        true => proxy_host.clone(),
        false => format!("{}:{}", proxy_host, proxy_port),
    };
    
    // Get proxy exclusions list
    let no_proxy = &system.proxy_exlist;
    
    // Create proxy with exclusions
    Ok(Some(cvt_res_error!(
        Proxy::all(&proxy_url)
            .no_proxy(no_proxy)
            .build()
            .map_err(Box::new),
        "Create system proxy failed",
    )))
}

/// Loads and parses certificates from the specified paths in the task configuration.
///
/// # Arguments
///
/// * `config` - The task configuration containing certificate paths.
///
/// # Returns
///
/// Returns `Ok(Vec<Certificate>)` with the loaded certificates, or an error if any
/// certificate fails to load or parse.
///
/// # Examples
///
/// ```rust
/// use crate::task::config::TaskConfig;
///
/// let mut config = TaskConfig::default();
/// config.common_data.uid = 1000;
/// config.certs_path = vec!["path/to/cert.pem".to_string()];
/// let certs = build_task_certs(&config)?; // Returns loaded certificates
/// ```

fn build_task_certs(config: &TaskConfig) -> Result<Vec<Certificate>, Box<dyn Error + Send + Sync>> {
    let uid = config.common_data.uid;
    let paths = config.certs_path.as_slice();
    let mut bundle_cache = BundleCache::new(config);

    let mut certs = Vec::new();
    
    // Load each certificate from the configured paths
    for (idx, path) in paths.iter().enumerate() {
        // Get bundle name for path conversion
        let bundle_name = bundle_cache.get_value()?;
        
        // Convert path to appropriate format based on user and bundle
        let path = convert_path(uid, &bundle_name, path);
        
        // Load and parse certificate
        let cert = cvt_res_error!(
            Certificate::from_path(&path).map_err(Box::new),
            "Parse task cert failed - idx: {}",
            idx,
        );
        certs.push(cert);
    }
    Ok(certs)
}

/// Converts an Action enum value to a domain type string used for policy checks.
///
/// # Arguments
///
/// * `action` - The action type to convert.
///
/// # Returns
///
/// Returns a string representing the domain type for the given action.
///
/// # Panics
///
/// Panics if an unexpected Action variant is provided.
///
/// # Examples
///
/// ```rust
/// use crate::task::config::Action;
///
/// assert_eq!(action_to_domain_type(Action::Download), "download");
/// assert_eq!(action_to_domain_type(Action::Upload), "upload");
/// assert_eq!(action_to_domain_type(Action::Any), "");
/// ```
fn action_to_domain_type(action: Action) -> String {
    match action {
        Action::Download => "download".to_string(),
        Action::Upload => "upload".to_string(),
        Action::Any => "".to_string(),
        _ => unreachable!(),
    }
}

/// Interceptor that validates redirect URLs against domain policies.
///
/// This interceptor checks if redirect URLs comply with the domain access policies
/// for the specified application and action type.
struct DomainInterceptor {
    /// The application ID to check domain policies against.
    app_id: String,
    /// The domain type (download/upload) for policy validation.
    domain_type: String,
}

impl DomainInterceptor {
    /// Creates a new DomainInterceptor with the specified application and domain type.
    ///
    /// # Arguments
    ///
    /// * `app_id` - The application ID to check policies for.
    /// * `domain_type` - The domain type for policy validation.
    ///
    /// # Returns
    ///
    /// Returns a new DomainInterceptor instance.
    fn new(app_id: String, domain_type: String) -> Self {
        DomainInterceptor {
            app_id,
            domain_type,
        }
    }
}

#[cfg(feature = "oh")]
impl Interceptor for DomainInterceptor {
    /// Validates a redirect request URL against domain policies.
    ///
    /// # Arguments
    ///
    /// * `request` - The redirect request to validate.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the redirect is allowed, or an error if the domain is not
    /// allowed by the policy.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The URL cannot be parsed
    /// - The domain is not allowed by the policy
    ///
    /// # Feature
    ///
    /// This implementation is only available when the `oh` feature is enabled.
    fn intercept_redirect_request(&self, request: &Request) -> Result<(), HttpClientError> {
        // Get the redirect URL
        let url = &request.uri().to_string();
        
        // Log the domain check attempt
        info!(
            "ApiPolicy Domain check redirect, bundle {}, domain_type {}, url {}",
            &self.app_id, &self.domain_type, &url
        );
        
        // Check if the URL is allowed by domain policy, defaulting to true if check fails
        match check_url_domain(&self.app_id, &self.domain_type, url).unwrap_or(true) {
            true => Ok(()),
            false => Err(HttpClientError::other(
                "Intercept redirect request by domain check",
            )),
        }
    }
}
