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

use request_core::{Action, FileSpec, TaskConfig, Version};

use crate::error::RequestError;
use crate::unfinished::get_context_cache;

mod acl;
pub mod path_mgr;

static INTERNAL_CACHE: &str = "internal://cache/";

pub fn check_file(config: &mut TaskConfig) -> Result<(), RequestError> {
    if config.common_data.action == Action::Download {
        check_file_download(config)?;
    }
    Ok(())
}

fn check_file_download(config: &mut TaskConfig) -> Result<(), RequestError> {
    if config.version == Version::API9 {
        check_file_download_api9(config)?
    } else {
        check_file_download_api10(config)?
    }

    Ok(())
}

fn check_file_download_api9(config: &mut TaskConfig) -> Result<(), RequestError> {
    let mut path = config
        .file_specs
        .get_mut(0)
        .unwrap_or(&mut FileSpec::new())
        .path
        .clone();
    if path.starts_with("/") {
        // API9 do not check
    } else {
        get_internal_path(&mut path)?;
    }
    Ok(())
}

fn get_internal_path(path: &mut String) -> Result<(), RequestError> {
    if let Some(pos) = path.find(INTERNAL_CACHE) {
        if pos != 0 {
            return Err(RequestError {
                code: 401,
                message:
                    "Parameter verification failed, GetInternalPath failed, filePath is not valid"
                        .to_owned(),
            });
        }
        path.drain(..INTERNAL_CACHE.len());
    }
    if path.len() == 0 {
        return Err(RequestError {
            code: 401,
            message: "Parameter verification failed, GetInternalPath failed, fileName is empty"
                .to_owned(),
        });
    }
    *path = get_context_cache() + "/" + path;
    Ok(())
}

fn check_file_download_api10(config: &mut TaskConfig) -> Result<(), RequestError> {
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ut_path() {
        // assert_eq!(get_internal_path(""));
    }
}
