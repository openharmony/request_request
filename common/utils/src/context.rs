// Copyright (c) 2023 Huawei Device Co., Ltd.
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

//! Application context utilities and wrapper.
//!
//! This module provides utilities for accessing application context information
//! and file system paths. It wraps the underlying native context implementation
//! and provides a safe Rust API.

use ani_rs::objects::AniObject;
use ani_rs::AniEnv;
use cxx::{SharedPtr};

use super::wrapper::GetCacheDir;
use crate::wrapper::{self, IsStageContext};

/// Retrieves the application's cache directory path.
///
/// Returns `None` if the cache directory is not available or empty.
#[inline]
pub fn get_cache_dir() -> Option<String> {
    let res = GetCacheDir();
    if res.is_empty() {
        None
    } else {
        Some(res)
    }
}

/// Determines whether the provided environment and object represent a stage context.
///
/// # Arguments
///
/// * `env` - The animation environment
/// * `ani_object` - The animation object
///
/// # Safety
///
/// This function performs pointer casting and calls an unsafe C function.
/// The caller must ensure that the provided environment and object are valid
/// and properly initialized.
#[inline]
pub fn is_stage_context(env: &AniEnv, ani_object: &AniObject) -> bool {
    // Cast to the appropriate types required by the C++ function
    let env = env as *const AniEnv as *mut AniEnv as *mut wrapper::AniEnv;
    let ani_object = ani_object as *const AniObject as *mut AniObject as *mut wrapper::AniObject;
    unsafe { IsStageContext(env, ani_object) }
}

pub struct Context {
    /// Inner C++ context shared pointer
    pub inner: SharedPtr<wrapper::Context>,
}

pub enum BundleType {
    /// Standard application bundle
    App = 0,
    /// Atomic service bundle
    AtomicService,
    /// Shared bundle
    Shared,
    /// Application service framework bundle
    AppServiceFwk,
    /// Application plugin bundle
    AppPlugin,
}

pub struct ApplicationInfo {
    /// The type of the application bundle
    pub bundle_type: BundleType,
}

impl From<wrapper::BundleType> for BundleType {
    /// Converts from the wrapper's BundleType to this module's BundleType.
    ///
    /// # Panics
    ///
    /// Panics if the provided BundleType value does not match any known variant.
    fn from(value: wrapper::BundleType) -> Self {
        match value {
            wrapper::BundleType::APP => BundleType::App,
            wrapper::BundleType::ATOMIC_SERVICE => BundleType::AtomicService,
            wrapper::BundleType::SHARED => BundleType::Shared,
            wrapper::BundleType::APP_SERVICE_FWK => BundleType::AppServiceFwk,
            wrapper::BundleType::APP_PLUGIN => BundleType::AppPlugin,
            _ => unimplemented!(),
        }
    }
}

impl Context {
    /// Creates a new Context from animation environment and object.
    ///
    /// # Arguments
    ///
    /// * `env` - The animation environment
    /// * `ani_object` - The animation object
    ///
    /// # Safety
    ///
    /// This function performs pointer casting and calls an unsafe C function.
    /// The caller must ensure that the provided environment and object are valid
    /// and properly initialized.
    pub fn new(env: &AniEnv, ani_object: &AniObject) -> Self {
        // Cast to the appropriate types required by the C++ function
        let env = env as *const AniEnv as *mut AniEnv as *mut *mut wrapper::AniEnv;
        let ani_object =
            ani_object as *const AniObject as *mut AniObject as *mut wrapper::AniObject;
        let inner = unsafe { wrapper::GetStageModeContext(env, ani_object) };
        Self { inner }
    }

    /// Retrieves the bundle name associated with this context.
    pub fn get_bundle_name(&self) -> String {
        wrapper::GetBundleName(&self.inner)
    }

    /// Retrieves the cache directory path associated with this context.
    pub fn get_cache_dir(&self) -> String {
        wrapper::ContextGetCacheDir(&self.inner)
    }

    /// Retrieves the base directory path associated with this context.
    pub fn get_base_dir(&self) -> String {
        wrapper::ContextGetBaseDir(&self.inner)
    }

    pub fn get_bundle_type(&self) -> BundleType {
        wrapper::BundleType(&self.inner.GetApplicationInfo()).into()
    }
}
