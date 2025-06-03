// Copyright (C) 2024 Huawei Device Co., Ltd.
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

use cxx::let_cxx_string;

use crate::wrapper;

pub fn acl_set_access(target_file: &str, entry_txt: &str) -> Result<(), i32> {
    let_cxx_string!(target_file = target_file);
    let_cxx_string!(entry_txt = entry_txt);
    let res = wrapper::AclSetAccess(&target_file, &entry_txt);
    if res != 0 {
        return Err(res);
    }
    Ok(())
}

pub fn acl_set_default(target_file: &str, entry_txt: &str) -> Result<(), i32> {
    let_cxx_string!(target_file = target_file);
    let_cxx_string!(entry_txt = entry_txt);
    let res = wrapper::AclSetDefault(&target_file, &entry_txt);
    if res != 0 {
        return Err(res);
    }
    Ok(())
}
