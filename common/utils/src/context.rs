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

use ani_rs::objects::AniObject;
use ani_rs::AniEnv;
use cxx::SharedPtr;

use super::wrapper::GetCacheDir;
use crate::wrapper::{self, IsStageContext};

#[inline]
pub fn get_cache_dir() -> Option<String> {
    let res = GetCacheDir();
    if res.is_empty() {
        None
    } else {
        Some(res)
    }
}

#[inline]
pub fn is_stage_context(env: &AniEnv, ani_object: &AniObject) -> bool {
    let env = env as *const AniEnv as *mut AniEnv as *mut wrapper::AniEnv;
    let ani_object = ani_object as *const AniObject as *mut AniObject as *mut wrapper::AniObject;
    unsafe { IsStageContext(env, ani_object) }
}

pub struct Context {
    inner: SharedPtr<wrapper::Context>,
}

pub enum BundleType {
    App,
    AtomicService,
    Shared,
    AppServiceFwk,
    AppPlugin,
}

pub struct ApplicationInfo {
    pub bundle_type: BundleType,
}

impl From<wrapper::BundleType> for BundleType {
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
    pub fn new(env: &AniEnv, ani_object: &AniObject) -> Self {
        let env = env as *const AniEnv as *mut AniEnv as *mut *mut wrapper::AniEnv;
        let ani_object =
            ani_object as *const AniObject as *mut AniObject as *mut wrapper::AniObject;
        let inner = unsafe { wrapper::GetStageModeContext(env, ani_object) };
        Self { inner }
    }

    pub fn get_bundle_name(&self) -> String {
        wrapper::GetBundleName(&self.inner)
    }

    pub fn get_cache_dir(&self) -> String {
        wrapper::ContextGetCacheDir(&self.inner)
    }

    pub fn get_base_dir(&self) -> String {
        wrapper::ContextGetBaseDir(&self.inner)
    }
}
