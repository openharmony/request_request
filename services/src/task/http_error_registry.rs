// Copyright (C) 2026 Huawei Device Co., Ltd.
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

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use ylong_http_client::StatusCode;

/// Registry that stores HTTP status codes for tasks that failed with
/// `ProtocolError`. The code is recorded when the task fails and consumed
/// (removed) when the first failure notification is built, ensuring the
/// detailed HTTP error message is delivered exactly once via the onFail
/// callback. Subsequent queries (e.g. getTask) will see the generic
/// "Http protocol error" message, keeping error information consistent.
pub(crate) struct HttpErrorRegistry {
    codes: Mutex<HashMap<u32, u16>>,
}

impl HttpErrorRegistry {
    /// Records the HTTP status code for a task that encountered a protocol error.
    pub(crate) fn set(&self, task_id: u32, code: u16) {
        self.codes.lock().unwrap().insert(task_id, code);
    }

    /// Takes (removes) and returns the HTTP status code recorded for a task.
    /// Returns 0 if no code was recorded.
    pub(crate) fn take(&self, task_id: u32) -> u16 {
        self.codes.lock().unwrap().remove(&task_id).unwrap_or(0)
    }
}

static HTTP_ERROR_REGISTRY: LazyLock<HttpErrorRegistry> =
    LazyLock::new(|| HttpErrorRegistry {
        codes: Mutex::new(HashMap::new()),
    });

/// Records the HTTP status code for a task that encountered a protocol error.
pub(crate) fn set_http_status_code(task_id: u32, code: u16) {
    HTTP_ERROR_REGISTRY.set(task_id, code);
}

/// Takes (removes) and returns the HTTP status code recorded for a task.
/// Returns 0 if no code was recorded. The code is consumed so that only
/// the first caller (the onFail notification) gets the detailed message.
pub(crate) fn take_http_status_code(task_id: u32) -> u16 {
    HTTP_ERROR_REGISTRY.take(task_id)
}

/// Returns the canonical HTTP reason phrase for a given status code.
/// Uses `ylong_http::StatusCode` to avoid duplicating the status code table.
pub(crate) fn http_status_reason_phrase(code: u16) -> &'static str {
    StatusCode::from_u16(code)
        .ok()
        .and_then(|s| s.reason())
        .unwrap_or("Unknown")
}
