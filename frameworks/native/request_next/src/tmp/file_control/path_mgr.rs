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

use std::fs;
use std::path::Path;

pub(crate) struct PathMgr {}

impl PathMgr {}

static AREA1: &str = "/data/storage/el1/base";
static AREA2: &str = "/data/storage/el2/base";
static AREA5: &str = "/data/storage/el5/base";

fn path_exists<P: AsRef<Path>>(path: P) -> bool {
    Path::new(path.as_ref()).exists()
}

fn create_dirs(dirs: Vec<&str>) -> bool {
    for path in dirs {
        if path_exists(path) {
            continue;
        }
        match fs::create_dir(path) {
            Ok(_) => {}
            Err(e) => {
                error!("Create dir err, {}, {}", path, e);
                return false;
            }
        }
    }
    true
}

pub(crate) fn belong_app_base(path: &str) -> bool {
    path.starts_with(AREA1) || path.starts_with(AREA2) || path.starts_with(AREA5)
}

// "/A/B/C" -> （vec!["A", "B", "C"]， vec!["/A", "/A/B", "/A/B/C"])
pub(crate) fn split_path(path: &str) -> Result<(Vec<&str>, Vec<&str>), ()> {
    if path.is_empty() || !path.starts_with('/') || path.ends_with('/') || path.contains("//") {
        return Err(());
    }
    let mut end: usize = 0;
    let mut full_paths = Vec::new();
    // 分割路径并过滤空字符串
    let parts: Vec<&str> = path
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| {
            end += "/".len() + s.len();
            full_paths.push(&path[..end]);
            s
        })
        .collect();
    // 检查是否为空路径
    if parts.is_empty() {
        return Err(());
    }
    Ok((parts, full_paths))
}

pub(crate) fn delete_base(v: &mut Vec<&str>) {
    v.retain(|s| s.len() > AREA1.len());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_split_path() {
        assert_eq!(
            split_path("/A/B/C"),
            Ok((vec!["A", "B", "C"], vec!["/A", "/A/B", "/A/B/C"]))
        );
        assert_eq!(split_path("/A/B/C/"), Err(()));
        assert_eq!(split_path("A/B/C/"), Err(()));
        assert_eq!(split_path("A/B/C"), Err(()));
        assert_eq!(split_path("//A//B//C"), Err(()));
        assert_eq!(split_path("/A/B//C"), Err(()));
        assert_eq!(split_path("/"), Err(()));
        assert_eq!(split_path(""), Err(()));
    }

    #[test]
    fn ut_delete_base() {
        let mut v = vec![
            "/data",
            "/data/storage",
            "/data/storage/el1",
            "/data/storage/el1/base",
            "/data/storage/el1/base/A",
            "/data/storage/el1/base/A/B",
        ];
        delete_base(&mut v);
        let v2 = vec!["/data/storage/el1/base/A", "/data/storage/el1/base/A/B"];
        assert_eq!(v, v2);
    }
}
