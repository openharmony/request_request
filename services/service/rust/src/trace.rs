/*
 * Copyright (C) 2023 Huawei Device Co., Ltd.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

extern crate hitrace_meter_rust;

use hitrace_meter_rust::{start_trace, finish_trace};

/// Hitrace adapter which provides timing capability.
///
/// The timing will end automatically when the structure drops. Users should
/// take care that the lifetime of this structure.
pub(crate) struct TraceScope {
    label: u64,
}

impl TraceScope {
    // Copies from `Hitrace`.
    const HITRACE_TAG_MISC: u64 = 1u64 << 41;

    /// Starts tracing.
    pub(crate) fn trace(value: &str) -> Self {
        let trace = Self { label: Self::HITRACE_TAG_MISC };
        start_trace(trace.label, value);
        trace
    }
}

impl Drop for TraceScope {
    // The timing will end automatically when the structure drops.
    fn drop(&mut self) {
        finish_trace(self.label);
    }
}