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

pub(crate) mod c_wrapper;
pub(crate) mod form_item;
pub(crate) mod task_id_generator;
pub(crate) mod url_policy;
use std::collections::HashMap;
use std::future::Future;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) use ffi::RequestTaskMsg;
use ylong_runtime::sync::oneshot::Receiver;
use ylong_runtime::task::JoinHandle;

use crate::task::info::ApplicationState;

pub(crate) struct Recv<T> {
    rx: Receiver<T>,
}

impl<T> Recv<T> {
    pub(crate) fn new(rx: Receiver<T>) -> Self {
        Self { rx }
    }

    pub(crate) fn get(self) -> Option<T> {
        // Here `self.rx` can never be hung up.
        ylong_runtime::block_on(self.rx).ok()
    }
}

pub(crate) fn build_vec<A, B, C>(ptr: *const A, len: usize, func: C) -> Vec<B>
where
    C: Fn(&A) -> B,
{
    if ptr.is_null() || len == 0 {
        return Vec::<B>::new();
    }
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    slice.iter().map(func).collect()
}

pub(crate) fn get_current_timestamp() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() as u64,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

pub(crate) fn hashmap_to_string(map: &HashMap<String, String>) -> String {
    let mut res = Vec::new();
    for (n, (k, v)) in map.iter().enumerate() {
        if n != 0 {
            let _ = write!(res, "\r\n");
        }
        let _ = write!(res, "{k}\t{v}");
    }
    unsafe { String::from_utf8_unchecked(res) }
}

pub(crate) fn string_to_hashmap(str: &mut str) -> HashMap<String, String> {
    let mut map = HashMap::<String, String>::new();
    if str.is_empty() {
        return map;
    }
    for item in str.split("\r\n") {
        let (k, v) = item.split_once('\t').unwrap();
        map.insert(k.into(), v.into());
    }
    map
}

pub(crate) fn split_string(str: &mut str) -> std::str::Split<'_, &str> {
    let pat: &[_] = &['[', ']'];
    str.trim_matches(pat).split(", ")
}

#[inline(always)]
pub(crate) fn runtime_spawn<F: Future<Output = ()> + Send + Sync + 'static>(
    fut: F,
) -> JoinHandle<()> {
    ylong_runtime::spawn(Box::into_pin(
        Box::new(fut) as Box<dyn Future<Output = ()> + Send + Sync>
    ))
}

pub(crate) fn query_app_state(uid: u64) -> ApplicationState {
    let top_uid = query_top_uid();
    match top_uid {
        Some(top_uid) => {
            if top_uid == uid {
                ApplicationState::Foreground
            } else {
                ApplicationState::Background
            }
        }
        None => ApplicationState::Foreground,
    }
}

fn query_top_uid() -> Option<u64> {
    let mut uid = 0;
    for i in 0..10 {
        let ret = ffi::GetTopUid(&mut uid);
        if ret != 0 || uid == 0 {
            error!("GetTopUid failed, ret: {} retry time: {}", ret, i);
            std::thread::sleep(std::time::Duration::from_millis(200));
        } else {
            debug!("GetTopUid ok: {}", uid);
            return Some(uid as u64);
        }
    }
    error!("GetTopUid failed");
    None
}

pub(crate) fn query_calling_bundle() -> String {
    let token_id = ipc::Skeleton::calling_full_token_id();
    ffi::GetCallingBundle(token_id)
}

pub(crate) fn is_system_api() -> bool {
    let token_id = ipc::Skeleton::calling_full_token_id();
    ffi::IsSystemAPI(token_id)
}

pub(crate) fn check_permission(permission: &str) -> bool {
    let token_id = ipc::Skeleton::calling_full_token_id();
    ffi::CheckPermission(token_id, permission)
}

pub(crate) fn publish_state_change_event(
    bundle_name: &str,
    task_id: u32,
    state: i32,
) -> Result<(), ()> {
    match ffi::PublishStateChangeEvent(bundle_name, task_id, state) {
        true => Ok(()),
        false => Err(()),
    }
}

