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
pub(crate) mod common_event;
pub(crate) mod form_item;
use std::collections::HashMap;
use std::future::Future;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) use common_event::{
    subscribe_common_event, CommonEventSubscriber, Want as CommonEventWant,
};
pub(crate) use ffi::PublishStateChangeEvent;

cfg_oh! {
    pub(crate) mod url_policy;
    #[cfg(not(test))]
    pub(crate) use ffi::GetForegroundAbilities;
}

pub(crate) mod task_id_generator;
use ylong_runtime::sync::oneshot::Receiver;
use ylong_runtime::task::JoinHandle;

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

#[cfg(feature = "oh")]
pub(crate) fn query_calling_bundle() -> String {
    let token_id = ipc::Skeleton::calling_full_token_id();
    ffi::GetCallingBundle(token_id)
}

#[cfg(feature = "oh")]
pub(crate) fn is_system_api() -> bool {
    let token_id = ipc::Skeleton::calling_full_token_id();
    ffi::IsSystemAPI(token_id)
}

#[cfg(feature = "oh")]
pub(crate) fn check_permission(permission: &str) -> bool {
    let token_id = ipc::Skeleton::calling_full_token_id();
    ffi::CheckPermission(token_id, permission)
}

#[cfg(feature = "oh")]
pub(crate) fn update_policy(any_tasks: bool) -> i32 {
    ffi::UpdatePolicy(any_tasks)
}

#[allow(unused)]
#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {

    unsafe extern "C++" {
        include!("request_utils.h");

        fn PublishStateChangeEvent(bundleName: &str, taskId: u32, state: i32, uid: i32) -> bool;
        fn GetForegroundAbilities(uid: &mut Vec<i32>) -> i32;
        fn GetCallingBundle(token_id: u64) -> String;
        fn IsSystemAPI(token_id: u64) -> bool;
        fn CheckPermission(token_id: u64, permission: &str) -> bool;
        fn UpdatePolicy(any_tasks: bool) -> i32;
    }
}

#[cfg(feature = "oh")]
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
        assert!(!check_permission(
            "ohos.permission.INTERACT_ACROSS_LOCAL_ACCOUNTS_EXTENSION"
        ));
    }
}
