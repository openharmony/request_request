// Copyright (C) 2025 Huawei Device Co., Ltd.
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

mod data;
mod description;
mod file_spec;
mod form_item;
mod index;
mod method;
mod min_speed;
mod notification;
mod proxy;
mod timeout;
mod title;
mod token;

// todo
pub(crate) mod url;

use std::sync::OnceLock;

use request_core::config::TaskConfig;

/// Aggregator that runs all registered config verifiers against a task config.
pub struct TaskConfigVerifier {
    verifiers: Vec<Box<dyn ConfigVerifier>>,
}

impl TaskConfigVerifier {
    /// Runs every registered verifier against the given config.
    ///
    /// # Returns
    /// `Ok(())` if all verifiers pass, or the first error code on failure.
    pub fn verify(&self, config: &TaskConfig) -> Result<(), i32> {
        for verifier in &self.verifiers {
            verifier.verify(config)?;
        }
        Ok(())
    }

    /// Returns the shared singleton verifier instance, initializing it with the
    /// full set of field verifiers on first access.
    pub fn get_instance() -> &'static Self {
        static INSTANCE: OnceLock<TaskConfigVerifier> = OnceLock::new();
        INSTANCE.get_or_init(|| TaskConfigVerifier {
            // todo: minspeed timeout new notification
            verifiers: vec![
                Box::new(url::UrlVerifier {}),
                Box::new(method::MethodVerifier {}),
                Box::new(file_spec::FileSpecVerifier {}),
                Box::new(form_item::FormItemVerifier {}),
                Box::new(index::IndexVerifier {}),
                Box::new(title::TitleVerifier {}),
                Box::new(data::DataVerifier {}),
                Box::new(proxy::ProxyVerifier {}),
                Box::new(token::TokenVerifier {}),
                Box::new(description::DescriptionVerifier {}),
                Box::new(notification::NotificationVerifier {}),
                Box::new(min_speed::MinSpeedVerifier {}),
                Box::new(timeout::TimeoutVerifier {}),
            ],
        })
    }
}

pub(crate) trait ConfigVerifier: Send + Sync {
    fn verify(&self, config: &TaskConfig) -> Result<(), i32>;
}