pub(crate) fn request_background_notify(
    msg: RequestTaskMsg,
    wrapped_path: &str,
    wrapped_file_name: &str,
    percent: u32,
) -> Result<(), i32> {
    match ffi::RequestBackgroundNotify(msg, wrapped_path, wrapped_file_name, percent) {
        0 => Ok(()),
        code => Err(code),
    }
}

#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {
    pub(crate) struct RequestTaskMsg {
        pub(crate) task_id: u32,
        pub(crate) uid: i32,
        pub(crate) action: u8,
    }

    unsafe extern "C++" {}

    unsafe extern "C++" {
        include!("request_utils.h");

        fn PublishStateChangeEvent(bundleName: &str, taskId: u32, state: i32) -> bool;

        fn RequestBackgroundNotify(
            msg: RequestTaskMsg,
            wrapped_path: &str,
            wrapped_file_name: &str,
            percent: u32,
        ) -> i32;

        fn GetTopUid(uid: &mut i32) -> i32;
        fn GetCallingBundle(token_id: u64) -> String;
        fn IsSystemAPI(token_id: u64) -> bool;
        fn CheckPermission(token_id: u64, permission: &str) -> bool;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tests::test_init;
    #[test]
    fn ut_utils_oh() {
        assert!(!is_system_api());
        assert_eq!(query_calling_bundle(), "");
    }

    #[test]
    fn ut_utils_publish_state_change_event() {
        test_init();
        publish_state_change_event("com.ohos.request", 1, 1).unwrap();
    }

    #[test]
    fn ut_utils_request_background_notify() {
        test_init();
        request_background_notify(
            RequestTaskMsg {
                task_id: 1,
                uid: 1,
                action: 1,
            },
            "path",
            "file",
            1,
        )
        .unwrap();
    }

    #[test]
    fn ut_utils_check_permission() {
        assert!(!check_permission("ohos.permission.INTERNET"));
        assert!(!check_permission("ohos.permission.GET_NETWORK_INFO"));
        assert!(!check_permission("ohos.permission.READ_MEDIA"));
        assert!(!check_permission("ohos.permission.WRITE_MEDIA"));
        assert!(!check_permission("ohos.permission.RUNNING_STATE_OBSERVER"));
        assert!(!check_permission("ohos.permission.GET_NETWORK_INFO"));
        assert!(!check_permission("ohos.permission.CONNECTIVITY_INTERNAL"));
        assert!(!check_permission(
            "ohos.permission.SEND_TASK_COMPLETE_EVENT"
        ));
        assert!(!check_permission("ohos.permission.ACCESS_CERT_MANAGER"));
        assert!(!check_permission(
            "ohos.permission.INTERACT_ACROSS_LOCAL_ACCOUNTS"
        ));
        assert!(!check_permission("ohos.permission.MANAGE_LOCAL_ACCOUNTS"));
    }

    #[test]
    fn ut_utils_check_permission_oh() {
        test_init();
        assert!(check_permission("ohos.permission.INTERNET"));
        assert!(check_permission("ohos.permission.GET_NETWORK_INFO"));
        assert!(check_permission("ohos.permission.READ_MEDIA"));
        assert!(check_permission("ohos.permission.WRITE_MEDIA"));
        assert!(check_permission("ohos.permission.RUNNING_STATE_OBSERVER"));
        assert!(check_permission("ohos.permission.GET_NETWORK_INFO"));
        assert!(check_permission("ohos.permission.CONNECTIVITY_INTERNAL"));
        assert!(check_permission("ohos.permission.SEND_TASK_COMPLETE_EVENT"));
        assert!(check_permission("ohos.permission.ACCESS_CERT_MANAGER"));
        assert!(check_permission(
            "ohos.permission.INTERACT_ACROSS_LOCAL_ACCOUNTS"
        ));
        assert!(check_permission("ohos.permission.MANAGE_LOCAL_ACCOUNTS"));
        assert!(check_permission("ohos.permission.GET_RUNNING_INFO"));
        assert!(!check_permission(
            "ohos.permission.INTERACT_ACROSS_LOCAL_ACCOUNTS_EXTENSION"
        ));
    }

    #[test]
    fn ut_utils_query_app_state() {
        test_init();
        assert_eq!(query_app_state(0), ApplicationState::Foreground);
    }
}
